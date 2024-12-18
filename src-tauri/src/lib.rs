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
fn file_search_simulate(mut path_str: String) -> Vec<ResultData> {
    let mut query_results: Vec<ResultData> = Vec::new();
    if path_str.starts_with("~") {
        if let Some(home_dir) = dirs::home_dir() {
            path_str = path_str.replacen("~", home_dir.to_str().unwrap(), 1);
        } else {
            return query_results;
        }
    }
    let path = std::path::Path::new(&path_str);
    if !path.exists() || !path.is_dir() {
        return query_results;
    }
    for file in std::fs::read_dir(path).unwrap() {
        if let Ok(entry) = file {
            let file_path = entry.path();
            let file_name = entry.file_name();
            query_results.push(ResultData {
                path: file_path.to_str().unwrap_or_default().into(),
                name: file_name.to_str().unwrap_or_default().into(),
                is_dir: file_path.is_dir(),
            });
        }
    }
    query_results
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
