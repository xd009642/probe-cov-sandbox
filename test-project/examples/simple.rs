#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};


#[entry]
fn main() -> ! {
    hprintln!("hello world").unwrap();
    loop {}
}
