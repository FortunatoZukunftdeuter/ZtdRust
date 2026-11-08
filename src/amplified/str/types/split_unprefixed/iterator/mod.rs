use std::str::pattern::{
    Pattern,
    ReverseSearcher
};
use crate::str::ZtdSplitUnprefixed;

impl<'a, P1, P2> Iterator for ZtdSplitUnprefixed<'a, P1, P2>
where
    P1: Pattern + Clone,
    for<'b> P1::Searcher<'b>: ReverseSearcher<'b>,
    P2: Pattern
{
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        return self.split.next_unprefixed();
    }
}
