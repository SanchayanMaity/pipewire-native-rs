// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::any::Any;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::os::fd::RawFd;
use std::pin::Pin;

use crate::interface;
use crate::interface::plugin::{Handle, HandleFactory};
use crate::interface::r#loop::LoopImpl;
use crate::interface::system::SystemImpl;
use crate::interface::{
    r#loop::{self, InvokeFn, Source},
    system::{self, PollEvents},
};

pub struct Loop {
    system: Box<SystemImpl>,
    pollfd: RawFd,
    sources: HashMap<RawFd, Pin<Box<Source>>>,
}

impl Loop {
    pub fn new() -> std::io::Result<LoopImpl> {
        let system_iface = super::plugin()
            .init(None, None)
            .unwrap()
            .get_interface(interface::SYSTEM)
            .unwrap();
        let system = (system_iface as Box<dyn Any>)
            .downcast::<SystemImpl>()
            .unwrap();
        let pollfd = system.pollfd_create(system::POLLFD_CLOEXEC)?;

        Ok(LoopImpl {
            inner: Box::pin(Self {
                system,
                pollfd,
                sources: HashMap::new(),
            }),

            add_source: Self::add_source,
            update_source: Self::update_source,
            remove_source: Self::remove_source,
            invoke: Self::invoke,
        })
    }
}

impl Loop {
    fn add_source(
        this: &mut LoopImpl,
        mut source: Pin<Box<r#loop::Source>>,
    ) -> std::io::Result<i32> {
        // Shenanigans until downcast_mut_unchecked() is stable
        let inner = unsafe { Pin::into_inner_unchecked(this.inner.as_mut()) };
        let self_ = unsafe { &mut *(inner as *mut dyn Any as *mut Loop) };

        let fd = source.fd;
        let events =
            PollEvents::from_bits(source.mask).ok_or(Error::from(ErrorKind::InvalidInput))?;
        let data = &*source as *const Source as u64;

        source.rmask = 0;
        self_.sources.insert(source.fd, source);

        self_.system.pollfd_add(self_.pollfd, fd, events, data)
    }

    fn update_source(
        this: &mut LoopImpl,
        source: Pin<Box<r#loop::Source>>,
    ) -> std::io::Result<i32> {
        // Shenanigans until downcast_mut_unchecked() is stable
        let inner = unsafe { Pin::into_inner_unchecked(this.inner.as_mut()) };
        let self_ = unsafe { &mut *(inner as *mut dyn Any as *mut Loop) };

        let fd = source.fd;
        let events =
            PollEvents::from_bits(source.mask).ok_or(Error::from(ErrorKind::InvalidInput))?;
        let data = &*source as *const Source as u64;

        self_.sources.entry(source.fd).or_insert(source);

        self_.system.pollfd_mod(self_.pollfd, fd, events, data)
    }

    fn remove_source(this: &mut LoopImpl, fd: RawFd) -> std::io::Result<i32> {
        // Shenanigans until downcast_mut_unchecked() is stable
        let inner = unsafe { Pin::into_inner_unchecked(this.inner.as_mut()) };
        let self_ = unsafe { &mut *(inner as *mut dyn Any as *mut Loop) };

        self_.system.pollfd_del(self_.pollfd, fd)?;
        self_.sources.remove(&fd);
        Ok(0)
    }

    fn invoke(this: &mut LoopImpl, func: Pin<Box<InvokeFn>>, block: bool) -> std::io::Result<i32> {
        Err(Error::from(ErrorKind::NotFound))
    }
}
