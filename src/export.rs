//! IMPLEMENTATION DETAILS USED BY MACROS

use core::{fmt::{self, Write}, arch::asm};

use crate::hio::{self, HStderr, HStdout};

static mut HSTDOUT: Option<HStdout> = None;

#[inline(always)]
unsafe fn int_disable() {
    asm!("msr daifset, #0b0011");
}

#[inline(always)]
unsafe fn int_enable() {
    asm!("msr daifclr, #0b0011");
}

pub fn hstdout_str(s: &str) -> Result<(), ()> {
    unsafe {
        int_disable();

        if HSTDOUT.is_none() {
            HSTDOUT = Some(hio::hstdout()?);
        }

        let ret = HSTDOUT.as_mut().unwrap().write_str(s).map_err(drop);
        
        int_enable();
        return ret;
    }
}

pub fn hstdout_fmt(args: fmt::Arguments) -> Result<(), ()> {
    unsafe {
        int_disable();
        if HSTDOUT.is_none() {
            HSTDOUT = Some(hio::hstdout()?);
        }

        let ret = HSTDOUT.as_mut().unwrap().write_fmt(args).map_err(drop);

        int_enable();
        return ret;
    }
}

static mut HSTDERR: Option<HStderr> = None;

pub fn hstderr_str(s: &str) -> Result<(), ()> {
    unsafe {
        int_disable();
        if HSTDERR.is_none() {
            HSTDERR = Some(hio::hstderr()?);
        }

        let ret = HSTDERR.as_mut().unwrap().write_str(s).map_err(drop);

        int_enable();
        return ret;
    }
}

pub fn hstderr_fmt(args: fmt::Arguments) -> Result<(), ()> {
    unsafe {
        int_disable();
        if HSTDERR.is_none() {
            HSTDERR = Some(hio::hstderr()?);
        }

        let ret = HSTDERR.as_mut().unwrap().write_fmt(args).map_err(drop);

        int_enable();
        return ret;
    }
}