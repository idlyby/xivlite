use std::process::Command;

use rustc_version::{version_meta, Channel};

fn main() {
    let output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .unwrap();

    let git_hash = String::from_utf8(output.stdout).unwrap();
    let mut version = String::new();
    if cfg!(debug_assertions) {
        version.insert_str(0, &git_hash);
    } else {
        version.insert_str(0, &format!("{} {}", env!("CARGO_PKG_VERSION"), git_hash));
    }
    println!("cargo:rustc-env=APP_VERSION={}", version);

    match version_meta().unwrap().channel {
        Channel::Stable => println!("cargo:rustc-cfg=RUSTC_IS_STABLE"),
        Channel::Beta => println!("cargo:rustc-cfg=RUSTC_IS_BETA"),
        Channel::Nightly => println!("cargo:rustc-cfg=RUSTC_IS_NIGHTLY"),
        Channel::Dev => println!("cargo:rustc-cfg=RUSTC_IS_DEV"),
    }
}