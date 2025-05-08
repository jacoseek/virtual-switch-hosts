const COMMANDS: &[&str] = &["start_vpn", "stop_vpn", "get_app_list"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
