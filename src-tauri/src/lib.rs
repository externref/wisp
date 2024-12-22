// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use std::path::Path;

#[derive(Debug, serde::Serialize)]
struct FileData {
    name: String,
    code: String,
    path: String,
}

#[tauri::command]
fn get_file_data(mut path_str: String) -> Result<FileData, String> {
    if path_str.starts_with("~") {
        if let Some(home_dir) = dirs::home_dir() {
            path_str = path_str.replacen("~", home_dir.to_str().unwrap(), 1);
        } else {
            return Err("Home directory not found".into());
        }
    }
    let path = Path::new(&path_str);
    let code =
        std::fs::read_to_string(path).map_err(|err| format!("Failed to read file: {}", err))?;
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| "Invalid file name".to_string())?
        .to_string();
    Ok(FileData {
        name,
        code,
        path: path.to_str().unwrap().into(),
    })
}

#[derive(serde::Serialize)]
struct ResultData {
    name: String,
    path: String,
    is_dir: bool,
}


#[tauri::command]
fn file_search_simulate(path_str: String) -> Result<Vec<ResultData>, String> {
    let path = std::path::PathBuf::from(path_str);

    if !path.exists() || !path.is_dir() {
        return Err("Invalid directory path".into());
    }

    let mut results = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_type = entry.file_type().unwrap();
                results.push(ResultData {
                    name: entry.file_name().to_string_lossy().to_string(),
                    path: entry.path().to_string_lossy().to_string(),
                    is_dir: file_type.is_dir(),
                });
            }
        }
    }
    Ok(results)
}


#[tauri::command]
fn write_to_file(path: String, contents: String) {
    std::fs::write(std::path::Path::new(&path), contents).unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_file_data,
            file_search_simulate,
            write_to_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
