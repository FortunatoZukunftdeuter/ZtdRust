use std::{
    io::Error, 
    path::Path,
};
use crate::vfs;

pub fn copy(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<(), Error>
{
    let from_ref = from.as_ref();
    let to_ref = to.as_ref();

    let target = vfs::symlink::read(from_ref)?;

    #[cfg(windows)] 
    {
        use std::os::windows::fs::MetadataExt;
        
        let metadata = vfs::symlink::metadata(from_ref)?;
        let attributes = metadata.file_attributes();
        const FILE_ATTRIBUTE_DIRECTORY: u32 = 0x00000010;

        let is_dir = (attributes & FILE_ATTRIBUTE_DIRECTORY) != 0;

    return vfs::symlink::create(to_ref, target, is_dir) ; }

    #[cfg(unix)] 
    {
        return vfs::symlink::create(to_ref, target, false);
    }   
}
