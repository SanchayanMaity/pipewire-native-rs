// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::ffi::c_void;

use pipewire_native::spa::pod::builder::Builder;
use pipewire_native::spa::pod::parser::Parser;
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
        .push_none()
        .push_bool(true)
        .push_id(Id(1))
        .push_int(2)
        .push_long(3)
        .push_float(4.0)
        .push_double(5.0)
        .push_string("hello")
        .push_bytes(&[6, 7, 8, 9])
        .push_pointer(Type::Int, 0xdeadc0de as *const c_void)
        .push_fd(-1)
        .push_rectangle(1920, 1080)
        .push_fraction(30001, 1)
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

fn test_a_pod<T: Copy + Pod>(pod: &T)
where
    <T as Pod>::DecodesTo: From<T> + std::cmp::PartialEq + std::fmt::Debug,
{
    let mut buf = [0u8; 1024];

    let size = pod.encode(&mut buf).unwrap();
    let (rv, rsize) = T::decode(&buf).unwrap();

    assert_eq!(size, rsize);
    assert_eq!(<T as Pod>::DecodesTo::from(*pod), rv);
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

#[test]
fn test_pod_parser() {
    let mut buf = [0u8; 1024];
    let builder = Builder::new(&mut buf);
    let res = builder
        .push_none()
        .push_bool(true)
        .push_id(Id(1))
        .push_int(2)
        .push_long(3)
        .push_float(4.0)
        .push_double(5.0)
        .push_string("hello")
        .push_bytes(&[6, 7, 8, 9])
        .push_pointer(Type::Int, 0xdeadc0de as *const c_void)
        .push_fd(-1)
        .push_rectangle(1920, 1080)
        .push_fraction(30001, 1)
        .build()
        .unwrap();

    let mut parser = Parser::new(&res);
    assert_eq!(parser.pop_none().unwrap(), ());
    assert_eq!(parser.pop_bool().unwrap(), true);
    assert_eq!(parser.pop_id().unwrap(), Id(1));
    assert_eq!(parser.pop_int().unwrap(), 2);
    assert_eq!(parser.pop_long().unwrap(), 3);
    assert_eq!(parser.pop_float().unwrap(), 4.0);
    assert_eq!(parser.pop_double().unwrap(), 5.0);
    assert_eq!(parser.pop_string().unwrap(), "hello");
    assert_eq!(parser.pop_bytes().unwrap(), vec![6, 7, 8, 9]);
    assert_eq!(
        parser.pop_pointer().unwrap(),
        Pointer {
            type_: Type::Int,
            ptr: 0xdeadc0de as *const c_void,
        }
    );
    assert_eq!(parser.pop_fd().unwrap(), Fd(-1));
    assert_eq!(
        parser.pop_rectangle().unwrap(),
        Rectangle {
            width: 1920,
            height: 1080,
        }
    );
    assert_eq!(
        parser.pop_fraction().unwrap(),
        Fraction {
            num: 30001,
            denom: 1,
        }
    );
}
