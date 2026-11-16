pub mod among;
pub mod unamong;

use std::str::pattern::{
    Pattern, 
    ReverseSearcher, 
    Searcher
};

pub(crate) fn split_with_among_once<'a, P1, P2, P3>(
    string: &'a str,
    prefix: P1,
    spliter: P2,
    subfix: P3,
    must_be_among: bool
) -> Option<(&'a str, &'a str)>
where
    P1: Pattern + Clone,
    for<'b> P1::Searcher<'b>: ReverseSearcher<'b>,
    P2: Pattern,
    P3: Pattern + Clone
{
    let mut searcher = spliter.into_searcher(string);

    while let Some((start, end)) = searcher.next_match() 
    {
        let prefix = prefix.clone();
        let subfix = subfix.clone();

        let before = &string[..start];
        let after = &string[end..];

        let has_among = before.ends_with(prefix) && after.starts_with(subfix);

        if must_be_among != has_among { 
            continue; 
        }

    
        return Some((
            before,
            after
        ));
    }

return None ; }
