use std::str::pattern::{
    Pattern,
    ReverseSearcher
};
use crate::str::{
    split_prefixed_once, 
    split_unprefixed_once,
    split_subfixed_once,
    split_unsubfixed_once,
    split_among_once,
    split_unamong_once
};

pub trait ZtdSpliterWithFixOnce {
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
    fn split_subfixed_once<P2, P3>(&self, spliter: P2, subfix: P3) -> Option<(&str, &str)>
    where
        P2: Pattern,
        P3: Pattern + Clone;
    fn split_unsubfixed_once<P2, P3>(&self, spliter: P2, subfix: P3) -> Option<(&str, &str)>
    where
        P2: Pattern,
        P3: Pattern + Clone;
    fn split_among_once<P1, P2, P3>(&self, prefix: P1, spliter: P2, subfix: P3) -> Option<(&str, &str)>
    where
        P1: Pattern + Clone,
        for<'b> P1::Searcher<'b>: ReverseSearcher<'b>,
        P2: Pattern,
        P3: Pattern + Clone;
    fn split_unamong_once<P1, P2, P3>(&self, prefix: P1, spliter: P2, subfix: P3) -> Option<(&str, &str)>
    where
        P1: Pattern + Clone,
        for<'b> P1::Searcher<'b>: ReverseSearcher<'b>,
        P2: Pattern,
        P3: Pattern + Clone;  
    
}

impl ZtdSpliterWithFixOnce for str {
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

    fn split_subfixed_once<P2, P3>(&self, spliter: P2, subfix: P3) -> Option<(&str, &str)>
    where
        P2: Pattern,
        P3: Pattern + Clone
    {
        return split_subfixed_once(self, spliter, subfix);
    }
    fn split_unsubfixed_once<P2, P3>(&self, spliter: P2, subfix: P3) -> Option<(&str, &str)>
    where
        P2: Pattern,
        P3: Pattern + Clone
    {
        return split_unsubfixed_once(self, spliter, subfix);
    }

    fn split_among_once<P1, P2, P3>(&self, prefix: P1, spliter: P2, subfix: P3) -> Option<(&str, &str)>
    where
        P1: Pattern + Clone,
        for<'b> P1::Searcher<'b>: ReverseSearcher<'b>,
        P2: Pattern,
        P3: Pattern + Clone
    {
        return split_among_once(self, prefix, spliter, subfix);
    }
    fn split_unamong_once<P1, P2, P3>(&self, prefix: P1, spliter: P2, subfix: P3) -> Option<(&str, &str)>
    where
        P1: Pattern + Clone,
        for<'b> P1::Searcher<'b>: ReverseSearcher<'b>,
        P2: Pattern,
        P3: Pattern + Clone
    {
        return split_unamong_once(self, prefix, spliter, subfix);
    }
}
