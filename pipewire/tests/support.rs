// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use pipewire_native::support;
use pipewire_native_spa::interface::plugin;

#[test]
fn test_load_support_handle() {
    let support = support::get();

    let _ = support
        .load_spa_handle(None, plugin::LOG_FACTORY, None)
        .expect("failed to load spa handle");
}
