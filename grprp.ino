// TODO add state as delay and to differentiate continue from cancel

/// Durations in milliseconds (ms).
namespace duration {
    constexpr unsigned long DEFAULT_LONG = 1000uL;
    constexpr unsigned long DEFAULT_NORMAL = 1000uL;
    constexpr unsigned long DEFAULT_SHORT = 1000uL;

    constexpr unsigned long BLINKING = 250uL;
    constexpr unsigned long HEATED_MIXING = DEFAULT_LONG;
    constexpr unsigned long MIXING = DEFAULT_LONG;
    constexpr unsigned long RINSE_BLEND_PAUSE = DEFAULT_NORMAL;
    constexpr unsigned long RINSING = DEFAULT_LONG;
    constexpr unsigned long WATER_DRAINING = DEFAULT_NORMAL;
    constexpr unsigned long WATER_PUMPING = DEFAULT_LONG;
}

/// Pin naming/aliasing.
namespace pin {
    constexpr uint8_t START = 2;
    constexpr uint8_t CANCEL_OR_CONTINUE = 3;
    constexpr uint8_t SEPARATOR_CLOSED = 4;
    constexpr uint8_t LOWER_WATER_PUMP = 5;
    constexpr uint8_t UPPER_WATER_PUMP = 6;
    constexpr uint8_t LOWER_DRAIN_CLOSED = 7;
    constexpr uint8_t UPPER_DRAIN_CLOSED = 8;
    constexpr uint8_t HEATER = 9;
    constexpr uint8_t BLENDER = 10;
    constexpr uint8_t MIXER = 11;
    constexpr uint8_t BLINK_1 = 12;
    constexpr uint8_t BLINK_2 = 13;
}

/// State definitions.
namespace state {
    /// Enumerates the valid states of this sketch.
    enum class State: uint8_t {
        /// Ready to start.
        READY,
        /// Manual confirmation components are properly closed/opened.
        PRE_MIXING_CHECKING,
        /// Intermediate state to detect unpress and avoid cancels.
        PRE_MIXING_GUARD,
        /// Pump water, used for mixing.
        MIXING_WATER_PUMPING,
        /// Mixing while also heating the water.
        HEATED_MIXING,
        /// Mixing.
        MIXING,
        /// Manual confirmation components are properly closed/opened.
        PRE_RINSING_CHECKING,
        /// Intermediate state to detect unpress and avoid cancels.
        PRE_RINSING_GUARD,
        /// Draining the water used in mixing.
        MIXING_WATER_DRAINING,
        /// Rinsing with running water.
        RINSING,
        /// Drain remaining rinse water.
        RINSE_WATER_DRAINING,
        /// Manual confirmation components are properly closed/opened.
        POST_RINSING_CHECKING,
        /// Intermediate state to detect unpress and avoid cancels.
        POST_RINSING_GUARD,
        /// A quick pause to allow most materials to slip down the open separator.
        RINSE_BLEND_PAUSE,
        /// Manual confirmation components are properly closed/opened.
        PRE_BLENDING_CHECKING,
        /// Intermediate state to detect unpress and avoid cancels.
        PRE_BLENDING_GUARD,
        /// Pump water, used for blending.
        BLENDING_WATER_PUMPING,
        /// Blending.
        BLENDING,
        /// Manual confirmation components are properly closed/opened.
        POST_BLENDING_CHECKING,
        /// Draining the water used in blending.
        BLENDING_WATER_DRAINING
    };

    /**
     * The pins that will receive {@link digitalWrite} for a state.
     * <p>
     * A set of pins encoded as binary starting the least significant bit to the
     * most significant bit with {@code 1}s meaning its part of the set.
     *
     * @param state the state
     * @return a set of pins in binary representation
     */
    uint16_t pins(state::State state) {
        switch (state) {
            case State::READY:
                return UINT16_C(0);
            case State::PRE_MIXING_CHECKING:
                return UINT16_C(0)
                    | UINT16_C(1) << pin::BLINK_1
                    | UINT16_C(1) << pin::SEPARATOR_CLOSED
                    | UINT16_C(1) << pin::LOWER_DRAIN_CLOSED
                    | UINT16_C(1) << pin::UPPER_DRAIN_CLOSED;
            case State::PRE_MIXING_GUARD:
                return UINT16_C(0)
                    | UINT16_C(1) << pin::BLINK_1
                    | UINT16_C(1) << pin::BLINK_2
                    | UINT16_C(1) << pin::SEPARATOR_CLOSED
                    | UINT16_C(1) << pin::LOWER_DRAIN_CLOSED
                    | UINT16_C(1) << pin::UPPER_DRAIN_CLOSED;
            case State::MIXING_WATER_PUMPING:
                return UINT16_C(0)
                    | UINT16_C(1) << pin::BLINK_1
                    | UINT16_C(1) << pin::SEPARATOR_CLOSED
                    | UINT16_C(1) << pin::UPPER_WATER_PUMP
                    | UINT16_C(1) << pin::LOWER_DRAIN_CLOSED
                    | UINT16_C(1) << pin::UPPER_DRAIN_CLOSED;
            case State::HEATED_MIXING:
                return UINT16_C(0)
                    | UINT16_C(1) << pin::BLINK_1
                    | UINT16_C(1) << pin::SEPARATOR_CLOSED
                    | UINT16_C(1) << pin::LOWER_DRAIN_CLOSED
                    | UINT16_C(1) << pin::UPPER_DRAIN_CLOSED
                    | UINT16_C(1) << pin::HEATER
                    | UINT16_C(1) << pin::MIXER;
            case State::MIXING:
                return UINT16_C(0)
                    | UINT16_C(1) << pin::BLINK_1
                    | UINT16_C(1) << pin::SEPARATOR_CLOSED
                    | UINT16_C(1) << pin::LOWER_DRAIN_CLOSED
                    | UINT16_C(1) << pin::UPPER_DRAIN_CLOSED
                    | UINT16_C(1) << pin::MIXER;
            case State::PRE_RINSING_CHECKING:
                return UINT16_C(0)
                    | UINT16_C(1) << pin::BLINK_1
                    | UINT16_C(1) << pin::BLINK_2
                    | UINT16_C(1) << pin::SEPARATOR_CLOSED
                    | UINT16_C(1) << pin::LOWER_DRAIN_CLOSED;
            case State::PRE_RINSING_GUARD:
                return UINT16_C(0)
                    | UINT16_C(1) << pin::BLINK_1
                    | UINT16_C(1) << pin::SEPARATOR_CLOSED
                    | UINT16_C(1) << pin::LOWER_DRAIN_CLOSED;
            case State::MIXING_WATER_DRAINING:
                return UINT16_C(0)
                    | UINT16_C(1) << pin::BLINK_1
                    | UINT16_C(1) << pin::SEPARATOR_CLOSED
                    | UINT16_C(1) << pin::LOWER_DRAIN_CLOSED;
            case State::RINSING:
                return UINT16_C(0)
                    | UINT16_C(1) << pin::BLINK_1
                    | UINT16_C(1) << pin::SEPARATOR_CLOSED
                    | UINT16_C(1) << pin::UPPER_WATER_PUMP
                    | UINT16_C(1) << pin::LOWER_DRAIN_CLOSED;
            case State::RINSE_WATER_DRAINING:
                return UINT16_C(0)
                    | UINT16_C(1) << pin::BLINK_1
                    | UINT16_C(1) << pin::SEPARATOR_CLOSED
                    | UINT16_C(1) << pin::LOWER_DRAIN_CLOSED;
            case State::POST_RINSING_CHECKING:
                return UINT16_C(0)
                    | UINT16_C(1) << pin::BLINK_1
                    | UINT16_C(1) << pin::BLINK_2
                    | UINT16_C(1) << pin::LOWER_DRAIN_CLOSED
                    | UINT16_C(1) << pin::UPPER_DRAIN_CLOSED;
            case State::POST_RINSING_GUARD:
                return UINT16_C(0)
                    | UINT16_C(1) << pin::BLINK_1
                    | UINT16_C(1) << pin::LOWER_DRAIN_CLOSED
                    | UINT16_C(1) << pin::UPPER_DRAIN_CLOSED;
            case State::RINSE_BLEND_PAUSE:
                return UINT16_C(0)
                    | UINT16_C(1) << pin::BLINK_1
                    | UINT16_C(1) << pin::LOWER_DRAIN_CLOSED
                    | UINT16_C(1) << pin::UPPER_DRAIN_CLOSED;
            case State::PRE_BLENDING_CHECKING:
                return UINT16_C(0)
                    | UINT16_C(1) << pin::BLINK_1
                    | UINT16_C(1) << pin::BLINK_2
                    | UINT16_C(1) << pin::SEPARATOR_CLOSED
                    | UINT16_C(1) << pin::LOWER_DRAIN_CLOSED
                    | UINT16_C(1) << pin::UPPER_DRAIN_CLOSED;
            case State::PRE_BLENDING_GUARD:
                return UINT16_C(0)
                    | UINT16_C(1) << pin::BLINK_1
                    | UINT16_C(1) << pin::SEPARATOR_CLOSED
                    | UINT16_C(1) << pin::LOWER_DRAIN_CLOSED
                    | UINT16_C(1) << pin::UPPER_DRAIN_CLOSED;
            case State::BLENDING_WATER_PUMPING:
                return UINT16_C(0)
                    | UINT16_C(1) << pin::BLINK_1
                    | UINT16_C(1) << pin::LOWER_WATER_PUMP
                    | UINT16_C(1) << pin::SEPARATOR_CLOSED
                    | UINT16_C(1) << pin::LOWER_DRAIN_CLOSED
                    | UINT16_C(1) << pin::UPPER_DRAIN_CLOSED;
            case State::BLENDING:
                return UINT16_C(0)
                    | UINT16_C(1) << pin::BLINK_1
                    | UINT16_C(1) << pin::SEPARATOR_CLOSED
                    | UINT16_C(1) << pin::LOWER_DRAIN_CLOSED
                    | UINT16_C(1) << pin::UPPER_DRAIN_CLOSED
                    | UINT16_C(1) << pin::BLENDER;
            case State::POST_BLENDING_CHECKING:
                return UINT16_C(0)
                    | UINT16_C(1) << pin::BLINK_1
                    | UINT16_C(1) << pin::SEPARATOR_CLOSED
                    | UINT16_C(1) << pin::UPPER_DRAIN_CLOSED;
            case State::BLENDING_WATER_DRAINING:
                return UINT16_C(0)
                    | UINT16_C(1) << pin::BLINK_1
                    | UINT16_C(1) << pin::SEPARATOR_CLOSED
                    | UINT16_C(1) << pin::UPPER_DRAIN_CLOSED;
            default:
                return UINT16_C(0);
        }
    }
}

using state::State;

// The current state.
State __state;

// The last time a state change occurred.
unsigned long __last_ms;

// The accumulated time of the current state.
unsigned long __delta_ms;

// Cancelling property will persist in succeeding states until a restart.
bool __cancelling;

/**
 * Digitally write the same value for a set of pins for a state (see
 * {@link state::pins}).
 *
 * @param state the state
 * @param value the value to set
 * @see state::pins
 */
void __batch_digital_write(state::State state, uint8_t value) {
    uint8_t pin = 0;
    uint16_t pins = state::pins(state);

    // Consider only d0 to d13 pins
    while (pins != 0 && pin <= 13) {
        if ((pins & 1) != 0) {
            digitalWrite(pin, value);
        }
        pin++;
        pins >>= 1;
    }
}

void setup() {
    pinMode(pin::START, INPUT_PULLUP);
    pinMode(pin::CANCEL_OR_CONTINUE, INPUT_PULLUP);

    pinMode(pin::SEPARATOR_CLOSED, OUTPUT);
    pinMode(pin::LOWER_WATER_PUMP, OUTPUT);
    pinMode(pin::UPPER_WATER_PUMP, OUTPUT);
    pinMode(pin::LOWER_DRAIN_CLOSED, OUTPUT);
    pinMode(pin::UPPER_DRAIN_CLOSED, OUTPUT);
    pinMode(pin::HEATER, OUTPUT);
    pinMode(pin::MIXER, OUTPUT);
    pinMode(pin::BLENDER, OUTPUT);
    pinMode(pin::BLINK_1, OUTPUT);
    pinMode(pin::BLINK_2, OUTPUT);

    __state = State::READY;
    __last_ms = millis();
    __delta_ms = 0uL;
    __cancelling = false;

    __batch_digital_write(__state, true);
}

void loop() {
    const unsigned long curr_ms = millis();
    const unsigned long last_ms = __last_ms;
    __last_ms = curr_ms;

    const unsigned long delta_ms = __delta_ms += curr_ms - last_ms;

    const bool start = !digitalRead(pin::START);
    const bool cancel_or_continue = !digitalRead(pin::CANCEL_OR_CONTINUE);

    State state = __state;
    switch (__state) {
        case State::READY:
            if (start) {
                state = State::PRE_MIXING_CHECKING;
                __cancelling = false;
            }
            break;
        case State::PRE_MIXING_CHECKING:
            if (cancel_or_continue) {
                state = State::PRE_MIXING_GUARD;
            } else {
                digitalWrite(pin::BLINK_2, delta_ms / (duration::BLINKING / 2) % 2);
                __delta_ms -= delta_ms >= duration::BLINKING * duration::BLINKING;
            }
            break;
        case State::PRE_MIXING_GUARD:
            if (!cancel_or_continue) {
                state = State::MIXING_WATER_PUMPING;
            }
            break;
        case State::MIXING_WATER_PUMPING:
            if (__cancelling = __cancelling || cancel_or_continue) {
                state = State::PRE_RINSING_CHECKING;
            } else if (delta_ms >= duration::WATER_PUMPING) {
                state = State::HEATED_MIXING;
            }
            break;
        case State::HEATED_MIXING:
            if (__cancelling = __cancelling || cancel_or_continue) {
                state = State::PRE_RINSING_CHECKING;
            } else if (delta_ms >= duration::HEATED_MIXING) {
                state = State::MIXING;
            }
            break;
        case State::MIXING:
            if ((__cancelling = __cancelling || cancel_or_continue)
                || delta_ms >= duration::MIXING
            ) {
                state = State::PRE_RINSING_CHECKING;
            }
            break;
        case State::PRE_RINSING_CHECKING:
            if (cancel_or_continue) {
                state = State::PRE_RINSING_GUARD;
            } else {
                digitalWrite(pin::BLINK_2, delta_ms / (duration::BLINKING / 2) % 2);
                __delta_ms -= delta_ms >= duration::BLINKING * duration::BLINKING;
            }
            break;
        case State::PRE_RINSING_GUARD:
            if (!cancel_or_continue) {
                state = State::MIXING_WATER_DRAINING;
            }
            break;
        case State::MIXING_WATER_DRAINING:
            if (delta_ms >= duration::WATER_DRAINING) {
                if (__cancelling = __cancelling || cancel_or_continue) {
                    state = State::READY;
                } else {
                    state = State::RINSING;
                }
            }
            break;
        case State::RINSING:
            if ((__cancelling = __cancelling || cancel_or_continue)
                || delta_ms >= duration::RINSING) {
                state = State::RINSE_WATER_DRAINING;
            }
            break;
        case State::RINSE_WATER_DRAINING:
            if (delta_ms >= duration::WATER_DRAINING) {
                if (__cancelling = __cancelling || cancel_or_continue) {
                    state = State::READY;
                } else {
                    state = State::POST_RINSING_CHECKING;
                }
            }
            break;
        case State::POST_RINSING_CHECKING:
            if (cancel_or_continue) {
                state = State::POST_RINSING_GUARD;
            } else {
                digitalWrite(pin::BLINK_2, delta_ms / (duration::BLINKING / 2) % 2);
                __delta_ms -= delta_ms >= duration::BLINKING * duration::BLINKING;
            }
            break;
        case State::POST_RINSING_GUARD:
            if (!cancel_or_continue) {
                state = State::RINSE_BLEND_PAUSE;
            }
            break;
        case State::RINSE_BLEND_PAUSE:
            if (delta_ms >= duration::RINSE_BLEND_PAUSE) {
                state = State::PRE_BLENDING_CHECKING;
            }
            break;
        case State::PRE_BLENDING_CHECKING:
            if (cancel_or_continue) {
                state = State::PRE_BLENDING_GUARD;
            } else {
                digitalWrite(pin::BLINK_2, delta_ms / (duration::BLINKING / 2) % 2);
                __delta_ms -= delta_ms >= duration::BLINKING * duration::BLINKING;
            }
            break;
        case State::PRE_BLENDING_GUARD:
            if (!cancel_or_continue) {
                state = State::BLENDING_WATER_PUMPING;
            }
            break;
        case State::BLENDING_WATER_PUMPING:
            if (delta_ms >= duration::WATER_PUMPING) {
                state = State::BLENDING;
            }
            break;
        case State::BLENDING:
            if (delta_ms >= duration::RINSING) {
                state = State::POST_BLENDING_CHECKING;
            }
            break;
        case State::POST_BLENDING_CHECKING:
            if (cancel_or_continue) {
                state = State::BLENDING_WATER_DRAINING;
            } else {
                digitalWrite(pin::BLINK_2, delta_ms / (duration::BLINKING / 2) % 2);
                __delta_ms -= delta_ms >= duration::BLINKING * duration::BLINKING;
            }
            break;
        case State::BLENDING_WATER_DRAINING:
            if (delta_ms >= duration::WATER_DRAINING) {
                state = State::READY;
            }
            break;
    }

    if (state != __state) {
        __batch_digital_write(__state, false);
        digitalWrite(pin::BLINK_2, __cancelling && state != State::READY);
        __batch_digital_write(state, true);

        __state = state;
        __delta_ms = 0uL;

    }
}
