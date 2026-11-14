#[cfg(feature = "nightly")]
mod split;
#[cfg(feature = "nightly")]
pub use split::*;

mod impls; pub use impls::ZtdIsCase;

#[cfg(feature = "nightly")]
pub use impls::{
    ZtdSplitWithFixIter,
    ZtdSpliterWithFixOnce
};
