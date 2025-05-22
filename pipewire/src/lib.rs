// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::sync::OnceLock;

use pipewire_native_spa as spa;

use properties::Properties;
use support::Support;

pub mod conf;
pub mod context;
pub mod keys;
pub mod log;
pub mod properties;

mod support;
mod utils;

pub(crate) static GLOBAL_SUPPORT: OnceLock<Support> = OnceLock::new();

pub fn init() {
    GLOBAL_SUPPORT.get_or_init(|| {
        let mut support = Support::new();

        // First, initialise logging
        let mut log_info = Properties::new();
        log_info.set(
            spa::interface::log::LEVEL,
            if support.no_color {
                "false".to_string()
            } else {
                utils::read_env_string("PIPEWIRE_LOG_COLOR", "true")
            },
        );
        log_info.set(
            spa::interface::log::TIMESTAMP,
            utils::read_env_string("PIPEWIRE_LOG_TIMESTAMP", "true"),
        );
        log_info.set(
            spa::interface::log::LINE,
            utils::read_env_string("PIPEWIRE_LOG_LINE", "true"),
        );
        let _ = std::env::var("PIPEWIRE_LOG").map(|v| {
            log_info.set(spa::interface::log::FILE, v);
        });
        let _ = std::env::var("PIPEWIRE_DEBUG").map(|v| {
            // FIXME: parse the level first
            log_info.set(spa::interface::log::LEVEL, v);
        });

        // TODO: Check for/load the systemd logger if PIPEWIRE_SYSTEMD is set
        support
            .load_interface(
                spa::interface::plugin::LOG_FACTORY,
                spa::interface::LOG,
                Some(&log_info),
            )
            .expect("failed to load log interface");

        // Next, load CPU support
        let mut cpu_info = Properties::new();
        let _ = std::env::var("PIPEWIRE_CPU").map(|v| {
            cpu_info.set(spa::interface::cpu::FORCE, v);
        });
        let _ = std::env::var("PIPEWIRE_VN").map(|v| {
            cpu_info.set(spa::interface::cpu::VM, v);
        });

        support
            .load_interface(
                spa::interface::plugin::CPU_FACTORY,
                spa::interface::CPU,
                Some(&cpu_info),
            )
            .expect("failed to load CPU interface");

        support.init_log();

        // TODO: Load i18n interface
        support
    });
}
