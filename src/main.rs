#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use grprp::{
    millis::{millis, Millis},
    pin_mode, sketch,
};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    #[allow(clippy::unwrap_used)]
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    grprp::millis::init(&dp.TC0);
    // Enable interrupts (needed by `millis`)
    unsafe { avr_device::interrupt::enable() };

    let mut led = pins.d13.into_output();
    let mut sketch = sketch!(pins);

    let mut blink_last_ms = millis();
    let mut sketch_last_ms = blink_last_ms;

    loop {
        let curr_ms = millis();

        if sketch.update(curr_ms.wrapping_sub(sketch_last_ms)) {
            sketch_last_ms = curr_ms;

            // Resync d13 blinks
            led.set_low();
            blink_last_ms = curr_ms;
        } else if curr_ms.wrapping_sub(blink_last_ms) >= Millis(500) {
            led.toggle();
            blink_last_ms = curr_ms;
        }
    }
}
