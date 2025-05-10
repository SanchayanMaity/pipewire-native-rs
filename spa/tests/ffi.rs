// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use pipewire_native_spa::interface;
use pipewire_native_spa::interface::log::{LogImpl, LogLevel};
use pipewire_native_spa::interface::plugin::{Handle, HandleFactory};
use pipewire_native_spa::support::ffi;

#[test]
fn test_load_log() {
    let plugin_path = std::env::var("SPA_TEST_PLUGIN_PATH")
        .unwrap_or("/usr/lib64/spa-0.2/support/libspa-support.so".to_string());

    let plugin = ffi::plugin::load(plugin_path.into()).expect("Plugin loading should not fail");

    let log_factory = plugin
        .find_factory(interface::plugin::LOG_FACTORY)
        .expect("Should find log factory");

    let interfaces = log_factory.enum_interface_info();
    assert_eq!(interfaces.len(), 1);

    let mut log_handle = log_factory
        .init(None, None)
        .expect("Log factory loading should succeed");

    let log_iface = log_handle
        .get_interface(interface::LOG)
        .expect("Log factory should produce an interface");

    let log = (log_iface as Box<dyn std::any::Any>)
        .downcast::<LogImpl>()
        .expect("Log interface should be a LogImpl");

    log.log(
        LogLevel::Error,
        "file_name",
        123,
        "function_name",
        format_args!("log test: {}", "some format"),
    );

    log_handle.clear();
}
