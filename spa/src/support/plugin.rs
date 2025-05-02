// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::sync::LazyLock;

use crate::interface::system;

use super::system::System;

static SYSTEM: LazyLock<Box<dyn system::System + Send + Sync>> =
    LazyLock::new(|| Box::new(System::new()));

pub fn system() -> &'static dyn system::System {
    &**SYSTEM
}
