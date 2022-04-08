//! Types for the `EFS` service.

/// The [`AWS::EFS::AccessPoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-efs-accesspoint.html) resource type.
#[derive(Debug, Default)]
pub struct AccessPoint {
    properties: AccessPointProperties
}

/// Properties for the `AccessPoint` resource.
#[derive(Debug, Default)]
pub struct AccessPointProperties {
    /// Property [`AccessPointTags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-efs-accesspoint.html#cfn-efs-accesspoint-accesspointtags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub access_point_tags: Option<::ValueList<self::access_point::AccessPointTag>>,
    /// Property [`ClientToken`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-efs-accesspoint.html#cfn-efs-accesspoint-clienttoken).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub client_token: Option<::Value<String>>,
    /// Property [`FileSystemId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-efs-accesspoint.html#cfn-efs-accesspoint-filesystemid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub file_system_id: ::Value<String>,
    /// Property [`PosixUser`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-efs-accesspoint.html#cfn-efs-accesspoint-posixuser).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub posix_user: Option<::Value<self::access_point::PosixUser>>,
    /// Property [`RootDirectory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-efs-accesspoint.html#cfn-efs-accesspoint-rootdirectory).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub root_directory: Option<::Value<self::access_point::RootDirectory>>,
}

impl ::serde::Serialize for AccessPointProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref access_point_tags) = self.access_point_tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessPointTags", access_point_tags)?;
        }
        if let Some(ref client_token) = self.client_token {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientToken", client_token)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FileSystemId", &self.file_system_id)?;
        if let Some(ref posix_user) = self.posix_user {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PosixUser", posix_user)?;
        }
        if let Some(ref root_directory) = self.root_directory {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RootDirectory", root_directory)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AccessPointProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AccessPointProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AccessPointProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AccessPointProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut access_point_tags: Option<::ValueList<self::access_point::AccessPointTag>> = None;
                let mut client_token: Option<::Value<String>> = None;
                let mut file_system_id: Option<::Value<String>> = None;
                let mut posix_user: Option<::Value<self::access_point::PosixUser>> = None;
                let mut root_directory: Option<::Value<self::access_point::RootDirectory>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AccessPointTags" => {
                            access_point_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClientToken" => {
                            client_token = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FileSystemId" => {
                            file_system_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PosixUser" => {
                            posix_user = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RootDirectory" => {
                            root_directory = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AccessPointProperties {
                    access_point_tags: access_point_tags,
                    client_token: client_token,
                    file_system_id: file_system_id.ok_or(::serde::de::Error::missing_field("FileSystemId"))?,
                    posix_user: posix_user,
                    root_directory: root_directory,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for AccessPoint {
    type Properties = AccessPointProperties;
    const TYPE: &'static str = "AWS::EFS::AccessPoint";
    fn properties(&self) -> &AccessPointProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AccessPointProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for AccessPoint {}

impl From<AccessPointProperties> for AccessPoint {
    fn from(properties: AccessPointProperties) -> AccessPoint {
        AccessPoint { properties }
    }
}

/// The [`AWS::EFS::FileSystem`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-efs-filesystem.html) resource type.
#[derive(Debug, Default)]
pub struct FileSystem {
    properties: FileSystemProperties
}

/// Properties for the `FileSystem` resource.
#[derive(Debug, Default)]
pub struct FileSystemProperties {
    /// Property [`AvailabilityZoneName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-efs-filesystem.html#cfn-efs-filesystem-availabilityzonename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub availability_zone_name: Option<::Value<String>>,
    /// Property [`BackupPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-efs-filesystem.html#cfn-efs-filesystem-backuppolicy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub backup_policy: Option<::Value<self::file_system::BackupPolicy>>,
    /// Property [`BypassPolicyLockoutSafetyCheck`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-efs-filesystem.html#cfn-efs-filesystem-bypasspolicylockoutsafetycheck).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub bypass_policy_lockout_safety_check: Option<::Value<bool>>,
    /// Property [`Encrypted`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-efs-filesystem.html#cfn-efs-filesystem-encrypted).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub encrypted: Option<::Value<bool>>,
    /// Property [`FileSystemPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-efs-filesystem.html#cfn-efs-filesystem-filesystempolicy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub file_system_policy: Option<::Value<::json::Value>>,
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
    /// Property [`LifecyclePolicies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-efs-filesystem.html#cfn-efs-filesystem-lifecyclepolicies).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub lifecycle_policies: Option<::ValueList<self::file_system::LifecyclePolicy>>,
    /// Property [`PerformanceMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-efs-filesystem.html#cfn-efs-filesystem-performancemode).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub performance_mode: Option<::Value<String>>,
    /// Property [`ProvisionedThroughputInMibps`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-efs-filesystem.html#cfn-efs-filesystem-provisionedthroughputinmibps).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub provisioned_throughput_in_mibps: Option<::Value<f64>>,
    /// Property [`ThroughputMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-efs-filesystem.html#cfn-efs-filesystem-throughputmode).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub throughput_mode: Option<::Value<String>>,
}

impl ::serde::Serialize for FileSystemProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref availability_zone_name) = self.availability_zone_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZoneName", availability_zone_name)?;
        }
        if let Some(ref backup_policy) = self.backup_policy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BackupPolicy", backup_policy)?;
        }
        if let Some(ref bypass_policy_lockout_safety_check) = self.bypass_policy_lockout_safety_check {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BypassPolicyLockoutSafetyCheck", bypass_policy_lockout_safety_check)?;
        }
        if let Some(ref encrypted) = self.encrypted {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Encrypted", encrypted)?;
        }
        if let Some(ref file_system_policy) = self.file_system_policy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FileSystemPolicy", file_system_policy)?;
        }
        if let Some(ref file_system_tags) = self.file_system_tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FileSystemTags", file_system_tags)?;
        }
        if let Some(ref kms_key_id) = self.kms_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
        }
        if let Some(ref lifecycle_policies) = self.lifecycle_policies {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LifecyclePolicies", lifecycle_policies)?;
        }
        if let Some(ref performance_mode) = self.performance_mode {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PerformanceMode", performance_mode)?;
        }
        if let Some(ref provisioned_throughput_in_mibps) = self.provisioned_throughput_in_mibps {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProvisionedThroughputInMibps", provisioned_throughput_in_mibps)?;
        }
        if let Some(ref throughput_mode) = self.throughput_mode {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThroughputMode", throughput_mode)?;
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
                let mut availability_zone_name: Option<::Value<String>> = None;
                let mut backup_policy: Option<::Value<self::file_system::BackupPolicy>> = None;
                let mut bypass_policy_lockout_safety_check: Option<::Value<bool>> = None;
                let mut encrypted: Option<::Value<bool>> = None;
                let mut file_system_policy: Option<::Value<::json::Value>> = None;
                let mut file_system_tags: Option<::ValueList<self::file_system::ElasticFileSystemTag>> = None;
                let mut kms_key_id: Option<::Value<String>> = None;
                let mut lifecycle_policies: Option<::ValueList<self::file_system::LifecyclePolicy>> = None;
                let mut performance_mode: Option<::Value<String>> = None;
                let mut provisioned_throughput_in_mibps: Option<::Value<f64>> = None;
                let mut throughput_mode: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AvailabilityZoneName" => {
                            availability_zone_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BackupPolicy" => {
                            backup_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BypassPolicyLockoutSafetyCheck" => {
                            bypass_policy_lockout_safety_check = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Encrypted" => {
                            encrypted = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FileSystemPolicy" => {
                            file_system_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FileSystemTags" => {
                            file_system_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsKeyId" => {
                            kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LifecyclePolicies" => {
                            lifecycle_policies = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PerformanceMode" => {
                            performance_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProvisionedThroughputInMibps" => {
                            provisioned_throughput_in_mibps = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ThroughputMode" => {
                            throughput_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(FileSystemProperties {
                    availability_zone_name: availability_zone_name,
                    backup_policy: backup_policy,
                    bypass_policy_lockout_safety_check: bypass_policy_lockout_safety_check,
                    encrypted: encrypted,
                    file_system_policy: file_system_policy,
                    file_system_tags: file_system_tags,
                    kms_key_id: kms_key_id,
                    lifecycle_policies: lifecycle_policies,
                    performance_mode: performance_mode,
                    provisioned_throughput_in_mibps: provisioned_throughput_in_mibps,
                    throughput_mode: throughput_mode,
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

pub mod access_point {
    //! Property types for the `AccessPoint` resource.

    /// The [`AWS::EFS::AccessPoint.AccessPointTag`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-efs-accesspoint-accesspointtag.html) property type.
    #[derive(Debug, Default)]
    pub struct AccessPointTag {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-efs-accesspoint-accesspointtag.html#cfn-efs-accesspoint-accesspointtag-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-efs-accesspoint-accesspointtag.html#cfn-efs-accesspoint-accesspointtag-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AccessPointTag {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref key) = self.key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", key)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AccessPointTag {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AccessPointTag, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AccessPointTag;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AccessPointTag")
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

                    Ok(AccessPointTag {
                        key: key,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EFS::AccessPoint.CreationInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-efs-accesspoint-creationinfo.html) property type.
    #[derive(Debug, Default)]
    pub struct CreationInfo {
        /// Property [`OwnerGid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-efs-accesspoint-creationinfo.html#cfn-efs-accesspoint-creationinfo-ownergid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub owner_gid: ::Value<String>,
        /// Property [`OwnerUid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-efs-accesspoint-creationinfo.html#cfn-efs-accesspoint-creationinfo-owneruid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub owner_uid: ::Value<String>,
        /// Property [`Permissions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-efs-accesspoint-creationinfo.html#cfn-efs-accesspoint-creationinfo-permissions).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub permissions: ::Value<String>,
    }

    impl ::codec::SerializeValue for CreationInfo {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OwnerGid", &self.owner_gid)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OwnerUid", &self.owner_uid)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Permissions", &self.permissions)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CreationInfo {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CreationInfo, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CreationInfo;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CreationInfo")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut owner_gid: Option<::Value<String>> = None;
                    let mut owner_uid: Option<::Value<String>> = None;
                    let mut permissions: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "OwnerGid" => {
                                owner_gid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OwnerUid" => {
                                owner_uid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Permissions" => {
                                permissions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CreationInfo {
                        owner_gid: owner_gid.ok_or(::serde::de::Error::missing_field("OwnerGid"))?,
                        owner_uid: owner_uid.ok_or(::serde::de::Error::missing_field("OwnerUid"))?,
                        permissions: permissions.ok_or(::serde::de::Error::missing_field("Permissions"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EFS::AccessPoint.PosixUser`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-efs-accesspoint-posixuser.html) property type.
    #[derive(Debug, Default)]
    pub struct PosixUser {
        /// Property [`Gid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-efs-accesspoint-posixuser.html#cfn-efs-accesspoint-posixuser-gid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub gid: ::Value<String>,
        /// Property [`SecondaryGids`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-efs-accesspoint-posixuser.html#cfn-efs-accesspoint-posixuser-secondarygids).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub secondary_gids: Option<::ValueList<String>>,
        /// Property [`Uid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-efs-accesspoint-posixuser.html#cfn-efs-accesspoint-posixuser-uid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub uid: ::Value<String>,
    }

    impl ::codec::SerializeValue for PosixUser {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Gid", &self.gid)?;
            if let Some(ref secondary_gids) = self.secondary_gids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecondaryGids", secondary_gids)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Uid", &self.uid)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PosixUser {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PosixUser, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PosixUser;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PosixUser")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut gid: Option<::Value<String>> = None;
                    let mut secondary_gids: Option<::ValueList<String>> = None;
                    let mut uid: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Gid" => {
                                gid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecondaryGids" => {
                                secondary_gids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Uid" => {
                                uid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PosixUser {
                        gid: gid.ok_or(::serde::de::Error::missing_field("Gid"))?,
                        secondary_gids: secondary_gids,
                        uid: uid.ok_or(::serde::de::Error::missing_field("Uid"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EFS::AccessPoint.RootDirectory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-efs-accesspoint-rootdirectory.html) property type.
    #[derive(Debug, Default)]
    pub struct RootDirectory {
        /// Property [`CreationInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-efs-accesspoint-rootdirectory.html#cfn-efs-accesspoint-rootdirectory-creationinfo).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub creation_info: Option<::Value<CreationInfo>>,
        /// Property [`Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-efs-accesspoint-rootdirectory.html#cfn-efs-accesspoint-rootdirectory-path).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub path: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for RootDirectory {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref creation_info) = self.creation_info {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CreationInfo", creation_info)?;
            }
            if let Some(ref path) = self.path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", path)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RootDirectory {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RootDirectory, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RootDirectory;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RootDirectory")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut creation_info: Option<::Value<CreationInfo>> = None;
                    let mut path: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CreationInfo" => {
                                creation_info = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Path" => {
                                path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RootDirectory {
                        creation_info: creation_info,
                        path: path,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod file_system {
    //! Property types for the `FileSystem` resource.

    /// The [`AWS::EFS::FileSystem.BackupPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-efs-filesystem-backuppolicy.html) property type.
    #[derive(Debug, Default)]
    pub struct BackupPolicy {
        /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-efs-filesystem-backuppolicy.html#cfn-efs-filesystem-backuppolicy-status).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub status: ::Value<String>,
    }

    impl ::codec::SerializeValue for BackupPolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", &self.status)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BackupPolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BackupPolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BackupPolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BackupPolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut status: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Status" => {
                                status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BackupPolicy {
                        status: status.ok_or(::serde::de::Error::missing_field("Status"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EFS::FileSystem.ElasticFileSystemTag`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-efs-filesystem-elasticfilesystemtag.html) property type.
    #[derive(Debug, Default)]
    pub struct ElasticFileSystemTag {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-efs-filesystem-elasticfilesystemtag.html#cfn-efs-filesystem-elasticfilesystemtag-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-efs-filesystem-elasticfilesystemtag.html#cfn-efs-filesystem-elasticfilesystemtag-value).
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

    /// The [`AWS::EFS::FileSystem.LifecyclePolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-efs-filesystem-lifecyclepolicy.html) property type.
    #[derive(Debug, Default)]
    pub struct LifecyclePolicy {
        /// Property [`TransitionToIA`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-efs-filesystem-lifecyclepolicy.html#cfn-efs-filesystem-lifecyclepolicy-transitiontoia).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub transition_to_ia: Option<::Value<String>>,
        /// Property [`TransitionToPrimaryStorageClass`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-efs-filesystem-lifecyclepolicy.html#cfn-efs-filesystem-lifecyclepolicy-transitiontoprimarystorageclass).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub transition_to_primary_storage_class: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LifecyclePolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref transition_to_ia) = self.transition_to_ia {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TransitionToIA", transition_to_ia)?;
            }
            if let Some(ref transition_to_primary_storage_class) = self.transition_to_primary_storage_class {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TransitionToPrimaryStorageClass", transition_to_primary_storage_class)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LifecyclePolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LifecyclePolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LifecyclePolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LifecyclePolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut transition_to_ia: Option<::Value<String>> = None;
                    let mut transition_to_primary_storage_class: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TransitionToIA" => {
                                transition_to_ia = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TransitionToPrimaryStorageClass" => {
                                transition_to_primary_storage_class = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LifecyclePolicy {
                        transition_to_ia: transition_to_ia,
                        transition_to_primary_storage_class: transition_to_primary_storage_class,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
