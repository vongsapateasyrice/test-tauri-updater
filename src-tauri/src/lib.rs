mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(desktop)]
            let _ = app.handle().plugin(tauri_plugin_updater::Builder::new().build());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![commands::greet, commands::add_number])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

