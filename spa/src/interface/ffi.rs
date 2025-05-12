// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::ffi::{c_char, c_void};

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
