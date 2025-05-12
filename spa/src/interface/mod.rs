// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::ffi::{c_void, CStr, CString};

use ffi::{CInterface, CSupport};
use log::LogImpl;
use r#loop::LoopImpl;
use system::SystemImpl;

use crate::support;

pub mod ffi;
pub mod log;
pub mod r#loop;
pub mod plugin;
pub mod system;

/* Well-known interface names */
pub const LOG: &str = "Spa:Pointer:Interface:Log";
pub const LOOP: &str = "Spa:Pointer:Interface:Loop";
pub const SYSTEM: &str = "Spa:Pointer:Interface:System";

pub struct Support {
    log: Option<LogImpl>,
    system: Option<SystemImpl>,
    loop_: Option<LoopImpl>,
    /* We keep a C-compatible array that won't get moved around, so we can reliably pass it on to
     * plugins */
    all: Vec<CSupport>,
}

impl Support {
    pub fn new() -> Support {
        Support {
            log: None,
            system: None,
            loop_: None,
            /* Reserve enough space so the array is always valid */
            all: Vec::with_capacity(16),
        }
    }

    pub fn all(&self) -> &Vec<CSupport> {
        &self.all
    }

    fn add_or_update(&mut self, name: &str, data: *mut CInterface) {
        for s in self.all.iter_mut() {
            let type_ = unsafe { CStr::from_ptr(s.type_).to_str() };
            if type_ == Ok(name) {
                s.data = data as *mut c_void;
                return;
            }
        }

        self.all.push(CSupport {
            type_: support::ffi::c_string(name).into_raw(),
            data: data as *mut c_void,
        });
    }

    pub fn set_log(&mut self, log: LogImpl) {
        self.add_or_update(LOG, support::ffi::log::make_native(&log));
        self.log = Some(log);
    }
}

impl Drop for Support {
    fn drop(&mut self) {
        for s in self.all.iter_mut() {
            let type_ = unsafe { CString::from_raw(s.type_) };
            match type_.to_str().unwrap() {
                LOG => support::ffi::log::free_native(s.data as *mut CInterface),
                _ => unreachable!(),
            }
        }
    }
}
