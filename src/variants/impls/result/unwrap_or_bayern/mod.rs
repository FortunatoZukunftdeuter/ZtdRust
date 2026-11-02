use crate::bayern::{
    Bayern,
    BayernOr,
    BayernRef,
    UnwrapOrBayern
};

impl<'a, T, E> UnwrapOrBayern<'a, T> for Result<T, E> {
    fn unwrap_or_bayern(self) -> BayernOr<'a, T> {
        BayernOr::new(
            self.ok(),
            BayernRef::Owned(Bayern::default())
        )
    }

    fn unwrap_or_bayern_from(self, bayern: &'a mut Bayern) -> BayernOr<'a, T> {
        BayernOr::new(
            self.ok(),
            BayernRef::Borrowed(bayern)
        )
    }
}
