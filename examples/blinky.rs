//! Blinks an LED
#![deny(unsafe_code)]
#![deny(warnings)]
#![feature(proc_macro)]
#![no_std]

extern crate f4;
extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;

use cortex_m::peripheral::SystClkSource;
use f4::led::{self, LEDS};
use f4::frequency;
use rtfm::{app, Threshold};

// CONFIGURATION
const FREQUENCY: u32 = 16; // Hz

// TASKS & RESOURCES
app! {
    device: f4::stm32f429,

    resources: {
        static COUNTER: u32 = 0;
    },

    tasks: {
        SYS_TICK: {
            path: toggle,
            resources: [COUNTER],
        },
    },
}

// INITIALIZATION PHASE
fn init(p: init::Peripherals, _r: init::Resources) {
    led::init(p.GPIOG, p.RCC);

    frequency::init(p.RCC, p.PWR, p.FLASH);

    p.SYST.set_clock_source(SystClkSource::Core);
    p.SYST.set_reload(168_000_000 / FREQUENCY);
    p.SYST.enable_interrupt();
    p.SYST.enable_counter();
}

// IDLE LOOP
fn idle() -> ! {
    // Sleep
    loop {
        rtfm::wfi();
    }
}

// TASKS
// Toggle the state of the LED
fn toggle(_t: &mut Threshold, r: SYS_TICK::Resources) {
    **r.COUNTER = (**r.COUNTER + 1) & 31;

    if (**r.COUNTER & 16) != 0 {
        LEDS[0].on();
        LEDS[1].off();
    } else {
        LEDS[0].off();
        LEDS[1].on();
    }
}
