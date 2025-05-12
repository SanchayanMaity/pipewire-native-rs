// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::{
    any::Any,
    ffi::{c_char, c_int, c_void, CStr},
    pin::Pin,
};

use pipewire_native_macros::EnumU32;

use crate::interface::log::{LogImpl, LogLevel, LogTopic};

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

extern "C" {
    fn c_log_from_impl(impl_: *const c_void, level: CLogLevel) -> *mut CLog;
    fn c_log_free(log: *mut CLog);
}

#[no_mangle]
pub extern "C" fn rust_logt(
    impl_: *mut c_void,
    level: CLogLevel,
    topic: *const CLogTopic,
    file: *const c_char,
    line: i32,
    func: *const c_char,
    log: *const c_char,
) {
    let log_impl: Pin<Box<LogImpl>> =
        unsafe { Box::into_pin(Box::from_raw(impl_ as *mut LogImpl)) };
    let level = LogLevel::try_from(level as u32).unwrap();
    let file = unsafe { CStr::from_ptr(file).to_str().unwrap() };
    let func = unsafe { CStr::from_ptr(func).to_str().unwrap() };
    let log = unsafe { CStr::from_ptr(log).to_str().unwrap() };

    if topic == std::ptr::null() {
        log_impl.log(level, file, line, func, format_args!("{}", log));
    } else {
        let topic = unsafe {
            let c_topic = &*topic;
            LogTopic {
                topic: CStr::from_ptr(c_topic.topic).to_str().unwrap().to_string(),
                level: LogLevel::try_from(c_topic.level as u32).unwrap(),
                has_custom_level: c_topic.has_custom_level,
            }
        };
        log_impl.logt(level, &topic, file, line, func, format_args!("{}", log));
    }
}

pub fn make_native(log: &LogImpl) -> *mut CInterface {
    unsafe {
        let inner = Pin::into_inner_unchecked(log.inner.as_ref());
        let level = CLogLevel::try_from(log.level as u32).unwrap();

        c_log_from_impl(inner as *const dyn Any as *const c_void, level) as *mut CInterface
    }
}

pub fn free_native(c_log: *mut CInterface) {
    unsafe {
        c_log_free(c_log as *mut CLog);
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
