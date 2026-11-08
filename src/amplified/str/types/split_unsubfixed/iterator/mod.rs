use std::str::pattern::Pattern;
use crate::str::ZtdSplitUnSubfixed;

impl<'a, P2, P3> Iterator for ZtdSplitUnSubfixed<'a, P2, P3>
where
    P2: Pattern,
    P3: Pattern + Clone
{
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        return self.split.next_unsubfixed();
    }
}
