use crate::variants::Possible;

pub trait ResultToPossible<T, E> {
    fn okay(self) -> Possible<T, E>;   
}
