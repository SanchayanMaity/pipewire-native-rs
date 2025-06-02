// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::{any::Any, pin::Pin};

use crate::dict::Dict;

use super::plugin::Interface;

pub const STACK_SIZE: &str = "thread.stack-size";

pub struct Thread {
    pub inner: Box<dyn Any>,
}

pub type ThreadReturn = Box<dyn Any + Send + 'static>;

#[allow(clippy::type_complexity)]
pub struct ThreadUtilsImpl {
    pub inner: Pin<Box<dyn Any>>,

    pub create: fn(
        this: &ThreadUtilsImpl,
        props: Option<&Dict>,
        start: Box<dyn FnOnce() -> ThreadReturn + Send + 'static>,
    ) -> Option<Thread>,
    pub join: fn(this: &ThreadUtilsImpl, thread: Thread) -> std::io::Result<ThreadReturn>,

    pub get_rt_range:
        fn(this: &ThreadUtilsImpl, props: Option<&Dict>) -> std::io::Result<(i32, i32)>,

    pub acquire_rt:
        fn(this: &ThreadUtilsImpl, thread: &Thread, priority: i32) -> std::io::Result<()>,
    pub drop_rt: fn(this: &ThreadUtilsImpl, thread: &Thread) -> std::io::Result<()>,
}

impl ThreadUtilsImpl {
    pub fn create<F>(&self, props: Option<&Dict>, start: F) -> Option<Thread>
    where
        F: FnOnce() -> ThreadReturn + Send + 'static,
    {
        (self.create)(self, props, Box::new(start))
    }

    pub fn join(&self, thread: Thread) -> std::io::Result<ThreadReturn> {
        (self.join)(self, thread)
    }

    pub fn get_rt_range(&self, props: Option<&Dict>) -> std::io::Result<(i32, i32)> {
        (self.get_rt_range)(self, props)
    }

    pub fn acquire_rt(&self, thread: &Thread, priority: i32) -> std::io::Result<()> {
        (self.acquire_rt)(self, thread, priority)
    }

    pub fn drop_rt(&self, thread: &Thread) -> std::io::Result<()> {
        (self.drop_rt)(self, thread)
    }
}

impl Interface for ThreadUtilsImpl {
    unsafe fn make_native(&self) -> *mut super::ffi::CInterface {
        crate::support::ffi::thread::make_native(self)
    }

    unsafe fn free_native(thread: *mut super::ffi::CInterface) {
        crate::support::ffi::thread::free_native(thread)
    }
}
