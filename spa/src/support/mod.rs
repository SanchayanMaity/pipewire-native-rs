// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::sync::LazyLock;

use plugin::Plugin;

pub mod r#loop;
pub mod plugin;
pub mod system;

static SUPPORT: LazyLock<Plugin> = LazyLock::new(|| Plugin::new());

fn support() -> &'static Plugin {
    &*SUPPORT
}
