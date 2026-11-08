use std::str::pattern::{
    Pattern,
    ReverseSearcher
};
use crate::str::ZtdSplitUnAmong;

pub fn split_unamong<'a, P1, P2, P3>(
    string: &'a str,
    prefix: P1,
    spliter: P2,
    subfix: P3
) -> ZtdSplitUnAmong<'a, P1, P2, P3>
where
    P1: Pattern + Clone,
    for<'b> P1::Searcher<'b>: ReverseSearcher<'b>,
    P2: Pattern,    
    P3: Pattern + Clone
{
    return ZtdSplitUnAmong::new(
        string, 
        prefix, 
        spliter,
        subfix
    );
}
