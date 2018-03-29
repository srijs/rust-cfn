/// The [`AWS::WorkSpaces::Workspace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspaces-workspace.html) resource type.
pub struct Workspace {
    properties: WorkspaceProperties
}

/// Properties for the `Workspace` resource.
#[derive(Serialize, Deserialize)]
pub struct WorkspaceProperties {
    #[serde(rename="BundleId")]
    pub bundle_id: String,
    #[serde(rename="DirectoryId")]
    pub directory_id: String,
    #[serde(rename="RootVolumeEncryptionEnabled")]
    pub root_volume_encryption_enabled: bool,
    #[serde(rename="UserName")]
    pub user_name: String,
    #[serde(rename="UserVolumeEncryptionEnabled")]
    pub user_volume_encryption_enabled: bool,
    #[serde(rename="VolumeEncryptionKey")]
    pub volume_encryption_key: String,
}

impl<'a> ::Resource<'a> for Workspace {
    type Properties = WorkspaceProperties;
    const TYPE: &'static str = "AWS::WorkSpaces::Workspace";
    fn properties(&self) -> &WorkspaceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut WorkspaceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Workspace {}

impl From<WorkspaceProperties> for Workspace {
    fn from(properties: WorkspaceProperties) -> Workspace {
        Workspace { properties }
    }
}

