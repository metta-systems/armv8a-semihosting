// Declare Crate Attributes
#![no_std]
// Declare Modules
#[macro_use]
pub mod macros;

pub mod export;

pub mod debug;
pub mod hio;
pub mod nr;
// Import Dependencies
use core::arch::asm;
// Declare Syscall
#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub unsafe fn syscall<T>(op_number: usize, arg: &T) -> usize {
    syscall1(op_number, arg as *const T as usize)
}
pub unsafe fn syscall1(op_number: usize, arg: usize) -> usize {
    // Cast Args
    let mut nr = op_number;
    // Execute Syscall
    asm!("hlt #0xF000", inout("x0") nr => nr, in("x1") arg);
    // Return Data
    return nr;
}
#[cfg(not(target_arch = "aarch64"))]
pub unsafe fn syscall<T>(_op_number: usize, _arg: &T) {
    // Noop - Incompatible Architechture
}