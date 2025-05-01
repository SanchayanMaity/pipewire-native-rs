// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use pipewire_native::properties::Properties;

#[test]
fn test_properties() {
    let mut props = Properties::new();

    props.set("key1", format! {"{}", 1});

    assert_eq!(props.get("key1"), Some(&"1".to_string()));

    for (k, v) in props.dict() {
        assert_eq!(k, "key1");
        assert_eq!(v, "1");
    }

    assert_eq!(props.get_u32("key1"), Some(1u32));
    assert_eq!(props.get_bool("key1"), Some(true));
}
