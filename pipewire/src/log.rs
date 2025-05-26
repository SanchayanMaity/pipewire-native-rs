// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use pipewire_native_spa as spa;

#[macro_export]
macro_rules! cstr {
    ($str:expr) => {
        ::std::ffi::CStr::from_bytes_with_nul(concat!($str, "\0").as_bytes()).unwrap()
    };
}

#[macro_export]
macro_rules! define_topic {
    ($name:ident, $topic:literal) => {
        pub static $name: (
            ::std::sync::OnceLock<::pipewire_native_spa::interface::log::LogTopic>,
            &str,
        ) = (::std::sync::OnceLock::new(), concat!($topic, "\0"));
    };
}

pub(crate) mod topic {
    use pipewire_native_spa as spa;

    define_topic!(CONF, "pw.conf");
    define_topic!(CONTEXT, "pw.context");
    define_topic!(SUPPORT, "pw.support");

    pub fn init(levels: &Vec<(String, spa::interface::log::LogLevel)>) {
        for topic in [&CONF, &CONTEXT, &SUPPORT] {
            // TODO: implement glob matching
            let pattern = levels.iter().find(|v| v.0 == topic.1);
            let (level, has_custom_level) = match pattern {
                Some(&(ref pat, level)) if pat == topic.1 => (level, true),
                _ => (spa::interface::log::LogLevel::Warn, false),
            };

            let _ = topic.0.set(spa::interface::log::LogTopic {
                topic: &std::ffi::CStr::from_bytes_with_nul(topic.1.as_bytes()).unwrap(),
                level,
                has_custom_level,
            });
        }
    }
}

#[macro_export]
macro_rules! default_topic {
    ($name:expr) => {
        static DEFAULT_TOPIC: ::std::sync::LazyLock<
            &::pipewire_native_spa::interface::log::LogTopic,
        > = ::std::sync::LazyLock::new(|| $name.0.get().unwrap());
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

pub(super) fn parse_levels(levels: Option<&str>) -> Vec<(String, spa::interface::log::LogLevel)> {
    let mut result = Vec::new();

    if levels.is_none() {
        result.push(("".to_string(), spa::interface::log::LogLevel::Warn));
        return result;
    }

    let levels = levels.unwrap().split(',').collect::<Vec<_>>();

    for pattern in levels {
        let parts = pattern.split(':').collect::<Vec<_>>();

        let (name, level_str) = match parts.len() {
            1 => ("".to_string(), parts[0]),
            2 => (parts[0].to_string(), parts[1]),
            _ => {
                continue;
            }
        };

        let level = spa::interface::log::LogLevel::try_from(level_str);

        if let Ok(level) = level {
            result.push((name.to_string(), level));
        } else {
            continue;
        }
    }

    result
}
