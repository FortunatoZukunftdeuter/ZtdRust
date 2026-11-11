use std::{
    ffi::OsStr, 
    process::Command
};

pub trait ZtdCommand
{
    fn new_inherited(name: impl AsRef<OsStr>) -> Command;
}
