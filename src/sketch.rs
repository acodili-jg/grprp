use arduino_hal::port::mode::{Input, Output, PullUp};

use crate::{
    duration,
    millis::{millis, Millis},
    state::State,
};

macro_rules! pin_type {
    (a0) => {
        ::arduino_hal::hal::port::PC0
    };
    (a1) => {
        ::arduino_hal::hal::port::PC1
    };
    (a2) => {
        ::arduino_hal::hal::port::PC2
    };
    (a3) => {
        ::arduino_hal::hal::port::PC3
    };
    (a4) => {
        ::arduino_hal::hal::port::PC4
    };
    (a5) => {
        ::arduino_hal::hal::port::PC5
    };
    (d0) => {
        ::arduino_hal::hal::port::PD0
    };
    (d1) => {
        ::arduino_hal::hal::port::PD1
    };
    (d2) => {
        ::arduino_hal::hal::port::PD2
    };
    (d3) => {
        ::arduino_hal::hal::port::PD3
    };
    (d4) => {
        ::arduino_hal::hal::port::PD4
    };
    (d5) => {
        ::arduino_hal::hal::port::PD5
    };
    (d6) => {
        ::arduino_hal::hal::port::PD6
    };
    (d7) => {
        ::arduino_hal::hal::port::PD7
    };
    (d8) => {
        ::arduino_hal::hal::port::PB0
    };
    (d9) => {
        ::arduino_hal::hal::port::PB1
    };
    (d10) => {
        ::arduino_hal::hal::port::PB2
    };
    (d11) => {
        ::arduino_hal::hal::port::PB3
    };
    (d12) => {
        ::arduino_hal::hal::port::PB4
    };
    (d13) => {
        ::arduino_hal::hal::port::PB5
    };
}

macro_rules! decl_sketch {
    {
        $($pin_field:ident : pin!( $mode:ty, $pin:ident ) $({ $init:stmt })?),* $(,)?
        @
        $($field:ident : $field_t:ty = $default:expr),* $(,)?
    } => {
        #[allow(dead_code, unused_variables)]
        pub struct Sketch {
            $($pin_field : ::arduino_hal::port::Pin<$mode, pin_type!($pin)>,)*
            $($field : $field_t,)*
        }

        impl Sketch {
            #[allow(unused_mut, clippy::too_many_arguments)]
            #[must_use]
            pub fn new(
                $(mut $pin_field : ::arduino_hal::port::Pin<$mode, pin_type!($pin)>,)*
            ) -> Self {
                $($($init)?)*
                Sketch {
                    $($pin_field,)*
                    $($field : $default,)*
                }
            }
        }

        #[macro_export]
        macro_rules! sketch {
            ($pins:expr) => {
                sketch::Sketch::new(
                    $(pin_mode!($pins.$pin, $mode),)*
                )
            };
        }
    };
}

decl_sketch! {
    stop: pin!(Input<PullUp>, d0),
    start: pin!(Input<PullUp>, d1),
    ready: pin!(Output, d2),
    blender: pin!(Output, d3),
    heater: pin!(Output, d4),
    mixer: pin!(Output, d5),
    separator_hatch_enable: pin!(Output, d6),
    separator_hatch_direction: pin!(Output, d7),
    input_hatch_lock_direction: pin!(Output, d8),
    input_hatch_lock_enable: pin!(Output, d9),
    water_pump: pin!(Output, d10),
    lower_drain_pump: pin!(Output, d11) { lower_drain_pump.set_high() },
    upper_drain_pump: pin!(Output, d12) { upper_drain_pump.set_high() },
    @
    state: State = State::default(),
    last_ms: Millis = millis(),
    stopping: bool = false,
}

impl Sketch {
    #[must_use]
    pub const fn state(&self) -> State {
        self.state
    }

    #[allow(clippy::too_many_lines)] // TODO - address
    pub fn invoke(&mut self) {
        let curr_ms = millis();
        let delta_ms = curr_ms.wrapping_sub(self.last_ms);
        let stopping = self.stopping | self.stop.is_low();
        self.stopping = stopping;

        macro_rules! transition_to {
            ($state:ident) => {
                self.state = $crate::state::State::$state;
                self.last_ms = curr_ms;
            };
        }

        #[allow(clippy::match_same_arms)]
        match self.state {
            State::InitialDraining if delta_ms < duration::DRAINING => {}
            State::InitialDraining => {
                transition_to!(InitialIdling);
                self.lower_drain_pump.set_low();
                self.ready.set_high();
                self.upper_drain_pump.set_low();
            }

            State::InitialIdling if self.start.is_high() => {}
            State::InitialIdling => {
                transition_to!(InitialLocking);
                self.input_hatch_lock_direction.set_low();
                self.input_hatch_lock_enable.set_high();
            }

            State::InitialLocking if self.start.is_high() => {
                transition_to!(InitialUnlocking);
                self.input_hatch_lock_direction.set_high();
            }
            State::InitialLocking if delta_ms < duration::LOCKING => {}
            State::InitialLocking => {
                transition_to!(InitialSetupSeparatorOpening);
                self.ready.set_low();
                self.input_hatch_lock_enable.set_low();
                self.separator_hatch_enable.set_high();
            }

            State::InitialUnlocking if delta_ms < duration::LOCKING => {}
            State::InitialUnlocking => {
                transition_to!(InitialIdling);
                self.input_hatch_lock_enable.set_low();
            }

            State::InitialSetupSeparatorOpening if delta_ms < duration::SEPARATOR_TRANSITION => {}
            State::InitialSetupSeparatorOpening => {
                transition_to!(InitialSetupWaterPumping);
                self.separator_hatch_enable.set_low();
                self.water_pump.set_high();
            }

            State::InitialSetupWaterPumping if delta_ms < duration::SOAK_WATER_PUMPING => {}
            State::InitialSetupWaterPumping => {
                transition_to!(InitialSetupSeparatorClosing);
                self.water_pump.set_low();
                self.separator_hatch_direction.set_high();
                self.separator_hatch_enable.set_high();
            }

            State::InitialSetupSeparatorClosing if delta_ms < duration::SEPARATOR_TRANSITION => {}
            State::InitialSetupSeparatorClosing => {
                transition_to!(SoakWaterPumping);
                self.separator_hatch_enable.set_low();
                self.water_pump.set_high();
            }

            State::SoakWaterPumping if stopping => {
                transition_to!(SoakWaterDraining);
                self.upper_drain_pump.set_high();
                self.water_pump.set_low();
            }
            State::SoakWaterPumping if delta_ms < duration::SOAK_WATER_PUMPING => {}
            State::SoakWaterPumping => {
                transition_to!(SoakWaterHeating);
                self.water_pump.set_low();
                self.heater.set_high();
            }

            State::SoakWaterHeating if stopping => {
                transition_to!(SoakWaterDraining);
                self.heater.set_low();
                self.upper_drain_pump.set_high();
            }
            State::SoakWaterHeating if delta_ms < duration::HEATING => {}
            State::SoakWaterHeating => {
                transition_to!(SoakWaterHeatedMixing);
                self.mixer.set_high();
            }

            State::SoakWaterHeatedMixing if stopping => {
                transition_to!(SoakWaterDraining);
                self.heater.set_low();
                self.mixer.set_low();
                self.upper_drain_pump.set_high();
            }
            State::SoakWaterHeatedMixing if delta_ms < duration::HEATED_MIXING => {}
            State::SoakWaterHeatedMixing => {
                transition_to!(SoakWaterMixing);
                self.heater.set_low();
            }

            State::SoakWaterMixing if delta_ms < duration::MIXING && !stopping => {}
            State::SoakWaterMixing => {
                transition_to!(SoakWaterDraining);
                self.mixer.set_low();
                self.upper_drain_pump.set_high();
            }

            State::SoakWaterDraining if delta_ms < duration::DRAINING => {}
            State::SoakWaterDraining if stopping => {
                transition_to!(Idling);
                self.upper_drain_pump.set_low();
                self.ready.set_high();
            }
            State::SoakWaterDraining => {
                transition_to!(RinseWaterPumping);
                self.water_pump.set_high();
            }

            State::RinseWaterPumping if delta_ms < duration::RINSING => {}
            State::RinseWaterPumping => {
                transition_to!(RinseWaterDraining);
                self.water_pump.set_low();
            }

            State::RinseWaterDraining if delta_ms < duration::DRAINING => {}
            State::RinseWaterDraining if stopping => {
                transition_to!(Idling);
                self.upper_drain_pump.set_low();
                self.ready.set_high();
            }
            State::RinseWaterDraining => {
                transition_to!(SeparatorOpening);
                self.upper_drain_pump.set_low();
                self.separator_hatch_direction.set_low();
                self.separator_hatch_enable.set_high();
            }

            State::SeparatorOpening if delta_ms < duration::SEPARATOR_TRANSITION => {}
            State::SeparatorOpening => {
                transition_to!(SeparatorHolding);
                self.separator_hatch_enable.set_low();
            }

            _ => { /* TODO */ }
        }
    }
}
