// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::os::fd::RawFd;

use bitflags::bitflags;

#[repr(C, packed(1))]
pub struct PollEvent {
    pub events: PollEvents,
    pub data: u64,
}

pub const POLLFD_CLOEXEC: i32 = libc::EPOLL_CLOEXEC;

bitflags! {
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

pub trait System {
    fn pollfd_create(&self, flags: i32) -> std::io::Result<i32>;
    fn pollfd_add(
        &self,
        pfd: RawFd,
        fd: RawFd,
        events: PollEvents,
        data: u64,
    ) -> std::io::Result<i32>;
    fn pollfd_mod(
        &self,
        pfd: RawFd,
        fd: RawFd,
        events: PollEvents,
        data: u64,
    ) -> std::io::Result<i32>;
    fn pollfd_del(&self, pfd: RawFd, fd: RawFd) -> std::io::Result<i32>;
    fn pollfd_wait(
        &self,
        pfd: RawFd,
        events: &mut [PollEvent],
        timeout: i32,
    ) -> std::io::Result<i32>;
}
