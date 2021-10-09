use std::{ffi::OsStr, path::Path};

use nix::{errno::Errno, mount::MsFlags};

pub fn mount_system() -> Result<(), Errno> {
    nix::mount::mount(
        Some(Path::new("/dev/block/by-name/system")),
        Path::new("/system"),
        Some(OsStr::new("ext4")),
        MsFlags::MS_RDONLY,
        None::<&OsStr>,
    )
}
