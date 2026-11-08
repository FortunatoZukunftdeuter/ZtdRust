use std::str::pattern::{
    Pattern,
    Searcher
};
use crate::str::ZtdSplitWithFix;

impl<'a, P1, P2, P3> ZtdSplitWithFix<'a, P1, P2, P3> 
where 
    P2: Pattern,
    P3: Pattern + Clone
{
    fn next_with_subfix(&mut self, must_be_subfixed: bool) -> Option<&'a str> 
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
            let subfix = self.subfix.clone();

            let after = &slice[matched_end..];

            let has_subfix = after.starts_with(subfix);

            if has_subfix != must_be_subfixed {
                continue;
            }

            self.start = matched_end;

            let before = &slice[start..matched_start];
        return Some(before) ; } 
    
        self.has_finished = true;
        let remaining = &slice[start..end];

    return Some(remaining) ; }

    pub fn next_unsubfixed(&mut self) -> Option<&'a str> 
    {
        return self.next_with_subfix(false);    
    }

    pub fn next_subfixed(&mut self) -> Option<&'a str>
    {
        return self.next_with_subfix(true);
    }
}
