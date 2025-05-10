// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use pipewire_native::properties::Properties;

#[test]
fn test_properties_simple() {
    let mut props = Properties::new();

    props.set("key1", format! {"{}", 1});

    assert_eq!(props.get("key1"), Some(&"1".to_string()));

    for (k, v) in props.iter() {
        assert_eq!(k, "key1");
        assert_eq!(v, "1");
    }

    assert_eq!(props.get_u32("key1"), Some(1u32));
    assert_eq!(props.get_bool("key1"), Some(true));
}

#[test]
fn test_properties_json() {
    let conf = r#"
    {
        "context.properties": {},
        "context.spa-libs": {
          "support.*": "support/libspa-support"
        },
        "context.objects": [
          {
            "factory": "spa-node-factory"
          }
        ]
    }
    "#;

    let props = Properties::new_string(conf).expect("config parsing should succeed");

    assert_eq!(props.dict().items().len(), 3);
    assert_eq!(props.get("context.properties"), Some(&"{}".to_string()));
    assert_eq!(
        props.get("context.objects"),
        Some(&r#"[{"factory":"spa-node-factory"}]"#.to_string())
    );
}
