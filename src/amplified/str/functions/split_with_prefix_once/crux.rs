use std::str::pattern::{
    Pattern, 
    ReverseSearcher, 
    Searcher
};

pub(crate) fn split_with_prefix_once<'a, P1, P2>(
    string: &'a str,
    prefix: P1,
    spliter: P2,
    must_be_prefixed: bool
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

        let has_prefixed = before.ends_with(prefix);
        if must_be_prefixed != has_prefixed { 
            continue; 
        }

        let after = &string[end..];
    
        return Some((
            before,
            after
        ));
    }

return None ; }
