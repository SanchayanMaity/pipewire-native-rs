// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

pub mod builder;
pub mod error;
pub mod parser;
pub mod types;

mod internal;

use std::ffi::c_void;
use std::os::fd::RawFd;

use error::Error;
use internal::primitive_pod_size;
use types::{Fd, Fraction, Id, Pointer, Rectangle, Type};

// T is the type we decode to (which might need to be the owned version for things like slices)
pub trait Pod<T> {
    fn encode(&self, data: &mut [u8]) -> Result<usize, Error>;
    fn decode(data: &[u8]) -> Result<(T, usize), Error>;
}

fn write_header_fixed(data: &mut [u8], t: Type) -> Result<usize, Error> {
    let (size, padding) = primitive_pod_size(t);

    if data.len() < 8 + size + padding {
        Err(Error::NoSpace)
    } else {
        data[0..4].copy_from_slice(&(size as u32).to_ne_bytes());
        data[4..8].copy_from_slice(&(t as u32).to_ne_bytes());
        Ok(8 + size + padding)
    }
}

fn decode_header_fixed(data: &[u8], t: Type) -> Result<usize, Error> {
    let (size, padding) = primitive_pod_size(t);

    if data.len() < 8 + size + padding {
        Err(Error::Invalid)
    } else if data[0..4] != (size as u32).to_ne_bytes() {
        return Err(Error::Invalid);
    } else if data[4..8] != (t as u32).to_ne_bytes() {
        return Err(Error::Invalid);
    } else {
        Ok(8 + size + padding)
    }
}

impl Pod<()> for () {
    fn encode(&self, data: &mut [u8]) -> Result<usize, Error> {
        write_header_fixed(data, Type::None)
    }

    fn decode(data: &[u8]) -> Result<(Self, usize), Error> {
        decode_header_fixed(data, Type::None).map(|s| ((), s))
    }
}

impl Pod<bool> for bool {
    fn encode(&self, data: &mut [u8]) -> Result<usize, Error> {
        let size = write_header_fixed(data, Type::Bool)?;

        data[8..12].copy_from_slice(&(*self as u32).to_ne_bytes());

        Ok(size)
    }

    fn decode(data: &[u8]) -> Result<(Self, usize), Error> {
        let size = decode_header_fixed(data, Type::Bool)?;
        let val = u32::from_ne_bytes(data[8..12].try_into().unwrap()) != 0;

        Ok((val, size))
    }
}

impl Pod<Id> for Id {
    fn encode(&self, data: &mut [u8]) -> Result<usize, Error> {
        let size = write_header_fixed(data, Type::Id)?;

        data[8..12].copy_from_slice(&self.0.to_ne_bytes());

        Ok(size)
    }

    fn decode(data: &[u8]) -> Result<(Self, usize), Error> {
        let size = decode_header_fixed(data, Type::Id)?;
        let val = u32::from_ne_bytes(data[8..12].try_into().unwrap());

        Ok((Id(val), size))
    }
}

impl Pod<i32> for i32 {
    fn encode(&self, data: &mut [u8]) -> Result<usize, Error> {
        let size = write_header_fixed(data, Type::Int)?;

        data[8..12].copy_from_slice(&self.to_ne_bytes());
        data[12..16].copy_from_slice(&[0, 0, 0, 0]);

        Ok(size)
    }

    fn decode(data: &[u8]) -> Result<(Self, usize), Error> {
        let size = decode_header_fixed(data, Type::Int)?;
        let val = i32::from_ne_bytes(data[8..12].try_into().unwrap());

        Ok((val, size))
    }
}

impl Pod<i64> for i64 {
    fn encode(&self, data: &mut [u8]) -> Result<usize, Error> {
        let size = write_header_fixed(data, Type::Long)?;

        data[8..16].copy_from_slice(&self.to_ne_bytes());

        Ok(size)
    }

    fn decode(data: &[u8]) -> Result<(Self, usize), Error> {
        let size = decode_header_fixed(data, Type::Long)?;
        let val = i64::from_ne_bytes(data[8..16].try_into().unwrap());

        Ok((val, size))
    }
}

impl Pod<f32> for f32 {
    fn encode(&self, data: &mut [u8]) -> Result<usize, Error> {
        let size = write_header_fixed(data, Type::Float)?;

        data[8..12].copy_from_slice(&self.to_ne_bytes());
        data[12..16].copy_from_slice(&[0, 0, 0, 0]);

        Ok(size)
    }

    fn decode(data: &[u8]) -> Result<(Self, usize), Error> {
        let size = decode_header_fixed(data, Type::Float)?;
        let val = f32::from_ne_bytes(data[8..12].try_into().unwrap());

        Ok((val, size))
    }
}

impl Pod<f64> for f64 {
    fn encode(&self, data: &mut [u8]) -> Result<usize, Error> {
        let size = write_header_fixed(data, Type::Double)?;

        data[8..16].copy_from_slice(&self.to_ne_bytes());

        Ok(size)
    }

    fn decode(data: &[u8]) -> Result<(Self, usize), Error> {
        let size = decode_header_fixed(data, Type::Double)?;
        let val = f64::from_ne_bytes(data[8..16].try_into().unwrap());

        Ok((val, size))
    }
}

impl Pod<String> for &str {
    fn encode(&self, data: &mut [u8]) -> Result<usize, Error> {
        let len = self.len() + 1;
        let padding = 8 - len % 8;

        if len as u32 > u32::MAX || data.len() < 8 + len + padding {
            return Err(Error::NoSpace);
        }

        data[0..4].copy_from_slice(&(len as u32).to_ne_bytes());
        data[4..8].copy_from_slice(&(Type::String as u32).to_ne_bytes());
        data[8..8 + self.len()].copy_from_slice(self.as_bytes());
        // Null terminator
        data[8 + self.len()] = 0;
        // Padding
        data[8 + len..8 + len + padding].copy_from_slice(&[0; 8][0..padding]);

        Ok(8 + len + padding)
    }

    fn decode(data: &[u8]) -> Result<(String, usize), Error> {
        let len = u32::from_ne_bytes(data[0..4].try_into().unwrap()) as usize;
        let padding = 8 - len % 8;

        if data.len() < 8 + len {
            return Err(Error::Invalid);
        }

        if data[4..8] != (Type::String as u32).to_ne_bytes() {
            return Err(Error::Invalid);
        }

        let s = String::from_utf8_lossy(&data[8..8 + len - 1]).to_string();
        // Null terminator
        if data[8 + len - 1] != 0 {
            return Err(Error::Invalid);
        }

        Ok((s, 8 + len + padding))
    }
}

impl Pod<Vec<u8>> for &[u8] {
    fn encode(&self, data: &mut [u8]) -> Result<usize, Error> {
        let len = self.len();
        let padding = 8 - len % 8;

        if len as u32 > u32::MAX || data.len() < 8 + len + padding {
            return Err(Error::NoSpace);
        }

        data[0..4].copy_from_slice(&(len as u32).to_ne_bytes());
        data[4..8].copy_from_slice(&(Type::Bytes as u32).to_ne_bytes());
        data[8..8 + self.len()].copy_from_slice(self);
        // Padding
        data[8 + len..8 + len + padding].copy_from_slice(&[0; 8][0..padding]);

        Ok(8 + len + padding)
    }

    fn decode(data: &[u8]) -> Result<(Vec<u8>, usize), Error> {
        let len = u32::from_ne_bytes(data[0..4].try_into().unwrap()) as usize;
        let padding = 8 - len % 8;

        if data.len() < 8 + len {
            return Err(Error::Invalid);
        }

        if data[4..8] != (Type::Bytes as u32).to_ne_bytes() {
            return Err(Error::Invalid);
        }

        Ok((data[8..8 + len].to_vec(), 8 + len + padding))
    }
}

impl Pod<Pointer> for Pointer {
    fn encode(&self, data: &mut [u8]) -> Result<usize, Error> {
        let ptr_size = std::mem::size_of::<*const c_void>();
        let size = 4 /* type */ + 4 /* _padding */ + ptr_size /* pointer */;

        // size + type + type_of_ptr + 4 (padding_) + ptr + (maybe padding for u32ptr)
        if data.len() < 24 {
            return Err(Error::NoSpace);
        }

        data[0..4].copy_from_slice(&(size as u32).to_ne_bytes());
        data[4..8].copy_from_slice(&(Type::Pointer as u32).to_ne_bytes());
        data[8..12].copy_from_slice(&(self.type_ as u32).to_ne_bytes());
        data[12..16].copy_from_slice(&[0, 0, 0, 0]);
        if ptr_size == 8 {
            data[16..24].copy_from_slice(&(self.ptr as u64).to_ne_bytes());
        } else {
            data[16..20].copy_from_slice(&(self.ptr as u32).to_ne_bytes());
            data[20..24].copy_from_slice(&[0, 0, 0, 0]);
        }

        Ok(24)
    }

    fn decode(data: &[u8]) -> Result<(Pointer, usize), Error> {
        let size = u32::from_ne_bytes(data[0..4].try_into().unwrap()) as usize;
        let ptr_size = std::mem::size_of::<*const c_void>();
        let padding = 8 - ptr_size;

        if data.len() < 24 {
            return Err(Error::Invalid);
        }

        if data[4..8] != (Type::Pointer as u32).to_ne_bytes() {
            return Err(Error::Invalid);
        }

        // FIXME: we should be able to do better than this
        let type_ =
            unsafe { std::mem::transmute(u32::from_ne_bytes(data[8..12].try_into().unwrap())) };
        let ptr = if ptr_size == 8 {
            u64::from_ne_bytes(data[16..24].try_into().unwrap()) as *const c_void
        } else {
            u32::from_ne_bytes(data[16..20].try_into().unwrap()) as *const c_void
        };

        Ok((Pointer { type_, ptr }, 8 + size + padding))
    }
}

impl Pod<Fd> for Fd {
    fn encode(&self, data: &mut [u8]) -> Result<usize, Error> {
        let size = write_header_fixed(data, Type::Fd)?;

        data[8..16].copy_from_slice(&(self.0 as i64).to_ne_bytes());

        Ok(size)
    }

    fn decode(data: &[u8]) -> Result<(Fd, usize), Error> {
        let size = decode_header_fixed(data, Type::Fd)?;
        let val = i64::from_ne_bytes(data[8..16].try_into().unwrap());

        Ok((Fd(val as RawFd), size))
    }
}

impl Pod<Rectangle> for Rectangle {
    fn encode(&self, data: &mut [u8]) -> Result<usize, Error> {
        let size = write_header_fixed(data, Type::Rectangle)?;

        data[8..12].copy_from_slice(&self.width.to_ne_bytes());
        data[12..16].copy_from_slice(&self.height.to_ne_bytes());

        Ok(size)
    }

    fn decode(data: &[u8]) -> Result<(Rectangle, usize), Error> {
        let size = decode_header_fixed(data, Type::Rectangle)?;
        let width = u32::from_ne_bytes(data[8..12].try_into().unwrap());
        let height = u32::from_ne_bytes(data[12..16].try_into().unwrap());

        Ok((Rectangle { width, height }, size))
    }
}

impl Pod<Fraction> for Fraction {
    fn encode(&self, data: &mut [u8]) -> Result<usize, Error> {
        let size = write_header_fixed(data, Type::Fraction)?;

        data[8..12].copy_from_slice(&self.num.to_ne_bytes());
        data[12..16].copy_from_slice(&self.denom.to_ne_bytes());

        Ok(size)
    }

    fn decode(data: &[u8]) -> Result<(Fraction, usize), Error> {
        let size = decode_header_fixed(data, Type::Fraction)?;
        let num = u32::from_ne_bytes(data[8..12].try_into().unwrap());
        let denom = u32::from_ne_bytes(data[12..16].try_into().unwrap());

        Ok((Fraction { num, denom }, size))
    }
}
