use std::str::pattern::Pattern;
use crate::str::ZtdSplitSubfixed;

pub fn split_subfixed<'a, P2, P3>(
    string: &'a str,
    spliter: P2,
    subfix: P3,
) -> ZtdSplitSubfixed<'a, P2, P3>
where
    P2: Pattern,
    P3: Pattern + Clone,
{
    return ZtdSplitSubfixed::new(
        string,
        spliter,
        subfix
    );
}
