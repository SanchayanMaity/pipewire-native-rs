// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::collections::HashMap;

pub mod interface;
pub mod param;
pub mod pod;
pub mod support;

pub type Dict = HashMap<String, String>;

pub fn atob(s: &String) -> bool {
    s == "true" || s == "1"
}
