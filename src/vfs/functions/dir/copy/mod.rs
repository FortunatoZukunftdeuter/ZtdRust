use std::{
    path::Path,
    io::Error
};
use crate::vfs;

pub fn copy(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<(), Error>
{
    let from_ref = from.as_ref();
    let to_ref = to.as_ref();

    vfs::dir::create_all(to_ref)?;

    let entries = from_ref.read_dir()?;

    for entry in entries {
        let entry = entry?;

        let destination = to_ref.join(entry.file_name());

        vfs::whatever::copy_all(&entry.path(), &destination)?;
    }
return Ok(());}
