use crate::bayern::{
    Bayern,
    UnwrapOrByeResult
};



impl<T, E> UnwrapOrByeResult<T, E> for Result<T, E> {
    fn unwrap_or_bye<FncErr>(self, fnc_err: FncErr) -> T
    where 
        FncErr: FnOnce(&mut Bayern, E) -> T
    {
        let mut bayern = Bayern::default();

        match self {
            Ok(val) => val,
            Err(e) => fnc_err(&mut bayern, e)
        }
    }
}
