use std::{fs::File, os::unix::prelude::FromRawFd, sync::Mutex};

use lazy_static::lazy_static;

mod panic;
mod ui_print;
pub use panic::panic_handler;
use ui_print::UiPrintFd;

lazy_static! {
    pub static ref CMD_PIPE: Mutex<Option<UiPrintFd>> = Mutex::new(None);
    pub static ref PKG_FILE: Mutex<Option<String>> = Mutex::new(None);
}

pub fn parse_args() {
    let mut args = std::env::args().skip(1);

    let api_version_str = args.next().unwrap();
    let cmd_pipe_fd_str = args.next().unwrap();
    let cmd_pipe_fd = cmd_pipe_fd_str.parse().expect("out_fd is not a number");
    let cmd_pipe = unsafe { File::from_raw_fd(cmd_pipe_fd) };
    CMD_PIPE.lock().unwrap().replace(UiPrintFd::new(cmd_pipe));

    let api_version: i32 = api_version_str
        .parse()
        .expect("api_version is not a number");
    assert!(
        api_version >= 1 && api_version <= 3,
        "unsupported API version"
    );

    let pkg_file = args.next().unwrap();
    PKG_FILE.lock().unwrap().replace(pkg_file);

    assert!(args.next().is_none(), "too many arguments");
}

pub fn finalize() {
    CMD_PIPE.lock().unwrap().take();
}
