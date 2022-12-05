//! IMPLEMENTATION DETAILS USED BY MACROS

use core::{fmt::{self, Write}, arch::asm};

use crate::hio::{self, HStderr, HStdout};

static mut HSTDOUT: Option<HStdout> = None;

struct IntDisable;

impl IntDisable {
    #[inline(always)]
    pub fn enter() -> IntDisable {
        unsafe { asm!("msr daifset, #0b0011"); }
        Self
    }
}

impl Drop for IntDisable {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe { asm!("msr daifclr, #0b0011"); }
    }
}

pub fn hstdout_str(s: &str) -> Result<(), ()> {
    unsafe {
        let ints = IntDisable::enter();

        if HSTDOUT.is_none() {
            HSTDOUT = Some(hio::hstdout()?);
        }

        let ret = HSTDOUT.as_mut().unwrap().write_str(s).map_err(drop);

        drop(ints);
        ret
    }
}

pub fn hstdout_fmt(args: fmt::Arguments) -> Result<(), ()> {
    unsafe {
        let ints = IntDisable::enter();

        if HSTDOUT.is_none() {
            HSTDOUT = Some(hio::hstdout()?);
        }

        let ret = HSTDOUT.as_mut().unwrap().write_fmt(args).map_err(drop);

        drop(ints);
        ret
    }
}

static mut HSTDERR: Option<HStderr> = None;

pub fn hstderr_str(s: &str) -> Result<(), ()> {
    unsafe {
        let ints = IntDisable::enter();

        if HSTDERR.is_none() {
            HSTDERR = Some(hio::hstderr()?);
        }

        let ret = HSTDERR.as_mut().unwrap().write_str(s).map_err(drop);

        drop(ints);
        ret
    }
}

pub fn hstderr_fmt(args: fmt::Arguments) -> Result<(), ()> {
    unsafe {
        let ints = IntDisable::enter();

        if HSTDERR.is_none() {
            HSTDERR = Some(hio::hstderr()?);
        }

        let ret = HSTDERR.as_mut().unwrap().write_fmt(args).map_err(drop);

        drop(ints);
        ret
    }
}
