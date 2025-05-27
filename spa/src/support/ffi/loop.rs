// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::{
    collections::HashMap,
    ffi::{c_int, c_uint, c_ulong, c_void, CString},
    os::fd::RawFd,
    pin::Pin,
};

use crate::interface::ffi::{CControlHooks, CHook, CLoop, CSource};
use crate::interface::r#loop::*;
use crate::interface::{
    self,
    ffi::CInterface,
    r#loop::{InvokeFn, LoopImpl, Source, SourceFn},
};

use super::c_string;

type CInvokeFunc = extern "C" fn(
    loop_: *mut CLoop,
    async_: bool,
    seq: u32,
    data: *const c_void,
    size: libc::size_t,
    user_data: *mut c_void,
) -> c_int;

#[repr(C)]
struct CLoopMethods {
    version: u32,
    add_source: extern "C" fn(object: *mut c_void, source: *mut CSource) -> c_int,
    update_source: extern "C" fn(object: *mut c_void, source: *mut CSource) -> c_int,
    remove_source: extern "C" fn(object: *mut c_void, source: *mut CSource) -> c_int,
    invoke: extern "C" fn(
        object: *mut c_void,
        func: CInvokeFunc,
        seq: u32,
        data: *const c_void,
        size: libc::size_t,
        block: bool,
        user_data: *mut c_void,
    ) -> c_int,
}

struct CLoopImpl {
    iface: *mut CLoop,
    sources: HashMap<RawFd, Pin<Box<CSourceImpl>>>,
}

pub fn new_impl(interface: *mut CInterface) -> LoopImpl {
    LoopImpl {
        inner: Box::pin(CLoopImpl {
            iface: interface as *mut CLoop,
            sources: HashMap::new(),
        }),

        add_source: CLoopImpl::add_source,
        update_source: CLoopImpl::update_source,
        remove_source: CLoopImpl::remove_source,
        invoke: CLoopImpl::invoke,
    }
}

#[repr(C)]
struct CSourceImpl {
    c_source: CSource,
    func: Box<SourceFn>,
}

fn result_from(res: i32) -> std::io::Result<i32> {
    if res >= 0 {
        Ok(0)
    } else {
        Err(std::io::Error::from_raw_os_error(-res))
    }
}

impl CLoopImpl {
    fn from_loop(this: &LoopImpl) -> (&mut CLoopImpl, &mut CLoop) {
        let c_loopimpl = unsafe {
            this.inner
                .as_ref()
                .downcast_ref::<*mut CLoopImpl>()
                .unwrap()
                .as_mut()
                .unwrap()
        };
        let c_loop = unsafe { c_loopimpl.iface.as_mut().unwrap() };

        (c_loopimpl, c_loop)
    }

    #[no_mangle]
    extern "C" fn source_trampoline(c_source: *mut CSource) {
        let source_impl = unsafe { (c_source as *mut CSourceImpl).as_mut().unwrap() };
        let source = Source {
            fd: source_impl.c_source.fd,
            mask: source_impl.c_source.mask,
            rmask: source_impl.c_source.rmask,
        };

        (source_impl.func)(&source);
    }

    fn add_source(
        loop_: &mut LoopImpl,
        source: &Source,
        func: Box<SourceFn>,
    ) -> std::io::Result<i32> {
        let (c_loopimpl, c_loop) = Self::from_loop(loop_);
        let funcs = unsafe {
            (c_loop.iface.cb.funcs as *const CLoopMethods)
                .as_ref()
                .unwrap()
        };

        let mut source_impl = Box::pin(CSourceImpl {
            c_source: CSource {
                loop_: loop_.inner.downcast_ref::<CLoop>().unwrap() as *const CLoop,
                func: Self::source_trampoline,
                data: std::ptr::null_mut(), /* see below */
                fd: source.fd,
                mask: source.mask,
                rmask: source.rmask,
                priv_: std::ptr::null_mut(),
            },
            func,
        });

        let c_source = &mut source_impl.c_source as *mut CSource;

        c_loopimpl
            .sources
            .insert(source_impl.c_source.fd, source_impl);

        result_from((funcs.add_source)(c_loop.iface.cb.data, c_source))
    }

    fn update_source(loop_: &mut LoopImpl, source: &Source) -> std::io::Result<i32> {
        let (c_loopimpl, c_loop) = Self::from_loop(loop_);
        let funcs = unsafe {
            (c_loop.iface.cb.funcs as *const CLoopMethods)
                .as_ref()
                .unwrap()
        };

        let source_impl = c_loopimpl
            .sources
            .get_mut(&source.fd)
            .ok_or(std::io::Error::from(std::io::ErrorKind::NotFound))?;

        source_impl.c_source.mask = source.mask;

        result_from((funcs.update_source)(
            c_loop.iface.cb.data,
            &mut source_impl.c_source as *mut CSource,
        ))
    }

    fn remove_source(loop_: &mut LoopImpl, fd: RawFd) -> std::io::Result<i32> {
        let (c_loopimpl, c_loop) = Self::from_loop(loop_);
        let funcs = unsafe {
            (c_loop.iface.cb.funcs as *const CLoopMethods)
                .as_ref()
                .unwrap()
        };

        let mut source_impl = c_loopimpl
            .sources
            .remove(&fd)
            .ok_or(std::io::Error::from(std::io::ErrorKind::NotFound))?;

        result_from((funcs.remove_source)(
            c_loop.iface.cb.data,
            &mut source_impl.c_source as *mut CSource,
        ))
    }

    #[no_mangle]
    extern "C" fn invoke_trampoline(
        _loop: *mut CLoop,
        async_: bool,
        seq: u32,
        data: *const c_void,
        size: libc::size_t,
        user_data: *mut c_void,
    ) -> i32 {
        //pub invoke: fn(&mut LoopImpl, func: Pin<Box<InvokeFn>>, block: bool) -> std::io::Result<i32>,
        //pub type InvokeFn = dyn FnMut(LoopImpl, bool, u32, &[u8]) -> i32 + 'static;
        let func = unsafe { (user_data as *mut Box<InvokeFn>).as_mut().unwrap() };
        let data = unsafe { std::slice::from_raw_parts(data as *const u8, size) };

        (func)(async_, seq, data)
    }

    fn invoke(
        loop_: &mut LoopImpl,
        seq: u32,
        data: &[u8],
        block: bool,
        func: Box<InvokeFn>,
    ) -> std::io::Result<i32> {
        let (_, c_loop) = Self::from_loop(loop_);
        let funcs = unsafe {
            (c_loop.iface.cb.funcs as *const CLoopMethods)
                .as_ref()
                .unwrap()
        };
        let mut invoke_func = Box::pin(func);

        result_from((funcs.invoke)(
            c_loop.iface.cb.data,
            Self::invoke_trampoline,
            seq,
            data.as_ptr() as *const c_void,
            data.len() as libc::size_t,
            block,
            Pin::into_inner(invoke_func.as_mut()) as *mut InvokeFn as *mut c_void,
        ))
    }
}

static LOOP_METHODS: CLoopMethods = CLoopMethods {
    version: 0,
    add_source: LoopImplIface::add_source,
    update_source: LoopImplIface::update_source,
    remove_source: LoopImplIface::remove_source,
    invoke: LoopImplIface::invoke,
};

pub(crate) unsafe fn make_native(loop_: &LoopImpl) -> *mut CInterface {
    let c_loop: *mut CLoop =
        unsafe { libc::calloc(1, std::mem::size_of::<CLoop>() as libc::size_t) as *mut CLoop };
    let c_loop = unsafe { &mut *c_loop };

    c_loop.iface.version = 0;
    c_loop.iface.type_ = c_string(interface::LOOP).into_raw();
    c_loop.iface.cb.funcs = &LOOP_METHODS as *const CLoopMethods as *mut c_void;
    c_loop.iface.cb.data = loop_ as *const LoopImpl as *mut c_void;

    c_loop as *mut CLoop as *mut CInterface
}

pub(crate) unsafe fn free_native(c_loop: *mut CInterface) {
    unsafe {
        let _ = CString::from_raw((*c_loop).type_ as *mut i8);
        libc::free(c_loop as *mut c_void);
    }
}

struct LoopImplIface {}

impl LoopImplIface {
    fn c_to_loop_impl(object: *mut c_void) -> &'static mut LoopImpl {
        unsafe { (object as *mut LoopImpl).as_mut().unwrap() }
    }

    extern "C" fn add_source(object: *mut c_void, source: *mut CSource) -> c_int {
        let loop_impl = Self::c_to_loop_impl(object);
        let c_source = unsafe { source.as_mut().unwrap() };
        let impl_source = Source {
            fd: c_source.fd,
            mask: c_source.mask,
            rmask: c_source.rmask,
        };

        let res = loop_impl.add_source(&impl_source, Box::new(|_| (c_source.func)(c_source)));

        match res {
            Ok(_) => 0,
            Err(e) => -e.raw_os_error().unwrap_or(-1),
        }
    }

    extern "C" fn update_source(object: *mut c_void, source: *mut CSource) -> c_int {
        let loop_impl = Self::c_to_loop_impl(object);
        let c_source = unsafe { source.as_mut().unwrap() };
        let impl_source = Source {
            fd: c_source.fd,
            mask: c_source.mask,
            rmask: c_source.rmask,
        };

        let res = loop_impl.update_source(&impl_source);

        match res {
            Ok(_) => 0,
            Err(e) => -e.raw_os_error().unwrap_or(-1),
        }
    }

    extern "C" fn remove_source(object: *mut c_void, source: *mut CSource) -> c_int {
        let loop_impl = Self::c_to_loop_impl(object);
        let c_source = unsafe { source.as_mut().unwrap() };

        let res = loop_impl.remove_source(c_source.fd);

        match res {
            Ok(_) => 0,
            Err(e) => -e.raw_os_error().unwrap_or(-1),
        }
    }

    extern "C" fn invoke(
        object: *mut c_void,
        func: CInvokeFunc,
        seq: u32,
        data: *const c_void,
        size: libc::size_t,
        block: bool,
        user_data: *mut c_void,
    ) -> c_int {
        let loop_impl = Self::c_to_loop_impl(object);

        let res = loop_impl.invoke(
            seq,
            unsafe { std::slice::from_raw_parts(data as *const u8, size) },
            block,
            Box::new(move |async_, seq, data| {
                func(
                    object as *mut CLoop,
                    async_,
                    seq,
                    data.as_ptr() as *const c_void,
                    data.len(),
                    user_data,
                )
            }),
        );

        match res {
            Ok(_) => 0,
            Err(e) => -e.raw_os_error().unwrap_or(-1),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct CControlMethodsMethods {
    version: u32,

    get_fd: extern "C" fn(object: *mut c_void) -> c_uint,
    add_hook:
        extern "C" fn(object: *mut c_void, hook: &CHook, hooks: &CControlHooks, data: *mut c_void),
    enter: extern "C" fn(object: *mut c_void),
    leave: extern "C" fn(object: *mut c_void),
    iterate: extern "C" fn(object: *mut c_void) -> c_int,
    check: extern "C" fn(object: *mut c_void) -> c_int,
}

#[repr(C)]
struct CControlMethods {
    iface: CInterface,
}

struct CControlMethodsImpl {}

pub fn control_methods_new_impl(interface: *mut CInterface) -> ControlMethodsImpl {
    ControlMethodsImpl {
        inner: Box::pin(interface as *mut CControlMethods),

        get_fd: CControlMethodsImpl::get_fd,
        add_hook: CControlMethodsImpl::add_hook,
        enter: CControlMethodsImpl::enter,
        leave: CControlMethodsImpl::leave,
        iterate: CControlMethodsImpl::iterate,
        check: CControlMethodsImpl::check,
    }
}

impl CControlMethodsImpl {
    fn from_control_methods(this: &ControlMethodsImpl) -> &CControlMethods {
        unsafe {
            this.inner
                .as_ref()
                .downcast_ref::<*const CControlMethods>()
                .unwrap()
                .as_ref()
                .unwrap()
        }
    }

    fn get_fd(this: &ControlMethodsImpl) -> u32 {
        unsafe {
            let control_impl = Self::from_control_methods(this);
            let funcs = control_impl.iface.cb.funcs as *const CControlMethodsMethods;

            ((*funcs).get_fd)(control_impl.iface.cb.data)
        }
    }

    fn add_hook(this: &ControlMethodsImpl, hook: &CHook, hooks: &CControlHooks, data: u64) {
        unsafe {
            let control_impl = Self::from_control_methods(this);
            let funcs = control_impl.iface.cb.funcs as *const CControlMethodsMethods;

            ((*funcs).add_hook)(control_impl.iface.cb.data, hook, hooks, data as *mut c_void);
        }
    }

    fn enter(this: &ControlMethodsImpl) {
        unsafe {
            let control_impl = Self::from_control_methods(this);
            let funcs = control_impl.iface.cb.funcs as *const CControlMethodsMethods;

            ((*funcs).enter)(control_impl.iface.cb.data)
        }
    }

    fn leave(this: &ControlMethodsImpl) {
        unsafe {
            let control_impl = Self::from_control_methods(this);
            let funcs = control_impl.iface.cb.funcs as *const CControlMethodsMethods;

            ((*funcs).leave)(control_impl.iface.cb.data);
        }
    }

    fn iterate(this: &ControlMethodsImpl) -> i32 {
        unsafe {
            let control_impl = Self::from_control_methods(this);
            let funcs = control_impl.iface.cb.funcs as *const CControlMethodsMethods;

            ((*funcs).iterate)(control_impl.iface.cb.data)
        }
    }

    fn check(this: &ControlMethodsImpl) -> i32 {
        unsafe {
            let control_impl = Self::from_control_methods(this);
            let funcs = control_impl.iface.cb.funcs as *const CControlMethodsMethods;

            ((*funcs).check)(control_impl.iface.cb.data)
        }
    }
}

static LOOP_CONTROL_METHODS: CControlMethodsMethods = CControlMethodsMethods {
    version: 0,

    get_fd: ControlMethodsIface::get_fd,
    add_hook: ControlMethodsIface::add_hook,
    enter: ControlMethodsIface::enter,
    leave: ControlMethodsIface::leave,
    iterate: ControlMethodsIface::iterate,
    check: ControlMethodsIface::check,
};

struct ControlMethodsIface {}

impl ControlMethodsIface {
    fn c_to_control_methods_impl(object: *mut c_void) -> &'static ControlMethodsImpl {
        unsafe { &*(object as *mut ControlMethodsImpl) }
    }

    extern "C" fn get_fd(object: *mut c_void) -> c_uint {
        let control_methods_impl = Self::c_to_control_methods_impl(object);

        control_methods_impl.get_fd()
    }

    extern "C" fn add_hook(
        object: *mut c_void,
        hook: &CHook,
        hooks: &CControlHooks,
        data: *mut c_void,
    ) {
        let control_methods_impl = Self::c_to_control_methods_impl(object);

        control_methods_impl.add_hook(hook, hooks, data as u64)
    }

    extern "C" fn enter(object: *mut c_void) {
        let control_methods_impl = Self::c_to_control_methods_impl(object);

        control_methods_impl.enter()
    }

    extern "C" fn leave(object: *mut c_void) {
        let control_methods_impl = Self::c_to_control_methods_impl(object);

        control_methods_impl.leave()
    }

    extern "C" fn iterate(object: *mut c_void) -> c_int {
        let control_methods_impl = Self::c_to_control_methods_impl(object);

        control_methods_impl.iterate()
    }

    extern "C" fn check(object: *mut c_void) -> c_int {
        let control_methods_impl = Self::c_to_control_methods_impl(object);

        control_methods_impl.check()
    }
}

pub unsafe fn control_methods_make_native(loop_ctrl: &ControlMethodsImpl) -> *mut CInterface {
    let c_ctrl_methods: *mut CControlMethods = unsafe {
        libc::calloc(1, std::mem::size_of::<CControlMethods>() as libc::size_t)
            as *mut CControlMethods
    };
    let c_ctrl_methods = unsafe { &mut *c_ctrl_methods };

    c_ctrl_methods.iface.version = 1;
    c_ctrl_methods.iface.type_ = c_string(interface::CPU).into_raw();
    c_ctrl_methods.iface.cb.funcs =
        &LOOP_CONTROL_METHODS as *const CControlMethodsMethods as *mut c_void;
    c_ctrl_methods.iface.cb.data = loop_ctrl as *const ControlMethodsImpl as *mut c_void;

    c_ctrl_methods as *mut CControlMethods as *mut CInterface
}

pub unsafe fn control_methods_free_native(c_loop_ctrl: *mut CInterface) {
    unsafe {
        let _ = CString::from_raw((*c_loop_ctrl).type_ as *mut i8);
        libc::free(c_loop_ctrl as *mut c_void);
    }
}

type CSourceIoFn = extern "C" fn(data: *mut c_void, fd: c_int, mask: c_uint);
type CSourceIdleFn = extern "C" fn(data: *mut c_void);
type CSourceEventFn = extern "C" fn(data: *mut c_void, count: c_ulong);
type CSourceTimerFn = extern "C" fn(data: *mut c_void, expirations: c_ulong);
type CSourceSignalFn = extern "C" fn(data: *mut c_void, signal_number: c_int);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct CLoopUtilsMethods {
    version: u32,

    add_io: extern "C" fn(
        object: *mut c_void,
        fd: c_int,
        mask: u32,
        close: bool,
        func: CSourceIoFn,
        data: *mut c_void,
    ) -> *mut CSource,
    update_io: extern "C" fn(object: *mut c_void, source: *mut CSource, mask: u32) -> c_int,
    add_idle: extern "C" fn(
        object: *mut c_void,
        enabled: bool,
        func: CSourceIdleFn,
        data: *mut c_void,
    ) -> *mut CSource,
    enable_idle: extern "C" fn(object: *mut c_void, source: *mut CSource, enabled: bool) -> c_int,
    add_event:
        extern "C" fn(object: *mut c_void, func: CSourceEventFn, data: *mut c_void) -> *mut CSource,
    signal_event: extern "C" fn(object: *mut c_void, source: *mut CSource) -> c_int,
    add_timer:
        extern "C" fn(object: *mut c_void, func: CSourceTimerFn, data: *mut c_void) -> *mut CSource,
    update_timer: extern "C" fn(
        object: *mut c_void,
        source: *mut CSource,
        value: &libc::timespec,
        interval: &libc::timespec,
        absolute: bool,
    ) -> c_int,
    add_signal: extern "C" fn(
        object: *mut c_void,
        signal_number: c_int,
        func: CSourceSignalFn,
        data: *mut c_void,
    ) -> *mut CSource,
    destroy_source: extern "C" fn(object: *mut c_void, source: *mut CSource),
}

struct CLoopUtils {
    iface: CInterface,
}

struct CLoopUtilsImpl {
    iface: *mut CLoopUtils,
}

pub fn loop_utils_new_impl(interface: *mut CInterface) -> LoopUtilsImpl {
    LoopUtilsImpl {
        inner: Box::pin(CLoopUtilsImpl {
            iface: interface as *mut CLoopUtils,
        }),

        add_io: CLoopUtilsImpl::add_io,
        update_io: CLoopUtilsImpl::update_io,
        add_idle: CLoopUtilsImpl::add_idle,
        enable_idle: CLoopUtilsImpl::enable_idle,
        add_event: CLoopUtilsImpl::add_event,
        signal_event: CLoopUtilsImpl::signal_event,
        add_timer: CLoopUtilsImpl::add_timer,
        update_timer: CLoopUtilsImpl::update_timer,
        add_signal: CLoopUtilsImpl::add_signal,
        destroy_source: CLoopUtilsImpl::destroy_source,
    }
}

impl CLoopUtilsImpl {
    fn from_loop_utils(this: &LoopUtilsImpl) -> (&mut CLoopUtilsImpl, &mut CLoopUtils) {
        let c_looputils_impl = unsafe {
            this.inner
                .as_ref()
                .downcast_ref::<*mut CLoopUtilsImpl>()
                .unwrap()
                .as_mut()
                .unwrap()
        };
        let c_looputils = unsafe { c_looputils_impl.iface.as_mut().unwrap() };

        (c_looputils_impl, c_looputils)
    }

    #[no_mangle]
    extern "C" fn source_io_trampoline(data: *mut c_void, fd: c_int, mask: c_uint) {
        let source = unsafe { (data as *mut Pin<Box<LoopUtilsSource>>).as_mut().unwrap() };

        let func = match source.cb {
            LoopUtilsSourceCb::Io(ref mut f) => f,
            _ => panic!("source_io_trampoline called with non-IO source"),
        };

        (func)(fd, mask)
    }

    fn add_io(
        this: &LoopUtilsImpl,
        fd: RawFd,
        mask: u32,
        close: bool,
        func: Box<SourceIoFn>,
    ) -> Option<Pin<Box<LoopUtilsSource>>> {
        let (_, utils) = Self::from_loop_utils(this);
        let funcs = utils.iface.cb.funcs as *const CLoopUtilsMethods;

        let mut source = Box::pin(LoopUtilsSource {
            cb: LoopUtilsSourceCb::Io(func),
            inner: std::ptr::null_mut(),
        });

        let c_source = unsafe {
            ((*funcs).add_io)(
                utils.iface.cb.data,
                fd,
                mask,
                close,
                Self::source_io_trampoline,
                &mut source as *mut Pin<Box<LoopUtilsSource>> as *mut c_void,
            )
        };

        if c_source.is_null() {
            None
        } else {
            source.inner = c_source as *mut c_void;
            Some(source)
        }
    }

    fn update_io(
        this: &LoopUtilsImpl,
        source: &mut LoopUtilsSource,
        mask: u32,
    ) -> std::io::Result<i32> {
        let utils_impl = Self::from_loop_utils(this).1;
        let funcs = utils_impl.iface.cb.funcs as *const CLoopUtilsMethods;
        let source = source.inner as *mut CSource;

        result_from(unsafe { ((*funcs).update_io)(utils_impl.iface.cb.data, source, mask) })
    }

    #[no_mangle]
    extern "C" fn source_idle_trampoline(data: *mut c_void) {
        let source = unsafe { (data as *mut Pin<Box<LoopUtilsSource>>).as_mut().unwrap() };

        let func = match source.cb {
            LoopUtilsSourceCb::Idle(ref mut f) => f,
            _ => panic!("source_idle_trampoline called with non-idle source"),
        };

        (func)()
    }

    fn add_idle(
        this: &LoopUtilsImpl,
        enabled: bool,
        func: Box<SourceIdleFn>,
    ) -> Option<Pin<Box<LoopUtilsSource>>> {
        let (_, utils) = Self::from_loop_utils(this);
        let funcs = utils.iface.cb.funcs as *const CLoopUtilsMethods;

        let mut source = Box::pin(LoopUtilsSource {
            cb: LoopUtilsSourceCb::Idle(func),
            inner: std::ptr::null_mut(),
        });

        let c_source = unsafe {
            ((*funcs).add_idle)(
                utils.iface.cb.data,
                enabled,
                Self::source_idle_trampoline,
                &mut source as *mut Pin<Box<LoopUtilsSource>> as *mut c_void,
            )
        };

        if c_source.is_null() {
            None
        } else {
            source.inner = c_source as *mut c_void;
            Some(source)
        }
    }

    fn enable_idle(
        this: &LoopUtilsImpl,
        source: &mut LoopUtilsSource,
        enabled: bool,
    ) -> std::io::Result<i32> {
        let utils_impl = Self::from_loop_utils(this).1;
        let funcs = utils_impl.iface.cb.funcs as *const CLoopUtilsMethods;
        let source = source.inner as *mut CSource;

        result_from(unsafe { ((*funcs).enable_idle)(utils_impl.iface.cb.data, source, enabled) })
    }

    #[no_mangle]
    extern "C" fn source_event_trampoline(data: *mut c_void, count: c_ulong) {
        let source = unsafe { (data as *mut Pin<Box<LoopUtilsSource>>).as_mut().unwrap() };

        let func = match source.cb {
            LoopUtilsSourceCb::Event(ref mut f) => f,
            _ => panic!("source_event_trampoline called with non-event source"),
        };

        (func)(count)
    }

    fn add_event(
        this: &LoopUtilsImpl,
        func: Box<SourceEventFn>,
    ) -> Option<Pin<Box<LoopUtilsSource>>> {
        let (_, utils) = Self::from_loop_utils(this);
        let funcs = utils.iface.cb.funcs as *const CLoopUtilsMethods;

        let mut source = Box::pin(LoopUtilsSource {
            cb: LoopUtilsSourceCb::Event(func),
            inner: std::ptr::null_mut(),
        });

        let c_source = unsafe {
            ((*funcs).add_event)(
                utils.iface.cb.data,
                Self::source_event_trampoline,
                &mut source as *mut Pin<Box<LoopUtilsSource>> as *mut c_void,
            )
        };

        if c_source.is_null() {
            None
        } else {
            source.inner = c_source as *mut c_void;
            Some(source)
        }
    }

    fn signal_event(this: &LoopUtilsImpl, source: &mut LoopUtilsSource) -> std::io::Result<i32> {
        let utils_impl = Self::from_loop_utils(this).1;
        let funcs = utils_impl.iface.cb.funcs as *const CLoopUtilsMethods;
        let source = source.inner as *mut CSource;

        result_from(unsafe { ((*funcs).signal_event)(utils_impl.iface.cb.data, source) })
    }

    #[no_mangle]
    extern "C" fn source_timer_trampoline(data: *mut c_void, expirations: c_ulong) {
        let source = unsafe { (data as *mut Pin<Box<LoopUtilsSource>>).as_mut().unwrap() };

        let func = match source.cb {
            LoopUtilsSourceCb::Timer(ref mut f) => f,
            _ => panic!("source_timer_trampoline called with non-timer source"),
        };

        (func)(expirations)
    }

    fn add_timer(
        this: &LoopUtilsImpl,
        func: Box<SourceTimerFn>,
    ) -> Option<Pin<Box<LoopUtilsSource>>> {
        let (_, utils) = Self::from_loop_utils(this);
        let funcs = utils.iface.cb.funcs as *const CLoopUtilsMethods;

        let mut source = Box::pin(LoopUtilsSource {
            cb: LoopUtilsSourceCb::Timer(func),
            inner: std::ptr::null_mut(),
        });

        let c_source = unsafe {
            ((*funcs).add_timer)(
                utils.iface.cb.data,
                Self::source_timer_trampoline,
                &mut source as *mut Pin<Box<LoopUtilsSource>> as *mut c_void,
            )
        };

        if c_source.is_null() {
            None
        } else {
            source.inner = c_source as *mut c_void;
            Some(source)
        }
    }

    fn update_timer(
        this: &LoopUtilsImpl,
        source: &mut LoopUtilsSource,
        value: &libc::timespec,
        interval: &libc::timespec,
        absolute: bool,
    ) -> std::io::Result<i32> {
        let utils_impl = Self::from_loop_utils(this).1;
        let funcs = utils_impl.iface.cb.funcs as *const CLoopUtilsMethods;
        let source = source.inner as *mut CSource;

        result_from(unsafe {
            ((*funcs).update_timer)(utils_impl.iface.cb.data, source, value, interval, absolute)
        })
    }

    #[no_mangle]
    extern "C" fn source_signal_trampoline(data: *mut c_void, signal_number: c_int) {
        let source = unsafe { (data as *mut Pin<Box<LoopUtilsSource>>).as_mut().unwrap() };

        let func = match source.cb {
            LoopUtilsSourceCb::Signal(ref mut f) => f,
            _ => panic!("source_signal_trampoline called with non-signal source"),
        };

        (func)(signal_number)
    }

    fn add_signal(
        this: &LoopUtilsImpl,
        signal_number: i32,
        func: Box<SourceSignalFn>,
    ) -> Option<Pin<Box<LoopUtilsSource>>> {
        let (_, utils) = Self::from_loop_utils(this);
        let funcs = utils.iface.cb.funcs as *const CLoopUtilsMethods;

        let mut source = Box::pin(LoopUtilsSource {
            cb: LoopUtilsSourceCb::Signal(func),
            inner: std::ptr::null_mut(),
        });

        let c_source = unsafe {
            ((*funcs).add_signal)(
                utils.iface.cb.data,
                signal_number,
                Self::source_signal_trampoline,
                &mut source as *mut Pin<Box<LoopUtilsSource>> as *mut c_void,
            )
        };

        if c_source.is_null() {
            None
        } else {
            source.inner = c_source as *mut c_void;
            Some(source)
        }
    }

    fn destroy_source(this: &LoopUtilsImpl, source: LoopUtilsSource) {
        let (_, utils) = Self::from_loop_utils(this);
        let funcs = utils.iface.cb.funcs as *const CLoopUtilsMethods;
        let c_source = source.inner as *mut CSource;

        unsafe {
            ((*funcs).destroy_source)(utils.iface.cb.data, c_source);
        }
    }
}

// struct LoopUtilsIface {}
//
// impl LoopUtilsIface {
//     fn c_to_loop_utils_impl(object: *mut c_void) -> &'static mut LoopUtilsImpl {
//         unsafe { (object as *mut LoopUtilsImpl).as_mut().unwrap() }
//     }
//
//     extern "C" fn add_io(
//         object: *mut c_void,
//         fd: c_int,
//         mask: u32,
//         close: bool,
//         func: CSourceIoFn,
//         data: *mut c_void,
//     ) -> *mut CSource {
//         let loop_utils_impl = Self::c_to_loop_utils_impl(object);
//
//         let Some(LoopUtilsSource {
//             stype: _,
//             source: mut csource,
//         }) = loop_utils_impl.add_io(
//             fd,
//             mask,
//             close,
//             Box::new(move |fd, mask| func(data, fd, mask)),
//         )
//         else {
//             return std::ptr::null_mut();
//         };
//
//         return &mut csource as *mut CSource;
//     }
//
//     extern "C" fn update_io(object: *mut c_void, source: *mut CSource, mask: u32) -> i32 {
//         let loop_utils_impl = Self::c_to_loop_utils_impl(object);
//         let c_source = unsafe { source.as_mut().unwrap() as CSource };
//         let mut lsrc = LoopUtilsSource::new(LoopUtilsSourceType::Io, c_source);
//
//         loop_utils_impl.update_io(&lsrc, mask)
//     }
//
//     extern "C" fn add_idle(
//         object: *mut c_void,
//         enabled: bool,
//         func: CSourceIdleFn,
//         data: *mut c_void,
//     ) -> *const CSource {
//         let loop_utils_impl = Self::c_to_loop_utils_impl(object);
//
//         loop_utils_impl.add_idle(enabled, Box::new(move || func(data)));
//     }
//
//     extern "C" fn enable_idle(object: *mut c_void, source: *mut CSource, enabled: bool) {
//         let loop_utils_impl = Self::c_to_loop_utils_impl(object);
//         let c_source = unsafe { source.as_mut().unwrap() };
//
//         loop_utils_impl.enable_idle(c_source, enabled)
//     }
//
//     extern "C" fn add_event(object: *mut c_void, func: CSourceEventFn, data: *mut c_void) {
//         let loop_utils_impl = Self::c_to_loop_utils_impl(object);
//
//         loop_utils_impl.add_event(Box::new(move |count| func(data, count)))
//     }
//
//     extern "C" fn signal_event(object: *mut c_void, source: *mut CSource) {
//         let loop_utils_impl = Self::c_to_loop_utils_impl(object);
//         let c_source = unsafe { source.as_mut().unwrap() };
//
//         loop_utils_impl.signal_event(c_source)
//     }
//
//     extern "C" fn add_timer(object: *mut c_void, func: CSourceTimerFn, data: *mut c_void) {
//         let loop_utils_impl = Self::c_to_loop_utils_impl(object);
//
//         loop_utils_impl.add_timer(Box::new(move |expirations| func(data, expirations)))
//     }
//
//     extern "C" fn update_timer(
//         object: *mut c_void,
//         source: *mut CSource,
//         value: &libc::timespec,
//         interval: &libc::timespec,
//         absolute: bool,
//     ) {
//         let loop_utils_impl = Self::c_to_loop_utils_impl(object);
//         let c_source = unsafe { source.as_mut().unwrap() };
//
//         loop_utils_impl.update_timer(c_source, value, interval, absolute)
//     }
//
//     extern "C" fn add_signal(
//         object: *mut c_void,
//         signal_number: c_int,
//         func: CSourceSignalFn,
//         data: *mut c_void,
//     ) {
//         let loop_utils_impl = Self::c_to_loop_utils_impl(object);
//
//         loop_utils_impl.add_signal(
//             signal_number,
//             Box::new(move |signal_number| func(data, signal_number)),
//         )
//     }
//
//     extern "C" fn destroy_source(object: *mut c_void, source: *mut CSource) {
//         let loop_utils_impl = Self::c_to_loop_utils_impl(object);
//         let c_source = unsafe { source.as_mut().unwrap() };
//
//         loop_utils_impl.destroy_source(c_source)
//     }
// }
//
// static LOOP_UTILS_METHODS: CLoopUtilsMethods = CLoopUtilsMethods {
//     version: 0,
//
//     add_io: LoopUtilsIface::add_io,
//     update_io: LoopUtilsIface::update_io,
//     add_idle: LoopUtilsIface::add_idle,
//     enable_idle: LoopUtilsIface::enable_idle,
//     add_event: LoopUtilsIface::add_event,
//     signal_event: LoopUtilsIface::signal_event,
//     add_timer: LoopUtilsIface::add_timer,
//     update_timer: LoopUtilsIface::update_timer,
//     add_signal: LoopUtilsIface::add_signal,
//     destroy_source: LoopUtilsIface::destroy_source,
// };
//
// pub unsafe fn loop_utils_make_native(cpu: &LoopUtilsImpl) -> *mut CInterface {
//     let c_loop_utils: *mut CLoopUtils = unsafe {
//         libc::calloc(1, std::mem::size_of::<CLoopUtils>() as libc::size_t) as *mut CLoopUtils
//     };
//     let c_loop_utils = unsafe { &mut *c_loop_utils };
//
//     c_loop_utils.iface.version = 0;
//     c_loop_utils.iface.type_ = c_string(interface::CPU).into_raw();
//     c_loop_utils.iface.cb.funcs = &LOOP_UTILS_METHODS as *const CLoopUtilsMethods as *mut c_void;
//     c_loop_utils.iface.cb.data = cpu as *const LoopUtilsImpl as *mut c_void;
//
//     c_loop_utils as *mut CLoopUtils as *mut CInterface
// }
//
// pub unsafe fn loop_utils_free_native(c_loop_util: *mut CInterface) {
//     unsafe {
//         let _ = CString::from_raw((*c_loop_util).type_ as *mut i8);
//         libc::free(c_loop_util as *mut c_void);
//     }
// }
