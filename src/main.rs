#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use grprp::{
    duration,
    millis::{millis, millis_init},
    state::State,
};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    millis_init(dp.TC0);

    // Enable interrupts globally (needed by `millis`)
    unsafe { avr_device::interrupt::enable() };

    let mut lower_drain_pump = pins.d0.into_output();
    let mut blender = pins.d1.into_output();
    let mut separator_hatch_direction = pins.d2.into_output();
    let mut separator_hatch_enable = pins.d3.into_output();
    let mut upper_drain_pump = pins.d4.into_output();
    let mut heater = pins.d5.into_output();
    let start = pins.d6;
    let stop = pins.d7;
    let mut ready = pins.d8.into_output();
    let mut input_hatch_lock = pins.d9.into_output();
    let mut mixer = pins.d10.into_output();
    let mut water_pump = pins.d11.into_output();

    let mut state = State::InitialIdling;
    let mut last_ms = millis();

    loop {
        let curr_ms = millis();
        let delta_ms = curr_ms.wrapping_sub(last_ms);

        match state {
            State::InitialIdling if start.is_high() => {
                input_hatch_lock.set_high();
            }
            State::InitialLocking if delta_ms >= duration::LOCKING && start.is_high() => {
                state = State::InitialSetupSeparatorOpening;
                last_ms = curr_ms;
                separator_hatch_direction.set_low();
                separator_hatch_enable.set_high();
            }
            State::InitialLocking if start.is_low() => {
                state = State::InitialIdling;
                input_hatch_lock.set_low();
            }
            State::InitialSetupSeparatorOpening if delta_ms >= duration::SEPARATOR_TRANSITION => {
                state = State::InitialSetupWaterPumping;
                last_ms = curr_ms;
                separator_hatch_enable.set_low();
                water_pump.set_high();
            }
            State::InitialSetupWaterPumping if delta_ms >= duration::SOAK_WATER_PUMPING => {
                state = State::InitialSetupSeparatorClosing;
                last_ms = curr_ms;
                water_pump.set_low();
                separator_hatch_direction.set_high();
                separator_hatch_enable.set_high();
            }
            State::InitialSetupSeparatorOpening if delta_ms >= duration::SEPARATOR_TRANSITION => {
                state = State::SoakWaterPumping;
                last_ms = curr_ms;
                separator_hatch_enable.set_low();
                water_pump.set_high();
            }
            _ => todo!(),
        }
    }
}
