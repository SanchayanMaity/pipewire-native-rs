// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use pipewire_native::support;
use pipewire_native_spa::interface::{self, plugin};

#[test]
fn test_load_support_handle() {
    let support = support::get();

    let _ = support
        .load_interface(plugin::LOG_FACTORY, interface::LOG, None)
        .expect("failed to load log interface");
}
