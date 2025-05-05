// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::os::fd::RawFd;
use std::pin::Pin;

use crate::interface::{
    r#loop::{self, InvokeFn, Source},
    system::{self, PollEvents, System},
};

use super::plugin;

pub struct Loop {
    system: Box<&'static dyn System>,
    pollfd: RawFd,
    sources: HashMap<RawFd, Pin<Box<Source>>>,
}

impl Loop {
    pub fn new() -> std::io::Result<Self> {
        let system = Box::new(plugin::system());
        let pollfd = system.pollfd_create(system::POLLFD_CLOEXEC)?;

        Ok(Self {
            system,
            pollfd,
            sources: HashMap::new(),
        })
    }
}

impl r#loop::Loop for Loop {
    fn add_source(&mut self, mut source: Pin<Box<r#loop::Source>>) -> std::io::Result<i32> {
        let fd = source.fd;
        let events =
            PollEvents::from_bits(source.mask).ok_or(Error::from(ErrorKind::InvalidInput))?;
        let data = &*source as *const Source as u64;

        source.rmask = 0;
        self.sources.insert(source.fd, source);

        self.system.pollfd_add(self.pollfd, fd, events, data)
    }

    fn update_source(&mut self, source: Pin<Box<r#loop::Source>>) -> std::io::Result<i32> {
        let fd = source.fd;
        let events =
            PollEvents::from_bits(source.mask).ok_or(Error::from(ErrorKind::InvalidInput))?;
        let data = &*source as *const Source as u64;

        self.sources.entry(source.fd).or_insert(source);

        self.system.pollfd_mod(self.pollfd, fd, events, data)
    }

    fn remove_source(&mut self, fd: RawFd) -> std::io::Result<i32> {
        self.system.pollfd_del(self.pollfd, fd)?;
        self.sources.remove(&fd);
        Ok(0)
    }

    fn invoke(&mut self, func: Pin<Box<InvokeFn>>, block: bool) -> std::io::Result<i32> {
        Err(Error::from(ErrorKind::NotFound))
    }
}
