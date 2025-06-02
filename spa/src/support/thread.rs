// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::any::Any;

use crate::{
    dict::Dict,
    interface::thread::{self, Thread, ThreadReturn, ThreadUtilsImpl},
};

struct ThreadUtils {}

pub fn new_utils() -> ThreadUtilsImpl {
    ThreadUtilsImpl {
        inner: Box::pin(ThreadUtils {}),

        create: ThreadUtils::create,
        join: ThreadUtils::join,

        get_rt_range: ThreadUtils::get_rt_range,

        acquire_rt: ThreadUtils::acquire_rt,
        drop_rt: ThreadUtils::drop_rt,
    }
}

impl ThreadUtils {
    pub fn create(
        _this: &ThreadUtilsImpl,
        props: Option<&Dict>,
        start: Box<dyn FnOnce() -> ThreadReturn + Send + 'static>,
    ) -> Option<Thread> {
        let builder = props
            .and_then(|p| {
                p.lookup(thread::STACK_SIZE)
                    .and_then(|k| k.parse::<u32>().ok())
            })
            .map(|size| std::thread::Builder::new().stack_size(size as usize))
            .unwrap_or_else(std::thread::Builder::new);

        let handle = builder.spawn(start).ok()?;

        Some(Thread {
            inner: Box::new(handle),
        })
    }

    fn join(_this: &ThreadUtilsImpl, thread: Thread) -> std::io::Result<ThreadReturn> {
        let handle = thread
            .inner
            .downcast::<std::thread::JoinHandle<Box<dyn Any + Send + 'static>>>()
            .unwrap();

        handle
            .join()
            .map_err(|e| std::io::Error::other(format!("Error while joining thread {e:?}")))
    }

    fn get_rt_range(_this: &ThreadUtilsImpl, _props: Option<&Dict>) -> std::io::Result<(i32, i32)> {
        Err(std::io::Error::from(std::io::ErrorKind::NotFound))
    }

    fn acquire_rt(
        _this: &ThreadUtilsImpl,
        _thread: &Thread,
        _priority: i32,
    ) -> std::io::Result<()> {
        Err(std::io::Error::from(std::io::ErrorKind::NotFound))
    }
    fn drop_rt(_this: &ThreadUtilsImpl, _thread: &Thread) -> std::io::Result<()> {
        Err(std::io::Error::from(std::io::ErrorKind::NotFound))
    }
}
