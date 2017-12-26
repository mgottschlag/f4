//! Definition of bus frequency details for f4 and functions to setup clocks.

use stm32f429::{RCC, PWR, FLASH};

macro_rules! frequency {
    ($FREQUENCY:expr) => {
        use time::*;

        /// Frequency
        pub const FREQUENCY: u32 = $FREQUENCY;

        /// Unit of time
        #[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
        pub struct Ticks(pub u32);

        impl Ticks {
            /// Applies the function `f` to the inner value
            pub fn map<F>(self, f: F) -> Self
                where F: FnOnce(u32) -> u32,
            {
                Ticks(f(self.0))
            }
        }

        impl From<Ticks> for Microseconds {
            fn from(ticks: Ticks) -> Self {
                Microseconds(ticks.0 / (FREQUENCY / 1_000_000))
            }
        }

        impl From<Ticks> for Milliseconds {
            fn from(ticks: Ticks) -> Self {
                Milliseconds(ticks.0 / (FREQUENCY / 1_000))
            }
        }

        impl From<Ticks> for Seconds {
            fn from(ticks: Ticks) -> Self {
                Seconds(ticks.0 / FREQUENCY)
            }
        }

        impl From<IHertz> for Ticks {
            fn from(ihz: IHertz) -> Ticks {
                Ticks(FREQUENCY / ihz.0)
            }
        }

        impl From<Microseconds> for Ticks {
            fn from(us: Microseconds) -> Ticks {
                Ticks(us.0 * (FREQUENCY / 1_000_000))
            }
        }

        impl From<Milliseconds> for Ticks {
            fn from(ms: Milliseconds) -> Ticks {
                Ticks(ms.0 * (FREQUENCY / 1_000))
            }
        }

        impl From<Seconds> for Ticks {
            fn from(s: Seconds) -> Ticks {
                Ticks(s.0 * FREQUENCY)
            }
        }

        impl Into<u32> for Ticks {
            fn into(self) -> u32 {
                self.0
            }
        }
    }
}

/// Advance High-performance Bus (AHB)
pub mod ahb {
    frequency!(168_000_000);
}

/// Advance Peripheral Bus 1 (APB1)
pub mod apb1 {
    frequency!(42_000_000);
}

/// Advance Peripheral Bus 2 (APB2)
pub mod apb2 {
    frequency!(84_000_000);
}

/// Initializes the system clocks for the given clock speeds.
pub fn init(rcc: &RCC, pwr: &PWR, flash: &FLASH) {
    // Reset RCC config to its default state:

    // Enable HSI
    rcc.cr.modify(|_, w| w.hsion().set_bit());
    // Reset CFGR
    rcc.cfgr.reset();
    // Disable HSE, CSS and the PLL
    rcc.cr.modify(|_, w| {
        w.hseon().clear_bit()
            .csson().clear_bit()
            .pllon().clear_bit()
    });
    // Reset PLLCFGR
    rcc.pllcfgr.reset();
    // Reset HSEBYP
    rcc.cr.modify(|_, w| w.hsebyp().clear_bit());
    // Disable all interrupts
    rcc.cir.reset();
    // Enable HSE
    rcc.cr.modify(|_, w| w.hseon().set_bit());
    // Wait until HSE is ready
    while rcc.cr.read().hserdy().bit_is_clear() {}
    // Select regulator scale 1 mode, system frequency up to 168 MHz
    rcc.apb1enr.modify(|_, w| w.pwren().set_bit());
    pwr.cr.modify(|_, w| w.vos().scale1());
    // HCLK = SYSCLK / 1
    // PCLK2 = HCLK / 2
    // PCLK1 = HCLK / 4
    rcc.cfgr.modify(|_, w| w.hpre().div1().ppre2().div2().ppre1().div4());
    // Configure the PLL
    const PLL_M: u8 = 4;
    const PLL_N: u16 = 168;
    const PLL_P: u8 = 2;
    const PLL_Q: u8 = 7;
    rcc.pllcfgr.write(|w| unsafe {
        w.pllm().bits(PLL_M)
            .plln().bits(PLL_N)
            .pllp().bits((PLL_P >> 1) - 1)
            .pllsrc().hse()
            .pllq().bits(PLL_Q)
    });
    // Enable the PLL
    rcc.cr.modify(|_, w| w.pllon().set_bit());
    // Wait until the PLL is ready
    while rcc.cr.read().pllrdy().bit_is_clear() {}
    // Configure prefetch and caches
    flash.acr.write(|w| w.prften().set_bit().icen().set_bit().dcen().set_bit().latency().bits(5));
    // Select PLL as system clock source
    rcc.cfgr.modify(|_, w| w.sw0().clear_bit().sw1().set_bit());
    // Wait until PLL is used as system clock source
    let mut cfgr = rcc.cfgr.read();
    while !(cfgr.sw1().bit_is_set() && cfgr.sw0().bit_is_clear()) {
        cfgr = rcc.cfgr.read();
    }
}

