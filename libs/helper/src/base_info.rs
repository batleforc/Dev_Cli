use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct BaseInfo {
    pub namespace: Option<String>,
    pub workspace_name: Option<String>,
    pub workspace_id: Option<String>,
    pub podname: Option<String>,
    pub is_in_pod: bool,
}
