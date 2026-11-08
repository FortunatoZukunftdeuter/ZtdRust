use std::str::pattern::{
    Pattern,
    ReverseSearcher
};
use crate::str::ZtdSplitAmong;

impl<'a, P1, P2, P3> Iterator for ZtdSplitAmong<'a, P1, P2, P3>
where
    P1: Pattern + Clone,
    for<'b> P1::Searcher<'b>: ReverseSearcher<'b>,
    P2: Pattern,
    P3: Pattern + Clone
{
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        return self.split.next_among();
    }
}
