use std::str::pattern::Pattern;
use crate::str::traits::ZtdPattern;

impl<P: Pattern> ZtdPattern for P {}

