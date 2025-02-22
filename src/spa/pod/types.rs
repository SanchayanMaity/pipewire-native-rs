// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::ffi::c_void;
use std::os::fd::RawFd;

// spa/utils/type.h: Basic SPA_TYPE_*
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Type {
    Start = 0,
    None,
    Bool,
    Id,
    Int,
    Long,
    Float,
    Double,
    String,
    Bytes,
    Rectangle,
    Fraction,
    Bitmap,
    Array,
    Struct,
    Object,
    Sequence,
    Pointer,
    Fd,
    Choice,
    Pod,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Id(pub u32);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Pointer {
    pub type_: Type,
    pub ptr: *const c_void,
}

// We can't directly use RawFd because it conflicts with i32 (being a type alias for it)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Fd(pub RawFd);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Fraction {
    pub num: u32,
    pub denom: u32,
}
