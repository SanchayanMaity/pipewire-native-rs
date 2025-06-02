// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::sync::{Arc, Mutex};

use pipewire_native_spa::interface;
use pipewire_native_spa::interface::plugin::{Handle, HandleFactory};
use pipewire_native_spa::interface::thread::ThreadUtilsImpl;
use pipewire_native_spa::support::plugin;

#[test]
fn test_thread() {
    let plugin = plugin::Plugin::new();
    let support = interface::Support::new();

    let handle = plugin
        .init(None, &support)
        .expect("Plugin should be able to provide a handle");
    let iface = handle
        .get_interface(interface::THREAD_UTILS)
        .expect("Handle should be able to provide a thread utils iface");

    let thread_utils = iface
        .downcast_box::<ThreadUtilsImpl>()
        .expect("Iface implementation should be a thread utils impl");

    let accum = Arc::new(Mutex::new(0i32));

    let accum_thr = accum.clone();

    let thread = thread_utils
        .create(None, move || {
            let mut a = accum_thr.lock().unwrap();

            *a += 1;

            Box::new(true)
        })
        .expect("Thread should be created");

    let retval = thread_utils
        .join(thread)
        .expect("Thread should return")
        .downcast::<bool>()
        .expect("Return value should be a bool");

    assert_eq!(*accum.lock().unwrap(), 1);
    assert_eq!(*retval, true);
}
