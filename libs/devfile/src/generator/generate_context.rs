use serde::{Deserialize, Serialize};

use super::projects::Project;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GenerateDevfileContext {
    pub devfile_path: Option<String>,
    pub devfile_url: Option<String>,
    pub devfile_content: Option<String>,
    pub editor_path: Option<String>,
    pub editor_url: Option<String>,
    pub editor_content: Option<String>,
    pub projects: Vec<Project>,
}

impl GenerateDevfileContext {
    pub async fn generate(&self) -> Result<(), ()> {
        // ...
        Ok(())
    }
}
