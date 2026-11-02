use crate::variants::{
    OptionToPossible,
    Possible
};

impl<T, E> OptionToPossible<T, E> for Option<T> {
    fn okay_or(self, err: E) -> Possible<T, E> {
        match self {
            Some(val) => Possible::Okay(val),
            None => Possible::Error(err)
        }
    }
    fn okay_or_else<F>(self, err: F) -> Possible<T, E>
    where
        F: FnOnce() -> E,
    {
        match self {
            Some(val) => Possible::Okay(val),
            None => Possible::Error(err()),
        }
    }
}
