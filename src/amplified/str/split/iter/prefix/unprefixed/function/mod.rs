use std::str::pattern::{
    Pattern,
    ReverseSearcher
};
use crate::str::ZtdSplitUnprefixed;

pub fn split_unprefixed<'a, P1, P2>(
    string: &'a str,
    prefix: P1,
    spliter: P2,
) -> ZtdSplitUnprefixed<'a, P1, P2>
where
    P1: Pattern + Clone,
    for<'b> P1::Searcher<'b>: ReverseSearcher<'b>,
    P2: Pattern
{
    return ZtdSplitUnprefixed::new(
        string, 
        prefix, 
        spliter
    );
}
