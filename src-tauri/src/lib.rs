mod error;
mod file_merge;
mod file_split;
mod progress_payload;
use file_merge::file_merge;
use file_split::{file_split, get_line_count};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(debug_assertions)]
    let devtoos = tauri_plugin_devtools::init();
    let mut builder = tauri::Builder::default();
    #[cfg(debug_assertions)]
    {
        builder = builder.plugin(devtoos);
        // builder.setup(|app| {
        //     {
        //       let window = app.get_webview_window("main").unwrap();
        //       window.open_devtools();
        //       window.close_devtools();
        //     }
        //     Ok(())
        //   });
    }

    builder
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            file_split,
            get_line_count,
            file_merge
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
