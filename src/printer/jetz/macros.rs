#[macro_export]
macro_rules! printj {
    ($($arg:tt)*) => {{
        use std::io::Write;

        let stdout = std::io::stdout();
        let mut stdout = stdout.lock();

        let result = write!(stdout, "{}", format_args!($($arg)*))
            .and_then(|_| stdout.flush());
    }};
}

#[macro_export]
macro_rules! printsj {
    ($($arg:tt)*) => {{
        use std::io::Write;

        let stdout = std::io::stdout();
        let mut stdout = stdout.lock();

        let result = write!(stdout, "{} ", format_args!($($arg)*))
            .and_then(|_| stdout.flush());
    }};
}

#[macro_export]
macro_rules! printdj {
    ($($arg:tt)*) => {{
        use std::io::Write;

        let stdout = std::io::stdout();
        let mut stdout = stdout.lock();

        let result = write!(stdout, "{}.", format_args!($($arg)*))
            .and_then(|_| stdout.flush());
    }};
}

#[macro_export]
macro_rules! printdsj {
    ($($arg:tt)*) => {{
        use std::io::Write;

        let stdout = std::io::stdout();
        let mut stdout = stdout.lock();

        let result = write!(stdout, "{}. ", format_args!($($arg)*))
            .and_then(|_| stdout.flush());
    }};
}

#[macro_export]
macro_rules! printlnj {
    ($($arg:tt)*) => {{
        use std::io::Write;

        let stdout = std::io::stdout();
        let mut stdout = stdout.lock();

        let result = writeln!(stdout, "{}", format_args!($($arg)*))
            .and_then(|_| stdout.flush());
    }};
}

#[macro_export]
macro_rules! printdlnj {
    ($($arg:tt)*) => {{
        use std::io::Write;

        let stdout = std::io::stdout();
        let mut stdout = stdout.lock();

        let result = writeln!(stdout, "{}.", format_args!($($arg)*))
            .and_then(|_| stdout.flush());
    }};
}
