// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::collections::HashMap;

use pipewire_native_spa::atob;
use pipewire_native_spa::dict::Dict;
use tinyjson::{JsonParseError, JsonValue};

#[derive(Clone, Debug)]
pub struct Properties {
    // TODO: Make a Vec<spa::dict::ITem> here, so we can easily construct a Dict view of properties
    // at runtime, a-la pw_properties vs. spa_dict
    map: HashMap<String, String>,
}

impl Default for Properties {
    fn default() -> Self {
        Self::new()
    }
}

pub fn parse_bool(value: &String) -> bool {
    atob(value)
}

impl Properties {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn new_dict(dict: &Dict) -> Self {
        let mut map = HashMap::new();

        for (k, v) in dict.items() {
            map.insert(k.to_string(), v.to_string());
        }

        Self { map }
    }

    pub fn new_string(args: &str) -> Result<Self, String> {
        let mut p = Self::new();

        match p.update_string(args) {
            Ok(_) => Ok(p),
            Err(e) => Err(e),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.map.iter()
    }

    pub fn dict(&self) -> Dict {
        Dict::new(
            self.map
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect::<Vec<(String, String)>>(),
        )
    }

    pub fn set(&mut self, key: &str, value: String) {
        self.map.insert(key.to_string(), value);
    }

    pub fn unset(&mut self, key: &str) -> Option<String> {
        self.map.remove(key)
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }

    pub fn get_u32(&self, key: &str) -> Option<u32> {
        self.get(key).and_then(|v| v.parse::<u32>().ok())
    }

    pub fn get_i32(&self, key: &str) -> Option<i32> {
        self.get(key).and_then(|v| v.parse::<i32>().ok())
    }

    pub fn get_u64(&self, key: &str) -> Option<u64> {
        self.get(key).and_then(|v| v.parse::<u64>().ok())
    }

    pub fn get_i64(&self, key: &str) -> Option<i64> {
        self.get(key).and_then(|v| v.parse::<i64>().ok())
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.get(key).map(pipewire_native_spa::atob)
    }

    pub fn update_string(&mut self, args: &str) -> Result<u32, String> {
        let parsed: JsonValue = args.parse().map_err(|e: JsonParseError| e.to_string())?;

        if !parsed.is_object() {
            return Ok(0);
        }

        let map = match parsed {
            JsonValue::Object(m) => m,
            _ => return Ok(0),
        };

        let mut count = 0;

        for (k, v) in map {
            if v.is_null() {
                self.unset(k.as_ref());
            } else {
                let old_v = self.get(k.as_ref());
                let value = v
                    .stringify()
                    .expect("parsed value should convert back to a String");

                if old_v == Some(&value) {
                    /* Unchanged */
                    continue;
                }

                self.set(k.as_ref(), value);
                count += 1;
            }
        }

        Ok(count)
    }

    pub fn update_keys(&mut self, dict: &Dict, keys: Vec<&str>) {
        for (k, v) in dict.items() {
            if keys.contains(&k) {
                self.set(k, v.to_string());
            }
        }
    }

    pub fn update_ignore(&mut self, dict: &Dict, ignore: Vec<&str>) {
        for (k, v) in dict.items() {
            if ignore.contains(&k) {
                continue;
            }

            self.set(k, v.to_string());
        }
    }
}
