use crate::{
    bayern::types::{
        Bayern,
        BayernOr
    }
};

/// Provides methods to unwrap a value or return a [`BayernOr`].
///
/// This trait allows values to be converted into [`BayernOr`],
/// optionally using an existing mutable [`Bayern`] instance.
///
/// # Methods
///
/// - [`unwrap_or_bayern`](Self::unwrap_or_bayern): Unwraps the value into a [`BayernOr`].
/// - [`unwrap_or_bayern_from`](Self::unwrap_or_bayern_from): Unwraps the value using an existing [`Bayern`] instance.
pub trait UnwrapOrBayern<'a, T> {
    /// Converts the value into a [`BayernOr`].
    ///
    /// # Returns
    ///
    /// A [`BayernOr`] containing the resulting value or Bayern state.
    fn unwrap_or_bayern(self) -> BayernOr<'a, T>;

    /// Converts the value into a [`BayernOr`] using an existing [`Bayern`] instance.
    ///
    /// # Arguments
    ///
    /// - `bayern`: A mutable reference to the [`Bayern`] instance to use.
    ///
    /// # Returns
    ///
    /// A [`BayernOr`] associated with the provided [`Bayern`] instance.
    fn unwrap_or_bayern_from(self, bayern: &'a mut Bayern) -> BayernOr<'a, T>;
}

