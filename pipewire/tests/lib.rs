// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use pipewire_native::{self as pipewire, properties::Properties};

#[test]
fn test_lib() {
    pipewire::init();

    let context = pipewire::context::Context::new(Properties::new());

    assert!(context.is_ok());
}
