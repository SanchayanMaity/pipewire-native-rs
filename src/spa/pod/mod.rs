// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

pub mod builder;
pub mod error;
pub mod parser;
pub mod types;

use std::ffi::c_void;
use std::os::fd::RawFd;

use error::Error;
use types::{Fd, Fraction, Id, Pointer, Rectangle, Type};

pub trait Pod {
    // Default to Self once that is stable, or try to generate references to owned data
    type DecodesTo;

    fn encode(&self, data: &mut [u8]) -> Result<usize, Error>;
    fn decode(data: &[u8]) -> Result<(Self::DecodesTo, usize), Error>;
}

trait Primitive {
    fn pod_type() -> Type;
    fn pod_size() -> usize;

    fn encode_body(&self, data: &mut [u8]) -> Result<(), Error>;
    fn decode_body(data: &[u8]) -> Result<Self, Error>
    where
        Self: Sized;
}

fn pad_8(size: usize) -> usize {
    if size % 8 > 0 {
        8 - size % 8
    } else {
        0
    }
}

impl<T> Pod for T
where
    T: Primitive,
{
    type DecodesTo = Self;

    fn encode(&self, data: &mut [u8]) -> Result<usize, Error> {
        let size = Self::pod_size();
        let padding = pad_8(size);

        if data.len() < 8 + size + padding {
            return Err(Error::NoSpace);
        }

        data[0..4].copy_from_slice(&(Self::pod_size() as u32).to_ne_bytes());
        data[4..8].copy_from_slice(&(Self::pod_type() as u32).to_ne_bytes());

        self.encode_body(&mut data[8..])?;

        if padding > 0 {
            data[8 + size..8 + size + padding].copy_from_slice(&[0; 8][0..padding]);
        }

        Ok(8 + size + padding)
    }

    fn decode(data: &[u8]) -> Result<(Self::DecodesTo, usize), Error> {
        if data.len() < 16 {
            return Err(Error::Invalid);
        }

        let size = u32::from_ne_bytes(data[0..4].try_into().unwrap()) as usize;
        if size != Self::pod_size() {
            return Err(Error::Invalid);
        }

        let t = u32::from_ne_bytes(data[4..8].try_into().unwrap());
        if t != Self::pod_type() as u32 {
            return Err(Error::Invalid);
        }

        let val = Self::decode_body(&data[8..])?;
        let padding = pad_8(size);
        Ok((val, 8 + size + padding))
    }
}

impl Primitive for () {
    fn pod_type() -> Type {
        Type::None
    }

    fn pod_size() -> usize {
        0
    }

    fn encode_body(&self, _data: &mut [u8]) -> Result<(), Error> {
        Ok(())
    }

    fn decode_body(_data: &[u8]) -> Result<Self, Error> {
        Ok(())
    }
}

impl Primitive for bool {
    fn pod_type() -> Type {
        Type::Bool
    }

    fn pod_size() -> usize {
        4
    }

    fn encode_body(&self, data: &mut [u8]) -> Result<(), Error> {
        data[0..4].copy_from_slice(&(*self as u32).to_ne_bytes());
        Ok(())
    }

    fn decode_body(data: &[u8]) -> Result<Self, Error> {
        let val = u32::from_ne_bytes(data[0..4].try_into().unwrap()) != 0;
        Ok(val)
    }
}

impl Primitive for Id {
    fn pod_type() -> Type {
        Type::Id
    }

    fn pod_size() -> usize {
        4
    }

    fn encode_body(&self, data: &mut [u8]) -> Result<(), Error> {
        data[0..4].copy_from_slice(&self.0.to_ne_bytes());
        Ok(())
    }

    fn decode_body(data: &[u8]) -> Result<Self, Error> {
        let val = u32::from_ne_bytes(data[0..4].try_into().unwrap());
        Ok(Id(val))
    }
}

impl Primitive for i32 {
    fn pod_type() -> Type {
        Type::Int
    }

    fn pod_size() -> usize {
        4
    }

    fn encode_body(&self, data: &mut [u8]) -> Result<(), Error> {
        data[0..4].copy_from_slice(&self.to_ne_bytes());
        Ok(())
    }

    fn decode_body(data: &[u8]) -> Result<Self, Error> {
        let val = i32::from_ne_bytes(data[0..4].try_into().unwrap());
        Ok(val)
    }
}

impl Primitive for i64 {
    fn pod_type() -> Type {
        Type::Long
    }

    fn pod_size() -> usize {
        8
    }

    fn encode_body(&self, data: &mut [u8]) -> Result<(), Error> {
        data[0..8].copy_from_slice(&self.to_ne_bytes());
        Ok(())
    }

    fn decode_body(data: &[u8]) -> Result<Self, Error> {
        let val = i64::from_ne_bytes(data[0..8].try_into().unwrap());
        Ok(val)
    }
}

impl Primitive for f32 {
    fn pod_type() -> Type {
        Type::Float
    }

    fn pod_size() -> usize {
        4
    }

    fn encode_body(&self, data: &mut [u8]) -> Result<(), Error> {
        data[0..4].copy_from_slice(&self.to_ne_bytes());
        Ok(())
    }

    fn decode_body(data: &[u8]) -> Result<Self, Error> {
        let val = f32::from_ne_bytes(data[0..4].try_into().unwrap());
        Ok(val)
    }
}

impl Primitive for f64 {
    fn pod_type() -> Type {
        Type::Double
    }

    fn pod_size() -> usize {
        8
    }

    fn encode_body(&self, data: &mut [u8]) -> Result<(), Error> {
        data[0..8].copy_from_slice(&self.to_ne_bytes());
        Ok(())
    }

    fn decode_body(data: &[u8]) -> Result<Self, Error> {
        let val = f64::from_ne_bytes(data[0..8].try_into().unwrap());
        Ok(val)
    }
}

impl Primitive for Fd {
    fn pod_type() -> Type {
        Type::Fd
    }

    fn pod_size() -> usize {
        8
    }

    fn encode_body(&self, data: &mut [u8]) -> Result<(), Error> {
        data[0..8].copy_from_slice(&(self.0 as i64).to_ne_bytes());
        Ok(())
    }

    fn decode_body(data: &[u8]) -> Result<Fd, Error> {
        let val = i64::from_ne_bytes(data[0..8].try_into().unwrap());
        Ok(Fd(val as RawFd))
    }
}

impl Primitive for Rectangle {
    fn pod_type() -> Type {
        Type::Rectangle
    }

    fn pod_size() -> usize {
        8
    }

    fn encode_body(&self, data: &mut [u8]) -> Result<(), Error> {
        data[0..4].copy_from_slice(&self.width.to_ne_bytes());
        data[4..8].copy_from_slice(&self.height.to_ne_bytes());
        Ok(())
    }

    fn decode_body(data: &[u8]) -> Result<Rectangle, Error> {
        let width = u32::from_ne_bytes(data[0..4].try_into().unwrap());
        let height = u32::from_ne_bytes(data[4..8].try_into().unwrap());

        Ok(Rectangle { width, height })
    }
}

impl Primitive for Fraction {
    fn pod_type() -> Type {
        Type::Fraction
    }

    fn pod_size() -> usize {
        8
    }

    fn encode_body(&self, data: &mut [u8]) -> Result<(), Error> {
        data[0..4].copy_from_slice(&self.num.to_ne_bytes());
        data[4..8].copy_from_slice(&self.denom.to_ne_bytes());
        Ok(())
    }

    fn decode_body(data: &[u8]) -> Result<Fraction, Error> {
        let num = u32::from_ne_bytes(data[0..4].try_into().unwrap());
        let denom = u32::from_ne_bytes(data[4..8].try_into().unwrap());

        Ok(Fraction { num, denom })
    }
}

impl Pod for &str {
    type DecodesTo = String;

    fn encode(&self, data: &mut [u8]) -> Result<usize, Error> {
        let len = self.len() + 1;
        let padding = pad_8(len);

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
        let padding = pad_8(len);

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

impl Pod for &[u8] {
    type DecodesTo = Vec<u8>;

    fn encode(&self, data: &mut [u8]) -> Result<usize, Error> {
        let len = self.len();
        let padding = pad_8(len);

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
        let padding = pad_8(len);

        if data.len() < 8 + len {
            return Err(Error::Invalid);
        }

        if data[4..8] != (Type::Bytes as u32).to_ne_bytes() {
            return Err(Error::Invalid);
        }

        Ok((data[8..8 + len].to_vec(), 8 + len + padding))
    }
}

impl Pod for Pointer {
    type DecodesTo = Pointer;

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
