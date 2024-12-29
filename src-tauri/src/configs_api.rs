use tauri::Manager;

use crate::interfaces;

pub fn default_configs() -> interfaces::EditorConfigs {
    interfaces::EditorConfigs {
        theme: "dracula".to_string(),
        editor_font_size: 12.0,
        buffer_font_size: 14.0,
    }
}

/// Write configuration data to the config file
// invoke("write_configs", {configs: {}})
#[tauri::command]
pub fn write_configs(app: tauri::AppHandle, configs: interfaces::EditorConfigs) {
    let data_dir = app
        .path()
        .app_local_data_dir()
        .unwrap()
        .join("configs.json");
    std::fs::write(data_dir, serde_json::to_string_pretty(&configs).unwrap()).unwrap();
}

/// Read JSON data from the config.json file
// invoke("read_configs")
#[tauri::command]
pub fn read_configs(app: tauri::AppHandle) -> interfaces::EditorConfigs {
    let data_dir = app
        .path()
        .app_local_data_dir()
        .unwrap()
        .join("configs.json");
    if !data_dir.exists() {
        std::fs::write(
            data_dir,
            serde_json::to_string_pretty(&default_configs()).unwrap(),
        )
        .unwrap();
        return default_configs();
    }
    serde_json::from_str(&std::fs::read_to_string(data_dir).unwrap()).unwrap()
}
