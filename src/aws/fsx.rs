//! Types for the `FSx` service.

/// The [`AWS::FSx::FileSystem`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fsx-filesystem.html) resource type.
#[derive(Debug, Default)]
pub struct FileSystem {
    properties: FileSystemProperties
}

/// Properties for the `FileSystem` resource.
#[derive(Debug, Default)]
pub struct FileSystemProperties {
    /// Property [`BackupId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fsx-filesystem.html#cfn-fsx-filesystem-backupid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub backup_id: Option<::Value<String>>,
    /// Property [`FileSystemType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fsx-filesystem.html#cfn-fsx-filesystem-filesystemtype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub file_system_type: ::Value<String>,
    /// Property [`FileSystemTypeVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fsx-filesystem.html#cfn-fsx-filesystem-filesystemtypeversion).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub file_system_type_version: Option<::Value<String>>,
    /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fsx-filesystem.html#cfn-fsx-filesystem-kmskeyid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kms_key_id: Option<::Value<String>>,
    /// Property [`LustreConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fsx-filesystem.html#cfn-fsx-filesystem-lustreconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub lustre_configuration: Option<::Value<self::file_system::LustreConfiguration>>,
    /// Property [`OntapConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fsx-filesystem.html#cfn-fsx-filesystem-ontapconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ontap_configuration: Option<::Value<self::file_system::OntapConfiguration>>,
    /// Property [`OpenZFSConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fsx-filesystem.html#cfn-fsx-filesystem-openzfsconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub open_zfs_configuration: Option<::Value<self::file_system::OpenZFSConfiguration>>,
    /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fsx-filesystem.html#cfn-fsx-filesystem-securitygroupids).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub security_group_ids: Option<::ValueList<String>>,
    /// Property [`StorageCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fsx-filesystem.html#cfn-fsx-filesystem-storagecapacity).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub storage_capacity: Option<::Value<u32>>,
    /// Property [`StorageType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fsx-filesystem.html#cfn-fsx-filesystem-storagetype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub storage_type: Option<::Value<String>>,
    /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fsx-filesystem.html#cfn-fsx-filesystem-subnetids).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub subnet_ids: ::ValueList<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fsx-filesystem.html#cfn-fsx-filesystem-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`WindowsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fsx-filesystem.html#cfn-fsx-filesystem-windowsconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub windows_configuration: Option<::Value<self::file_system::WindowsConfiguration>>,
}

impl ::serde::Serialize for FileSystemProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref backup_id) = self.backup_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BackupId", backup_id)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FileSystemType", &self.file_system_type)?;
        if let Some(ref file_system_type_version) = self.file_system_type_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FileSystemTypeVersion", file_system_type_version)?;
        }
        if let Some(ref kms_key_id) = self.kms_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
        }
        if let Some(ref lustre_configuration) = self.lustre_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LustreConfiguration", lustre_configuration)?;
        }
        if let Some(ref ontap_configuration) = self.ontap_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OntapConfiguration", ontap_configuration)?;
        }
        if let Some(ref open_zfs_configuration) = self.open_zfs_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OpenZFSConfiguration", open_zfs_configuration)?;
        }
        if let Some(ref security_group_ids) = self.security_group_ids {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", security_group_ids)?;
        }
        if let Some(ref storage_capacity) = self.storage_capacity {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StorageCapacity", storage_capacity)?;
        }
        if let Some(ref storage_type) = self.storage_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StorageType", storage_type)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", &self.subnet_ids)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref windows_configuration) = self.windows_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WindowsConfiguration", windows_configuration)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for FileSystemProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<FileSystemProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = FileSystemProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type FileSystemProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut backup_id: Option<::Value<String>> = None;
                let mut file_system_type: Option<::Value<String>> = None;
                let mut file_system_type_version: Option<::Value<String>> = None;
                let mut kms_key_id: Option<::Value<String>> = None;
                let mut lustre_configuration: Option<::Value<self::file_system::LustreConfiguration>> = None;
                let mut ontap_configuration: Option<::Value<self::file_system::OntapConfiguration>> = None;
                let mut open_zfs_configuration: Option<::Value<self::file_system::OpenZFSConfiguration>> = None;
                let mut security_group_ids: Option<::ValueList<String>> = None;
                let mut storage_capacity: Option<::Value<u32>> = None;
                let mut storage_type: Option<::Value<String>> = None;
                let mut subnet_ids: Option<::ValueList<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut windows_configuration: Option<::Value<self::file_system::WindowsConfiguration>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "BackupId" => {
                            backup_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FileSystemType" => {
                            file_system_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FileSystemTypeVersion" => {
                            file_system_type_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsKeyId" => {
                            kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LustreConfiguration" => {
                            lustre_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OntapConfiguration" => {
                            ontap_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OpenZFSConfiguration" => {
                            open_zfs_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecurityGroupIds" => {
                            security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StorageCapacity" => {
                            storage_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StorageType" => {
                            storage_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SubnetIds" => {
                            subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "WindowsConfiguration" => {
                            windows_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(FileSystemProperties {
                    backup_id: backup_id,
                    file_system_type: file_system_type.ok_or(::serde::de::Error::missing_field("FileSystemType"))?,
                    file_system_type_version: file_system_type_version,
                    kms_key_id: kms_key_id,
                    lustre_configuration: lustre_configuration,
                    ontap_configuration: ontap_configuration,
                    open_zfs_configuration: open_zfs_configuration,
                    security_group_ids: security_group_ids,
                    storage_capacity: storage_capacity,
                    storage_type: storage_type,
                    subnet_ids: subnet_ids.ok_or(::serde::de::Error::missing_field("SubnetIds"))?,
                    tags: tags,
                    windows_configuration: windows_configuration,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for FileSystem {
    type Properties = FileSystemProperties;
    const TYPE: &'static str = "AWS::FSx::FileSystem";
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

/// The [`AWS::FSx::Snapshot`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fsx-snapshot.html) resource type.
#[derive(Debug, Default)]
pub struct Snapshot {
    properties: SnapshotProperties
}

/// Properties for the `Snapshot` resource.
#[derive(Debug, Default)]
pub struct SnapshotProperties {
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fsx-snapshot.html#cfn-fsx-snapshot-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fsx-snapshot.html#cfn-fsx-snapshot-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`VolumeId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fsx-snapshot.html#cfn-fsx-snapshot-volumeid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub volume_id: ::Value<String>,
}

impl ::serde::Serialize for SnapshotProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeId", &self.volume_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SnapshotProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SnapshotProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SnapshotProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SnapshotProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut volume_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VolumeId" => {
                            volume_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SnapshotProperties {
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    tags: tags,
                    volume_id: volume_id.ok_or(::serde::de::Error::missing_field("VolumeId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Snapshot {
    type Properties = SnapshotProperties;
    const TYPE: &'static str = "AWS::FSx::Snapshot";
    fn properties(&self) -> &SnapshotProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SnapshotProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Snapshot {}

impl From<SnapshotProperties> for Snapshot {
    fn from(properties: SnapshotProperties) -> Snapshot {
        Snapshot { properties }
    }
}

/// The [`AWS::FSx::StorageVirtualMachine`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fsx-storagevirtualmachine.html) resource type.
#[derive(Debug, Default)]
pub struct StorageVirtualMachine {
    properties: StorageVirtualMachineProperties
}

/// Properties for the `StorageVirtualMachine` resource.
#[derive(Debug, Default)]
pub struct StorageVirtualMachineProperties {
    /// Property [`ActiveDirectoryConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fsx-storagevirtualmachine.html#cfn-fsx-storagevirtualmachine-activedirectoryconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub active_directory_configuration: Option<::Value<self::storage_virtual_machine::ActiveDirectoryConfiguration>>,
    /// Property [`FileSystemId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fsx-storagevirtualmachine.html#cfn-fsx-storagevirtualmachine-filesystemid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub file_system_id: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fsx-storagevirtualmachine.html#cfn-fsx-storagevirtualmachine-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`RootVolumeSecurityStyle`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fsx-storagevirtualmachine.html#cfn-fsx-storagevirtualmachine-rootvolumesecuritystyle).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub root_volume_security_style: Option<::Value<String>>,
    /// Property [`SvmAdminPassword`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fsx-storagevirtualmachine.html#cfn-fsx-storagevirtualmachine-svmadminpassword).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub svm_admin_password: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fsx-storagevirtualmachine.html#cfn-fsx-storagevirtualmachine-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for StorageVirtualMachineProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref active_directory_configuration) = self.active_directory_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ActiveDirectoryConfiguration", active_directory_configuration)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FileSystemId", &self.file_system_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref root_volume_security_style) = self.root_volume_security_style {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RootVolumeSecurityStyle", root_volume_security_style)?;
        }
        if let Some(ref svm_admin_password) = self.svm_admin_password {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SvmAdminPassword", svm_admin_password)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for StorageVirtualMachineProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<StorageVirtualMachineProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = StorageVirtualMachineProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type StorageVirtualMachineProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut active_directory_configuration: Option<::Value<self::storage_virtual_machine::ActiveDirectoryConfiguration>> = None;
                let mut file_system_id: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut root_volume_security_style: Option<::Value<String>> = None;
                let mut svm_admin_password: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ActiveDirectoryConfiguration" => {
                            active_directory_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FileSystemId" => {
                            file_system_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RootVolumeSecurityStyle" => {
                            root_volume_security_style = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SvmAdminPassword" => {
                            svm_admin_password = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(StorageVirtualMachineProperties {
                    active_directory_configuration: active_directory_configuration,
                    file_system_id: file_system_id.ok_or(::serde::de::Error::missing_field("FileSystemId"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    root_volume_security_style: root_volume_security_style,
                    svm_admin_password: svm_admin_password,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for StorageVirtualMachine {
    type Properties = StorageVirtualMachineProperties;
    const TYPE: &'static str = "AWS::FSx::StorageVirtualMachine";
    fn properties(&self) -> &StorageVirtualMachineProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut StorageVirtualMachineProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for StorageVirtualMachine {}

impl From<StorageVirtualMachineProperties> for StorageVirtualMachine {
    fn from(properties: StorageVirtualMachineProperties) -> StorageVirtualMachine {
        StorageVirtualMachine { properties }
    }
}

/// The [`AWS::FSx::Volume`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fsx-volume.html) resource type.
#[derive(Debug, Default)]
pub struct Volume {
    properties: VolumeProperties
}

/// Properties for the `Volume` resource.
#[derive(Debug, Default)]
pub struct VolumeProperties {
    /// Property [`BackupId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fsx-volume.html#cfn-fsx-volume-backupid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub backup_id: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fsx-volume.html#cfn-fsx-volume-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`OntapConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fsx-volume.html#cfn-fsx-volume-ontapconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ontap_configuration: Option<::Value<self::volume::OntapConfiguration>>,
    /// Property [`OpenZFSConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fsx-volume.html#cfn-fsx-volume-openzfsconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub open_zfs_configuration: Option<::Value<self::volume::OpenZFSConfiguration>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fsx-volume.html#cfn-fsx-volume-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`VolumeType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fsx-volume.html#cfn-fsx-volume-volumetype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub volume_type: Option<::Value<String>>,
}

impl ::serde::Serialize for VolumeProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref backup_id) = self.backup_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BackupId", backup_id)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref ontap_configuration) = self.ontap_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OntapConfiguration", ontap_configuration)?;
        }
        if let Some(ref open_zfs_configuration) = self.open_zfs_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OpenZFSConfiguration", open_zfs_configuration)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref volume_type) = self.volume_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeType", volume_type)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for VolumeProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<VolumeProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = VolumeProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type VolumeProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut backup_id: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut ontap_configuration: Option<::Value<self::volume::OntapConfiguration>> = None;
                let mut open_zfs_configuration: Option<::Value<self::volume::OpenZFSConfiguration>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut volume_type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "BackupId" => {
                            backup_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OntapConfiguration" => {
                            ontap_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OpenZFSConfiguration" => {
                            open_zfs_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VolumeType" => {
                            volume_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(VolumeProperties {
                    backup_id: backup_id,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    ontap_configuration: ontap_configuration,
                    open_zfs_configuration: open_zfs_configuration,
                    tags: tags,
                    volume_type: volume_type,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Volume {
    type Properties = VolumeProperties;
    const TYPE: &'static str = "AWS::FSx::Volume";
    fn properties(&self) -> &VolumeProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VolumeProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Volume {}

impl From<VolumeProperties> for Volume {
    fn from(properties: VolumeProperties) -> Volume {
        Volume { properties }
    }
}

pub mod file_system {
    //! Property types for the `FileSystem` resource.

    /// The [`AWS::FSx::FileSystem.AuditLogConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-windowsconfiguration-auditlogconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct AuditLogConfiguration {
        /// Property [`AuditLogDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-windowsconfiguration-auditlogconfiguration.html#cfn-fsx-filesystem-windowsconfiguration-auditlogconfiguration-auditlogdestination).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub audit_log_destination: Option<::Value<String>>,
        /// Property [`FileAccessAuditLogLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-windowsconfiguration-auditlogconfiguration.html#cfn-fsx-filesystem-windowsconfiguration-auditlogconfiguration-fileaccessauditloglevel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub file_access_audit_log_level: ::Value<String>,
        /// Property [`FileShareAccessAuditLogLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-windowsconfiguration-auditlogconfiguration.html#cfn-fsx-filesystem-windowsconfiguration-auditlogconfiguration-fileshareaccessauditloglevel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub file_share_access_audit_log_level: ::Value<String>,
    }

    impl ::codec::SerializeValue for AuditLogConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref audit_log_destination) = self.audit_log_destination {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuditLogDestination", audit_log_destination)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FileAccessAuditLogLevel", &self.file_access_audit_log_level)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FileShareAccessAuditLogLevel", &self.file_share_access_audit_log_level)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AuditLogConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AuditLogConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AuditLogConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AuditLogConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut audit_log_destination: Option<::Value<String>> = None;
                    let mut file_access_audit_log_level: Option<::Value<String>> = None;
                    let mut file_share_access_audit_log_level: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AuditLogDestination" => {
                                audit_log_destination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FileAccessAuditLogLevel" => {
                                file_access_audit_log_level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FileShareAccessAuditLogLevel" => {
                                file_share_access_audit_log_level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AuditLogConfiguration {
                        audit_log_destination: audit_log_destination,
                        file_access_audit_log_level: file_access_audit_log_level.ok_or(::serde::de::Error::missing_field("FileAccessAuditLogLevel"))?,
                        file_share_access_audit_log_level: file_share_access_audit_log_level.ok_or(::serde::de::Error::missing_field("FileShareAccessAuditLogLevel"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::FSx::FileSystem.ClientConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-openzfsconfiguration-rootvolumeconfiguration-nfsexports-clientconfigurations.html) property type.
    #[derive(Debug, Default)]
    pub struct ClientConfigurations {
        /// Property [`Clients`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-openzfsconfiguration-rootvolumeconfiguration-nfsexports-clientconfigurations.html#cfn-fsx-filesystem-openzfsconfiguration-rootvolumeconfiguration-nfsexports-clientconfigurations-clients).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub clients: Option<::Value<String>>,
        /// Property [`Options`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-openzfsconfiguration-rootvolumeconfiguration-nfsexports-clientconfigurations.html#cfn-fsx-filesystem-openzfsconfiguration-rootvolumeconfiguration-nfsexports-clientconfigurations-options).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub options: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for ClientConfigurations {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref clients) = self.clients {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Clients", clients)?;
            }
            if let Some(ref options) = self.options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Options", options)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ClientConfigurations {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ClientConfigurations, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ClientConfigurations;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ClientConfigurations")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut clients: Option<::Value<String>> = None;
                    let mut options: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Clients" => {
                                clients = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Options" => {
                                options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ClientConfigurations {
                        clients: clients,
                        options: options,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::FSx::FileSystem.DiskIopsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-openzfsconfiguration-diskiopsconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct DiskIopsConfiguration {
        /// Property [`Iops`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-openzfsconfiguration-diskiopsconfiguration.html#cfn-fsx-filesystem-openzfsconfiguration-diskiopsconfiguration-iops).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub iops: Option<::Value<u32>>,
        /// Property [`Mode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-openzfsconfiguration-diskiopsconfiguration.html#cfn-fsx-filesystem-openzfsconfiguration-diskiopsconfiguration-mode).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub mode: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DiskIopsConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref iops) = self.iops {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Iops", iops)?;
            }
            if let Some(ref mode) = self.mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Mode", mode)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DiskIopsConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DiskIopsConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DiskIopsConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DiskIopsConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut iops: Option<::Value<u32>> = None;
                    let mut mode: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Iops" => {
                                iops = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Mode" => {
                                mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DiskIopsConfiguration {
                        iops: iops,
                        mode: mode,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::FSx::FileSystem.LustreConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-lustreconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct LustreConfiguration {
        /// Property [`AutoImportPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-lustreconfiguration.html#cfn-fsx-filesystem-lustreconfiguration-autoimportpolicy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub auto_import_policy: Option<::Value<String>>,
        /// Property [`AutomaticBackupRetentionDays`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-lustreconfiguration.html#cfn-fsx-filesystem-lustreconfiguration-automaticbackupretentiondays).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub automatic_backup_retention_days: Option<::Value<u32>>,
        /// Property [`CopyTagsToBackups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-lustreconfiguration.html#cfn-fsx-filesystem-lustreconfiguration-copytagstobackups).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub copy_tags_to_backups: Option<::Value<bool>>,
        /// Property [`DailyAutomaticBackupStartTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-lustreconfiguration.html#cfn-fsx-filesystem-lustreconfiguration-dailyautomaticbackupstarttime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub daily_automatic_backup_start_time: Option<::Value<String>>,
        /// Property [`DataCompressionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-lustreconfiguration.html#cfn-fsx-filesystem-lustreconfiguration-datacompressiontype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_compression_type: Option<::Value<String>>,
        /// Property [`DeploymentType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-lustreconfiguration.html#cfn-fsx-filesystem-lustreconfiguration-deploymenttype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub deployment_type: Option<::Value<String>>,
        /// Property [`DriveCacheType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-lustreconfiguration.html#cfn-fsx-filesystem-lustreconfiguration-drivecachetype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub drive_cache_type: Option<::Value<String>>,
        /// Property [`ExportPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-lustreconfiguration.html#cfn-fsx-filesystem-lustreconfiguration-exportpath).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub export_path: Option<::Value<String>>,
        /// Property [`ImportPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-lustreconfiguration.html#cfn-fsx-filesystem-lustreconfiguration-importpath).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub import_path: Option<::Value<String>>,
        /// Property [`ImportedFileChunkSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-lustreconfiguration.html#cfn-fsx-filesystem-lustreconfiguration-importedfilechunksize).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub imported_file_chunk_size: Option<::Value<u32>>,
        /// Property [`PerUnitStorageThroughput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-lustreconfiguration.html#cfn-fsx-filesystem-lustreconfiguration-perunitstoragethroughput).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub per_unit_storage_throughput: Option<::Value<u32>>,
        /// Property [`WeeklyMaintenanceStartTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-lustreconfiguration.html#cfn-fsx-filesystem-lustreconfiguration-weeklymaintenancestarttime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub weekly_maintenance_start_time: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LustreConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref auto_import_policy) = self.auto_import_policy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoImportPolicy", auto_import_policy)?;
            }
            if let Some(ref automatic_backup_retention_days) = self.automatic_backup_retention_days {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutomaticBackupRetentionDays", automatic_backup_retention_days)?;
            }
            if let Some(ref copy_tags_to_backups) = self.copy_tags_to_backups {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CopyTagsToBackups", copy_tags_to_backups)?;
            }
            if let Some(ref daily_automatic_backup_start_time) = self.daily_automatic_backup_start_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DailyAutomaticBackupStartTime", daily_automatic_backup_start_time)?;
            }
            if let Some(ref data_compression_type) = self.data_compression_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataCompressionType", data_compression_type)?;
            }
            if let Some(ref deployment_type) = self.deployment_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeploymentType", deployment_type)?;
            }
            if let Some(ref drive_cache_type) = self.drive_cache_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DriveCacheType", drive_cache_type)?;
            }
            if let Some(ref export_path) = self.export_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExportPath", export_path)?;
            }
            if let Some(ref import_path) = self.import_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImportPath", import_path)?;
            }
            if let Some(ref imported_file_chunk_size) = self.imported_file_chunk_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImportedFileChunkSize", imported_file_chunk_size)?;
            }
            if let Some(ref per_unit_storage_throughput) = self.per_unit_storage_throughput {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PerUnitStorageThroughput", per_unit_storage_throughput)?;
            }
            if let Some(ref weekly_maintenance_start_time) = self.weekly_maintenance_start_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WeeklyMaintenanceStartTime", weekly_maintenance_start_time)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LustreConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LustreConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LustreConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LustreConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut auto_import_policy: Option<::Value<String>> = None;
                    let mut automatic_backup_retention_days: Option<::Value<u32>> = None;
                    let mut copy_tags_to_backups: Option<::Value<bool>> = None;
                    let mut daily_automatic_backup_start_time: Option<::Value<String>> = None;
                    let mut data_compression_type: Option<::Value<String>> = None;
                    let mut deployment_type: Option<::Value<String>> = None;
                    let mut drive_cache_type: Option<::Value<String>> = None;
                    let mut export_path: Option<::Value<String>> = None;
                    let mut import_path: Option<::Value<String>> = None;
                    let mut imported_file_chunk_size: Option<::Value<u32>> = None;
                    let mut per_unit_storage_throughput: Option<::Value<u32>> = None;
                    let mut weekly_maintenance_start_time: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AutoImportPolicy" => {
                                auto_import_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AutomaticBackupRetentionDays" => {
                                automatic_backup_retention_days = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CopyTagsToBackups" => {
                                copy_tags_to_backups = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DailyAutomaticBackupStartTime" => {
                                daily_automatic_backup_start_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataCompressionType" => {
                                data_compression_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeploymentType" => {
                                deployment_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DriveCacheType" => {
                                drive_cache_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExportPath" => {
                                export_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ImportPath" => {
                                import_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ImportedFileChunkSize" => {
                                imported_file_chunk_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PerUnitStorageThroughput" => {
                                per_unit_storage_throughput = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WeeklyMaintenanceStartTime" => {
                                weekly_maintenance_start_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LustreConfiguration {
                        auto_import_policy: auto_import_policy,
                        automatic_backup_retention_days: automatic_backup_retention_days,
                        copy_tags_to_backups: copy_tags_to_backups,
                        daily_automatic_backup_start_time: daily_automatic_backup_start_time,
                        data_compression_type: data_compression_type,
                        deployment_type: deployment_type,
                        drive_cache_type: drive_cache_type,
                        export_path: export_path,
                        import_path: import_path,
                        imported_file_chunk_size: imported_file_chunk_size,
                        per_unit_storage_throughput: per_unit_storage_throughput,
                        weekly_maintenance_start_time: weekly_maintenance_start_time,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::FSx::FileSystem.NfsExports`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-openzfsconfiguration-rootvolumeconfiguration-nfsexports.html) property type.
    #[derive(Debug, Default)]
    pub struct NfsExports {
        /// Property [`ClientConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-openzfsconfiguration-rootvolumeconfiguration-nfsexports.html#cfn-fsx-filesystem-openzfsconfiguration-rootvolumeconfiguration-nfsexports-clientconfigurations).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub client_configurations: Option<::ValueList<ClientConfigurations>>,
    }

    impl ::codec::SerializeValue for NfsExports {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref client_configurations) = self.client_configurations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientConfigurations", client_configurations)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NfsExports {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NfsExports, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NfsExports;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NfsExports")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut client_configurations: Option<::ValueList<ClientConfigurations>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ClientConfigurations" => {
                                client_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NfsExports {
                        client_configurations: client_configurations,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::FSx::FileSystem.OntapConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-ontapconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct OntapConfiguration {
        /// Property [`AutomaticBackupRetentionDays`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-ontapconfiguration.html#cfn-fsx-filesystem-ontapconfiguration-automaticbackupretentiondays).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub automatic_backup_retention_days: Option<::Value<u32>>,
        /// Property [`DailyAutomaticBackupStartTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-ontapconfiguration.html#cfn-fsx-filesystem-ontapconfiguration-dailyautomaticbackupstarttime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub daily_automatic_backup_start_time: Option<::Value<String>>,
        /// Property [`DeploymentType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-ontapconfiguration.html#cfn-fsx-filesystem-ontapconfiguration-deploymenttype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub deployment_type: ::Value<String>,
        /// Property [`DiskIopsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-ontapconfiguration.html#cfn-fsx-filesystem-ontapconfiguration-diskiopsconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub disk_iops_configuration: Option<::Value<DiskIopsConfiguration>>,
        /// Property [`EndpointIpAddressRange`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-ontapconfiguration.html#cfn-fsx-filesystem-ontapconfiguration-endpointipaddressrange).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub endpoint_ip_address_range: Option<::Value<String>>,
        /// Property [`FsxAdminPassword`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-ontapconfiguration.html#cfn-fsx-filesystem-ontapconfiguration-fsxadminpassword).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub fsx_admin_password: Option<::Value<String>>,
        /// Property [`PreferredSubnetId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-ontapconfiguration.html#cfn-fsx-filesystem-ontapconfiguration-preferredsubnetid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub preferred_subnet_id: Option<::Value<String>>,
        /// Property [`RouteTableIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-ontapconfiguration.html#cfn-fsx-filesystem-ontapconfiguration-routetableids).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub route_table_ids: Option<::ValueList<String>>,
        /// Property [`ThroughputCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-ontapconfiguration.html#cfn-fsx-filesystem-ontapconfiguration-throughputcapacity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub throughput_capacity: Option<::Value<u32>>,
        /// Property [`WeeklyMaintenanceStartTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-ontapconfiguration.html#cfn-fsx-filesystem-ontapconfiguration-weeklymaintenancestarttime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub weekly_maintenance_start_time: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for OntapConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref automatic_backup_retention_days) = self.automatic_backup_retention_days {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutomaticBackupRetentionDays", automatic_backup_retention_days)?;
            }
            if let Some(ref daily_automatic_backup_start_time) = self.daily_automatic_backup_start_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DailyAutomaticBackupStartTime", daily_automatic_backup_start_time)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeploymentType", &self.deployment_type)?;
            if let Some(ref disk_iops_configuration) = self.disk_iops_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DiskIopsConfiguration", disk_iops_configuration)?;
            }
            if let Some(ref endpoint_ip_address_range) = self.endpoint_ip_address_range {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointIpAddressRange", endpoint_ip_address_range)?;
            }
            if let Some(ref fsx_admin_password) = self.fsx_admin_password {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FsxAdminPassword", fsx_admin_password)?;
            }
            if let Some(ref preferred_subnet_id) = self.preferred_subnet_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredSubnetId", preferred_subnet_id)?;
            }
            if let Some(ref route_table_ids) = self.route_table_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RouteTableIds", route_table_ids)?;
            }
            if let Some(ref throughput_capacity) = self.throughput_capacity {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThroughputCapacity", throughput_capacity)?;
            }
            if let Some(ref weekly_maintenance_start_time) = self.weekly_maintenance_start_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WeeklyMaintenanceStartTime", weekly_maintenance_start_time)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OntapConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OntapConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OntapConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OntapConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut automatic_backup_retention_days: Option<::Value<u32>> = None;
                    let mut daily_automatic_backup_start_time: Option<::Value<String>> = None;
                    let mut deployment_type: Option<::Value<String>> = None;
                    let mut disk_iops_configuration: Option<::Value<DiskIopsConfiguration>> = None;
                    let mut endpoint_ip_address_range: Option<::Value<String>> = None;
                    let mut fsx_admin_password: Option<::Value<String>> = None;
                    let mut preferred_subnet_id: Option<::Value<String>> = None;
                    let mut route_table_ids: Option<::ValueList<String>> = None;
                    let mut throughput_capacity: Option<::Value<u32>> = None;
                    let mut weekly_maintenance_start_time: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AutomaticBackupRetentionDays" => {
                                automatic_backup_retention_days = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DailyAutomaticBackupStartTime" => {
                                daily_automatic_backup_start_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeploymentType" => {
                                deployment_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DiskIopsConfiguration" => {
                                disk_iops_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EndpointIpAddressRange" => {
                                endpoint_ip_address_range = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FsxAdminPassword" => {
                                fsx_admin_password = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PreferredSubnetId" => {
                                preferred_subnet_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RouteTableIds" => {
                                route_table_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ThroughputCapacity" => {
                                throughput_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WeeklyMaintenanceStartTime" => {
                                weekly_maintenance_start_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OntapConfiguration {
                        automatic_backup_retention_days: automatic_backup_retention_days,
                        daily_automatic_backup_start_time: daily_automatic_backup_start_time,
                        deployment_type: deployment_type.ok_or(::serde::de::Error::missing_field("DeploymentType"))?,
                        disk_iops_configuration: disk_iops_configuration,
                        endpoint_ip_address_range: endpoint_ip_address_range,
                        fsx_admin_password: fsx_admin_password,
                        preferred_subnet_id: preferred_subnet_id,
                        route_table_ids: route_table_ids,
                        throughput_capacity: throughput_capacity,
                        weekly_maintenance_start_time: weekly_maintenance_start_time,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::FSx::FileSystem.OpenZFSConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-openzfsconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct OpenZFSConfiguration {
        /// Property [`AutomaticBackupRetentionDays`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-openzfsconfiguration.html#cfn-fsx-filesystem-openzfsconfiguration-automaticbackupretentiondays).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub automatic_backup_retention_days: Option<::Value<u32>>,
        /// Property [`CopyTagsToBackups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-openzfsconfiguration.html#cfn-fsx-filesystem-openzfsconfiguration-copytagstobackups).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub copy_tags_to_backups: Option<::Value<bool>>,
        /// Property [`CopyTagsToVolumes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-openzfsconfiguration.html#cfn-fsx-filesystem-openzfsconfiguration-copytagstovolumes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub copy_tags_to_volumes: Option<::Value<bool>>,
        /// Property [`DailyAutomaticBackupStartTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-openzfsconfiguration.html#cfn-fsx-filesystem-openzfsconfiguration-dailyautomaticbackupstarttime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub daily_automatic_backup_start_time: Option<::Value<String>>,
        /// Property [`DeploymentType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-openzfsconfiguration.html#cfn-fsx-filesystem-openzfsconfiguration-deploymenttype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub deployment_type: ::Value<String>,
        /// Property [`DiskIopsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-openzfsconfiguration.html#cfn-fsx-filesystem-openzfsconfiguration-diskiopsconfiguration).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub disk_iops_configuration: Option<::Value<DiskIopsConfiguration>>,
        /// Property [`Options`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-openzfsconfiguration.html#cfn-fsx-filesystem-openzfsconfiguration-options).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub options: Option<::ValueList<String>>,
        /// Property [`RootVolumeConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-openzfsconfiguration.html#cfn-fsx-filesystem-openzfsconfiguration-rootvolumeconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub root_volume_configuration: Option<::Value<RootVolumeConfiguration>>,
        /// Property [`ThroughputCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-openzfsconfiguration.html#cfn-fsx-filesystem-openzfsconfiguration-throughputcapacity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub throughput_capacity: Option<::Value<u32>>,
        /// Property [`WeeklyMaintenanceStartTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-openzfsconfiguration.html#cfn-fsx-filesystem-openzfsconfiguration-weeklymaintenancestarttime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub weekly_maintenance_start_time: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for OpenZFSConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref automatic_backup_retention_days) = self.automatic_backup_retention_days {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutomaticBackupRetentionDays", automatic_backup_retention_days)?;
            }
            if let Some(ref copy_tags_to_backups) = self.copy_tags_to_backups {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CopyTagsToBackups", copy_tags_to_backups)?;
            }
            if let Some(ref copy_tags_to_volumes) = self.copy_tags_to_volumes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CopyTagsToVolumes", copy_tags_to_volumes)?;
            }
            if let Some(ref daily_automatic_backup_start_time) = self.daily_automatic_backup_start_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DailyAutomaticBackupStartTime", daily_automatic_backup_start_time)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeploymentType", &self.deployment_type)?;
            if let Some(ref disk_iops_configuration) = self.disk_iops_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DiskIopsConfiguration", disk_iops_configuration)?;
            }
            if let Some(ref options) = self.options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Options", options)?;
            }
            if let Some(ref root_volume_configuration) = self.root_volume_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RootVolumeConfiguration", root_volume_configuration)?;
            }
            if let Some(ref throughput_capacity) = self.throughput_capacity {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThroughputCapacity", throughput_capacity)?;
            }
            if let Some(ref weekly_maintenance_start_time) = self.weekly_maintenance_start_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WeeklyMaintenanceStartTime", weekly_maintenance_start_time)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OpenZFSConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OpenZFSConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OpenZFSConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OpenZFSConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut automatic_backup_retention_days: Option<::Value<u32>> = None;
                    let mut copy_tags_to_backups: Option<::Value<bool>> = None;
                    let mut copy_tags_to_volumes: Option<::Value<bool>> = None;
                    let mut daily_automatic_backup_start_time: Option<::Value<String>> = None;
                    let mut deployment_type: Option<::Value<String>> = None;
                    let mut disk_iops_configuration: Option<::Value<DiskIopsConfiguration>> = None;
                    let mut options: Option<::ValueList<String>> = None;
                    let mut root_volume_configuration: Option<::Value<RootVolumeConfiguration>> = None;
                    let mut throughput_capacity: Option<::Value<u32>> = None;
                    let mut weekly_maintenance_start_time: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AutomaticBackupRetentionDays" => {
                                automatic_backup_retention_days = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CopyTagsToBackups" => {
                                copy_tags_to_backups = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CopyTagsToVolumes" => {
                                copy_tags_to_volumes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DailyAutomaticBackupStartTime" => {
                                daily_automatic_backup_start_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeploymentType" => {
                                deployment_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DiskIopsConfiguration" => {
                                disk_iops_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Options" => {
                                options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RootVolumeConfiguration" => {
                                root_volume_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ThroughputCapacity" => {
                                throughput_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WeeklyMaintenanceStartTime" => {
                                weekly_maintenance_start_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OpenZFSConfiguration {
                        automatic_backup_retention_days: automatic_backup_retention_days,
                        copy_tags_to_backups: copy_tags_to_backups,
                        copy_tags_to_volumes: copy_tags_to_volumes,
                        daily_automatic_backup_start_time: daily_automatic_backup_start_time,
                        deployment_type: deployment_type.ok_or(::serde::de::Error::missing_field("DeploymentType"))?,
                        disk_iops_configuration: disk_iops_configuration,
                        options: options,
                        root_volume_configuration: root_volume_configuration,
                        throughput_capacity: throughput_capacity,
                        weekly_maintenance_start_time: weekly_maintenance_start_time,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::FSx::FileSystem.RootVolumeConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-openzfsconfiguration-rootvolumeconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct RootVolumeConfiguration {
        /// Property [`CopyTagsToSnapshots`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-openzfsconfiguration-rootvolumeconfiguration.html#cfn-fsx-filesystem-openzfsconfiguration-rootvolumeconfiguration-copytagstosnapshots).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub copy_tags_to_snapshots: Option<::Value<bool>>,
        /// Property [`DataCompressionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-openzfsconfiguration-rootvolumeconfiguration.html#cfn-fsx-filesystem-openzfsconfiguration-rootvolumeconfiguration-datacompressiontype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub data_compression_type: Option<::Value<String>>,
        /// Property [`NfsExports`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-openzfsconfiguration-rootvolumeconfiguration.html#cfn-fsx-filesystem-openzfsconfiguration-rootvolumeconfiguration-nfsexports).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub nfs_exports: Option<::ValueList<NfsExports>>,
        /// Property [`ReadOnly`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-openzfsconfiguration-rootvolumeconfiguration.html#cfn-fsx-filesystem-openzfsconfiguration-rootvolumeconfiguration-readonly).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub read_only: Option<::Value<bool>>,
        /// Property [`RecordSizeKiB`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-openzfsconfiguration-rootvolumeconfiguration.html#cfn-fsx-filesystem-openzfsconfiguration-rootvolumeconfiguration-recordsizekib).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub record_size_ki_b: Option<::Value<u32>>,
        /// Property [`UserAndGroupQuotas`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-openzfsconfiguration-rootvolumeconfiguration.html#cfn-fsx-filesystem-openzfsconfiguration-rootvolumeconfiguration-userandgroupquotas).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub user_and_group_quotas: Option<::ValueList<UserAndGroupQuotas>>,
    }

    impl ::codec::SerializeValue for RootVolumeConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref copy_tags_to_snapshots) = self.copy_tags_to_snapshots {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CopyTagsToSnapshots", copy_tags_to_snapshots)?;
            }
            if let Some(ref data_compression_type) = self.data_compression_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataCompressionType", data_compression_type)?;
            }
            if let Some(ref nfs_exports) = self.nfs_exports {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NfsExports", nfs_exports)?;
            }
            if let Some(ref read_only) = self.read_only {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReadOnly", read_only)?;
            }
            if let Some(ref record_size_ki_b) = self.record_size_ki_b {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecordSizeKiB", record_size_ki_b)?;
            }
            if let Some(ref user_and_group_quotas) = self.user_and_group_quotas {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserAndGroupQuotas", user_and_group_quotas)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RootVolumeConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RootVolumeConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RootVolumeConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RootVolumeConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut copy_tags_to_snapshots: Option<::Value<bool>> = None;
                    let mut data_compression_type: Option<::Value<String>> = None;
                    let mut nfs_exports: Option<::ValueList<NfsExports>> = None;
                    let mut read_only: Option<::Value<bool>> = None;
                    let mut record_size_ki_b: Option<::Value<u32>> = None;
                    let mut user_and_group_quotas: Option<::ValueList<UserAndGroupQuotas>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CopyTagsToSnapshots" => {
                                copy_tags_to_snapshots = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataCompressionType" => {
                                data_compression_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NfsExports" => {
                                nfs_exports = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReadOnly" => {
                                read_only = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RecordSizeKiB" => {
                                record_size_ki_b = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserAndGroupQuotas" => {
                                user_and_group_quotas = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RootVolumeConfiguration {
                        copy_tags_to_snapshots: copy_tags_to_snapshots,
                        data_compression_type: data_compression_type,
                        nfs_exports: nfs_exports,
                        read_only: read_only,
                        record_size_ki_b: record_size_ki_b,
                        user_and_group_quotas: user_and_group_quotas,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::FSx::FileSystem.SelfManagedActiveDirectoryConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-windowsconfiguration-selfmanagedactivedirectoryconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct SelfManagedActiveDirectoryConfiguration {
        /// Property [`DnsIps`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-windowsconfiguration-selfmanagedactivedirectoryconfiguration.html#cfn-fsx-filesystem-windowsconfiguration-selfmanagedactivedirectoryconfiguration-dnsips).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dns_ips: Option<::ValueList<String>>,
        /// Property [`DomainName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-windowsconfiguration-selfmanagedactivedirectoryconfiguration.html#cfn-fsx-filesystem-windowsconfiguration-selfmanagedactivedirectoryconfiguration-domainname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub domain_name: Option<::Value<String>>,
        /// Property [`FileSystemAdministratorsGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-windowsconfiguration-selfmanagedactivedirectoryconfiguration.html#cfn-fsx-filesystem-windowsconfiguration-selfmanagedactivedirectoryconfiguration-filesystemadministratorsgroup).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub file_system_administrators_group: Option<::Value<String>>,
        /// Property [`OrganizationalUnitDistinguishedName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-windowsconfiguration-selfmanagedactivedirectoryconfiguration.html#cfn-fsx-filesystem-windowsconfiguration-selfmanagedactivedirectoryconfiguration-organizationalunitdistinguishedname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub organizational_unit_distinguished_name: Option<::Value<String>>,
        /// Property [`Password`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-windowsconfiguration-selfmanagedactivedirectoryconfiguration.html#cfn-fsx-filesystem-windowsconfiguration-selfmanagedactivedirectoryconfiguration-password).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub password: Option<::Value<String>>,
        /// Property [`UserName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-windowsconfiguration-selfmanagedactivedirectoryconfiguration.html#cfn-fsx-filesystem-windowsconfiguration-selfmanagedactivedirectoryconfiguration-username).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SelfManagedActiveDirectoryConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref dns_ips) = self.dns_ips {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DnsIps", dns_ips)?;
            }
            if let Some(ref domain_name) = self.domain_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainName", domain_name)?;
            }
            if let Some(ref file_system_administrators_group) = self.file_system_administrators_group {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FileSystemAdministratorsGroup", file_system_administrators_group)?;
            }
            if let Some(ref organizational_unit_distinguished_name) = self.organizational_unit_distinguished_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OrganizationalUnitDistinguishedName", organizational_unit_distinguished_name)?;
            }
            if let Some(ref password) = self.password {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Password", password)?;
            }
            if let Some(ref user_name) = self.user_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserName", user_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SelfManagedActiveDirectoryConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SelfManagedActiveDirectoryConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SelfManagedActiveDirectoryConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SelfManagedActiveDirectoryConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dns_ips: Option<::ValueList<String>> = None;
                    let mut domain_name: Option<::Value<String>> = None;
                    let mut file_system_administrators_group: Option<::Value<String>> = None;
                    let mut organizational_unit_distinguished_name: Option<::Value<String>> = None;
                    let mut password: Option<::Value<String>> = None;
                    let mut user_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DnsIps" => {
                                dns_ips = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DomainName" => {
                                domain_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FileSystemAdministratorsGroup" => {
                                file_system_administrators_group = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OrganizationalUnitDistinguishedName" => {
                                organizational_unit_distinguished_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Password" => {
                                password = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserName" => {
                                user_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SelfManagedActiveDirectoryConfiguration {
                        dns_ips: dns_ips,
                        domain_name: domain_name,
                        file_system_administrators_group: file_system_administrators_group,
                        organizational_unit_distinguished_name: organizational_unit_distinguished_name,
                        password: password,
                        user_name: user_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::FSx::FileSystem.UserAndGroupQuotas`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-openzfsconfiguration-rootvolumeconfiguration-userandgroupquotas.html) property type.
    #[derive(Debug, Default)]
    pub struct UserAndGroupQuotas {
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-openzfsconfiguration-rootvolumeconfiguration-userandgroupquotas.html#cfn-fsx-filesystem-openzfsconfiguration-rootvolumeconfiguration-userandgroupquotas-id).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub id: Option<::Value<u32>>,
        /// Property [`StorageCapacityQuotaGiB`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-openzfsconfiguration-rootvolumeconfiguration-userandgroupquotas.html#cfn-fsx-filesystem-openzfsconfiguration-rootvolumeconfiguration-userandgroupquotas-storagecapacityquotagib).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub storage_capacity_quota_gi_b: Option<::Value<u32>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-openzfsconfiguration-rootvolumeconfiguration-userandgroupquotas.html#cfn-fsx-filesystem-openzfsconfiguration-rootvolumeconfiguration-userandgroupquotas-type).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub r#type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for UserAndGroupQuotas {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref id) = self.id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", id)?;
            }
            if let Some(ref storage_capacity_quota_gi_b) = self.storage_capacity_quota_gi_b {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StorageCapacityQuotaGiB", storage_capacity_quota_gi_b)?;
            }
            if let Some(ref r#type) = self.r#type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for UserAndGroupQuotas {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<UserAndGroupQuotas, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = UserAndGroupQuotas;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type UserAndGroupQuotas")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut id: Option<::Value<u32>> = None;
                    let mut storage_capacity_quota_gi_b: Option<::Value<u32>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StorageCapacityQuotaGiB" => {
                                storage_capacity_quota_gi_b = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(UserAndGroupQuotas {
                        id: id,
                        storage_capacity_quota_gi_b: storage_capacity_quota_gi_b,
                        r#type: r#type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::FSx::FileSystem.WindowsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-windowsconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct WindowsConfiguration {
        /// Property [`ActiveDirectoryId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-windowsconfiguration.html#cfn-fsx-filesystem-windowsconfiguration-activedirectoryid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub active_directory_id: Option<::Value<String>>,
        /// Property [`Aliases`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-windowsconfiguration.html#cfn-fsx-filesystem-windowsconfiguration-aliases).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aliases: Option<::ValueList<String>>,
        /// Property [`AuditLogConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-windowsconfiguration.html#cfn-fsx-filesystem-windowsconfiguration-auditlogconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub audit_log_configuration: Option<::Value<AuditLogConfiguration>>,
        /// Property [`AutomaticBackupRetentionDays`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-windowsconfiguration.html#cfn-fsx-filesystem-windowsconfiguration-automaticbackupretentiondays).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub automatic_backup_retention_days: Option<::Value<u32>>,
        /// Property [`CopyTagsToBackups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-windowsconfiguration.html#cfn-fsx-filesystem-windowsconfiguration-copytagstobackups).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub copy_tags_to_backups: Option<::Value<bool>>,
        /// Property [`DailyAutomaticBackupStartTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-windowsconfiguration.html#cfn-fsx-filesystem-windowsconfiguration-dailyautomaticbackupstarttime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub daily_automatic_backup_start_time: Option<::Value<String>>,
        /// Property [`DeploymentType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-windowsconfiguration.html#cfn-fsx-filesystem-windowsconfiguration-deploymenttype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub deployment_type: Option<::Value<String>>,
        /// Property [`PreferredSubnetId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-windowsconfiguration.html#cfn-fsx-filesystem-windowsconfiguration-preferredsubnetid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub preferred_subnet_id: Option<::Value<String>>,
        /// Property [`SelfManagedActiveDirectoryConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-windowsconfiguration.html#cfn-fsx-filesystem-windowsconfiguration-selfmanagedactivedirectoryconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub self_managed_active_directory_configuration: Option<::Value<SelfManagedActiveDirectoryConfiguration>>,
        /// Property [`ThroughputCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-windowsconfiguration.html#cfn-fsx-filesystem-windowsconfiguration-throughputcapacity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub throughput_capacity: ::Value<u32>,
        /// Property [`WeeklyMaintenanceStartTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-windowsconfiguration.html#cfn-fsx-filesystem-windowsconfiguration-weeklymaintenancestarttime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub weekly_maintenance_start_time: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for WindowsConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref active_directory_id) = self.active_directory_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ActiveDirectoryId", active_directory_id)?;
            }
            if let Some(ref aliases) = self.aliases {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Aliases", aliases)?;
            }
            if let Some(ref audit_log_configuration) = self.audit_log_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuditLogConfiguration", audit_log_configuration)?;
            }
            if let Some(ref automatic_backup_retention_days) = self.automatic_backup_retention_days {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutomaticBackupRetentionDays", automatic_backup_retention_days)?;
            }
            if let Some(ref copy_tags_to_backups) = self.copy_tags_to_backups {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CopyTagsToBackups", copy_tags_to_backups)?;
            }
            if let Some(ref daily_automatic_backup_start_time) = self.daily_automatic_backup_start_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DailyAutomaticBackupStartTime", daily_automatic_backup_start_time)?;
            }
            if let Some(ref deployment_type) = self.deployment_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeploymentType", deployment_type)?;
            }
            if let Some(ref preferred_subnet_id) = self.preferred_subnet_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredSubnetId", preferred_subnet_id)?;
            }
            if let Some(ref self_managed_active_directory_configuration) = self.self_managed_active_directory_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SelfManagedActiveDirectoryConfiguration", self_managed_active_directory_configuration)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThroughputCapacity", &self.throughput_capacity)?;
            if let Some(ref weekly_maintenance_start_time) = self.weekly_maintenance_start_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WeeklyMaintenanceStartTime", weekly_maintenance_start_time)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for WindowsConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<WindowsConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = WindowsConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type WindowsConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut active_directory_id: Option<::Value<String>> = None;
                    let mut aliases: Option<::ValueList<String>> = None;
                    let mut audit_log_configuration: Option<::Value<AuditLogConfiguration>> = None;
                    let mut automatic_backup_retention_days: Option<::Value<u32>> = None;
                    let mut copy_tags_to_backups: Option<::Value<bool>> = None;
                    let mut daily_automatic_backup_start_time: Option<::Value<String>> = None;
                    let mut deployment_type: Option<::Value<String>> = None;
                    let mut preferred_subnet_id: Option<::Value<String>> = None;
                    let mut self_managed_active_directory_configuration: Option<::Value<SelfManagedActiveDirectoryConfiguration>> = None;
                    let mut throughput_capacity: Option<::Value<u32>> = None;
                    let mut weekly_maintenance_start_time: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ActiveDirectoryId" => {
                                active_directory_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Aliases" => {
                                aliases = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AuditLogConfiguration" => {
                                audit_log_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AutomaticBackupRetentionDays" => {
                                automatic_backup_retention_days = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CopyTagsToBackups" => {
                                copy_tags_to_backups = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DailyAutomaticBackupStartTime" => {
                                daily_automatic_backup_start_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeploymentType" => {
                                deployment_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PreferredSubnetId" => {
                                preferred_subnet_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SelfManagedActiveDirectoryConfiguration" => {
                                self_managed_active_directory_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ThroughputCapacity" => {
                                throughput_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WeeklyMaintenanceStartTime" => {
                                weekly_maintenance_start_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(WindowsConfiguration {
                        active_directory_id: active_directory_id,
                        aliases: aliases,
                        audit_log_configuration: audit_log_configuration,
                        automatic_backup_retention_days: automatic_backup_retention_days,
                        copy_tags_to_backups: copy_tags_to_backups,
                        daily_automatic_backup_start_time: daily_automatic_backup_start_time,
                        deployment_type: deployment_type,
                        preferred_subnet_id: preferred_subnet_id,
                        self_managed_active_directory_configuration: self_managed_active_directory_configuration,
                        throughput_capacity: throughput_capacity.ok_or(::serde::de::Error::missing_field("ThroughputCapacity"))?,
                        weekly_maintenance_start_time: weekly_maintenance_start_time,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod storage_virtual_machine {
    //! Property types for the `StorageVirtualMachine` resource.

    /// The [`AWS::FSx::StorageVirtualMachine.ActiveDirectoryConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-storagevirtualmachine-activedirectoryconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ActiveDirectoryConfiguration {
        /// Property [`NetBiosName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-storagevirtualmachine-activedirectoryconfiguration.html#cfn-fsx-storagevirtualmachine-activedirectoryconfiguration-netbiosname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub net_bios_name: Option<::Value<String>>,
        /// Property [`SelfManagedActiveDirectoryConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-storagevirtualmachine-activedirectoryconfiguration.html#cfn-fsx-storagevirtualmachine-activedirectoryconfiguration-selfmanagedactivedirectoryconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub self_managed_active_directory_configuration: Option<::Value<SelfManagedActiveDirectoryConfiguration>>,
    }

    impl ::codec::SerializeValue for ActiveDirectoryConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref net_bios_name) = self.net_bios_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetBiosName", net_bios_name)?;
            }
            if let Some(ref self_managed_active_directory_configuration) = self.self_managed_active_directory_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SelfManagedActiveDirectoryConfiguration", self_managed_active_directory_configuration)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ActiveDirectoryConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ActiveDirectoryConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ActiveDirectoryConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ActiveDirectoryConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut net_bios_name: Option<::Value<String>> = None;
                    let mut self_managed_active_directory_configuration: Option<::Value<SelfManagedActiveDirectoryConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "NetBiosName" => {
                                net_bios_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SelfManagedActiveDirectoryConfiguration" => {
                                self_managed_active_directory_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ActiveDirectoryConfiguration {
                        net_bios_name: net_bios_name,
                        self_managed_active_directory_configuration: self_managed_active_directory_configuration,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::FSx::StorageVirtualMachine.SelfManagedActiveDirectoryConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-storagevirtualmachine-activedirectoryconfiguration-selfmanagedactivedirectoryconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct SelfManagedActiveDirectoryConfiguration {
        /// Property [`DnsIps`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-storagevirtualmachine-activedirectoryconfiguration-selfmanagedactivedirectoryconfiguration.html#cfn-fsx-storagevirtualmachine-activedirectoryconfiguration-selfmanagedactivedirectoryconfiguration-dnsips).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dns_ips: Option<::ValueList<String>>,
        /// Property [`DomainName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-storagevirtualmachine-activedirectoryconfiguration-selfmanagedactivedirectoryconfiguration.html#cfn-fsx-storagevirtualmachine-activedirectoryconfiguration-selfmanagedactivedirectoryconfiguration-domainname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub domain_name: Option<::Value<String>>,
        /// Property [`FileSystemAdministratorsGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-storagevirtualmachine-activedirectoryconfiguration-selfmanagedactivedirectoryconfiguration.html#cfn-fsx-storagevirtualmachine-activedirectoryconfiguration-selfmanagedactivedirectoryconfiguration-filesystemadministratorsgroup).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub file_system_administrators_group: Option<::Value<String>>,
        /// Property [`OrganizationalUnitDistinguishedName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-storagevirtualmachine-activedirectoryconfiguration-selfmanagedactivedirectoryconfiguration.html#cfn-fsx-storagevirtualmachine-activedirectoryconfiguration-selfmanagedactivedirectoryconfiguration-organizationalunitdistinguishedname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub organizational_unit_distinguished_name: Option<::Value<String>>,
        /// Property [`Password`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-storagevirtualmachine-activedirectoryconfiguration-selfmanagedactivedirectoryconfiguration.html#cfn-fsx-storagevirtualmachine-activedirectoryconfiguration-selfmanagedactivedirectoryconfiguration-password).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub password: Option<::Value<String>>,
        /// Property [`UserName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-storagevirtualmachine-activedirectoryconfiguration-selfmanagedactivedirectoryconfiguration.html#cfn-fsx-storagevirtualmachine-activedirectoryconfiguration-selfmanagedactivedirectoryconfiguration-username).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SelfManagedActiveDirectoryConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref dns_ips) = self.dns_ips {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DnsIps", dns_ips)?;
            }
            if let Some(ref domain_name) = self.domain_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainName", domain_name)?;
            }
            if let Some(ref file_system_administrators_group) = self.file_system_administrators_group {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FileSystemAdministratorsGroup", file_system_administrators_group)?;
            }
            if let Some(ref organizational_unit_distinguished_name) = self.organizational_unit_distinguished_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OrganizationalUnitDistinguishedName", organizational_unit_distinguished_name)?;
            }
            if let Some(ref password) = self.password {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Password", password)?;
            }
            if let Some(ref user_name) = self.user_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserName", user_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SelfManagedActiveDirectoryConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SelfManagedActiveDirectoryConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SelfManagedActiveDirectoryConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SelfManagedActiveDirectoryConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dns_ips: Option<::ValueList<String>> = None;
                    let mut domain_name: Option<::Value<String>> = None;
                    let mut file_system_administrators_group: Option<::Value<String>> = None;
                    let mut organizational_unit_distinguished_name: Option<::Value<String>> = None;
                    let mut password: Option<::Value<String>> = None;
                    let mut user_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DnsIps" => {
                                dns_ips = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DomainName" => {
                                domain_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FileSystemAdministratorsGroup" => {
                                file_system_administrators_group = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OrganizationalUnitDistinguishedName" => {
                                organizational_unit_distinguished_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Password" => {
                                password = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserName" => {
                                user_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SelfManagedActiveDirectoryConfiguration {
                        dns_ips: dns_ips,
                        domain_name: domain_name,
                        file_system_administrators_group: file_system_administrators_group,
                        organizational_unit_distinguished_name: organizational_unit_distinguished_name,
                        password: password,
                        user_name: user_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod volume {
    //! Property types for the `Volume` resource.

    /// The [`AWS::FSx::Volume.ClientConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-openzfsconfiguration-nfsexports-clientconfigurations.html) property type.
    #[derive(Debug, Default)]
    pub struct ClientConfigurations {
        /// Property [`Clients`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-openzfsconfiguration-nfsexports-clientconfigurations.html#cfn-fsx-volume-openzfsconfiguration-nfsexports-clientconfigurations-clients).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub clients: ::Value<String>,
        /// Property [`Options`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-openzfsconfiguration-nfsexports-clientconfigurations.html#cfn-fsx-volume-openzfsconfiguration-nfsexports-clientconfigurations-options).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub options: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for ClientConfigurations {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Clients", &self.clients)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Options", &self.options)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ClientConfigurations {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ClientConfigurations, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ClientConfigurations;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ClientConfigurations")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut clients: Option<::Value<String>> = None;
                    let mut options: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Clients" => {
                                clients = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Options" => {
                                options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ClientConfigurations {
                        clients: clients.ok_or(::serde::de::Error::missing_field("Clients"))?,
                        options: options.ok_or(::serde::de::Error::missing_field("Options"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::FSx::Volume.NfsExports`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-openzfsconfiguration-nfsexports.html) property type.
    #[derive(Debug, Default)]
    pub struct NfsExports {
        /// Property [`ClientConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-openzfsconfiguration-nfsexports.html#cfn-fsx-volume-openzfsconfiguration-nfsexports-clientconfigurations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_configurations: ::ValueList<ClientConfigurations>,
    }

    impl ::codec::SerializeValue for NfsExports {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientConfigurations", &self.client_configurations)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NfsExports {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NfsExports, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NfsExports;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NfsExports")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut client_configurations: Option<::ValueList<ClientConfigurations>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ClientConfigurations" => {
                                client_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NfsExports {
                        client_configurations: client_configurations.ok_or(::serde::de::Error::missing_field("ClientConfigurations"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::FSx::Volume.OntapConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-ontapconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct OntapConfiguration {
        /// Property [`JunctionPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-ontapconfiguration.html#cfn-fsx-volume-ontapconfiguration-junctionpath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub junction_path: ::Value<String>,
        /// Property [`SecurityStyle`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-ontapconfiguration.html#cfn-fsx-volume-ontapconfiguration-securitystyle).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_style: Option<::Value<String>>,
        /// Property [`SizeInMegabytes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-ontapconfiguration.html#cfn-fsx-volume-ontapconfiguration-sizeinmegabytes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub size_in_megabytes: ::Value<String>,
        /// Property [`StorageEfficiencyEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-ontapconfiguration.html#cfn-fsx-volume-ontapconfiguration-storageefficiencyenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub storage_efficiency_enabled: ::Value<String>,
        /// Property [`StorageVirtualMachineId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-ontapconfiguration.html#cfn-fsx-volume-ontapconfiguration-storagevirtualmachineid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub storage_virtual_machine_id: ::Value<String>,
        /// Property [`TieringPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-ontapconfiguration.html#cfn-fsx-volume-ontapconfiguration-tieringpolicy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tiering_policy: Option<::Value<TieringPolicy>>,
    }

    impl ::codec::SerializeValue for OntapConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "JunctionPath", &self.junction_path)?;
            if let Some(ref security_style) = self.security_style {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityStyle", security_style)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SizeInMegabytes", &self.size_in_megabytes)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StorageEfficiencyEnabled", &self.storage_efficiency_enabled)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StorageVirtualMachineId", &self.storage_virtual_machine_id)?;
            if let Some(ref tiering_policy) = self.tiering_policy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TieringPolicy", tiering_policy)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OntapConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OntapConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OntapConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OntapConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut junction_path: Option<::Value<String>> = None;
                    let mut security_style: Option<::Value<String>> = None;
                    let mut size_in_megabytes: Option<::Value<String>> = None;
                    let mut storage_efficiency_enabled: Option<::Value<String>> = None;
                    let mut storage_virtual_machine_id: Option<::Value<String>> = None;
                    let mut tiering_policy: Option<::Value<TieringPolicy>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "JunctionPath" => {
                                junction_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecurityStyle" => {
                                security_style = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SizeInMegabytes" => {
                                size_in_megabytes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StorageEfficiencyEnabled" => {
                                storage_efficiency_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StorageVirtualMachineId" => {
                                storage_virtual_machine_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TieringPolicy" => {
                                tiering_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OntapConfiguration {
                        junction_path: junction_path.ok_or(::serde::de::Error::missing_field("JunctionPath"))?,
                        security_style: security_style,
                        size_in_megabytes: size_in_megabytes.ok_or(::serde::de::Error::missing_field("SizeInMegabytes"))?,
                        storage_efficiency_enabled: storage_efficiency_enabled.ok_or(::serde::de::Error::missing_field("StorageEfficiencyEnabled"))?,
                        storage_virtual_machine_id: storage_virtual_machine_id.ok_or(::serde::de::Error::missing_field("StorageVirtualMachineId"))?,
                        tiering_policy: tiering_policy,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::FSx::Volume.OpenZFSConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-openzfsconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct OpenZFSConfiguration {
        /// Property [`CopyTagsToSnapshots`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-openzfsconfiguration.html#cfn-fsx-volume-openzfsconfiguration-copytagstosnapshots).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub copy_tags_to_snapshots: Option<::Value<bool>>,
        /// Property [`DataCompressionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-openzfsconfiguration.html#cfn-fsx-volume-openzfsconfiguration-datacompressiontype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_compression_type: Option<::Value<String>>,
        /// Property [`NfsExports`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-openzfsconfiguration.html#cfn-fsx-volume-openzfsconfiguration-nfsexports).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub nfs_exports: Option<::ValueList<NfsExports>>,
        /// Property [`Options`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-openzfsconfiguration.html#cfn-fsx-volume-openzfsconfiguration-options).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub options: Option<::ValueList<String>>,
        /// Property [`OriginSnapshot`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-openzfsconfiguration.html#cfn-fsx-volume-openzfsconfiguration-originsnapshot).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub origin_snapshot: Option<::Value<OriginSnapshot>>,
        /// Property [`ParentVolumeId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-openzfsconfiguration.html#cfn-fsx-volume-openzfsconfiguration-parentvolumeid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub parent_volume_id: ::Value<String>,
        /// Property [`ReadOnly`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-openzfsconfiguration.html#cfn-fsx-volume-openzfsconfiguration-readonly).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub read_only: Option<::Value<bool>>,
        /// Property [`RecordSizeKiB`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-openzfsconfiguration.html#cfn-fsx-volume-openzfsconfiguration-recordsizekib).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub record_size_ki_b: Option<::Value<u32>>,
        /// Property [`StorageCapacityQuotaGiB`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-openzfsconfiguration.html#cfn-fsx-volume-openzfsconfiguration-storagecapacityquotagib).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub storage_capacity_quota_gi_b: Option<::Value<u32>>,
        /// Property [`StorageCapacityReservationGiB`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-openzfsconfiguration.html#cfn-fsx-volume-openzfsconfiguration-storagecapacityreservationgib).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub storage_capacity_reservation_gi_b: Option<::Value<u32>>,
        /// Property [`UserAndGroupQuotas`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-openzfsconfiguration.html#cfn-fsx-volume-openzfsconfiguration-userandgroupquotas).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_and_group_quotas: Option<::ValueList<UserAndGroupQuotas>>,
    }

    impl ::codec::SerializeValue for OpenZFSConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref copy_tags_to_snapshots) = self.copy_tags_to_snapshots {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CopyTagsToSnapshots", copy_tags_to_snapshots)?;
            }
            if let Some(ref data_compression_type) = self.data_compression_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataCompressionType", data_compression_type)?;
            }
            if let Some(ref nfs_exports) = self.nfs_exports {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NfsExports", nfs_exports)?;
            }
            if let Some(ref options) = self.options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Options", options)?;
            }
            if let Some(ref origin_snapshot) = self.origin_snapshot {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OriginSnapshot", origin_snapshot)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParentVolumeId", &self.parent_volume_id)?;
            if let Some(ref read_only) = self.read_only {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReadOnly", read_only)?;
            }
            if let Some(ref record_size_ki_b) = self.record_size_ki_b {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecordSizeKiB", record_size_ki_b)?;
            }
            if let Some(ref storage_capacity_quota_gi_b) = self.storage_capacity_quota_gi_b {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StorageCapacityQuotaGiB", storage_capacity_quota_gi_b)?;
            }
            if let Some(ref storage_capacity_reservation_gi_b) = self.storage_capacity_reservation_gi_b {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StorageCapacityReservationGiB", storage_capacity_reservation_gi_b)?;
            }
            if let Some(ref user_and_group_quotas) = self.user_and_group_quotas {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserAndGroupQuotas", user_and_group_quotas)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OpenZFSConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OpenZFSConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OpenZFSConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OpenZFSConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut copy_tags_to_snapshots: Option<::Value<bool>> = None;
                    let mut data_compression_type: Option<::Value<String>> = None;
                    let mut nfs_exports: Option<::ValueList<NfsExports>> = None;
                    let mut options: Option<::ValueList<String>> = None;
                    let mut origin_snapshot: Option<::Value<OriginSnapshot>> = None;
                    let mut parent_volume_id: Option<::Value<String>> = None;
                    let mut read_only: Option<::Value<bool>> = None;
                    let mut record_size_ki_b: Option<::Value<u32>> = None;
                    let mut storage_capacity_quota_gi_b: Option<::Value<u32>> = None;
                    let mut storage_capacity_reservation_gi_b: Option<::Value<u32>> = None;
                    let mut user_and_group_quotas: Option<::ValueList<UserAndGroupQuotas>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CopyTagsToSnapshots" => {
                                copy_tags_to_snapshots = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataCompressionType" => {
                                data_compression_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NfsExports" => {
                                nfs_exports = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Options" => {
                                options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OriginSnapshot" => {
                                origin_snapshot = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ParentVolumeId" => {
                                parent_volume_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReadOnly" => {
                                read_only = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RecordSizeKiB" => {
                                record_size_ki_b = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StorageCapacityQuotaGiB" => {
                                storage_capacity_quota_gi_b = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StorageCapacityReservationGiB" => {
                                storage_capacity_reservation_gi_b = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserAndGroupQuotas" => {
                                user_and_group_quotas = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OpenZFSConfiguration {
                        copy_tags_to_snapshots: copy_tags_to_snapshots,
                        data_compression_type: data_compression_type,
                        nfs_exports: nfs_exports,
                        options: options,
                        origin_snapshot: origin_snapshot,
                        parent_volume_id: parent_volume_id.ok_or(::serde::de::Error::missing_field("ParentVolumeId"))?,
                        read_only: read_only,
                        record_size_ki_b: record_size_ki_b,
                        storage_capacity_quota_gi_b: storage_capacity_quota_gi_b,
                        storage_capacity_reservation_gi_b: storage_capacity_reservation_gi_b,
                        user_and_group_quotas: user_and_group_quotas,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::FSx::Volume.OriginSnapshot`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-openzfsconfiguration-originsnapshot.html) property type.
    #[derive(Debug, Default)]
    pub struct OriginSnapshot {
        /// Property [`CopyStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-openzfsconfiguration-originsnapshot.html#cfn-fsx-volume-openzfsconfiguration-originsnapshot-copystrategy).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub copy_strategy: ::Value<String>,
        /// Property [`SnapshotARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-openzfsconfiguration-originsnapshot.html#cfn-fsx-volume-openzfsconfiguration-originsnapshot-snapshotarn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub snapshot_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for OriginSnapshot {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CopyStrategy", &self.copy_strategy)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotARN", &self.snapshot_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OriginSnapshot {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OriginSnapshot, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OriginSnapshot;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OriginSnapshot")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut copy_strategy: Option<::Value<String>> = None;
                    let mut snapshot_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CopyStrategy" => {
                                copy_strategy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SnapshotARN" => {
                                snapshot_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OriginSnapshot {
                        copy_strategy: copy_strategy.ok_or(::serde::de::Error::missing_field("CopyStrategy"))?,
                        snapshot_arn: snapshot_arn.ok_or(::serde::de::Error::missing_field("SnapshotARN"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::FSx::Volume.TieringPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-ontapconfiguration-tieringpolicy.html) property type.
    #[derive(Debug, Default)]
    pub struct TieringPolicy {
        /// Property [`CoolingPeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-ontapconfiguration-tieringpolicy.html#cfn-fsx-volume-ontapconfiguration-tieringpolicy-coolingperiod).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cooling_period: Option<::Value<u32>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-ontapconfiguration-tieringpolicy.html#cfn-fsx-volume-ontapconfiguration-tieringpolicy-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for TieringPolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cooling_period) = self.cooling_period {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CoolingPeriod", cooling_period)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TieringPolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TieringPolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TieringPolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TieringPolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cooling_period: Option<::Value<u32>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CoolingPeriod" => {
                                cooling_period = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TieringPolicy {
                        cooling_period: cooling_period,
                        name: name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::FSx::Volume.UserAndGroupQuotas`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-openzfsconfiguration-userandgroupquotas.html) property type.
    #[derive(Debug, Default)]
    pub struct UserAndGroupQuotas {
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-openzfsconfiguration-userandgroupquotas.html#cfn-fsx-volume-openzfsconfiguration-userandgroupquotas-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: ::Value<u32>,
        /// Property [`StorageCapacityQuotaGiB`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-openzfsconfiguration-userandgroupquotas.html#cfn-fsx-volume-openzfsconfiguration-userandgroupquotas-storagecapacityquotagib).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub storage_capacity_quota_gi_b: ::Value<u32>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-openzfsconfiguration-userandgroupquotas.html#cfn-fsx-volume-openzfsconfiguration-userandgroupquotas-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for UserAndGroupQuotas {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StorageCapacityQuotaGiB", &self.storage_capacity_quota_gi_b)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for UserAndGroupQuotas {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<UserAndGroupQuotas, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = UserAndGroupQuotas;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type UserAndGroupQuotas")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut id: Option<::Value<u32>> = None;
                    let mut storage_capacity_quota_gi_b: Option<::Value<u32>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StorageCapacityQuotaGiB" => {
                                storage_capacity_quota_gi_b = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(UserAndGroupQuotas {
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                        storage_capacity_quota_gi_b: storage_capacity_quota_gi_b.ok_or(::serde::de::Error::missing_field("StorageCapacityQuotaGiB"))?,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
