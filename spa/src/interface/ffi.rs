// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::ffi::{c_char, c_int, c_void};

/* spa_callbacks */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CCallbacks {
    pub funcs: *const c_void,
    pub data: *mut c_void,
}

/* spa_interface */
#[repr(C)]
pub struct CInterface {
    pub type_: *const c_char,
    pub version: u32,
    pub cb: CCallbacks,
}

/* spa_support */
#[repr(C)]
pub struct CSupport {
    pub type_: *mut c_char,
    pub data: *mut c_void,
}

/* spa_list */
#[repr(C)]
pub struct CList {
    pub next: *mut CList,
    pub prev: *mut CList,
}

/* spa_hook_list */
#[repr(C)]
pub struct CHookList {
    pub list: CList,
}

/* spa_hook */
#[repr(C)]
pub struct CHook {
    pub link: CList,
    pub cb: CCallbacks,
    pub removed: extern "C" fn(hook: *mut CHook),
    pub priv_: *mut c_void,
}

#[repr(C)]
pub struct CControlHooks {
    pub version: u32,
    pub before: extern "C" fn(data: *mut c_void),
    pub after: extern "C" fn(data: *mut c_void),
}

#[repr(C)]
pub struct CLoop {
    pub iface: CInterface,
}

#[repr(C)]
pub struct CSource {
    /*
     * this should be *mut, but mutable dererefence via dyn Any
     * (when used as CLoopImpl inner) does not work
     */
    pub loop_: *const CLoop,
    pub func: CSourceFunc,
    pub data: *mut c_void,
    pub fd: c_int,
    pub mask: u32,
    pub rmask: u32,
    pub priv_: *mut c_void,
}

pub type CSourceFunc = extern "C" fn(source: *mut CSource);
