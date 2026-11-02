 use std::{
    process::exit,
    mem::take
};

/// Represents an exit configuration containing an exit code,
/// an error message, and a collection of functions to execute
/// before terminating the process.
///
/// # Examples
///
/// ```
/// let mut bayern = Bayern::new();
///
/// bayern
///     .code(1)
///     .msgdln("An error occurred");
/// ```
pub struct Bayern {
    /// The exit code returned by the process.
    ///
    /// A value of `0` conventionally indicates successful execution,
    /// while non-zero values indicate an error or abnormal termination.
    pub code: i32,

    /// The message printed to standard error before exiting.
    pub msg: String,

    /// Functions executed sequentially before process termination.
    ///
    /// Each function implements [`FnOnce`], meaning it can only
    /// be consumed and executed once.
    pub fncs: Vec<Box<dyn FnOnce()>>
}

impl Bayern {

    /* Initializers */

        /// Creates a new [`Bayern`] instance.
        ///
        /// # Returns
        ///
        /// A new instance with:
        /// - Exit code set to `0`.
        /// - An empty message.
        /// - No registered functions.
        pub fn new() -> Self {
            Self {
                code: 0,
                msg: String::new(),
                fncs: Vec::new()
            }
        }

    /* Chainers */

        /// Sets the process exit code.
        ///
        /// # Arguments
        ///
        /// * `code` - The exit code to use.
        ///
        /// # Returns
        ///
        /// A mutable reference to `self`, allowing method chaining.
        pub fn code(&mut self, code: i32) -> &mut Self {
            self.code = code;
            return self;
        }

        /// Registers a function to execute before process termination.
        ///
        /// Registered functions are executed in insertion order.
        ///
        /// # Type Parameters
        ///
        /// * `F` - A function or closure implementing `FnOnce() + 'static`.
        ///
        /// # Arguments
        ///
        /// * `fnc` - The function to register.
        ///
        /// # Returns
        ///
        /// A mutable reference to `self`.
        pub fn fnc<F>(&mut self, fnc: F) -> &mut Self
        where
            F: FnOnce() + 'static
        {
            self.fncs.push(Box::new(fnc));
            return self;
        }

        /// Appends a message to the existing exit message.
        ///
        /// # Type Parameters
        ///
        /// * `AsStr` - A type that can be referenced as a string slice.
        ///
        /// # Arguments
        ///
        /// * `msg` - The message to append.
        ///
        /// # Returns
        ///
        /// A mutable reference to `self`.
        pub fn msg<AsStr>(&mut self, msg: AsStr) -> &mut Self
        where
            AsStr: AsRef<str>
        {
            let msg_ref = msg.as_ref();
            self.msg.push_str(msg_ref);
            return self;
        }

        /* Adders */

            /// Appends a message followed by a specified suffix.
            ///
            /// This is an internal helper used by the message convenience methods.
            ///
            /// # Arguments
            ///
            /// * `msg` - The message to append.
            /// * `tail` - The suffix to append after the message.
            fn msg_plus<AsStr>(&mut self, msg: AsStr, tail: &str) -> &mut Self
            where
                AsStr: AsRef<str>
            {
                self.msg(msg);
                self.msg(tail);
                self
            }

            /// Appends a message followed by a period.
            pub fn msgd<AsStr>(&mut self, msg: AsStr) -> &mut Self
            where
                AsStr: AsRef<str>
            {
                self.msg_plus(msg, ".");
                self
            }

            /// Appends a message followed by a newline.
            pub fn msgln<AsStr>(&mut self, msg: AsStr) -> &mut Self
            where
                AsStr: AsRef<str>
            {
                self.msg_plus(msg, "\n");
                self
            }

            /// Appends a message followed by a period and newline.
            pub fn msgdln<AsStr>(&mut self, msg: AsStr) -> &mut Self
            where
                AsStr: AsRef<str>
            {
                self.msg_plus(msg, ".\n");
                self
            }

    /* Erasers */

        /// Resets the exit code to `0`.
        pub fn erase_code(&mut self) -> &mut Self {
            self.code(0);
            self
        }

        /// Clears the entire exit message.
        pub fn erase_msg(&mut self) -> &mut Self {
            self.msg = String::new();
            self
        }

        /// Removes all registered functions.
        ///
        /// Removed functions are dropped without being executed.
        pub fn erase_fnc(&mut self) -> &mut Self {
            self.fncs.clear();
            self
        }

    /* Overwriters */

        /// Removes all registered functions and replaces them with a new one.
        ///
        /// # Arguments
        ///
        /// * `fnc` - The replacement function.
        pub fn over_fnc<F>(&mut self, fnc: F) -> &mut Self
        where
            F: FnOnce() + 'static
        {
            self.erase_fnc();
            self.fnc(fnc);
            self
        }

        /// Replaces the existing exit message with a new message.
        ///
        /// # Arguments
        ///
        /// * `msg` - The replacement message.
        pub fn over_msg<AsStr>(&mut self, msg: AsStr) -> &mut Self
        where
            AsStr: AsRef<str>
        {
            self.erase_msg();
            self.msg(msg);
            self
        }

        /* Adders */

            /// Replaces the existing message and appends a specified suffix.
            fn over_msg_plus<AsStr>(&mut self, msg: AsStr, tail: &str) -> &mut Self
            where
                AsStr: AsRef<str>
            {
                self.over_msg(msg);
                self.msg(tail);
                self
            }

            /// Replaces the existing message with one followed by a period.
            pub fn over_msgd<AsStr>(&mut self, msg: AsStr) -> &mut Self
            where
                AsStr: AsRef<str>
            {
                self.over_msg_plus(msg, ".");
                self
            }

            /// Replaces the existing message with one followed by a newline.
            pub fn over_msgln<AsStr>(&mut self, msg: AsStr) -> &mut Self
            where
                AsStr: AsRef<str>
            {
                self.over_msg_plus(msg, "\n");
                self
            }

            /// Replaces the existing message with one followed by a period and newline.
            pub fn over_msgdln<AsStr>(&mut self, msg: AsStr) -> &mut Self
            where
                AsStr: AsRef<str>
            {
                self.over_msg_plus(msg, ".\n");
                self
            }

    /* Exiters */

        /// Executes all registered functions, prints the exit message,
        /// and terminates the process.
        ///
        /// Functions are executed in insertion order.
        ///
        /// The exit message is printed to standard error.
        ///
        /// # Panics
        ///
        /// This method does not explicitly catch panics from registered functions.
        ///
        /// # Process Termination
        ///
        /// This method never returns.
        pub fn bye(&mut self) -> ! {

            let fncs = take(&mut self.fncs);
            for fnc in fncs {
                let _ = fnc();
            }

            let msg = &self.msg;
            eprint!("{msg}");

            let code = self.code;
            exit(code);
        }

        /// Sets the exit code, executes the registered functions,
        /// prints the exit message, and terminates the process.
        ///
        /// Equivalent to calling [`Bayern::code`] followed by [`Bayern::bye`].
        ///
        /// # Arguments
        ///
        /// * `code` - The exit code to use.
        ///
        /// # Process Termination
        ///
        /// This method never returns.
        pub fn exit(&mut self, code: i32) -> ! {
            self.code(code);
            self.bye();
        }
}


impl Default for Bayern {
    fn default() -> Self {
        Self::new()
    }
}
