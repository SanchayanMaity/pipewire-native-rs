// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::{any::Any, os::fd::RawFd, pin::Pin};

use crate::flags;

use super::plugin::Interface;

#[repr(C, packed(1))]
pub struct PollEvent {
    pub events: flags::Io,
    pub data: u64,
}

pub fn result_or_error<T: num::Integer>(res: T) -> std::io::Result<T> {
    if res >= num::zero() {
        Ok(res)
    } else {
        Err(std::io::Error::last_os_error())
    }
}

pub struct SystemImpl {
    pub inner: Pin<Box<dyn Any>>,

    /* read/write/ioctl */
    pub read: fn(this: &SystemImpl, fd: RawFd, buf: &mut [u8]) -> std::io::Result<isize>,
    pub write: fn(this: &SystemImpl, fd: RawFd, buf: &[u8]) -> std::io::Result<isize>,
    pub ioctl: unsafe extern "C" fn(fd: RawFd, request: u64, ...) -> i32,
    pub close: fn(this: &SystemImpl, fd: RawFd) -> std::io::Result<i32>,

    /* clock */
    pub clock_gettime: fn(
        this: &SystemImpl,
        clockid: libc::clockid_t,
        value: &mut libc::timespec,
    ) -> std::io::Result<i32>,
    pub clock_getres: fn(
        this: &SystemImpl,
        clockid: libc::clockid_t,
        res: &mut libc::timespec,
    ) -> std::io::Result<i32>,

    /* poll */
    pub pollfd_create: fn(this: &SystemImpl, flags: flags::Fd) -> std::io::Result<i32>,
    pub pollfd_add: fn(
        this: &SystemImpl,
        pfd: RawFd,
        fd: RawFd,
        events: flags::Io,
        data: u64,
    ) -> std::io::Result<i32>,
    pub pollfd_mod: fn(
        this: &SystemImpl,
        pfd: RawFd,
        fd: RawFd,
        events: flags::Io,
        data: u64,
    ) -> std::io::Result<i32>,
    pub pollfd_del: fn(this: &SystemImpl, pfd: RawFd, fd: RawFd) -> std::io::Result<i32>,
    pub pollfd_wait: fn(
        this: &SystemImpl,
        pfd: RawFd,
        events: &mut [PollEvent],
        timeout: i32,
    ) -> std::io::Result<i32>,

    /* timers */
    pub timerfd_create:
        fn(this: &SystemImpl, clockid: i32, flags: flags::Fd) -> std::io::Result<i32>,
    pub timerfd_settime: fn(
        this: &SystemImpl,
        fd: RawFd,
        flags: flags::Fd,
        new_value: &libc::itimerspec,
        old_value: Option<&mut libc::itimerspec>,
    ) -> std::io::Result<i32>,
    pub timerfd_gettime:
        fn(this: &SystemImpl, fd: RawFd, curr_value: &mut libc::itimerspec) -> std::io::Result<i32>,
    pub timerfd_read: fn(this: &SystemImpl, fd: RawFd) -> std::io::Result<u64>,

    /* events */
    pub eventfd_create: fn(this: &SystemImpl, flags: flags::Fd) -> std::io::Result<i32>,
    pub eventfd_write: fn(this: &SystemImpl, fd: RawFd, count: u64) -> std::io::Result<i32>,
    pub eventfd_read: fn(this: &SystemImpl, fd: RawFd) -> std::io::Result<u64>,

    /* signals */
    pub signalfd_create:
        fn(this: &SystemImpl, signal: u32, flags: flags::Fd) -> std::io::Result<i32>,
    pub signalfd_read: fn(this: &SystemImpl, fd: RawFd) -> std::io::Result<u32>,
}

impl SystemImpl {
    pub fn read(&self, fd: RawFd, buf: &mut [u8]) -> std::io::Result<isize> {
        (self.read)(self, fd, buf)
    }

    pub fn write(&self, fd: RawFd, buf: &[u8]) -> std::io::Result<isize> {
        (self.write)(self, fd, buf)
    }

    /* ioctl will need to be invoked directly because of varargs */

    pub fn close(&self, fd: RawFd) -> std::io::Result<i32> {
        (self.close)(self, fd)
    }

    pub fn clock_gettime(
        &self,
        clockid: libc::clockid_t,
        value: &mut libc::timespec,
    ) -> std::io::Result<i32> {
        (self.clock_gettime)(self, clockid, value)
    }

    pub fn clock_getres(
        &self,
        clockid: libc::clockid_t,
        res: &mut libc::timespec,
    ) -> std::io::Result<i32> {
        (self.clock_getres)(self, clockid, res)
    }

    pub fn pollfd_create(&self, flags: flags::Fd) -> std::io::Result<i32> {
        (self.pollfd_create)(self, flags)
    }

    pub fn pollfd_add(
        &self,
        pfd: RawFd,
        fd: RawFd,
        events: flags::Io,
        data: u64,
    ) -> std::io::Result<i32> {
        (self.pollfd_add)(self, pfd, fd, events, data)
    }

    pub fn pollfd_mod(
        &self,
        pfd: RawFd,
        fd: RawFd,
        events: flags::Io,
        data: u64,
    ) -> std::io::Result<i32> {
        (self.pollfd_mod)(self, pfd, fd, events, data)
    }

    pub fn pollfd_del(&self, pfd: RawFd, fd: RawFd) -> std::io::Result<i32> {
        (self.pollfd_del)(self, pfd, fd)
    }

    pub fn pollfd_wait(
        &self,
        pfd: RawFd,
        events: &mut [PollEvent],
        timeout: i32,
    ) -> std::io::Result<i32> {
        (self.pollfd_wait)(self, pfd, events, timeout)
    }

    pub fn timerfd_create(&self, clockid: i32, flags: flags::Fd) -> std::io::Result<i32> {
        (self.timerfd_create)(self, clockid, flags)
    }

    pub fn timerfd_settime(
        &self,
        fd: RawFd,
        flags: flags::Fd,
        new_value: &libc::itimerspec,
        old_value: Option<&mut libc::itimerspec>,
    ) -> std::io::Result<i32> {
        (self.timerfd_settime)(self, fd, flags, new_value, old_value)
    }

    pub fn timerfd_gettime(
        &self,
        fd: RawFd,
        curr_value: &mut libc::itimerspec,
    ) -> std::io::Result<i32> {
        (self.timerfd_gettime)(self, fd, curr_value)
    }

    pub fn timerfd_read(&self, fd: RawFd) -> std::io::Result<u64> {
        (self.timerfd_read)(self, fd)
    }

    pub fn eventfd_create(&self, flags: flags::Fd) -> std::io::Result<i32> {
        (self.eventfd_create)(self, flags)
    }

    pub fn eventfd_write(&self, fd: RawFd, count: u64) -> std::io::Result<i32> {
        (self.eventfd_write)(self, fd, count)
    }

    pub fn eventfd_read(&self, fd: RawFd) -> std::io::Result<u64> {
        (self.eventfd_read)(self, fd)
    }

    pub fn signalfd_create(&self, signal: u32, flags: flags::Fd) -> std::io::Result<i32> {
        (self.signalfd_create)(self, signal, flags)
    }

    pub fn signalfd_read(&self, fd: RawFd) -> std::io::Result<u32> {
        (self.signalfd_read)(self, fd)
    }
}

impl Interface for SystemImpl {
    unsafe fn make_native(&self) -> *mut super::ffi::CInterface {
        crate::support::ffi::system::make_native(self)
    }

    unsafe fn free_native(system: *mut super::ffi::CInterface) {
        crate::support::ffi::system::free_native(system)
    }
}

unsafe impl Send for SystemImpl {}
unsafe impl Sync for SystemImpl {}
