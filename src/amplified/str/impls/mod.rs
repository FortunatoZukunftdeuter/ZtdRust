#[cfg(feature = "nightly")]
mod split_with_fix; 
#[cfg(feature = "nightly")]
pub use split_with_fix::{
    ZtdSplitWithFixIter,
    ZtdSpliterWithFixOnce
};

mod is_case; pub use is_case::ZtdIsCase;
