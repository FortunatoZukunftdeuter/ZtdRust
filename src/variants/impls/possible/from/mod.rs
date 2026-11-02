use crate::variants::Possible;

impl<T, E> From<Result<T, E>> for Possible<T, E> {
    fn from(result: Result<T, E>) -> Self {
        match result {
            Ok(value) => Possible::Okay(value),
            Err(error) => Possible::Error(error),
        }
    }
}

impl<T, E> From<Option<T>> for Possible<T, E> {
    fn from(option: Option<T>) -> Self {
        match option {
            Some(value) => Possible::Okay(value),
            None => Possible::Null,
        }
    }
}
