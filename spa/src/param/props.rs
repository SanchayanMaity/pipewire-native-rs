// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use pipewire_native_macros::EnumU32;

use crate::pod::types::ObjectType;

use super::ParamObject;

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

impl ParamObject for PropInfo {
    const TYPE: ObjectType = ObjectType::PropInfo;
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumU32)]
pub enum Prop {
    Start,
    Unknown,

    StartDevice = 0x100,
    Device,
    DeviceName,
    DeviceFd,
    Card,
    CardName,
    MinLatency,
    MaxLatency,
    Periods,
    PeriodSize,
    PeriodEvent,
    Live,
    Rate,
    Quality,
    BluetoothAudioCodec,
    BluetoothOffloadActive,

    StartAudio = 0x10000,
    WaveType,
    Frequency,
    Volume,
    Mute,
    PatternType,
    DitherType,
    Truncate,
    ChannelVolumes,
    VolumeBase,
    VolumeStep,
    ChannelMap,
    MonitorMute,
    MonitorVolumes,
    LatencyOffsetNsec,
    SoftMute,
    SoftVolumes,
    Iec958Codecs,
    VolumeRampSamples,
    VolumeRampStepSamples,
    VolumeRampTime,
    VolumeRampStepTime,
    VolumeRampScale,

    StartVideo = 0x20000,
    Brightness,
    Contrast,
    Saturation,
    Hue,
    Gamma,
    Exposure,
    Gain,
    Sharpness,

    StartOther = 0x80000,
    Params,

    StartCustom = 0x1000000,
}

impl ParamObject for Prop {
    const TYPE: ObjectType = ObjectType::Props;
}
