use std::str::pattern::{
    Pattern, 
    ReverseSearcher,
};
use crate::amplified::str::functions::split_with_prefix_once::split_with_prefix_once;

pub fn split_unprefixed_once<'a, P1, P2>(
    string: &'a str,
    prefix: P1,
    spliter: P2
) -> Option<(&'a str, &'a str)>
where
    P1: Pattern + Clone,
    for<'b> P1::Searcher<'b>: ReverseSearcher<'b>,
    P2: Pattern,
{
    return split_with_prefix_once(string, prefix, spliter, false);
}
