// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::ffi::c_void;
use std::os::fd::RawFd;

use super::error::Error;
use super::types::{Choice, Fd, Fraction, Id, Pointer, Rectangle, Type};
use super::{Pod, Primitive, RawPod};

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

    pub fn build(self) -> Result<RawPod<'a>, Error> {
        if let Some(e) = self.error {
            Err(e)
        } else {
            Ok(RawPod {
                data: &self.data[0..self.pos],
            })
        }
    }

    pub fn push_pod<U: Pod>(mut self, value: U) -> Self {
        if self.error.is_none() {
            match value.encode(&mut self.data[self.pos..]) {
                Ok(size) => self.pos += size,
                Err(e) => self.error = Some(e),
            }
        }

        self
    }

    pub fn push_none(self) -> Self {
        self.push_pod(())
    }

    pub fn push_bool(self, value: bool) -> Self {
        self.push_pod(value)
    }

    pub fn push_id(self, value: Id) -> Self {
        self.push_pod(value)
    }

    pub fn push_int(self, value: i32) -> Self {
        self.push_pod(value)
    }

    pub fn push_long(self, value: i64) -> Self {
        self.push_pod(value)
    }

    pub fn push_float(self, value: f32) -> Self {
        self.push_pod(value)
    }

    pub fn push_double(self, value: f64) -> Self {
        self.push_pod(value)
    }

    pub fn push_fd(self, value: RawFd) -> Self {
        self.push_pod(Fd(value))
    }

    pub fn push_rectangle(self, width: u32, height: u32) -> Self {
        self.push_pod(Rectangle { width, height })
    }

    pub fn push_fraction(self, num: u32, denom: u32) -> Self {
        self.push_pod(Fraction { num, denom })
    }

    pub fn push_string(self, value: &str) -> Self {
        self.push_pod(value)
    }

    pub fn push_bytes(self, value: &[u8]) -> Self {
        self.push_pod(value)
    }

    pub fn push_pointer(self, typ: Type, value: *const c_void) -> Self {
        self.push_pod(Pointer {
            type_: typ,
            ptr: value,
        })
    }

    pub fn push_array<T>(self, values: &[T]) -> Self
    where
        T: Pod + Primitive,
    {
        self.push_pod(values)
    }

    pub fn push_choice<T>(self, value: Choice<T>) -> Self
    where
        T: Pod + Primitive,
    {
        self.push_pod(value)
    }

    pub fn push_struct<F>(mut self, build_struct: F) -> Self
    where
        F: FnOnce(Builder) -> Builder,
    {
        if self.error.is_some() {
            return self;
        }

        if self.data.len() < 8 {
            self.error = Some(Error::NoSpace);
            return self;
        }

        let size = {
            let struct_builder = Builder::new(&mut self.data[self.pos + 8..]);
            match build_struct(struct_builder).build() {
                Ok(pod) => pod.data.len(),
                Err(e) => {
                    self.error = Some(e);
                    return self;
                }
            }
        };

        self.data[self.pos..self.pos + 4].copy_from_slice(&(size as u32).to_ne_bytes());
        self.data[self.pos + 4..self.pos + 8].copy_from_slice(&(Type::Struct as u32).to_ne_bytes());

        self.pos += 8 + size;
        self
    }
}
