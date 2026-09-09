// Copyright (c) Akeoot / Akeoott <contact@kyanite.mov>. Licensed under the GPL-3.0 Licence.
// See the LICENSE file in the repository root for full license text.use std::{thread, time::Duration};

use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::fs;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target = env::var("TARGET").unwrap();

    // Map Rust target triple to .NET RID
    let rid = match target.as_str() {
        "x86_64-unknown-linux-gnu" => "linux-x64",
        "aarch64-unknown-linux-gnu" => "linux-arm64",
        "x86_64-pc-windows-msvc" => "win-x64",
        "aarch64-pc-windows-msvc" => "win-arm64",
        "x86_64-apple-darwin" => "osx-x64",
        "aarch64-apple-darwin" => "osx-arm64",
        _ => panic!("Unsupported target triple: {}", target),
    };

    let status = Command::new("dotnet")
        .current_dir("../csharp_lib")
        .args([
            "publish",
            "-c", "Release",
            "-r", rid,
            "--self-contained", "true",
            "-p:NativeLib=Shared",
            "-o", out_dir.to_str().unwrap(),
        ])
        .status()
        .expect("Failed to run `dotnet publish`. Is the .NET SDK installed?");
    assert!(status.success(), "`dotnet publish` failed");

    // Auto‑detect the generated shared library
    let assembly_name = "dotnet-backend";
    let entries = fs::read_dir(&out_dir)
        .expect("Failed to read output directory");

    let lib_path = entries
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .find(|path| {
            // Check if it's a shared library with the right name
            if !path.is_file() { return false; }
            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            // On Linux/macOS, names may have 'lib' prefix, on Windows, they don't.
            // We just check if the name contains the assembly name (case‑insensitive).
            file_name.to_lowercase().contains(&assembly_name.to_lowercase())
                && (
                    path.extension().and_then(|e| e.to_str()) == Some("so") ||
                    path.extension().and_then(|e| e.to_str()) == Some("dylib") ||
                    path.extension().and_then(|e| e.to_str()) == Some("dll")
                )
        })
        .unwrap_or_else(|| {
            panic!(
                "No shared library found in {:?} containing '{}' with extension .so/.dylib/.dll",
                out_dir, assembly_name
            );
        });

    println!("cargo:warning=Found library at: {:?}", lib_path);

    // Read and embed the file
    let bytes = fs::read(&lib_path)
        .unwrap_or_else(|e| panic!("Failed to read {:?}: {}", lib_path, e));

    let generated_rs = out_dir.join("native_lib_bytes.rs");
    fs::write(
        &generated_rs,
        format!("pub const NATIVE_LIB_BYTES: &[u8] = &{:?};", bytes),
    )
    .unwrap_or_else(|e| panic!("Failed to write generated Rust file: {}", e));

    println!("cargo:rerun-if-changed=csharp_lib");
    println!("cargo:rerun-if-changed=csharp_lib/Exports.cs");
    println!("cargo:rerun-if-changed=csharp_lib/ClassLibrary.csproj");
    println!("cargo:rustc-env=OUT_DIR={}", out_dir.display());

    tauri_build::build()
}
