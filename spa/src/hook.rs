// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use std::{cell::RefCell, collections::LinkedList, rc::Rc};

pub type HookId = u32;

pub struct Hook<T> {
    id: HookId,
    callbacks: T,
}

impl<T> Hook<T> {
    pub fn callbacks(&mut self) -> &mut T {
        &mut self.callbacks
    }
}

pub struct HookList<T> {
    hooks: LinkedList<Hook<T>>,
    next_id: HookId,
}

impl<T> HookList<T> {
    pub fn new() -> Rc<RefCell<HookList<T>>> {
        Rc::new(RefCell::new(HookList {
            hooks: LinkedList::new(),
            next_id: 0,
        }))
    }

    pub fn prepend(&mut self, callbacks: T) -> HookId {
        let id = self.next_id;
        let hook = Hook { id, callbacks };

        self.hooks.push_front(hook);
        self.next_id += 1;

        id
    }

    pub fn append(&mut self, callbacks: T) -> HookId {
        let id = self.next_id;
        let hook = Hook { id, callbacks };

        self.hooks.push_back(hook);
        self.next_id += 1;

        id
    }

    // We only implement `iter_mut()` because we expect T to contain `FnMut`s, which need to be
    // borrowed mutably while being called (as they might mutate captured variables in their
    // context)
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Hook<T>> {
        self.hooks.iter_mut()
    }

    pub fn remove(&mut self, id: HookId) -> Option<T> {
        self.hooks
            .extract_if(|h| h.id == id)
            .next()
            .map(|h| h.callbacks)
    }
}

#[macro_export]
macro_rules! emit_hook {
    ($hook_list:expr, $method:ident, $($args:tt)*) => {
        {
            let _hooks = $hook_list.clone();
            for h in _hooks.borrow_mut().iter_mut() {
                (h.callbacks().$method)($($args)*);
            }
        }
    };
}
