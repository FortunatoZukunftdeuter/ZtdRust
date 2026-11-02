use std::process::exit;

mod macros;

#[inline]
pub fn bye_msg<AsStr>(code: i32, msg: AsStr, ender: &str) -> ! 
where 
    AsStr: AsRef<str>
{
    let msg_ref = msg.as_ref();
    eprint!("{}{}", msg_ref, ender);
    exit(code);
}
