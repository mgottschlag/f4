//! User LEDs

use stm32f429::{GPIOG, RCC};

/// All the user LEDs
pub static LEDS: [Led; 2] = [
    Led { i: 13 },
    Led { i: 14 },
];

/// An LED
pub struct Led {
    i: u8,
}

impl Led {
    /// Turns off the LED
    pub fn off(&self) {
        // NOTE(safe) atomic write
        unsafe { (*GPIOG.get()).bsrr.write(|w| w.bits(1 << (self.i + 16))) }
    }

    /// Turns on the LED
    pub fn on(&self) {
        // NOTE(safe) atomic write
        unsafe { (*GPIOG.get()).bsrr.write(|w| w.bits(1 << self.i)) }
    }
}

/// Initializes all the user LEDs
pub fn init(gpiog: &GPIOG, rcc: &RCC) {
    // Power up peripherals
    rcc.ahb1enr.modify(|_, w| w.gpiogen().set_bit());

    // Configure pins 8-15 as outputs
    gpiog
        .moder
        .modify(
            |_, w| {
                w.moder13()
                    .bits(1)
                    .moder14()
                    .bits(1)
            },
        );
}
