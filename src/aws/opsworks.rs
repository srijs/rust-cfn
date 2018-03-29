//! Types for the `OpsWorks` service.

/// The [`AWS::OpsWorks::App`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-app.html) resource type.
#[derive(Debug)]
pub struct App {
    properties: AppProperties
}

/// Properties for the `App` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct AppProperties {
    /// Property `AppSource`.
    #[serde(rename="AppSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_source: Option<self::app::Source>,
    /// Property `Attributes`.
    #[serde(rename="Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// Property `DataSources`.
    #[serde(rename="DataSources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<Vec<self::app::DataSource>>,
    /// Property `Description`.
    #[serde(rename="Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Property `Domains`.
    #[serde(rename="Domains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<String>>,
    /// Property `EnableSsl`.
    #[serde(rename="EnableSsl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_ssl: Option<bool>,
    /// Property `Environment`.
    #[serde(rename="Environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<self::app::EnvironmentVariable>>,
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
    /// Property `Shortname`.
    #[serde(rename="Shortname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shortname: Option<String>,
    /// Property `SslConfiguration`.
    #[serde(rename="SslConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_configuration: Option<self::app::SslConfiguration>,
    /// Property `StackId`.
    #[serde(rename="StackId")]
    pub stack_id: String,
    /// Property `Type`.
    #[serde(rename="Type")]
    pub type_: String,
}

impl<'a> ::Resource<'a> for App {
    type Properties = AppProperties;
    const TYPE: &'static str = "AWS::OpsWorks::App";
    fn properties(&self) -> &AppProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AppProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for App {}

impl From<AppProperties> for App {
    fn from(properties: AppProperties) -> App {
        App { properties }
    }
}

/// The [`AWS::OpsWorks::ElasticLoadBalancerAttachment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-elbattachment.html) resource type.
#[derive(Debug)]
pub struct ElasticLoadBalancerAttachment {
    properties: ElasticLoadBalancerAttachmentProperties
}

/// Properties for the `ElasticLoadBalancerAttachment` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ElasticLoadBalancerAttachmentProperties {
    /// Property `ElasticLoadBalancerName`.
    #[serde(rename="ElasticLoadBalancerName")]
    pub elastic_load_balancer_name: String,
    /// Property `LayerId`.
    #[serde(rename="LayerId")]
    pub layer_id: String,
}

impl<'a> ::Resource<'a> for ElasticLoadBalancerAttachment {
    type Properties = ElasticLoadBalancerAttachmentProperties;
    const TYPE: &'static str = "AWS::OpsWorks::ElasticLoadBalancerAttachment";
    fn properties(&self) -> &ElasticLoadBalancerAttachmentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ElasticLoadBalancerAttachmentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ElasticLoadBalancerAttachment {}

impl From<ElasticLoadBalancerAttachmentProperties> for ElasticLoadBalancerAttachment {
    fn from(properties: ElasticLoadBalancerAttachmentProperties) -> ElasticLoadBalancerAttachment {
        ElasticLoadBalancerAttachment { properties }
    }
}

/// The [`AWS::OpsWorks::Instance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-instance.html) resource type.
#[derive(Debug)]
pub struct Instance {
    properties: InstanceProperties
}

/// Properties for the `Instance` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct InstanceProperties {
    /// Property `AgentVersion`.
    #[serde(rename="AgentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    /// Property `AmiId`.
    #[serde(rename="AmiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ami_id: Option<String>,
    /// Property `Architecture`.
    #[serde(rename="Architecture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// Property `AutoScalingType`.
    #[serde(rename="AutoScalingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_type: Option<String>,
    /// Property `AvailabilityZone`.
    #[serde(rename="AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// Property `BlockDeviceMappings`.
    #[serde(rename="BlockDeviceMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_device_mappings: Option<Vec<self::instance::BlockDeviceMapping>>,
    /// Property `EbsOptimized`.
    #[serde(rename="EbsOptimized")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_optimized: Option<bool>,
    /// Property `ElasticIps`.
    #[serde(rename="ElasticIps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elastic_ips: Option<Vec<String>>,
    /// Property `Hostname`.
    #[serde(rename="Hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// Property `InstallUpdatesOnBoot`.
    #[serde(rename="InstallUpdatesOnBoot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_updates_on_boot: Option<bool>,
    /// Property `InstanceType`.
    #[serde(rename="InstanceType")]
    pub instance_type: String,
    /// Property `LayerIds`.
    #[serde(rename="LayerIds")]
    pub layer_ids: Vec<String>,
    /// Property `Os`.
    #[serde(rename="Os")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// Property `RootDeviceType`.
    #[serde(rename="RootDeviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_device_type: Option<String>,
    /// Property `SshKeyName`.
    #[serde(rename="SshKeyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_key_name: Option<String>,
    /// Property `StackId`.
    #[serde(rename="StackId")]
    pub stack_id: String,
    /// Property `SubnetId`.
    #[serde(rename="SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    /// Property `Tenancy`.
    #[serde(rename="Tenancy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenancy: Option<String>,
    /// Property `TimeBasedAutoScaling`.
    #[serde(rename="TimeBasedAutoScaling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_based_auto_scaling: Option<self::instance::TimeBasedAutoScaling>,
    /// Property `VirtualizationType`.
    #[serde(rename="VirtualizationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtualization_type: Option<String>,
    /// Property `Volumes`.
    #[serde(rename="Volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<String>>,
}

impl<'a> ::Resource<'a> for Instance {
    type Properties = InstanceProperties;
    const TYPE: &'static str = "AWS::OpsWorks::Instance";
    fn properties(&self) -> &InstanceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut InstanceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Instance {}

impl From<InstanceProperties> for Instance {
    fn from(properties: InstanceProperties) -> Instance {
        Instance { properties }
    }
}

/// The [`AWS::OpsWorks::Layer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-layer.html) resource type.
#[derive(Debug)]
pub struct Layer {
    properties: LayerProperties
}

/// Properties for the `Layer` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct LayerProperties {
    /// Property `Attributes`.
    #[serde(rename="Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// Property `AutoAssignElasticIps`.
    #[serde(rename="AutoAssignElasticIps")]
    pub auto_assign_elastic_ips: bool,
    /// Property `AutoAssignPublicIps`.
    #[serde(rename="AutoAssignPublicIps")]
    pub auto_assign_public_ips: bool,
    /// Property `CustomInstanceProfileArn`.
    #[serde(rename="CustomInstanceProfileArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_instance_profile_arn: Option<String>,
    /// Property `CustomJson`.
    #[serde(rename="CustomJson")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_json: Option<::json::Value>,
    /// Property `CustomRecipes`.
    #[serde(rename="CustomRecipes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_recipes: Option<self::layer::Recipes>,
    /// Property `CustomSecurityGroupIds`.
    #[serde(rename="CustomSecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_security_group_ids: Option<Vec<String>>,
    /// Property `EnableAutoHealing`.
    #[serde(rename="EnableAutoHealing")]
    pub enable_auto_healing: bool,
    /// Property `InstallUpdatesOnBoot`.
    #[serde(rename="InstallUpdatesOnBoot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_updates_on_boot: Option<bool>,
    /// Property `LifecycleEventConfiguration`.
    #[serde(rename="LifecycleEventConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_event_configuration: Option<self::layer::LifecycleEventConfiguration>,
    /// Property `LoadBasedAutoScaling`.
    #[serde(rename="LoadBasedAutoScaling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_based_auto_scaling: Option<self::layer::LoadBasedAutoScaling>,
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
    /// Property `Packages`.
    #[serde(rename="Packages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packages: Option<Vec<String>>,
    /// Property `Shortname`.
    #[serde(rename="Shortname")]
    pub shortname: String,
    /// Property `StackId`.
    #[serde(rename="StackId")]
    pub stack_id: String,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::Tags>,
    /// Property `Type`.
    #[serde(rename="Type")]
    pub type_: String,
    /// Property `UseEbsOptimizedInstances`.
    #[serde(rename="UseEbsOptimizedInstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_ebs_optimized_instances: Option<bool>,
    /// Property `VolumeConfigurations`.
    #[serde(rename="VolumeConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_configurations: Option<Vec<self::layer::VolumeConfiguration>>,
}

impl<'a> ::Resource<'a> for Layer {
    type Properties = LayerProperties;
    const TYPE: &'static str = "AWS::OpsWorks::Layer";
    fn properties(&self) -> &LayerProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LayerProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Layer {}

impl From<LayerProperties> for Layer {
    fn from(properties: LayerProperties) -> Layer {
        Layer { properties }
    }
}

/// The [`AWS::OpsWorks::Stack`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-stack.html) resource type.
#[derive(Debug)]
pub struct Stack {
    properties: StackProperties
}

/// Properties for the `Stack` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct StackProperties {
    /// Property `AgentVersion`.
    #[serde(rename="AgentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    /// Property `Attributes`.
    #[serde(rename="Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::std::collections::HashMap<String, String>>,
    /// Property `ChefConfiguration`.
    #[serde(rename="ChefConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chef_configuration: Option<self::stack::ChefConfiguration>,
    /// Property `CloneAppIds`.
    #[serde(rename="CloneAppIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clone_app_ids: Option<Vec<String>>,
    /// Property `ClonePermissions`.
    #[serde(rename="ClonePermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clone_permissions: Option<bool>,
    /// Property `ConfigurationManager`.
    #[serde(rename="ConfigurationManager")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_manager: Option<self::stack::StackConfigurationManager>,
    /// Property `CustomCookbooksSource`.
    #[serde(rename="CustomCookbooksSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_cookbooks_source: Option<self::stack::Source>,
    /// Property `CustomJson`.
    #[serde(rename="CustomJson")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_json: Option<::json::Value>,
    /// Property `DefaultAvailabilityZone`.
    #[serde(rename="DefaultAvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_availability_zone: Option<String>,
    /// Property `DefaultInstanceProfileArn`.
    #[serde(rename="DefaultInstanceProfileArn")]
    pub default_instance_profile_arn: String,
    /// Property `DefaultOs`.
    #[serde(rename="DefaultOs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_os: Option<String>,
    /// Property `DefaultRootDeviceType`.
    #[serde(rename="DefaultRootDeviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_root_device_type: Option<String>,
    /// Property `DefaultSshKeyName`.
    #[serde(rename="DefaultSshKeyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_ssh_key_name: Option<String>,
    /// Property `DefaultSubnetId`.
    #[serde(rename="DefaultSubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_subnet_id: Option<String>,
    /// Property `EcsClusterArn`.
    #[serde(rename="EcsClusterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecs_cluster_arn: Option<String>,
    /// Property `ElasticIps`.
    #[serde(rename="ElasticIps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elastic_ips: Option<Vec<self::stack::ElasticIp>>,
    /// Property `HostnameTheme`.
    #[serde(rename="HostnameTheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname_theme: Option<String>,
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
    /// Property `RdsDbInstances`.
    #[serde(rename="RdsDbInstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rds_db_instances: Option<Vec<self::stack::RdsDbInstance>>,
    /// Property `ServiceRoleArn`.
    #[serde(rename="ServiceRoleArn")]
    pub service_role_arn: String,
    /// Property `SourceStackId`.
    #[serde(rename="SourceStackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_stack_id: Option<String>,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::Tags>,
    /// Property `UseCustomCookbooks`.
    #[serde(rename="UseCustomCookbooks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_custom_cookbooks: Option<bool>,
    /// Property `UseOpsworksSecurityGroups`.
    #[serde(rename="UseOpsworksSecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_opsworks_security_groups: Option<bool>,
    /// Property `VpcId`.
    #[serde(rename="VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

impl<'a> ::Resource<'a> for Stack {
    type Properties = StackProperties;
    const TYPE: &'static str = "AWS::OpsWorks::Stack";
    fn properties(&self) -> &StackProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut StackProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Stack {}

impl From<StackProperties> for Stack {
    fn from(properties: StackProperties) -> Stack {
        Stack { properties }
    }
}

/// The [`AWS::OpsWorks::UserProfile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-userprofile.html) resource type.
#[derive(Debug)]
pub struct UserProfile {
    properties: UserProfileProperties
}

/// Properties for the `UserProfile` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfileProperties {
    /// Property `AllowSelfManagement`.
    #[serde(rename="AllowSelfManagement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_self_management: Option<bool>,
    /// Property `IamUserArn`.
    #[serde(rename="IamUserArn")]
    pub iam_user_arn: String,
    /// Property `SshPublicKey`.
    #[serde(rename="SshPublicKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_public_key: Option<String>,
    /// Property `SshUsername`.
    #[serde(rename="SshUsername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_username: Option<String>,
}

impl<'a> ::Resource<'a> for UserProfile {
    type Properties = UserProfileProperties;
    const TYPE: &'static str = "AWS::OpsWorks::UserProfile";
    fn properties(&self) -> &UserProfileProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut UserProfileProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for UserProfile {}

impl From<UserProfileProperties> for UserProfile {
    fn from(properties: UserProfileProperties) -> UserProfile {
        UserProfile { properties }
    }
}

/// The [`AWS::OpsWorks::Volume`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-volume.html) resource type.
#[derive(Debug)]
pub struct Volume {
    properties: VolumeProperties
}

/// Properties for the `Volume` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeProperties {
    /// Property `Ec2VolumeId`.
    #[serde(rename="Ec2VolumeId")]
    pub ec2_volume_id: String,
    /// Property `MountPoint`.
    #[serde(rename="MountPoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_point: Option<String>,
    /// Property `Name`.
    #[serde(rename="Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Property `StackId`.
    #[serde(rename="StackId")]
    pub stack_id: String,
}

impl<'a> ::Resource<'a> for Volume {
    type Properties = VolumeProperties;
    const TYPE: &'static str = "AWS::OpsWorks::Volume";
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

pub mod app {
    //! Property types for the `App` resource.

    /// The [`AWS::OpsWorks::App.DataSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-app-datasource.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DataSource {
        /// Property `Arn`.
        #[serde(rename="Arn")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub arn: Option<String>,
        /// Property `DatabaseName`.
        #[serde(rename="DatabaseName")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub database_name: Option<String>,
        /// Property `Type`.
        #[serde(rename="Type")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    /// The [`AWS::OpsWorks::App.EnvironmentVariable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-app-environment.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EnvironmentVariable {
        /// Property `Key`.
        #[serde(rename="Key")]
        pub key: String,
        /// Property `Secure`.
        #[serde(rename="Secure")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub secure: Option<bool>,
        /// Property `Value`.
        #[serde(rename="Value")]
        pub value: String,
    }

    /// The [`AWS::OpsWorks::App.Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-source.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Source {
        /// Property `Password`.
        #[serde(rename="Password")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub password: Option<String>,
        /// Property `Revision`.
        #[serde(rename="Revision")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub revision: Option<String>,
        /// Property `SshKey`.
        #[serde(rename="SshKey")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ssh_key: Option<String>,
        /// Property `Type`.
        #[serde(rename="Type")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
        /// Property `Url`.
        #[serde(rename="Url")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub url: Option<String>,
        /// Property `Username`.
        #[serde(rename="Username")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub username: Option<String>,
    }

    /// The [`AWS::OpsWorks::App.SslConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-app-sslconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SslConfiguration {
        /// Property `Certificate`.
        #[serde(rename="Certificate")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub certificate: Option<String>,
        /// Property `Chain`.
        #[serde(rename="Chain")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub chain: Option<String>,
        /// Property `PrivateKey`.
        #[serde(rename="PrivateKey")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub private_key: Option<String>,
    }
}

pub mod instance {
    //! Property types for the `Instance` resource.

    /// The [`AWS::OpsWorks::Instance.BlockDeviceMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-blockdevicemapping.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct BlockDeviceMapping {
        /// Property `DeviceName`.
        #[serde(rename="DeviceName")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub device_name: Option<String>,
        /// Property `Ebs`.
        #[serde(rename="Ebs")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ebs: Option<EbsBlockDevice>,
        /// Property `NoDevice`.
        #[serde(rename="NoDevice")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub no_device: Option<String>,
        /// Property `VirtualName`.
        #[serde(rename="VirtualName")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub virtual_name: Option<String>,
    }

    /// The [`AWS::OpsWorks::Instance.EbsBlockDevice`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-ebsblockdevice.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EbsBlockDevice {
        /// Property `DeleteOnTermination`.
        #[serde(rename="DeleteOnTermination")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub delete_on_termination: Option<bool>,
        /// Property `Iops`.
        #[serde(rename="Iops")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub iops: Option<u32>,
        /// Property `SnapshotId`.
        #[serde(rename="SnapshotId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub snapshot_id: Option<String>,
        /// Property `VolumeSize`.
        #[serde(rename="VolumeSize")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub volume_size: Option<u32>,
        /// Property `VolumeType`.
        #[serde(rename="VolumeType")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub volume_type: Option<String>,
    }

    /// The [`AWS::OpsWorks::Instance.TimeBasedAutoScaling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-timebasedautoscaling.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TimeBasedAutoScaling {
        /// Property `Friday`.
        #[serde(rename="Friday")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub friday: Option<::std::collections::HashMap<String, String>>,
        /// Property `Monday`.
        #[serde(rename="Monday")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub monday: Option<::std::collections::HashMap<String, String>>,
        /// Property `Saturday`.
        #[serde(rename="Saturday")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub saturday: Option<::std::collections::HashMap<String, String>>,
        /// Property `Sunday`.
        #[serde(rename="Sunday")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sunday: Option<::std::collections::HashMap<String, String>>,
        /// Property `Thursday`.
        #[serde(rename="Thursday")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub thursday: Option<::std::collections::HashMap<String, String>>,
        /// Property `Tuesday`.
        #[serde(rename="Tuesday")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tuesday: Option<::std::collections::HashMap<String, String>>,
        /// Property `Wednesday`.
        #[serde(rename="Wednesday")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub wednesday: Option<::std::collections::HashMap<String, String>>,
    }
}

pub mod layer {
    //! Property types for the `Layer` resource.

    /// The [`AWS::OpsWorks::Layer.AutoScalingThresholds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-loadbasedautoscaling-autoscalingthresholds.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AutoScalingThresholds {
        /// Property `CpuThreshold`.
        #[serde(rename="CpuThreshold")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cpu_threshold: Option<f64>,
        /// Property `IgnoreMetricsTime`.
        #[serde(rename="IgnoreMetricsTime")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ignore_metrics_time: Option<u32>,
        /// Property `InstanceCount`.
        #[serde(rename="InstanceCount")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub instance_count: Option<u32>,
        /// Property `LoadThreshold`.
        #[serde(rename="LoadThreshold")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub load_threshold: Option<f64>,
        /// Property `MemoryThreshold`.
        #[serde(rename="MemoryThreshold")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub memory_threshold: Option<f64>,
        /// Property `ThresholdsWaitTime`.
        #[serde(rename="ThresholdsWaitTime")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub thresholds_wait_time: Option<u32>,
    }

    /// The [`AWS::OpsWorks::Layer.LifecycleEventConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-lifecycleeventconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LifecycleEventConfiguration {
        /// Property `ShutdownEventConfiguration`.
        #[serde(rename="ShutdownEventConfiguration")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub shutdown_event_configuration: Option<ShutdownEventConfiguration>,
    }

    /// The [`AWS::OpsWorks::Layer.LoadBasedAutoScaling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-loadbasedautoscaling.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LoadBasedAutoScaling {
        /// Property `DownScaling`.
        #[serde(rename="DownScaling")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub down_scaling: Option<AutoScalingThresholds>,
        /// Property `Enable`.
        #[serde(rename="Enable")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub enable: Option<bool>,
        /// Property `UpScaling`.
        #[serde(rename="UpScaling")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub up_scaling: Option<AutoScalingThresholds>,
    }

    /// The [`AWS::OpsWorks::Layer.Recipes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-recipes.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Recipes {
        /// Property `Configure`.
        #[serde(rename="Configure")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub configure: Option<Vec<String>>,
        /// Property `Deploy`.
        #[serde(rename="Deploy")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub deploy: Option<Vec<String>>,
        /// Property `Setup`.
        #[serde(rename="Setup")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub setup: Option<Vec<String>>,
        /// Property `Shutdown`.
        #[serde(rename="Shutdown")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub shutdown: Option<Vec<String>>,
        /// Property `Undeploy`.
        #[serde(rename="Undeploy")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub undeploy: Option<Vec<String>>,
    }

    /// The [`AWS::OpsWorks::Layer.ShutdownEventConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-lifecycleeventconfiguration-shutdowneventconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ShutdownEventConfiguration {
        /// Property `DelayUntilElbConnectionsDrained`.
        #[serde(rename="DelayUntilElbConnectionsDrained")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub delay_until_elb_connections_drained: Option<bool>,
        /// Property `ExecutionTimeout`.
        #[serde(rename="ExecutionTimeout")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub execution_timeout: Option<u32>,
    }

    /// The [`AWS::OpsWorks::Layer.VolumeConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-volumeconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VolumeConfiguration {
        /// Property `Iops`.
        #[serde(rename="Iops")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub iops: Option<u32>,
        /// Property `MountPoint`.
        #[serde(rename="MountPoint")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub mount_point: Option<String>,
        /// Property `NumberOfDisks`.
        #[serde(rename="NumberOfDisks")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub number_of_disks: Option<u32>,
        /// Property `RaidLevel`.
        #[serde(rename="RaidLevel")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub raid_level: Option<u32>,
        /// Property `Size`.
        #[serde(rename="Size")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub size: Option<u32>,
        /// Property `VolumeType`.
        #[serde(rename="VolumeType")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub volume_type: Option<String>,
    }
}

pub mod stack {
    //! Property types for the `Stack` resource.

    /// The [`AWS::OpsWorks::Stack.ChefConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-chefconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ChefConfiguration {
        /// Property `BerkshelfVersion`.
        #[serde(rename="BerkshelfVersion")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub berkshelf_version: Option<String>,
        /// Property `ManageBerkshelf`.
        #[serde(rename="ManageBerkshelf")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub manage_berkshelf: Option<bool>,
    }

    /// The [`AWS::OpsWorks::Stack.ElasticIp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-elasticip.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ElasticIp {
        /// Property `Ip`.
        #[serde(rename="Ip")]
        pub ip: String,
        /// Property `Name`.
        #[serde(rename="Name")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    /// The [`AWS::OpsWorks::Stack.RdsDbInstance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-rdsdbinstance.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RdsDbInstance {
        /// Property `DbPassword`.
        #[serde(rename="DbPassword")]
        pub db_password: String,
        /// Property `DbUser`.
        #[serde(rename="DbUser")]
        pub db_user: String,
        /// Property `RdsDbInstanceArn`.
        #[serde(rename="RdsDbInstanceArn")]
        pub rds_db_instance_arn: String,
    }

    /// The [`AWS::OpsWorks::Stack.Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-source.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Source {
        /// Property `Password`.
        #[serde(rename="Password")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub password: Option<String>,
        /// Property `Revision`.
        #[serde(rename="Revision")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub revision: Option<String>,
        /// Property `SshKey`.
        #[serde(rename="SshKey")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ssh_key: Option<String>,
        /// Property `Type`.
        #[serde(rename="Type")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
        /// Property `Url`.
        #[serde(rename="Url")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub url: Option<String>,
        /// Property `Username`.
        #[serde(rename="Username")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub username: Option<String>,
    }

    /// The [`AWS::OpsWorks::Stack.StackConfigurationManager`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-stackconfigmanager.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StackConfigurationManager {
        /// Property `Name`.
        #[serde(rename="Name")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        /// Property `Version`.
        #[serde(rename="Version")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub version: Option<String>,
    }
}
