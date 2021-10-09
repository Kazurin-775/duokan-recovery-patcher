use std::{
    fs::File,
    io::{BufWriter, Write},
};

pub struct UiPrintFd {
    inner: BufWriter<File>,
}

impl UiPrintFd {
    pub fn new(file: File) -> UiPrintFd {
        let mut inner = BufWriter::new(file);
        write!(inner, "ui_print ").unwrap();
        UiPrintFd { inner }
    }

    pub fn flush(&mut self) {
        self.inner.flush().unwrap();
    }
}

impl std::fmt::Write for UiPrintFd {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        let mut lines = s.split('\n');
        // write the first line
        let first_line = lines.next().unwrap();
        write!(self.inner, "{}", first_line).map_err(|_| std::fmt::Error)?;
        // write the remaining lines
        for line in lines {
            write!(self.inner, "\nui_print {}", line).map_err(|_| std::fmt::Error)?;
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! ui_print {
    ($($args:tt)+) => ({
        use std::fmt::Write;

        if let Some(cmd_pipe) = $crate::recovery::CMD_PIPE.lock().unwrap().as_mut() {
            write!(cmd_pipe, $($args)+).unwrap();
            cmd_pipe.flush();
        } else {
            print!($($args)+);
        }
    });
}

#[macro_export]
macro_rules! ui_println {
    () => ({
        $crate::ui_print!("\n")
    });
    ($fmt:expr) => ({
        $crate::ui_print!(concat!($fmt, "\n"))
    });
    ($fmt:expr, $($args:tt)+) => ({
        $crate::ui_print!(concat!($fmt, "\n"), $($args)+)
    });
}
