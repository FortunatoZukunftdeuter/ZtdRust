pub mod subfixed;
pub mod unsubfixed;

use std::str::pattern::{
    Pattern, 
    Searcher
};

pub(crate) fn split_with_subfix_once<'a, P2, P3>(
    string: &'a str,
    spliter: P2,
    subfix: P3,
    must_be_subfixed: bool
) -> Option<(&'a str, &'a str)>
where
    P2: Pattern,
    P3: Pattern + Clone
{
    let mut searcher = spliter.into_searcher(string);

    while let Some((start, end)) = searcher.next_match() 
    {
        let subfix = subfix.clone();

        let after = &string[end..];

        let has_subfix = after.starts_with(subfix);

        if must_be_subfixed != has_subfix { 
            continue; 
        }

        let before = &string[..start];
    
        return Some((
            before,
            after
        ));
    }

return None ; }
