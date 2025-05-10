// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::collections::HashMap;

use crate::interface::{
    self,
    plugin::{Handle, HandleFactory, Interface, InterfaceInfo},
};

use super::system;

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

    fn get_interface(&self, type_: &str) -> Option<Box<dyn Interface>> {
        match type_ {
            interface::SYSTEM => Some(Box::new(system::new())),
            _ => None,
        }
    }

    fn clear(&mut self) {}
}
