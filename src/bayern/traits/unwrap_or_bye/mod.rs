use crate::bayern::Bayern;

/// Provides an `unwrap_or_bye` method for [`Option`] values.
///
/// Returns the contained value if the option is [`Some`].
/// If the option is [`None`], executes the supplied closure, passing
/// a mutable reference to a [`Bayern`] instance.
///
/// # Type Parameters
///
/// * `T` - The type of the value returned by the method.
///
/// # Arguments
///
/// * `fnc_null` - A closure executed when the option is [`None`].
///   It receives a mutable reference to [`Bayern`] and must return `T`.
///
/// # Examples
///
/// ```
///
/// let value = Some(42);
/// let result = value.unwrap_or_bye(|bayern| {
///     bayern.msg("Value was missing");
///     0
/// });
///
/// assert_eq!(result, 42);
/// ```
pub trait UnwrapOrByeOption<T> {
    /// Returns the contained value or executes the fallback closure.
    ///
    /// The closure is only executed when the option is [`None`].
    fn unwrap_or_bye<FncNull>(self, fnc_null: FncNull) -> T
    where
        FncNull: FnOnce(&mut Bayern) -> T;
}

/// Provides an `unwrap_or_bye` method for [`Result`] values.
///
/// Returns the contained value if the result is [`Ok`].
/// If the result is [`Err`], executes the supplied closure, passing
/// a mutable reference to a [`Bayern`] instance and ownership of the error.
///
/// # Type Parameters
///
/// * `T` - The type of the value returned by the method.
/// * `E` - The error type contained in [`Result::Err`].
///
/// # Arguments
///
/// * `fnc_err` - A closure executed when the result is [`Err`].
///   It receives a mutable reference to [`Bayern`] and the error value,
///   and must return `T`.
///
/// # Examples
///
/// ```
///
/// let value: Result<i32, &str> = Err("something went wrong");
///
/// let result = value.unwrap_or_bye(|bayern, error| {
///     bayern.msg(error);
///     0
/// });
///
/// assert_eq!(result, 0);
/// ```
pub trait UnwrapOrByeResult<T, E> {
    /// Returns the contained value or executes the error-handling closure.
    ///
    /// The closure is only executed when the result is [`Err`].
    fn unwrap_or_bye<FncErr>(self, fnc_err: FncErr) -> T
    where
        FncErr: FnOnce(&mut Bayern, E) -> T;
}

/// Provides an `unwrap_or_bye` method for values that can represent
/// both an error and a missing value.
///
/// This trait combines the behavior of [`UnwrapOrByeOption`] and
/// [`UnwrapOrByeResult`].
///
/// Its implementation determines how the input is classified as a
/// successful value, an error, or a missing value.
///
/// # Type Parameters
///
/// * `T` - The type of the value returned by the method.
/// * `E` - The error type passed to the error-handling closure.
///
/// # Arguments
///
/// * `fnc_err` - A closure executed when an error occurs. It receives
///   a mutable reference to [`Bayern`] and the error value.
/// * `fnc_null` - A closure executed when the value is missing. It receives
///   a mutable reference to [`Bayern`].
///
/// Both closures must return `T`.
///
/// # Examples
///
/// ```
///
/// // Example usage depends on the type implementing this trait.
/// ```
pub trait UnwrapOrByePossible<T, E> {
    /// Returns the contained value or executes the appropriate fallback closure.
    ///
    /// Calls `fnc_err` when an error occurs, or `fnc_null` when the value
    /// is missing.
    fn unwrap_or_bye<FncErr, FncNull>(self, fnc_err: FncErr, fnc_null: FncNull) -> T
    where
        FncErr: FnOnce(&mut Bayern, E) -> T,
        FncNull: FnOnce(&mut Bayern) -> T;
}
