use std::str::pattern::{
    Pattern,
    ReverseSearcher
};
use crate::str::ZtdSplitPrefixed;

pub fn split_prefixed<'a, P1, P2>(
    string: &'a str,
    prefix: P1,
    spliter: P2,
) -> ZtdSplitPrefixed<'a, P1, P2>
where
    P1: Pattern + Clone,
    for<'b> P1::Searcher<'b>: ReverseSearcher<'b>,
    P2: Pattern
{
    return ZtdSplitPrefixed::new(
        string, 
        prefix, 
        spliter
    );
}
