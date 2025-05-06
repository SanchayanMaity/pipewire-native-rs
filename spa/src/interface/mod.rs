// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

pub mod r#loop;
pub mod plugin;
pub mod system;

/* Well-known interface names */
pub const LOOP: &str = "Spa:Pointer:Interface:Loop";
pub const SYSTEM: &str = "Spa:Pointer:Interface:System";
