mod recovery;

fn main() {
    std::panic::set_hook(Box::new(recovery::panic_handler));
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

    recovery::finalize();
}
