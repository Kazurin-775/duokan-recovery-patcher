use std::process::Command;

mod recovery;

fn main() {
    recovery::install_panic_handler();
    recovery::parse_args();

    ui_println!("Hello, world!");
    ui_println!(
        "Package file: {}",
        recovery::PKG_FILE.lock().unwrap().as_ref().unwrap()
    );

    if let Err(err) = recovery::mount::mount_system() {
        ui_println!("Warning: failed to mount /system: {}", err);
    }
    if !std::fs::metadata("/system/bin/toybox").unwrap().is_file() {
        panic!("Error: /system/bin/toybox does not exist");
    }

    let status = recovery::process::exec_logged(Command::new("/system/bin/toybox").arg("mount"))
        .unwrap()
        .success();
    ui_println!("Success: {}", status);

    recovery::finalize();
}
