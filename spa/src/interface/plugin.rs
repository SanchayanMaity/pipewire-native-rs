// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

pub trait Interface {}

pub trait Handle {
    const VERSION: u32;

    fn get_interface<T: Interface>(&self, type_: &str) -> Option<&'static T>;
    fn clear(&self);
}
