fn main() {
    tauri_plugin::Builder::new(&["set_theme", "get_theme", "set_color", "get_color"]).build();
}
