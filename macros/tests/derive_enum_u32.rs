// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use pipewire_native_macros::EnumU32;

#[repr(u32)]
#[derive(Debug, Eq, PartialEq, EnumU32)]
enum Test {
    Start = 0x1000,
    First,
    Second,
    Last,
}

#[test]
fn test_derive_enum_u32() {
    assert_eq!(u32::from(Test::Start), 0x1000);
    assert_eq!(Test::try_from(0x1001).unwrap(), Test::First);
    assert_eq!(Test::try_from(0x1002).unwrap(), Test::Second);
    assert_eq!(Test::try_from(0x1003).unwrap(), Test::Last);
    assert_eq!(Test::try_from(1), Err(()));
}
