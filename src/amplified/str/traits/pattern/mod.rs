use std::str::pattern::Pattern;

pub trait ZtdPattern: Pattern {
    fn is_empty(&self) -> bool 
    where Self: Clone
    {
        let ipsum = self.clone();
        return ipsum.is_prefix_of("");
    }
}
