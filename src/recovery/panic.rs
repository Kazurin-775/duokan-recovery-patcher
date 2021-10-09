use std::fmt::Write;
use std::panic::PanicInfo;

use super::CMD_PIPE;

pub fn panic_handler(info: &PanicInfo) {
    use std::sync::TryLockError;

    // take out_fd out of Mutex
    let maybe_out_fd = match CMD_PIPE.try_lock() {
        Ok(mut guard) => guard.take(),
        Err(TryLockError::Poisoned(poisoned)) => poisoned.into_inner().take(),
        Err(TryLockError::WouldBlock) => None,
    };

    // write the error message to either out_fd or stderr
    if let Some(mut out_fd) = maybe_out_fd {
        write!(out_fd, "FATAL ERROR: {}", info).unwrap_or_else(|_| {
            eprintln!("FATAL ERROR: {}", info);
        });
        // drop out_fd in case it doesn't
        std::mem::drop(out_fd);
    } else {
        eprintln!("FATAL ERROR: {}", info);
    }
}
