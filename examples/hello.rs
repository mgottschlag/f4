//! Prints "Hello, world" on the OpenOCD console
#![deny(unsafe_code)]
#![deny(warnings)]
#![feature(proc_macro)]
#![no_std]

extern crate cortex_m_rtfm as rtfm;
extern crate cortex_m_semihosting as semihosting;
extern crate f4;

use core::fmt::Write;

use rtfm::app;
use semihosting::hio;

// TASKS & RESOURCES
app! {
    device: f4::stm32f429,
}

// INITIALIZATION PHASE
fn init(_p: init::Peripherals) {}

// IDLE LOOP
fn idle() -> ! {
    writeln!(hio::hstdout().unwrap(), "Hello, world!").unwrap();

    loop {
        rtfm::wfi();
    }
}
