// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::collections::HashMap;

use pipewire_native_spa::Dict;

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

    /* Easier to provide read-only access to the dict, but maybe we should just implement Iter */
    pub fn dict(&self) -> &Dict {
        &self.dict
    }

    pub fn set(&mut self, key: &str, value: String) {
        self.dict.insert(key.to_string(), value);
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

    /* TODO: new_string() and update_string() need SPA JSON parsing */
}
