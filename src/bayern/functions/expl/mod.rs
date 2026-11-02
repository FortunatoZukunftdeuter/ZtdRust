use std::process::exit;

mod macros;

/// Executes a function, prints an exit message, and terminates the process.
///
/// The supplied function is executed before the message is printed.
/// The process then terminates using the specified exit code.
///
/// # Type Parameters
///
/// * `Fnc` - A function or closure implementing [`FnOnce()`].
/// * `AsStr` - A type that implements [`AsRef<str>`].
///
/// # Arguments
///
/// * `code` - The exit code returned by the process.
/// * `fnc` - The function to execute before termination.
/// * `msg` - The message to print to standard error.
/// * `ender` - A string appended to the message before printing.
///
/// # Process Termination
///
/// This function never returns.
///
/// # Panics
///
/// This function does not explicitly catch panics originating from `fnc`.
///
/// If `fnc` panics, the message is not printed and the process is not
/// terminated through the specified exit code.
///
/// # Examples
///
/// ```
/// use your_crate::bye_expl;
///
/// bye_expl(
///     1,
///     || {
///         println!("Performing cleanup...");
///     },
///     "An error occurred",
///     "\n"
/// );
/// ```
#[inline]
pub fn bye_expl<Fnc, AsStr>(
    code: i32,
    fnc: Fnc,
    msg: AsStr,
    ender: &str
) -> !
where
    Fnc: FnOnce(),
    AsStr: AsRef<str>
{
    let _ = fnc();

    let msg_ref = msg.as_ref();
    eprint!("{msg_ref}{ender}");

    exit(code);
}

