use std::str::pattern::{
    Pattern, 
    ReverseSearcher
};
use crate::str::ZtdSplitWithFix;

mod iterator;

pub struct ZtdSplitPrefixed<'a, P1, P2: Pattern> {
    split: ZtdSplitWithFix<'a, P1, P2, &'static str>,
}

impl<'a, P1, P2> ZtdSplitUnprefixed<'a, P1, P2>
where
    P1: Pattern + Clone,
    for<'b> P1::Searcher<'b>: ReverseSearcher<'b>,
    P2: Pattern,
{
    pub fn new(slice: &'a str, prefix: P1, spliter: P2) -> Self {
        Self {
            split: ZtdSplitWithFix::new(slice, prefix, spliter, "", false),
        }
    }
}
