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
        if let Some(ref kms_key_id) = self.kms_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
        }
        if let Some(ref lustre_configuration) = self.lustre_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LustreConfiguration", lustre_configuration)?;
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
                let mut kms_key_id: Option<::Value<String>> = None;
                let mut lustre_configuration: Option<::Value<self::file_system::LustreConfiguration>> = None;
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
                        "KmsKeyId" => {
                            kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LustreConfiguration" => {
                            lustre_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
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
                    kms_key_id: kms_key_id,
                    lustre_configuration: lustre_configuration,
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

pub mod file_system {
    //! Property types for the `FileSystem` resource.

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
