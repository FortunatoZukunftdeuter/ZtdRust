use std::str::pattern::{
    Pattern, 
    ReverseSearcher, 
    Searcher
};

pub fn split_unprefixed_once<'a, P1, P2>(
    string: &'a str,
    prefix: P1,
    spliter: P2,
) -> Option<(&'a str, &'a str)>
where
    P1: Pattern + Clone,
    for<'b> P1::Searcher<'b>: ReverseSearcher<'b>,
    P2: Pattern,
{
    let mut searcher = spliter.into_searcher(string);

    while let Some((start, end)) = searcher.next_match() 
    {
        let prefix = prefix.clone();

        let before = &string[..start];

        if before.ends_with(prefix) { 
            continue; 
        }

        let after = &string[end..];
    
        return Some((
            before,
            after
        ));
    }

return None ; }
