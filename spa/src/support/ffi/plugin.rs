// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::ffi::{c_char, c_int, c_void, CStr};
use std::path::PathBuf;

use libloading::os::unix::{Library, Symbol, RTLD_NOW};

use crate::dict::Dict;
use crate::interface::plugin::{Handle, HandleFactory, Interface, InterfaceInfo};
use crate::interface::{self, Support};

use super::c_string;
use super::log;

const ENTRYPOINT: &str = "spa_handle_factory_enum";
type EntryPointFn = unsafe extern "C" fn(*const *mut CHandleFactory, *mut u32) -> c_int;

pub struct Plugin {
    _library: Library,
    factories: Vec<*mut CHandleFactory>,
}

impl Plugin {
    pub fn find_factory(&self, name: &str) -> Option<&impl HandleFactory> {
        for f in &self.factories {
            let f_name = unsafe {
                let factory = f.as_ref().unwrap();
                CStr::from_ptr(factory.name).to_str()
            };

            if f_name == Ok(name) {
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
pub struct CHandleFactory {
    pub version: u32,
    pub name: *const c_char,
    pub info: *const Dict,

    pub get_size: fn(*const CHandleFactory, *const Dict) -> usize,
    pub init: fn(*const CHandleFactory, *mut CHandle, *const Dict, *const c_void, u32) -> c_int,
    pub enum_interface_info:
        fn(*const CHandleFactory, *const *mut CInterfaceInfo, *mut u32) -> c_int,
}

impl HandleFactory for *mut CHandleFactory {
    fn version(&self) -> u32 {
        unsafe { self.as_ref().unwrap().version }
    }

    fn name(&self) -> &str {
        unsafe {
            CStr::from_ptr(self.as_ref().unwrap().name)
                .to_str()
                .unwrap()
        }
    }

    fn info(&self) -> Option<&Dict> {
        unsafe { self.as_ref().unwrap().info.as_ref() }
    }

    fn init(&self, info: Option<Dict>, _support: Option<Support>) -> std::io::Result<impl Handle> {
        unsafe {
            let info_ptr = match &info {
                Some(i) => i.as_raw(),
                None => std::ptr::null(),
            };
            let size = (self.as_ref().unwrap().get_size)(*self, info_ptr);
            let handle = libc::malloc(size) as *mut CHandle;
            let ret = (self.as_ref().unwrap().init)(
                *self,
                handle,
                info_ptr,
                std::ptr::null(), /* FIXME: implement Support -> spa_supoprt */
                0,
            );

            match ret {
                0 => Ok(handle),
                err => Err(std::io::Error::from_raw_os_error(err as i32)),
            }
        }
    }

    fn enum_interface_info(&self) -> Vec<crate::interface::plugin::InterfaceInfo> {
        let mut interfaces = vec![];
        let info: *mut CInterfaceInfo = std::ptr::null_mut();
        let mut i: u32 = 0;

        loop {
            unsafe {
                match (self.as_ref().unwrap().enum_interface_info)(*self, &info, &mut i) {
                    1 => interfaces.push(InterfaceInfo {
                        type_: CStr::from_ptr((*info).type_).to_string_lossy().to_string(),
                    }),
                    _ => return interfaces,
                }
            }
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CCallbacks {
    pub funcs: *const c_void,
    pub data: *mut c_void,
}

#[repr(C)]
pub struct CInterface {
    pub type_: *const c_char,
    pub version: u32,
    pub cb: CCallbacks,
}

#[repr(C)]
pub struct CHandle {
    pub version: u32,
    pub get_interface: fn(*const CHandle, *const c_char, *const *mut CInterface) -> c_int,
    pub clear: fn(*mut CHandle) -> c_int,
}

impl Handle for *mut CHandle {
    fn version(&self) -> u32 {
        unsafe { self.as_ref().unwrap().version }
    }

    fn get_interface(&self, type_: &str) -> Option<Box<dyn Interface>> {
        let iface: *mut CInterface = std::ptr::null_mut();

        unsafe { (self.as_ref().unwrap().get_interface)(*self, c_string(type_).as_ptr(), &iface) };

        match type_ {
            interface::LOG => return Some(Box::new(log::new_impl(iface))),
            _ => return None,
        }
    }

    fn clear(&mut self) {
        unsafe {
            (self.as_ref().unwrap().clear)(*self);
            libc::free(*self as *mut c_void);
        }
    }
}

pub fn load(path: PathBuf) -> Result<Plugin, String> {
    unsafe {
        let library = Library::open(Some(path), RTLD_NOW).map_err(|e| format!("{}", e))?;
        let entrypoint: Symbol<EntryPointFn> = library
            .get(ENTRYPOINT.as_bytes())
            .map_err(|e| format!("{}", e))?;

        let h: *mut CHandleFactory = std::ptr::null_mut();
        let h_ptr: *const *mut CHandleFactory = &h;
        let mut i: u32 = 0;
        let i_ptr: *mut u32 = &mut i;
        let mut factories = vec![];

        loop {
            match entrypoint(h_ptr, i_ptr) {
                1 => factories.push(h),
                0 => break,
                err => return Err(format!("Could not load plugin: {}", err)),
            }
        }

        Ok(Plugin {
            _library: library,
            factories,
        })
    }
}
