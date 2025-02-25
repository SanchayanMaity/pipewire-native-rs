// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::ffi::c_void;
use std::os::fd::RawFd;

// spa/utils/type.h: Basic SPA_TYPE_*
#[repr(u32)]
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

impl TryFrom<u32> for Type {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Start),
            1 => Ok(Self::None),
            2 => Ok(Self::Bool),
            3 => Ok(Self::Id),
            4 => Ok(Self::Int),
            5 => Ok(Self::Long),
            6 => Ok(Self::Float),
            7 => Ok(Self::Double),
            8 => Ok(Self::String),
            9 => Ok(Self::Bytes),
            10 => Ok(Self::Rectangle),
            11 => Ok(Self::Fraction),
            12 => Ok(Self::Bitmap),
            13 => Ok(Self::Array),
            14 => Ok(Self::Struct),
            15 => Ok(Self::Object),
            16 => Ok(Self::Sequence),
            17 => Ok(Self::Pointer),
            18 => Ok(Self::Fd),
            19 => Ok(Self::Choice),
            20 => Ok(Self::Pod),
            _ => Err(()),
        }
    }
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Choice<T> {
    None(T),
    Range { default: T, min: T, max: T },
    Step { default: T, min: T, max: T, step: T },
    Enum { default: T, alternatives: Vec<T> },
    Flags { default: T, flags: T },
}

pub mod choice {
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum Flags {
        None = 0,
    }
}
