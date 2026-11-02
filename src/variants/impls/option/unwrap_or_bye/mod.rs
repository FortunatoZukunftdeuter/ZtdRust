use crate::bayern::{
    Bayern,
    UnwrapOrByeOption
};

impl<T> UnwrapOrByeOption<T> for Option<T> {
    fn unwrap_or_bye<FncNull>(self, fnc_null: FncNull) -> T
    where 
        FncNull: FnOnce(&mut Bayern) -> T
    {
        let mut bayern = Bayern::default();

        match self {
            Some(val) => val,
            None => fnc_null(&mut bayern)
        }
    }
}
