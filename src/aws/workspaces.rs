//! Types for the `WorkSpaces` service.

/// The [`AWS::WorkSpaces::Workspace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspaces-workspace.html) resource type.
#[derive(Debug)]
pub struct Workspace {
    properties: WorkspaceProperties
}

/// Properties for the `Workspace` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceProperties {
    /// Property `BundleId`.
    #[serde(rename = "BundleId")]
    pub bundle_id: ::Value<String>,
    /// Property `DirectoryId`.
    #[serde(rename = "DirectoryId")]
    pub directory_id: ::Value<String>,
    /// Property `RootVolumeEncryptionEnabled`.
    #[serde(rename = "RootVolumeEncryptionEnabled")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root_volume_encryption_enabled: Option<::Value<bool>>,
    /// Property `UserName`.
    #[serde(rename = "UserName")]
    pub user_name: ::Value<String>,
    /// Property `UserVolumeEncryptionEnabled`.
    #[serde(rename = "UserVolumeEncryptionEnabled")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_volume_encryption_enabled: Option<::Value<bool>>,
    /// Property `VolumeEncryptionKey`.
    #[serde(rename = "VolumeEncryptionKey")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume_encryption_key: Option<::Value<String>>,
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
