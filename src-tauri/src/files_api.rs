use tauri::Manager;

use crate::interfaces;

#[tauri::command]
pub fn last_session(app: tauri::AppHandle) -> Option<interfaces::Session> {
    let mut path = app.path().app_local_data_dir().unwrap();
    path.push("session.json");
    if !path.exists() {
        return None;
    }
    serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
}

#[tauri::command]
pub fn save_session(app: tauri::AppHandle, session: interfaces::Session) {
    let mut path = app.path().app_local_data_dir().unwrap();
    path.push("session.json");
    std::fs::write(path, serde_json::to_string_pretty(&session).unwrap()).unwrap();
}

#[tauri::command]
pub fn open_directory(path: String) -> Vec<interfaces::FileListing> {
    let directory_path = std::path::PathBuf::from(path);
    let mut files: Vec<interfaces::FileListing> = vec![];

    for entry in std::fs::read_dir(directory_path).unwrap() {
        let local = entry.unwrap().path();

        let is_directory = local.is_dir();
        let children = if is_directory {
            open_directory(local.to_string_lossy().to_string())
        } else {
            vec![]
        };

        files.push(interfaces::FileListing {
            name: local.file_name().unwrap().to_str().unwrap().to_string(),
            path: local.to_string_lossy().to_string(),
            is_directory,
            children,
        });
    }

    files
}

#[tauri::command]
pub fn read_file(path: String) {}
