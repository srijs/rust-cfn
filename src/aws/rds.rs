//! Types for the `RDS` service.

/// The [`AWS::RDS::CustomDBEngineVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-customdbengineversion.html) resource type.
#[derive(Debug, Default)]
pub struct CustomDBEngineVersion {
    properties: CustomDBEngineVersionProperties
}

/// Properties for the `CustomDBEngineVersion` resource.
#[derive(Debug, Default)]
pub struct CustomDBEngineVersionProperties {
    /// Property [`DatabaseInstallationFilesS3BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-customdbengineversion.html#cfn-rds-customdbengineversion-databaseinstallationfiless3bucketname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub database_installation_files_s3_bucket_name: ::Value<String>,
    /// Property [`DatabaseInstallationFilesS3Prefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-customdbengineversion.html#cfn-rds-customdbengineversion-databaseinstallationfiless3prefix).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub database_installation_files_s3_prefix: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-customdbengineversion.html#cfn-rds-customdbengineversion-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Engine`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-customdbengineversion.html#cfn-rds-customdbengineversion-engine).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub engine: ::Value<String>,
    /// Property [`EngineVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-customdbengineversion.html#cfn-rds-customdbengineversion-engineversion).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub engine_version: ::Value<String>,
    /// Property [`KMSKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-customdbengineversion.html#cfn-rds-customdbengineversion-kmskeyid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kms_key_id: Option<::Value<String>>,
    /// Property [`Manifest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-customdbengineversion.html#cfn-rds-customdbengineversion-manifest).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub manifest: Option<::Value<String>>,
    /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-customdbengineversion.html#cfn-rds-customdbengineversion-status).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub status: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-customdbengineversion.html#cfn-rds-customdbengineversion-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for CustomDBEngineVersionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseInstallationFilesS3BucketName", &self.database_installation_files_s3_bucket_name)?;
        if let Some(ref database_installation_files_s3_prefix) = self.database_installation_files_s3_prefix {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseInstallationFilesS3Prefix", database_installation_files_s3_prefix)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Engine", &self.engine)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EngineVersion", &self.engine_version)?;
        if let Some(ref kms_key_id) = self.kms_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KMSKeyId", kms_key_id)?;
        }
        if let Some(ref manifest) = self.manifest {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Manifest", manifest)?;
        }
        if let Some(ref status) = self.status {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", status)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CustomDBEngineVersionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomDBEngineVersionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CustomDBEngineVersionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CustomDBEngineVersionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut database_installation_files_s3_bucket_name: Option<::Value<String>> = None;
                let mut database_installation_files_s3_prefix: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut engine: Option<::Value<String>> = None;
                let mut engine_version: Option<::Value<String>> = None;
                let mut kms_key_id: Option<::Value<String>> = None;
                let mut manifest: Option<::Value<String>> = None;
                let mut status: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DatabaseInstallationFilesS3BucketName" => {
                            database_installation_files_s3_bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DatabaseInstallationFilesS3Prefix" => {
                            database_installation_files_s3_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Engine" => {
                            engine = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EngineVersion" => {
                            engine_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KMSKeyId" => {
                            kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Manifest" => {
                            manifest = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Status" => {
                            status = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CustomDBEngineVersionProperties {
                    database_installation_files_s3_bucket_name: database_installation_files_s3_bucket_name.ok_or(::serde::de::Error::missing_field("DatabaseInstallationFilesS3BucketName"))?,
                    database_installation_files_s3_prefix: database_installation_files_s3_prefix,
                    description: description,
                    engine: engine.ok_or(::serde::de::Error::missing_field("Engine"))?,
                    engine_version: engine_version.ok_or(::serde::de::Error::missing_field("EngineVersion"))?,
                    kms_key_id: kms_key_id,
                    manifest: manifest,
                    status: status,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for CustomDBEngineVersion {
    type Properties = CustomDBEngineVersionProperties;
    const TYPE: &'static str = "AWS::RDS::CustomDBEngineVersion";
    fn properties(&self) -> &CustomDBEngineVersionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CustomDBEngineVersionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for CustomDBEngineVersion {}

impl From<CustomDBEngineVersionProperties> for CustomDBEngineVersion {
    fn from(properties: CustomDBEngineVersionProperties) -> CustomDBEngineVersion {
        CustomDBEngineVersion { properties }
    }
}

/// The [`AWS::RDS::DBCluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html) resource type.
#[derive(Debug, Default)]
pub struct DBCluster {
    properties: DBClusterProperties
}

/// Properties for the `DBCluster` resource.
#[derive(Debug, Default)]
pub struct DBClusterProperties {
    /// Property [`AllocatedStorage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-allocatedstorage).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub allocated_storage: Option<::Value<u32>>,
    /// Property [`AssociatedRoles`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-associatedroles).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub associated_roles: Option<::ValueList<self::db_cluster::DBClusterRole>>,
    /// Property [`AutoMinorVersionUpgrade`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-autominorversionupgrade).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub auto_minor_version_upgrade: Option<::Value<bool>>,
    /// Property [`AvailabilityZones`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-availabilityzones).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub availability_zones: Option<::ValueList<String>>,
    /// Property [`BacktrackWindow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-backtrackwindow).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub backtrack_window: Option<::Value<u32>>,
    /// Property [`BackupRetentionPeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-backupretentionperiod).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub backup_retention_period: Option<::Value<u32>>,
    /// Property [`CopyTagsToSnapshot`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-copytagstosnapshot).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub copy_tags_to_snapshot: Option<::Value<bool>>,
    /// Property [`DBClusterIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-dbclusteridentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub db_cluster_identifier: Option<::Value<String>>,
    /// Property [`DBClusterInstanceClass`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-dbclusterinstanceclass).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub db_cluster_instance_class: Option<::Value<String>>,
    /// Property [`DBClusterParameterGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-dbclusterparametergroupname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub db_cluster_parameter_group_name: Option<::Value<String>>,
    /// Property [`DBInstanceParameterGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-dbinstanceparametergroupname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub db_instance_parameter_group_name: Option<::Value<String>>,
    /// Property [`DBSubnetGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-dbsubnetgroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub db_subnet_group_name: Option<::Value<String>>,
    /// Property [`DBSystemId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-dbsystemid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub db_system_id: Option<::Value<String>>,
    /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-databasename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub database_name: Option<::Value<String>>,
    /// Property [`DeletionProtection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-deletionprotection).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub deletion_protection: Option<::Value<bool>>,
    /// Property [`Domain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-domain).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub domain: Option<::Value<String>>,
    /// Property [`DomainIAMRoleName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-domainiamrolename).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub domain_iam_role_name: Option<::Value<String>>,
    /// Property [`EnableCloudwatchLogsExports`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-enablecloudwatchlogsexports).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enable_cloudwatch_logs_exports: Option<::ValueList<String>>,
    /// Property [`EnableGlobalWriteForwarding`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-enableglobalwriteforwarding).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enable_global_write_forwarding: Option<::Value<bool>>,
    /// Property [`EnableHttpEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-enablehttpendpoint).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enable_http_endpoint: Option<::Value<bool>>,
    /// Property [`EnableIAMDatabaseAuthentication`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-enableiamdatabaseauthentication).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enable_iam_database_authentication: Option<::Value<bool>>,
    /// Property [`Engine`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-engine).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub engine: Option<::Value<String>>,
    /// Property [`EngineMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-enginemode).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub engine_mode: Option<::Value<String>>,
    /// Property [`EngineVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-engineversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub engine_version: Option<::Value<String>>,
    /// Property [`GlobalClusterIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-globalclusteridentifier).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub global_cluster_identifier: Option<::Value<String>>,
    /// Property [`Iops`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-iops).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub iops: Option<::Value<u32>>,
    /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-kmskeyid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kms_key_id: Option<::Value<String>>,
    /// Property [`ManageMasterUserPassword`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-managemasteruserpassword).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub manage_master_user_password: Option<::Value<bool>>,
    /// Property [`MasterUserPassword`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-masteruserpassword).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub master_user_password: Option<::Value<String>>,
    /// Property [`MasterUserSecret`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-masterusersecret).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub master_user_secret: Option<::Value<self::db_cluster::MasterUserSecret>>,
    /// Property [`MasterUsername`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-masterusername).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub master_username: Option<::Value<String>>,
    /// Property [`MonitoringInterval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-monitoringinterval).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub monitoring_interval: Option<::Value<u32>>,
    /// Property [`MonitoringRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-monitoringrolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub monitoring_role_arn: Option<::Value<String>>,
    /// Property [`NetworkType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-networktype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub network_type: Option<::Value<String>>,
    /// Property [`PerformanceInsightsEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-performanceinsightsenabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub performance_insights_enabled: Option<::Value<bool>>,
    /// Property [`PerformanceInsightsKmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-performanceinsightskmskeyid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub performance_insights_kms_key_id: Option<::Value<String>>,
    /// Property [`PerformanceInsightsRetentionPeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-performanceinsightsretentionperiod).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub performance_insights_retention_period: Option<::Value<u32>>,
    /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-port).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub port: Option<::Value<u32>>,
    /// Property [`PreferredBackupWindow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-preferredbackupwindow).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub preferred_backup_window: Option<::Value<String>>,
    /// Property [`PreferredMaintenanceWindow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-preferredmaintenancewindow).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub preferred_maintenance_window: Option<::Value<String>>,
    /// Property [`PubliclyAccessible`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-publiclyaccessible).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub publicly_accessible: Option<::Value<bool>>,
    /// Property [`ReplicationSourceIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-replicationsourceidentifier).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub replication_source_identifier: Option<::Value<String>>,
    /// Property [`RestoreToTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-restoretotime).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub restore_to_time: Option<::Value<String>>,
    /// Property [`RestoreType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-restoretype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub restore_type: Option<::Value<String>>,
    /// Property [`ScalingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-scalingconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub scaling_configuration: Option<::Value<self::db_cluster::ScalingConfiguration>>,
    /// Property [`ServerlessV2ScalingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-serverlessv2scalingconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub serverless_v2_scaling_configuration: Option<::Value<self::db_cluster::ServerlessV2ScalingConfiguration>>,
    /// Property [`SnapshotIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-snapshotidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub snapshot_identifier: Option<::Value<String>>,
    /// Property [`SourceDBClusterIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-sourcedbclusteridentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub source_db_cluster_identifier: Option<::Value<String>>,
    /// Property [`SourceRegion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-sourceregion).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub source_region: Option<::Value<String>>,
    /// Property [`StorageEncrypted`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-storageencrypted).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub storage_encrypted: Option<::Value<bool>>,
    /// Property [`StorageType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-storagetype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub storage_type: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`UseLatestRestorableTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-uselatestrestorabletime).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub use_latest_restorable_time: Option<::Value<bool>>,
    /// Property [`VpcSecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html#cfn-rds-dbcluster-vpcsecuritygroupids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub vpc_security_group_ids: Option<::ValueList<String>>,
}

impl ::serde::Serialize for DBClusterProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref allocated_storage) = self.allocated_storage {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllocatedStorage", allocated_storage)?;
        }
        if let Some(ref associated_roles) = self.associated_roles {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssociatedRoles", associated_roles)?;
        }
        if let Some(ref auto_minor_version_upgrade) = self.auto_minor_version_upgrade {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoMinorVersionUpgrade", auto_minor_version_upgrade)?;
        }
        if let Some(ref availability_zones) = self.availability_zones {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZones", availability_zones)?;
        }
        if let Some(ref backtrack_window) = self.backtrack_window {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BacktrackWindow", backtrack_window)?;
        }
        if let Some(ref backup_retention_period) = self.backup_retention_period {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BackupRetentionPeriod", backup_retention_period)?;
        }
        if let Some(ref copy_tags_to_snapshot) = self.copy_tags_to_snapshot {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CopyTagsToSnapshot", copy_tags_to_snapshot)?;
        }
        if let Some(ref db_cluster_identifier) = self.db_cluster_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBClusterIdentifier", db_cluster_identifier)?;
        }
        if let Some(ref db_cluster_instance_class) = self.db_cluster_instance_class {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBClusterInstanceClass", db_cluster_instance_class)?;
        }
        if let Some(ref db_cluster_parameter_group_name) = self.db_cluster_parameter_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBClusterParameterGroupName", db_cluster_parameter_group_name)?;
        }
        if let Some(ref db_instance_parameter_group_name) = self.db_instance_parameter_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBInstanceParameterGroupName", db_instance_parameter_group_name)?;
        }
        if let Some(ref db_subnet_group_name) = self.db_subnet_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBSubnetGroupName", db_subnet_group_name)?;
        }
        if let Some(ref db_system_id) = self.db_system_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBSystemId", db_system_id)?;
        }
        if let Some(ref database_name) = self.database_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", database_name)?;
        }
        if let Some(ref deletion_protection) = self.deletion_protection {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeletionProtection", deletion_protection)?;
        }
        if let Some(ref domain) = self.domain {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Domain", domain)?;
        }
        if let Some(ref domain_iam_role_name) = self.domain_iam_role_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainIAMRoleName", domain_iam_role_name)?;
        }
        if let Some(ref enable_cloudwatch_logs_exports) = self.enable_cloudwatch_logs_exports {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableCloudwatchLogsExports", enable_cloudwatch_logs_exports)?;
        }
        if let Some(ref enable_global_write_forwarding) = self.enable_global_write_forwarding {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableGlobalWriteForwarding", enable_global_write_forwarding)?;
        }
        if let Some(ref enable_http_endpoint) = self.enable_http_endpoint {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableHttpEndpoint", enable_http_endpoint)?;
        }
        if let Some(ref enable_iam_database_authentication) = self.enable_iam_database_authentication {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableIAMDatabaseAuthentication", enable_iam_database_authentication)?;
        }
        if let Some(ref engine) = self.engine {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Engine", engine)?;
        }
        if let Some(ref engine_mode) = self.engine_mode {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EngineMode", engine_mode)?;
        }
        if let Some(ref engine_version) = self.engine_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EngineVersion", engine_version)?;
        }
        if let Some(ref global_cluster_identifier) = self.global_cluster_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GlobalClusterIdentifier", global_cluster_identifier)?;
        }
        if let Some(ref iops) = self.iops {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Iops", iops)?;
        }
        if let Some(ref kms_key_id) = self.kms_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
        }
        if let Some(ref manage_master_user_password) = self.manage_master_user_password {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManageMasterUserPassword", manage_master_user_password)?;
        }
        if let Some(ref master_user_password) = self.master_user_password {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MasterUserPassword", master_user_password)?;
        }
        if let Some(ref master_user_secret) = self.master_user_secret {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MasterUserSecret", master_user_secret)?;
        }
        if let Some(ref master_username) = self.master_username {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MasterUsername", master_username)?;
        }
        if let Some(ref monitoring_interval) = self.monitoring_interval {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MonitoringInterval", monitoring_interval)?;
        }
        if let Some(ref monitoring_role_arn) = self.monitoring_role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MonitoringRoleArn", monitoring_role_arn)?;
        }
        if let Some(ref network_type) = self.network_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkType", network_type)?;
        }
        if let Some(ref performance_insights_enabled) = self.performance_insights_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PerformanceInsightsEnabled", performance_insights_enabled)?;
        }
        if let Some(ref performance_insights_kms_key_id) = self.performance_insights_kms_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PerformanceInsightsKmsKeyId", performance_insights_kms_key_id)?;
        }
        if let Some(ref performance_insights_retention_period) = self.performance_insights_retention_period {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PerformanceInsightsRetentionPeriod", performance_insights_retention_period)?;
        }
        if let Some(ref port) = self.port {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
        }
        if let Some(ref preferred_backup_window) = self.preferred_backup_window {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredBackupWindow", preferred_backup_window)?;
        }
        if let Some(ref preferred_maintenance_window) = self.preferred_maintenance_window {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredMaintenanceWindow", preferred_maintenance_window)?;
        }
        if let Some(ref publicly_accessible) = self.publicly_accessible {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PubliclyAccessible", publicly_accessible)?;
        }
        if let Some(ref replication_source_identifier) = self.replication_source_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicationSourceIdentifier", replication_source_identifier)?;
        }
        if let Some(ref restore_to_time) = self.restore_to_time {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RestoreToTime", restore_to_time)?;
        }
        if let Some(ref restore_type) = self.restore_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RestoreType", restore_type)?;
        }
        if let Some(ref scaling_configuration) = self.scaling_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScalingConfiguration", scaling_configuration)?;
        }
        if let Some(ref serverless_v2_scaling_configuration) = self.serverless_v2_scaling_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerlessV2ScalingConfiguration", serverless_v2_scaling_configuration)?;
        }
        if let Some(ref snapshot_identifier) = self.snapshot_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotIdentifier", snapshot_identifier)?;
        }
        if let Some(ref source_db_cluster_identifier) = self.source_db_cluster_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceDBClusterIdentifier", source_db_cluster_identifier)?;
        }
        if let Some(ref source_region) = self.source_region {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceRegion", source_region)?;
        }
        if let Some(ref storage_encrypted) = self.storage_encrypted {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StorageEncrypted", storage_encrypted)?;
        }
        if let Some(ref storage_type) = self.storage_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StorageType", storage_type)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref use_latest_restorable_time) = self.use_latest_restorable_time {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UseLatestRestorableTime", use_latest_restorable_time)?;
        }
        if let Some(ref vpc_security_group_ids) = self.vpc_security_group_ids {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcSecurityGroupIds", vpc_security_group_ids)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DBClusterProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DBClusterProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DBClusterProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DBClusterProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut allocated_storage: Option<::Value<u32>> = None;
                let mut associated_roles: Option<::ValueList<self::db_cluster::DBClusterRole>> = None;
                let mut auto_minor_version_upgrade: Option<::Value<bool>> = None;
                let mut availability_zones: Option<::ValueList<String>> = None;
                let mut backtrack_window: Option<::Value<u32>> = None;
                let mut backup_retention_period: Option<::Value<u32>> = None;
                let mut copy_tags_to_snapshot: Option<::Value<bool>> = None;
                let mut db_cluster_identifier: Option<::Value<String>> = None;
                let mut db_cluster_instance_class: Option<::Value<String>> = None;
                let mut db_cluster_parameter_group_name: Option<::Value<String>> = None;
                let mut db_instance_parameter_group_name: Option<::Value<String>> = None;
                let mut db_subnet_group_name: Option<::Value<String>> = None;
                let mut db_system_id: Option<::Value<String>> = None;
                let mut database_name: Option<::Value<String>> = None;
                let mut deletion_protection: Option<::Value<bool>> = None;
                let mut domain: Option<::Value<String>> = None;
                let mut domain_iam_role_name: Option<::Value<String>> = None;
                let mut enable_cloudwatch_logs_exports: Option<::ValueList<String>> = None;
                let mut enable_global_write_forwarding: Option<::Value<bool>> = None;
                let mut enable_http_endpoint: Option<::Value<bool>> = None;
                let mut enable_iam_database_authentication: Option<::Value<bool>> = None;
                let mut engine: Option<::Value<String>> = None;
                let mut engine_mode: Option<::Value<String>> = None;
                let mut engine_version: Option<::Value<String>> = None;
                let mut global_cluster_identifier: Option<::Value<String>> = None;
                let mut iops: Option<::Value<u32>> = None;
                let mut kms_key_id: Option<::Value<String>> = None;
                let mut manage_master_user_password: Option<::Value<bool>> = None;
                let mut master_user_password: Option<::Value<String>> = None;
                let mut master_user_secret: Option<::Value<self::db_cluster::MasterUserSecret>> = None;
                let mut master_username: Option<::Value<String>> = None;
                let mut monitoring_interval: Option<::Value<u32>> = None;
                let mut monitoring_role_arn: Option<::Value<String>> = None;
                let mut network_type: Option<::Value<String>> = None;
                let mut performance_insights_enabled: Option<::Value<bool>> = None;
                let mut performance_insights_kms_key_id: Option<::Value<String>> = None;
                let mut performance_insights_retention_period: Option<::Value<u32>> = None;
                let mut port: Option<::Value<u32>> = None;
                let mut preferred_backup_window: Option<::Value<String>> = None;
                let mut preferred_maintenance_window: Option<::Value<String>> = None;
                let mut publicly_accessible: Option<::Value<bool>> = None;
                let mut replication_source_identifier: Option<::Value<String>> = None;
                let mut restore_to_time: Option<::Value<String>> = None;
                let mut restore_type: Option<::Value<String>> = None;
                let mut scaling_configuration: Option<::Value<self::db_cluster::ScalingConfiguration>> = None;
                let mut serverless_v2_scaling_configuration: Option<::Value<self::db_cluster::ServerlessV2ScalingConfiguration>> = None;
                let mut snapshot_identifier: Option<::Value<String>> = None;
                let mut source_db_cluster_identifier: Option<::Value<String>> = None;
                let mut source_region: Option<::Value<String>> = None;
                let mut storage_encrypted: Option<::Value<bool>> = None;
                let mut storage_type: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut use_latest_restorable_time: Option<::Value<bool>> = None;
                let mut vpc_security_group_ids: Option<::ValueList<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AllocatedStorage" => {
                            allocated_storage = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AssociatedRoles" => {
                            associated_roles = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AutoMinorVersionUpgrade" => {
                            auto_minor_version_upgrade = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AvailabilityZones" => {
                            availability_zones = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BacktrackWindow" => {
                            backtrack_window = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BackupRetentionPeriod" => {
                            backup_retention_period = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CopyTagsToSnapshot" => {
                            copy_tags_to_snapshot = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DBClusterIdentifier" => {
                            db_cluster_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DBClusterInstanceClass" => {
                            db_cluster_instance_class = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DBClusterParameterGroupName" => {
                            db_cluster_parameter_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DBInstanceParameterGroupName" => {
                            db_instance_parameter_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DBSubnetGroupName" => {
                            db_subnet_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DBSystemId" => {
                            db_system_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DatabaseName" => {
                            database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DeletionProtection" => {
                            deletion_protection = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Domain" => {
                            domain = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DomainIAMRoleName" => {
                            domain_iam_role_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnableCloudwatchLogsExports" => {
                            enable_cloudwatch_logs_exports = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnableGlobalWriteForwarding" => {
                            enable_global_write_forwarding = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnableHttpEndpoint" => {
                            enable_http_endpoint = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnableIAMDatabaseAuthentication" => {
                            enable_iam_database_authentication = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Engine" => {
                            engine = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EngineMode" => {
                            engine_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EngineVersion" => {
                            engine_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GlobalClusterIdentifier" => {
                            global_cluster_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Iops" => {
                            iops = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsKeyId" => {
                            kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ManageMasterUserPassword" => {
                            manage_master_user_password = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MasterUserPassword" => {
                            master_user_password = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MasterUserSecret" => {
                            master_user_secret = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MasterUsername" => {
                            master_username = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MonitoringInterval" => {
                            monitoring_interval = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MonitoringRoleArn" => {
                            monitoring_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NetworkType" => {
                            network_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PerformanceInsightsEnabled" => {
                            performance_insights_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PerformanceInsightsKmsKeyId" => {
                            performance_insights_kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PerformanceInsightsRetentionPeriod" => {
                            performance_insights_retention_period = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Port" => {
                            port = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PreferredBackupWindow" => {
                            preferred_backup_window = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PreferredMaintenanceWindow" => {
                            preferred_maintenance_window = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PubliclyAccessible" => {
                            publicly_accessible = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReplicationSourceIdentifier" => {
                            replication_source_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RestoreToTime" => {
                            restore_to_time = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RestoreType" => {
                            restore_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ScalingConfiguration" => {
                            scaling_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServerlessV2ScalingConfiguration" => {
                            serverless_v2_scaling_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SnapshotIdentifier" => {
                            snapshot_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceDBClusterIdentifier" => {
                            source_db_cluster_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceRegion" => {
                            source_region = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StorageEncrypted" => {
                            storage_encrypted = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StorageType" => {
                            storage_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UseLatestRestorableTime" => {
                            use_latest_restorable_time = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcSecurityGroupIds" => {
                            vpc_security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DBClusterProperties {
                    allocated_storage: allocated_storage,
                    associated_roles: associated_roles,
                    auto_minor_version_upgrade: auto_minor_version_upgrade,
                    availability_zones: availability_zones,
                    backtrack_window: backtrack_window,
                    backup_retention_period: backup_retention_period,
                    copy_tags_to_snapshot: copy_tags_to_snapshot,
                    db_cluster_identifier: db_cluster_identifier,
                    db_cluster_instance_class: db_cluster_instance_class,
                    db_cluster_parameter_group_name: db_cluster_parameter_group_name,
                    db_instance_parameter_group_name: db_instance_parameter_group_name,
                    db_subnet_group_name: db_subnet_group_name,
                    db_system_id: db_system_id,
                    database_name: database_name,
                    deletion_protection: deletion_protection,
                    domain: domain,
                    domain_iam_role_name: domain_iam_role_name,
                    enable_cloudwatch_logs_exports: enable_cloudwatch_logs_exports,
                    enable_global_write_forwarding: enable_global_write_forwarding,
                    enable_http_endpoint: enable_http_endpoint,
                    enable_iam_database_authentication: enable_iam_database_authentication,
                    engine: engine,
                    engine_mode: engine_mode,
                    engine_version: engine_version,
                    global_cluster_identifier: global_cluster_identifier,
                    iops: iops,
                    kms_key_id: kms_key_id,
                    manage_master_user_password: manage_master_user_password,
                    master_user_password: master_user_password,
                    master_user_secret: master_user_secret,
                    master_username: master_username,
                    monitoring_interval: monitoring_interval,
                    monitoring_role_arn: monitoring_role_arn,
                    network_type: network_type,
                    performance_insights_enabled: performance_insights_enabled,
                    performance_insights_kms_key_id: performance_insights_kms_key_id,
                    performance_insights_retention_period: performance_insights_retention_period,
                    port: port,
                    preferred_backup_window: preferred_backup_window,
                    preferred_maintenance_window: preferred_maintenance_window,
                    publicly_accessible: publicly_accessible,
                    replication_source_identifier: replication_source_identifier,
                    restore_to_time: restore_to_time,
                    restore_type: restore_type,
                    scaling_configuration: scaling_configuration,
                    serverless_v2_scaling_configuration: serverless_v2_scaling_configuration,
                    snapshot_identifier: snapshot_identifier,
                    source_db_cluster_identifier: source_db_cluster_identifier,
                    source_region: source_region,
                    storage_encrypted: storage_encrypted,
                    storage_type: storage_type,
                    tags: tags,
                    use_latest_restorable_time: use_latest_restorable_time,
                    vpc_security_group_ids: vpc_security_group_ids,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DBCluster {
    type Properties = DBClusterProperties;
    const TYPE: &'static str = "AWS::RDS::DBCluster";
    fn properties(&self) -> &DBClusterProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DBClusterProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DBCluster {}

impl From<DBClusterProperties> for DBCluster {
    fn from(properties: DBClusterProperties) -> DBCluster {
        DBCluster { properties }
    }
}

/// The [`AWS::RDS::DBClusterParameterGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbclusterparametergroup.html) resource type.
#[derive(Debug, Default)]
pub struct DBClusterParameterGroup {
    properties: DBClusterParameterGroupProperties
}

/// Properties for the `DBClusterParameterGroup` resource.
#[derive(Debug, Default)]
pub struct DBClusterParameterGroupProperties {
    /// Property [`DBClusterParameterGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbclusterparametergroup.html#cfn-rds-dbclusterparametergroup-dbclusterparametergroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub db_cluster_parameter_group_name: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbclusterparametergroup.html#cfn-rds-dbclusterparametergroup-description).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub description: ::Value<String>,
    /// Property [`Family`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbclusterparametergroup.html#cfn-rds-dbclusterparametergroup-family).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub family: ::Value<String>,
    /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbclusterparametergroup.html#cfn-rds-dbclusterparametergroup-parameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub parameters: ::Value<::json::Value>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbclusterparametergroup.html#cfn-rds-dbclusterparametergroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for DBClusterParameterGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref db_cluster_parameter_group_name) = self.db_cluster_parameter_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBClusterParameterGroupName", db_cluster_parameter_group_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Family", &self.family)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", &self.parameters)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DBClusterParameterGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DBClusterParameterGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DBClusterParameterGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DBClusterParameterGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut db_cluster_parameter_group_name: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut family: Option<::Value<String>> = None;
                let mut parameters: Option<::Value<::json::Value>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DBClusterParameterGroupName" => {
                            db_cluster_parameter_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Family" => {
                            family = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Parameters" => {
                            parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DBClusterParameterGroupProperties {
                    db_cluster_parameter_group_name: db_cluster_parameter_group_name,
                    description: description.ok_or(::serde::de::Error::missing_field("Description"))?,
                    family: family.ok_or(::serde::de::Error::missing_field("Family"))?,
                    parameters: parameters.ok_or(::serde::de::Error::missing_field("Parameters"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DBClusterParameterGroup {
    type Properties = DBClusterParameterGroupProperties;
    const TYPE: &'static str = "AWS::RDS::DBClusterParameterGroup";
    fn properties(&self) -> &DBClusterParameterGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DBClusterParameterGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DBClusterParameterGroup {}

impl From<DBClusterParameterGroupProperties> for DBClusterParameterGroup {
    fn from(properties: DBClusterParameterGroupProperties) -> DBClusterParameterGroup {
        DBClusterParameterGroup { properties }
    }
}

/// The [`AWS::RDS::DBInstance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html) resource type.
#[derive(Debug, Default)]
pub struct DBInstance {
    properties: DBInstanceProperties
}

/// Properties for the `DBInstance` resource.
#[derive(Debug, Default)]
pub struct DBInstanceProperties {
    /// Property [`AllocatedStorage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-allocatedstorage).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub allocated_storage: Option<::Value<String>>,
    /// Property [`AllowMajorVersionUpgrade`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-allowmajorversionupgrade).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub allow_major_version_upgrade: Option<::Value<bool>>,
    /// Property [`AssociatedRoles`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-associatedroles).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub associated_roles: Option<::ValueList<self::db_instance::DBInstanceRole>>,
    /// Property [`AutoMinorVersionUpgrade`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-autominorversionupgrade).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub auto_minor_version_upgrade: Option<::Value<bool>>,
    /// Property [`AutomaticBackupReplicationRegion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-automaticbackupreplicationregion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub automatic_backup_replication_region: Option<::Value<String>>,
    /// Property [`AvailabilityZone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-availabilityzone).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub availability_zone: Option<::Value<String>>,
    /// Property [`BackupRetentionPeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-backupretentionperiod).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub backup_retention_period: Option<::Value<u32>>,
    /// Property [`CACertificateIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-cacertificateidentifier).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ca_certificate_identifier: Option<::Value<String>>,
    /// Property [`CertificateDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-certificatedetails).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub certificate_details: Option<::Value<self::db_instance::CertificateDetails>>,
    /// Property [`CertificateRotationRestart`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-certificaterotationrestart).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub certificate_rotation_restart: Option<::Value<bool>>,
    /// Property [`CharacterSetName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-charactersetname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub character_set_name: Option<::Value<String>>,
    /// Property [`CopyTagsToSnapshot`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-copytagstosnapshot).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub copy_tags_to_snapshot: Option<::Value<bool>>,
    /// Property [`CustomIAMInstanceProfile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-customiaminstanceprofile).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub custom_iam_instance_profile: Option<::Value<String>>,
    /// Property [`DBClusterIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-dbclusteridentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub db_cluster_identifier: Option<::Value<String>>,
    /// Property [`DBClusterSnapshotIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-dbclustersnapshotidentifier).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub db_cluster_snapshot_identifier: Option<::Value<String>>,
    /// Property [`DBInstanceClass`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-dbinstanceclass).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub db_instance_class: Option<::Value<String>>,
    /// Property [`DBInstanceIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-dbinstanceidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub db_instance_identifier: Option<::Value<String>>,
    /// Property [`DBName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-dbname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub db_name: Option<::Value<String>>,
    /// Property [`DBParameterGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-dbparametergroupname).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub db_parameter_group_name: Option<::Value<String>>,
    /// Property [`DBSecurityGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-dbsecuritygroups).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub db_security_groups: Option<::ValueList<String>>,
    /// Property [`DBSnapshotIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-dbsnapshotidentifier).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub db_snapshot_identifier: Option<::Value<String>>,
    /// Property [`DBSubnetGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-dbsubnetgroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub db_subnet_group_name: Option<::Value<String>>,
    /// Property [`DedicatedLogVolume`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-dedicatedlogvolume).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub dedicated_log_volume: Option<::Value<bool>>,
    /// Property [`DeleteAutomatedBackups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-deleteautomatedbackups).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub delete_automated_backups: Option<::Value<bool>>,
    /// Property [`DeletionProtection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-deletionprotection).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub deletion_protection: Option<::Value<bool>>,
    /// Property [`Domain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-domain).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub domain: Option<::Value<String>>,
    /// Property [`DomainAuthSecretArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-domainauthsecretarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub domain_auth_secret_arn: Option<::Value<String>>,
    /// Property [`DomainDnsIps`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-domaindnsips).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub domain_dns_ips: Option<::ValueList<String>>,
    /// Property [`DomainFqdn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-domainfqdn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub domain_fqdn: Option<::Value<String>>,
    /// Property [`DomainIAMRoleName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-domainiamrolename).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub domain_iam_role_name: Option<::Value<String>>,
    /// Property [`DomainOu`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-domainou).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub domain_ou: Option<::Value<String>>,
    /// Property [`EnableCloudwatchLogsExports`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-enablecloudwatchlogsexports).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enable_cloudwatch_logs_exports: Option<::ValueList<String>>,
    /// Property [`EnableIAMDatabaseAuthentication`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-enableiamdatabaseauthentication).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enable_iam_database_authentication: Option<::Value<bool>>,
    /// Property [`EnablePerformanceInsights`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-enableperformanceinsights).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enable_performance_insights: Option<::Value<bool>>,
    /// Property [`Endpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-endpoint).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub endpoint: Option<::Value<self::db_instance::Endpoint>>,
    /// Property [`Engine`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-engine).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub engine: Option<::Value<String>>,
    /// Property [`EngineVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-engineversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub engine_version: Option<::Value<String>>,
    /// Property [`Iops`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-iops).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub iops: Option<::Value<u32>>,
    /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-kmskeyid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kms_key_id: Option<::Value<String>>,
    /// Property [`LicenseModel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-licensemodel).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub license_model: Option<::Value<String>>,
    /// Property [`ManageMasterUserPassword`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-managemasteruserpassword).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub manage_master_user_password: Option<::Value<bool>>,
    /// Property [`MasterUserPassword`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-masteruserpassword).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub master_user_password: Option<::Value<String>>,
    /// Property [`MasterUserSecret`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-masterusersecret).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub master_user_secret: Option<::Value<self::db_instance::MasterUserSecret>>,
    /// Property [`MasterUsername`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-masterusername).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub master_username: Option<::Value<String>>,
    /// Property [`MaxAllocatedStorage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-maxallocatedstorage).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub max_allocated_storage: Option<::Value<u32>>,
    /// Property [`MonitoringInterval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-monitoringinterval).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub monitoring_interval: Option<::Value<u32>>,
    /// Property [`MonitoringRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-monitoringrolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub monitoring_role_arn: Option<::Value<String>>,
    /// Property [`MultiAZ`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-multiaz).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub multi_az: Option<::Value<bool>>,
    /// Property [`NcharCharacterSetName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-ncharcharactersetname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub nchar_character_set_name: Option<::Value<String>>,
    /// Property [`NetworkType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-networktype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub network_type: Option<::Value<String>>,
    /// Property [`OptionGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-optiongroupname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub option_group_name: Option<::Value<String>>,
    /// Property [`PerformanceInsightsKMSKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-performanceinsightskmskeyid).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub performance_insights_kms_key_id: Option<::Value<String>>,
    /// Property [`PerformanceInsightsRetentionPeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-performanceinsightsretentionperiod).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub performance_insights_retention_period: Option<::Value<u32>>,
    /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-port).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub port: Option<::Value<String>>,
    /// Property [`PreferredBackupWindow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-preferredbackupwindow).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub preferred_backup_window: Option<::Value<String>>,
    /// Property [`PreferredMaintenanceWindow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-preferredmaintenancewindow).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub preferred_maintenance_window: Option<::Value<String>>,
    /// Property [`ProcessorFeatures`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-processorfeatures).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub processor_features: Option<::ValueList<self::db_instance::ProcessorFeature>>,
    /// Property [`PromotionTier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-promotiontier).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub promotion_tier: Option<::Value<u32>>,
    /// Property [`PubliclyAccessible`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-publiclyaccessible).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub publicly_accessible: Option<::Value<bool>>,
    /// Property [`ReplicaMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-replicamode).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub replica_mode: Option<::Value<String>>,
    /// Property [`RestoreTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-restoretime).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub restore_time: Option<::Value<String>>,
    /// Property [`SourceDBClusterIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-sourcedbclusteridentifier).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub source_db_cluster_identifier: Option<::Value<String>>,
    /// Property [`SourceDBInstanceAutomatedBackupsArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-sourcedbinstanceautomatedbackupsarn).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub source_db_instance_automated_backups_arn: Option<::Value<String>>,
    /// Property [`SourceDBInstanceIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-sourcedbinstanceidentifier).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub source_db_instance_identifier: Option<::Value<String>>,
    /// Property [`SourceDbiResourceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-sourcedbiresourceid).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub source_dbi_resource_id: Option<::Value<String>>,
    /// Property [`SourceRegion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-sourceregion).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub source_region: Option<::Value<String>>,
    /// Property [`StorageEncrypted`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-storageencrypted).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub storage_encrypted: Option<::Value<bool>>,
    /// Property [`StorageThroughput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-storagethroughput).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub storage_throughput: Option<::Value<u32>>,
    /// Property [`StorageType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-storagetype).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub storage_type: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Timezone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-timezone).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub timezone: Option<::Value<String>>,
    /// Property [`UseDefaultProcessorFeatures`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-usedefaultprocessorfeatures).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub use_default_processor_features: Option<::Value<bool>>,
    /// Property [`UseLatestRestorableTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-uselatestrestorabletime).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub use_latest_restorable_time: Option<::Value<bool>>,
    /// Property [`VPCSecurityGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbinstance.html#cfn-rds-dbinstance-vpcsecuritygroups).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub vpc_security_groups: Option<::ValueList<String>>,
}

impl ::serde::Serialize for DBInstanceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref allocated_storage) = self.allocated_storage {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllocatedStorage", allocated_storage)?;
        }
        if let Some(ref allow_major_version_upgrade) = self.allow_major_version_upgrade {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowMajorVersionUpgrade", allow_major_version_upgrade)?;
        }
        if let Some(ref associated_roles) = self.associated_roles {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssociatedRoles", associated_roles)?;
        }
        if let Some(ref auto_minor_version_upgrade) = self.auto_minor_version_upgrade {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoMinorVersionUpgrade", auto_minor_version_upgrade)?;
        }
        if let Some(ref automatic_backup_replication_region) = self.automatic_backup_replication_region {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutomaticBackupReplicationRegion", automatic_backup_replication_region)?;
        }
        if let Some(ref availability_zone) = self.availability_zone {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZone", availability_zone)?;
        }
        if let Some(ref backup_retention_period) = self.backup_retention_period {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BackupRetentionPeriod", backup_retention_period)?;
        }
        if let Some(ref ca_certificate_identifier) = self.ca_certificate_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CACertificateIdentifier", ca_certificate_identifier)?;
        }
        if let Some(ref certificate_details) = self.certificate_details {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateDetails", certificate_details)?;
        }
        if let Some(ref certificate_rotation_restart) = self.certificate_rotation_restart {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateRotationRestart", certificate_rotation_restart)?;
        }
        if let Some(ref character_set_name) = self.character_set_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CharacterSetName", character_set_name)?;
        }
        if let Some(ref copy_tags_to_snapshot) = self.copy_tags_to_snapshot {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CopyTagsToSnapshot", copy_tags_to_snapshot)?;
        }
        if let Some(ref custom_iam_instance_profile) = self.custom_iam_instance_profile {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomIAMInstanceProfile", custom_iam_instance_profile)?;
        }
        if let Some(ref db_cluster_identifier) = self.db_cluster_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBClusterIdentifier", db_cluster_identifier)?;
        }
        if let Some(ref db_cluster_snapshot_identifier) = self.db_cluster_snapshot_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBClusterSnapshotIdentifier", db_cluster_snapshot_identifier)?;
        }
        if let Some(ref db_instance_class) = self.db_instance_class {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBInstanceClass", db_instance_class)?;
        }
        if let Some(ref db_instance_identifier) = self.db_instance_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBInstanceIdentifier", db_instance_identifier)?;
        }
        if let Some(ref db_name) = self.db_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBName", db_name)?;
        }
        if let Some(ref db_parameter_group_name) = self.db_parameter_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBParameterGroupName", db_parameter_group_name)?;
        }
        if let Some(ref db_security_groups) = self.db_security_groups {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBSecurityGroups", db_security_groups)?;
        }
        if let Some(ref db_snapshot_identifier) = self.db_snapshot_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBSnapshotIdentifier", db_snapshot_identifier)?;
        }
        if let Some(ref db_subnet_group_name) = self.db_subnet_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBSubnetGroupName", db_subnet_group_name)?;
        }
        if let Some(ref dedicated_log_volume) = self.dedicated_log_volume {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DedicatedLogVolume", dedicated_log_volume)?;
        }
        if let Some(ref delete_automated_backups) = self.delete_automated_backups {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeleteAutomatedBackups", delete_automated_backups)?;
        }
        if let Some(ref deletion_protection) = self.deletion_protection {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeletionProtection", deletion_protection)?;
        }
        if let Some(ref domain) = self.domain {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Domain", domain)?;
        }
        if let Some(ref domain_auth_secret_arn) = self.domain_auth_secret_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainAuthSecretArn", domain_auth_secret_arn)?;
        }
        if let Some(ref domain_dns_ips) = self.domain_dns_ips {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainDnsIps", domain_dns_ips)?;
        }
        if let Some(ref domain_fqdn) = self.domain_fqdn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainFqdn", domain_fqdn)?;
        }
        if let Some(ref domain_iam_role_name) = self.domain_iam_role_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainIAMRoleName", domain_iam_role_name)?;
        }
        if let Some(ref domain_ou) = self.domain_ou {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainOu", domain_ou)?;
        }
        if let Some(ref enable_cloudwatch_logs_exports) = self.enable_cloudwatch_logs_exports {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableCloudwatchLogsExports", enable_cloudwatch_logs_exports)?;
        }
        if let Some(ref enable_iam_database_authentication) = self.enable_iam_database_authentication {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableIAMDatabaseAuthentication", enable_iam_database_authentication)?;
        }
        if let Some(ref enable_performance_insights) = self.enable_performance_insights {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnablePerformanceInsights", enable_performance_insights)?;
        }
        if let Some(ref endpoint) = self.endpoint {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Endpoint", endpoint)?;
        }
        if let Some(ref engine) = self.engine {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Engine", engine)?;
        }
        if let Some(ref engine_version) = self.engine_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EngineVersion", engine_version)?;
        }
        if let Some(ref iops) = self.iops {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Iops", iops)?;
        }
        if let Some(ref kms_key_id) = self.kms_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
        }
        if let Some(ref license_model) = self.license_model {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LicenseModel", license_model)?;
        }
        if let Some(ref manage_master_user_password) = self.manage_master_user_password {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManageMasterUserPassword", manage_master_user_password)?;
        }
        if let Some(ref master_user_password) = self.master_user_password {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MasterUserPassword", master_user_password)?;
        }
        if let Some(ref master_user_secret) = self.master_user_secret {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MasterUserSecret", master_user_secret)?;
        }
        if let Some(ref master_username) = self.master_username {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MasterUsername", master_username)?;
        }
        if let Some(ref max_allocated_storage) = self.max_allocated_storage {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxAllocatedStorage", max_allocated_storage)?;
        }
        if let Some(ref monitoring_interval) = self.monitoring_interval {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MonitoringInterval", monitoring_interval)?;
        }
        if let Some(ref monitoring_role_arn) = self.monitoring_role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MonitoringRoleArn", monitoring_role_arn)?;
        }
        if let Some(ref multi_az) = self.multi_az {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MultiAZ", multi_az)?;
        }
        if let Some(ref nchar_character_set_name) = self.nchar_character_set_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NcharCharacterSetName", nchar_character_set_name)?;
        }
        if let Some(ref network_type) = self.network_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkType", network_type)?;
        }
        if let Some(ref option_group_name) = self.option_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OptionGroupName", option_group_name)?;
        }
        if let Some(ref performance_insights_kms_key_id) = self.performance_insights_kms_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PerformanceInsightsKMSKeyId", performance_insights_kms_key_id)?;
        }
        if let Some(ref performance_insights_retention_period) = self.performance_insights_retention_period {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PerformanceInsightsRetentionPeriod", performance_insights_retention_period)?;
        }
        if let Some(ref port) = self.port {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
        }
        if let Some(ref preferred_backup_window) = self.preferred_backup_window {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredBackupWindow", preferred_backup_window)?;
        }
        if let Some(ref preferred_maintenance_window) = self.preferred_maintenance_window {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredMaintenanceWindow", preferred_maintenance_window)?;
        }
        if let Some(ref processor_features) = self.processor_features {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProcessorFeatures", processor_features)?;
        }
        if let Some(ref promotion_tier) = self.promotion_tier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PromotionTier", promotion_tier)?;
        }
        if let Some(ref publicly_accessible) = self.publicly_accessible {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PubliclyAccessible", publicly_accessible)?;
        }
        if let Some(ref replica_mode) = self.replica_mode {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicaMode", replica_mode)?;
        }
        if let Some(ref restore_time) = self.restore_time {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RestoreTime", restore_time)?;
        }
        if let Some(ref source_db_cluster_identifier) = self.source_db_cluster_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceDBClusterIdentifier", source_db_cluster_identifier)?;
        }
        if let Some(ref source_db_instance_automated_backups_arn) = self.source_db_instance_automated_backups_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceDBInstanceAutomatedBackupsArn", source_db_instance_automated_backups_arn)?;
        }
        if let Some(ref source_db_instance_identifier) = self.source_db_instance_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceDBInstanceIdentifier", source_db_instance_identifier)?;
        }
        if let Some(ref source_dbi_resource_id) = self.source_dbi_resource_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceDbiResourceId", source_dbi_resource_id)?;
        }
        if let Some(ref source_region) = self.source_region {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceRegion", source_region)?;
        }
        if let Some(ref storage_encrypted) = self.storage_encrypted {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StorageEncrypted", storage_encrypted)?;
        }
        if let Some(ref storage_throughput) = self.storage_throughput {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StorageThroughput", storage_throughput)?;
        }
        if let Some(ref storage_type) = self.storage_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StorageType", storage_type)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref timezone) = self.timezone {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Timezone", timezone)?;
        }
        if let Some(ref use_default_processor_features) = self.use_default_processor_features {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UseDefaultProcessorFeatures", use_default_processor_features)?;
        }
        if let Some(ref use_latest_restorable_time) = self.use_latest_restorable_time {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UseLatestRestorableTime", use_latest_restorable_time)?;
        }
        if let Some(ref vpc_security_groups) = self.vpc_security_groups {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VPCSecurityGroups", vpc_security_groups)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DBInstanceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DBInstanceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DBInstanceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DBInstanceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut allocated_storage: Option<::Value<String>> = None;
                let mut allow_major_version_upgrade: Option<::Value<bool>> = None;
                let mut associated_roles: Option<::ValueList<self::db_instance::DBInstanceRole>> = None;
                let mut auto_minor_version_upgrade: Option<::Value<bool>> = None;
                let mut automatic_backup_replication_region: Option<::Value<String>> = None;
                let mut availability_zone: Option<::Value<String>> = None;
                let mut backup_retention_period: Option<::Value<u32>> = None;
                let mut ca_certificate_identifier: Option<::Value<String>> = None;
                let mut certificate_details: Option<::Value<self::db_instance::CertificateDetails>> = None;
                let mut certificate_rotation_restart: Option<::Value<bool>> = None;
                let mut character_set_name: Option<::Value<String>> = None;
                let mut copy_tags_to_snapshot: Option<::Value<bool>> = None;
                let mut custom_iam_instance_profile: Option<::Value<String>> = None;
                let mut db_cluster_identifier: Option<::Value<String>> = None;
                let mut db_cluster_snapshot_identifier: Option<::Value<String>> = None;
                let mut db_instance_class: Option<::Value<String>> = None;
                let mut db_instance_identifier: Option<::Value<String>> = None;
                let mut db_name: Option<::Value<String>> = None;
                let mut db_parameter_group_name: Option<::Value<String>> = None;
                let mut db_security_groups: Option<::ValueList<String>> = None;
                let mut db_snapshot_identifier: Option<::Value<String>> = None;
                let mut db_subnet_group_name: Option<::Value<String>> = None;
                let mut dedicated_log_volume: Option<::Value<bool>> = None;
                let mut delete_automated_backups: Option<::Value<bool>> = None;
                let mut deletion_protection: Option<::Value<bool>> = None;
                let mut domain: Option<::Value<String>> = None;
                let mut domain_auth_secret_arn: Option<::Value<String>> = None;
                let mut domain_dns_ips: Option<::ValueList<String>> = None;
                let mut domain_fqdn: Option<::Value<String>> = None;
                let mut domain_iam_role_name: Option<::Value<String>> = None;
                let mut domain_ou: Option<::Value<String>> = None;
                let mut enable_cloudwatch_logs_exports: Option<::ValueList<String>> = None;
                let mut enable_iam_database_authentication: Option<::Value<bool>> = None;
                let mut enable_performance_insights: Option<::Value<bool>> = None;
                let mut endpoint: Option<::Value<self::db_instance::Endpoint>> = None;
                let mut engine: Option<::Value<String>> = None;
                let mut engine_version: Option<::Value<String>> = None;
                let mut iops: Option<::Value<u32>> = None;
                let mut kms_key_id: Option<::Value<String>> = None;
                let mut license_model: Option<::Value<String>> = None;
                let mut manage_master_user_password: Option<::Value<bool>> = None;
                let mut master_user_password: Option<::Value<String>> = None;
                let mut master_user_secret: Option<::Value<self::db_instance::MasterUserSecret>> = None;
                let mut master_username: Option<::Value<String>> = None;
                let mut max_allocated_storage: Option<::Value<u32>> = None;
                let mut monitoring_interval: Option<::Value<u32>> = None;
                let mut monitoring_role_arn: Option<::Value<String>> = None;
                let mut multi_az: Option<::Value<bool>> = None;
                let mut nchar_character_set_name: Option<::Value<String>> = None;
                let mut network_type: Option<::Value<String>> = None;
                let mut option_group_name: Option<::Value<String>> = None;
                let mut performance_insights_kms_key_id: Option<::Value<String>> = None;
                let mut performance_insights_retention_period: Option<::Value<u32>> = None;
                let mut port: Option<::Value<String>> = None;
                let mut preferred_backup_window: Option<::Value<String>> = None;
                let mut preferred_maintenance_window: Option<::Value<String>> = None;
                let mut processor_features: Option<::ValueList<self::db_instance::ProcessorFeature>> = None;
                let mut promotion_tier: Option<::Value<u32>> = None;
                let mut publicly_accessible: Option<::Value<bool>> = None;
                let mut replica_mode: Option<::Value<String>> = None;
                let mut restore_time: Option<::Value<String>> = None;
                let mut source_db_cluster_identifier: Option<::Value<String>> = None;
                let mut source_db_instance_automated_backups_arn: Option<::Value<String>> = None;
                let mut source_db_instance_identifier: Option<::Value<String>> = None;
                let mut source_dbi_resource_id: Option<::Value<String>> = None;
                let mut source_region: Option<::Value<String>> = None;
                let mut storage_encrypted: Option<::Value<bool>> = None;
                let mut storage_throughput: Option<::Value<u32>> = None;
                let mut storage_type: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut timezone: Option<::Value<String>> = None;
                let mut use_default_processor_features: Option<::Value<bool>> = None;
                let mut use_latest_restorable_time: Option<::Value<bool>> = None;
                let mut vpc_security_groups: Option<::ValueList<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AllocatedStorage" => {
                            allocated_storage = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AllowMajorVersionUpgrade" => {
                            allow_major_version_upgrade = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AssociatedRoles" => {
                            associated_roles = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AutoMinorVersionUpgrade" => {
                            auto_minor_version_upgrade = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AutomaticBackupReplicationRegion" => {
                            automatic_backup_replication_region = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AvailabilityZone" => {
                            availability_zone = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BackupRetentionPeriod" => {
                            backup_retention_period = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CACertificateIdentifier" => {
                            ca_certificate_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CertificateDetails" => {
                            certificate_details = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CertificateRotationRestart" => {
                            certificate_rotation_restart = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CharacterSetName" => {
                            character_set_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CopyTagsToSnapshot" => {
                            copy_tags_to_snapshot = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CustomIAMInstanceProfile" => {
                            custom_iam_instance_profile = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DBClusterIdentifier" => {
                            db_cluster_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DBClusterSnapshotIdentifier" => {
                            db_cluster_snapshot_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DBInstanceClass" => {
                            db_instance_class = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DBInstanceIdentifier" => {
                            db_instance_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DBName" => {
                            db_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DBParameterGroupName" => {
                            db_parameter_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DBSecurityGroups" => {
                            db_security_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DBSnapshotIdentifier" => {
                            db_snapshot_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DBSubnetGroupName" => {
                            db_subnet_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DedicatedLogVolume" => {
                            dedicated_log_volume = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DeleteAutomatedBackups" => {
                            delete_automated_backups = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DeletionProtection" => {
                            deletion_protection = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Domain" => {
                            domain = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DomainAuthSecretArn" => {
                            domain_auth_secret_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DomainDnsIps" => {
                            domain_dns_ips = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DomainFqdn" => {
                            domain_fqdn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DomainIAMRoleName" => {
                            domain_iam_role_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DomainOu" => {
                            domain_ou = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnableCloudwatchLogsExports" => {
                            enable_cloudwatch_logs_exports = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnableIAMDatabaseAuthentication" => {
                            enable_iam_database_authentication = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnablePerformanceInsights" => {
                            enable_performance_insights = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Endpoint" => {
                            endpoint = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Engine" => {
                            engine = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EngineVersion" => {
                            engine_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Iops" => {
                            iops = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsKeyId" => {
                            kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LicenseModel" => {
                            license_model = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ManageMasterUserPassword" => {
                            manage_master_user_password = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MasterUserPassword" => {
                            master_user_password = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MasterUserSecret" => {
                            master_user_secret = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MasterUsername" => {
                            master_username = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaxAllocatedStorage" => {
                            max_allocated_storage = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MonitoringInterval" => {
                            monitoring_interval = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MonitoringRoleArn" => {
                            monitoring_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MultiAZ" => {
                            multi_az = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NcharCharacterSetName" => {
                            nchar_character_set_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NetworkType" => {
                            network_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OptionGroupName" => {
                            option_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PerformanceInsightsKMSKeyId" => {
                            performance_insights_kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PerformanceInsightsRetentionPeriod" => {
                            performance_insights_retention_period = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Port" => {
                            port = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PreferredBackupWindow" => {
                            preferred_backup_window = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PreferredMaintenanceWindow" => {
                            preferred_maintenance_window = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProcessorFeatures" => {
                            processor_features = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PromotionTier" => {
                            promotion_tier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PubliclyAccessible" => {
                            publicly_accessible = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReplicaMode" => {
                            replica_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RestoreTime" => {
                            restore_time = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceDBClusterIdentifier" => {
                            source_db_cluster_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceDBInstanceAutomatedBackupsArn" => {
                            source_db_instance_automated_backups_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceDBInstanceIdentifier" => {
                            source_db_instance_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceDbiResourceId" => {
                            source_dbi_resource_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceRegion" => {
                            source_region = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StorageEncrypted" => {
                            storage_encrypted = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StorageThroughput" => {
                            storage_throughput = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StorageType" => {
                            storage_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Timezone" => {
                            timezone = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UseDefaultProcessorFeatures" => {
                            use_default_processor_features = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UseLatestRestorableTime" => {
                            use_latest_restorable_time = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VPCSecurityGroups" => {
                            vpc_security_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DBInstanceProperties {
                    allocated_storage: allocated_storage,
                    allow_major_version_upgrade: allow_major_version_upgrade,
                    associated_roles: associated_roles,
                    auto_minor_version_upgrade: auto_minor_version_upgrade,
                    automatic_backup_replication_region: automatic_backup_replication_region,
                    availability_zone: availability_zone,
                    backup_retention_period: backup_retention_period,
                    ca_certificate_identifier: ca_certificate_identifier,
                    certificate_details: certificate_details,
                    certificate_rotation_restart: certificate_rotation_restart,
                    character_set_name: character_set_name,
                    copy_tags_to_snapshot: copy_tags_to_snapshot,
                    custom_iam_instance_profile: custom_iam_instance_profile,
                    db_cluster_identifier: db_cluster_identifier,
                    db_cluster_snapshot_identifier: db_cluster_snapshot_identifier,
                    db_instance_class: db_instance_class,
                    db_instance_identifier: db_instance_identifier,
                    db_name: db_name,
                    db_parameter_group_name: db_parameter_group_name,
                    db_security_groups: db_security_groups,
                    db_snapshot_identifier: db_snapshot_identifier,
                    db_subnet_group_name: db_subnet_group_name,
                    dedicated_log_volume: dedicated_log_volume,
                    delete_automated_backups: delete_automated_backups,
                    deletion_protection: deletion_protection,
                    domain: domain,
                    domain_auth_secret_arn: domain_auth_secret_arn,
                    domain_dns_ips: domain_dns_ips,
                    domain_fqdn: domain_fqdn,
                    domain_iam_role_name: domain_iam_role_name,
                    domain_ou: domain_ou,
                    enable_cloudwatch_logs_exports: enable_cloudwatch_logs_exports,
                    enable_iam_database_authentication: enable_iam_database_authentication,
                    enable_performance_insights: enable_performance_insights,
                    endpoint: endpoint,
                    engine: engine,
                    engine_version: engine_version,
                    iops: iops,
                    kms_key_id: kms_key_id,
                    license_model: license_model,
                    manage_master_user_password: manage_master_user_password,
                    master_user_password: master_user_password,
                    master_user_secret: master_user_secret,
                    master_username: master_username,
                    max_allocated_storage: max_allocated_storage,
                    monitoring_interval: monitoring_interval,
                    monitoring_role_arn: monitoring_role_arn,
                    multi_az: multi_az,
                    nchar_character_set_name: nchar_character_set_name,
                    network_type: network_type,
                    option_group_name: option_group_name,
                    performance_insights_kms_key_id: performance_insights_kms_key_id,
                    performance_insights_retention_period: performance_insights_retention_period,
                    port: port,
                    preferred_backup_window: preferred_backup_window,
                    preferred_maintenance_window: preferred_maintenance_window,
                    processor_features: processor_features,
                    promotion_tier: promotion_tier,
                    publicly_accessible: publicly_accessible,
                    replica_mode: replica_mode,
                    restore_time: restore_time,
                    source_db_cluster_identifier: source_db_cluster_identifier,
                    source_db_instance_automated_backups_arn: source_db_instance_automated_backups_arn,
                    source_db_instance_identifier: source_db_instance_identifier,
                    source_dbi_resource_id: source_dbi_resource_id,
                    source_region: source_region,
                    storage_encrypted: storage_encrypted,
                    storage_throughput: storage_throughput,
                    storage_type: storage_type,
                    tags: tags,
                    timezone: timezone,
                    use_default_processor_features: use_default_processor_features,
                    use_latest_restorable_time: use_latest_restorable_time,
                    vpc_security_groups: vpc_security_groups,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DBInstance {
    type Properties = DBInstanceProperties;
    const TYPE: &'static str = "AWS::RDS::DBInstance";
    fn properties(&self) -> &DBInstanceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DBInstanceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DBInstance {}

impl From<DBInstanceProperties> for DBInstance {
    fn from(properties: DBInstanceProperties) -> DBInstance {
        DBInstance { properties }
    }
}

/// The [`AWS::RDS::DBParameterGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbparametergroup.html) resource type.
#[derive(Debug, Default)]
pub struct DBParameterGroup {
    properties: DBParameterGroupProperties
}

/// Properties for the `DBParameterGroup` resource.
#[derive(Debug, Default)]
pub struct DBParameterGroupProperties {
    /// Property [`DBParameterGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbparametergroup.html#cfn-rds-dbparametergroup-dbparametergroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub db_parameter_group_name: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbparametergroup.html#cfn-rds-dbparametergroup-description).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub description: ::Value<String>,
    /// Property [`Family`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbparametergroup.html#cfn-rds-dbparametergroup-family).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub family: ::Value<String>,
    /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbparametergroup.html#cfn-rds-dbparametergroup-parameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub parameters: Option<::Value<::json::Value>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbparametergroup.html#cfn-rds-dbparametergroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for DBParameterGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref db_parameter_group_name) = self.db_parameter_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBParameterGroupName", db_parameter_group_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Family", &self.family)?;
        if let Some(ref parameters) = self.parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", parameters)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DBParameterGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DBParameterGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DBParameterGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DBParameterGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut db_parameter_group_name: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut family: Option<::Value<String>> = None;
                let mut parameters: Option<::Value<::json::Value>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DBParameterGroupName" => {
                            db_parameter_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Family" => {
                            family = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Parameters" => {
                            parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DBParameterGroupProperties {
                    db_parameter_group_name: db_parameter_group_name,
                    description: description.ok_or(::serde::de::Error::missing_field("Description"))?,
                    family: family.ok_or(::serde::de::Error::missing_field("Family"))?,
                    parameters: parameters,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DBParameterGroup {
    type Properties = DBParameterGroupProperties;
    const TYPE: &'static str = "AWS::RDS::DBParameterGroup";
    fn properties(&self) -> &DBParameterGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DBParameterGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DBParameterGroup {}

impl From<DBParameterGroupProperties> for DBParameterGroup {
    fn from(properties: DBParameterGroupProperties) -> DBParameterGroup {
        DBParameterGroup { properties }
    }
}

/// The [`AWS::RDS::DBProxy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxy.html) resource type.
#[derive(Debug, Default)]
pub struct DBProxy {
    properties: DBProxyProperties
}

/// Properties for the `DBProxy` resource.
#[derive(Debug, Default)]
pub struct DBProxyProperties {
    /// Property [`Auth`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxy.html#cfn-rds-dbproxy-auth).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub auth: ::ValueList<self::db_proxy::AuthFormat>,
    /// Property [`DBProxyName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxy.html#cfn-rds-dbproxy-dbproxyname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub db_proxy_name: ::Value<String>,
    /// Property [`DebugLogging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxy.html#cfn-rds-dbproxy-debuglogging).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub debug_logging: Option<::Value<bool>>,
    /// Property [`EngineFamily`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxy.html#cfn-rds-dbproxy-enginefamily).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub engine_family: ::Value<String>,
    /// Property [`IdleClientTimeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxy.html#cfn-rds-dbproxy-idleclienttimeout).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub idle_client_timeout: Option<::Value<u32>>,
    /// Property [`RequireTLS`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxy.html#cfn-rds-dbproxy-requiretls).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub require_tls: Option<::Value<bool>>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxy.html#cfn-rds-dbproxy-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxy.html#cfn-rds-dbproxy-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<self::db_proxy::TagFormat>>,
    /// Property [`VpcSecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxy.html#cfn-rds-dbproxy-vpcsecuritygroupids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub vpc_security_group_ids: Option<::ValueList<String>>,
    /// Property [`VpcSubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxy.html#cfn-rds-dbproxy-vpcsubnetids).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub vpc_subnet_ids: ::ValueList<String>,
}

impl ::serde::Serialize for DBProxyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Auth", &self.auth)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBProxyName", &self.db_proxy_name)?;
        if let Some(ref debug_logging) = self.debug_logging {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DebugLogging", debug_logging)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EngineFamily", &self.engine_family)?;
        if let Some(ref idle_client_timeout) = self.idle_client_timeout {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdleClientTimeout", idle_client_timeout)?;
        }
        if let Some(ref require_tls) = self.require_tls {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequireTLS", require_tls)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref vpc_security_group_ids) = self.vpc_security_group_ids {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcSecurityGroupIds", vpc_security_group_ids)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcSubnetIds", &self.vpc_subnet_ids)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DBProxyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DBProxyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DBProxyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DBProxyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut auth: Option<::ValueList<self::db_proxy::AuthFormat>> = None;
                let mut db_proxy_name: Option<::Value<String>> = None;
                let mut debug_logging: Option<::Value<bool>> = None;
                let mut engine_family: Option<::Value<String>> = None;
                let mut idle_client_timeout: Option<::Value<u32>> = None;
                let mut require_tls: Option<::Value<bool>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<self::db_proxy::TagFormat>> = None;
                let mut vpc_security_group_ids: Option<::ValueList<String>> = None;
                let mut vpc_subnet_ids: Option<::ValueList<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Auth" => {
                            auth = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DBProxyName" => {
                            db_proxy_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DebugLogging" => {
                            debug_logging = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EngineFamily" => {
                            engine_family = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IdleClientTimeout" => {
                            idle_client_timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RequireTLS" => {
                            require_tls = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcSecurityGroupIds" => {
                            vpc_security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcSubnetIds" => {
                            vpc_subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DBProxyProperties {
                    auth: auth.ok_or(::serde::de::Error::missing_field("Auth"))?,
                    db_proxy_name: db_proxy_name.ok_or(::serde::de::Error::missing_field("DBProxyName"))?,
                    debug_logging: debug_logging,
                    engine_family: engine_family.ok_or(::serde::de::Error::missing_field("EngineFamily"))?,
                    idle_client_timeout: idle_client_timeout,
                    require_tls: require_tls,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    tags: tags,
                    vpc_security_group_ids: vpc_security_group_ids,
                    vpc_subnet_ids: vpc_subnet_ids.ok_or(::serde::de::Error::missing_field("VpcSubnetIds"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DBProxy {
    type Properties = DBProxyProperties;
    const TYPE: &'static str = "AWS::RDS::DBProxy";
    fn properties(&self) -> &DBProxyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DBProxyProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DBProxy {}

impl From<DBProxyProperties> for DBProxy {
    fn from(properties: DBProxyProperties) -> DBProxy {
        DBProxy { properties }
    }
}

/// The [`AWS::RDS::DBProxyEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxyendpoint.html) resource type.
#[derive(Debug, Default)]
pub struct DBProxyEndpoint {
    properties: DBProxyEndpointProperties
}

/// Properties for the `DBProxyEndpoint` resource.
#[derive(Debug, Default)]
pub struct DBProxyEndpointProperties {
    /// Property [`DBProxyEndpointName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxyendpoint.html#cfn-rds-dbproxyendpoint-dbproxyendpointname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub db_proxy_endpoint_name: ::Value<String>,
    /// Property [`DBProxyName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxyendpoint.html#cfn-rds-dbproxyendpoint-dbproxyname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub db_proxy_name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxyendpoint.html#cfn-rds-dbproxyendpoint-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<self::db_proxy_endpoint::TagFormat>>,
    /// Property [`TargetRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxyendpoint.html#cfn-rds-dbproxyendpoint-targetrole).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub target_role: Option<::Value<String>>,
    /// Property [`VpcSecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxyendpoint.html#cfn-rds-dbproxyendpoint-vpcsecuritygroupids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub vpc_security_group_ids: Option<::ValueList<String>>,
    /// Property [`VpcSubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxyendpoint.html#cfn-rds-dbproxyendpoint-vpcsubnetids).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub vpc_subnet_ids: ::ValueList<String>,
}

impl ::serde::Serialize for DBProxyEndpointProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBProxyEndpointName", &self.db_proxy_endpoint_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBProxyName", &self.db_proxy_name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref target_role) = self.target_role {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetRole", target_role)?;
        }
        if let Some(ref vpc_security_group_ids) = self.vpc_security_group_ids {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcSecurityGroupIds", vpc_security_group_ids)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcSubnetIds", &self.vpc_subnet_ids)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DBProxyEndpointProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DBProxyEndpointProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DBProxyEndpointProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DBProxyEndpointProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut db_proxy_endpoint_name: Option<::Value<String>> = None;
                let mut db_proxy_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<self::db_proxy_endpoint::TagFormat>> = None;
                let mut target_role: Option<::Value<String>> = None;
                let mut vpc_security_group_ids: Option<::ValueList<String>> = None;
                let mut vpc_subnet_ids: Option<::ValueList<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DBProxyEndpointName" => {
                            db_proxy_endpoint_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DBProxyName" => {
                            db_proxy_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetRole" => {
                            target_role = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcSecurityGroupIds" => {
                            vpc_security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcSubnetIds" => {
                            vpc_subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DBProxyEndpointProperties {
                    db_proxy_endpoint_name: db_proxy_endpoint_name.ok_or(::serde::de::Error::missing_field("DBProxyEndpointName"))?,
                    db_proxy_name: db_proxy_name.ok_or(::serde::de::Error::missing_field("DBProxyName"))?,
                    tags: tags,
                    target_role: target_role,
                    vpc_security_group_ids: vpc_security_group_ids,
                    vpc_subnet_ids: vpc_subnet_ids.ok_or(::serde::de::Error::missing_field("VpcSubnetIds"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DBProxyEndpoint {
    type Properties = DBProxyEndpointProperties;
    const TYPE: &'static str = "AWS::RDS::DBProxyEndpoint";
    fn properties(&self) -> &DBProxyEndpointProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DBProxyEndpointProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DBProxyEndpoint {}

impl From<DBProxyEndpointProperties> for DBProxyEndpoint {
    fn from(properties: DBProxyEndpointProperties) -> DBProxyEndpoint {
        DBProxyEndpoint { properties }
    }
}

/// The [`AWS::RDS::DBProxyTargetGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxytargetgroup.html) resource type.
#[derive(Debug, Default)]
pub struct DBProxyTargetGroup {
    properties: DBProxyTargetGroupProperties
}

/// Properties for the `DBProxyTargetGroup` resource.
#[derive(Debug, Default)]
pub struct DBProxyTargetGroupProperties {
    /// Property [`ConnectionPoolConfigurationInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxytargetgroup.html#cfn-rds-dbproxytargetgroup-connectionpoolconfigurationinfo).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub connection_pool_configuration_info: Option<::Value<self::db_proxy_target_group::ConnectionPoolConfigurationInfoFormat>>,
    /// Property [`DBClusterIdentifiers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxytargetgroup.html#cfn-rds-dbproxytargetgroup-dbclusteridentifiers).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub db_cluster_identifiers: Option<::ValueList<String>>,
    /// Property [`DBInstanceIdentifiers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxytargetgroup.html#cfn-rds-dbproxytargetgroup-dbinstanceidentifiers).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub db_instance_identifiers: Option<::ValueList<String>>,
    /// Property [`DBProxyName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxytargetgroup.html#cfn-rds-dbproxytargetgroup-dbproxyname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub db_proxy_name: ::Value<String>,
    /// Property [`TargetGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbproxytargetgroup.html#cfn-rds-dbproxytargetgroup-targetgroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub target_group_name: ::Value<String>,
}

impl ::serde::Serialize for DBProxyTargetGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref connection_pool_configuration_info) = self.connection_pool_configuration_info {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionPoolConfigurationInfo", connection_pool_configuration_info)?;
        }
        if let Some(ref db_cluster_identifiers) = self.db_cluster_identifiers {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBClusterIdentifiers", db_cluster_identifiers)?;
        }
        if let Some(ref db_instance_identifiers) = self.db_instance_identifiers {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBInstanceIdentifiers", db_instance_identifiers)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBProxyName", &self.db_proxy_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetGroupName", &self.target_group_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DBProxyTargetGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DBProxyTargetGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DBProxyTargetGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DBProxyTargetGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut connection_pool_configuration_info: Option<::Value<self::db_proxy_target_group::ConnectionPoolConfigurationInfoFormat>> = None;
                let mut db_cluster_identifiers: Option<::ValueList<String>> = None;
                let mut db_instance_identifiers: Option<::ValueList<String>> = None;
                let mut db_proxy_name: Option<::Value<String>> = None;
                let mut target_group_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ConnectionPoolConfigurationInfo" => {
                            connection_pool_configuration_info = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DBClusterIdentifiers" => {
                            db_cluster_identifiers = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DBInstanceIdentifiers" => {
                            db_instance_identifiers = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DBProxyName" => {
                            db_proxy_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetGroupName" => {
                            target_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DBProxyTargetGroupProperties {
                    connection_pool_configuration_info: connection_pool_configuration_info,
                    db_cluster_identifiers: db_cluster_identifiers,
                    db_instance_identifiers: db_instance_identifiers,
                    db_proxy_name: db_proxy_name.ok_or(::serde::de::Error::missing_field("DBProxyName"))?,
                    target_group_name: target_group_name.ok_or(::serde::de::Error::missing_field("TargetGroupName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DBProxyTargetGroup {
    type Properties = DBProxyTargetGroupProperties;
    const TYPE: &'static str = "AWS::RDS::DBProxyTargetGroup";
    fn properties(&self) -> &DBProxyTargetGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DBProxyTargetGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DBProxyTargetGroup {}

impl From<DBProxyTargetGroupProperties> for DBProxyTargetGroup {
    fn from(properties: DBProxyTargetGroupProperties) -> DBProxyTargetGroup {
        DBProxyTargetGroup { properties }
    }
}

/// The [`AWS::RDS::DBSecurityGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-security-group.html) resource type.
#[derive(Debug, Default)]
pub struct DBSecurityGroup {
    properties: DBSecurityGroupProperties
}

/// Properties for the `DBSecurityGroup` resource.
#[derive(Debug, Default)]
pub struct DBSecurityGroupProperties {
    /// Property [`DBSecurityGroupIngress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-security-group.html#cfn-rds-dbsecuritygroup-dbsecuritygroupingress).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub db_security_group_ingress: ::ValueList<self::db_security_group::Ingress>,
    /// Property [`EC2VpcId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-security-group.html#cfn-rds-dbsecuritygroup-ec2vpcid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub ec2_vpc_id: Option<::Value<String>>,
    /// Property [`GroupDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-security-group.html#cfn-rds-dbsecuritygroup-groupdescription).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub group_description: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-security-group.html#cfn-rds-dbsecuritygroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for DBSecurityGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBSecurityGroupIngress", &self.db_security_group_ingress)?;
        if let Some(ref ec2_vpc_id) = self.ec2_vpc_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EC2VpcId", ec2_vpc_id)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupDescription", &self.group_description)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DBSecurityGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DBSecurityGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DBSecurityGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DBSecurityGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut db_security_group_ingress: Option<::ValueList<self::db_security_group::Ingress>> = None;
                let mut ec2_vpc_id: Option<::Value<String>> = None;
                let mut group_description: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DBSecurityGroupIngress" => {
                            db_security_group_ingress = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EC2VpcId" => {
                            ec2_vpc_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GroupDescription" => {
                            group_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DBSecurityGroupProperties {
                    db_security_group_ingress: db_security_group_ingress.ok_or(::serde::de::Error::missing_field("DBSecurityGroupIngress"))?,
                    ec2_vpc_id: ec2_vpc_id,
                    group_description: group_description.ok_or(::serde::de::Error::missing_field("GroupDescription"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DBSecurityGroup {
    type Properties = DBSecurityGroupProperties;
    const TYPE: &'static str = "AWS::RDS::DBSecurityGroup";
    fn properties(&self) -> &DBSecurityGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DBSecurityGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DBSecurityGroup {}

impl From<DBSecurityGroupProperties> for DBSecurityGroup {
    fn from(properties: DBSecurityGroupProperties) -> DBSecurityGroup {
        DBSecurityGroup { properties }
    }
}

/// The [`AWS::RDS::DBSecurityGroupIngress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-security-group-ingress.html) resource type.
#[derive(Debug, Default)]
pub struct DBSecurityGroupIngress {
    properties: DBSecurityGroupIngressProperties
}

/// Properties for the `DBSecurityGroupIngress` resource.
#[derive(Debug, Default)]
pub struct DBSecurityGroupIngressProperties {
    /// Property [`CIDRIP`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-security-group-ingress.html#cfn-rds-securitygroup-ingress-cidrip).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cidrip: Option<::Value<String>>,
    /// Property [`DBSecurityGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-security-group-ingress.html#cfn-rds-securitygroup-ingress-dbsecuritygroupname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub db_security_group_name: ::Value<String>,
    /// Property [`EC2SecurityGroupId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-security-group-ingress.html#cfn-rds-securitygroup-ingress-ec2securitygroupid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ec2_security_group_id: Option<::Value<String>>,
    /// Property [`EC2SecurityGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-security-group-ingress.html#cfn-rds-securitygroup-ingress-ec2securitygroupname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ec2_security_group_name: Option<::Value<String>>,
    /// Property [`EC2SecurityGroupOwnerId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-security-group-ingress.html#cfn-rds-securitygroup-ingress-ec2securitygroupownerid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ec2_security_group_owner_id: Option<::Value<String>>,
}

impl ::serde::Serialize for DBSecurityGroupIngressProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref cidrip) = self.cidrip {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CIDRIP", cidrip)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBSecurityGroupName", &self.db_security_group_name)?;
        if let Some(ref ec2_security_group_id) = self.ec2_security_group_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EC2SecurityGroupId", ec2_security_group_id)?;
        }
        if let Some(ref ec2_security_group_name) = self.ec2_security_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EC2SecurityGroupName", ec2_security_group_name)?;
        }
        if let Some(ref ec2_security_group_owner_id) = self.ec2_security_group_owner_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EC2SecurityGroupOwnerId", ec2_security_group_owner_id)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DBSecurityGroupIngressProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DBSecurityGroupIngressProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DBSecurityGroupIngressProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DBSecurityGroupIngressProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut cidrip: Option<::Value<String>> = None;
                let mut db_security_group_name: Option<::Value<String>> = None;
                let mut ec2_security_group_id: Option<::Value<String>> = None;
                let mut ec2_security_group_name: Option<::Value<String>> = None;
                let mut ec2_security_group_owner_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CIDRIP" => {
                            cidrip = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DBSecurityGroupName" => {
                            db_security_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EC2SecurityGroupId" => {
                            ec2_security_group_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EC2SecurityGroupName" => {
                            ec2_security_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EC2SecurityGroupOwnerId" => {
                            ec2_security_group_owner_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DBSecurityGroupIngressProperties {
                    cidrip: cidrip,
                    db_security_group_name: db_security_group_name.ok_or(::serde::de::Error::missing_field("DBSecurityGroupName"))?,
                    ec2_security_group_id: ec2_security_group_id,
                    ec2_security_group_name: ec2_security_group_name,
                    ec2_security_group_owner_id: ec2_security_group_owner_id,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DBSecurityGroupIngress {
    type Properties = DBSecurityGroupIngressProperties;
    const TYPE: &'static str = "AWS::RDS::DBSecurityGroupIngress";
    fn properties(&self) -> &DBSecurityGroupIngressProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DBSecurityGroupIngressProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DBSecurityGroupIngress {}

impl From<DBSecurityGroupIngressProperties> for DBSecurityGroupIngress {
    fn from(properties: DBSecurityGroupIngressProperties) -> DBSecurityGroupIngress {
        DBSecurityGroupIngress { properties }
    }
}

/// The [`AWS::RDS::DBSubnetGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbsubnetgroup.html) resource type.
#[derive(Debug, Default)]
pub struct DBSubnetGroup {
    properties: DBSubnetGroupProperties
}

/// Properties for the `DBSubnetGroup` resource.
#[derive(Debug, Default)]
pub struct DBSubnetGroupProperties {
    /// Property [`DBSubnetGroupDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbsubnetgroup.html#cfn-rds-dbsubnetgroup-dbsubnetgroupdescription).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub db_subnet_group_description: ::Value<String>,
    /// Property [`DBSubnetGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbsubnetgroup.html#cfn-rds-dbsubnetgroup-dbsubnetgroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub db_subnet_group_name: Option<::Value<String>>,
    /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbsubnetgroup.html#cfn-rds-dbsubnetgroup-subnetids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub subnet_ids: ::ValueList<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbsubnetgroup.html#cfn-rds-dbsubnetgroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for DBSubnetGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBSubnetGroupDescription", &self.db_subnet_group_description)?;
        if let Some(ref db_subnet_group_name) = self.db_subnet_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBSubnetGroupName", db_subnet_group_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", &self.subnet_ids)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DBSubnetGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DBSubnetGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DBSubnetGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DBSubnetGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut db_subnet_group_description: Option<::Value<String>> = None;
                let mut db_subnet_group_name: Option<::Value<String>> = None;
                let mut subnet_ids: Option<::ValueList<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DBSubnetGroupDescription" => {
                            db_subnet_group_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DBSubnetGroupName" => {
                            db_subnet_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SubnetIds" => {
                            subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DBSubnetGroupProperties {
                    db_subnet_group_description: db_subnet_group_description.ok_or(::serde::de::Error::missing_field("DBSubnetGroupDescription"))?,
                    db_subnet_group_name: db_subnet_group_name,
                    subnet_ids: subnet_ids.ok_or(::serde::de::Error::missing_field("SubnetIds"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DBSubnetGroup {
    type Properties = DBSubnetGroupProperties;
    const TYPE: &'static str = "AWS::RDS::DBSubnetGroup";
    fn properties(&self) -> &DBSubnetGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DBSubnetGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DBSubnetGroup {}

impl From<DBSubnetGroupProperties> for DBSubnetGroup {
    fn from(properties: DBSubnetGroupProperties) -> DBSubnetGroup {
        DBSubnetGroup { properties }
    }
}

/// The [`AWS::RDS::EventSubscription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-eventsubscription.html) resource type.
#[derive(Debug, Default)]
pub struct EventSubscription {
    properties: EventSubscriptionProperties
}

/// Properties for the `EventSubscription` resource.
#[derive(Debug, Default)]
pub struct EventSubscriptionProperties {
    /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-eventsubscription.html#cfn-rds-eventsubscription-enabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enabled: Option<::Value<bool>>,
    /// Property [`EventCategories`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-eventsubscription.html#cfn-rds-eventsubscription-eventcategories).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub event_categories: Option<::ValueList<String>>,
    /// Property [`SnsTopicArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-eventsubscription.html#cfn-rds-eventsubscription-snstopicarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub sns_topic_arn: ::Value<String>,
    /// Property [`SourceIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-eventsubscription.html#cfn-rds-eventsubscription-sourceids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub source_ids: Option<::ValueList<String>>,
    /// Property [`SourceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-eventsubscription.html#cfn-rds-eventsubscription-sourcetype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub source_type: Option<::Value<String>>,
    /// Property [`SubscriptionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-eventsubscription.html#cfn-rds-eventsubscription-subscriptionname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub subscription_name: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-eventsubscription.html#cfn-rds-eventsubscription-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for EventSubscriptionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref enabled) = self.enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
        }
        if let Some(ref event_categories) = self.event_categories {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventCategories", event_categories)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnsTopicArn", &self.sns_topic_arn)?;
        if let Some(ref source_ids) = self.source_ids {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceIds", source_ids)?;
        }
        if let Some(ref source_type) = self.source_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceType", source_type)?;
        }
        if let Some(ref subscription_name) = self.subscription_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubscriptionName", subscription_name)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EventSubscriptionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EventSubscriptionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EventSubscriptionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EventSubscriptionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut enabled: Option<::Value<bool>> = None;
                let mut event_categories: Option<::ValueList<String>> = None;
                let mut sns_topic_arn: Option<::Value<String>> = None;
                let mut source_ids: Option<::ValueList<String>> = None;
                let mut source_type: Option<::Value<String>> = None;
                let mut subscription_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Enabled" => {
                            enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EventCategories" => {
                            event_categories = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SnsTopicArn" => {
                            sns_topic_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceIds" => {
                            source_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceType" => {
                            source_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SubscriptionName" => {
                            subscription_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(EventSubscriptionProperties {
                    enabled: enabled,
                    event_categories: event_categories,
                    sns_topic_arn: sns_topic_arn.ok_or(::serde::de::Error::missing_field("SnsTopicArn"))?,
                    source_ids: source_ids,
                    source_type: source_type,
                    subscription_name: subscription_name,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for EventSubscription {
    type Properties = EventSubscriptionProperties;
    const TYPE: &'static str = "AWS::RDS::EventSubscription";
    fn properties(&self) -> &EventSubscriptionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EventSubscriptionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for EventSubscription {}

impl From<EventSubscriptionProperties> for EventSubscription {
    fn from(properties: EventSubscriptionProperties) -> EventSubscription {
        EventSubscription { properties }
    }
}

/// The [`AWS::RDS::GlobalCluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-globalcluster.html) resource type.
#[derive(Debug, Default)]
pub struct GlobalCluster {
    properties: GlobalClusterProperties
}

/// Properties for the `GlobalCluster` resource.
#[derive(Debug, Default)]
pub struct GlobalClusterProperties {
    /// Property [`DeletionProtection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-globalcluster.html#cfn-rds-globalcluster-deletionprotection).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub deletion_protection: Option<::Value<bool>>,
    /// Property [`Engine`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-globalcluster.html#cfn-rds-globalcluster-engine).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub engine: Option<::Value<String>>,
    /// Property [`EngineVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-globalcluster.html#cfn-rds-globalcluster-engineversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub engine_version: Option<::Value<String>>,
    /// Property [`GlobalClusterIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-globalcluster.html#cfn-rds-globalcluster-globalclusteridentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub global_cluster_identifier: Option<::Value<String>>,
    /// Property [`SourceDBClusterIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-globalcluster.html#cfn-rds-globalcluster-sourcedbclusteridentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub source_db_cluster_identifier: Option<::Value<String>>,
    /// Property [`StorageEncrypted`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-globalcluster.html#cfn-rds-globalcluster-storageencrypted).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub storage_encrypted: Option<::Value<bool>>,
}

impl ::serde::Serialize for GlobalClusterProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref deletion_protection) = self.deletion_protection {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeletionProtection", deletion_protection)?;
        }
        if let Some(ref engine) = self.engine {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Engine", engine)?;
        }
        if let Some(ref engine_version) = self.engine_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EngineVersion", engine_version)?;
        }
        if let Some(ref global_cluster_identifier) = self.global_cluster_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GlobalClusterIdentifier", global_cluster_identifier)?;
        }
        if let Some(ref source_db_cluster_identifier) = self.source_db_cluster_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceDBClusterIdentifier", source_db_cluster_identifier)?;
        }
        if let Some(ref storage_encrypted) = self.storage_encrypted {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StorageEncrypted", storage_encrypted)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for GlobalClusterProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<GlobalClusterProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = GlobalClusterProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type GlobalClusterProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut deletion_protection: Option<::Value<bool>> = None;
                let mut engine: Option<::Value<String>> = None;
                let mut engine_version: Option<::Value<String>> = None;
                let mut global_cluster_identifier: Option<::Value<String>> = None;
                let mut source_db_cluster_identifier: Option<::Value<String>> = None;
                let mut storage_encrypted: Option<::Value<bool>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DeletionProtection" => {
                            deletion_protection = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Engine" => {
                            engine = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EngineVersion" => {
                            engine_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GlobalClusterIdentifier" => {
                            global_cluster_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceDBClusterIdentifier" => {
                            source_db_cluster_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StorageEncrypted" => {
                            storage_encrypted = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(GlobalClusterProperties {
                    deletion_protection: deletion_protection,
                    engine: engine,
                    engine_version: engine_version,
                    global_cluster_identifier: global_cluster_identifier,
                    source_db_cluster_identifier: source_db_cluster_identifier,
                    storage_encrypted: storage_encrypted,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for GlobalCluster {
    type Properties = GlobalClusterProperties;
    const TYPE: &'static str = "AWS::RDS::GlobalCluster";
    fn properties(&self) -> &GlobalClusterProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut GlobalClusterProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for GlobalCluster {}

impl From<GlobalClusterProperties> for GlobalCluster {
    fn from(properties: GlobalClusterProperties) -> GlobalCluster {
        GlobalCluster { properties }
    }
}

/// The [`AWS::RDS::Integration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-integration.html) resource type.
#[derive(Debug, Default)]
pub struct Integration {
    properties: IntegrationProperties
}

/// Properties for the `Integration` resource.
#[derive(Debug, Default)]
pub struct IntegrationProperties {
    /// Property [`AdditionalEncryptionContext`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-integration.html#cfn-rds-integration-additionalencryptioncontext).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub additional_encryption_context: Option<::ValueMap<String>>,
    /// Property [`IntegrationName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-integration.html#cfn-rds-integration-integrationname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub integration_name: Option<::Value<String>>,
    /// Property [`KMSKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-integration.html#cfn-rds-integration-kmskeyid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kms_key_id: Option<::Value<String>>,
    /// Property [`SourceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-integration.html#cfn-rds-integration-sourcearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub source_arn: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-integration.html#cfn-rds-integration-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TargetArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-integration.html#cfn-rds-integration-targetarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub target_arn: ::Value<String>,
}

impl ::serde::Serialize for IntegrationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref additional_encryption_context) = self.additional_encryption_context {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdditionalEncryptionContext", additional_encryption_context)?;
        }
        if let Some(ref integration_name) = self.integration_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntegrationName", integration_name)?;
        }
        if let Some(ref kms_key_id) = self.kms_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KMSKeyId", kms_key_id)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceArn", &self.source_arn)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetArn", &self.target_arn)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for IntegrationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<IntegrationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = IntegrationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type IntegrationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut additional_encryption_context: Option<::ValueMap<String>> = None;
                let mut integration_name: Option<::Value<String>> = None;
                let mut kms_key_id: Option<::Value<String>> = None;
                let mut source_arn: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut target_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AdditionalEncryptionContext" => {
                            additional_encryption_context = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IntegrationName" => {
                            integration_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KMSKeyId" => {
                            kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceArn" => {
                            source_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetArn" => {
                            target_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(IntegrationProperties {
                    additional_encryption_context: additional_encryption_context,
                    integration_name: integration_name,
                    kms_key_id: kms_key_id,
                    source_arn: source_arn.ok_or(::serde::de::Error::missing_field("SourceArn"))?,
                    tags: tags,
                    target_arn: target_arn.ok_or(::serde::de::Error::missing_field("TargetArn"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Integration {
    type Properties = IntegrationProperties;
    const TYPE: &'static str = "AWS::RDS::Integration";
    fn properties(&self) -> &IntegrationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut IntegrationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Integration {}

impl From<IntegrationProperties> for Integration {
    fn from(properties: IntegrationProperties) -> Integration {
        Integration { properties }
    }
}

/// The [`AWS::RDS::OptionGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-optiongroup.html) resource type.
#[derive(Debug, Default)]
pub struct OptionGroup {
    properties: OptionGroupProperties
}

/// Properties for the `OptionGroup` resource.
#[derive(Debug, Default)]
pub struct OptionGroupProperties {
    /// Property [`EngineName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-optiongroup.html#cfn-rds-optiongroup-enginename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub engine_name: ::Value<String>,
    /// Property [`MajorEngineVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-optiongroup.html#cfn-rds-optiongroup-majorengineversion).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub major_engine_version: ::Value<String>,
    /// Property [`OptionConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-optiongroup.html#cfn-rds-optiongroup-optionconfigurations).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub option_configurations: Option<::ValueList<self::option_group::OptionConfiguration>>,
    /// Property [`OptionGroupDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-optiongroup.html#cfn-rds-optiongroup-optiongroupdescription).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub option_group_description: ::Value<String>,
    /// Property [`OptionGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-optiongroup.html#cfn-rds-optiongroup-optiongroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub option_group_name: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-optiongroup.html#cfn-rds-optiongroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for OptionGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EngineName", &self.engine_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MajorEngineVersion", &self.major_engine_version)?;
        if let Some(ref option_configurations) = self.option_configurations {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OptionConfigurations", option_configurations)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "OptionGroupDescription", &self.option_group_description)?;
        if let Some(ref option_group_name) = self.option_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OptionGroupName", option_group_name)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for OptionGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<OptionGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = OptionGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type OptionGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut engine_name: Option<::Value<String>> = None;
                let mut major_engine_version: Option<::Value<String>> = None;
                let mut option_configurations: Option<::ValueList<self::option_group::OptionConfiguration>> = None;
                let mut option_group_description: Option<::Value<String>> = None;
                let mut option_group_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "EngineName" => {
                            engine_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MajorEngineVersion" => {
                            major_engine_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OptionConfigurations" => {
                            option_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OptionGroupDescription" => {
                            option_group_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OptionGroupName" => {
                            option_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(OptionGroupProperties {
                    engine_name: engine_name.ok_or(::serde::de::Error::missing_field("EngineName"))?,
                    major_engine_version: major_engine_version.ok_or(::serde::de::Error::missing_field("MajorEngineVersion"))?,
                    option_configurations: option_configurations,
                    option_group_description: option_group_description.ok_or(::serde::de::Error::missing_field("OptionGroupDescription"))?,
                    option_group_name: option_group_name,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for OptionGroup {
    type Properties = OptionGroupProperties;
    const TYPE: &'static str = "AWS::RDS::OptionGroup";
    fn properties(&self) -> &OptionGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut OptionGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for OptionGroup {}

impl From<OptionGroupProperties> for OptionGroup {
    fn from(properties: OptionGroupProperties) -> OptionGroup {
        OptionGroup { properties }
    }
}

pub mod db_cluster {
    //! Property types for the `DBCluster` resource.

    /// The [`AWS::RDS::DBCluster.DBClusterRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbcluster-dbclusterrole.html) property type.
    #[derive(Debug, Default)]
    pub struct DBClusterRole {
        /// Property [`FeatureName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbcluster-dbclusterrole.html#cfn-rds-dbcluster-dbclusterrole-featurename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub feature_name: Option<::Value<String>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbcluster-dbclusterrole.html#cfn-rds-dbcluster-dbclusterrole-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for DBClusterRole {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref feature_name) = self.feature_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FeatureName", feature_name)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DBClusterRole {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DBClusterRole, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DBClusterRole;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DBClusterRole")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut feature_name: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FeatureName" => {
                                feature_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DBClusterRole {
                        feature_name: feature_name,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::RDS::DBCluster.Endpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbcluster-endpoint.html) property type.
    #[derive(Debug, Default)]
    pub struct Endpoint {
        /// Property [`Address`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbcluster-endpoint.html#cfn-rds-dbcluster-endpoint-address).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub address: Option<::Value<String>>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbcluster-endpoint.html#cfn-rds-dbcluster-endpoint-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Endpoint {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref address) = self.address {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Address", address)?;
            }
            if let Some(ref port) = self.port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Endpoint {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Endpoint, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Endpoint;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Endpoint")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut address: Option<::Value<String>> = None;
                    let mut port: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Address" => {
                                address = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Endpoint {
                        address: address,
                        port: port,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::RDS::DBCluster.MasterUserSecret`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbcluster-masterusersecret.html) property type.
    #[derive(Debug, Default)]
    pub struct MasterUserSecret {
        /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbcluster-masterusersecret.html#cfn-rds-dbcluster-masterusersecret-kmskeyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kms_key_id: Option<::Value<String>>,
        /// Property [`SecretArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbcluster-masterusersecret.html#cfn-rds-dbcluster-masterusersecret-secretarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secret_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for MasterUserSecret {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref kms_key_id) = self.kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
            }
            if let Some(ref secret_arn) = self.secret_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretArn", secret_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MasterUserSecret {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MasterUserSecret, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MasterUserSecret;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MasterUserSecret")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut kms_key_id: Option<::Value<String>> = None;
                    let mut secret_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KmsKeyId" => {
                                kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretArn" => {
                                secret_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MasterUserSecret {
                        kms_key_id: kms_key_id,
                        secret_arn: secret_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::RDS::DBCluster.ReadEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbcluster-readendpoint.html) property type.
    #[derive(Debug, Default)]
    pub struct ReadEndpoint {
        /// Property [`Address`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbcluster-readendpoint.html#cfn-rds-dbcluster-readendpoint-address).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub address: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ReadEndpoint {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref address) = self.address {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Address", address)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ReadEndpoint {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ReadEndpoint, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ReadEndpoint;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ReadEndpoint")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut address: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Address" => {
                                address = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ReadEndpoint {
                        address: address,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::RDS::DBCluster.ScalingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbcluster-scalingconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ScalingConfiguration {
        /// Property [`AutoPause`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbcluster-scalingconfiguration.html#cfn-rds-dbcluster-scalingconfiguration-autopause).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub auto_pause: Option<::Value<bool>>,
        /// Property [`MaxCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbcluster-scalingconfiguration.html#cfn-rds-dbcluster-scalingconfiguration-maxcapacity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_capacity: Option<::Value<u32>>,
        /// Property [`MinCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbcluster-scalingconfiguration.html#cfn-rds-dbcluster-scalingconfiguration-mincapacity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min_capacity: Option<::Value<u32>>,
        /// Property [`SecondsBeforeTimeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbcluster-scalingconfiguration.html#cfn-rds-dbcluster-scalingconfiguration-secondsbeforetimeout).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub seconds_before_timeout: Option<::Value<u32>>,
        /// Property [`SecondsUntilAutoPause`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbcluster-scalingconfiguration.html#cfn-rds-dbcluster-scalingconfiguration-secondsuntilautopause).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub seconds_until_auto_pause: Option<::Value<u32>>,
        /// Property [`TimeoutAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbcluster-scalingconfiguration.html#cfn-rds-dbcluster-scalingconfiguration-timeoutaction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timeout_action: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ScalingConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref auto_pause) = self.auto_pause {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoPause", auto_pause)?;
            }
            if let Some(ref max_capacity) = self.max_capacity {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxCapacity", max_capacity)?;
            }
            if let Some(ref min_capacity) = self.min_capacity {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinCapacity", min_capacity)?;
            }
            if let Some(ref seconds_before_timeout) = self.seconds_before_timeout {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecondsBeforeTimeout", seconds_before_timeout)?;
            }
            if let Some(ref seconds_until_auto_pause) = self.seconds_until_auto_pause {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecondsUntilAutoPause", seconds_until_auto_pause)?;
            }
            if let Some(ref timeout_action) = self.timeout_action {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeoutAction", timeout_action)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ScalingConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ScalingConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ScalingConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ScalingConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut auto_pause: Option<::Value<bool>> = None;
                    let mut max_capacity: Option<::Value<u32>> = None;
                    let mut min_capacity: Option<::Value<u32>> = None;
                    let mut seconds_before_timeout: Option<::Value<u32>> = None;
                    let mut seconds_until_auto_pause: Option<::Value<u32>> = None;
                    let mut timeout_action: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AutoPause" => {
                                auto_pause = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxCapacity" => {
                                max_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinCapacity" => {
                                min_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecondsBeforeTimeout" => {
                                seconds_before_timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecondsUntilAutoPause" => {
                                seconds_until_auto_pause = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimeoutAction" => {
                                timeout_action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ScalingConfiguration {
                        auto_pause: auto_pause,
                        max_capacity: max_capacity,
                        min_capacity: min_capacity,
                        seconds_before_timeout: seconds_before_timeout,
                        seconds_until_auto_pause: seconds_until_auto_pause,
                        timeout_action: timeout_action,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::RDS::DBCluster.ServerlessV2ScalingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbcluster-serverlessv2scalingconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ServerlessV2ScalingConfiguration {
        /// Property [`MaxCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbcluster-serverlessv2scalingconfiguration.html#cfn-rds-dbcluster-serverlessv2scalingconfiguration-maxcapacity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_capacity: Option<::Value<f64>>,
        /// Property [`MinCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbcluster-serverlessv2scalingconfiguration.html#cfn-rds-dbcluster-serverlessv2scalingconfiguration-mincapacity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min_capacity: Option<::Value<f64>>,
    }

    impl ::codec::SerializeValue for ServerlessV2ScalingConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref max_capacity) = self.max_capacity {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxCapacity", max_capacity)?;
            }
            if let Some(ref min_capacity) = self.min_capacity {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinCapacity", min_capacity)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ServerlessV2ScalingConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ServerlessV2ScalingConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ServerlessV2ScalingConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ServerlessV2ScalingConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut max_capacity: Option<::Value<f64>> = None;
                    let mut min_capacity: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxCapacity" => {
                                max_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinCapacity" => {
                                min_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ServerlessV2ScalingConfiguration {
                        max_capacity: max_capacity,
                        min_capacity: min_capacity,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod db_instance {
    //! Property types for the `DBInstance` resource.

    /// The [`AWS::RDS::DBInstance.CertificateDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbinstance-certificatedetails.html) property type.
    #[derive(Debug, Default)]
    pub struct CertificateDetails {
        /// Property [`CAIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbinstance-certificatedetails.html#cfn-rds-dbinstance-certificatedetails-caidentifier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ca_identifier: Option<::Value<String>>,
        /// Property [`ValidTill`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbinstance-certificatedetails.html#cfn-rds-dbinstance-certificatedetails-validtill).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub valid_till: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CertificateDetails {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref ca_identifier) = self.ca_identifier {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CAIdentifier", ca_identifier)?;
            }
            if let Some(ref valid_till) = self.valid_till {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ValidTill", valid_till)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CertificateDetails {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CertificateDetails, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CertificateDetails;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CertificateDetails")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ca_identifier: Option<::Value<String>> = None;
                    let mut valid_till: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CAIdentifier" => {
                                ca_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ValidTill" => {
                                valid_till = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CertificateDetails {
                        ca_identifier: ca_identifier,
                        valid_till: valid_till,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::RDS::DBInstance.DBInstanceRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbinstance-dbinstancerole.html) property type.
    #[derive(Debug, Default)]
    pub struct DBInstanceRole {
        /// Property [`FeatureName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbinstance-dbinstancerole.html#cfn-rds-dbinstance-dbinstancerole-featurename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub feature_name: ::Value<String>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbinstance-dbinstancerole.html#cfn-rds-dbinstance-dbinstancerole-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for DBInstanceRole {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FeatureName", &self.feature_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DBInstanceRole {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DBInstanceRole, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DBInstanceRole;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DBInstanceRole")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut feature_name: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FeatureName" => {
                                feature_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DBInstanceRole {
                        feature_name: feature_name.ok_or(::serde::de::Error::missing_field("FeatureName"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::RDS::DBInstance.Endpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbinstance-endpoint.html) property type.
    #[derive(Debug, Default)]
    pub struct Endpoint {
        /// Property [`Address`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbinstance-endpoint.html#cfn-rds-dbinstance-endpoint-address).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub address: Option<::Value<String>>,
        /// Property [`HostedZoneId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbinstance-endpoint.html#cfn-rds-dbinstance-endpoint-hostedzoneid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hosted_zone_id: Option<::Value<String>>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbinstance-endpoint.html#cfn-rds-dbinstance-endpoint-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Endpoint {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref address) = self.address {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Address", address)?;
            }
            if let Some(ref hosted_zone_id) = self.hosted_zone_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HostedZoneId", hosted_zone_id)?;
            }
            if let Some(ref port) = self.port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Endpoint {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Endpoint, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Endpoint;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Endpoint")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut address: Option<::Value<String>> = None;
                    let mut hosted_zone_id: Option<::Value<String>> = None;
                    let mut port: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Address" => {
                                address = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HostedZoneId" => {
                                hosted_zone_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Endpoint {
                        address: address,
                        hosted_zone_id: hosted_zone_id,
                        port: port,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::RDS::DBInstance.MasterUserSecret`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbinstance-masterusersecret.html) property type.
    #[derive(Debug, Default)]
    pub struct MasterUserSecret {
        /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbinstance-masterusersecret.html#cfn-rds-dbinstance-masterusersecret-kmskeyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kms_key_id: Option<::Value<String>>,
        /// Property [`SecretArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbinstance-masterusersecret.html#cfn-rds-dbinstance-masterusersecret-secretarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secret_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for MasterUserSecret {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref kms_key_id) = self.kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
            }
            if let Some(ref secret_arn) = self.secret_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretArn", secret_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MasterUserSecret {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MasterUserSecret, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MasterUserSecret;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MasterUserSecret")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut kms_key_id: Option<::Value<String>> = None;
                    let mut secret_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KmsKeyId" => {
                                kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretArn" => {
                                secret_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MasterUserSecret {
                        kms_key_id: kms_key_id,
                        secret_arn: secret_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::RDS::DBInstance.ProcessorFeature`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbinstance-processorfeature.html) property type.
    #[derive(Debug, Default)]
    pub struct ProcessorFeature {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbinstance-processorfeature.html#cfn-rds-dbinstance-processorfeature-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbinstance-processorfeature.html#cfn-rds-dbinstance-processorfeature-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ProcessorFeature {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ProcessorFeature {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ProcessorFeature, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ProcessorFeature;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ProcessorFeature")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ProcessorFeature {
                        name: name,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod db_proxy {
    //! Property types for the `DBProxy` resource.

    /// The [`AWS::RDS::DBProxy.AuthFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbproxy-authformat.html) property type.
    #[derive(Debug, Default)]
    pub struct AuthFormat {
        /// Property [`AuthScheme`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbproxy-authformat.html#cfn-rds-dbproxy-authformat-authscheme).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub auth_scheme: Option<::Value<String>>,
        /// Property [`ClientPasswordAuthType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbproxy-authformat.html#cfn-rds-dbproxy-authformat-clientpasswordauthtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_password_auth_type: Option<::Value<String>>,
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbproxy-authformat.html#cfn-rds-dbproxy-authformat-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`IAMAuth`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbproxy-authformat.html#cfn-rds-dbproxy-authformat-iamauth).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub iam_auth: Option<::Value<String>>,
        /// Property [`SecretArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbproxy-authformat.html#cfn-rds-dbproxy-authformat-secretarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secret_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AuthFormat {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref auth_scheme) = self.auth_scheme {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthScheme", auth_scheme)?;
            }
            if let Some(ref client_password_auth_type) = self.client_password_auth_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientPasswordAuthType", client_password_auth_type)?;
            }
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            if let Some(ref iam_auth) = self.iam_auth {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IAMAuth", iam_auth)?;
            }
            if let Some(ref secret_arn) = self.secret_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretArn", secret_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AuthFormat {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AuthFormat, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AuthFormat;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AuthFormat")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut auth_scheme: Option<::Value<String>> = None;
                    let mut client_password_auth_type: Option<::Value<String>> = None;
                    let mut description: Option<::Value<String>> = None;
                    let mut iam_auth: Option<::Value<String>> = None;
                    let mut secret_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AuthScheme" => {
                                auth_scheme = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ClientPasswordAuthType" => {
                                client_password_auth_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IAMAuth" => {
                                iam_auth = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretArn" => {
                                secret_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AuthFormat {
                        auth_scheme: auth_scheme,
                        client_password_auth_type: client_password_auth_type,
                        description: description,
                        iam_auth: iam_auth,
                        secret_arn: secret_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::RDS::DBProxy.TagFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbproxy-tagformat.html) property type.
    #[derive(Debug, Default)]
    pub struct TagFormat {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbproxy-tagformat.html#cfn-rds-dbproxy-tagformat-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbproxy-tagformat.html#cfn-rds-dbproxy-tagformat-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for TagFormat {
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

    impl ::codec::DeserializeValue for TagFormat {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TagFormat, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TagFormat;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TagFormat")
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

                    Ok(TagFormat {
                        key: key,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod db_proxy_endpoint {
    //! Property types for the `DBProxyEndpoint` resource.

    /// The [`AWS::RDS::DBProxyEndpoint.TagFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbproxyendpoint-tagformat.html) property type.
    #[derive(Debug, Default)]
    pub struct TagFormat {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbproxyendpoint-tagformat.html#cfn-rds-dbproxyendpoint-tagformat-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbproxyendpoint-tagformat.html#cfn-rds-dbproxyendpoint-tagformat-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for TagFormat {
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

    impl ::codec::DeserializeValue for TagFormat {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TagFormat, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TagFormat;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TagFormat")
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

                    Ok(TagFormat {
                        key: key,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod db_proxy_target_group {
    //! Property types for the `DBProxyTargetGroup` resource.

    /// The [`AWS::RDS::DBProxyTargetGroup.ConnectionPoolConfigurationInfoFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbproxytargetgroup-connectionpoolconfigurationinfoformat.html) property type.
    #[derive(Debug, Default)]
    pub struct ConnectionPoolConfigurationInfoFormat {
        /// Property [`ConnectionBorrowTimeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbproxytargetgroup-connectionpoolconfigurationinfoformat.html#cfn-rds-dbproxytargetgroup-connectionpoolconfigurationinfoformat-connectionborrowtimeout).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connection_borrow_timeout: Option<::Value<u32>>,
        /// Property [`InitQuery`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbproxytargetgroup-connectionpoolconfigurationinfoformat.html#cfn-rds-dbproxytargetgroup-connectionpoolconfigurationinfoformat-initquery).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub init_query: Option<::Value<String>>,
        /// Property [`MaxConnectionsPercent`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbproxytargetgroup-connectionpoolconfigurationinfoformat.html#cfn-rds-dbproxytargetgroup-connectionpoolconfigurationinfoformat-maxconnectionspercent).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_connections_percent: Option<::Value<u32>>,
        /// Property [`MaxIdleConnectionsPercent`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbproxytargetgroup-connectionpoolconfigurationinfoformat.html#cfn-rds-dbproxytargetgroup-connectionpoolconfigurationinfoformat-maxidleconnectionspercent).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_idle_connections_percent: Option<::Value<u32>>,
        /// Property [`SessionPinningFilters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbproxytargetgroup-connectionpoolconfigurationinfoformat.html#cfn-rds-dbproxytargetgroup-connectionpoolconfigurationinfoformat-sessionpinningfilters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub session_pinning_filters: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for ConnectionPoolConfigurationInfoFormat {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref connection_borrow_timeout) = self.connection_borrow_timeout {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionBorrowTimeout", connection_borrow_timeout)?;
            }
            if let Some(ref init_query) = self.init_query {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InitQuery", init_query)?;
            }
            if let Some(ref max_connections_percent) = self.max_connections_percent {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxConnectionsPercent", max_connections_percent)?;
            }
            if let Some(ref max_idle_connections_percent) = self.max_idle_connections_percent {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxIdleConnectionsPercent", max_idle_connections_percent)?;
            }
            if let Some(ref session_pinning_filters) = self.session_pinning_filters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SessionPinningFilters", session_pinning_filters)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConnectionPoolConfigurationInfoFormat {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConnectionPoolConfigurationInfoFormat, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConnectionPoolConfigurationInfoFormat;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConnectionPoolConfigurationInfoFormat")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut connection_borrow_timeout: Option<::Value<u32>> = None;
                    let mut init_query: Option<::Value<String>> = None;
                    let mut max_connections_percent: Option<::Value<u32>> = None;
                    let mut max_idle_connections_percent: Option<::Value<u32>> = None;
                    let mut session_pinning_filters: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConnectionBorrowTimeout" => {
                                connection_borrow_timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InitQuery" => {
                                init_query = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxConnectionsPercent" => {
                                max_connections_percent = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxIdleConnectionsPercent" => {
                                max_idle_connections_percent = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SessionPinningFilters" => {
                                session_pinning_filters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConnectionPoolConfigurationInfoFormat {
                        connection_borrow_timeout: connection_borrow_timeout,
                        init_query: init_query,
                        max_connections_percent: max_connections_percent,
                        max_idle_connections_percent: max_idle_connections_percent,
                        session_pinning_filters: session_pinning_filters,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod db_security_group {
    //! Property types for the `DBSecurityGroup` resource.

    /// The [`AWS::RDS::DBSecurityGroup.Ingress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-security-group-rule.html) property type.
    #[derive(Debug, Default)]
    pub struct Ingress {
        /// Property [`CIDRIP`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-security-group-rule.html#cfn-rds-securitygroup-cidrip).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cidrip: Option<::Value<String>>,
        /// Property [`EC2SecurityGroupId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-security-group-rule.html#cfn-rds-securitygroup-ec2securitygroupid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub ec2_security_group_id: Option<::Value<String>>,
        /// Property [`EC2SecurityGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-security-group-rule.html#cfn-rds-securitygroup-ec2securitygroupname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub ec2_security_group_name: Option<::Value<String>>,
        /// Property [`EC2SecurityGroupOwnerId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-security-group-rule.html#cfn-rds-securitygroup-ec2securitygroupownerid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub ec2_security_group_owner_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Ingress {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cidrip) = self.cidrip {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CIDRIP", cidrip)?;
            }
            if let Some(ref ec2_security_group_id) = self.ec2_security_group_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EC2SecurityGroupId", ec2_security_group_id)?;
            }
            if let Some(ref ec2_security_group_name) = self.ec2_security_group_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EC2SecurityGroupName", ec2_security_group_name)?;
            }
            if let Some(ref ec2_security_group_owner_id) = self.ec2_security_group_owner_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EC2SecurityGroupOwnerId", ec2_security_group_owner_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Ingress {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Ingress, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Ingress;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Ingress")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cidrip: Option<::Value<String>> = None;
                    let mut ec2_security_group_id: Option<::Value<String>> = None;
                    let mut ec2_security_group_name: Option<::Value<String>> = None;
                    let mut ec2_security_group_owner_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CIDRIP" => {
                                cidrip = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EC2SecurityGroupId" => {
                                ec2_security_group_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EC2SecurityGroupName" => {
                                ec2_security_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EC2SecurityGroupOwnerId" => {
                                ec2_security_group_owner_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Ingress {
                        cidrip: cidrip,
                        ec2_security_group_id: ec2_security_group_id,
                        ec2_security_group_name: ec2_security_group_name,
                        ec2_security_group_owner_id: ec2_security_group_owner_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod option_group {
    //! Property types for the `OptionGroup` resource.

    /// The [`AWS::RDS::OptionGroup.OptionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-optiongroup-optionconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct OptionConfiguration {
        /// Property [`DBSecurityGroupMemberships`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-optiongroup-optionconfiguration.html#cfn-rds-optiongroup-optionconfiguration-dbsecuritygroupmemberships).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub db_security_group_memberships: Option<::ValueList<String>>,
        /// Property [`OptionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-optiongroup-optionconfiguration.html#cfn-rds-optiongroup-optionconfiguration-optionname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub option_name: ::Value<String>,
        /// Property [`OptionSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-optiongroup-optionconfiguration.html#cfn-rds-optiongroup-optionconfiguration-optionsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub option_settings: Option<::ValueList<OptionSetting>>,
        /// Property [`OptionVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-optiongroup-optionconfiguration.html#cfn-rds-optiongroup-optionconfiguration-optionversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub option_version: Option<::Value<String>>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-optiongroup-optionconfiguration.html#cfn-rds-optiongroup-optionconfiguration-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: Option<::Value<u32>>,
        /// Property [`VpcSecurityGroupMemberships`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-optiongroup-optionconfiguration.html#cfn-rds-optiongroup-optionconfiguration-vpcsecuritygroupmemberships).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vpc_security_group_memberships: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for OptionConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref db_security_group_memberships) = self.db_security_group_memberships {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBSecurityGroupMemberships", db_security_group_memberships)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OptionName", &self.option_name)?;
            if let Some(ref option_settings) = self.option_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OptionSettings", option_settings)?;
            }
            if let Some(ref option_version) = self.option_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OptionVersion", option_version)?;
            }
            if let Some(ref port) = self.port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
            }
            if let Some(ref vpc_security_group_memberships) = self.vpc_security_group_memberships {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcSecurityGroupMemberships", vpc_security_group_memberships)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OptionConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OptionConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OptionConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OptionConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut db_security_group_memberships: Option<::ValueList<String>> = None;
                    let mut option_name: Option<::Value<String>> = None;
                    let mut option_settings: Option<::ValueList<OptionSetting>> = None;
                    let mut option_version: Option<::Value<String>> = None;
                    let mut port: Option<::Value<u32>> = None;
                    let mut vpc_security_group_memberships: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DBSecurityGroupMemberships" => {
                                db_security_group_memberships = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OptionName" => {
                                option_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OptionSettings" => {
                                option_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OptionVersion" => {
                                option_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpcSecurityGroupMemberships" => {
                                vpc_security_group_memberships = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OptionConfiguration {
                        db_security_group_memberships: db_security_group_memberships,
                        option_name: option_name.ok_or(::serde::de::Error::missing_field("OptionName"))?,
                        option_settings: option_settings,
                        option_version: option_version,
                        port: port,
                        vpc_security_group_memberships: vpc_security_group_memberships,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::RDS::OptionGroup.OptionSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-optiongroup-optionsetting.html) property type.
    #[derive(Debug, Default)]
    pub struct OptionSetting {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-optiongroup-optionsetting.html#cfn-rds-optiongroup-optionsetting-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-optiongroup-optionsetting.html#cfn-rds-optiongroup-optionsetting-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for OptionSetting {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OptionSetting {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OptionSetting, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OptionSetting;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OptionSetting")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OptionSetting {
                        name: name,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
