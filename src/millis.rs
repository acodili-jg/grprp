//! A basic implementation of the `millis()` function from Arduino:
//!
//! <https://www.arduino.cc/reference/en/language/functions/time/millis/>
//!
//! Uses timer TC0 and one of its interrupts to update a global millisecond
//! counter. A walkthough of this code is available here:
//!
//! <https://blog.rahix.de/005-avr-hal-millis/>
//!
//! This implementation is a modification of:
//!
//! <https://github.com/Rahix/avr-hal/blob/96c9979/examples/arduino-uno/src/bin/uno-millis.rs>
//!
//! Interrupts *must* be enabled.

use arduino_hal::simple_pwm::Prescaler;
use core::cell;
use panic_halt as _;

/////////////////////
// Public Functions
/////////////////////

/// Returns the number of milliseconds passed since the Arduino board began
/// running the current program. This number will overflow (go back to zero),
/// after approximately 50 days.
#[must_use]
pub fn millis() -> Millis {
    avr_device::interrupt::free(|cs| MILLIS_COUNTER.borrow(cs).get()).into()
}

/// Initialization for [`millis`] to behavior correctly.
pub fn init(tc0: &arduino_hal::pac::TC0) {
    // Configure the timer for the above interval (in CTC mode)
    // and enable its interrupt.
    tc0.tccr0a.write(|w| w.wgm0().ctc());
    tc0.ocr0a.write(|w| w.bits(TIMER_COUNTS));
    tc0.tccr0b.write(|w| match PRESCALER {
        Prescaler::Direct => w.cs0().direct(),
        Prescaler::Prescale8 => w.cs0().prescale_8(),
        Prescaler::Prescale64 => w.cs0().prescale_64(),
        Prescaler::Prescale256 => w.cs0().prescale_256(),
        Prescaler::Prescale1024 => w.cs0().prescale_1024(),
    });
    tc0.timsk0.write(|w| w.ocie0a().set_bit());

    // Reset the global millisecond counter
    avr_device::interrupt::free(|cs| {
        MILLIS_COUNTER.borrow(cs).set(0);
    });
}

/////////////////
// Custom Types
/////////////////

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, ufmt::derive::uDebug)]
pub struct Millis(pub u32);

///////////////
// Interrupts
///////////////

static MILLIS_COUNTER: avr_device::interrupt::Mutex<cell::Cell<u32>> =
    avr_device::interrupt::Mutex::new(cell::Cell::new(0));

#[avr_device::interrupt(atmega328p)]
fn TIMER0_COMPA() {
    avr_device::interrupt::free(|cs| {
        let counter_cell = MILLIS_COUNTER.borrow(cs);
        let counter = counter_cell.get();
        counter_cell.set(counter + MILLIS_INCREMENT);
    });
}

// Possible Values:
//
// ╔═══════════╦══════════════╦═══════════════════╗
// ║ PRESCALER ║ TIMER_COUNTS ║ Overflow Interval ║
// ╠═══════════╬══════════════╬═══════════════════╣
// ║        64 ║          250 ║              1 ms ║
// ║       256 ║          125 ║              2 ms ║
// ║       256 ║          250 ║              4 ms ║
// ║      1024 ║          125 ║              8 ms ║
// ║      1024 ║          250 ║             16 ms ║
// ╚═══════════╩══════════════╩═══════════════════╝
const PRESCALER: Prescaler = Prescaler::Prescale1024;
const TIMER_COUNTS: u8 = 125;

const MILLIS_INCREMENT: u32 = divisions(PRESCALER) * TIMER_COUNTS as u32 / 16000;

#[inline]
const fn divisions(prescaler: Prescaler) -> u32 {
    match prescaler {
        Prescaler::Direct => 0,
        Prescaler::Prescale8 => 8,
        Prescaler::Prescale64 => 64,
        Prescaler::Prescale256 => 256,
        Prescaler::Prescale1024 => 1024,
    }
}

////////////////////////////////
// Custom Type Implementations
////////////////////////////////

impl From<Millis> for u32 {
    #[inline]
    fn from(Millis(millis): Millis) -> Self {
        millis
    }
}

impl From<u32> for Millis {
    #[inline]
    fn from(millis: u32) -> Self {
        Self(millis)
    }
}

macro_rules! millis_biops {
    ($vis:vis const $name:ident -> $ret_t:ty { bind $tmp:ident; $expr:expr }) => {
        #[inline]
        #[must_use] $vis const fn $name(self, rhs: Self) -> $ret_t {
            let $tmp = self.0.$name(rhs.0);
            $expr
        }
    };

    ($vis:vis $name:ident -> $ret_t:ty { bind $tmp:ident; $expr:expr }) => {
        #[inline]
        $vis fn $name(self, rhs: Self) -> $ret_t {
            let $tmp = self.0.$name(rhs.0);
            $expr
        }
    };
}

macro_rules! millis_biops_group {
    ($vis:vis const $($name:ident),+ $(,)? -> $ret_t:ty { bind $tmp:ident; $expr:expr }) => {
        $(millis_biops!($vis const $name -> $ret_t { bind $tmp; $expr });)+
    };

    ($vis:vis $($name:ident),+ $(,)? -> $ret_t:ty { bind $tmp:ident; $expr:expr }) => {
        $(millis_biops!($vis $name -> $ret_t { bind $tmp; $expr });)+
    };
}

impl Millis {
    pub const ONE_SECOND: Self = Self(1000);

    millis_biops_group!(
        pub const
            wrapping_add,
            wrapping_div,
            wrapping_div_euclid,
            wrapping_mul,
            wrapping_pow,
            wrapping_rem,
            wrapping_rem_euclid,
            wrapping_shl,
            wrapping_shr,
            wrapping_sub,
        -> Self {
            bind tmp;
            Self(tmp)
        }
    );
}
