// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::{ops::Deref, os::fd::RawFd, pin::Pin};

use super::plugin::Interface;

pub type SourceFn = dyn FnMut(&dyn Loop, &Source);

pub struct Source {
    pub func: Pin<Box<SourceFn>>,
    pub fd: RawFd,
    pub mask: u32,
    pub rmask: u32,
}

pub type InvokeFn = dyn FnMut(&dyn Loop, bool, u32) -> i32;

pub trait Loop {
    fn add_source(&mut self, source: Pin<Box<Source>>) -> std::io::Result<i32>;
    fn update_source(&mut self, source: Pin<Box<Source>>) -> std::io::Result<i32>;
    fn remove_source(&mut self, fd: RawFd) -> std::io::Result<i32>;
    fn invoke(&mut self, func: Pin<Box<InvokeFn>>, block: bool) -> std::io::Result<i32>;
}

pub struct LoopImpl {
    pub inner: Box<dyn Loop>,
}

impl Deref for LoopImpl {
    type Target = dyn Loop;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl Interface for LoopImpl {}
