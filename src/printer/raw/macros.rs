#[macro_export]
macro_rules! printf {
    ($($arg:tt)*) => {{
        use std::io::Write;

        let stdout = std::io::stdout();
        let mut stdout = stdout.lock();

        let result = write!(stdout, "{}", format_args!($($arg)*))
            .and_then(|_| stdout.flush());

        result
    }};
}

#[macro_export]
macro_rules! printsf {
    ($($arg:tt)*) => {{
        use std::io::Write;

        let stdout = std::io::stdout();
        let mut stdout = stdout.lock();

        let result = write!(stdout, "{} ", format_args!($($arg)*))
            .and_then(|_| stdout.flush());

        result
    }};
}

#[macro_export]
macro_rules! printdf {
    ($($arg:tt)*) => {{
        use std::io::Write;

        let stdout = std::io::stdout();
        let mut stdout = stdout.lock();

        let result = write!(stdout, "{}.", format_args!($($arg)*))
            .and_then(|_| stdout.flush());

        result
    }};
}

#[macro_export]
macro_rules! printdsf {
    ($($arg:tt)*) => {{
        use std::io::Write;

        let stdout = std::io::stdout();
        let mut stdout = stdout.lock();

        let result = write!(stdout, "{}. ", format_args!($($arg)*))
            .and_then(|_| stdout.flush());

        result
    }};
}

#[macro_export]
macro_rules! printlnf {
    ($($arg:tt)*) => {{
        use std::io::Write;

        let stdout = std::io::stdout();
        let mut stdout = stdout.lock();

        let result = writeln!(stdout, "{}", format_args!($($arg)*))
            .and_then(|_| stdout.flush());

        result
    }};
}

#[macro_export]
macro_rules! printdlnf {
    ($($arg:tt)*) => {{
        use std::io::Write;

        let stdout = std::io::stdout();
        let mut stdout = stdout.lock();

        let result = writeln!(stdout, "{}.", format_args!($($arg)*))
            .and_then(|_| stdout.flush());

        result
    }};
}
