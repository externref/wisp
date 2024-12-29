// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod configs_api;
mod files_api;
mod interfaces;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // configs
            configs_api::write_configs,
            configs_api::read_configs,
            // files
            files_api::open_directory,
            files_api::last_session,
            files_api::save_session
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
