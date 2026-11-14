use std::str::pattern::{
    Pattern,
    ReverseSearcher,
    Searcher
};
use crate::str::ZtdSplitWithFix;

impl<'a, P1, P2, P3> ZtdSplitWithFix<'a, P1, P2, P3> 
where 
    P1: Pattern + Clone,
    for<'b> P1::Searcher<'b>: ReverseSearcher<'b>,
    P2: Pattern,
{
    fn next_with_prefix(&mut self, must_be_prefixed: bool) -> Option<&'a str> 
    {
        if self.has_finished 
        {
            return None;
        }

        let slice: &'a str = self.slice;
        let start = self.start;
        let end = self.end;

        while let Some((matched_start, matched_end)) = self.searcher.next_match() 
        {
            let prefix = self.prefix.clone();

            let before = &slice[start..matched_start];

            let has_prefix = before.ends_with(prefix);

            if has_prefix != must_be_prefixed {
                continue;
            }

            self.start = matched_end;

        return Some(before) ; } 
    
        self.has_finished = true;
        let remaining = &slice[start..end];

    return Some(remaining) ; }

    pub fn next_unprefixed(&mut self) -> Option<&'a str> 
    {
        return self.next_with_prefix(false);    
    }

    pub fn next_prefixed(&mut self) -> Option<&'a str>
    {
        return self.next_with_prefix(true);
    }
}
