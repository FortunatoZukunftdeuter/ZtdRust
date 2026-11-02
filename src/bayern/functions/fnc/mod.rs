use std::process::exit;

mod macros;

#[inline]
pub fn bye_fnc<F>(code: i32, fnc_to_exec: F) -> ! 
where 
    F: FnOnce()
{
    let _ = fnc_to_exec();
    exit(code);
}
