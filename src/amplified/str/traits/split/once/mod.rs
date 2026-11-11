use std::str::pattern::{
    Pattern,
    ReverseSearcher
};

pub trait ZtdSpliterOnce {
    fn split_unprefixed_once<P1, P2>(&self, prefix: P1, spliter: P2) -> Option<(&str, &str)>
    where
        P1: Pattern + Clone,
        for<'b> P1::Searcher<'b>: ReverseSearcher<'b>,
        P2: Pattern;
    fn split_prefixed_once<P1, P2>(&self, prefix: P1, spliter: P2) -> Option<(&str, &str)>
    where
        P1: Pattern + Clone,
        for<'b> P1::Searcher<'b>: ReverseSearcher<'b>,
        P2: Pattern;   
}
