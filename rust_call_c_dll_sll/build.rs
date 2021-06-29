/// https://doc.rust-lang.org/cargo/reference/build-script-examples.html#building-a-native-library
fn main() -> Result<(), Box<dyn std::error::Error>> {
    dbg!(std::env::current_dir().unwrap());
    if std::env::var("PWD").is_ok() {
        dbg!(std::env::var("PWD")?);
    }
    let out_dir_str = std::env::var("OUT_DIR")?;
    let out_dir = std::path::Path::new(&out_dir_str);

    // compile dynamic linking library
    let process_exit_status = std::process::Command::new("gcc")
        .args(&["-shared", "src/dll.c", "-o"])
        .arg(out_dir.join("libdll.so"))
        // spawn
        .status()?;
    assert_eq!(process_exit_status.code(), Some(0));

    // compile static linking library
    let process_exit_status = std::process::Command::new("gcc")
        // -fPIC arg is optional
        .args(&["-c", "src/sll.c", "-o"])
        .arg(out_dir.join("sll.o"))
        .status()?;
    assert_eq!(process_exit_status.code(), Some(0));
    let process_exit_status = std::process::Command::new("ar")
        .current_dir(&out_dir)
        // `ar crus` or `ar crv`
        .args(&["rs", "libsll.a", "sll.o"])
        .status()?;
    assert_eq!(process_exit_status.code(), Some(0));

    println!("cargo:rustc-link-search=native={}", out_dir_str);
    println!("cargo:rustc-link-lib=dylib=dll");
    println!("cargo:rustc-link-lib=static=sll");
    
    println!("cargo:rerun-if-changed=src/dll.c");
    println!("cargo:rerun-if-changed=src/sll.c");
    Ok(())
}
