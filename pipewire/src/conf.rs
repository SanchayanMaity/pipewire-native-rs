// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::path::{Path, PathBuf};

// FIXME: get via build configuration
const PIPEWIRE_CONFIG_DIR: &str = "/etc/pipewire";
const PIPEWIRE_CONFIG_DATA_DIR: &str = "/usr/share/pipewire";

use crate::{debug, default_topic, log, properties::Properties, trace};

default_topic!(log::topic::CONF);

fn is_valid_name(name: &str) -> bool {
    name == "null" || name.ends_with(".conf")
}

fn try_path(path: PathBuf) -> std::io::Result<PathBuf> {
    trace!("Trying path: {}", path.display());
    match path.try_exists() {
        Ok(true) => Ok(path),
        Ok(false) => Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Path does not exist: {}", path.display()),
        )),
        Err(e) => Err(e),
    }
}

fn get_abs_path(path: &Path) -> Option<std::io::Result<PathBuf>> {
    if path.is_absolute() {
        Some(try_path(path.to_path_buf()))
    } else {
        None
    }
}

fn get_envconf_path(path: &PathBuf) -> Option<std::io::Result<PathBuf>> {
    if let Ok(config_dir) = std::env::var("PIPEWIRE_CONFIG_DIR") {
        Some(try_path(PathBuf::from(config_dir).join(path)))
    } else {
        None
    }
}

fn get_homeconf_path(path: &PathBuf) -> Option<std::io::Result<PathBuf>> {
    if let Ok(xdg_home) = std::env::var("XDG_CONFIG_HOME") {
        let xdg_home_path = try_path(
            [&xdg_home, "pipewire"]
                .iter()
                .collect::<PathBuf>()
                .join(path),
        );
        if xdg_home_path.is_ok() {
            return Some(xdg_home_path);
        }
    }

    std::env::home_dir().map(|home| try_path(home.join(".config").join("pipewire").join(path)))
}

fn get_configdir_path(path: &PathBuf) -> std::io::Result<PathBuf> {
    try_path(PathBuf::from(PIPEWIRE_CONFIG_DIR).join(path))
}

fn get_configdatadir_path(path: &PathBuf) -> std::io::Result<PathBuf> {
    try_path(PathBuf::from(PIPEWIRE_CONFIG_DATA_DIR).join(path))
}

// Try to locate a file in some standard paths
fn get_config_path(prefix: Option<&str>, name: &str) -> std::io::Result<PathBuf> {
    let mut config_path = PathBuf::new();

    if let Some(prefix) = prefix {
        config_path.push(prefix);
    }

    config_path.push(name);

    if let Some(Ok(abs_path)) = get_abs_path(&config_path) {
        return Ok(abs_path);
    }

    if super::GLOBAL_SUPPORT.get().unwrap().no_config {
        debug!("User config disabled via global no-config");
        return get_configdatadir_path(&config_path);
    }

    if let Some(Ok(envconf_path)) = get_envconf_path(&config_path) {
        return Ok(envconf_path);
    }

    if let Some(Ok(home_path)) = get_homeconf_path(&config_path) {
        return Ok(home_path);
    }

    get_configdir_path(&config_path).or_else(|_| get_configdatadir_path(&config_path))
}

fn read_file(path: &PathBuf, properties: &mut Properties) -> std::io::Result<()> {
    debug!("Reading config file: {}", path.display());

    if let Ok(config) = std::fs::read(path) {
        match std::str::from_utf8(&config) {
            Ok(config_str) => {
                properties.update_string(config_str).map_err(|err| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Could not update properties from config: {err}"),
                    )
                })?;
                Ok(())
            }
            Err(e) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Config file is not valid UTF-8: {e}"),
            )),
        }
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Could not read config file: {}", path.display()),
        ))
    }
}

pub fn load(prefix: Option<&str>, name: &str, properties: &mut Properties) -> std::io::Result<()> {
    debug!("Trying to load config file: {prefix:?}/{name}");

    if !is_valid_name(name) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Invalid config file name: {name}"),
        ));
    }

    if name == "null" {
        debug!("Null config, nothing to do");
        return Ok(());
    }

    let path = get_config_path(prefix, name)?;

    if let Some(prefix) = prefix {
        properties.set("config.prefix", prefix.to_string());
    }
    properties.set("config.name", name.to_string());
    properties.set("config.path", path.display().to_string());

    read_file(&path, properties)?;

    debug!("Config loaded successfully from: {}", path.display());

    // TODO: <name>.d overrides

    Ok(())
}
