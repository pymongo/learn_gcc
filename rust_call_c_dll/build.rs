fn main() -> Result<(), Box<dyn std::error::Error>> {
    dbg!(std::env::current_dir().unwrap());
    if std::env::var("PWD").is_ok() {
        dbg!(std::env::var("PWD")?);
    }
    let out_dir = std::env::var("OUT_DIR")?;
    dbg!(&out_dir);
    // let gcc_status = std::process::Command::new("gcc")
    //     .arg("-Wall")
    //     .arg("-shared")
    //     .arg("-o ../target/")
    //     // spawn
    //     .status()?;
    // assert_eq!(gcc_status.code(), Some(1));
    // println!("cargo:rustc-link-lib=dylib=add");
    Ok(())
}
