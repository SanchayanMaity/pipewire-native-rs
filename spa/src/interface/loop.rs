// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::{any::Any, os::fd::RawFd, pin::Pin};

use super::plugin::Interface;

pub type SourceFn = dyn FnMut(&LoopImpl, &Source);

pub struct Source {
    pub func: Pin<Box<SourceFn>>,
    pub fd: RawFd,
    pub mask: u32,
    pub rmask: u32,
}

pub type InvokeFn = dyn FnMut(LoopImpl, bool, u32) -> i32;

pub struct LoopImpl {
    pub inner: Box<dyn Any>,

    pub add_source: fn(&mut LoopImpl, source: Pin<Box<Source>>) -> std::io::Result<i32>,
    pub update_source: fn(&mut LoopImpl, source: Pin<Box<Source>>) -> std::io::Result<i32>,
    pub remove_source: fn(&mut LoopImpl, fd: RawFd) -> std::io::Result<i32>,
    pub invoke: fn(&mut LoopImpl, func: Pin<Box<InvokeFn>>, block: bool) -> std::io::Result<i32>,
}

impl LoopImpl {
    pub fn add_source(&mut self, source: Pin<Box<Source>>) -> std::io::Result<i32> {
        (self.add_source)(self, source)
    }
    pub fn update_source(&mut self, source: Pin<Box<Source>>) -> std::io::Result<i32> {
        (self.update_source)(self, source)
    }

    pub fn remove_source(&mut self, fd: RawFd) -> std::io::Result<i32> {
        (self.remove_source)(self, fd)
    }

    pub fn invoke(&mut self, func: Pin<Box<InvokeFn>>, block: bool) -> std::io::Result<i32> {
        (self.invoke)(self, func, block)
    }
}

impl Interface for LoopImpl {}
