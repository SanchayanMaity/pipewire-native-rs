// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::any::Any;

use crate::dict::Dict;

use super::ffi::CInterface;

pub const LOG_FACTORY: &str = "support.log";
pub const SYSTEM_FACTORY: &str = "support.system";
pub const CPU_FACTORY: &str = "support.cpu";

pub trait Interface: Any {
    unsafe fn make_native(&self) -> *mut CInterface;
    unsafe fn free_native(cpu: *mut CInterface)
    where
        Self: Sized;
}

pub struct InterfaceInfo {
    pub type_: String,
}

pub trait HandleFactory {
    /* Data fields */
    fn version(&self) -> u32;
    fn name(&self) -> &str;
    fn info(&self) -> Option<&Dict>;

    /* Methods */
    fn init(
        &self,
        info: Option<Dict>,
        support: &super::Support,
    ) -> std::io::Result<Box<dyn Handle>>;
    fn enum_interface_info(&self) -> Vec<InterfaceInfo>;
}

pub trait Handle {
    /* Data fields */
    fn version(&self) -> u32;

    /* Methods */
    fn get_interface(&self, type_: &str) -> Option<Box<dyn Interface>>;
}
