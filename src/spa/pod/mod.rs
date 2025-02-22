// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

pub mod builder;
pub mod error;
pub mod types;

use std::ffi::c_void;

use error::Error;
use types::{Fd, Fraction, Id, Pointer, Rectangle, Type};

pub trait Pod {
    fn encode(&self, data: &mut [u8]) -> Result<usize, Error>;
}

fn write_header_fixed(data: &mut [u8], t: Type) -> Result<usize, Error> {
    let (size, padding) = match t {
        Type::None => (0, 0),
        Type::Bool => (4, 4),
        Type::Id => (4, 4),
        Type::Int => (4, 4),
        Type::Long => (8, 0),
        Type::Float => (4, 4),
        Type::Double => (8, 0),
        Type::Fd => (8, 0),
        Type::Rectangle => (8, 0),
        Type::Fraction => (8, 0),
        _ => unreachable!(),
    };

    if data.len() < 8 + size + padding {
        Err(Error::NoSpace)
    } else {
        data[0..4].copy_from_slice(&(size as u32).to_ne_bytes());
        data[4..8].copy_from_slice(&(t as u32).to_ne_bytes());
        Ok(8 + size + padding)
    }
}

impl Pod for () {
    fn encode(&self, data: &mut [u8]) -> Result<usize, Error> {
        write_header_fixed(data, Type::None)
    }
}

impl Pod for bool {
    fn encode(&self, data: &mut [u8]) -> Result<usize, Error> {
        let size = write_header_fixed(data, Type::Bool)?;

        data[8..12].copy_from_slice(&(*self as u32).to_ne_bytes());

        Ok(size)
    }
}

impl Pod for Id {
    fn encode(&self, data: &mut [u8]) -> Result<usize, Error> {
        let size = write_header_fixed(data, Type::Id)?;

        data[8..12].copy_from_slice(&self.0.to_ne_bytes());

        Ok(size)
    }
}

impl Pod for i32 {
    fn encode(&self, data: &mut [u8]) -> Result<usize, Error> {
        let size = write_header_fixed(data, Type::Int)?;

        data[8..12].copy_from_slice(&self.to_ne_bytes());
        data[12..16].copy_from_slice(&[0, 0, 0, 0]);

        Ok(size)
    }
}

impl Pod for i64 {
    fn encode(&self, data: &mut [u8]) -> Result<usize, Error> {
        let size = write_header_fixed(data, Type::Long)?;

        data[8..16].copy_from_slice(&self.to_ne_bytes());

        Ok(size)
    }
}

impl Pod for f32 {
    fn encode(&self, data: &mut [u8]) -> Result<usize, Error> {
        let size = write_header_fixed(data, Type::Float)?;

        data[8..12].copy_from_slice(&self.to_ne_bytes());
        data[12..16].copy_from_slice(&[0, 0, 0, 0]);

        Ok(size)
    }
}

impl Pod for f64 {
    fn encode(&self, data: &mut [u8]) -> Result<usize, Error> {
        let size = write_header_fixed(data, Type::Double)?;

        data[8..16].copy_from_slice(&self.to_ne_bytes());

        Ok(size)
    }
}

impl Pod for &str {
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
}

impl Pod for &[u8] {
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
}

impl Pod for Pointer {
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
}

impl Pod for Fd {
    fn encode(&self, data: &mut [u8]) -> Result<usize, Error> {
        let size = write_header_fixed(data, Type::Fd)?;

        data[8..16].copy_from_slice(&(self.0 as i64).to_ne_bytes());

        Ok(size)
    }
}

impl Pod for Rectangle {
    fn encode(&self, data: &mut [u8]) -> Result<usize, Error> {
        let size = write_header_fixed(data, Type::Rectangle)?;

        data[8..12].copy_from_slice(&self.width.to_ne_bytes());
        data[12..16].copy_from_slice(&self.height.to_ne_bytes());

        Ok(size)
    }
}

impl Pod for Fraction {
    fn encode(&self, data: &mut [u8]) -> Result<usize, Error> {
        let size = write_header_fixed(data, Type::Fraction)?;

        data[8..12].copy_from_slice(&self.num.to_ne_bytes());
        data[12..16].copy_from_slice(&self.denom.to_ne_bytes());

        Ok(size)
    }
}
