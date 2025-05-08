// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: Copyright (c) 2025 Asymptotic Inc.
// SPDX-FileCopyrightText: Copyright (c) 2025 Arun Raghavan

use r#loop::LoopImpl;
use system::SystemImpl;

pub mod r#loop;
pub mod plugin;
pub mod system;

/* Well-known interface names */
pub const LOOP: &str = "Spa:Pointer:Interface:Loop";
pub const SYSTEM: &str = "Spa:Pointer:Interface:System";

/* While spa_support is a list of generic support features, for now we use the specific set of
 * interfaces we know, to keep things less messy. If necessary, this can be replaced with a
 * HashMap<String, Any>, and we can do some coercion. */
pub struct Support {
    pub system: Option<SystemImpl>,
    pub loop_: Option<LoopImpl>,
}
