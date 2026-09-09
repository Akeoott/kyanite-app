#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
fn main() {
    #[cfg(target_os = "linux")]
    unsafe {
        // Doesn't work on wayland so gotta add these var's...
        std::env::set_var("GDK_BACKEND", "x11");
        std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
    }
    kyanite_mov_lib::run()
}
