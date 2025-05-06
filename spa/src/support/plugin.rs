// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::sync::LazyLock;

use crate::interface::{
    self,
    plugin::{Handle, Interface},
    system::SystemImpl,
};

use super::system;

static SYSTEM: LazyLock<SystemImpl> = LazyLock::new(|| system::System::new());

pub struct Plugin {}

impl Plugin {
    pub fn new() -> Self {
        Self {}
    }
}

impl Handle for Plugin {
    const VERSION: u32 = 0;

    fn get_interface<T: Interface>(&self, type_: &str) -> Option<&'static T> {
        match type_ {
            interface::SYSTEM => unsafe { Some(&*(&*SYSTEM as *const dyn Interface as *const T)) },
            _ => None,
        }
    }

    fn clear(&self) {}
}
