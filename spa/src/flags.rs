// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan
// SPDX-FileCopyrightText: Copyright (c) 2025 Sanchayan Maity

use bitflags::bitflags;

bitflags! {
    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Io: u32 {
        const IN  = (1 << 0);
        const OUT = (1 << 2);
        const ERR = (1 << 3);
        const HUP = (1 << 4);
    }
}

bitflags! {
    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Fd: u32 {
        const CLOEXEC             = (1 << 0);
        const NONBLOCK            = (1 << 1);
        const EVENT_SEMAPHORE     = (1 << 2);
        const TIMER_ABSTIME       = (1 << 3);
        const TIMER_CANCEL_ON_SET = (1 << 4);
    }
}
