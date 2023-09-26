#[cfg(target_os="macos")]
fn main() {
    println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path");
}
