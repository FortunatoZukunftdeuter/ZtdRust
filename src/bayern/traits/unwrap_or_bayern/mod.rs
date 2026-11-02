use crate::{
    bayern::types::{
        Bayern,
        BayernOr
    }
};

pub trait UnwrapOrBayern<'a, T> {
    fn unwrap_or_bayern(self) -> BayernOr<'a, T>;

    fn unwrap_or_bayern_from(self, bayern: &'a mut Bayern) -> BayernOr<'a, T>;
}
