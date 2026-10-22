mod command;
mod db;

use command::{create_post, detail_post, get_posts, update_post, upload_post};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_posts,
            create_post,
            detail_post,
            update_post,
            upload_post
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
