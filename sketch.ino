namespace duration {
    // Unless otherwise state, the following are in milliseconds (ms)

    const unsigned long DEFAULT_LONG  = 5000uL;
    const unsigned long DEFAULT_SHORT =  500uL;
    // Intentionally misspelt due to the pressence of a macro named `DEFAULT`
    const unsigned long DEFUALT       = 2000uL;

    const unsigned long BLENDING = DEFAULT_LONG;
    const unsigned long DRAINING = DEFAULT_LONG;
    const unsigned long HEATED_MIXING = DEFUALT;
    const unsigned long HEATING = DEFAULT_LONG;
    const unsigned long LOCKING = DEFAULT_SHORT;
    const unsigned long MIXING = DEFAULT_LONG;
    const unsigned long RINSING = DEFAULT_LONG;
    const unsigned long SEPARATOR_HOLDING = DEFAULT_LONG;
    const unsigned long SEPARATOR_TRANSITION = DEFUALT;
    const unsigned long WATER_PUMPING = DEFAULT_LONG;
}

namespace pin {
    const unsigned int START = 0;
    const unsigned int STOP = 1;
    const unsigned int READY = 2;
    const unsigned int BLENDER = 3;
    const unsigned int HEATER = 4;
    const unsigned int MIXER = 5;
    const unsigned int SEPARATOR_HATCH_ENABLE = 6;
    const unsigned int SEPARATOR_HATCH_DIRECTION = 7;
    const unsigned int INPUT_HATCH_LOCK_DIRECTION = 8;
    const unsigned int INPUT_HATCH_LOCK_ENABLE = 9;
    const unsigned int WATER_PUMP = 10;
    const unsigned int LOWER_DRAIN_PUMP = 11;
    const unsigned int UPPER_DRAIN_PUMP = 12;
    const unsigned int BLINK = 13;
}

namespace state {
    enum class State {
        INITIAL_DRAINING,
        INITIAL_IDLING,
        INITIAL_LOCKING,
        INITIAL_UNLOCKING,
        INITIAL_SETUP_SEPARATOR_OPENING,
        INITIAL_SETUP_WATER_PUMPING,
        INITIAL_SETUP_SEPARATOR_CLOSING,
        SOAK_WATER_PUMPING,
        SOAK_WATER_HEATING,
        SOAK_WATER_HEATED_MIXING,
        SOAK_WATER_MIXING,
        SOAK_WATER_DRAINING,
        RINSE_WATER_PUMPING,
        RINSE_WATER_DRAINING,
        SEPARATOR_OPENING,
        SEPARATOR_HOLDING,
        SEPARATOR_CLOSING,
        BLENDING,
        PULP_DRAINING,
        SETUP_SEPARATOR_OPENING,
        SETUP_WATER_PUMPING,
        SETUP_SEPARATOR_CLOSING,
        IDLING,
        LOCKING,
        UNLOCKING
    };

    bool is_idling(state::State state) {
        switch (state) {
            case State::INITIAL_IDLING:
            case State::IDLING:
                return true;
            default:
                return false;
        }
    }

    // Determines the state to override the current or the same state which
    // would that there is no change. As of writing this, TinkerCAD uses C++ 14
    // which does not have std::optional from C++ 17.
    State override_for(
        // compiler error when not qualified
        state::State curr,
        unsigned long delta_ms,
        bool starting,
        bool stopping
    ) {
        switch (curr) {
            case State::INITIAL_DRAINING: if (delta_ms >= duration::DRAINING) {
                return State::INITIAL_IDLING;
            } break;

            case State::INITIAL_IDLING: if (starting) {
                return State::INITIAL_LOCKING;
            } break;

            case State::INITIAL_LOCKING: if (!starting) {
                return State::INITIAL_UNLOCKING;
            } else if (delta_ms >= duration::LOCKING) {
                return State::INITIAL_SETUP_SEPARATOR_OPENING;
            } break;

            case State::INITIAL_UNLOCKING: if (delta_ms >= duration::LOCKING) {
                return State::INITIAL_IDLING;
            } break;

            case State::INITIAL_SETUP_SEPARATOR_OPENING: if (delta_ms >= duration::SEPARATOR_TRANSITION) {
                return State::INITIAL_SETUP_WATER_PUMPING;
            } break;

            case State::INITIAL_SETUP_WATER_PUMPING: if (delta_ms >= duration::WATER_PUMPING) {
                return State::INITIAL_SETUP_SEPARATOR_CLOSING;
            } break;

            case State::INITIAL_SETUP_SEPARATOR_CLOSING: if (delta_ms >= duration::SEPARATOR_TRANSITION) {
                return State::SOAK_WATER_PUMPING;
            } break;

            case State::SOAK_WATER_PUMPING: if (stopping) {
                return State::SOAK_WATER_DRAINING;
            } else if (delta_ms >= duration::WATER_PUMPING) {
                return State::SOAK_WATER_HEATING;
            } break;

            case State::SOAK_WATER_HEATING: if (stopping) {
                return State::SOAK_WATER_DRAINING;
            } else if (delta_ms >= duration::HEATING) {
                return State::SOAK_WATER_HEATED_MIXING;
            } break;

            case State::SOAK_WATER_HEATED_MIXING: if (stopping) {
                return State::SOAK_WATER_DRAINING;
            } else if (delta_ms >= duration::HEATED_MIXING) {
                return State::SOAK_WATER_MIXING;
            } break;

            case State::SOAK_WATER_MIXING: if (stopping || delta_ms >= duration::MIXING) {
                return State::SOAK_WATER_DRAINING;
            } break;

            case State::SOAK_WATER_DRAINING: if (delta_ms >= duration::DRAINING) {
                if (stopping) {
                    return State::IDLING;
                } else {
                    return State::RINSE_WATER_PUMPING;
                }
            } break;

            case State::RINSE_WATER_PUMPING: if (stopping || delta_ms >= duration::RINSING) {
                return State::RINSE_WATER_DRAINING;
            } break;

            case State::RINSE_WATER_DRAINING: if (delta_ms >= duration::DRAINING) {
                if (stopping) {
                    return State::IDLING;
                } else {
                    return State::SEPARATOR_OPENING;
                }
            } break;

            case State::SEPARATOR_OPENING: if (delta_ms >= duration::SEPARATOR_TRANSITION) {
                return State::SEPARATOR_HOLDING;
            } break;

            case State::SEPARATOR_HOLDING: if (delta_ms >= duration::SEPARATOR_HOLDING) {
                return State::SEPARATOR_CLOSING;
            } break;

            case State::SEPARATOR_CLOSING: if (delta_ms >= duration::SEPARATOR_TRANSITION) {
                return State::BLENDING;
            } break;

            case State::BLENDING: if (delta_ms >= duration::BLENDING) {
                return State::PULP_DRAINING;
            } break;

            case State::PULP_DRAINING: if (delta_ms >= duration::DRAINING) {
                return State::SETUP_SEPARATOR_OPENING;
            } break;

            case State::SETUP_SEPARATOR_OPENING: if (delta_ms >= duration::SEPARATOR_TRANSITION) {
                return State::SETUP_WATER_PUMPING;
            } break;

            case State::SETUP_WATER_PUMPING: if (delta_ms >= duration::WATER_PUMPING) {
                return State::SETUP_SEPARATOR_CLOSING;
            } break;

            case State::SETUP_SEPARATOR_CLOSING: if (delta_ms >= duration::SEPARATOR_TRANSITION) {
                return State::IDLING;
            } break;

            case State::IDLING: if (starting) {
                return State::LOCKING;
            } break;

            case State::LOCKING: if (!starting) {
                return State::UNLOCKING;
            } else if (delta_ms >= duration::LOCKING) {
                return State::SOAK_WATER_PUMPING;
            } break;

            case State::UNLOCKING: if (delta_ms >= duration::LOCKING) {
                return State::IDLING;
            } break;
        }

        // default return-value
        return curr;
    }
}

using state::State;

uint_fast16_t affected_components(state::State state) {
    switch (state) {
        case State::INITIAL_DRAINING:
            return UINT16_C(0)
                | 1 << pin::LOWER_DRAIN_PUMP
                | 1 << pin::UPPER_DRAIN_PUMP;
        case State::INITIAL_IDLING:
            return UINT16_C(0)
                | 1 << pin::READY;
        case State::INITIAL_LOCKING:
            return UINT16_C(0)
                | 1 << pin::INPUT_HATCH_LOCK_ENABLE
                | 1 << pin::UPPER_DRAIN_PUMP;
        case State::INITIAL_UNLOCKING:
            return UINT16_C(0)
                | 1 << pin::INPUT_HATCH_LOCK_DIRECTION
                | 1 << pin::INPUT_HATCH_LOCK_ENABLE
                | 1 << pin::READY;
        case State::INITIAL_SETUP_SEPARATOR_OPENING:
            return UINT16_C(0)
                | 1 << pin::SEPARATOR_HATCH_ENABLE;
        case State::INITIAL_SETUP_WATER_PUMPING:
            return UINT16_C(0)
                | 1 << pin::WATER_PUMP;
        case State::INITIAL_SETUP_SEPARATOR_CLOSING:
            return UINT16_C(0)
                | 1 << pin::SEPARATOR_HATCH_DIRECTION
                | 1 << pin::SEPARATOR_HATCH_ENABLE;
        case State::SOAK_WATER_PUMPING:
            return UINT16_C(0)
                | 1 << pin::WATER_PUMP;
        case State::SOAK_WATER_HEATING:
            return UINT16_C(0)
                | 1 << pin::HEATER;
        case State::SOAK_WATER_HEATED_MIXING:
            return UINT16_C(0)
                | 1 << pin::HEATER
                | 1 << pin::MIXER;
        case State::SOAK_WATER_MIXING:
            return UINT16_C(0)
                | 1 << pin::MIXER;
        case State::SOAK_WATER_DRAINING:
            return UINT16_C(0)
                | 1 << pin::UPPER_DRAIN_PUMP
                | 1 << pin::WATER_PUMP;
        case State::RINSE_WATER_PUMPING:
            return UINT16_C(0)
                | 1 << pin::UPPER_DRAIN_PUMP;
        case State::RINSE_WATER_DRAINING:
            return UINT16_C(0)
                | 1 << pin::UPPER_DRAIN_PUMP;
        case State::SEPARATOR_OPENING:
            return UINT16_C(0)
                | 1 << pin::SEPARATOR_HATCH_ENABLE;
        case State::SEPARATOR_HOLDING:
            return UINT16_C(0);
        case State::SEPARATOR_CLOSING:
            return UINT16_C(0)
                | 1 << pin::SEPARATOR_HATCH_DIRECTION
                | 1 << pin::SEPARATOR_HATCH_ENABLE;
        case State::BLENDING:
            return UINT16_C(0)
                | 1 << pin::BLENDER;
        case State::PULP_DRAINING:
            return UINT16_C(0)
                | 1 << pin::LOWER_DRAIN_PUMP;
        case State::SETUP_SEPARATOR_OPENING:
            return UINT16_C(0)
                | 1 << pin::SEPARATOR_HATCH_ENABLE;
        case State::SETUP_WATER_PUMPING:
            return UINT16_C(0)
                | 1 << pin::WATER_PUMP;
        case State::SETUP_SEPARATOR_CLOSING:
            return UINT16_C(0)
                | 1 << pin::SEPARATOR_HATCH_DIRECTION
                | 1 << pin::SEPARATOR_HATCH_ENABLE;
        case State::IDLING:
            return UINT16_C(0)
                | 1 << pin::READY;
        case State::LOCKING:
            return UINT16_C(0)
                | 1 << pin::INPUT_HATCH_LOCK_ENABLE
                | 1 << pin::UPPER_DRAIN_PUMP;
        case State::UNLOCKING:
            return UINT16_C(0)
                | 1 << pin::INPUT_HATCH_LOCK_DIRECTION
                | 1 << pin::INPUT_HATCH_LOCK_ENABLE
                | 1 << pin::READY;
    }
}

void toggle_components(state::State state, bool voltage) {
    uint_fast8_t pin = 0;
    uint_fast64_t components = affected_components(state);

    // Consider only d0 to d13 pins
    while (components != 0 && pin <= 13) {
        if (0 != (components & 1)) {
            digitalWrite(pin, voltage);
        }
        pin++;
        components >>= 1;
    }
}

unsigned long __blink_last_ms;
unsigned long __blink_voltage;

unsigned long __state_last_ms;
bool __stopping;
State __state;

void setup() {
    pinMode(pin::STOP, INPUT_PULLUP);
    pinMode(pin::START, INPUT_PULLUP);
    pinMode(pin::READY, OUTPUT);
    pinMode(pin::BLENDER, OUTPUT);
    pinMode(pin::HEATER, OUTPUT);
    pinMode(pin::MIXER, OUTPUT);
    pinMode(pin::SEPARATOR_HATCH_ENABLE, OUTPUT);
    pinMode(pin::SEPARATOR_HATCH_DIRECTION, OUTPUT);
    pinMode(pin::INPUT_HATCH_LOCK_DIRECTION, OUTPUT);
    pinMode(pin::INPUT_HATCH_LOCK_ENABLE, OUTPUT);
    pinMode(pin::WATER_PUMP, OUTPUT);
    pinMode(pin::LOWER_DRAIN_PUMP, OUTPUT);
    pinMode(pin::UPPER_DRAIN_PUMP, OUTPUT);
    pinMode(pin::BLINK, OUTPUT);

    __blink_last_ms = millis();
    __blink_voltage = LOW;

    __state_last_ms = millis();
    __stopping = false;
    __state = State::INITIAL_DRAINING;

    toggle_components(__state, HIGH);
}

bool update(unsigned long delta_ms) {
    const bool starting = !digitalRead(pin::START);
    const bool stopping = __stopping || !digitalRead(pin::STOP);
    __stopping = stopping;

    const State curr_state = __state;
    const State next_state = state::override_for(curr_state, delta_ms, starting, stopping);

    if (curr_state != next_state) {
        toggle_components(curr_state, LOW);

        __state = next_state;
        __stopping = stopping && !state::is_idling(curr_state);

        toggle_components(next_state, HIGH);

        return true;
    } else {
        return false;
    }
}

void loop() {
    const unsigned long curr_ms = millis();

    if (update((unsigned long) (curr_ms - __state_last_ms))) {
        __state_last_ms = curr_ms;

        // Resync blinks
        digitalWrite(pin::BLINK, __blink_voltage = LOW);
        __blink_last_ms = curr_ms;
    } else if ((unsigned long) (curr_ms - __blink_last_ms) >= 500) {
        digitalWrite(pin::BLINK, __blink_voltage ^= HIGH);
        __blink_last_ms = curr_ms;
    }
}
