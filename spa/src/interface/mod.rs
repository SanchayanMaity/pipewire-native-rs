// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::{
    collections::HashMap,
    ffi::{c_void, CStr, CString},
    pin::Pin,
    rc::Rc,
};

use cpu::CpuImpl;
use ffi::{CInterface, CSupport};
use log::LogImpl;
use r#loop::LoopImpl;
use system::SystemImpl;

use crate::support;

pub mod cpu;
pub mod ffi;
pub mod log;
pub mod r#loop;
pub mod plugin;
pub mod system;

/* Well-known interface names */
pub const LOG: &str = "Spa:Pointer:Interface:Log";
pub const LOOP: &str = "Spa:Pointer:Interface:Loop";
pub const SYSTEM: &str = "Spa:Pointer:Interface:System";
pub const CPU: &str = "Spa:Pointer:Interface:Cpu";

pub struct Support {
    supports: HashMap<&'static str, Rc<Pin<Box<dyn plugin::Interface>>>>,
    /* We keep a C-compatible array that won't get moved around, so we can reliably pass it on to
     * plugins */
    c_supports: Vec<CSupport>,
}

impl Default for Support {
    fn default() -> Self {
        Support {
            supports: HashMap::new(),
            /* Reserve enough space so the array is always valid */
            c_supports: Vec::with_capacity(16),
        }
    }
}

impl Support {
    pub fn new() -> Support {
        Support::default()
    }

    pub fn c_support(&self) -> &Vec<CSupport> {
        &self.c_supports
    }

    fn add_or_update_c(&mut self, name: &str, data: *mut CInterface) {
        for s in self.c_supports.iter_mut() {
            let type_ = unsafe { CStr::from_ptr(s.type_).to_str() };
            if type_ == Ok(name) {
                s.data = data as *mut c_void;
                return;
            }
        }

        self.c_supports.push(CSupport {
            type_: support::ffi::c_string(name).into_raw(),
            data: data as *mut c_void,
        });
    }

    pub fn add_interface(&mut self, name: &'static str, iface: Box<dyn plugin::Interface>) {
        let pin = Box::into_pin(iface);
        let data = unsafe { pin.make_native() };

        self.supports.insert(name, Rc::new(pin));
        self.add_or_update_c(name, data);
    }

    pub fn get_interface<T>(&self, name: &str) -> Option<Rc<Pin<Box<T>>>>
    where
        T: plugin::Interface + 'static,
    {
        let iface = self.supports.get(name).cloned();

        iface.and_then(|iface| iface.downcast_rc_pin_box::<T>().ok())
    }
}

impl Drop for Support {
    fn drop(&mut self) {
        for s in self.c_supports.iter_mut() {
            unsafe {
                let type_ = CString::from_raw(s.type_);
                match type_.to_str().unwrap() {
                    CPU => <CpuImpl as plugin::Interface>::free_native(s.data as *mut CInterface),
                    LOOP => <LoopImpl as plugin::Interface>::free_native(s.data as *mut CInterface),
                    LOG => <LogImpl as plugin::Interface>::free_native(s.data as *mut CInterface),
                    SYSTEM => {
                        <SystemImpl as plugin::Interface>::free_native(s.data as *mut CInterface)
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}
