use crate::variants::Possible;

pub trait OptionToPossible<T, E> {
    fn okay_or(self, err: E) -> Possible<T, E>;
    
    fn okay_or_else<F>(self, err: F) -> Possible<T, E>
    where 
        F: FnOnce() -> E;
}
