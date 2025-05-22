// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use crate::{debug, default_topic, log, properties::Properties};

default_topic!(log::topic::CONF);

pub fn is_valid_name(name: &str) -> bool {
    name == "null" || name.ends_with(".conf")
}

pub fn load(prefix: Option<&str>, name: &str, properties: &mut Properties) -> std::io::Result<()> {
    debug!("Trying to load config file: {prefix:?}/{name}");
    Ok(())
}
