use crate::bayern::{
    Bayern,
    BayernOr,
    BayernRef,
    UnwrapOrBayern
};

impl<'a, T> UnwrapOrBayern<'a, T> for Option<T> {
   fn unwrap_or_bayern(self) -> BayernOr<'a, T> {
        BayernOr::new(
            self,
            BayernRef::Owned(Bayern::default())
        )
    }

   fn unwrap_or_bayern_from(self, bayern: &'a mut Bayern) -> BayernOr<'a, T> {
       BayernOr::new(
            self,
            BayernRef::Borrowed(bayern)
        )
   }
}
