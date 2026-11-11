use std::str::pattern::{
    Pattern,
    ReverseSearcher
};
use crate::str::{
    split_prefixed_once, 
    split_unprefixed_once, 
    traits::ZtdSpliterOnce
};

impl ZtdSpliterOnce for str {
    fn split_unprefixed_once<P1, P2>(&self, prefix: P1, spliter: P2) -> Option<(&str, &str)>
    where
        P1: Pattern + Clone,
        for<'b> P1::Searcher<'b>: ReverseSearcher<'b>,
        P2: Pattern,
    {
        return split_unprefixed_once(self, prefix, spliter);
    }
    fn split_prefixed_once<P1, P2>(&self, prefix: P1, spliter: P2) -> Option<(&str, &str)>
    where
        P1: Pattern + Clone,
        for<'b> P1::Searcher<'b>: ReverseSearcher<'b>,
        P2: Pattern
    {
        return split_prefixed_once(self, prefix, spliter);
    }
}
