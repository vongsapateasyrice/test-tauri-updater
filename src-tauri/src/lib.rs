mod commands;
use tauri_plugin_updater::UpdaterExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new()
        .target(tauri_plugin_log::Target::new(
        tauri_plugin_log::TargetKind::Stdout,
        ))
        .target(tauri_plugin_log::Target::new(
        tauri_plugin_log::TargetKind::LogDir {
          file_name: Some("logs".to_string()),
          },
        ))
        .build())
        .setup(|app| {
            #[cfg(desktop)]
            let _ = app.handle().plugin(tauri_plugin_updater::Builder::new().build());
            
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                update(handle).await.unwrap();
            });
            log::info!("meow2");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![commands::greet, commands::add_number])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/


  async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    if let Some(update) = app.updater()?.check().await? {
      let mut downloaded = 0;
      log::info!("hasUpdate! :3");
      log::info!("currentver: {}", update.current_version);
      log::info!("ver: {}", update.version);
      log::info!("sign: {}", update.signature);
  
      // alternatively we could also call update.download() and update.install() separately
      match update
        .download_and_install(
          |chunk_length, content_length| {
            downloaded += chunk_length;
            log::info!("downloaded {downloaded} from {content_length:?}");
          },
          || {
            log::info!("download finished");
          },

        )
        .await {
          Ok(_) => {
            // Handle successful download
            log::info!("Download successful")
            // Use data here...
          },
          Err(e) => {
            // Handle error case
            log::info!("Download failed: {}", e);
            // Additional error handling...
          }
        }
      log::info!("update installed");
      // app.restart();
    } else {
      log::info!("no update aw :(")
    }
  
    Ok(())
  }