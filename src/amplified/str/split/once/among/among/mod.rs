use std::str::pattern::{
    Pattern, 
    ReverseSearcher,
};
use crate::amplified::str::split_with_among_once;

pub fn split_among_once<'a, P1, P2, P3>(
    string: &'a str,
    prefix: P1,
    spliter: P2,
    subfix: P3,
) -> Option<(&'a str, &'a str)>
where
    P1: Pattern + Clone,
    for<'b> P1::Searcher<'b>: ReverseSearcher<'b>,
    P2: Pattern,
    P3: Pattern + Clone
{
    return split_with_among_once(string, prefix, spliter, subfix, true);
}
