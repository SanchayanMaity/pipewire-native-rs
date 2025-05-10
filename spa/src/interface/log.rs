// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::{any::Any, pin::Pin};

use pipewire_native_macros::EnumU32;

use super::plugin::Interface;

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
    pub topic: String,
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
        file: &str,
        line: i32,
        func: &str,
        args: std::fmt::Arguments,
    ),
    pub logt: fn(
        this: &LogImpl,
        level: LogLevel,
        topic: &LogTopic,
        file: &str,
        line: i32,
        func: &str,
        args: std::fmt::Arguments,
    ),
}

impl LogImpl {
    pub fn log(
        &self,
        level: LogLevel,
        file: &str,
        line: i32,
        func: &str,
        args: std::fmt::Arguments,
    ) {
        (self.log)(self, level, file, line, func, args)
    }

    pub fn logt(
        &self,
        level: LogLevel,
        topic: &LogTopic,
        file: &str,
        line: i32,
        func: &str,
        args: std::fmt::Arguments,
    ) {
        (self.logt)(self, level, topic, file, line, func, args)
    }
}

impl Interface for LogImpl {}
