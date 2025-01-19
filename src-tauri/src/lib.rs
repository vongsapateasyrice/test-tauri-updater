mod commands;
use tauri_plugin_updater::UpdaterExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(desktop)]
            let _ = app.handle().plugin(tauri_plugin_updater::Builder::new().build());
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                update(handle).await.unwrap();
            });
            println!("meow2");
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
      println!("hasUpdate! :3");
      println!("currentver: {}", update.current_version);
      println!("ver: {}", update.version);
      println!("sign: {}", update.signature);
  
      // alternatively we could also call update.download() and update.install() separately
      match update
        .download(
          |chunk_length, content_length| {
            downloaded += chunk_length;
            println!("downloaded {downloaded} from {content_length:?}");
          },
          || {
            println!("download finished");
          },

        )
        .await {
          Ok(data) => {
            // Handle successful download
            println!("Download successful, got {} bytes", data.len());
            // Use data here...
          },
          Err(e) => {
            // Handle error case
            println!("Download failed: {}", e);
            // Additional error handling...
          }
        }
      println!("update downloaded :3");
      println!("update installed");
      app.restart();
    } else {
      println!("no update aw :(")
    }
  
    Ok(())
  }