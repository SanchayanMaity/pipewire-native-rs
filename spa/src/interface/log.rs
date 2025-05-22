// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::{any::Any, ffi::CStr, pin::Pin};

use pipewire_native_macros::EnumU32;

use super::plugin::Interface;

pub const LEVEL: &str = "log.level";
pub const COLORS: &str = "log.colors";
pub const FILE: &str = "log.file";
pub const TIMESTAMP: &str = "log.timestamp";
pub const LINE: &str = "log.line";
pub const PATTERNS: &str = "log.patterns";

#[repr(u32)]
#[derive(Copy, Clone, Debug, EnumU32)]
pub enum LogLevel {
    None = 0,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[derive(Debug)]
pub struct LogTopic {
    /* We use a CStr to make translation to C more efficient. We don't need this to be owned, as we
     * expect to allocate it from global strings */
    pub topic: &'static CStr,
    pub level: LogLevel,
    pub has_custom_level: bool,
}

/* TODO: need some macros to make logging less cumbersome */
pub struct LogImpl {
    pub inner: Pin<Box<dyn Any>>,
    pub level: LogLevel,

    pub log: fn(
        this: &LogImpl,
        level: LogLevel,
        file: &CStr,
        line: i32,
        func: &CStr,
        args: std::fmt::Arguments,
    ),
    pub logt: fn(
        this: &LogImpl,
        level: LogLevel,
        topic: &LogTopic,
        file: &CStr,
        line: i32,
        func: &CStr,
        args: std::fmt::Arguments,
    ),
}

impl LogImpl {
    pub fn log(
        &self,
        level: LogLevel,
        file: &CStr,
        line: i32,
        func: &CStr,
        args: std::fmt::Arguments,
    ) {
        (self.log)(self, level, file, line, func, args)
    }

    pub fn logt(
        &self,
        level: LogLevel,
        topic: &LogTopic,
        file: &CStr,
        line: i32,
        func: &CStr,
        args: std::fmt::Arguments,
    ) {
        (self.logt)(self, level, topic, file, line, func, args)
    }
}

impl Interface for LogImpl {
    unsafe fn make_native(&self) -> *mut super::ffi::CInterface {
        crate::support::ffi::log::make_native(self)
    }

    unsafe fn free_native(log: *mut super::ffi::CInterface) {
        crate::support::ffi::log::free_native(log)
    }
}
