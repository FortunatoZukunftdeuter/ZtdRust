use std::str::pattern::{
    Pattern, 
    ReverseSearcher
};
use crate::str::{
    ZtdSplitPrefixed,
    ZtdSplitUnprefixed,
    ZtdSplitSubfixed,
    ZtdSplitUnSubfixed,
    ZtdSplitAmong,
    ZtdSplitUnAmong,

    split_prefixed,
    split_unprefixed,
    split_subfixed,
    split_unsubfixed,
    split_among,
    split_unamong,

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

    fn split_subfixed<P2, P3>(&self, spliter: P2, subfix: P3) -> ZtdSplitSubfixed<'_, P2, P3>
    where
        P2: Pattern,
        P3: Pattern + Clone
    {
        return split_subfixed(self, spliter, subfix);
    }
    fn split_unsubfixed<P2, P3>(&self, spliter: P2, subfix: P3) -> ZtdSplitUnSubfixed<'_, P2, P3>
    where
        P2: Pattern,
        P3: Pattern + Clone
    {
        return split_unsubfixed(self, spliter, subfix);
    }

    fn split_among<P1, P2, P3>(&self, prefix: P1, spliter: P2, subfix: P3) -> ZtdSplitAmong<'_, P1, P2, P3>
    where
        P1: Pattern + Clone,
        for<'b> P1::Searcher<'b>: ReverseSearcher<'b>,
        P2: Pattern,
        P3: Pattern + Clone
    {
        return split_among(self, prefix, spliter, subfix);
    }
    fn split_unamong<P1, P2, P3>(&self, prefix: P1, spliter: P2, subfix: P3) -> ZtdSplitUnAmong<'_, P1, P2, P3>
    where
        P1: Pattern + Clone,
        for<'b> P1::Searcher<'b>: ReverseSearcher<'b>,
        P2: Pattern,
        P3: Pattern + Clone
    {
        return split_unamong(self, prefix, spliter, subfix);
    }
}
