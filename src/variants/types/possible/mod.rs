/// Represents a value that can be successful, erroneous, or absent.
///
/// `Possible` provides three possible states:
///
/// - [`Possible::Okay`] contains a successful value.
/// - [`Possible::Error`] contains an error.
/// - [`Possible::Null`] represents the absence of a value.
///
/// It combines the concepts of `Result<T, E>` and `Option<T>`
/// into a single type.
///
/// # Examples
///
/// ```
/// use bayern::Possible;
///
/// let value: Possible<i32, &str> = Possible::Okay(42);
/// let error: Possible<i32, &str> = Possible::Error("failed");
/// let absent: Possible<i32, &str> = Possible::Null;
///
/// assert!(value.is_okay());
/// assert!(error.is_error());
/// assert!(absent.is_null());
/// ```
#[must_use = "a Possible value may contain an error or an absent value that should be handled"]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Possible<T, E> {
    /// Contains a successful value.
    Okay(T),

    /// Contains an error.
    Error(E),

    /// Represents the absence of a value.
    Null,
}

impl<T, E> Possible<T, E> {
    /// Returns `Some(T)` if the value is [`Possible::Okay`].
    ///
    /// Returns `None` for [`Possible::Error`] and [`Possible::Null`].
    ///
    /// Consumes `self`.
    pub fn okay(self) -> Option<T> {
        match self {
            Self::Okay(value) => Some(value),
            _ => None,
        }
    }

    /// Returns `Some(E)` if the value is [`Possible::Error`].
    ///
    /// Returns `None` for [`Possible::Okay`] and [`Possible::Null`].
    ///
    /// Consumes `self`.
    pub fn error(self) -> Option<E> {
        match self {
            Self::Error(error) => Some(error),
            _ => None,
        }
    }

    /// Returns a reference to the contained value or error.
    ///
    /// The returned `Possible` preserves the original variant.
    pub const fn as_ref(&self) -> Possible<&T, &E> {
        match self {
            Self::Okay(value) => Possible::Okay(value),
            Self::Error(error) => Possible::Error(error),
            Self::Null => Possible::Null,
        }
    }

    /// Returns a mutable reference to the contained value or error.
    ///
    /// The returned `Possible` preserves the original variant.
    pub const fn as_mut(&mut self) -> Possible<&mut T, &mut E> {
        match self {
            Self::Okay(value) => Possible::Okay(value),
            Self::Error(error) => Possible::Error(error),
            Self::Null => Possible::Null,
        }
    }

    /// Returns `true` if the value is [`Possible::Okay`].
    pub const fn is_okay(&self) -> bool {
        matches!(self, Self::Okay(_))
    }

    /// Returns `true` if the value is [`Possible::Error`].
    pub const fn is_error(&self) -> bool {
        matches!(self, Self::Error(_))
    }

    /// Returns `true` if the value is [`Possible::Null`].
    pub const fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    /// Transforms the contained value using the provided function.
    ///
    /// Applies `f` only to [`Possible::Okay`].
    /// [`Possible::Error`] and [`Possible::Null`] are preserved.
    pub fn map<U, F>(self, f: F) -> Possible<U, E>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Self::Okay(value) => Possible::Okay(f(value)),
            Self::Error(error) => Possible::Error(error),
            Self::Null => Possible::Null,
        }
    }

    /// Transforms the contained error using the provided function.
    ///
    /// Applies `f` only to [`Possible::Error`].
    /// [`Possible::Okay`] and [`Possible::Null`] are preserved.
    pub fn map_error<F, U>(self, f: F) -> Possible<T, U>
    where
        F: FnOnce(E) -> U,
    {
        match self {
            Self::Okay(value) => Possible::Okay(value),
            Self::Error(error) => Possible::Error(f(error)),
            Self::Null => Possible::Null,
        }
    }

    /// Chains another `Possible`-returning operation.
    ///
    /// Applies `f` only to [`Possible::Okay`].
    /// [`Possible::Error`] and [`Possible::Null`] are preserved.
    pub fn and_then<U, F>(self, f: F) -> Possible<U, E>
    where
        F: FnOnce(T) -> Possible<U, E>,
    {
        match self {
            Self::Okay(value) => f(value),
            Self::Error(error) => Possible::Error(error),
            Self::Null => Possible::Null,
        }
    }

    /// Recovers from an error using the provided function.
    ///
    /// Applies `f` only to [`Possible::Error`].
    /// [`Possible::Okay`] and [`Possible::Null`] are preserved.
    pub fn or_else<F, U>(self, f: F) -> Possible<T, U>
    where
        F: FnOnce(E) -> Possible<T, U>,
    {
        match self {
            Self::Okay(value) => Possible::Okay(value),
            Self::Error(error) => f(error),
            Self::Null => Possible::Null,
        }
    }

    /// Returns the contained value or the provided fallback.
    ///
    /// The fallback is returned for both [`Possible::Error`] and
    /// [`Possible::Null`].
    pub fn unwrap_or(self, fallback: T) -> T {
        match self {
            Self::Okay(value) => value,
            _ => fallback,
        }
    }

    /// Returns the contained value or computes a fallback.
    ///
    /// The closure is called for both [`Possible::Error`] and
    /// [`Possible::Null`].
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self {
            Self::Okay(value) => value,
            _ => f(),
        }
    }

    /// Converts the value into an [`Option`].
    ///
    /// Returns `Some(T)` for [`Possible::Okay`] and `None` otherwise.
    pub fn into_option(self) -> Option<T> {
        self.okay()
    }
}

