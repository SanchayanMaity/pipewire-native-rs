// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::{any::Any, os::fd::RawFd, pin::Pin};

use bitflags::bitflags;

use super::plugin::Interface;

#[repr(C, packed(1))]
pub struct PollEvent {
    pub events: PollEvents,
    pub data: u64,
}

pub const POLLFD_CLOEXEC: i32 = libc::EPOLL_CLOEXEC;

bitflags! {
    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct PollEvents: u32 {
        /* Events */
        const IN = libc::EPOLLIN as u32;
        const PRI = libc::EPOLLPRI as u32;
        const OUT = libc::EPOLLOUT as u32;
        const ERR = libc::EPOLLERR as u32;
        const HUP = libc::EPOLLHUP as u32;
        const RDHUP = libc::EPOLLRDHUP as u32;
        /* Input flags */
        const ET = libc::EPOLLET as u32;
        const ONESHOT = libc::EPOLLONESHOT as u32;
        const WAKEUP = libc::EPOLLWAKEUP as u32;
        const EXCLUSIVE = libc::EPOLLEXCLUSIVE as u32;
    }
}

pub fn result_or_error(res: i32) -> std::io::Result<i32> {
    if res >= 0 {
        Ok(res)
    } else {
        Err(std::io::Error::last_os_error())
    }
}

pub struct SystemImpl {
    pub inner: Pin<Box<dyn Any>>,

    pub pollfd_create: fn(this: &SystemImpl, flags: i32) -> std::io::Result<i32>,
    pub pollfd_add: fn(
        this: &SystemImpl,
        pfd: RawFd,
        fd: RawFd,
        events: PollEvents,
        data: u64,
    ) -> std::io::Result<i32>,
    pub pollfd_mod: fn(
        this: &SystemImpl,
        pfd: RawFd,
        fd: RawFd,
        events: PollEvents,
        data: u64,
    ) -> std::io::Result<i32>,
    pub pollfd_del: fn(this: &SystemImpl, pfd: RawFd, fd: RawFd) -> std::io::Result<i32>,
    pub pollfd_wait: fn(
        this: &SystemImpl,
        pfd: RawFd,
        events: &mut [PollEvent],
        timeout: i32,
    ) -> std::io::Result<i32>,
}

impl SystemImpl {
    pub fn pollfd_create(&self, flags: i32) -> std::io::Result<i32> {
        (self.pollfd_create)(self, flags)
    }

    pub fn pollfd_add(
        &self,
        pfd: RawFd,
        fd: RawFd,
        events: PollEvents,
        data: u64,
    ) -> std::io::Result<i32> {
        (self.pollfd_add)(self, pfd, fd, events, data)
    }

    pub fn pollfd_mod(
        &self,
        pfd: RawFd,
        fd: RawFd,
        events: PollEvents,
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
}

impl Interface for SystemImpl {}

unsafe impl Send for SystemImpl {}
unsafe impl Sync for SystemImpl {}
