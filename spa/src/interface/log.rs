// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::ops::Deref;

use super::plugin::Interface;

#[derive(Copy, Clone, Debug)]
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
pub trait Log {
    fn log(&self, level: LogLevel, file: &str, line: i32, func: &str, args: std::fmt::Arguments);
    fn logt(
        &self,
        level: LogLevel,
        topic: &LogTopic,
        file: &str,
        line: i32,
        func: &str,
        args: std::fmt::Arguments,
    );
}

pub struct LogImpl {
    pub inner: Box<dyn Log>,
}

impl Deref for LogImpl {
    type Target = dyn Log;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

impl Interface for LogImpl {}
