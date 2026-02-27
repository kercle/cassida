#[macro_export]
#[cfg(feature = "debug_matcher")]
macro_rules! dbg_matcher {
    ($($arg:tt)*) => {
        println!($($arg)*);
    };
}

#[macro_export]
#[cfg(not(feature = "debug_matcher"))]
macro_rules! dbg_matcher {
    ($($arg:tt)*) => {};
}
