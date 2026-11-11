use std::{
    ffi::OsStr,
    process::{
        Stdio,
        Command
    }
};
use crate::amplified::prcs::traits::ZtdCommand;

impl ZtdCommand for Command 
{
    fn new_inherited(name: impl AsRef<OsStr>) -> Command {
        let name = name.as_ref();

        let stdin = Stdio::inherit();
        let stderr = Stdio::inherit();
        let stdout = Stdio::inherit();

        let mut cmd = Command::new(name);
        cmd
            .stdin(stdin)
            .stderr(stderr)
            .stdout(stdout);

    return cmd ; }
}
