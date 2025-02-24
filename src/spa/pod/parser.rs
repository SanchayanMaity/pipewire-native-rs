// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use super::error::Error;
use super::types::{Fd, Fraction, Id, Pointer, Rectangle};
use super::{Pod, Primitive};

pub struct Parser<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(data: &'a [u8]) -> Parser<'a> {
        Parser { data, pos: 0 }
    }

    pub fn pop_pod<U: Pod>(&mut self) -> Result<<U as Pod>::DecodesTo, Error> {
        let (res, size) = U::decode(&self.data[self.pos..])?;

        self.pos += size;

        Ok(res)
    }

    pub fn pop_none(&mut self) -> Result<(), Error> {
        self.pop_pod::<()>()
    }

    pub fn pop_bool(&mut self) -> Result<bool, Error> {
        self.pop_pod::<bool>()
    }

    pub fn pop_id(&mut self) -> Result<Id, Error> {
        self.pop_pod::<Id>()
    }

    pub fn pop_int(&mut self) -> Result<i32, Error> {
        self.pop_pod::<i32>()
    }

    pub fn pop_long(&mut self) -> Result<i64, Error> {
        self.pop_pod::<i64>()
    }

    pub fn pop_float(&mut self) -> Result<f32, Error> {
        self.pop_pod::<f32>()
    }

    pub fn pop_double(&mut self) -> Result<f64, Error> {
        self.pop_pod::<f64>()
    }

    pub fn pop_string(&mut self) -> Result<String, Error> {
        self.pop_pod::<&str>()
    }

    pub fn pop_bytes(&mut self) -> Result<Vec<u8>, Error> {
        self.pop_pod::<&[u8]>()
    }

    pub fn pop_pointer(&mut self) -> Result<Pointer, Error> {
        self.pop_pod::<Pointer>()
    }

    pub fn pop_fd(&mut self) -> Result<Fd, Error> {
        self.pop_pod::<Fd>()
    }

    pub fn pop_rectangle(&mut self) -> Result<Rectangle, Error> {
        self.pop_pod::<Rectangle>()
    }

    pub fn pop_fraction(&mut self) -> Result<Fraction, Error> {
        self.pop_pod::<Fraction>()
    }

    pub fn pop_array<T>(&mut self) -> Result<Vec<T>, Error>
    where
        T: Pod + Primitive,
    {
        self.pop_pod::<&[T]>()
    }
}
