/// The [`AWS::WorkSpaces::Workspace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspaces-workspace.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Workspace {
    properties: WorkspaceProperties
}

/// Properties for the `Workspace` resource.
#[derive(Serialize, Deserialize)]
pub struct WorkspaceProperties {
    #[serde(rename="BundleId")]
    pub bundle_id: (),
    #[serde(rename="DirectoryId")]
    pub directory_id: (),
    #[serde(rename="RootVolumeEncryptionEnabled")]
    pub root_volume_encryption_enabled: (),
    #[serde(rename="UserName")]
    pub user_name: (),
    #[serde(rename="UserVolumeEncryptionEnabled")]
    pub user_volume_encryption_enabled: (),
    #[serde(rename="VolumeEncryptionKey")]
    pub volume_encryption_key: (),
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

impl From<WorkspaceProperties> for Workspace {
    fn from(properties: WorkspaceProperties) -> Workspace {
        Workspace { properties }
    }
}

