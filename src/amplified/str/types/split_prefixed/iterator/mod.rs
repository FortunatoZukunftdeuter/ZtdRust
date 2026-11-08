use std::str::pattern::{
    Pattern,
    ReverseSearcher
};
use crate::str::ZtdSplitPrefixed;

impl<'a, P1, P2> Iterator for ZtdSplitPrefixed<'a, P1, P2>
where
    P1: Pattern + Clone,
    for<'b> P1::Searcher<'b>: ReverseSearcher<'b>,
    P2: Pattern
{
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        return self.split.next_prefixed();
    }
}
