use std::str::pattern::Pattern;
use crate::str::ZtdSplitWithFix;

mod iterator;

pub struct ZtdSplitUnSubfixed<'a, P2: Pattern, P3> {
    split: ZtdSplitWithFix<'a, &'static str, P2, P3>,
}

impl<'a, P2, P3> ZtdSplitUnSubfixed<'a, P2, P3>
where
    P2: Pattern,
    P3: Pattern + Clone
{
    pub fn new(slice: &'a str, spliter: P2, subfix: P3) -> Self {
        return Self {
            split: ZtdSplitWithFix::new(slice, "", spliter, subfix, false),
        };
    }
}
