// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::{io::Error, os::fd::RawFd};

use crate::interface::system::{self, PollEvent, PollEvents, SystemImpl};

pub struct System {}

impl System {
    pub fn new() -> SystemImpl {
        SystemImpl {
            inner: Box::new(System {}),
        }
    }
}

fn result_or_error(res: i32) -> std::io::Result<i32> {
    if res >= 0 {
        Ok(res)
    } else {
        Err(Error::last_os_error())
    }
}

impl system::System for System {
    fn pollfd_create(&self, flags: i32) -> std::io::Result<i32> {
        let res = unsafe { libc::epoll_create1(flags) };
        result_or_error(res)
    }

    fn pollfd_add(
        &self,
        pfd: RawFd,
        fd: RawFd,
        events: PollEvents,
        data: u64,
    ) -> std::io::Result<i32> {
        let mut event = libc::epoll_event {
            events: events.bits(),
            u64: data,
        };
        let res = unsafe {
            libc::epoll_ctl(
                pfd,
                libc::EPOLL_CTL_ADD,
                fd,
                &mut event as *mut libc::epoll_event,
            )
        };
        result_or_error(res)
    }

    fn pollfd_mod(
        &self,
        pfd: RawFd,
        fd: RawFd,
        events: PollEvents,
        data: u64,
    ) -> std::io::Result<i32> {
        let mut event = libc::epoll_event {
            events: events.bits(),
            u64: data,
        };
        let res = unsafe {
            libc::epoll_ctl(
                pfd,
                libc::EPOLL_CTL_MOD,
                fd,
                &mut event as *mut libc::epoll_event,
            )
        };
        result_or_error(res)
    }

    fn pollfd_del(&self, pfd: RawFd, fd: RawFd) -> std::io::Result<i32> {
        let res = unsafe {
            libc::epoll_ctl(
                pfd,
                libc::EPOLL_CTL_DEL,
                fd,
                std::ptr::null_mut::<libc::epoll_event>(),
            )
        };
        result_or_error(res)
    }

    fn pollfd_wait(
        &self,
        pfd: RawFd,
        events: &mut [PollEvent],
        timeout: i32,
    ) -> std::io::Result<i32> {
        let res = unsafe {
            libc::epoll_wait(
                pfd,
                events.as_mut_ptr() as *mut libc::epoll_event,
                events.len() as i32,
                timeout,
            )
        };
        result_or_error(res)
    }
}

unsafe impl Send for System {}

unsafe impl Sync for System {}
