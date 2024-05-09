#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use grprp::{millis, pin_mode, sketch};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    #[allow(clippy::unwrap_used)]
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    millis::init(&dp.TC0);
    // Enable interrupts (needed by `millis`)
    unsafe { avr_device::interrupt::enable() };

    let mut sketch = sketch!(pins);
    let mut led = pins.d13.into_output();

    let mut last_ms = millis();
    loop {
        let curr_ms = millis();
        if curr_ms.wrapping_sub(last_ms) >= 1_000 {
            led.toggle();
            last_ms = millis();
        } //
        sketch.invoke();
    }
}
