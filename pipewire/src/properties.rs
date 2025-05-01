// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::collections::HashMap;

use pipewire_native_spa::Dict;
use tinyjson::{JsonParseError, JsonValue};

#[derive(Clone, Debug)]
pub struct Properties {
    dict: Dict,
}

impl Properties {
    pub fn new() -> Self {
        Self {
            dict: HashMap::new(),
        }
    }

    pub fn new_dict(dict: Dict) -> Self {
        Self { dict }
    }

    pub fn new_string(args: &str) -> Result<Self, String> {
        let mut p = Self::new();

        match p.update_string(args) {
            Ok(_) => Ok(p),
            Err(e) => Err(e),
        }
    }

    /* Easier to provide read-only access to the dict, but maybe we should just implement Iter */
    pub fn dict(&self) -> &Dict {
        &self.dict
    }

    pub fn set(&mut self, key: &str, value: String) {
        self.dict.insert(key.to_string(), value);
    }

    pub fn unset(&mut self, key: &str) -> Option<String> {
        self.dict.remove(key)
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.dict.get(key)
    }

    pub fn get_u32(&self, key: &str) -> Option<u32> {
        self.get(key).and_then(|v| u32::from_str_radix(v, 10).ok())
    }

    pub fn get_i32(&self, key: &str) -> Option<i32> {
        self.get(key).and_then(|v| i32::from_str_radix(v, 10).ok())
    }

    pub fn get_u64(&self, key: &str) -> Option<u64> {
        self.get(key).and_then(|v| u64::from_str_radix(v, 10).ok())
    }

    pub fn get_i64(&self, key: &str) -> Option<i64> {
        self.get(key).and_then(|v| i64::from_str_radix(v, 10).ok())
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
        for k in keys {
            if let Some(v) = dict.get(k) {
                self.set(k, v.clone());
            }
        }
    }

    pub fn update_ignore(&mut self, dict: &Dict, ignore: Vec<&str>) {
        for (k, v) in dict {
            if ignore.contains(&k.as_ref()) {
                continue;
            }

            self.set(k, v.clone());
        }
    }
}
