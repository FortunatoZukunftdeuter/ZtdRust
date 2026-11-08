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
    P3: Pattern + Clone
{
    fn next_with_among(&mut self, must_be_among: bool) -> Option<&'a str> 
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
            let subfix = self.subfix.clone();

            let before = &slice[..matched_start];
            let after = &slice[matched_end..];

            let is_among = before.ends_with(prefix) && after.starts_with(subfix);

            if is_among != must_be_among {
                continue;
            }

            self.start = matched_end;

        return Some(before) ; } 
    
        self.has_finished = true;
        let remaining = &slice[start..end];

    return Some(remaining) ; }

    pub fn next_unamong(&mut self) -> Option<&'a str> 
    {
        return self.next_with_among(false);    
    }

    pub fn next_among(&mut self) -> Option<&'a str>
    {
        return self.next_with_among(true);
    }
}
