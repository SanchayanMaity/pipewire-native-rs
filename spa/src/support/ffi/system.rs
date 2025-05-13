// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::{
    ffi::{c_int, c_ulong, c_void},
    os::fd::RawFd,
};

use crate::interface::{
    ffi::CInterface,
    system::{result_or_error, PollEvent, PollEvents, SystemImpl},
};

#[repr(C)]
#[derive(Copy, Clone)]
struct CSystemMethods {
    version: u32,

    /* read/write/ioctl */
    //read: extern "C" fn(object: *mut c_void, fd: c_int, buf: *mut c_void, count: usize) -> isize,
    //write: extern "C" fn(object: *mut c_void, fd: c_int, buf: *mut c_void, count: usize) -> isize,
    //ioctl: extern "C" fn(object: *mut c_void, fd: c_int, request: c_ulong, ...) -> c_int,
    //close: extern "C" fn(object: *mut c_void, fd: c_int) -> c_int,

    /* clock */
    //clock_gettime:
    //    extern "C" fn(object: *mut c_void, clockid: c_int, value: libc::timespec) -> c_int,
    //clock_getres: extern "C" fn(object: *mut c_void, clockid: c_int, res: libc::timespec) -> c_int,

    /* poll */
    pollfd_create: extern "C" fn(object: *mut c_void, pfd: c_int) -> c_int,
    pollfd_add: extern "C" fn(
        object: *mut c_void,
        pfd: c_int,
        fd: c_int,
        events: PollEvents,
        data: *mut c_void,
    ) -> c_int,
    pollfd_mod: extern "C" fn(
        object: *mut c_void,
        pfd: c_int,
        fd: c_int,
        events: PollEvents,
        data: *mut c_void,
    ) -> c_int,
    pollfd_del: extern "C" fn(object: *mut c_void, pfd: c_int, fd: c_int) -> c_int,
    pollfd_wait: extern "C" fn(
        object: *mut c_void,
        pfd: c_int,
        ev: *mut PollEvent,
        n_ev: c_int,
        timeout: c_int,
    ) -> c_int,
    /* timers */
    //timerfd_create: extern "C" fn(object: *mut c_void, clockid: c_int, flags: c_int) -> c_int,
    //timerfd_settime: extern "C" fn(
    //    object: *mut c_void,
    //    fd: c_int,
    //    flags: c_int,
    //    new_value: *const libc::itimerspec,
    //    old_value: *mut libc::itimerspec,
    //) -> c_int,
    //timerfd_gettime:
    //    extern "C" fn(object: *mut c_void, fd: c_int, curr_value: *mut libc::itimerspec) -> c_int,
    //timerfd_read: extern "C" fn(object: *mut c_void, fd: c_int, expierations: *mut u64) -> c_int,

    /* events */
    //eventfd_create: extern "C" fn(object: *mut c_void, flags: c_int) -> c_int,
    //eventfd_write: extern "C" fn(object: *mut c_void, fd: c_int, count: u64) -> c_int,
    //eventfd_read: extern "C" fn(object: *mut c_void, fd: c_int, count: *mut u64) -> c_int,

    /* signals */
    //signalfd_create: extern "C" fn(object: *mut c_void, signal: c_int, flags: c_int) -> c_int,
    //signalfd_read: extern "C" fn(object: *mut c_void, fd: c_int, signal: *mut c_int) -> c_int,
}

#[repr(C)]
struct CSystem {
    iface: CInterface,
}

struct CSystemImpl {}

pub fn new_impl(interface: *mut CInterface) -> SystemImpl {
    SystemImpl {
        inner: Box::pin(interface as *mut CSystem),

        pollfd_create: CSystemImpl::pollfd_create,
        pollfd_add: CSystemImpl::pollfd_add,
        pollfd_mod: CSystemImpl::pollfd_mod,
        pollfd_del: CSystemImpl::pollfd_del,
        pollfd_wait: CSystemImpl::pollfd_wait,
    }
}

impl CSystemImpl {
    fn pollfd_create(this: &SystemImpl, flags: i32) -> std::io::Result<i32> {
        unsafe {
            let system = this
                .inner
                .as_ref()
                .downcast_ref::<*mut CSystem>()
                .unwrap()
                .as_ref()
                .unwrap();
            let funcs = system.iface.cb.funcs as *const CSystemMethods;

            result_or_error(((*funcs).pollfd_create)(system.iface.cb.data, flags))
        }
    }

    fn pollfd_add(
        this: &SystemImpl,
        pfd: RawFd,
        fd: RawFd,
        events: PollEvents,
        data: u64,
    ) -> std::io::Result<i32> {
        unsafe {
            let system = this
                .inner
                .as_ref()
                .downcast_ref::<*mut CSystem>()
                .unwrap()
                .as_ref()
                .unwrap();
            let funcs = system.iface.cb.funcs as *const CSystemMethods;

            result_or_error(((*funcs).pollfd_add)(
                system.iface.cb.data,
                pfd,
                fd,
                events,
                data as *mut c_void,
            ))
        }
    }

    fn pollfd_mod(
        this: &SystemImpl,
        pfd: RawFd,
        fd: RawFd,
        events: PollEvents,
        data: u64,
    ) -> std::io::Result<i32> {
        unsafe {
            let system = this
                .inner
                .as_ref()
                .downcast_ref::<*mut CSystem>()
                .unwrap()
                .as_ref()
                .unwrap();
            let funcs = system.iface.cb.funcs as *const CSystemMethods;

            result_or_error(((*funcs).pollfd_mod)(
                system.iface.cb.data,
                pfd,
                fd,
                events,
                data as *mut c_void,
            ))
        }
    }

    fn pollfd_del(this: &SystemImpl, pfd: RawFd, fd: RawFd) -> std::io::Result<i32> {
        unsafe {
            let system = this
                .inner
                .as_ref()
                .downcast_ref::<*mut CSystem>()
                .unwrap()
                .as_ref()
                .unwrap();
            let funcs = system.iface.cb.funcs as *const CSystemMethods;

            result_or_error(((*funcs).pollfd_del)(system.iface.cb.data, pfd, fd))
        }
    }

    fn pollfd_wait(
        this: &SystemImpl,
        pfd: RawFd,
        events: &mut [PollEvent],
        timeout: i32,
    ) -> std::io::Result<i32> {
        unsafe {
            let system = this
                .inner
                .as_ref()
                .downcast_ref::<*mut CSystem>()
                .unwrap()
                .as_ref()
                .unwrap();
            let funcs = system.iface.cb.funcs as *const CSystemMethods;

            result_or_error(((*funcs).pollfd_wait)(
                system.iface.cb.data,
                pfd,
                events.as_mut_ptr(),
                events.len() as i32,
                timeout,
            ))
        }
    }
}
