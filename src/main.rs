use std::process::Command;

mod recovery;

fn main() {
    // initialize
    recovery::install_panic_handler();
    recovery::parse_args();

    // print some debug info
    ui_println!("Hello, world!");
    ui_println!(
        "Package file: {}",
        recovery::PKG_FILE.lock().unwrap().as_ref().unwrap()
    );

    // mount /system to get access to toybox and sh
    if let Err(err) = recovery::mount::mount_system() {
        ui_println!("Warning: failed to mount /system: {}", err);
    }
    if !std::fs::metadata("/system/bin/toybox").unwrap().is_file() {
        panic!("Error: /system/bin/toybox does not exist");
    }

    // extract payload
    let mut pkg = recovery::package::open_package().unwrap();
    recovery::package::extract_file(&mut pkg, "payload.sh", "/tmp/payload.sh").unwrap();
    recovery::package::extract_file(&mut pkg, "magiskboot", "/tmp/magiskboot").unwrap();

    // execute payload
    let mut cmd = Command::new("/system/bin/sh");
    cmd.arg("/tmp/payload.sh");
    let status = recovery::process::exec_logged(&mut cmd).unwrap().success();
    ui_println!("Success: {}", status);

    recovery::finalize();
}
