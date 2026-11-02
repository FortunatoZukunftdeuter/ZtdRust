/* Print Jetz */
#[macro_export]
macro_rules! printj {
    ($($arg:tt)*) => {{
        use std::io::Write;

        let stdout = std::io::stdout();
        let mut stdout = stdout.lock();

        write!(stdout, $($arg)*)
            .and_then(|_| write!(stdout, ". "))
            .and_then(|_| stdout.flush())
    }};
}

/* Print Spaced Jetz */
#[macro_export]
macro_rules! printsj {
    ($($arg:tt)*) => {{
        use std::io::Write;

        let stdout = std::io::stdout();
        let mut stdout = stdout.lock();

        write!(stdout, $($arg)*)
            .and_then(|_| write!(stdout, " "))
            .and_then(|_| stdout.flush())
    }};
}

/* Print Dotted Jetz */
#[macro_export]
macro_rules! printdj {
    ($($arg:tt)*) => {{
        use std::io::Write;

        let stdout = std::io::stdout();
        let mut stdout = stdout.lock();

        write!(stdout, $($arg)*)
            .and_then(|_| write!(stdout, "."))
            .and_then(|_| stdout.flush())
    }};
}

/* Print Spaced Dotted Jetz */
#[macro_export]
macro_rules! printdsj {
    ($($arg:tt)*) => {{
        use std::io::Write;

        let stdout = std::io::stdout();
        let mut stdout = stdout.lock();

        write!(stdout, $($arg)*)
            .and_then(|_| write!(stdout, ". "))
            .and_then(|_| stdout.flush())
    }};
}
