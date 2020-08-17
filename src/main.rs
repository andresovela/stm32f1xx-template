#![no_std]
#![no_main]

use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use cortex_m_rt::entry;

use stm32f1xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    rtt_init_print!(BlockIfFull, 16384);
    rprintln!("Hello, world!");
    loop {
        continue;
    }
}
