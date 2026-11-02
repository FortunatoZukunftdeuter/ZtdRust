use std::{
    path::Path,
    io::Error,
    os
};

pub fn create(link: impl AsRef<Path>, to: impl AsRef<Path>, _is_dir: bool) -> Result<(), Error>
{
    let link_ref = link.as_ref();
    let to_ref = to.as_ref();

    #[cfg(windows)] 
    {
        if _is_dir {
            return os::windows::fs::symlink_dir(to_ref, link_ref);
        } else {
            return os::windows::fs::symlink_file(to_ref, link_ref);
        }
    }

    #[cfg(unix)] 
    {
        return os::unix::fs::symlink(to_ref, link_ref);
    }
}
