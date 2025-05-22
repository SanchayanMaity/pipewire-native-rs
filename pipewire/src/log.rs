// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

#[macro_export]
macro_rules! cstr {
    ($str:expr) => {
        ::std::ffi::CStr::from_bytes_with_nul(concat!($str, "\0").as_bytes()).unwrap()
    };
}

#[macro_export]
macro_rules! define_topic {
    ($name:ident, $topic:literal) => {
        pub static $name: ::std::sync::LazyLock<::pipewire_native_spa::interface::log::LogTopic> =
            ::std::sync::LazyLock::new(|| ::pipewire_native_spa::interface::log::LogTopic {
                topic: &crate::cstr!($topic),
                level: ::pipewire_native_spa::interface::log::LogLevel::Debug,
                has_custom_level: false,
            });
    };
}

pub(crate) mod topic {
    define_topic!(CONF, "pw.conf");
    define_topic!(CONTEXT, "pw.context");
}

#[macro_export]
macro_rules! default_topic {
    ($name:expr) => {
        static DEFAULT_TOPIC: ::std::sync::LazyLock<
            &::pipewire_native_spa::interface::log::LogTopic,
        > = ::std::sync::LazyLock::new(|| &*$name);
    };
}

#[macro_export]
macro_rules! log_topic {
    ($level:expr, $topic:expr, $($args:tt),+) => {
        let log = crate::GLOBAL_SUPPORT.get().unwrap().log();
        log.logt(
            $level,
            &$topic,
            &crate::cstr!(file!()),
            line!() as i32,
            crate::cstr!("TODO"),
            format_args!($($args),+),
        );
    };
}

#[macro_export]
macro_rules! log_default {
    ($level:expr, $($args:tt),+) => {
        crate::log_topic!($level, DEFAULT_TOPIC, $($args),+);
    };
}

#[macro_export]
macro_rules! error {
    ($($args:tt),+) => {
        crate::log_default!(::pipewire_native_spa::interface::log::LogLevel::Error, $($args),+);
    };
}

#[macro_export]
macro_rules! warn {
    ($($args:tt),+) => {
        crate::log_default!(::pipewire_native_spa::interface::log::LogLevel::Warn, $($args),+);
    };
}

#[macro_export]
macro_rules! info {
    ($($args:tt),+) => {
        crate::log_default!(::pipewire_native_spa::interface::log::LogLevel::Info, $($args),+);
    };
}

#[macro_export]
macro_rules! debug {
    ($($args:tt),+) => {
        crate::log_default!(::pipewire_native_spa::interface::log::LogLevel::Debug, $($args),+);
    };
}

#[macro_export]
macro_rules! trace {
    ($(args:tt),+) => {
        crate::log_default!(::pipewire_native_spa::interface::log::LogLevel::Trace, $($args),+);
    };
}
