use std::str::pattern::{
    Pattern,
    ReverseSearcher
};
use crate::str::ZtdSplitAmong;

pub fn split_among<'a, P1, P2, P3>(
    string: &'a str,
    prefix: P1,
    spliter: P2,
    subfix: P3
) -> ZtdSplitAmong<'a, P1, P2, P3>
where
    P1: Pattern + Clone,
    for<'b> P1::Searcher<'b>: ReverseSearcher<'b>,
    P2: Pattern,    
    P3: Pattern + Clone
{
    return ZtdSplitAmong::new(
        string, 
        prefix, 
        spliter,
        subfix
    );
}
