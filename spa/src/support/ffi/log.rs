// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::ffi::{c_char, c_int, c_void};

use pipewire_native_macros::EnumU32;

use crate::interface::log::{LogImpl, LogLevel};

use super::{c_string, plugin::CInterface};

struct CLogImpl {}

#[repr(u32)]
#[derive(Copy, Clone, Debug, EnumU32)]
pub enum CLogLevel {
    None = 0,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CLogTopic {
    version: u32,
    topic: *const c_char,
    level: CLogLevel,
    has_custom_level: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CLogMethods {
    version: u32,
    log: extern "C" fn(
        *mut c_void,
        CLogLevel,
        *const c_char,
        c_int,
        *const c_char,
        *const c_char,
        ...
    ),
    logv: *const c_void, /* va_list currently only in nightly */
    logt: extern "C" fn(
        *mut c_void,
        CLogLevel,
        *const CLogTopic,
        *const c_char,
        c_int,
        *const c_char,
        *const c_char,
        ...
    ),
    logtv: *const c_void, /* va_list currently only in nightly */
}

#[repr(C)]
pub struct CLog {
    iface: CInterface,
    level: CLogLevel,
}

pub fn new_impl(interface: *mut CInterface) -> LogImpl {
    let clevel = unsafe { (interface as *mut CLog).as_ref().unwrap().level };

    LogImpl {
        inner: Box::pin(interface as *mut CLog),
        level: LogLevel::try_from(clevel as u32).unwrap(),

        log: CLogImpl::log,
        logt: CLogImpl::logt,
    }
}

impl CLogImpl {
    fn log(
        this: &LogImpl,
        level: crate::interface::log::LogLevel,
        file: &str,
        line: i32,
        func: &str,
        args: std::fmt::Arguments,
    ) {
        let clevel = CLogLevel::try_from(level as u32).unwrap();
        let log_line = match args.as_str() {
            Some(s) => s,
            _ => return,
        };

        unsafe {
            let log = this
                .inner
                .as_ref()
                .downcast_ref::<*mut CLog>()
                .unwrap()
                .as_ref()
                .unwrap();
            let funcs = log.iface.cb.funcs as *const CLogMethods;
            ((*funcs).log)(
                log.iface.cb.data,
                clevel,
                c_string(file).as_ptr(),
                line,
                c_string(func).as_ptr(),
                c_string(log_line).as_ptr(),
            )
        };
    }

    fn logt(
        this: &LogImpl,
        level: LogLevel,
        topic: &crate::interface::log::LogTopic,
        file: &str,
        line: i32,
        func: &str,
        args: std::fmt::Arguments,
    ) {
        let clevel = CLogLevel::try_from(level as u32).unwrap();
        let topic_name = c_string(topic.topic.as_str());
        let ctopic = CLogTopic {
            version: 0,
            topic: topic_name.as_ptr(),
            level: CLogLevel::try_from(topic.level as u32).unwrap(),
            has_custom_level: topic.has_custom_level,
        };
        let log_line = match args.as_str() {
            Some(s) => s,
            _ => return,
        };

        unsafe {
            let log = this
                .inner
                .as_ref()
                .downcast_ref::<*mut CLog>()
                .unwrap()
                .as_ref()
                .unwrap();
            let funcs = log.iface.cb.funcs as *const CLogMethods;

            ((*funcs).logt)(
                log.iface.cb.data,
                clevel,
                &ctopic,
                c_string(file).as_ptr(),
                line,
                c_string(func).as_ptr(),
                c_string(log_line).as_ptr(),
            )
        };
    }
}
