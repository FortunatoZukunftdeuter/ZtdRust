use std::str::pattern::Pattern;

mod prefix;
mod subfix;
mod among;

pub(crate) struct ZtdSplitWithFix<'a, P1, P2: Pattern, P3> {

    slice: &'a str,

    start: usize,
    end: usize,

    prefix: P1,
    searcher: P2::Searcher<'a>,
    subfix: P3,

    has_finished: bool,
    include: bool
}

impl<'a, P1, P2, P3> ZtdSplitWithFix<'a, P1, P2, P3> 
where
    P2: Pattern
{
    pub fn new(slice: &'a str, prefix: P1, spliter: P2, subfix: P3, include: bool) -> Self {
        return ZtdSplitWithFix {
            slice,
            start: 0,
            end: slice.len(),
            prefix,
            searcher: spliter.into_searcher(slice),
            subfix,
            has_finished: false,
            include,
        };
    }
}
