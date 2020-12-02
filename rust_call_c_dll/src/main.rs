fn main() {
    let a = std::process::Command::new("gcc")
        .arg("-Wall")
        .arg("").output();
    dbg!(std::env::current_dir().unwrap());
    // println!("cargo:rustc-link-lib=dylib=add");
}
