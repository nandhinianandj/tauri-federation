// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_barcode_scanner;
use tauri_plugin_biometric;
use tauri_plugin_opener;


#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::cbwrite]
fn cbwrite(text: &str) -> String {
j   app.clipboard()
        .write_text("Tauri is awesome!".to_string())
        .unwrap();
}

#[tauri::cbread]
fn cbread() -> String {
    // Read content from clipboard
    let content = app.clipboard().read_text();
    println!("{:?}", content.unwrap());
    content
}
// Prints "Tauri is awesome!" to the terminal
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_biometric::init())
        .plugin(tauri_plugin_barcode_scanner::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
