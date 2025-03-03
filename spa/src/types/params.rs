// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use pipewire_native_macros::EnumU32;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumU32)]
pub enum ParamType {
    Invalid,
    PropInfo,
    Props,
    EnumFormat,
    Format,
    Buffers,
    Meta,
    IO,
    EnumProfile,
    Profile,
    EnumPortConfig,
    PortConfig,
    EnumRoute,
    Route,
    Control,
    Latency,
    ProcessLatency,
    Tag,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumU32)]
pub enum PropInfo {
    Start,
    Id,
    Name,
    Type,
    Labels,
    Container,
    Params,
    Description,
}
