 /// Terminates the process after printing a formatted message to standard error.
///
/// The message is formatted using [`format!`] and printed with the suffix
/// specified by the macro. The process exits with the provided exit code,
/// or `0` if no exit code is provided.
///
/// # Arguments
///
/// * `$code` - The process exit code. Optional; defaults to `0`.
/// * `$msg` - A format string and its arguments, following the syntax of
///   [`format!`].
///
/// # Output
///
/// The formatted message is printed to standard error (`stderr`).
///
/// # Process Termination
///
/// This macro does not return. It terminates the process after printing
/// the message.
///
/// # Panics
///
/// Formatting can panic if the format string and its arguments are invalid.
/// Otherwise, process termination is delegated to [`bayern::bye_msg`].
///
/// # Examples
///
/// ```
///
/// // Print a message and exit with code 1.
/// bye_msg!(1, "An error occurred: {}", "file not found");
///
/// // Print a message and exit with code 0.
/// bye_msg!("Operation completed successfully");
/// ```
#[macro_export]
macro_rules! bye_msg {
    ($code:expr, $($msg:tt)*) => {
        $crate::bayern::bye_msg($code, &format!($($msg)*), "");
    };
    ($($msg:tt)*) => {
        $crate::bayern::bye_msg(0, &format!($($msg)*), "");
    }
}

/// Terminates the process after printing a formatted message followed by a period.
///
/// This macro behaves like [`bye_msg!`], but appends `.` to the message.
///
/// # Arguments
///
/// * `$code` - The process exit code. Optional; defaults to `0`.
/// * `$msg` - A format string and its arguments, following the syntax of
///   [`format!`].
///
/// # Output
///
/// Prints the formatted message followed by a period (`.`) to standard error.
///
/// # Examples
///
/// ```
///
/// bye_msgd!(1, "An error occurred: {}", "file not found");
/// bye_msgd!("Operation completed successfully");
/// ```
#[macro_export]
macro_rules! bye_msgd {
    ($code:expr, $($msg:tt)*) => {
        $crate::bayern::bye_msg($code, &format!($($msg)*), ".");
    };
    ($($msg:tt)*) => {
        $crate::bayern::bye_msg(0, &format!($($msg)*), ".");
    }
}

/// Terminates the process after printing a formatted message followed by a newline.
///
/// This macro behaves like [`bye_msg!`], but appends a newline (`\n`) to the
/// message.
///
/// # Arguments
///
/// * `$code` - The process exit code. Optional; defaults to `0`.
/// * `$msg` - A format string and its arguments, following the syntax of
///   [`format!`].
///
/// # Output
///
/// Prints the formatted message followed by a newline to standard error.
///
/// # Examples
///
/// ```
///
/// bye_msgln!(1, "An error occurred: {}", "file not found");
/// bye_msgln!("Operation completed successfully");
/// ```
#[macro_export]
macro_rules! bye_msgln {
    ($code:expr, $($msg:tt)*) => {
        $crate::bayern::bye_msg($code, &format!($($msg)*), "\n");
    };
    ($($msg:tt)*) => {
        $crate::bayern::bye_msg(0, &format!($($msg)*), "\n");
    }
}

/// Terminates the process after printing a formatted message followed by a period
/// and a newline.
///
/// This macro behaves like [`bye_msg!`], but appends `.\n` to the message.
///
/// # Arguments
///
/// * `$code` - The process exit code. Optional; defaults to `0`.
/// * `$msg` - A format string and its arguments, following the syntax of
///   [`format!`].
///
/// # Output
///
/// Prints the formatted message followed by a period and a newline to standard
/// error.
///
/// # Examples
///
/// ```
///
/// bye_msgdln!(1, "An error occurred: {}", "file not found");
/// bye_msgdln!("Operation completed successfully");
/// ```
#[macro_export]
macro_rules! bye_msgdln {
    ($code:expr, $($msg:tt)*) => {
        $crate::bayern::bye_msg($code, &format!($($msg)*), ".\n");
    };
    ($($msg:tt)*) => {
        $crate::bayern::bye_msg(0, &format!($($msg)*), ".\n");
    }
}
