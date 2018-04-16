//! Types for the `EFS` service.

/// The [`AWS::EFS::FileSystem`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-efs-filesystem.html) resource type.
#[derive(Debug, Default)]
pub struct FileSystem {
    properties: FileSystemProperties
}

/// Properties for the `FileSystem` resource.
#[derive(Debug, Default)]
pub struct FileSystemProperties {
    /// Property [`Encrypted`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-efs-filesystem.html#cfn-efs-filesystem-encrypted).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub encrypted: Option<::Value<bool>>,
    /// Property [`FileSystemTags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-efs-filesystem.html#cfn-efs-filesystem-filesystemtags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub file_system_tags: Option<::ValueList<self::file_system::ElasticFileSystemTag>>,
    /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-efs-filesystem.html#cfn-efs-filesystem-kmskeyid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kms_key_id: Option<::Value<String>>,
    /// Property [`PerformanceMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-efs-filesystem.html#cfn-efs-filesystem-performancemode).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub performance_mode: Option<::Value<String>>,
}

impl ::serde::Serialize for FileSystemProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref encrypted) = self.encrypted {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Encrypted", encrypted)?;
        }
        if let Some(ref file_system_tags) = self.file_system_tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FileSystemTags", file_system_tags)?;
        }
        if let Some(ref kms_key_id) = self.kms_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
        }
        if let Some(ref performance_mode) = self.performance_mode {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PerformanceMode", performance_mode)?;
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
                let mut encrypted: Option<::Value<bool>> = None;
                let mut file_system_tags: Option<::ValueList<self::file_system::ElasticFileSystemTag>> = None;
                let mut kms_key_id: Option<::Value<String>> = None;
                let mut performance_mode: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Encrypted" => {
                            encrypted = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FileSystemTags" => {
                            file_system_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsKeyId" => {
                            kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PerformanceMode" => {
                            performance_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(FileSystemProperties {
                    encrypted: encrypted,
                    file_system_tags: file_system_tags,
                    kms_key_id: kms_key_id,
                    performance_mode: performance_mode,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for FileSystem {
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
#[derive(Debug, Default)]
pub struct MountTarget {
    properties: MountTargetProperties
}

/// Properties for the `MountTarget` resource.
#[derive(Debug, Default)]
pub struct MountTargetProperties {
    /// Property [`FileSystemId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-efs-mounttarget.html#cfn-efs-mounttarget-filesystemid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub file_system_id: ::Value<String>,
    /// Property [`IpAddress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-efs-mounttarget.html#cfn-efs-mounttarget-ipaddress).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub ip_address: Option<::Value<String>>,
    /// Property [`SecurityGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-efs-mounttarget.html#cfn-efs-mounttarget-securitygroups).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub security_groups: ::ValueList<String>,
    /// Property [`SubnetId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-efs-mounttarget.html#cfn-efs-mounttarget-subnetid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub subnet_id: ::Value<String>,
}

impl ::serde::Serialize for MountTargetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FileSystemId", &self.file_system_id)?;
        if let Some(ref ip_address) = self.ip_address {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IpAddress", ip_address)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroups", &self.security_groups)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetId", &self.subnet_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for MountTargetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<MountTargetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = MountTargetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type MountTargetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut file_system_id: Option<::Value<String>> = None;
                let mut ip_address: Option<::Value<String>> = None;
                let mut security_groups: Option<::ValueList<String>> = None;
                let mut subnet_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "FileSystemId" => {
                            file_system_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IpAddress" => {
                            ip_address = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecurityGroups" => {
                            security_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SubnetId" => {
                            subnet_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(MountTargetProperties {
                    file_system_id: file_system_id.ok_or(::serde::de::Error::missing_field("FileSystemId"))?,
                    ip_address: ip_address,
                    security_groups: security_groups.ok_or(::serde::de::Error::missing_field("SecurityGroups"))?,
                    subnet_id: subnet_id.ok_or(::serde::de::Error::missing_field("SubnetId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for MountTarget {
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
    #[derive(Debug, Default)]
    pub struct ElasticFileSystemTag {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-efs-filesystem-filesystemtags.html#cfn-efs-filesystem-filesystemtags-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-efs-filesystem-filesystemtags.html#cfn-efs-filesystem-filesystemtags-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for ElasticFileSystemTag {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ElasticFileSystemTag {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ElasticFileSystemTag, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ElasticFileSystemTag;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ElasticFileSystemTag")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ElasticFileSystemTag {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
