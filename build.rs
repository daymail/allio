use std::env;

fn main() {
    let home = env::var("HOME").unwrap();
    let palette_path = format!("{}/.local/share/wallwatch/exports/palette.rs", home);
    println!("cargo:rustc-env=PALETTE_PATH={}", palette_path);
    println!("cargo:rerun-if-changed={}", palette_path);
}
