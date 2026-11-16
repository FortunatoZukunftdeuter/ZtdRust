use std::{
    ffi::OsStr, 
    process::Command
};

pub trait ZtdCommand
{
     /// Creates a new [`Command`] with inherited standard streams.
    ///
    /// The command inherits the standard input, standard output, and
    /// standard error of the parent process.
    ///
    /// # Arguments
    ///
    /// - `name`: The name or path of the executable to execute.
    ///
    /// # Returns
    ///
    /// A [`Command`] configured to inherit all standard streams.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut command = Command::new_inherited("cargo");
    /// command.arg("run");
    /// ```
    fn new_inherited(name: impl AsRef<OsStr>) -> Command;
}
