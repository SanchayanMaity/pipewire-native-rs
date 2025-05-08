// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::{collections::HashMap, sync::LazyLock};

use crate::interface::{
    self,
    plugin::{Handle, HandleFactory, Interface, InterfaceInfo},
    system::SystemImpl,
};

use super::system;

static SYSTEM: LazyLock<SystemImpl> = LazyLock::new(|| system::new());

pub struct Plugin {}

pub struct PluginHandle {}

impl Plugin {
    pub fn new() -> Self {
        Self {}
    }
}

impl HandleFactory for Plugin {
    fn version(&self) -> u32 {
        0
    }

    fn name(&self) -> String {
        "rust-support".to_string()
    }

    fn info(&self) -> crate::Dict {
        HashMap::new()
    }

    fn init(
        &self,
        _: Option<crate::Dict>,
        _: Option<interface::Support>,
    ) -> std::io::Result<impl Handle> {
        Ok(PluginHandle {})
    }

    fn enum_interface_info(&self) -> Vec<InterfaceInfo> {
        vec![InterfaceInfo {
            type_: interface::SYSTEM.to_string(),
        }]
    }
}

impl Handle for PluginHandle {
    fn version(&self) -> u32 {
        0
    }

    fn get_interface<T: Interface>(&self, type_: &str) -> Option<&'static T> {
        match type_ {
            interface::SYSTEM => unsafe { Some(&*(&*SYSTEM as *const dyn Interface as *const T)) },
            _ => None,
        }
    }

    fn clear(&mut self) {}
}
