// Copyright (c) Akeoot / Akeoott <contact@kyanite.mov>. Licensed under the GPL-3.0 Licence.
// See the LICENSE file in the repository root for full license text.

#![allow(unsafe_op_in_unsafe_fn)]

mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::update_all,
            commands::cpu_info,
            commands::memory_info,
            commands::gpu_info,
            commands::drive_info,
            commands::process_info,
            commands::system_info,
            commands::network_info,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
// Map dotnet backend methods

use libloading::{Library, Symbol};
use once_cell::sync::Lazy;
use std::env;
use std::ffi::CStr;
use std::fs::File;
use std::io::Write;

include!(concat!(env!("OUT_DIR"), "/native_lib_bytes.rs"));

static LIB: Lazy<Library> = Lazy::new(|| {
    let temp_dir = env::temp_dir();

    let ext = if cfg!(target_os = "windows") {
        "dll"
    } else if cfg!(target_os = "macos") {
        "dylib"
    } else {
        "so"
    };

    let lib_name = format!("dotnet-backend-{}.{}", std::process::id(), ext);
    let lib_path = temp_dir.join(lib_name);

    let mut file =
        File::create(&lib_path).expect("Failed to create temporary file for native library");
    file.write_all(NATIVE_LIB_BYTES)
        .expect("Failed to write native library to temporary file");
    file.sync_all().expect("Failed to sync temporary file");

    unsafe {
        Library::new(&lib_path)
            .unwrap_or_else(|e| panic!("Failed to load native library at {:?}: {}", lib_path, e))
    }
});

type UpdateAllFn = unsafe extern "C" fn();
type SnapshotFn = unsafe extern "C" fn() -> *mut std::os::raw::c_char;

unsafe fn free_string(ptr: *mut std::os::raw::c_char) {
    type FreeStringFn = unsafe extern "C" fn(*mut std::os::raw::c_char);
    let func: Symbol<FreeStringFn> = LIB
        .get(b"free_string")
        .expect("Missing exported function 'free_string'");
    func(ptr);
}

unsafe fn call_string_fn<F>(func: F) -> String
where
    F: FnOnce() -> *mut std::os::raw::c_char,
{
    let ptr = func();
    if ptr.is_null() {
        String::new()
    } else {
        let cstr = CStr::from_ptr(ptr);
        let s = cstr.to_string_lossy().into_owned();
        free_string(ptr);
        s
    }
}

pub fn update_all() {
    unsafe {
        let func: Symbol<UpdateAllFn> = LIB
            .get(b"update_all")
            .expect("Missing exported function 'update_all'");
        func();
    }
}

pub fn cpu_snapshot() -> String {
    unsafe {
        call_string_fn(|| {
            let f: Symbol<SnapshotFn> = LIB
                .get(b"cpu_snapshot")
                .expect("Missing exported function 'cpu_snapshot'");
            f()
        })
    }
}

pub fn memory_snapshot() -> String {
    unsafe {
        call_string_fn(|| {
            let f: Symbol<SnapshotFn> = LIB
                .get(b"memory_snapshot")
                .expect("Missing exported function 'memory_snapshot'");
            f()
        })
    }
}

pub fn gpu_snapshot() -> String {
    unsafe {
        call_string_fn(|| {
            let f: Symbol<SnapshotFn> = LIB
                .get(b"gpu_snapshot")
                .expect("Missing exported function 'gpu_snapshot'");
            f()
        })
    }
}

pub fn drive_snapshot() -> String {
    unsafe {
        call_string_fn(|| {
            let f: Symbol<SnapshotFn> = LIB
                .get(b"drive_snapshot")
                .expect("Missing exported function 'drive_snapshot'");
            f()
        })
    }
}

pub fn process_snapshot() -> String {
    unsafe {
        call_string_fn(|| {
            let f: Symbol<SnapshotFn> = LIB
                .get(b"process_snapshot")
                .expect("Missing exported function 'process_snapshot'");
            f()
        })
    }
}

pub fn system_snapshot() -> String {
    unsafe {
        call_string_fn(|| {
            let f: Symbol<SnapshotFn> = LIB
                .get(b"system_snapshot")
                .expect("Missing exported function 'system_snapshot'");
            f()
        })
    }
}

pub fn network_snapshot() -> String {
    unsafe {
        call_string_fn(|| {
            let f: Symbol<SnapshotFn> = LIB
                .get(b"network_snapshot")
                .expect("Missing exported function 'network_snapshot'");
            f()
        })
    }
}
