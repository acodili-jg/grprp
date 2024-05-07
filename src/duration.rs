macro_rules! constants {
    ($($name:ident = $expr:expr;)*) => {
        $(pub const $name: u32 = $expr;)*
    };
}

constants! {
    DEFAULT = 1000;
    LOCKING = DEFAULT;
    SOAK_WATER_PUMPING = DEFAULT;
    SEPARATOR_TRANSITION = DEFAULT;
}
