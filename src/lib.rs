#![no_std]
#![feature(asm_const)]
#![feature(naked_functions)]

extern crate alloc;

mod context;
mod current;
mod trampoline;

pub use context::TaskContext;
pub use trampoline::trap_entry;