/* Print Line New Jetz */
#[macro_export]
macro_rules! printlnj {
    ($($arg:tt)*) => {{
        use std::io::Write;

        let stdout = std::io::stdout();
        let mut stdout = stdout.lock();

        write!(stdout, $($arg)*)
            .and_then(|_| write!(stdout, "\n"))
            .and_then(|_| stdout.flush())
    }};
}

/* Print Dotted Line New Jetz */
#[macro_export]
macro_rules! printdlnj {
    ($($arg:tt)*) => {{
        use std::io::Write;

        let stdout = std::io::stdout();
        let mut stdout = stdout.lock();

        write!(stdout, $($arg)*)
            .and_then(|_| write!(stdout, ".\n"))
            .and_then(|_| stdout.flush())
    }};
}
