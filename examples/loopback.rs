//! Serial interface loopback
#![deny(unsafe_code)]
#![deny(warnings)]
#![feature(proc_macro)]
#![no_std]

extern crate cortex_m_rtfm as rtfm;
extern crate f4;

use f4::prelude::*;
use f4::Serial;
use f4::serial::Event;
use f4::time::Hertz;
use f4::frequency;
use rtfm::{app, Threshold};

// CONFIGURATION
const BAUD_RATE: Hertz = Hertz(115_200);

// TASKS & RESOURCES
app! {
    device: f4::stm32f429,

    tasks: {
        USART1: {
            path: loopback,
            resources: [USART1],
        },
    }
}

// INITIALIZATION PHASE
fn init(p: init::Peripherals) {
    frequency::init(p.RCC, p.PWR, p.FLASH);
    let serial = Serial(p.USART1);

    serial.init(BAUD_RATE.invert(), Some(p.DMA1), p.GPIOA, p.RCC);
    serial.listen(Event::Rxne);
}

// IDLE LOOP
fn idle() -> ! {
    // Sleep
    loop {
        rtfm::wfi();
    }
}

// TASKS
// Send back the received byte
fn loopback(_t: &mut Threshold, r: USART1::Resources) {
    let serial = Serial(&**r.USART1);

    let byte = serial.read().unwrap();
    serial.write(byte).unwrap();
}
