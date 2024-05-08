use crate::state::State;
use embedded_hal::digital::v2::OutputPin;

macro_rules! pin_mode_t {
    (Input) => { ::arduino_hal::port::mode::Input<::arduino_hal::port::mode::AnyInput> };
    (InputFloating) => { ::arduino_hal::port::mode::Input<::arduino_hal::port::mode::Floating> };
    (InputPullUp) => { ::arduino_hal::port::mode::Input<::arduino_hal::port::mode::PullUp> };
    (OpenDrain) => { ::arduino_hal::port::mode::OpenDrain };
    (OpenDrainHigh) => { ::arduino_hal::port::mode::OpenDrain };
    (Output) => { ::arduino_hal::port::mode::Output };
    (OutputHigh) => { ::arduino_hal::port::mode::Output };
}

macro_rules! sketch_builder {
    {
        params_t: {$($param_t:tt)*},
        fields: {$($field:tt)*},
        pins_init: {$($name:ident : $pin:ident into $mode:ident),* $(,)?},
        fields_init: {$($field_init:tt)*},
    } => {
        pub struct Sketch<$($param_t,)*> { $($field)* }

        #[macro_export]
        macro_rules! sketch {
            ($pins:expr) => { $crate::sketch::Sketch {
                $($name : pin_mode!($pins.$pin, $mode),)*
                $($field_init)*
            }};
        }

        impl<$($param_t,)*> Sketch<$($param_t,)*>
        where
            $($param_t: arduino_hal::port::PinOps,)*
        {
            pub fn invoke(&mut self) {
                let curr_ms = $crate::millis::millis();
                let delta_ms = curr_ms.wrapping_sub(self.last_ms);
                let _stop = self.stop.is_high();

                match self.state {
                    State::InitialIdling if self.start.is_high() => {
                        self.state = State::InitialLocking;
                        self.input_hatch_lock.set_high();
                    }
                    State::InitialLocking if delta_ms >= $crate::duration::LOCKING && self.start.is_high() => {
                        self.state = State::InitialSetupSeparatorOpening;
                        self.last_ms = curr_ms;
                        self.separator_hatch_direction.set_low();
                        self.separator_hatch_enable.set_high();
                    }
                    State::InitialLocking if self.start.is_low() => {
                        self.state = State::InitialIdling;
                        self.input_hatch_lock.set_low();
                    }
                    State::InitialSetupSeparatorOpening if delta_ms >= $crate::duration::SEPARATOR_TRANSITION => {
                        self.state = State::InitialSetupWaterPumping;
                        self.last_ms = curr_ms;
                        self.separator_hatch_enable.set_low();
                        self.water_pump.set_high();
                    }
                    State::InitialSetupWaterPumping if delta_ms >= $crate::duration::SOAK_WATER_PUMPING => {
                        self.state = State::InitialSetupSeparatorClosing;
                        self.last_ms = curr_ms;
                        self.water_pump.set_low();
                        self.separator_hatch_direction.set_high();
                        self.separator_hatch_enable.set_high();
                    }
                    State::InitialSetupSeparatorOpening if delta_ms >= $crate::duration::SEPARATOR_TRANSITION => {
                        self.state = State::SoakWaterPumping;
                        self.last_ms = curr_ms;
                        self.separator_hatch_enable.set_low();
                        self.water_pump.set_high();
                    }
                    _ => {/* TODO */},
                }
            }
        }
    };
    {
        params_t: {$($param_t:tt)*},
        fields: {$($field:tt)*},
        pins_init: {$($pin_init:tt)*},
        fields_init: {$($field_init:tt)*},
        $name:ident : pin!($mode:ident, $pin:ident : $pin_t:ident),
        $($rest:tt)*
    } => {
        sketch_builder! {
            params_t: {
                $($param_t)*
                $pin_t
            },
            fields: {
                $($field)*
               pub $name : arduino_hal::port::Pin<pin_mode_t!($mode), $pin_t>,
            },
            pins_init: {
                $($pin_init)*
                $name : $pin into $mode,
            },
            fields_init: {
                $($field_init)*
            },
            $($rest)*
        }
    };
    {
        params_t: {$($param_t:tt)*},
        fields: {$($field:tt)*},
        pins_init: {$($pin_init:tt)*},
        fields_init: {$($field_init:tt)*},
        $name:ident : $type:ty = $value:expr,
        $($rest:tt)*
    } => {
        sketch_builder! {
            params_t: {
                $($param_t)*
            },
            fields: {
                $($field)*
                pub $name : $type,
            },
            pins_init: {
                $($pin_init)*
            },
            fields_init: {
                $($field_init)*
                $name : $value,
            },
            $($rest)*
        }
    };
}

macro_rules! build_sketch {
    {$($all:tt)* } => {
        sketch_builder! {
            params_t: {},
            fields: {},
            pins_init: {},
            fields_init: {},
            $($all)*
        }
    };
}

build_sketch! {
    lower_drain_pump: pin!(Output, d0: LowerDrainPump),
    blender: pin!(Output, d1: BlenderPin),
    separator_hatch_direction: pin!(Output, d2: SeparatorHatchDirectionPin),
    separator_hatch_enable: pin!(Output, d3: SeparatorHatchEnablePin),
    upper_drain_pump: pin!(Output, d4: UpperDrainPumpPin),
    heater: pin!(Output, d5: HeaterPin),
    start: pin!(InputFloating, d7: StartPin),
    stop: pin!(InputFloating, d8: StopPin),
    ready: pin!(Output, d9: ReadPin),
    input_hatch_lock: pin!(Output, d10: InputHatchLockPin),
    mixer: pin!(Output, d11: MixerPin),
    water_pump: pin!(Output, d12: WaterPumpPin),

    state: State = State::InitialIdling,
    last_ms: u32 = millis(),
}
