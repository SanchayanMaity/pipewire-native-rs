// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use cc;

fn main() {
    cc::Build::new()
        .file("src/support/ffi/log.c")
        .compile("support-ffi");

    println!("cargo::rerun-if-changed=src/support/ffi/plugin.h");
    println!("cargo::rerun-if-changed=src/support/ffi/log.c");
}
