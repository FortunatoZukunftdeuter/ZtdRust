use std::{
    path::Path,
    io::Error,
    fs::File
};
use crate::vfs;

pub fn create_all(path: impl AsRef<Path>, force: bool) -> Result<File, Error> 
{
    let path_ref = path.as_ref();

    vfs::create_parent_all(path_ref)?;

    if force {
        return File::create_new(path_ref);
    } else {
        return File::create(path_ref);
    }
}
