// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use pipewire_native_macros::EnumU32;

use crate::pod::types::ObjectType;

use super::ParamObject;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumU32)]
pub enum Buffers {
    Start,
    Buffers,
    Blocks,
    Size,
    Stride,
    Align,
    DataType,
    MetaType,
}

impl ParamObject for Buffers {
    const TYPE: ObjectType = ObjectType::ParamBuffers;
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumU32)]
pub enum Meta {
    Start,
    Type,
    Size,
}

impl ParamObject for Meta {
    const TYPE: ObjectType = ObjectType::ParamMeta;
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumU32)]
pub enum Io {
    Start,
    Id,
    Size,
}

impl ParamObject for Io {
    const TYPE: ObjectType = ObjectType::ParamIo;
}
