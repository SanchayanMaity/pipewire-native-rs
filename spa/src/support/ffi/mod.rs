// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::ffi::CString;

pub mod log;
pub mod plugin;

/* TODO: can we avoid an allocation here? */
fn c_string(s: &str) -> CString {
    CString::new(s).expect("&str should wrap as CString")
}
