use std::{
    fs,
    io::Error,
    path::Path
};

pub fn create_parent_all(path: impl AsRef<Path>) -> Result<(), Error> 
{
    let path_ref = path.as_ref();

    if let Some(parent_path) = path_ref.parent() {
        if !parent_path.exists() {
            fs::create_dir_all(&parent_path)?;
        }
    }

return Ok(());}
