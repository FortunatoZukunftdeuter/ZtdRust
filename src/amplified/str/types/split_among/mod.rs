use std::str::pattern::{
    Pattern,
    ReverseSearcher
};
use crate::str::ZtdSplitWithFix;

mod iterator;

pub struct ZtdSplitAmong<'a, P1, P2: Pattern, P3> {
    split: ZtdSplitWithFix<'a, P1, P2, P3>,
}

impl<'a, P1, P2, P3> ZtdSplitAmong<'a, P1, P2, P3>
where
    P1: Pattern + Clone,
    for<'b> P1::Searcher<'b>: ReverseSearcher<'b>,
    P2: Pattern,
    P3: Pattern + Clone
{
    pub fn new(slice: &'a str, prefix: P1, spliter: P2, subfix: P3) -> Self {
        return Self {
            split: ZtdSplitWithFix::new(
                slice, 
                prefix, 
                spliter, 
                subfix, 
        false
            )
        };
    }
}
