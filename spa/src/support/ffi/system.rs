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
    read: extern "C" fn(object: *mut c_void, fd: c_int, buf: *mut c_void, count: usize) -> isize,
    write: extern "C" fn(object: *mut c_void, fd: c_int, buf: *const c_void, count: usize) -> isize,
    ioctl: extern "C" fn(object: *mut c_void, fd: c_int, request: c_ulong, ...) -> c_int,
    close: extern "C" fn(object: *mut c_void, fd: c_int) -> c_int,

    /* clock */
    clock_gettime:
        extern "C" fn(object: *mut c_void, clockid: c_int, value: libc::timespec) -> c_int,
    clock_getres: extern "C" fn(object: *mut c_void, clockid: c_int, res: libc::timespec) -> c_int,

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
    timerfd_create: extern "C" fn(object: *mut c_void, clockid: c_int, flags: c_int) -> c_int,
    timerfd_settime: extern "C" fn(
        object: *mut c_void,
        fd: c_int,
        flags: c_int,
        new_value: *const libc::itimerspec,
        old_value: *mut libc::itimerspec,
    ) -> c_int,
    timerfd_gettime:
        extern "C" fn(object: *mut c_void, fd: c_int, curr_value: *mut libc::itimerspec) -> c_int,
    timerfd_read: extern "C" fn(object: *mut c_void, fd: c_int, expirations: *mut u64) -> c_int,

    /* events */
    eventfd_create: extern "C" fn(object: *mut c_void, flags: c_int) -> c_int,
    eventfd_write: extern "C" fn(object: *mut c_void, fd: c_int, count: u64) -> c_int,
    eventfd_read: extern "C" fn(object: *mut c_void, fd: c_int, count: *mut u64) -> c_int,

    /* signals */
    signalfd_create: extern "C" fn(object: *mut c_void, signal: c_int, flags: c_int) -> c_int,
    signalfd_read: extern "C" fn(object: *mut c_void, fd: c_int, signal: *mut c_int) -> c_int,
}

#[repr(C)]
struct CSystem {
    iface: CInterface,
}

struct CSystemImpl {}

pub fn new_impl(interface: *mut CInterface) -> SystemImpl {
    SystemImpl {
        inner: Box::pin(interface as *mut CSystem),

        read: CSystemImpl::read,
        write: CSystemImpl::write,
        ioctl: libc::ioctl,
        close: CSystemImpl::close,

        clock_gettime: CSystemImpl::clock_gettime,
        clock_getres: CSystemImpl::clock_getres,

        pollfd_create: CSystemImpl::pollfd_create,
        pollfd_add: CSystemImpl::pollfd_add,
        pollfd_mod: CSystemImpl::pollfd_mod,
        pollfd_del: CSystemImpl::pollfd_del,
        pollfd_wait: CSystemImpl::pollfd_wait,

        timerfd_create: CSystemImpl::timerfd_create,
        timerfd_settime: CSystemImpl::timerfd_settime,
        timerfd_gettime: CSystemImpl::timerfd_gettime,
        timerfd_read: CSystemImpl::timerfd_read,

        eventfd_create: CSystemImpl::eventfd_create,
        eventfd_read: CSystemImpl::eventfd_read,
        eventfd_write: CSystemImpl::eventfd_write,

        signalfd_create: CSystemImpl::signalfd_create,
        signalfd_read: CSystemImpl::signalfd_read,
    }
}

impl CSystemImpl {
    fn from_system(this: &SystemImpl) -> &CSystem {
        unsafe {
            this.inner
                .as_ref()
                .downcast_ref::<*const CSystem>()
                .unwrap()
                .as_ref()
                .unwrap()
        }
    }

    fn read(this: &SystemImpl, fd: RawFd, buf: &mut [u8]) -> std::io::Result<isize> {
        unsafe {
            let system = Self::from_system(this);
            let funcs = system.iface.cb.funcs as *const CSystemMethods;

            result_or_error(((*funcs).read)(
                system.iface.cb.data,
                fd,
                buf.as_mut_ptr() as *mut c_void,
                buf.len(),
            ))
        }
    }

    fn write(this: &SystemImpl, fd: RawFd, buf: &[u8]) -> std::io::Result<isize> {
        unsafe {
            let system = Self::from_system(this);
            let funcs = system.iface.cb.funcs as *const CSystemMethods;

            result_or_error(((*funcs).write)(
                system.iface.cb.data,
                fd,
                buf.as_ptr() as *const c_void,
                buf.len(),
            ))
        }
    }

    fn close(this: &SystemImpl, fd: RawFd) -> std::io::Result<i32> {
        unsafe {
            let system = Self::from_system(this);
            let funcs = system.iface.cb.funcs as *const CSystemMethods;

            result_or_error(((*funcs).close)(system.iface.cb.data, fd))
        }
    }

    fn clock_gettime(
        this: &SystemImpl,
        clockid: libc::clockid_t,
        value: &mut libc::timespec,
    ) -> std::io::Result<i32> {
        unsafe {
            let system = Self::from_system(this);
            let funcs = system.iface.cb.funcs as *const CSystemMethods;

            result_or_error(((*funcs).clock_gettime)(
                system.iface.cb.data,
                clockid,
                *value,
            ))
        }
    }

    fn clock_getres(
        this: &SystemImpl,
        clockid: libc::clockid_t,
        res: &mut libc::timespec,
    ) -> std::io::Result<i32> {
        unsafe {
            let system = Self::from_system(this);
            let funcs = system.iface.cb.funcs as *const CSystemMethods;

            result_or_error(((*funcs).clock_getres)(system.iface.cb.data, clockid, *res))
        }
    }

    fn pollfd_create(this: &SystemImpl, flags: i32) -> std::io::Result<i32> {
        unsafe {
            let system = Self::from_system(this);
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
            let system = Self::from_system(this);
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
            let system = Self::from_system(this);
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
            let system = Self::from_system(this);
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
            let system = Self::from_system(this);
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

    fn timerfd_create(
        this: &SystemImpl,
        clockid: libc::clockid_t,
        flags: i32,
    ) -> std::io::Result<i32> {
        unsafe {
            let system = Self::from_system(this);
            let funcs = system.iface.cb.funcs as *const CSystemMethods;

            result_or_error(((*funcs).timerfd_create)(
                system.iface.cb.data,
                clockid,
                flags,
            ))
        }
    }

    fn timerfd_settime(
        this: &SystemImpl,
        fd: RawFd,
        flags: i32,
        new_value: &libc::itimerspec,
        old_value: &mut libc::itimerspec,
    ) -> std::io::Result<i32> {
        unsafe {
            let system = Self::from_system(this);
            let funcs = system.iface.cb.funcs as *const CSystemMethods;

            result_or_error(((*funcs).timerfd_settime)(
                system.iface.cb.data,
                fd,
                flags,
                new_value,
                old_value,
            ))
        }
    }

    fn timerfd_gettime(
        this: &SystemImpl,
        fd: RawFd,
        curr_value: &mut libc::itimerspec,
    ) -> std::io::Result<i32> {
        unsafe {
            let system = Self::from_system(this);
            let funcs = system.iface.cb.funcs as *const CSystemMethods;

            result_or_error(((*funcs).timerfd_gettime)(
                system.iface.cb.data,
                fd,
                curr_value,
            ))
        }
    }

    fn timerfd_read(this: &SystemImpl, fd: RawFd) -> std::io::Result<u64> {
        let mut expirations: u64 = 0;
        let res = unsafe {
            let system = Self::from_system(this);
            let funcs = system.iface.cb.funcs as *const CSystemMethods;

            ((*funcs).timerfd_read)(system.iface.cb.data, fd, &mut expirations)
        };

        if res < 0 {
            return Err(std::io::Error::last_os_error());
        } else if res != 8 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "read timerfd returned unexpected size",
            ));
        } else {
            Ok(expirations)
        }
    }

    fn eventfd_create(this: &SystemImpl, flags: i32) -> std::io::Result<i32> {
        unsafe {
            let system = Self::from_system(this);
            let funcs = system.iface.cb.funcs as *const CSystemMethods;

            result_or_error(((*funcs).eventfd_create)(system.iface.cb.data, flags))
        }
    }

    fn eventfd_read(this: &SystemImpl, fd: RawFd) -> std::io::Result<u64> {
        let mut buf = 0u64;
        let res = unsafe {
            let system = Self::from_system(this);
            let funcs = system.iface.cb.funcs as *const CSystemMethods;

            ((*funcs).eventfd_read)(system.iface.cb.data, fd, &mut buf)
        };

        if res < 0 {
            return Err(std::io::Error::last_os_error());
        } else if res != 8 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "read eventfd returned unexpected size",
            ));
        } else {
            Ok(buf)
        }
    }

    fn eventfd_write(this: &SystemImpl, fd: RawFd, value: u64) -> std::io::Result<i32> {
        let res = unsafe {
            let system = Self::from_system(this);
            let funcs = system.iface.cb.funcs as *const CSystemMethods;

            ((*funcs).eventfd_write)(system.iface.cb.data, fd, value)
        };

        if res < 0 {
            return Err(std::io::Error::last_os_error());
        } else if res != 8 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "write eventfd returned unexpected size",
            ));
        } else {
            Ok(0)
        }
    }

    fn signalfd_create(this: &SystemImpl, signal: u32, flags: i32) -> std::io::Result<i32> {
        unsafe {
            let system = Self::from_system(this);
            let funcs = system.iface.cb.funcs as *const CSystemMethods;

            result_or_error(((*funcs).signalfd_create)(
                system.iface.cb.data,
                signal as c_int,
                flags,
            ))
        }
    }

    fn signalfd_read(this: &SystemImpl, fd: RawFd) -> std::io::Result<u32> {
        let mut signal = 0u32;

        let res = unsafe {
            let system = Self::from_system(this);
            let funcs = system.iface.cb.funcs as *const CSystemMethods;

            ((*funcs).signalfd_read)(
                system.iface.cb.data,
                fd,
                &mut signal as *mut u32 as *mut c_int,
            )
        };

        if res < 0 {
            return Err(std::io::Error::last_os_error());
        } else if res != 8 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "read signalfd returned unexpected size",
            ));
        } else {
            Ok(signal)
        }
    }
}
