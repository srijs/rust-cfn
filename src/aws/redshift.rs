//! Types for the `Redshift` service.

/// The [`AWS::Redshift::Cluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html) resource type.
#[derive(Debug, Default)]
pub struct Cluster {
    properties: ClusterProperties
}

/// Properties for the `Cluster` resource.
#[derive(Debug, Default)]
pub struct ClusterProperties {
    /// Property [`AllowVersionUpgrade`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-allowversionupgrade).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub allow_version_upgrade: Option<::Value<bool>>,
    /// Property [`AquaConfigurationStatus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-aquaconfigurationstatus).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub aqua_configuration_status: Option<::Value<String>>,
    /// Property [`AutomatedSnapshotRetentionPeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-automatedsnapshotretentionperiod).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub automated_snapshot_retention_period: Option<::Value<u32>>,
    /// Property [`AvailabilityZone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-availabilityzone).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub availability_zone: Option<::Value<String>>,
    /// Property [`AvailabilityZoneRelocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-availabilityzonerelocation).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub availability_zone_relocation: Option<::Value<bool>>,
    /// Property [`AvailabilityZoneRelocationStatus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-availabilityzonerelocationstatus).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub availability_zone_relocation_status: Option<::Value<String>>,
    /// Property [`Classic`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-classic).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub classic: Option<::Value<bool>>,
    /// Property [`ClusterIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-clusteridentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub cluster_identifier: Option<::Value<String>>,
    /// Property [`ClusterParameterGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-clusterparametergroupname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cluster_parameter_group_name: Option<::Value<String>>,
    /// Property [`ClusterSecurityGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-clustersecuritygroups).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cluster_security_groups: Option<::ValueList<String>>,
    /// Property [`ClusterSubnetGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-clustersubnetgroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub cluster_subnet_group_name: Option<::Value<String>>,
    /// Property [`ClusterType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-clustertype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cluster_type: ::Value<String>,
    /// Property [`ClusterVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-clusterversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cluster_version: Option<::Value<String>>,
    /// Property [`DBName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-dbname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub db_name: ::Value<String>,
    /// Property [`DeferMaintenance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-defermaintenance).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub defer_maintenance: Option<::Value<bool>>,
    /// Property [`DeferMaintenanceDuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-defermaintenanceduration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub defer_maintenance_duration: Option<::Value<u32>>,
    /// Property [`DeferMaintenanceEndTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-defermaintenanceendtime).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub defer_maintenance_end_time: Option<::Value<String>>,
    /// Property [`DeferMaintenanceStartTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-defermaintenancestarttime).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub defer_maintenance_start_time: Option<::Value<String>>,
    /// Property [`DestinationRegion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-destinationregion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub destination_region: Option<::Value<String>>,
    /// Property [`ElasticIp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-elasticip).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub elastic_ip: Option<::Value<String>>,
    /// Property [`Encrypted`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-encrypted).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub encrypted: Option<::Value<bool>>,
    /// Property [`Endpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-endpoint).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub endpoint: Option<::Value<self::cluster::Endpoint>>,
    /// Property [`EnhancedVpcRouting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-enhancedvpcrouting).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enhanced_vpc_routing: Option<::Value<bool>>,
    /// Property [`HsmClientCertificateIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-hsmclientcertificateidentifier).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub hsm_client_certificate_identifier: Option<::Value<String>>,
    /// Property [`HsmConfigurationIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-hsmconfigurationidentifier).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub hsm_configuration_identifier: Option<::Value<String>>,
    /// Property [`IamRoles`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-iamroles).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub iam_roles: Option<::ValueList<String>>,
    /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-kmskeyid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub kms_key_id: Option<::Value<String>>,
    /// Property [`LoggingProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-loggingproperties).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub logging_properties: Option<::Value<self::cluster::LoggingProperties>>,
    /// Property [`MaintenanceTrackName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-maintenancetrackname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub maintenance_track_name: Option<::Value<String>>,
    /// Property [`ManageMasterPassword`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-managemasterpassword).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub manage_master_password: Option<::Value<bool>>,
    /// Property [`ManualSnapshotRetentionPeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-manualsnapshotretentionperiod).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub manual_snapshot_retention_period: Option<::Value<u32>>,
    /// Property [`MasterPasswordSecretKmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-masterpasswordsecretkmskeyid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub master_password_secret_kms_key_id: Option<::Value<String>>,
    /// Property [`MasterUserPassword`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-masteruserpassword).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub master_user_password: Option<::Value<String>>,
    /// Property [`MasterUsername`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-masterusername).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub master_username: ::Value<String>,
    /// Property [`MultiAZ`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-multiaz).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub multi_az: Option<::Value<bool>>,
    /// Property [`NamespaceResourcePolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-namespaceresourcepolicy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub namespace_resource_policy: Option<::Value<::json::Value>>,
    /// Property [`NodeType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-nodetype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub node_type: ::Value<String>,
    /// Property [`NumberOfNodes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-numberofnodes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub number_of_nodes: Option<::Value<u32>>,
    /// Property [`OwnerAccount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-owneraccount).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub owner_account: Option<::Value<String>>,
    /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-port).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub port: Option<::Value<u32>>,
    /// Property [`PreferredMaintenanceWindow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-preferredmaintenancewindow).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub preferred_maintenance_window: Option<::Value<String>>,
    /// Property [`PubliclyAccessible`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-publiclyaccessible).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub publicly_accessible: Option<::Value<bool>>,
    /// Property [`ResourceAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-resourceaction).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub resource_action: Option<::Value<String>>,
    /// Property [`RevisionTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-revisiontarget).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub revision_target: Option<::Value<String>>,
    /// Property [`RotateEncryptionKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-rotateencryptionkey).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub rotate_encryption_key: Option<::Value<bool>>,
    /// Property [`SnapshotClusterIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-snapshotclusteridentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub snapshot_cluster_identifier: Option<::Value<String>>,
    /// Property [`SnapshotCopyGrantName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-snapshotcopygrantname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub snapshot_copy_grant_name: Option<::Value<String>>,
    /// Property [`SnapshotCopyManual`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-snapshotcopymanual).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub snapshot_copy_manual: Option<::Value<bool>>,
    /// Property [`SnapshotCopyRetentionPeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-snapshotcopyretentionperiod).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub snapshot_copy_retention_period: Option<::Value<u32>>,
    /// Property [`SnapshotIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-snapshotidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub snapshot_identifier: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`VpcSecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html#cfn-redshift-cluster-vpcsecuritygroupids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub vpc_security_group_ids: Option<::ValueList<String>>,
}

impl ::serde::Serialize for ClusterProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref allow_version_upgrade) = self.allow_version_upgrade {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowVersionUpgrade", allow_version_upgrade)?;
        }
        if let Some(ref aqua_configuration_status) = self.aqua_configuration_status {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AquaConfigurationStatus", aqua_configuration_status)?;
        }
        if let Some(ref automated_snapshot_retention_period) = self.automated_snapshot_retention_period {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutomatedSnapshotRetentionPeriod", automated_snapshot_retention_period)?;
        }
        if let Some(ref availability_zone) = self.availability_zone {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZone", availability_zone)?;
        }
        if let Some(ref availability_zone_relocation) = self.availability_zone_relocation {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZoneRelocation", availability_zone_relocation)?;
        }
        if let Some(ref availability_zone_relocation_status) = self.availability_zone_relocation_status {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZoneRelocationStatus", availability_zone_relocation_status)?;
        }
        if let Some(ref classic) = self.classic {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Classic", classic)?;
        }
        if let Some(ref cluster_identifier) = self.cluster_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterIdentifier", cluster_identifier)?;
        }
        if let Some(ref cluster_parameter_group_name) = self.cluster_parameter_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterParameterGroupName", cluster_parameter_group_name)?;
        }
        if let Some(ref cluster_security_groups) = self.cluster_security_groups {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterSecurityGroups", cluster_security_groups)?;
        }
        if let Some(ref cluster_subnet_group_name) = self.cluster_subnet_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterSubnetGroupName", cluster_subnet_group_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterType", &self.cluster_type)?;
        if let Some(ref cluster_version) = self.cluster_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterVersion", cluster_version)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBName", &self.db_name)?;
        if let Some(ref defer_maintenance) = self.defer_maintenance {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeferMaintenance", defer_maintenance)?;
        }
        if let Some(ref defer_maintenance_duration) = self.defer_maintenance_duration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeferMaintenanceDuration", defer_maintenance_duration)?;
        }
        if let Some(ref defer_maintenance_end_time) = self.defer_maintenance_end_time {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeferMaintenanceEndTime", defer_maintenance_end_time)?;
        }
        if let Some(ref defer_maintenance_start_time) = self.defer_maintenance_start_time {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeferMaintenanceStartTime", defer_maintenance_start_time)?;
        }
        if let Some(ref destination_region) = self.destination_region {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationRegion", destination_region)?;
        }
        if let Some(ref elastic_ip) = self.elastic_ip {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ElasticIp", elastic_ip)?;
        }
        if let Some(ref encrypted) = self.encrypted {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Encrypted", encrypted)?;
        }
        if let Some(ref endpoint) = self.endpoint {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Endpoint", endpoint)?;
        }
        if let Some(ref enhanced_vpc_routing) = self.enhanced_vpc_routing {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnhancedVpcRouting", enhanced_vpc_routing)?;
        }
        if let Some(ref hsm_client_certificate_identifier) = self.hsm_client_certificate_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HsmClientCertificateIdentifier", hsm_client_certificate_identifier)?;
        }
        if let Some(ref hsm_configuration_identifier) = self.hsm_configuration_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HsmConfigurationIdentifier", hsm_configuration_identifier)?;
        }
        if let Some(ref iam_roles) = self.iam_roles {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IamRoles", iam_roles)?;
        }
        if let Some(ref kms_key_id) = self.kms_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
        }
        if let Some(ref logging_properties) = self.logging_properties {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoggingProperties", logging_properties)?;
        }
        if let Some(ref maintenance_track_name) = self.maintenance_track_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaintenanceTrackName", maintenance_track_name)?;
        }
        if let Some(ref manage_master_password) = self.manage_master_password {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManageMasterPassword", manage_master_password)?;
        }
        if let Some(ref manual_snapshot_retention_period) = self.manual_snapshot_retention_period {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManualSnapshotRetentionPeriod", manual_snapshot_retention_period)?;
        }
        if let Some(ref master_password_secret_kms_key_id) = self.master_password_secret_kms_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MasterPasswordSecretKmsKeyId", master_password_secret_kms_key_id)?;
        }
        if let Some(ref master_user_password) = self.master_user_password {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MasterUserPassword", master_user_password)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MasterUsername", &self.master_username)?;
        if let Some(ref multi_az) = self.multi_az {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MultiAZ", multi_az)?;
        }
        if let Some(ref namespace_resource_policy) = self.namespace_resource_policy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NamespaceResourcePolicy", namespace_resource_policy)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "NodeType", &self.node_type)?;
        if let Some(ref number_of_nodes) = self.number_of_nodes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumberOfNodes", number_of_nodes)?;
        }
        if let Some(ref owner_account) = self.owner_account {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OwnerAccount", owner_account)?;
        }
        if let Some(ref port) = self.port {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
        }
        if let Some(ref preferred_maintenance_window) = self.preferred_maintenance_window {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredMaintenanceWindow", preferred_maintenance_window)?;
        }
        if let Some(ref publicly_accessible) = self.publicly_accessible {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PubliclyAccessible", publicly_accessible)?;
        }
        if let Some(ref resource_action) = self.resource_action {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceAction", resource_action)?;
        }
        if let Some(ref revision_target) = self.revision_target {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RevisionTarget", revision_target)?;
        }
        if let Some(ref rotate_encryption_key) = self.rotate_encryption_key {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RotateEncryptionKey", rotate_encryption_key)?;
        }
        if let Some(ref snapshot_cluster_identifier) = self.snapshot_cluster_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotClusterIdentifier", snapshot_cluster_identifier)?;
        }
        if let Some(ref snapshot_copy_grant_name) = self.snapshot_copy_grant_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotCopyGrantName", snapshot_copy_grant_name)?;
        }
        if let Some(ref snapshot_copy_manual) = self.snapshot_copy_manual {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotCopyManual", snapshot_copy_manual)?;
        }
        if let Some(ref snapshot_copy_retention_period) = self.snapshot_copy_retention_period {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotCopyRetentionPeriod", snapshot_copy_retention_period)?;
        }
        if let Some(ref snapshot_identifier) = self.snapshot_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotIdentifier", snapshot_identifier)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref vpc_security_group_ids) = self.vpc_security_group_ids {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcSecurityGroupIds", vpc_security_group_ids)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ClusterProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ClusterProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ClusterProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ClusterProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut allow_version_upgrade: Option<::Value<bool>> = None;
                let mut aqua_configuration_status: Option<::Value<String>> = None;
                let mut automated_snapshot_retention_period: Option<::Value<u32>> = None;
                let mut availability_zone: Option<::Value<String>> = None;
                let mut availability_zone_relocation: Option<::Value<bool>> = None;
                let mut availability_zone_relocation_status: Option<::Value<String>> = None;
                let mut classic: Option<::Value<bool>> = None;
                let mut cluster_identifier: Option<::Value<String>> = None;
                let mut cluster_parameter_group_name: Option<::Value<String>> = None;
                let mut cluster_security_groups: Option<::ValueList<String>> = None;
                let mut cluster_subnet_group_name: Option<::Value<String>> = None;
                let mut cluster_type: Option<::Value<String>> = None;
                let mut cluster_version: Option<::Value<String>> = None;
                let mut db_name: Option<::Value<String>> = None;
                let mut defer_maintenance: Option<::Value<bool>> = None;
                let mut defer_maintenance_duration: Option<::Value<u32>> = None;
                let mut defer_maintenance_end_time: Option<::Value<String>> = None;
                let mut defer_maintenance_start_time: Option<::Value<String>> = None;
                let mut destination_region: Option<::Value<String>> = None;
                let mut elastic_ip: Option<::Value<String>> = None;
                let mut encrypted: Option<::Value<bool>> = None;
                let mut endpoint: Option<::Value<self::cluster::Endpoint>> = None;
                let mut enhanced_vpc_routing: Option<::Value<bool>> = None;
                let mut hsm_client_certificate_identifier: Option<::Value<String>> = None;
                let mut hsm_configuration_identifier: Option<::Value<String>> = None;
                let mut iam_roles: Option<::ValueList<String>> = None;
                let mut kms_key_id: Option<::Value<String>> = None;
                let mut logging_properties: Option<::Value<self::cluster::LoggingProperties>> = None;
                let mut maintenance_track_name: Option<::Value<String>> = None;
                let mut manage_master_password: Option<::Value<bool>> = None;
                let mut manual_snapshot_retention_period: Option<::Value<u32>> = None;
                let mut master_password_secret_kms_key_id: Option<::Value<String>> = None;
                let mut master_user_password: Option<::Value<String>> = None;
                let mut master_username: Option<::Value<String>> = None;
                let mut multi_az: Option<::Value<bool>> = None;
                let mut namespace_resource_policy: Option<::Value<::json::Value>> = None;
                let mut node_type: Option<::Value<String>> = None;
                let mut number_of_nodes: Option<::Value<u32>> = None;
                let mut owner_account: Option<::Value<String>> = None;
                let mut port: Option<::Value<u32>> = None;
                let mut preferred_maintenance_window: Option<::Value<String>> = None;
                let mut publicly_accessible: Option<::Value<bool>> = None;
                let mut resource_action: Option<::Value<String>> = None;
                let mut revision_target: Option<::Value<String>> = None;
                let mut rotate_encryption_key: Option<::Value<bool>> = None;
                let mut snapshot_cluster_identifier: Option<::Value<String>> = None;
                let mut snapshot_copy_grant_name: Option<::Value<String>> = None;
                let mut snapshot_copy_manual: Option<::Value<bool>> = None;
                let mut snapshot_copy_retention_period: Option<::Value<u32>> = None;
                let mut snapshot_identifier: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut vpc_security_group_ids: Option<::ValueList<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AllowVersionUpgrade" => {
                            allow_version_upgrade = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AquaConfigurationStatus" => {
                            aqua_configuration_status = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AutomatedSnapshotRetentionPeriod" => {
                            automated_snapshot_retention_period = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AvailabilityZone" => {
                            availability_zone = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AvailabilityZoneRelocation" => {
                            availability_zone_relocation = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AvailabilityZoneRelocationStatus" => {
                            availability_zone_relocation_status = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Classic" => {
                            classic = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClusterIdentifier" => {
                            cluster_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClusterParameterGroupName" => {
                            cluster_parameter_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClusterSecurityGroups" => {
                            cluster_security_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClusterSubnetGroupName" => {
                            cluster_subnet_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClusterType" => {
                            cluster_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClusterVersion" => {
                            cluster_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DBName" => {
                            db_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DeferMaintenance" => {
                            defer_maintenance = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DeferMaintenanceDuration" => {
                            defer_maintenance_duration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DeferMaintenanceEndTime" => {
                            defer_maintenance_end_time = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DeferMaintenanceStartTime" => {
                            defer_maintenance_start_time = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DestinationRegion" => {
                            destination_region = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ElasticIp" => {
                            elastic_ip = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Encrypted" => {
                            encrypted = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Endpoint" => {
                            endpoint = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnhancedVpcRouting" => {
                            enhanced_vpc_routing = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HsmClientCertificateIdentifier" => {
                            hsm_client_certificate_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HsmConfigurationIdentifier" => {
                            hsm_configuration_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IamRoles" => {
                            iam_roles = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsKeyId" => {
                            kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LoggingProperties" => {
                            logging_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaintenanceTrackName" => {
                            maintenance_track_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ManageMasterPassword" => {
                            manage_master_password = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ManualSnapshotRetentionPeriod" => {
                            manual_snapshot_retention_period = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MasterPasswordSecretKmsKeyId" => {
                            master_password_secret_kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MasterUserPassword" => {
                            master_user_password = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MasterUsername" => {
                            master_username = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MultiAZ" => {
                            multi_az = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NamespaceResourcePolicy" => {
                            namespace_resource_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NodeType" => {
                            node_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NumberOfNodes" => {
                            number_of_nodes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OwnerAccount" => {
                            owner_account = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Port" => {
                            port = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PreferredMaintenanceWindow" => {
                            preferred_maintenance_window = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PubliclyAccessible" => {
                            publicly_accessible = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourceAction" => {
                            resource_action = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RevisionTarget" => {
                            revision_target = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RotateEncryptionKey" => {
                            rotate_encryption_key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SnapshotClusterIdentifier" => {
                            snapshot_cluster_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SnapshotCopyGrantName" => {
                            snapshot_copy_grant_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SnapshotCopyManual" => {
                            snapshot_copy_manual = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SnapshotCopyRetentionPeriod" => {
                            snapshot_copy_retention_period = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SnapshotIdentifier" => {
                            snapshot_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcSecurityGroupIds" => {
                            vpc_security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ClusterProperties {
                    allow_version_upgrade: allow_version_upgrade,
                    aqua_configuration_status: aqua_configuration_status,
                    automated_snapshot_retention_period: automated_snapshot_retention_period,
                    availability_zone: availability_zone,
                    availability_zone_relocation: availability_zone_relocation,
                    availability_zone_relocation_status: availability_zone_relocation_status,
                    classic: classic,
                    cluster_identifier: cluster_identifier,
                    cluster_parameter_group_name: cluster_parameter_group_name,
                    cluster_security_groups: cluster_security_groups,
                    cluster_subnet_group_name: cluster_subnet_group_name,
                    cluster_type: cluster_type.ok_or(::serde::de::Error::missing_field("ClusterType"))?,
                    cluster_version: cluster_version,
                    db_name: db_name.ok_or(::serde::de::Error::missing_field("DBName"))?,
                    defer_maintenance: defer_maintenance,
                    defer_maintenance_duration: defer_maintenance_duration,
                    defer_maintenance_end_time: defer_maintenance_end_time,
                    defer_maintenance_start_time: defer_maintenance_start_time,
                    destination_region: destination_region,
                    elastic_ip: elastic_ip,
                    encrypted: encrypted,
                    endpoint: endpoint,
                    enhanced_vpc_routing: enhanced_vpc_routing,
                    hsm_client_certificate_identifier: hsm_client_certificate_identifier,
                    hsm_configuration_identifier: hsm_configuration_identifier,
                    iam_roles: iam_roles,
                    kms_key_id: kms_key_id,
                    logging_properties: logging_properties,
                    maintenance_track_name: maintenance_track_name,
                    manage_master_password: manage_master_password,
                    manual_snapshot_retention_period: manual_snapshot_retention_period,
                    master_password_secret_kms_key_id: master_password_secret_kms_key_id,
                    master_user_password: master_user_password,
                    master_username: master_username.ok_or(::serde::de::Error::missing_field("MasterUsername"))?,
                    multi_az: multi_az,
                    namespace_resource_policy: namespace_resource_policy,
                    node_type: node_type.ok_or(::serde::de::Error::missing_field("NodeType"))?,
                    number_of_nodes: number_of_nodes,
                    owner_account: owner_account,
                    port: port,
                    preferred_maintenance_window: preferred_maintenance_window,
                    publicly_accessible: publicly_accessible,
                    resource_action: resource_action,
                    revision_target: revision_target,
                    rotate_encryption_key: rotate_encryption_key,
                    snapshot_cluster_identifier: snapshot_cluster_identifier,
                    snapshot_copy_grant_name: snapshot_copy_grant_name,
                    snapshot_copy_manual: snapshot_copy_manual,
                    snapshot_copy_retention_period: snapshot_copy_retention_period,
                    snapshot_identifier: snapshot_identifier,
                    tags: tags,
                    vpc_security_group_ids: vpc_security_group_ids,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Cluster {
    type Properties = ClusterProperties;
    const TYPE: &'static str = "AWS::Redshift::Cluster";
    fn properties(&self) -> &ClusterProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ClusterProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Cluster {}

impl From<ClusterProperties> for Cluster {
    fn from(properties: ClusterProperties) -> Cluster {
        Cluster { properties }
    }
}

/// The [`AWS::Redshift::ClusterParameterGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clusterparametergroup.html) resource type.
#[derive(Debug, Default)]
pub struct ClusterParameterGroup {
    properties: ClusterParameterGroupProperties
}

/// Properties for the `ClusterParameterGroup` resource.
#[derive(Debug, Default)]
pub struct ClusterParameterGroupProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clusterparametergroup.html#cfn-redshift-clusterparametergroup-description).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub description: ::Value<String>,
    /// Property [`ParameterGroupFamily`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clusterparametergroup.html#cfn-redshift-clusterparametergroup-parametergroupfamily).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub parameter_group_family: ::Value<String>,
    /// Property [`ParameterGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clusterparametergroup.html#cfn-redshift-clusterparametergroup-parametergroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub parameter_group_name: Option<::Value<String>>,
    /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clusterparametergroup.html#cfn-redshift-clusterparametergroup-parameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub parameters: Option<::ValueList<self::cluster_parameter_group::Parameter>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clusterparametergroup.html#cfn-redshift-clusterparametergroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ClusterParameterGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParameterGroupFamily", &self.parameter_group_family)?;
        if let Some(ref parameter_group_name) = self.parameter_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParameterGroupName", parameter_group_name)?;
        }
        if let Some(ref parameters) = self.parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", parameters)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ClusterParameterGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ClusterParameterGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ClusterParameterGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ClusterParameterGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut parameter_group_family: Option<::Value<String>> = None;
                let mut parameter_group_name: Option<::Value<String>> = None;
                let mut parameters: Option<::ValueList<self::cluster_parameter_group::Parameter>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ParameterGroupFamily" => {
                            parameter_group_family = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ParameterGroupName" => {
                            parameter_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
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

                Ok(ClusterParameterGroupProperties {
                    description: description.ok_or(::serde::de::Error::missing_field("Description"))?,
                    parameter_group_family: parameter_group_family.ok_or(::serde::de::Error::missing_field("ParameterGroupFamily"))?,
                    parameter_group_name: parameter_group_name,
                    parameters: parameters,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ClusterParameterGroup {
    type Properties = ClusterParameterGroupProperties;
    const TYPE: &'static str = "AWS::Redshift::ClusterParameterGroup";
    fn properties(&self) -> &ClusterParameterGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ClusterParameterGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ClusterParameterGroup {}

impl From<ClusterParameterGroupProperties> for ClusterParameterGroup {
    fn from(properties: ClusterParameterGroupProperties) -> ClusterParameterGroup {
        ClusterParameterGroup { properties }
    }
}

/// The [`AWS::Redshift::ClusterSecurityGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clustersecuritygroup.html) resource type.
#[derive(Debug, Default)]
pub struct ClusterSecurityGroup {
    properties: ClusterSecurityGroupProperties
}

/// Properties for the `ClusterSecurityGroup` resource.
#[derive(Debug, Default)]
pub struct ClusterSecurityGroupProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clustersecuritygroup.html#cfn-redshift-clustersecuritygroup-description).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub description: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clustersecuritygroup.html#cfn-redshift-clustersecuritygroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ClusterSecurityGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ClusterSecurityGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ClusterSecurityGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ClusterSecurityGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ClusterSecurityGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ClusterSecurityGroupProperties {
                    description: description.ok_or(::serde::de::Error::missing_field("Description"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ClusterSecurityGroup {
    type Properties = ClusterSecurityGroupProperties;
    const TYPE: &'static str = "AWS::Redshift::ClusterSecurityGroup";
    fn properties(&self) -> &ClusterSecurityGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ClusterSecurityGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ClusterSecurityGroup {}

impl From<ClusterSecurityGroupProperties> for ClusterSecurityGroup {
    fn from(properties: ClusterSecurityGroupProperties) -> ClusterSecurityGroup {
        ClusterSecurityGroup { properties }
    }
}

/// The [`AWS::Redshift::ClusterSecurityGroupIngress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clustersecuritygroupingress.html) resource type.
#[derive(Debug, Default)]
pub struct ClusterSecurityGroupIngress {
    properties: ClusterSecurityGroupIngressProperties
}

/// Properties for the `ClusterSecurityGroupIngress` resource.
#[derive(Debug, Default)]
pub struct ClusterSecurityGroupIngressProperties {
    /// Property [`CIDRIP`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clustersecuritygroupingress.html#cfn-redshift-clustersecuritygroupingress-cidrip).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub cidrip: Option<::Value<String>>,
    /// Property [`ClusterSecurityGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clustersecuritygroupingress.html#cfn-redshift-clustersecuritygroupingress-clustersecuritygroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub cluster_security_group_name: ::Value<String>,
    /// Property [`EC2SecurityGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clustersecuritygroupingress.html#cfn-redshift-clustersecuritygroupingress-ec2securitygroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub ec2_security_group_name: Option<::Value<String>>,
    /// Property [`EC2SecurityGroupOwnerId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clustersecuritygroupingress.html#cfn-redshift-clustersecuritygroupingress-ec2securitygroupownerid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub ec2_security_group_owner_id: Option<::Value<String>>,
}

impl ::serde::Serialize for ClusterSecurityGroupIngressProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref cidrip) = self.cidrip {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CIDRIP", cidrip)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterSecurityGroupName", &self.cluster_security_group_name)?;
        if let Some(ref ec2_security_group_name) = self.ec2_security_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EC2SecurityGroupName", ec2_security_group_name)?;
        }
        if let Some(ref ec2_security_group_owner_id) = self.ec2_security_group_owner_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EC2SecurityGroupOwnerId", ec2_security_group_owner_id)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ClusterSecurityGroupIngressProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ClusterSecurityGroupIngressProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ClusterSecurityGroupIngressProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ClusterSecurityGroupIngressProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut cidrip: Option<::Value<String>> = None;
                let mut cluster_security_group_name: Option<::Value<String>> = None;
                let mut ec2_security_group_name: Option<::Value<String>> = None;
                let mut ec2_security_group_owner_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CIDRIP" => {
                            cidrip = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClusterSecurityGroupName" => {
                            cluster_security_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
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

                Ok(ClusterSecurityGroupIngressProperties {
                    cidrip: cidrip,
                    cluster_security_group_name: cluster_security_group_name.ok_or(::serde::de::Error::missing_field("ClusterSecurityGroupName"))?,
                    ec2_security_group_name: ec2_security_group_name,
                    ec2_security_group_owner_id: ec2_security_group_owner_id,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ClusterSecurityGroupIngress {
    type Properties = ClusterSecurityGroupIngressProperties;
    const TYPE: &'static str = "AWS::Redshift::ClusterSecurityGroupIngress";
    fn properties(&self) -> &ClusterSecurityGroupIngressProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ClusterSecurityGroupIngressProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ClusterSecurityGroupIngress {}

impl From<ClusterSecurityGroupIngressProperties> for ClusterSecurityGroupIngress {
    fn from(properties: ClusterSecurityGroupIngressProperties) -> ClusterSecurityGroupIngress {
        ClusterSecurityGroupIngress { properties }
    }
}

/// The [`AWS::Redshift::ClusterSubnetGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clustersubnetgroup.html) resource type.
#[derive(Debug, Default)]
pub struct ClusterSubnetGroup {
    properties: ClusterSubnetGroupProperties
}

/// Properties for the `ClusterSubnetGroup` resource.
#[derive(Debug, Default)]
pub struct ClusterSubnetGroupProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clustersubnetgroup.html#cfn-redshift-clustersubnetgroup-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: ::Value<String>,
    /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clustersubnetgroup.html#cfn-redshift-clustersubnetgroup-subnetids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub subnet_ids: ::ValueList<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clustersubnetgroup.html#cfn-redshift-clustersubnetgroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ClusterSubnetGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", &self.subnet_ids)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ClusterSubnetGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ClusterSubnetGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ClusterSubnetGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ClusterSubnetGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut subnet_ids: Option<::ValueList<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
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

                Ok(ClusterSubnetGroupProperties {
                    description: description.ok_or(::serde::de::Error::missing_field("Description"))?,
                    subnet_ids: subnet_ids.ok_or(::serde::de::Error::missing_field("SubnetIds"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ClusterSubnetGroup {
    type Properties = ClusterSubnetGroupProperties;
    const TYPE: &'static str = "AWS::Redshift::ClusterSubnetGroup";
    fn properties(&self) -> &ClusterSubnetGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ClusterSubnetGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ClusterSubnetGroup {}

impl From<ClusterSubnetGroupProperties> for ClusterSubnetGroup {
    fn from(properties: ClusterSubnetGroupProperties) -> ClusterSubnetGroup {
        ClusterSubnetGroup { properties }
    }
}

/// The [`AWS::Redshift::EndpointAccess`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-endpointaccess.html) resource type.
#[derive(Debug, Default)]
pub struct EndpointAccess {
    properties: EndpointAccessProperties
}

/// Properties for the `EndpointAccess` resource.
#[derive(Debug, Default)]
pub struct EndpointAccessProperties {
    /// Property [`ClusterIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-endpointaccess.html#cfn-redshift-endpointaccess-clusteridentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub cluster_identifier: ::Value<String>,
    /// Property [`EndpointName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-endpointaccess.html#cfn-redshift-endpointaccess-endpointname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub endpoint_name: ::Value<String>,
    /// Property [`ResourceOwner`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-endpointaccess.html#cfn-redshift-endpointaccess-resourceowner).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub resource_owner: Option<::Value<String>>,
    /// Property [`SubnetGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-endpointaccess.html#cfn-redshift-endpointaccess-subnetgroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub subnet_group_name: ::Value<String>,
    /// Property [`VpcSecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-endpointaccess.html#cfn-redshift-endpointaccess-vpcsecuritygroupids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub vpc_security_group_ids: ::ValueList<String>,
}

impl ::serde::Serialize for EndpointAccessProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterIdentifier", &self.cluster_identifier)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointName", &self.endpoint_name)?;
        if let Some(ref resource_owner) = self.resource_owner {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceOwner", resource_owner)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetGroupName", &self.subnet_group_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcSecurityGroupIds", &self.vpc_security_group_ids)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EndpointAccessProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EndpointAccessProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EndpointAccessProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EndpointAccessProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut cluster_identifier: Option<::Value<String>> = None;
                let mut endpoint_name: Option<::Value<String>> = None;
                let mut resource_owner: Option<::Value<String>> = None;
                let mut subnet_group_name: Option<::Value<String>> = None;
                let mut vpc_security_group_ids: Option<::ValueList<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ClusterIdentifier" => {
                            cluster_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EndpointName" => {
                            endpoint_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourceOwner" => {
                            resource_owner = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SubnetGroupName" => {
                            subnet_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcSecurityGroupIds" => {
                            vpc_security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(EndpointAccessProperties {
                    cluster_identifier: cluster_identifier.ok_or(::serde::de::Error::missing_field("ClusterIdentifier"))?,
                    endpoint_name: endpoint_name.ok_or(::serde::de::Error::missing_field("EndpointName"))?,
                    resource_owner: resource_owner,
                    subnet_group_name: subnet_group_name.ok_or(::serde::de::Error::missing_field("SubnetGroupName"))?,
                    vpc_security_group_ids: vpc_security_group_ids.ok_or(::serde::de::Error::missing_field("VpcSecurityGroupIds"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for EndpointAccess {
    type Properties = EndpointAccessProperties;
    const TYPE: &'static str = "AWS::Redshift::EndpointAccess";
    fn properties(&self) -> &EndpointAccessProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EndpointAccessProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for EndpointAccess {}

impl From<EndpointAccessProperties> for EndpointAccess {
    fn from(properties: EndpointAccessProperties) -> EndpointAccess {
        EndpointAccess { properties }
    }
}

/// The [`AWS::Redshift::EndpointAuthorization`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-endpointauthorization.html) resource type.
#[derive(Debug, Default)]
pub struct EndpointAuthorization {
    properties: EndpointAuthorizationProperties
}

/// Properties for the `EndpointAuthorization` resource.
#[derive(Debug, Default)]
pub struct EndpointAuthorizationProperties {
    /// Property [`Account`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-endpointauthorization.html#cfn-redshift-endpointauthorization-account).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub account: ::Value<String>,
    /// Property [`ClusterIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-endpointauthorization.html#cfn-redshift-endpointauthorization-clusteridentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub cluster_identifier: ::Value<String>,
    /// Property [`Force`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-endpointauthorization.html#cfn-redshift-endpointauthorization-force).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub force: Option<::Value<bool>>,
    /// Property [`VpcIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-endpointauthorization.html#cfn-redshift-endpointauthorization-vpcids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub vpc_ids: Option<::ValueList<String>>,
}

impl ::serde::Serialize for EndpointAuthorizationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Account", &self.account)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterIdentifier", &self.cluster_identifier)?;
        if let Some(ref force) = self.force {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Force", force)?;
        }
        if let Some(ref vpc_ids) = self.vpc_ids {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcIds", vpc_ids)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EndpointAuthorizationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EndpointAuthorizationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EndpointAuthorizationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EndpointAuthorizationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut account: Option<::Value<String>> = None;
                let mut cluster_identifier: Option<::Value<String>> = None;
                let mut force: Option<::Value<bool>> = None;
                let mut vpc_ids: Option<::ValueList<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Account" => {
                            account = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClusterIdentifier" => {
                            cluster_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Force" => {
                            force = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcIds" => {
                            vpc_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(EndpointAuthorizationProperties {
                    account: account.ok_or(::serde::de::Error::missing_field("Account"))?,
                    cluster_identifier: cluster_identifier.ok_or(::serde::de::Error::missing_field("ClusterIdentifier"))?,
                    force: force,
                    vpc_ids: vpc_ids,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for EndpointAuthorization {
    type Properties = EndpointAuthorizationProperties;
    const TYPE: &'static str = "AWS::Redshift::EndpointAuthorization";
    fn properties(&self) -> &EndpointAuthorizationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EndpointAuthorizationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for EndpointAuthorization {}

impl From<EndpointAuthorizationProperties> for EndpointAuthorization {
    fn from(properties: EndpointAuthorizationProperties) -> EndpointAuthorization {
        EndpointAuthorization { properties }
    }
}

/// The [`AWS::Redshift::EventSubscription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-eventsubscription.html) resource type.
#[derive(Debug, Default)]
pub struct EventSubscription {
    properties: EventSubscriptionProperties
}

/// Properties for the `EventSubscription` resource.
#[derive(Debug, Default)]
pub struct EventSubscriptionProperties {
    /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-eventsubscription.html#cfn-redshift-eventsubscription-enabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enabled: Option<::Value<bool>>,
    /// Property [`EventCategories`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-eventsubscription.html#cfn-redshift-eventsubscription-eventcategories).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub event_categories: Option<::ValueList<String>>,
    /// Property [`Severity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-eventsubscription.html#cfn-redshift-eventsubscription-severity).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub severity: Option<::Value<String>>,
    /// Property [`SnsTopicArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-eventsubscription.html#cfn-redshift-eventsubscription-snstopicarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sns_topic_arn: Option<::Value<String>>,
    /// Property [`SourceIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-eventsubscription.html#cfn-redshift-eventsubscription-sourceids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub source_ids: Option<::ValueList<String>>,
    /// Property [`SourceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-eventsubscription.html#cfn-redshift-eventsubscription-sourcetype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub source_type: Option<::Value<String>>,
    /// Property [`SubscriptionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-eventsubscription.html#cfn-redshift-eventsubscription-subscriptionname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub subscription_name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-eventsubscription.html#cfn-redshift-eventsubscription-tags).
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
        if let Some(ref severity) = self.severity {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Severity", severity)?;
        }
        if let Some(ref sns_topic_arn) = self.sns_topic_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnsTopicArn", sns_topic_arn)?;
        }
        if let Some(ref source_ids) = self.source_ids {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceIds", source_ids)?;
        }
        if let Some(ref source_type) = self.source_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceType", source_type)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubscriptionName", &self.subscription_name)?;
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
                let mut severity: Option<::Value<String>> = None;
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
                        "Severity" => {
                            severity = ::serde::de::MapAccess::next_value(&mut map)?;
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
                    severity: severity,
                    sns_topic_arn: sns_topic_arn,
                    source_ids: source_ids,
                    source_type: source_type,
                    subscription_name: subscription_name.ok_or(::serde::de::Error::missing_field("SubscriptionName"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for EventSubscription {
    type Properties = EventSubscriptionProperties;
    const TYPE: &'static str = "AWS::Redshift::EventSubscription";
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

/// The [`AWS::Redshift::ScheduledAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-scheduledaction.html) resource type.
#[derive(Debug, Default)]
pub struct ScheduledAction {
    properties: ScheduledActionProperties
}

/// Properties for the `ScheduledAction` resource.
#[derive(Debug, Default)]
pub struct ScheduledActionProperties {
    /// Property [`Enable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-scheduledaction.html#cfn-redshift-scheduledaction-enable).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enable: Option<::Value<bool>>,
    /// Property [`EndTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-scheduledaction.html#cfn-redshift-scheduledaction-endtime).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub end_time: Option<::Value<String>>,
    /// Property [`IamRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-scheduledaction.html#cfn-redshift-scheduledaction-iamrole).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub iam_role: Option<::Value<String>>,
    /// Property [`Schedule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-scheduledaction.html#cfn-redshift-scheduledaction-schedule).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub schedule: Option<::Value<String>>,
    /// Property [`ScheduledActionDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-scheduledaction.html#cfn-redshift-scheduledaction-scheduledactiondescription).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub scheduled_action_description: Option<::Value<String>>,
    /// Property [`ScheduledActionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-scheduledaction.html#cfn-redshift-scheduledaction-scheduledactionname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub scheduled_action_name: ::Value<String>,
    /// Property [`StartTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-scheduledaction.html#cfn-redshift-scheduledaction-starttime).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub start_time: Option<::Value<String>>,
    /// Property [`TargetAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-scheduledaction.html#cfn-redshift-scheduledaction-targetaction).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub target_action: Option<::Value<self::scheduled_action::ScheduledActionType>>,
}

impl ::serde::Serialize for ScheduledActionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref enable) = self.enable {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enable", enable)?;
        }
        if let Some(ref end_time) = self.end_time {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndTime", end_time)?;
        }
        if let Some(ref iam_role) = self.iam_role {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IamRole", iam_role)?;
        }
        if let Some(ref schedule) = self.schedule {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Schedule", schedule)?;
        }
        if let Some(ref scheduled_action_description) = self.scheduled_action_description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScheduledActionDescription", scheduled_action_description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScheduledActionName", &self.scheduled_action_name)?;
        if let Some(ref start_time) = self.start_time {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartTime", start_time)?;
        }
        if let Some(ref target_action) = self.target_action {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetAction", target_action)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ScheduledActionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ScheduledActionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ScheduledActionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ScheduledActionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut enable: Option<::Value<bool>> = None;
                let mut end_time: Option<::Value<String>> = None;
                let mut iam_role: Option<::Value<String>> = None;
                let mut schedule: Option<::Value<String>> = None;
                let mut scheduled_action_description: Option<::Value<String>> = None;
                let mut scheduled_action_name: Option<::Value<String>> = None;
                let mut start_time: Option<::Value<String>> = None;
                let mut target_action: Option<::Value<self::scheduled_action::ScheduledActionType>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Enable" => {
                            enable = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EndTime" => {
                            end_time = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IamRole" => {
                            iam_role = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Schedule" => {
                            schedule = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ScheduledActionDescription" => {
                            scheduled_action_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ScheduledActionName" => {
                            scheduled_action_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StartTime" => {
                            start_time = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetAction" => {
                            target_action = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ScheduledActionProperties {
                    enable: enable,
                    end_time: end_time,
                    iam_role: iam_role,
                    schedule: schedule,
                    scheduled_action_description: scheduled_action_description,
                    scheduled_action_name: scheduled_action_name.ok_or(::serde::de::Error::missing_field("ScheduledActionName"))?,
                    start_time: start_time,
                    target_action: target_action,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ScheduledAction {
    type Properties = ScheduledActionProperties;
    const TYPE: &'static str = "AWS::Redshift::ScheduledAction";
    fn properties(&self) -> &ScheduledActionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ScheduledActionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ScheduledAction {}

impl From<ScheduledActionProperties> for ScheduledAction {
    fn from(properties: ScheduledActionProperties) -> ScheduledAction {
        ScheduledAction { properties }
    }
}

pub mod cluster {
    //! Property types for the `Cluster` resource.

    /// The [`AWS::Redshift::Cluster.Endpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-cluster-endpoint.html) property type.
    #[derive(Debug, Default)]
    pub struct Endpoint {
        /// Property [`Address`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-cluster-endpoint.html#cfn-redshift-cluster-endpoint-address).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub address: Option<::Value<String>>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-cluster-endpoint.html#cfn-redshift-cluster-endpoint-port).
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

    /// The [`AWS::Redshift::Cluster.LoggingProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-cluster-loggingproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct LoggingProperties {
        /// Property [`BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-cluster-loggingproperties.html#cfn-redshift-cluster-loggingproperties-bucketname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_name: Option<::Value<String>>,
        /// Property [`S3KeyPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-cluster-loggingproperties.html#cfn-redshift-cluster-loggingproperties-s3keyprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_key_prefix: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LoggingProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref bucket_name) = self.bucket_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketName", bucket_name)?;
            }
            if let Some(ref s3_key_prefix) = self.s3_key_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3KeyPrefix", s3_key_prefix)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LoggingProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LoggingProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LoggingProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LoggingProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket_name: Option<::Value<String>> = None;
                    let mut s3_key_prefix: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketName" => {
                                bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3KeyPrefix" => {
                                s3_key_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LoggingProperties {
                        bucket_name: bucket_name,
                        s3_key_prefix: s3_key_prefix,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod cluster_parameter_group {
    //! Property types for the `ClusterParameterGroup` resource.

    /// The [`AWS::Redshift::ClusterParameterGroup.Parameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-clusterparametergroup-parameter.html) property type.
    #[derive(Debug, Default)]
    pub struct Parameter {
        /// Property [`ParameterName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-clusterparametergroup-parameter.html#cfn-redshift-clusterparametergroup-parameter-parametername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameter_name: ::Value<String>,
        /// Property [`ParameterValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-clusterparametergroup-parameter.html#cfn-redshift-clusterparametergroup-parameter-parametervalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameter_value: ::Value<String>,
    }

    impl ::codec::SerializeValue for Parameter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParameterName", &self.parameter_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParameterValue", &self.parameter_value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Parameter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Parameter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Parameter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Parameter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut parameter_name: Option<::Value<String>> = None;
                    let mut parameter_value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ParameterName" => {
                                parameter_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ParameterValue" => {
                                parameter_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Parameter {
                        parameter_name: parameter_name.ok_or(::serde::de::Error::missing_field("ParameterName"))?,
                        parameter_value: parameter_value.ok_or(::serde::de::Error::missing_field("ParameterValue"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod endpoint_access {
    //! Property types for the `EndpointAccess` resource.

    /// The [`AWS::Redshift::EndpointAccess.NetworkInterface`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-endpointaccess-networkinterface.html) property type.
    #[derive(Debug, Default)]
    pub struct NetworkInterface {
        /// Property [`AvailabilityZone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-endpointaccess-networkinterface.html#cfn-redshift-endpointaccess-networkinterface-availabilityzone).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub availability_zone: Option<::Value<String>>,
        /// Property [`NetworkInterfaceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-endpointaccess-networkinterface.html#cfn-redshift-endpointaccess-networkinterface-networkinterfaceid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub network_interface_id: Option<::Value<String>>,
        /// Property [`PrivateIpAddress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-endpointaccess-networkinterface.html#cfn-redshift-endpointaccess-networkinterface-privateipaddress).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub private_ip_address: Option<::Value<String>>,
        /// Property [`SubnetId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-endpointaccess-networkinterface.html#cfn-redshift-endpointaccess-networkinterface-subnetid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subnet_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for NetworkInterface {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref availability_zone) = self.availability_zone {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZone", availability_zone)?;
            }
            if let Some(ref network_interface_id) = self.network_interface_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkInterfaceId", network_interface_id)?;
            }
            if let Some(ref private_ip_address) = self.private_ip_address {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrivateIpAddress", private_ip_address)?;
            }
            if let Some(ref subnet_id) = self.subnet_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetId", subnet_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NetworkInterface {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NetworkInterface, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NetworkInterface;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NetworkInterface")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut availability_zone: Option<::Value<String>> = None;
                    let mut network_interface_id: Option<::Value<String>> = None;
                    let mut private_ip_address: Option<::Value<String>> = None;
                    let mut subnet_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AvailabilityZone" => {
                                availability_zone = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NetworkInterfaceId" => {
                                network_interface_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PrivateIpAddress" => {
                                private_ip_address = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubnetId" => {
                                subnet_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NetworkInterface {
                        availability_zone: availability_zone,
                        network_interface_id: network_interface_id,
                        private_ip_address: private_ip_address,
                        subnet_id: subnet_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Redshift::EndpointAccess.VpcEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-endpointaccess-vpcendpoint.html) property type.
    #[derive(Debug, Default)]
    pub struct VpcEndpoint {
        /// Property [`NetworkInterfaces`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-endpointaccess-vpcendpoint.html#cfn-redshift-endpointaccess-vpcendpoint-networkinterfaces).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub network_interfaces: Option<::ValueList<NetworkInterface>>,
        /// Property [`VpcEndpointId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-endpointaccess-vpcendpoint.html#cfn-redshift-endpointaccess-vpcendpoint-vpcendpointid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vpc_endpoint_id: Option<::Value<String>>,
        /// Property [`VpcId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-endpointaccess-vpcendpoint.html#cfn-redshift-endpointaccess-vpcendpoint-vpcid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vpc_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for VpcEndpoint {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref network_interfaces) = self.network_interfaces {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkInterfaces", network_interfaces)?;
            }
            if let Some(ref vpc_endpoint_id) = self.vpc_endpoint_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcEndpointId", vpc_endpoint_id)?;
            }
            if let Some(ref vpc_id) = self.vpc_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcId", vpc_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VpcEndpoint {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VpcEndpoint, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VpcEndpoint;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VpcEndpoint")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut network_interfaces: Option<::ValueList<NetworkInterface>> = None;
                    let mut vpc_endpoint_id: Option<::Value<String>> = None;
                    let mut vpc_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "NetworkInterfaces" => {
                                network_interfaces = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpcEndpointId" => {
                                vpc_endpoint_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpcId" => {
                                vpc_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VpcEndpoint {
                        network_interfaces: network_interfaces,
                        vpc_endpoint_id: vpc_endpoint_id,
                        vpc_id: vpc_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Redshift::EndpointAccess.VpcSecurityGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-endpointaccess-vpcsecuritygroup.html) property type.
    #[derive(Debug, Default)]
    pub struct VpcSecurityGroup {
        /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-endpointaccess-vpcsecuritygroup.html#cfn-redshift-endpointaccess-vpcsecuritygroup-status).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub status: Option<::Value<String>>,
        /// Property [`VpcSecurityGroupId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-endpointaccess-vpcsecuritygroup.html#cfn-redshift-endpointaccess-vpcsecuritygroup-vpcsecuritygroupid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vpc_security_group_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for VpcSecurityGroup {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref status) = self.status {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", status)?;
            }
            if let Some(ref vpc_security_group_id) = self.vpc_security_group_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcSecurityGroupId", vpc_security_group_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VpcSecurityGroup {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VpcSecurityGroup, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VpcSecurityGroup;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VpcSecurityGroup")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut status: Option<::Value<String>> = None;
                    let mut vpc_security_group_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Status" => {
                                status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpcSecurityGroupId" => {
                                vpc_security_group_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VpcSecurityGroup {
                        status: status,
                        vpc_security_group_id: vpc_security_group_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod scheduled_action {
    //! Property types for the `ScheduledAction` resource.

    /// The [`AWS::Redshift::ScheduledAction.PauseClusterMessage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-scheduledaction-pauseclustermessage.html) property type.
    #[derive(Debug, Default)]
    pub struct PauseClusterMessage {
        /// Property [`ClusterIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-scheduledaction-pauseclustermessage.html#cfn-redshift-scheduledaction-pauseclustermessage-clusteridentifier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cluster_identifier: ::Value<String>,
    }

    impl ::codec::SerializeValue for PauseClusterMessage {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterIdentifier", &self.cluster_identifier)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PauseClusterMessage {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PauseClusterMessage, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PauseClusterMessage;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PauseClusterMessage")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cluster_identifier: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ClusterIdentifier" => {
                                cluster_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PauseClusterMessage {
                        cluster_identifier: cluster_identifier.ok_or(::serde::de::Error::missing_field("ClusterIdentifier"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Redshift::ScheduledAction.ResizeClusterMessage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-scheduledaction-resizeclustermessage.html) property type.
    #[derive(Debug, Default)]
    pub struct ResizeClusterMessage {
        /// Property [`Classic`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-scheduledaction-resizeclustermessage.html#cfn-redshift-scheduledaction-resizeclustermessage-classic).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub classic: Option<::Value<bool>>,
        /// Property [`ClusterIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-scheduledaction-resizeclustermessage.html#cfn-redshift-scheduledaction-resizeclustermessage-clusteridentifier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cluster_identifier: ::Value<String>,
        /// Property [`ClusterType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-scheduledaction-resizeclustermessage.html#cfn-redshift-scheduledaction-resizeclustermessage-clustertype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cluster_type: Option<::Value<String>>,
        /// Property [`NodeType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-scheduledaction-resizeclustermessage.html#cfn-redshift-scheduledaction-resizeclustermessage-nodetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub node_type: Option<::Value<String>>,
        /// Property [`NumberOfNodes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-scheduledaction-resizeclustermessage.html#cfn-redshift-scheduledaction-resizeclustermessage-numberofnodes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub number_of_nodes: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for ResizeClusterMessage {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref classic) = self.classic {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Classic", classic)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterIdentifier", &self.cluster_identifier)?;
            if let Some(ref cluster_type) = self.cluster_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterType", cluster_type)?;
            }
            if let Some(ref node_type) = self.node_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NodeType", node_type)?;
            }
            if let Some(ref number_of_nodes) = self.number_of_nodes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumberOfNodes", number_of_nodes)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResizeClusterMessage {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResizeClusterMessage, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResizeClusterMessage;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResizeClusterMessage")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut classic: Option<::Value<bool>> = None;
                    let mut cluster_identifier: Option<::Value<String>> = None;
                    let mut cluster_type: Option<::Value<String>> = None;
                    let mut node_type: Option<::Value<String>> = None;
                    let mut number_of_nodes: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Classic" => {
                                classic = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ClusterIdentifier" => {
                                cluster_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ClusterType" => {
                                cluster_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NodeType" => {
                                node_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NumberOfNodes" => {
                                number_of_nodes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResizeClusterMessage {
                        classic: classic,
                        cluster_identifier: cluster_identifier.ok_or(::serde::de::Error::missing_field("ClusterIdentifier"))?,
                        cluster_type: cluster_type,
                        node_type: node_type,
                        number_of_nodes: number_of_nodes,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Redshift::ScheduledAction.ResumeClusterMessage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-scheduledaction-resumeclustermessage.html) property type.
    #[derive(Debug, Default)]
    pub struct ResumeClusterMessage {
        /// Property [`ClusterIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-scheduledaction-resumeclustermessage.html#cfn-redshift-scheduledaction-resumeclustermessage-clusteridentifier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cluster_identifier: ::Value<String>,
    }

    impl ::codec::SerializeValue for ResumeClusterMessage {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterIdentifier", &self.cluster_identifier)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResumeClusterMessage {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResumeClusterMessage, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResumeClusterMessage;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResumeClusterMessage")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cluster_identifier: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ClusterIdentifier" => {
                                cluster_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResumeClusterMessage {
                        cluster_identifier: cluster_identifier.ok_or(::serde::de::Error::missing_field("ClusterIdentifier"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Redshift::ScheduledAction.ScheduledActionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-scheduledaction-scheduledactiontype.html) property type.
    #[derive(Debug, Default)]
    pub struct ScheduledActionType {
        /// Property [`PauseCluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-scheduledaction-scheduledactiontype.html#cfn-redshift-scheduledaction-scheduledactiontype-pausecluster).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pause_cluster: Option<::Value<PauseClusterMessage>>,
        /// Property [`ResizeCluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-scheduledaction-scheduledactiontype.html#cfn-redshift-scheduledaction-scheduledactiontype-resizecluster).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resize_cluster: Option<::Value<ResizeClusterMessage>>,
        /// Property [`ResumeCluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-scheduledaction-scheduledactiontype.html#cfn-redshift-scheduledaction-scheduledactiontype-resumecluster).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resume_cluster: Option<::Value<ResumeClusterMessage>>,
    }

    impl ::codec::SerializeValue for ScheduledActionType {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref pause_cluster) = self.pause_cluster {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PauseCluster", pause_cluster)?;
            }
            if let Some(ref resize_cluster) = self.resize_cluster {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResizeCluster", resize_cluster)?;
            }
            if let Some(ref resume_cluster) = self.resume_cluster {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResumeCluster", resume_cluster)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ScheduledActionType {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ScheduledActionType, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ScheduledActionType;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ScheduledActionType")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut pause_cluster: Option<::Value<PauseClusterMessage>> = None;
                    let mut resize_cluster: Option<::Value<ResizeClusterMessage>> = None;
                    let mut resume_cluster: Option<::Value<ResumeClusterMessage>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PauseCluster" => {
                                pause_cluster = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResizeCluster" => {
                                resize_cluster = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResumeCluster" => {
                                resume_cluster = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ScheduledActionType {
                        pause_cluster: pause_cluster,
                        resize_cluster: resize_cluster,
                        resume_cluster: resume_cluster,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
