//! Logging macroses

#[macro_export]

macro_rules! log_info {
    ($($arg:tt)*) => {{
        use anscape::seq::{
            base::RESET,
            colors::{BLUE, GREEN},
            styles::BOLD,
        };
        println!("{}{}[Chobs]{} {}{}{}{}",GREEN, BOLD,RESET, BLUE, BOLD, format_args!($($arg)*), RESET);
    }};
}
