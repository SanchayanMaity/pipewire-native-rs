// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::collections::HashMap;
use std::ffi::{c_char, c_int, c_void, CStr, CString};
use std::path::PathBuf;

use crate::interface::plugin::{Handle, HandleFactory, InterfaceInfo};
use crate::interface::Support;

const ENTRYPOINT: &str = "spa_handle_factory_enum";

pub struct Plugin {
    factories: Vec<CHandleFactory>,
}

impl Plugin {
    pub fn find_factory(&self, name: &String) -> Option<&CHandleFactory> {
        for f in &self.factories {
            let f_name = unsafe { CStr::from_ptr(f.name).to_str() };

            if f_name == Ok(name.as_str()) {
                return Some(f);
            }
        }

        None
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct CInterfaceInfo {
    pub type_: *const c_char,
}

impl Clone for CInterfaceInfo {
    fn clone(&self) -> Self {
        CInterfaceInfo {
            type_: unsafe { libc::strdup(self.type_) },
        }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct CHandleFactory {
    pub version: u32,
    pub name: *const c_char,
    pub info: *const c_void,

    pub get_size: fn(*const CHandleFactory, *const c_void) -> usize,
    pub init: fn(*const CHandleFactory, *mut CHandle, *const c_void, *const c_void, u32) -> c_int,
    pub enum_interface_info:
        fn(*const CHandleFactory, *const *mut CInterfaceInfo, *mut u32) -> c_int,
}

impl Clone for CHandleFactory {
    fn clone(&self) -> Self {
        CHandleFactory {
            version: self.version,
            name: unsafe { libc::strdup(self.name) },
            info: std::ptr::null(), /* FIXME: implement spa_dict */
            get_size: self.get_size,
            init: self.init,
            enum_interface_info: self.enum_interface_info,
        }
    }
}

impl HandleFactory for CHandleFactory {
    fn version(&self) -> u32 {
        self.version
    }

    fn name(&self) -> String {
        unsafe { CStr::from_ptr(self.name).to_string_lossy().to_string() }
    }

    fn info(&self) -> crate::Dict {
        /* FIXME: implement spa_dict */
        HashMap::new()
    }

    fn init(
        &self,
        info: Option<crate::Dict>,
        support: Option<Support>,
    ) -> std::io::Result<impl Handle> {
        let size = (self.get_size)(self, std::ptr::null() /* FIXME: implement spa_dict */);
        let handle = unsafe { libc::malloc(size) as *mut CHandle };
        let ret = (self.init)(
            self,
            handle,
            std::ptr::null(), /* FIXME: implement spa_dict */
            std::ptr::null(), /* FIXME: implement Support -> spa_supoprt */
            0,
        );

        match ret {
            0 => unsafe { Ok(*handle) },
            err => Err(std::io::Error::from_raw_os_error(err as i32)),
        }
    }

    fn enum_interface_info(&self) -> Vec<crate::interface::plugin::InterfaceInfo> {
        let mut interfaces = vec![];
        let info: *mut CInterfaceInfo = std::ptr::null_mut();
        let mut i: u32 = 0;

        loop {
            match (self.enum_interface_info)(self, &info, &mut i) {
                1 => interfaces.push(InterfaceInfo {
                    type_: unsafe { CStr::from_ptr((*info).type_).to_string_lossy().to_string() },
                }),
                _ => return interfaces,
            }
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CHandle {
    pub version: u32,
    pub get_interface: fn(*const CHandle, *const c_char, *const *mut c_void) -> c_int,
    pub clear: fn(*mut CHandle) -> c_int,
}

impl Handle for CHandle {
    fn version(&self) -> u32 {
        self.version
    }

    fn get_interface<T: crate::interface::plugin::Interface>(
        &self,
        type_: &str,
    ) -> Option<&'static T> {
        let iface: *mut c_void = std::ptr::null_mut();

        (self.get_interface)(self, CString::new(type_).unwrap().as_ptr(), &iface);

        None
    }

    fn clear(&mut self) {
        (self.clear)(self);
    }
}

pub fn load(path: PathBuf) -> Result<Plugin, String> {
    let factories = unsafe {
        let lib = libloading::Library::new(path).map_err(|e| format!("{}", e))?;
        let entry: libloading::Symbol<
            unsafe extern "C" fn(*const *mut CHandleFactory, *mut u32) -> c_int,
        > = lib
            .get(ENTRYPOINT.as_bytes())
            .map_err(|e| format!("{}", e))?;

        let h: *mut CHandleFactory = std::ptr::null_mut();
        let h_ptr: *const *mut CHandleFactory = &h;
        let mut i: u32 = 0;
        let i_ptr: *mut u32 = &mut i;
        let mut factories = vec![];

        loop {
            match entry(h_ptr, i_ptr) {
                1 => factories.push(*h),
                0 => break,
                err => return Err(format!("Could not load plugin: {}", err)),
            }
        }

        factories
    };

    Ok(Plugin { factories })
}
