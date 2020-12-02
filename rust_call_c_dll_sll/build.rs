/// https://doc.rust-lang.org/cargo/reference/build-script-examples.html#building-a-native-library
fn main() -> Result<(), Box<dyn std::error::Error>> {
    dbg!(std::env::current_dir().unwrap());
    if std::env::var("PWD").is_ok() {
        dbg!(std::env::var("PWD")?);
    }
    let out_dir_str = std::env::var("OUT_DIR")?;
    let out_dir = std::path::Path::new(&out_dir_str);
    /* assert_eq!(format!("{:?}", out_dir), out_dir_str);
    thread 'main' panicked at 'assertion failed: `(left == right)`
    left: `"\"/home/pi/workspace/learn_gcc/target/debug/build/rust_call_c_dll-ce1fabefe99e23f2/out\""`,
   right: `"/home/pi/workspace/learn_gcc/target/debug/build/rust_call_c_dll-ce1fabefe99e23f2/out"`', rust_call_c_dll/build.rs:9:5
    */

    // compile dynamic linking library
    let process_exit_status = std::process::Command::new("gcc")
        .args(&["-Wall", "-shared", "src/dll.c", "-o"])
        .arg(out_dir.join("libdll.so").as_os_str())
        // spawn
        .status()?;
    assert_eq!(process_exit_status.code(), Some(0));

    // compile static linking library
    let process_exit_status = std::process::Command::new("gcc")
        // -fPIC arg is optional
        .args(&["-c", "src/sll.c", "-o"])
        .arg(out_dir.join("sll.o").as_os_str())
        .status()?;
    assert_eq!(process_exit_status.code(), Some(0));
    let process_exit_status = std::process::Command::new("ar")
        .current_dir(&out_dir)
        .args(&["crus", "libsll.a", "sll.o"])
        .status()?;
    assert_eq!(process_exit_status.code(), Some(0));

    println!("cargo:rustc-link-search=native={}", out_dir_str);
    println!("cargo:rustc-link-lib=dylib=dll");
    println!("cargo:rustc-link-lib=static=sll");
    println!("cargo:rerun-if-changed=src/dll.c");
    println!("cargo:rerun-if-changed=src/sll.c");
    Ok(())
}
