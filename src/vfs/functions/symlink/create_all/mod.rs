use std::{
    path::Path,
    io::Error
};
use crate::vfs;

pub fn create_all(link: impl AsRef<Path>, to: impl AsRef<Path>, is_dir: bool) -> Result<(), Error>
{
    let link_ref = link.as_ref();
    let to_ref = to.as_ref();
    
    vfs::create_parent_all(to_ref)?;

return vfs::symlink::create(link_ref, to_ref, is_dir) ; }
