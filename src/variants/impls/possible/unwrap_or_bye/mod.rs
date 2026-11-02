use crate::{
    variants::Possible::{
        self,
        *
    },
    bayern::{
        Bayern,
        UnwrapOrByePossible
    }
};

impl<T, E> UnwrapOrByePossible<T, E> for Possible<T, E> {
    fn unwrap_or_bye<FncErr, FncNull>(self, fnc_err: FncErr, fnc_null: FncNull) -> T
       where 
           FncErr: FnOnce(&mut Bayern, E) -> T,
           FncNull: FnOnce(&mut Bayern) -> T
       {
           let mut bayern = Bayern::default();

           match self {
               Okay(val) => val,
               Error(e) => fnc_err(&mut bayern, e),
               Null => fnc_null(&mut bayern)
           }
       }   
}
