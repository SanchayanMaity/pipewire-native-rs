// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

impl From<ParamType> for u32 {
    fn from(value: ParamType) -> Self {
        value as u32
    }
}

impl TryFrom<u32> for ParamType {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Invalid),
            1 => Ok(Self::PropInfo),
            2 => Ok(Self::Props),
            3 => Ok(Self::EnumFormat),
            4 => Ok(Self::Format),
            5 => Ok(Self::Buffers),
            6 => Ok(Self::Meta),
            7 => Ok(Self::IO),
            8 => Ok(Self::EnumProfile),
            9 => Ok(Self::Profile),
            10 => Ok(Self::EnumPortConfig),
            11 => Ok(Self::PortConfig),
            12 => Ok(Self::EnumRoute),
            13 => Ok(Self::Route),
            14 => Ok(Self::Control),
            15 => Ok(Self::Latency),
            16 => Ok(Self::ProcessLatency),
            17 => Ok(Self::Tag),
            _ => Err(()),
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

impl From<PropInfo> for u32 {
    fn from(value: PropInfo) -> Self {
        value as u32
    }
}

impl TryFrom<u32> for PropInfo {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Start),
            1 => Ok(Self::Id),
            2 => Ok(Self::Name),
            3 => Ok(Self::Type),
            4 => Ok(Self::Labels),
            5 => Ok(Self::Container),
            6 => Ok(Self::Params),
            7 => Ok(Self::Description),
            _ => Err(()),
        }
    }
}
