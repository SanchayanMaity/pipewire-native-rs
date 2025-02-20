// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::ffi::c_void;
use std::os::fd::RawFd;

use crate::spa::pod::error::Error;
use crate::spa::pod::types::{Fraction, Id, Rectangle, Type};

pub struct Builder<'a> {
    data: &'a mut [u8],
    pos: usize,
    error: Option<Error>,
}

impl<'a> Builder<'a> {
    pub fn new(data: &'a mut [u8]) -> Self {
        Self {
            data,
            pos: 0,
            error: None,
        }
    }

    pub fn build(self) -> Result<&'a [u8], Error> {
        if let Some(e) = self.error {
            Err(e)
        } else {
            Ok(&self.data[0..self.pos])
        }
    }

    fn write_header_fixed(&mut self, t: Type) {
        if self.error.is_some() {
            return;
        }

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

        if self.data.len() - self.pos < 8 + size + padding {
            self.error = Some(Error::NoSpace);
        } else {
            self.data[self.pos..self.pos + 4].copy_from_slice(&(size as u32).to_ne_bytes());
            self.pos += 4;
            self.data[self.pos..self.pos + 4].copy_from_slice(&(t as u32).to_ne_bytes());
            self.pos += 4;
        }
    }

    fn write_padding(&mut self) {
        let padding = 8 - self.pos % 8;

        if padding > 0 {
            self.data[self.pos..self.pos + padding].copy_from_slice(&[0; 8][0..padding]);
            self.pos += padding;
        }
    }

    pub fn none(mut self) -> Self {
        self.write_header_fixed(Type::None);

        self
    }

    pub fn bool(mut self, value: bool) -> Self {
        self.write_header_fixed(Type::Bool);

        if self.error.is_none() {
            let v: u32 = if value { 1 } else { 0 };
            self.data[self.pos..self.pos + 4].copy_from_slice(&v.to_ne_bytes());
            self.pos += 4;
            self.data[self.pos..self.pos + 4].copy_from_slice(&[0, 0, 0, 0]);
            self.pos += 4;
        }

        self
    }

    pub fn id(mut self, value: Id) -> Self {
        self.write_header_fixed(Type::Id);

        if self.error.is_none() {
            self.data[self.pos..self.pos + 4].copy_from_slice(&value.0.to_ne_bytes());
            self.pos += 4;
            self.data[self.pos..self.pos + 4].copy_from_slice(&[0, 0, 0, 0]);
            self.pos += 4;
        }

        self
    }

    pub fn int(mut self, value: i32) -> Self {
        self.write_header_fixed(Type::Int);

        if self.error.is_none() {
            self.data[self.pos..self.pos + 4].copy_from_slice(&value.to_ne_bytes());
            self.pos += 4;
            self.data[self.pos..self.pos + 4].copy_from_slice(&[0, 0, 0, 0]);
            self.pos += 4;
        }

        self
    }

    pub fn long(mut self, value: i64) -> Self {
        self.write_header_fixed(Type::Long);

        if self.error.is_none() {
            self.data[self.pos..self.pos + 8].copy_from_slice(&value.to_ne_bytes());
            self.pos += 8;
        }

        self
    }

    pub fn float(mut self, value: f32) -> Self {
        self.write_header_fixed(Type::Float);

        if self.error.is_none() {
            self.data[self.pos..self.pos + 4].copy_from_slice(&value.to_ne_bytes());
            self.pos += 4;
            self.data[self.pos..self.pos + 4].copy_from_slice(&[0, 0, 0, 0]);
            self.pos += 4;
        }

        self
    }

    pub fn double(mut self, value: f64) -> Self {
        self.write_header_fixed(Type::Double);

        if self.error.is_none() {
            self.data[self.pos..self.pos + 8].copy_from_slice(&value.to_ne_bytes());
            self.pos += 8;
        }

        self
    }

    pub fn string(mut self, value: &str) -> Self {
        if self.error.is_some() {
            return self;
        }

        let len = value.len() + 1;

        if len as u32 > u32::MAX || self.data.len() - self.pos < 8 + len {
            self.error = Some(Error::NoSpace);
            return self;
        }

        self.data[self.pos..self.pos + 4].copy_from_slice(&(len as u32).to_ne_bytes());
        self.pos += 4;
        self.data[self.pos..self.pos + 4].copy_from_slice(&(Type::String as u32).to_ne_bytes());
        self.pos += 4;
        self.data[self.pos..self.pos + value.len()].copy_from_slice(value.as_bytes());
        self.pos += value.len();
        self.data[self.pos] = 0;
        self.pos += 1;

        self.write_padding();

        self
    }

    pub fn bytes(mut self, value: &[u8]) -> Self {
        if self.error.is_some() {
            return self;
        }

        let len = value.len();

        if len as u32 > u32::MAX || self.data.len() - self.pos < 8 + len {
            self.error = Some(Error::NoSpace);
            return self;
        }

        self.data[self.pos..self.pos + 4].copy_from_slice(&(len as u32).to_ne_bytes());
        self.pos += 4;
        self.data[self.pos..self.pos + 4].copy_from_slice(&(Type::Bytes as u32).to_ne_bytes());
        self.pos += 4;
        self.data[self.pos..self.pos + value.len()].copy_from_slice(value);
        self.pos += value.len();

        self.write_padding();

        self
    }

    pub fn pointer(mut self, typ: Type, value: *const c_void) -> Self {
        if self.error.is_some() {
            return self;
        }

        let ptr_size = std::mem::size_of::<*const c_void>();
        let size = 4 /* type */ + 4 /* _padding */ + ptr_size /* pointer */;
        let padding = 8 - ptr_size;

        if self.data.len() - self.pos < size + padding {
            self.error = Some(Error::NoSpace);
            return self;
        }

        self.data[self.pos..self.pos + 4].copy_from_slice(&(size as u32).to_ne_bytes());
        self.pos += 4;
        self.data[self.pos..self.pos + 4].copy_from_slice(&(Type::Pointer as u32).to_ne_bytes());
        self.pos += 4;
        self.data[self.pos..self.pos + 4].copy_from_slice(&(typ as u32).to_ne_bytes());
        self.pos += 4;
        self.data[self.pos..self.pos + 4].copy_from_slice(&[0, 0, 0, 0]);
        self.pos += 4;
        if ptr_size == 8 {
            self.data[self.pos..self.pos + ptr_size].copy_from_slice(&(value as u64).to_ne_bytes());
            self.pos += 8;
        } else {
            self.data[self.pos..self.pos + ptr_size].copy_from_slice(&(value as u32).to_ne_bytes());
            self.pos += 4;
            self.data[self.pos..self.pos + 4].copy_from_slice(&[0, 0, 0, 0]);
            self.pos += 4;
        }

        self
    }

    pub fn fd(mut self, value: RawFd) -> Self {
        self.write_header_fixed(Type::Fd);

        if self.error.is_none() {
            self.data[self.pos..self.pos + 8].copy_from_slice(&(value as i64).to_ne_bytes());
            self.pos += 8;
        }

        self
    }

    pub fn rectangle(mut self, rect: Rectangle) -> Self {
        self.write_header_fixed(Type::Rectangle);

        if self.error.is_none() {
            self.data[self.pos..self.pos + 4].copy_from_slice(&rect.width.to_ne_bytes());
            self.pos += 4;
            self.data[self.pos..self.pos + 4].copy_from_slice(&rect.height.to_ne_bytes());
            self.pos += 4;
        }

        self
    }

    pub fn fraction(mut self, frac: Fraction) -> Self {
        self.write_header_fixed(Type::Fraction);

        if self.error.is_none() {
            self.data[self.pos..self.pos + 4].copy_from_slice(&frac.num.to_ne_bytes());
            self.pos += 4;
            self.data[self.pos..self.pos + 4].copy_from_slice(&frac.denom.to_ne_bytes());
            self.pos += 4;
        }

        self
    }
}
