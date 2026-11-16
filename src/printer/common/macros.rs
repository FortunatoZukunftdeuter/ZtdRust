 /* Print Dotted */
#[macro_export]
macro_rules! printd {
    ($($arg:tt)*) => {
        print!("{}.", format_args!($($arg)*));
    };
}

/* Print Spaced */
#[macro_export]
macro_rules! prints {
    ($($arg:tt)*) => {
        print!("{} ", format_args!($($arg)*));
    };
}

/* Print Dotted Spaced */
#[macro_export]
macro_rules! printds {
    ($($arg:tt)*) => {
        print!("{}. ", format_args!($($arg)*));
    };
}

/* Print Dotted New Line */
#[macro_export]
macro_rules! printdln {
    ($($arg:tt)*) => {
        println!("{}.", format_args!($($arg)*));
    };
}
