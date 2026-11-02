use crate::{
    variants::Possible,
    bayern::{
        Bayern,
        BayernOr,
        BayernRef,
        UnwrapOrBayern
    }
};

impl<'a, T, E> UnwrapOrBayern<'a, T> for Possible<T, E> {
    fn unwrap_or_bayern(self) -> BayernOr<'a, T> {
        BayernOr::new(
            self.okay(),    
            BayernRef::Owned(Bayern::default())            
        )
    }

    fn unwrap_or_bayern_from(self, bayern: &'a mut Bayern) -> BayernOr<'a, T> {
        BayernOr::new(
            self.okay(),
            BayernRef::Borrowed(bayern)
        )
    }
}
