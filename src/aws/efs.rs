//! Types for the `EFS` service.

/// The [`AWS::EFS::FileSystem`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-efs-filesystem.html) resource type.
#[derive(Debug)]
pub struct FileSystem {
    properties: FileSystemProperties
}

/// Properties for the `FileSystem` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct FileSystemProperties {
    /// Property `Encrypted`.
    #[serde(rename = "Encrypted")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<::Value<bool>>,
    /// Property `FileSystemTags`.
    #[serde(rename = "FileSystemTags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_system_tags: Option<::ValueList<self::file_system::ElasticFileSystemTag>>,
    /// Property `KmsKeyId`.
    #[serde(rename = "KmsKeyId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<::Value<String>>,
    /// Property `PerformanceMode`.
    #[serde(rename = "PerformanceMode")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performance_mode: Option<::Value<String>>,
}

impl<'a> ::Resource<'a> for FileSystem {
    type Properties = FileSystemProperties;
    const TYPE: &'static str = "AWS::EFS::FileSystem";
    fn properties(&self) -> &FileSystemProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut FileSystemProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for FileSystem {}

impl From<FileSystemProperties> for FileSystem {
    fn from(properties: FileSystemProperties) -> FileSystem {
        FileSystem { properties }
    }
}

/// The [`AWS::EFS::MountTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-efs-mounttarget.html) resource type.
#[derive(Debug)]
pub struct MountTarget {
    properties: MountTargetProperties
}

/// Properties for the `MountTarget` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct MountTargetProperties {
    /// Property `FileSystemId`.
    #[serde(rename = "FileSystemId")]
    pub file_system_id: ::Value<String>,
    /// Property `IpAddress`.
    #[serde(rename = "IpAddress")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<::Value<String>>,
    /// Property `SecurityGroups`.
    #[serde(rename = "SecurityGroups")]
    pub security_groups: ::ValueList<String>,
    /// Property `SubnetId`.
    #[serde(rename = "SubnetId")]
    pub subnet_id: ::Value<String>,
}

impl<'a> ::Resource<'a> for MountTarget {
    type Properties = MountTargetProperties;
    const TYPE: &'static str = "AWS::EFS::MountTarget";
    fn properties(&self) -> &MountTargetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut MountTargetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for MountTarget {}

impl From<MountTargetProperties> for MountTarget {
    fn from(properties: MountTargetProperties) -> MountTarget {
        MountTarget { properties }
    }
}

pub mod file_system {
    //! Property types for the `FileSystem` resource.

    /// The [`AWS::EFS::FileSystem.ElasticFileSystemTag`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-efs-filesystem-filesystemtags.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ElasticFileSystemTag {
        /// Property `Key`.
        #[serde(rename = "Key")]
        pub key: ::Value<String>,
        /// Property `Value`.
        #[serde(rename = "Value")]
        pub value: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(ElasticFileSystemTag);
}
