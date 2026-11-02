use std::{
    io::Error, 
    path::Path
};
use crate::{
    vfs,
    kern::*
};

pub fn copy(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<u64, Error>
{
    let from_ref = from.as_ref();
    let to_ref = to.as_ref();
    if from_ref.is_dir() {
        return vfs::dir::copy(from_ref, to_ref).map(|_| 0);
    }
    else if from_ref.is_file() {
        return vfs::file::copy(from_ref, to_ref);
    }
    else if from_ref.is_symlink() {
        return vfs::symlink::copy(from_ref, to_ref).map(|_| 0);
    }
    else if from_ref.is_hardlink() {
        return vfs::hardlink::create(from_ref, to_ref).map(|_| 0);
    }
    else {
        return Err(Error::other("Unknown entry type"));
    } 
}
