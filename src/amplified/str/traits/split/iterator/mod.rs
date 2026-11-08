use std::str::pattern::{
    Pattern, 
    ReverseSearcher
};
use crate::str::{
    ZtdSplitUnprefixed,
    ZtdSplitPrefixed,
    ZtdSplitAmong,
    ZtdSplitUnAmong,
    ZtdSplitSubfixed,
    ZtdSplitUnSubfixed
};

pub trait ZtdSpliteIterator {
    fn split_unprefixed<P1, P2>(&self, prefix: P1, spliter: P2) -> ZtdSplitUnprefixed<'_, P1, P2>
    where
        P1: Pattern + Clone,
        for<'b> P1::Searcher<'b>: ReverseSearcher<'b>,
        P2: Pattern;
    fn split_prefixed<P1, P2>(&self, prefix: P1, spliter: P2) -> ZtdSplitPrefixed<'_, P1, P2>
    where
        P1: Pattern + Clone,
        for<'b> P1::Searcher<'b>: ReverseSearcher<'b>,
        P2: Pattern;

    fn split_among<P1, P2, P3>(&self, prefix: P1, spliter: P2, subfix: P3) -> ZtdSplitAmong<'_, P1, P2, P3>
    where
        P1: Pattern + Clone,
        for<'b> P1::Searcher<'b>: ReverseSearcher<'b>,
        P2: Pattern,
        P3: Pattern + Clone;

    fn split_unamong<P1, P2, P3>(&self, prefix: P1, spliter: P2, subfix: P3) -> ZtdSplitUnAmong<'_, P1, P2, P3>
    where
        P1: Pattern + Clone,
        for<'b> P1::Searcher<'b>: ReverseSearcher<'b>,
        P2: Pattern,
        P3: Pattern + Clone;

    fn split_subfixed<P2, P3>(&self, spliter: P2, subfix: P3) -> ZtdSplitSubfixed<'_, P2, P3>
    where
        P2: Pattern,
        P3: Pattern + Clone;
    fn split_unsubfixed<P2, P3>(&self, spliter: P2, subfix: P3) -> ZtdSplitUnSubfixed<'_, P2, P3>
    where
        P2: Pattern,
        P3: Pattern + Clone;


}
