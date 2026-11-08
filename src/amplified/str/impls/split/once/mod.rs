use std::str::pattern::{
    Pattern,
    ReverseSearcher
};
use crate::{
    str::{
        traits::ZtdSpliterOnce,
        split_unprefixed_once
    }
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
}
