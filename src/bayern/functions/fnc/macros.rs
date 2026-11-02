/// Executes a function and terminates the process.
///
/// The supplied function is executed before the process terminates.
/// The process then exits using the specified exit code.
///
/// # Arguments
///
/// * `$code` - The process exit code.
/// * `$fnc_to_exec` - The function or closure to execute before termination.
///
/// # Exit Code
///
/// If an exit code is provided, it is used for process termination.
/// Otherwise, the default exit code is `0`.
///
/// # Process Termination
///
/// This macro never returns.
///
/// # Panics
///
/// If the supplied function panics, process termination through the
/// specified exit code will not occur.
///
/// # Examples
///
/// ```
/// use your_crate::bye_fnc;
///
/// // Execute a function and exit with code 1.
/// bye_fnc!(1, || {
///     println!("Performing cleanup...");
/// });
///
/// // Execute a function and exit with code 0.
/// bye_fnc!(|| {
///     println!("Finishing execution...");
/// });
/// ```
#[macro_export]
macro_rules! bye_fnc {
    ($code:expr, $fnc_to_exec:expr) => {
        $crate::bayern::bye_fnc($code, $fnc_to_exec);
    };

    ($fnc_to_exec:expr) => {
        $crate::bayern::bye_fnc(0, $fnc_to_exec);
    };
}

