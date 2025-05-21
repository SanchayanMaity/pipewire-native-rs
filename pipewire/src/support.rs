// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use pipewire_native_spa as spa;
use pipewire_native_spa::dict::Dict;
use pipewire_native_spa::interface::plugin::{self, Handle};
use pipewire_native_spa::support::ffi;

use crate::properties;

pub struct Support {
    do_dlclose: bool,
    no_color: bool,
    no_config: bool,

    plugin_dirs: Vec<String>,
    support_lib: String,

    inner: Mutex<Inner>,
}

struct Inner {
    plugins: HashMap<String, ffi::plugin::Plugin>,
    factories: HashMap<String, Box<dyn plugin::HandleFactory>>,
    support: spa::interface::Support,
}

fn read_env_bool(var: &str) -> bool {
    std::env::var(var)
        .map(|v| properties::parse_bool(&v))
        .unwrap_or(false)
}

const SUPPORTLIB: &str = "support/libspa-support.so";

static SUPPORT: OnceLock<Support> = OnceLock::new();

pub fn get() -> &'static Support {
    SUPPORT.get_or_init(|| {
        let do_dlclose = read_env_bool("PIPEWIRE_DLCLOSE");
        let no_color = read_env_bool("NO_COLOR");
        let no_config = read_env_bool("PIPEWIRE_NO_CONFIG");
        /* FIXME: unhardcode */
        let plugin_dir = std::env::var("SPA_PLUGIN_DIR")
            .unwrap_or("/usr/lib64/spa-0.2".to_string())
            .split(':')
            .map(|s| s.to_string())
            .collect();
        let support_lib = std::env::var("SPA_SUPPORT_LIB").unwrap_or(SUPPORTLIB.to_string());

        Support {
            do_dlclose,
            no_config,
            no_color,
            plugin_dirs: plugin_dir,
            support_lib,
            inner: Mutex::new(Inner {
                plugins: HashMap::new(),
                factories: HashMap::new(),
                support: spa::interface::Support::new(),
            }),
        }
    })
}

impl Support {
    pub fn load_spa_handle(
        &self,
        lib: Option<&str>,
        factory_name: &str,
        info: Option<Dict>,
    ) -> std::io::Result<Box<dyn Handle>> {
        let mut inner = self.inner.lock().unwrap();
        let lib = lib.unwrap_or(&self.support_lib);

        let mut lib_name = "".to_string();
        let mut plugin = None;

        for dir in self.plugin_dirs.iter() {
            let mut path = PathBuf::from(dir);
            path.push(lib);

            lib_name = path.to_string_lossy().to_string();

            match inner.plugins.get(&lib_name) {
                Some(p) => {
                    plugin = Some(p);
                    break;
                }
                None => match ffi::plugin::load(&path) {
                    Ok(p) => {
                        inner.plugins.insert(lib_name.to_string(), p);
                        plugin = inner.plugins.get(&lib_name);
                        break;
                    }
                    Err(_) => {
                        // Try the next directory
                        continue;
                    }
                },
            };
        }

        let plugin = plugin.ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Plugin not found: {}", lib),
            )
        })?;

        let factory_key = format!("{}/{}", lib_name, factory_name);
        let factory = match inner.factories.get(&factory_key) {
            Some(factory) => factory,
            None => match plugin.find_factory(factory_name) {
                Some(factory) => {
                    inner.factories.insert(factory_key.clone(), factory);
                    inner.factories.get(&factory_key).unwrap()
                }
                None => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        format!("Factory not found: {}", factory_name),
                    ))
                }
            },
        };

        let handle = factory.init(info, &inner.support).map_err(|_| {
            std::io::Error::other(format!("Failed to initialize factory: {}", factory_name))
        })?;

        Ok(handle)
    }
}

unsafe impl Send for Support {}
unsafe impl Sync for Support {}
