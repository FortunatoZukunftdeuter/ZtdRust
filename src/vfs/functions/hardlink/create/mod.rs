use std::{
    io::Error,
    path::Path,
    fs::hard_link
};

pub fn create(link: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<(), Error> 
{
    let link_ref = link.as_ref();
    let to_ref = to.as_ref();

return hard_link(to_ref, link_ref) ; }
