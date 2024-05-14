use arduino_hal::port::mode::{Input, Output, PullUp};

use crate::{
    millis::{millis, Millis},
    state::{self, State},
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
                let mut this = Sketch {
                    $($pin_field,)*
                    $($field : $default,)*
                };
                this.toggle_components();
                this
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
    lower_drain_pump: pin!(Output, d11),
    upper_drain_pump: pin!(Output, d12),
    @
    state: State = State::default(),
    last_ms: Millis = millis(),
    stopping: bool = false,
}

impl Sketch {
    #[inline]
    #[must_use]
    pub const fn state(&self) -> State {
        self.state
    }

    #[inline]
    pub fn update(&mut self, delta_ms: Millis) -> bool {
        let starting = self.start.is_low();
        let stopping = self.stopping || self.stop.is_low();
        self.stopping = stopping;

        let curr_state = self.state;
        let overriding_state = state::override_for(curr_state, delta_ms, starting, stopping);

        if let Some(next_state) = overriding_state {
            self.toggle_components();

            self.state = next_state;
            self.stopping = self.stopping && !curr_state.is_idling();

            self.toggle_components();

            true
        } else {
            false
        }
    }

    /// Toggles components based on the current state.
    ///
    /// Usually called before and after a change in state with the former for
    /// leaving and the latter for entering.
    fn toggle_components(&mut self) {
        macro_rules! implementation {
            ($($state:ident ( $($pin:ident),* $(,)? )),* $(,)?) => {
                match self.state {
                    $($crate::state::State::$state => { $(self.$pin.toggle();)* })*
                }
            };
        }

        implementation!(
            InitialDraining(lower_drain_pump, upper_drain_pump),
            InitialIdling(ready),
            InitialLocking(input_hatch_lock_enable, ready),
            InitialUnlocking(input_hatch_lock_direction, input_hatch_lock_enable, ready),
            InitialSetupSeparatorOpening(separator_hatch_enable),
            InitialSetupWaterPumping(water_pump),
            InitialSetupSeparatorClosing(separator_hatch_direction, separator_hatch_enable),
            SoakWaterPumping(water_pump),
            SoakWaterHeating(heater),
            SoakWaterHeatedMixing(heater, mixer),
            SoakWaterMixing(mixer),
            SoakWaterDraining(upper_drain_pump),
            RinseWaterPumping(upper_drain_pump, water_pump),
            RinseWaterDraining(upper_drain_pump),
            SeparatorOpening(separator_hatch_enable),
            SeparatorHolding(),
            SeparatorClosing(separator_hatch_direction, separator_hatch_enable),
            Blending(blender),
            PulpDraining(lower_drain_pump),
            SetupSeparatorOpening(separator_hatch_enable),
            SetupWaterPumping(water_pump),
            SetupSeparatorClosing(separator_hatch_direction, separator_hatch_enable),
            Idling(ready),
            Locking(input_hatch_lock_enable, ready),
            Unlocking(input_hatch_lock_direction, input_hatch_lock_enable, ready),
        );
    }
}
