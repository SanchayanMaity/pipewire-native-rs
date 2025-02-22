// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::ffi::c_void;

use pipewire_native::spa::pod::builder::Builder;
use pipewire_native::spa::pod::types::{Fd, Fraction, Id, Pointer, Rectangle, Type};
use pipewire_native::spa::pod::Pod;

use libspa::pod as spa_pod;
use libspa::sys as spa_sys;
use libspa::utils as spa_utils;

#[test]
fn test_pod_builder() {
    let mut buf = [0u8; 1024];
    let builder = Builder::new(&mut buf);
    let res = builder
        .none()
        .bool(true)
        .id(Id(1))
        .int(2)
        .long(3)
        .float(4.0)
        .double(5.0)
        .string("hello")
        .bytes(&[6, 7, 8, 9])
        .pointer(Type::Int, 0xdeadc0de as *const c_void)
        .fd(-1)
        .rectangle(1920, 1080)
        .fraction(30001, 1)
        .build()
        .unwrap();

    let mut sbuf = Vec::with_capacity(1024);
    let mut sbuilder = spa_pod::builder::Builder::new(&mut sbuf);
    sbuilder.add_none().unwrap();
    sbuilder.add_bool(true).unwrap();
    sbuilder.add_id(spa_utils::Id(1)).unwrap();
    sbuilder.add_int(2).unwrap();
    sbuilder.add_long(3).unwrap();
    sbuilder.add_float(4.0).unwrap();
    sbuilder.add_double(5.0).unwrap();
    sbuilder.add_string("hello").unwrap();
    sbuilder.add_bytes(&[6, 7, 8, 9]).unwrap();
    unsafe {
        sbuilder
            .add_pointer(
                spa_utils::Id(spa_sys::SPA_TYPE_Int),
                0xdeadc0de as *const c_void,
            )
            .unwrap();
    }
    sbuilder.add_fd(-1).unwrap();
    sbuilder
        .add_rectangle(spa_utils::Rectangle {
            width: 1920,
            height: 1080,
        })
        .unwrap();
    sbuilder
        .add_fraction(spa_utils::Fraction {
            num: 30001,
            denom: 1,
        })
        .unwrap();

    assert_eq!(res, sbuf.as_slice());
}

fn test_a_pod<U: From<T> + std::cmp::PartialEq + std::fmt::Debug, T: Copy + Pod<U>>(pod: &T) {
    let mut buf = [0u8; 1024];

    let size = pod.encode(&mut buf).unwrap();
    let (rv, rsize) = T::decode(&buf).unwrap();

    assert_eq!(size, rsize);
    assert_eq!(U::from(*pod), rv);
}

#[test]
fn test_pod_decode() {
    test_a_pod(&());
    test_a_pod(&true);
    test_a_pod(&(-123 as i32));
    test_a_pod(&(i64::MIN));
    test_a_pod(&"hello");
    test_a_pod(&vec![1u8, 2, 3, 4].as_slice());
    test_a_pod(&Pointer {
        type_: Type::Int,
        ptr: 0xdeadbeef as *const c_void,
    });
    test_a_pod(&Fd(-1));
    test_a_pod(&Rectangle {
        width: 1920,
        height: 1080,
    });
    test_a_pod(&Fraction {
        num: 30001,
        denom: 1,
    });
}
