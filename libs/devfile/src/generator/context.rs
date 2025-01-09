use crd::dev_work_space::{DevWorkspace, DevWorkspaceTemplate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DevfileContext {
    /// Suffix to append on generated name
    pub suffix: Option<String>,
    /// devWorkspace
    pub dev_workspace: Option<DevWorkspace>,
    /// devWorkspaceTemplate
    pub dev_workspace_template: Option<DevWorkspaceTemplate>,
    /// devfile
    pub devfile: Option<String>,
}
