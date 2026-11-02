 use std::{
     fs, 
     io::Error, 
     path::Path
 };

pub fn may_be(link: impl AsRef<Path>) -> bool {
    may_be_raw(link).unwrap_or(false)
}

pub fn may_be_raw(link: impl AsRef<Path>) -> Result<bool, Error> {
    let link_ref = link.as_ref();

    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;

        let metadata = fs::metadata(link_ref)?;
        if !metadata.is_file() {
            return Ok(false);
        }
        Ok(metadata.nlink() > 1)
    }

    #[cfg(windows)]
    {
        use std::os::windows::{
            fs::OpenOptionsExt, 
            io::AsRawHandle
        };
        use windows_sys::Win32::Storage::FileSystem::{
            GetFileInformationByHandle, 
            BY_HANDLE_FILE_INFORMATION,
            FILE_FLAG_BACKUP_SEMANTICS
        };

        let file = fs::OpenOptions::new()
            .access_mode(0)
            .custom_flags(FILE_FLAG_BACKUP_SEMANTICS)
            .open(link_ref)?;

        if !file.metadata()?.is_file() {
            return Ok(false);
        }

        let mut info: BY_HANDLE_FILE_INFORMATION = unsafe { std::mem::zeroed() };

        let ok = unsafe { GetFileInformationByHandle(file.as_raw_handle() as _, &mut info) };
        if ok == 0 {
            return Err(Error::last_os_error());
        }

        Ok(info.nNumberOfLinks > 1)
    }

    #[cfg(not(any(unix, windows)))]
    {
        use std::io::ErrorKind;

        Err(Error::new(
            ErrorKind::Unsupported,
            "Hardlink detection is unsupported on target os",
        ))
    }
}
