use std::collections::HashMap;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct EditorConfigs {
    pub theme: String,
    pub editor_font_size: f32,
    pub buffer_font_size: f32,
}

#[derive(serde::Serialize)]
pub struct FileListing {
    pub name: String,
    pub path: String,
    pub is_directory: bool,
    pub children: Vec<FileListing>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Session {
    pub directory: String,
    pub opened_buffers: HashMap<String, String>,
}
