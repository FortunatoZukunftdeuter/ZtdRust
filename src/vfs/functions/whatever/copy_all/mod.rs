use std::{
    io::Error, 
    path::Path,
};
use crate::vfs;

pub fn copy_all(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<u64, Error>
{
    let from_ref = from.as_ref();
    let to_ref = to.as_ref();

    vfs::create_parent_all(to_ref)?;

return vfs::whatever::copy(from_ref, to_ref) ; }
