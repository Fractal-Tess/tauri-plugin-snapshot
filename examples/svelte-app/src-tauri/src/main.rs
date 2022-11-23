#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[tauri::command]
async fn my_custom_command(app: tauri::AppHandle, window: tauri::Window) -> Result<(), String> {
  println!("Custom command");
  Ok(())
}

fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_screen_shot::init())
    .invoke_handler(tauri::generate_handler![my_custom_command])
    .run(tauri::generate_context!())
    .expect("failed to run app");
}
