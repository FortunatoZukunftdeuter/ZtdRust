use crate::variants::{
    ResultToPossible,
    Possible
};

impl<T, E> ResultToPossible<T, E> for Result<T, E> {
    fn okay(self) -> Possible<T, E> {
        match self {
            Ok(val) => Possible::Okay(val),
            Err(e) => Possible::Error(e)
        }
    }
}
