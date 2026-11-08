use std::str::pattern::Pattern;
use crate::str::ZtdSplitUnSubfixed;

pub fn split_unsubfixed<'a, P2, P3>(
    string: &'a str,
    spliter: P2,
    subfix: P3,
) -> ZtdSplitUnSubfixed<'a, P2, P3>
where
    P2: Pattern,
    P3: Pattern + Clone,
{
    return ZtdSplitUnSubfixed::new(
        string,
        spliter,
        subfix
    );
}
