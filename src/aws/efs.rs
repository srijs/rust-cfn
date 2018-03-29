/// The [`AWS::EFS::FileSystem`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-efs-filesystem.html) resource.
pub struct FileSystem {
    properties: FileSystemProperties
}

/// Properties for the `FileSystem` resource.
#[derive(Serialize, Deserialize)]
pub struct FileSystemProperties {
    #[serde(rename="Encrypted")]
    pub encrypted: bool,
    #[serde(rename="FileSystemTags")]
    pub file_system_tags: Vec<self::file_system::ElasticFileSystemTag>,
    #[serde(rename="KmsKeyId")]
    pub kms_key_id: String,
    #[serde(rename="PerformanceMode")]
    pub performance_mode: String,
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

impl From<FileSystemProperties> for FileSystem {
    fn from(properties: FileSystemProperties) -> FileSystem {
        FileSystem { properties }
    }
}

/// The [`AWS::EFS::MountTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-efs-mounttarget.html) resource.
pub struct MountTarget {
    properties: MountTargetProperties
}

/// Properties for the `MountTarget` resource.
#[derive(Serialize, Deserialize)]
pub struct MountTargetProperties {
    #[serde(rename="FileSystemId")]
    pub file_system_id: String,
    #[serde(rename="IpAddress")]
    pub ip_address: String,
    #[serde(rename="SecurityGroups")]
    pub security_groups: Vec<String>,
    #[serde(rename="SubnetId")]
    pub subnet_id: String,
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

impl From<MountTargetProperties> for MountTarget {
    fn from(properties: MountTargetProperties) -> MountTarget {
        MountTarget { properties }
    }
}

pub mod file_system {
    #[derive(Serialize, Deserialize)]
    pub struct ElasticFileSystemTag {
        #[serde(rename="Key")]
        pub key: String,
        #[serde(rename="Value")]
        pub value: String,
    }

}

