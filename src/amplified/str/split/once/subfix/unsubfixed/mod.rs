use std::str::pattern::Pattern;
use crate::amplified::str::split_with_subfix_once;

pub fn split_unsubfixed_once<'a, P2, P3>(
    string: &'a str,
    spliter: P2,
    subfix: P3
) -> Option<(&'a str, &'a str)>
where
    P2: Pattern,
    P3: Pattern + Clone
{
    return split_with_subfix_once(string, spliter, subfix, false);
}
