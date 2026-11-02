use std::{
    io::Error, 
    path::Path
};
use crate::vfs;

pub trait ZtdHardlink {
    fn may_be_hardlink_raw(&self) -> Result<bool, Error>;

    fn may_be_hardlink(&self) -> bool;
}

impl<T: AsRef<Path> + ?Sized> ZtdHardlink for T {
    fn may_be_hardlink_raw(&self) -> Result<bool, Error> {
        vfs::hardlink::may_be_raw(self)
    }
    fn may_be_hardlink(&self) -> bool {
        vfs::hardlink::may_be(self)
    }
}
