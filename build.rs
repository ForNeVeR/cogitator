use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!(r"cargo:rustc-link-search=windivert\WinDivert-1.1.8-MSVC\amd64");
    let target = env::var("OUT_DIR").unwrap();
    fs::copy(
        r"windivert\WinDivert-1.1.8-MSVC\amd64\WinDivert.dll",
        Path::new(&target).join(r"..\..\..\WinDivert.dll"));
    fs::copy(
        r"windivert\WinDivert-1.1.8-MSVC\amd64\WinDivert64.sys",
        Path::new(&target).join(r"..\..\..\WinDivert64.sys"));
}
