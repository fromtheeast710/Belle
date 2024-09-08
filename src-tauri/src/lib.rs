use tauri::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  Builder::default()
    .run(generate_context!())
    .expect("error while running tauri application");
}
