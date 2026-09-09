// Copyright (c) Ame (Akeoot/Akeoott) <akeoot@pm.me>. Licensed under the LGPL-3.0 Licence.
// See the LICENSE file in the repository root for full license text.


use crate::{
    update_all as update_all_export, cpu_snapshot, drive_snapshot, gpu_snapshot,
    memory_snapshot, network_snapshot, process_snapshot, system_snapshot
};

#[tauri::command]
pub fn update_all() {
    update_all_export();
}
#[tauri::command]
pub fn cpu_info() -> Result<serde_json::Value, String> {
    let json_str = cpu_snapshot();
    serde_json::from_str(&json_str).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn memory_info() -> Result<serde_json::Value, String> {
    let json_str = memory_snapshot();
    serde_json::from_str(&json_str).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn gpu_info() -> Result<serde_json::Value, String> {
    let json_str = gpu_snapshot();
    serde_json::from_str(&json_str).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn drive_info() -> Result<serde_json::Value, String> {
    let json_str = drive_snapshot();
    serde_json::from_str(&json_str).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn process_info() -> Result<serde_json::Value, String> {
    let json_str = process_snapshot();
    serde_json::from_str(&json_str).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn system_info() -> Result<serde_json::Value, String> {
    let json_str = system_snapshot();
    serde_json::from_str(&json_str).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn network_info() -> Result<serde_json::Value, String> {
    let json_str = network_snapshot();
    serde_json::from_str(&json_str).map_err(|e| e.to_string())
}
