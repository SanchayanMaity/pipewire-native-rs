// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::ffi::c_void;
use std::os::fd::RawFd;

use super::error::Error;
use super::types::{Fd, Fraction, Id, Pointer, Rectangle, Type};
use super::Pod;

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

    pub fn pod<T, U: Pod<T>>(mut self, value: U) -> Self {
        if self.error.is_none() {
            match value.encode(&mut self.data[self.pos..]) {
                Ok(size) => self.pos += size,
                Err(e) => self.error = Some(e),
            }
        }

        self
    }

    pub fn none(self) -> Self {
        self.pod(())
    }

    pub fn bool(self, value: bool) -> Self {
        self.pod(value)
    }

    pub fn id(self, value: Id) -> Self {
        self.pod(value)
    }

    pub fn int(self, value: i32) -> Self {
        self.pod(value)
    }

    pub fn long(self, value: i64) -> Self {
        self.pod(value)
    }

    pub fn float(self, value: f32) -> Self {
        self.pod(value)
    }

    pub fn double(self, value: f64) -> Self {
        self.pod(value)
    }

    pub fn string(self, value: &str) -> Self {
        self.pod(value)
    }

    pub fn bytes(self, value: &[u8]) -> Self {
        self.pod(value)
    }

    pub fn pointer(self, typ: Type, value: *const c_void) -> Self {
        self.pod(Pointer {
            type_: typ,
            ptr: value,
        })
    }

    pub fn fd(self, value: RawFd) -> Self {
        self.pod(Fd(value))
    }

    pub fn rectangle(self, width: u32, height: u32) -> Self {
        self.pod(Rectangle { width, height })
    }

    pub fn fraction(self, num: u32, denom: u32) -> Self {
        self.pod(Fraction { num, denom })
    }
}
