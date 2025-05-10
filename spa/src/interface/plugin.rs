// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::any::Any;

use crate::Dict;

pub const LOG_FACTORY: &str = "support.log";

pub trait Interface: Any {}

pub struct InterfaceInfo {
    pub type_: String,
}

pub trait HandleFactory {
    /* Data fields */
    fn version(&self) -> u32;
    fn name(&self) -> String;
    fn info(&self) -> Dict;

    /* Methods */
    fn init(
        &self,
        info: Option<Dict>,
        support: Option<super::Support>,
    ) -> std::io::Result<impl Handle>;
    fn enum_interface_info(&self) -> Vec<InterfaceInfo>;
}

pub trait Handle {
    /* Data fields */
    fn version(&self) -> u32;

    /* Methods */
    fn get_interface(&self, type_: &str) -> Option<Box<dyn Interface>>;
    fn clear(&mut self);
}
