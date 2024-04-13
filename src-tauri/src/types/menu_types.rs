use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct OpenFile {
    pub file_path: String,
    pub content: String,
}
