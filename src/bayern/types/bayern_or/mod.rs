use std::{
    process::exit,
    mem::take
};

use crate::bayern::Bayern;

/// Represents either an owned or mutably borrowed [`Bayern`] instance.
///
/// This enum allows [`BayernOr`] to operate on a [`Bayern`] instance
/// regardless of whether it owns the instance or borrows it.
///
/// # Variants
///
/// - [`BayernRef::Owned`] stores and owns a [`Bayern`] instance.
/// - [`BayernRef::Borrowed`] stores a mutable reference to an existing
///   [`Bayern`] instance.
///
/// # Examples
///
/// ```
/// let mut bayern = Bayern::new();
///
/// let mut reference = BayernRef::Borrowed(&mut bayern);
/// reference.get_mut().code(1);
/// ```
pub enum BayernRef<'a> {
    /// An owned [`Bayern`] instance.
    Owned(Bayern),

    /// A mutable reference to an existing [`Bayern`] instance.
    Borrowed(&'a mut Bayern)
}

impl<'a> BayernRef<'a> {

    /// Returns a mutable reference to the underlying [`Bayern`] instance.
    ///
    /// This method provides mutable access regardless of whether the
    /// instance is owned or borrowed.
    ///
    /// # Returns
    ///
    /// A mutable reference to the underlying [`Bayern`] instance.
    pub fn get_mut(&mut self) -> &mut Bayern {
        match self {
            BayernRef::Owned(bayern) => bayern,
            BayernRef::Borrowed(bayern) => bayern,
        }
    }
}

/// Represents an optional value associated with a [`Bayern`] exit configuration.
///
/// [`BayernOr`] provides a way to handle an optional value while configuring
/// the behavior that should occur when that value is absent.
///
/// If the value exists, [`BayernOr::bye`] returns it immediately.
///
/// If the value is absent, the associated [`Bayern`] configuration is used
/// to execute registered functions, print an exit message, and terminate
/// the process.
///
/// # Configuration
///
/// The exit code and message can be configured locally through [`BayernOr`].
/// When not specified, the corresponding values from the underlying
/// [`Bayern`] instance are used.
///
/// Functions registered through [`BayernOr::fnc`] are stored in the
/// underlying [`Bayern`] instance.
///
/// # Type Parameters
///
/// * `'a` - The lifetime of a borrowed [`Bayern`] instance.
/// * `T` - The value type returned when the optional value is present.
///
/// # Examples
///
/// ```
/// let mut bayern = Bayern::new();
/// let mut result = BayernOr::new(Some(42), BayernRef::Borrowed(&mut bayern));
///
/// let value = result.bye();
/// assert_eq!(value, 42);
/// ```
pub struct BayernOr<'a, T> {

    /// The optional value returned when [`BayernOr::bye`] is called.
    ///
    /// If `Some`, the contained value is returned without executing
    /// the exit procedures.
    ///
    /// If `None`, the underlying [`Bayern`] exit procedures are executed.
    pub(self) option: Option<T>,

    /// The [`Bayern`] instance used when the optional value is absent.
    ///
    /// Can either own an instance or borrow an existing one.
    pub(self) bayern: BayernRef<'a>,

    /// An optional exit code that overrides the underlying
    /// [`Bayern::code`] configuration.
    ///
    /// If `None`, the exit code from the underlying [`Bayern`] instance
    /// is used.
    pub(self) code: Option<i32>,

    /// An optional exit message that overrides the underlying
    /// [`Bayern::msg`] configuration.
    ///
    /// If `None`, the message from the underlying [`Bayern`] instance
    /// is used.
    pub(self) msg: Option<String>
}

impl<'a, T> BayernOr<'a, T> {

    /// Creates a new [`BayernOr`] instance.
    ///
    /// # Arguments
    ///
    /// * `option` - The optional value to be returned if present.
    /// * `bayern` - The owned or borrowed [`Bayern`] instance used
    ///   for exit handling.
    ///
    /// # Returns
    ///
    /// A new [`BayernOr`] instance with no local exit code or message
    /// overrides configured.
    pub fn new(option: Option<T>, bayern: BayernRef<'a>) -> Self {
        Self {
            option,
            bayern: bayern,
            code: None,
            msg: None
        }
    }

    /* Chainers */

        /// Sets the local exit code.
        ///
        /// This value takes precedence over the exit code configured
        /// in the underlying [`Bayern`] instance.
        ///
        /// # Arguments
        ///
        /// * `code` - The exit code to use if the optional value is absent.
        ///
        /// # Returns
        ///
        /// A mutable reference to `self`, allowing method chaining.
        pub fn code(&mut self, code: i32) -> &mut Self {
            self.code = Some(code);
            self
        }

        /// Registers a function to execute if the optional value is absent.
        ///
        /// The function is registered in the underlying [`Bayern`] instance.
        ///
        /// Functions are executed in insertion order when the exit
        /// procedures are triggered.
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
            let bayern = self.bayern.get_mut();
            bayern.fnc(fnc);

            self
        }

        /// Appends a message to the local exit message.
        ///
        /// This message takes precedence over the message configured
        /// in the underlying [`Bayern`] instance.
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
            self.msg
                .get_or_insert(String::new())
                .push_str(msg_ref);

            self
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
            ///
            /// # Returns
            ///
            /// A mutable reference to `self`.
            fn msg_plus<AsStr>(&mut self, msg: AsStr, tail: &str) -> &mut Self
            where
                AsStr: AsRef<str>
            {
                self.msg(msg);
                self.msg(tail);
                self
            }

            /// Appends a message followed by a period.
            ///
            /// # Arguments
            ///
            /// * `msg` - The message to append.
            ///
            /// # Returns
            ///
            /// A mutable reference to `self`.
            pub fn msgd<AsStr>(&mut self, msg: AsStr) -> &mut Self
            where
                AsStr: AsRef<str>
            {
                self.msg_plus(msg, ".");
                self
            }

            /// Appends a message followed by a newline.
            ///
            /// # Arguments
            ///
            /// * `msg` - The message to append.
            ///
            /// # Returns
            ///
            /// A mutable reference to `self`.
            pub fn msgln<AsStr>(&mut self, msg: AsStr) -> &mut Self
            where
                AsStr: AsRef<str>
            {
                self.msg_plus(msg, "\n");
                self
            }

            /// Appends a message followed by a period and newline.
            ///
            /// # Arguments
            ///
            /// * `msg` - The message to append.
            ///
            /// # Returns
            ///
            /// A mutable reference to `self`.
            pub fn msgdln<AsStr>(&mut self, msg: AsStr) -> &mut Self
            where
                AsStr: AsRef<str>
            {
                self.msg_plus(msg, ".\n");
                self
            }

    /* Erasers */

        /// Removes the locally configured exit code.
        ///
        /// After this method is called, the exit code from the underlying
        /// [`Bayern`] instance will be used instead.
        ///
        /// # Returns
        ///
        /// A mutable reference to `self`.
        pub fn erase_code(&mut self) -> &mut Self {
            self.code = None;
            self
        }

        /// Removes the locally configured exit message.
        ///
        /// After this method is called, the message from the underlying
        /// [`Bayern`] instance will be used instead.
        ///
        /// # Returns
        ///
        /// A mutable reference to `self`.
        pub fn erase_msg(&mut self) -> &mut Self {
            self.msg = None;
            self
        }

        /// Removes all functions registered in the underlying [`Bayern`] instance.
        ///
        /// Removed functions are dropped without being executed.
        ///
        /// # Returns
        ///
        /// A mutable reference to `self`.
        pub fn erase_fnc(&mut self) -> &mut Self {
            let bayern = self.bayern.get_mut();
            bayern.erase_fnc();
            self
        }

    /* Overwriters */

        /// Removes all previously registered functions and registers a replacement.
        ///
        /// The functions are removed from the underlying [`Bayern`] instance
        /// without being executed.
        ///
        /// # Type Parameters
        ///
        /// * `F` - A function or closure implementing `FnOnce() + 'static`.
        ///
        /// # Arguments
        ///
        /// * `fnc` - The replacement function.
        ///
        /// # Returns
        ///
        /// A mutable reference to `self`.
        pub fn over_fnc<F>(&mut self, fnc: F) -> &mut Self
        where
            F: FnOnce() + 'static
        {
            self.erase_fnc();
            self.fnc(fnc);
            self
        }

        /// Replaces the local exit message with a new message.
        ///
        /// This does not modify the message stored in the underlying
        /// [`Bayern`] instance.
        ///
        /// # Arguments
        ///
        /// * `msg` - The replacement message.
        ///
        /// # Returns
        ///
        /// A mutable reference to `self`.
        pub fn over_msg<AsStr>(&mut self, msg: AsStr) -> &mut Self
        where
            AsStr: AsRef<str>
        {
            self.erase_msg();
            self.msg(msg);
            self
        }

        /* Adders */

            /// Replaces the local message and appends a specified suffix.
            ///
            /// This is an internal helper used by the message replacement methods.
            ///
            /// # Arguments
            ///
            /// * `msg` - The replacement message.
            /// * `tail` - The suffix to append after the message.
            ///
            /// # Returns
            ///
            /// A mutable reference to `self`.
            fn over_msg_plus<AsStr>(&mut self, msg: AsStr, tail: &str) -> &mut Self
            where
                AsStr: AsRef<str>
            {
                self.over_msg(msg);
                self.msg(tail);
                self
            }

            /// Replaces the local message with one followed by a period.
            ///
            /// # Arguments
            ///
            /// * `msg` - The replacement message.
            ///
            /// # Returns
            ///
            /// A mutable reference to `self`.
            pub fn over_msgd<AsStr>(&mut self, msg: AsStr) -> &mut Self
            where
                AsStr: AsRef<str>
            {
                self.over_msg_plus(msg, ".");
                self
            }

            /// Replaces the local message with one followed by a newline.
            ///
            /// # Arguments
            ///
            /// * `msg` - The replacement message.
            ///
            /// # Returns
            ///
            /// A mutable reference to `self`.
            pub fn over_msgln<AsStr>(&mut self, msg: AsStr) -> &mut Self
            where
                AsStr: AsRef<str>
            {
                self.over_msg_plus(msg, "\n");
                self
            }

            /// Replaces the local message with one followed by a period and newline.
            ///
            /// # Arguments
            ///
            /// * `msg` - The replacement message.
            ///
            /// # Returns
            ///
            /// A mutable reference to `self`.
            pub fn over_msgdln<AsStr>(&mut self, msg: AsStr) -> &mut Self
            where
                AsStr: AsRef<str>
            {
                self.over_msg_plus(msg, ".\n");
                self
            }

    /* Exiters */

        /// Returns the contained value or terminates the process.
        ///
        /// If the optional value is `Some`, its contents are returned
        /// immediately. No registered functions are executed, and no
        /// exit message is printed.
        ///
        /// If the optional value is `None`, this method:
        ///
        /// 1. Executes all functions registered in the underlying [`Bayern`]
        ///    instance, in insertion order.
        /// 2. Prints the selected exit message to standard error.
        /// 3. Terminates the process using the selected exit code.
        ///
        /// # Message Resolution
        ///
        /// The locally configured message takes precedence over the message
        /// configured in the underlying [`Bayern`] instance.
        ///
        /// # Exit Code Resolution
        ///
        /// The locally configured exit code takes precedence over the exit
        /// code configured in the underlying [`Bayern`] instance.
        ///
        /// # Returns
        ///
        /// The contained value of type `T`, if present.
        ///
        /// # Process Termination
        ///
        /// This method never returns when the optional value is `None`.
        ///
        /// # Panics
        ///
        /// This method does not explicitly catch panics originating from
        /// registered functions.
        pub fn bye(&mut self) -> T {

            if let Some(value) = self.option.take() {
                return value;
            }

            let bayern = self.bayern.get_mut();
            let fncs = take(&mut bayern.fncs);

            for fnc in fncs {
                let _ = fnc();
            }

            let msg = match &self.msg {
                Some(msg) => msg,
                None => &bayern.msg
            };

            eprint!("{msg}");

            let code = match self.code {
                Some(code) => code,
                None => bayern.code
            };

            exit(code);
        }

        /// Overrides the exit code and returns the contained value
        /// or terminates the process.
        ///
        /// This method sets a local exit code and then delegates
        /// to [`BayernOr::bye`].
        ///
        /// If the optional value is present, it is returned immediately,
        /// and the process is not terminated.
        ///
        /// If the optional value is absent, the registered functions
        /// are executed, the exit message is printed, and the process
        /// terminates with the specified exit code.
        ///
        /// # Arguments
        ///
        /// * `code` - The exit code to use if the optional value is absent.
        ///
        /// # Returns
        ///
        /// The contained value of type `T`, if present.
        ///
        /// # Process Termination
        ///
        /// This method never returns when the optional value is `None`.
        pub fn exit(&mut self, code: i32) -> T {
            self.code(code);
            let value = self.bye();
            value
        }
}
