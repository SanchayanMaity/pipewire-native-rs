// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::{any::Any, os::fd::RawFd, pin::Pin};

use super::plugin::Interface;

#[derive(Copy, Clone, Debug)]
pub struct Source {
    pub fd: RawFd,
    pub mask: u32,
    pub rmask: u32,
}

pub type SourceFn = dyn FnMut(&Source) + 'static;
pub type InvokeFn = dyn FnMut(bool, u32, &[u8]) -> i32 + 'static;

pub struct LoopImpl {
    pub inner: Pin<Box<dyn Any>>,

    pub add_source: fn(&mut LoopImpl, source: &Source, func: Box<SourceFn>) -> std::io::Result<i32>,
    pub update_source: fn(&mut LoopImpl, source: &Source) -> std::io::Result<i32>,
    pub remove_source: fn(&mut LoopImpl, fd: RawFd) -> std::io::Result<i32>,
    pub invoke: fn(
        this: &mut LoopImpl,
        seq: u32,
        data: &[u8],
        block: bool,
        func: Box<InvokeFn>,
    ) -> std::io::Result<i32>,
}

impl LoopImpl {
    pub fn add_source(&mut self, source: &Source, func: Box<SourceFn>) -> std::io::Result<i32> {
        (self.add_source)(self, source, func)
    }

    pub fn update_source(&mut self, source: &Source) -> std::io::Result<i32> {
        (self.update_source)(self, source)
    }

    pub fn remove_source(&mut self, fd: RawFd) -> std::io::Result<i32> {
        (self.remove_source)(self, fd)
    }

    pub fn invoke(
        &mut self,
        seq: u32,
        data: &[u8],
        block: bool,
        func: Box<InvokeFn>,
    ) -> std::io::Result<i32> {
        (self.invoke)(self, seq, data, block, func)
    }
}

impl Interface for LoopImpl {}
