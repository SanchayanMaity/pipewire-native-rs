// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::{ffi::CStr, sync::LazyLock};

use crate::{conf, keys, properties::Properties};

pub struct Context {
    properties: Properties,
}

static PROCESS_NAME: LazyLock<String> = LazyLock::new(|| {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(name_part) = exe.file_name() {
            if let Some(name_str) = name_part.to_str() {
                return name_str.to_string();
            }
        }
    }

    // Fallback to the process ID if we can't get the name
    format!("pid-{}", std::process::id())
});

impl Context {
    pub fn new(properties: Properties) -> std::io::Result<Self> {
        let mut this = Context { properties };

        this.set_default_properties();
        this.load_conf()?;

        Ok(this)
    }

    fn load_conf(&mut self) -> std::io::Result<()> {
        let conf_prefix = std::env::var("PIPEWIRE_CONFIG_PREFIX")
            .ok()
            .or_else(|| self.properties.get(keys::CONFIG_PREFIX).cloned());

        let conf_name = std::env::var("PIPEWIRE_CONFIG_NAME")
            .ok()
            .or_else(|| self.properties.get(keys::CONFIG_NAME).cloned())
            .and_then(|s| if s == "client-rt.conf" { None } else { Some(s) })
            .unwrap_or("client.conf".to_string());

        conf::load(conf_prefix.as_deref(), &conf_name, &mut self.properties)?;

        // TODO: overrides

        Ok(())
    }

    fn set_default_properties(&mut self) {
        if None == self.properties.get(keys::APP_NAME) {
            self.properties.set(keys::APP_NAME, PROCESS_NAME.clone());
        };
        if None == self.properties.get(keys::APP_PROCESS_BINARY) {
            self.properties.set(keys::APP_NAME, PROCESS_NAME.clone());
        };
        if None == self.properties.get(keys::APP_LANGUAGE) {
            if let Ok(lang) = std::env::var("LANG") {
                self.properties.set(keys::APP_LANGUAGE, lang);
            }
        };
        if None == self.properties.get(keys::APP_PROCESS_ID) {
            self.properties
                .set(keys::APP_PROCESS_ID, std::process::id().to_string());
        };
        if None == self.properties.get(keys::APP_PROCESS_USER) {
            if let Some(user) = unsafe {
                libc::getpwuid(libc::getuid())
                    .as_ref()
                    .map(|p| CStr::from_ptr(p.pw_name).to_string_lossy().to_string())
            } {
                self.properties.set(keys::APP_PROCESS_USER, user);
            }
        };
        if None == self.properties.get(keys::APP_PROCESS_HOST) {
            let mut name: [u8; 256] = [0; 256];
            unsafe { libc::gethostname(name.as_mut_ptr() as *mut i8, name.len() as libc::size_t) };
            if let Ok(hostname) = CStr::from_bytes_until_nul(&name) {
                self.properties.set(
                    keys::APP_PROCESS_HOST,
                    hostname.to_string_lossy().to_string(),
                );
            }
        };
        if None == self.properties.get(keys::APP_PROCESS_SESSION_ID) {
            if let Ok(session_id) = std::env::var("XDG_SESSION_ID") {
                self.properties
                    .set(keys::APP_PROCESS_SESSION_ID, session_id);
            }
        };
        if None == self.properties.get(keys::WINDOW_X11_DISPLAY) {
            if let Ok(display) = std::env::var("DISPLAY") {
                self.properties.set(keys::WINDOW_X11_DISPLAY, display);
            }
        };
    }
}
