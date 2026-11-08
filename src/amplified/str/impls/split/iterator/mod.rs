use std::str::pattern::{
    Pattern, 
    ReverseSearcher
};
use crate::str::{
    ZtdSplitUnprefixed,
    ZtdSplitPrefixed,
    split_unprefixed,
    split_prefixed,
    traits::ZtdSpliteIterator
};

impl ZtdSpliteIterator for str {
    fn split_unprefixed<P1, P2>(&self, prefix: P1, spliter: P2) -> ZtdSplitUnprefixed<'_, P1, P2>
    where
        P1: Pattern + Clone,
        for<'b> P1::Searcher<'b>: ReverseSearcher<'b>,
        P2: Pattern,
    {
        return split_unprefixed(self, prefix, spliter);
    }
    fn split_prefixed<P1, P2>(&self, prefix: P1, spliter: P2) -> ZtdSplitPrefixed<'_, P1, P2>
    where
        P1: Pattern + Clone,
        for<'b> P1::Searcher<'b>: ReverseSearcher<'b>,
        P2: Pattern
    {
        return split_prefixed(self, prefix, spliter);
    }
}
