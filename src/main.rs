mod recovery;

fn main() {
    std::panic::set_hook(Box::new(recovery::panic_handler));
    recovery::parse_args();

    ui_println!("Hello, world!");
    ui_println!(
        "Package file: {}",
        recovery::PKG_FILE.lock().unwrap().as_ref().unwrap()
    );

    recovery::finalize();
}
