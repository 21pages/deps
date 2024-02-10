use cc::Build;
use std::{env, path::PathBuf};

fn main() {
    build_amf();
}

fn build_amf() {
    let mut builder = Build::new();
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let externals_dir = manifest_dir.parent().unwrap().join("externals");
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed={}", externals_dir.display());

    // system
    #[cfg(windows)]
    println!("cargo:rustc-link-lib=ole32");
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=stdc++");

    // amf
    let amf_path = externals_dir.join("AMF_v1.4.29");
    builder.include(format!("{}/amf/public/common", amf_path.display()));
    builder.include(amf_path.join("amf"));
    for f in vec![
        "AMFFactory.cpp",
        "AMFSTL.cpp",
        "Thread.cpp",
        #[cfg(windows)]
        "Windows/ThreadWindows.cpp",
        #[cfg(target_os = "linux")]
        "Linux/ThreadLinux.cpp",
        "TraceAdapter.cpp",
    ] {
        builder.file(format!("{}/amf/public/common/{}", amf_path.display(), f));
    }

    // crate
    builder.compile("amf");
}
