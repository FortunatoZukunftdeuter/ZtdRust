/// Executes a function, prints a formatted message to standard error,
/// and terminates the process.
///
/// The message is printed without an additional suffix.
///
/// # Arguments
///
/// * `$code` - The process exit code.
/// * `$fnc_to_exec` - The function or closure executed before termination.
/// * `$msg` - The message and optional formatting arguments.
///
/// # Exit Code
///
/// If an exit code is provided, it is used for process termination.
/// Otherwise, the default exit code is `0`.
///
/// # Examples
///
/// ```
/// bye_expl!(1, || cleanup(), "An error occurred: {}", error);
/// bye_expl!(|| cleanup(), "An error occurred");
/// ```
#[macro_export]
macro_rules! bye_expl {
    ($code:expr, $fnc_to_exec:expr, $($msg:tt)*) => {
        $crate::bayern::bye_expl(
            $code,
            $fnc_to_exec,
            &format!($($msg)*),
            ""
        );
    };

    ($fnc_to_exec:expr, $($msg:tt)*) => {
        $crate::bayern::bye_expl(
            0,
            $fnc_to_exec,
            &format!($($msg)*),
            ""
        );
    };
}

/// Executes a function, prints a formatted message followed by a period
/// to standard error, and terminates the process.
///
/// # Arguments
///
/// * `$code` - The process exit code.
/// * `$fnc_to_exec` - The function or closure executed before termination.
/// * `$msg` - The message and optional formatting arguments.
///
/// # Exit Code
///
/// If an exit code is provided, it is used for process termination.
/// Otherwise, the default exit code is `0`.
///
/// # Examples
///
/// ```
/// bye_expld!(1, || cleanup(), "An error occurred");
/// bye_expld!(|| cleanup(), "An error occurred");
/// ```
#[macro_export]
macro_rules! bye_expld {
    ($code:expr, $fnc_to_exec:expr, $($msg:tt)*) => {
        $crate::bayern::bye_expl(
            $code,
            $fnc_to_exec,
            &format!($($msg)*),
            "."
        );
    };

    ($fnc_to_exec:expr, $($msg:tt)*) => {
        $crate::bayern::bye_expl(
            0,
            $fnc_to_exec,
            &format!($($msg)*),
            "."
        );
    };
}

/// Executes a function, prints a formatted message followed by a newline
/// to standard error, and terminates the process.
///
/// # Arguments
///
/// * `$code` - The process exit code.
/// * `$fnc_to_exec` - The function or closure executed before termination.
/// * `$msg` - The message and optional formatting arguments.
///
/// # Exit Code
///
/// If an exit code is provided, it is used for process termination.
/// Otherwise, the default exit code is `0`.
///
/// # Examples
///
/// ```
/// bye_explln!(1, || cleanup(), "An error occurred");
/// bye_explln!(|| cleanup(), "An error occurred");
/// ```
#[macro_export]
macro_rules! bye_explln {
    ($code:expr, $fnc_to_exec:expr, $($msg:tt)*) => {
        $crate::bayern::bye_expl(
            $code,
            $fnc_to_exec,
            &format!($($msg)*),
            "\n"
        );
    };

    ($fnc_to_exec:expr, $($msg:tt)*) => {
        $crate::bayern::bye_expl(
            0,
            $fnc_to_exec,
            &format!($($msg)*),
            "\n"
        );
    };
}

/// Executes a function, prints a formatted message followed by a period
/// and a newline to standard error, and terminates the process.
///
/// # Arguments
///
/// * `$code` - The process exit code.
/// * `$fnc_to_exec` - The function or closure executed before termination.
/// * `$msg` - The message and optional formatting arguments.
///
/// # Exit Code
///
/// If an exit code is provided, it is used for process termination.
/// Otherwise, the default exit code is `0`.
///
/// # Examples
///
/// ```
/// bye_expldln!(1, || cleanup(), "An error occurred");
/// bye_expldln!(|| cleanup(), "An error occurred");
/// ```
#[macro_export]
macro_rules! bye_expldln {
    ($code:expr, $fnc_to_exec:expr, $($msg:tt)*) => {
        $crate::bayern::bye_expl(
            $code,
            $fnc_to_exec,
            &format!($($msg)*),
            ".\n"
        );
    };

    ($fnc_to_exec:expr, $($msg:tt)*) => {
        $crate::bayern::bye_expl(
            0,
            $fnc_to_exec,
            &format!($($msg)*),
            ".\n"
        );
    };
}
