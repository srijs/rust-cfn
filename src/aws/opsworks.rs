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
    #[serde(rename = "AppSource")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_source: Option<::Value<self::app::Source>>,
    /// Property `Attributes`.
    #[serde(rename = "Attributes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::ValueMap<String>>,
    /// Property `DataSources`.
    #[serde(rename = "DataSources")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<::ValueList<self::app::DataSource>>,
    /// Property `Description`.
    #[serde(rename = "Description")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<::Value<String>>,
    /// Property `Domains`.
    #[serde(rename = "Domains")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domains: Option<::ValueList<String>>,
    /// Property `EnableSsl`.
    #[serde(rename = "EnableSsl")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_ssl: Option<::Value<bool>>,
    /// Property `Environment`.
    #[serde(rename = "Environment")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<::ValueList<self::app::EnvironmentVariable>>,
    /// Property `Name`.
    #[serde(rename = "Name")]
    pub name: ::Value<String>,
    /// Property `Shortname`.
    #[serde(rename = "Shortname")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shortname: Option<::Value<String>>,
    /// Property `SslConfiguration`.
    #[serde(rename = "SslConfiguration")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssl_configuration: Option<::Value<self::app::SslConfiguration>>,
    /// Property `StackId`.
    #[serde(rename = "StackId")]
    pub stack_id: ::Value<String>,
    /// Property `Type`.
    #[serde(rename = "Type")]
    pub type_: ::Value<String>,
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
    #[serde(rename = "ElasticLoadBalancerName")]
    pub elastic_load_balancer_name: ::Value<String>,
    /// Property `LayerId`.
    #[serde(rename = "LayerId")]
    pub layer_id: ::Value<String>,
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
    #[serde(rename = "AgentVersion")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<::Value<String>>,
    /// Property `AmiId`.
    #[serde(rename = "AmiId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ami_id: Option<::Value<String>>,
    /// Property `Architecture`.
    #[serde(rename = "Architecture")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architecture: Option<::Value<String>>,
    /// Property `AutoScalingType`.
    #[serde(rename = "AutoScalingType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_scaling_type: Option<::Value<String>>,
    /// Property `AvailabilityZone`.
    #[serde(rename = "AvailabilityZone")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<::Value<String>>,
    /// Property `BlockDeviceMappings`.
    #[serde(rename = "BlockDeviceMappings")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_device_mappings: Option<::ValueList<self::instance::BlockDeviceMapping>>,
    /// Property `EbsOptimized`.
    #[serde(rename = "EbsOptimized")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ebs_optimized: Option<::Value<bool>>,
    /// Property `ElasticIps`.
    #[serde(rename = "ElasticIps")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub elastic_ips: Option<::ValueList<String>>,
    /// Property `Hostname`.
    #[serde(rename = "Hostname")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<::Value<String>>,
    /// Property `InstallUpdatesOnBoot`.
    #[serde(rename = "InstallUpdatesOnBoot")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub install_updates_on_boot: Option<::Value<bool>>,
    /// Property `InstanceType`.
    #[serde(rename = "InstanceType")]
    pub instance_type: ::Value<String>,
    /// Property `LayerIds`.
    #[serde(rename = "LayerIds")]
    pub layer_ids: ::ValueList<String>,
    /// Property `Os`.
    #[serde(rename = "Os")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os: Option<::Value<String>>,
    /// Property `RootDeviceType`.
    #[serde(rename = "RootDeviceType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root_device_type: Option<::Value<String>>,
    /// Property `SshKeyName`.
    #[serde(rename = "SshKeyName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssh_key_name: Option<::Value<String>>,
    /// Property `StackId`.
    #[serde(rename = "StackId")]
    pub stack_id: ::Value<String>,
    /// Property `SubnetId`.
    #[serde(rename = "SubnetId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<::Value<String>>,
    /// Property `Tenancy`.
    #[serde(rename = "Tenancy")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenancy: Option<::Value<String>>,
    /// Property `TimeBasedAutoScaling`.
    #[serde(rename = "TimeBasedAutoScaling")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_based_auto_scaling: Option<::Value<self::instance::TimeBasedAutoScaling>>,
    /// Property `VirtualizationType`.
    #[serde(rename = "VirtualizationType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub virtualization_type: Option<::Value<String>>,
    /// Property `Volumes`.
    #[serde(rename = "Volumes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volumes: Option<::ValueList<String>>,
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
    #[serde(rename = "Attributes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::ValueMap<String>>,
    /// Property `AutoAssignElasticIps`.
    #[serde(rename = "AutoAssignElasticIps")]
    pub auto_assign_elastic_ips: ::Value<bool>,
    /// Property `AutoAssignPublicIps`.
    #[serde(rename = "AutoAssignPublicIps")]
    pub auto_assign_public_ips: ::Value<bool>,
    /// Property `CustomInstanceProfileArn`.
    #[serde(rename = "CustomInstanceProfileArn")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_instance_profile_arn: Option<::Value<String>>,
    /// Property `CustomJson`.
    #[serde(rename = "CustomJson")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_json: Option<::Value<::json::Value>>,
    /// Property `CustomRecipes`.
    #[serde(rename = "CustomRecipes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_recipes: Option<::Value<self::layer::Recipes>>,
    /// Property `CustomSecurityGroupIds`.
    #[serde(rename = "CustomSecurityGroupIds")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_security_group_ids: Option<::ValueList<String>>,
    /// Property `EnableAutoHealing`.
    #[serde(rename = "EnableAutoHealing")]
    pub enable_auto_healing: ::Value<bool>,
    /// Property `InstallUpdatesOnBoot`.
    #[serde(rename = "InstallUpdatesOnBoot")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub install_updates_on_boot: Option<::Value<bool>>,
    /// Property `LifecycleEventConfiguration`.
    #[serde(rename = "LifecycleEventConfiguration")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_event_configuration: Option<::Value<self::layer::LifecycleEventConfiguration>>,
    /// Property `LoadBasedAutoScaling`.
    #[serde(rename = "LoadBasedAutoScaling")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_based_auto_scaling: Option<::Value<self::layer::LoadBasedAutoScaling>>,
    /// Property `Name`.
    #[serde(rename = "Name")]
    pub name: ::Value<String>,
    /// Property `Packages`.
    #[serde(rename = "Packages")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub packages: Option<::ValueList<String>>,
    /// Property `Shortname`.
    #[serde(rename = "Shortname")]
    pub shortname: ::Value<String>,
    /// Property `StackId`.
    #[serde(rename = "StackId")]
    pub stack_id: ::Value<String>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `Type`.
    #[serde(rename = "Type")]
    pub type_: ::Value<String>,
    /// Property `UseEbsOptimizedInstances`.
    #[serde(rename = "UseEbsOptimizedInstances")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_ebs_optimized_instances: Option<::Value<bool>>,
    /// Property `VolumeConfigurations`.
    #[serde(rename = "VolumeConfigurations")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume_configurations: Option<::ValueList<self::layer::VolumeConfiguration>>,
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
    #[serde(rename = "AgentVersion")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<::Value<String>>,
    /// Property `Attributes`.
    #[serde(rename = "Attributes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<::ValueMap<String>>,
    /// Property `ChefConfiguration`.
    #[serde(rename = "ChefConfiguration")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chef_configuration: Option<::Value<self::stack::ChefConfiguration>>,
    /// Property `CloneAppIds`.
    #[serde(rename = "CloneAppIds")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clone_app_ids: Option<::ValueList<String>>,
    /// Property `ClonePermissions`.
    #[serde(rename = "ClonePermissions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clone_permissions: Option<::Value<bool>>,
    /// Property `ConfigurationManager`.
    #[serde(rename = "ConfigurationManager")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration_manager: Option<::Value<self::stack::StackConfigurationManager>>,
    /// Property `CustomCookbooksSource`.
    #[serde(rename = "CustomCookbooksSource")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_cookbooks_source: Option<::Value<self::stack::Source>>,
    /// Property `CustomJson`.
    #[serde(rename = "CustomJson")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_json: Option<::Value<::json::Value>>,
    /// Property `DefaultAvailabilityZone`.
    #[serde(rename = "DefaultAvailabilityZone")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_availability_zone: Option<::Value<String>>,
    /// Property `DefaultInstanceProfileArn`.
    #[serde(rename = "DefaultInstanceProfileArn")]
    pub default_instance_profile_arn: ::Value<String>,
    /// Property `DefaultOs`.
    #[serde(rename = "DefaultOs")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_os: Option<::Value<String>>,
    /// Property `DefaultRootDeviceType`.
    #[serde(rename = "DefaultRootDeviceType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_root_device_type: Option<::Value<String>>,
    /// Property `DefaultSshKeyName`.
    #[serde(rename = "DefaultSshKeyName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_ssh_key_name: Option<::Value<String>>,
    /// Property `DefaultSubnetId`.
    #[serde(rename = "DefaultSubnetId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_subnet_id: Option<::Value<String>>,
    /// Property `EcsClusterArn`.
    #[serde(rename = "EcsClusterArn")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ecs_cluster_arn: Option<::Value<String>>,
    /// Property `ElasticIps`.
    #[serde(rename = "ElasticIps")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub elastic_ips: Option<::ValueList<self::stack::ElasticIp>>,
    /// Property `HostnameTheme`.
    #[serde(rename = "HostnameTheme")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname_theme: Option<::Value<String>>,
    /// Property `Name`.
    #[serde(rename = "Name")]
    pub name: ::Value<String>,
    /// Property `RdsDbInstances`.
    #[serde(rename = "RdsDbInstances")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rds_db_instances: Option<::ValueList<self::stack::RdsDbInstance>>,
    /// Property `ServiceRoleArn`.
    #[serde(rename = "ServiceRoleArn")]
    pub service_role_arn: ::Value<String>,
    /// Property `SourceStackId`.
    #[serde(rename = "SourceStackId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_stack_id: Option<::Value<String>>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `UseCustomCookbooks`.
    #[serde(rename = "UseCustomCookbooks")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_custom_cookbooks: Option<::Value<bool>>,
    /// Property `UseOpsworksSecurityGroups`.
    #[serde(rename = "UseOpsworksSecurityGroups")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_opsworks_security_groups: Option<::Value<bool>>,
    /// Property `VpcId`.
    #[serde(rename = "VpcId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<::Value<String>>,
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
    #[serde(rename = "AllowSelfManagement")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_self_management: Option<::Value<bool>>,
    /// Property `IamUserArn`.
    #[serde(rename = "IamUserArn")]
    pub iam_user_arn: ::Value<String>,
    /// Property `SshPublicKey`.
    #[serde(rename = "SshPublicKey")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssh_public_key: Option<::Value<String>>,
    /// Property `SshUsername`.
    #[serde(rename = "SshUsername")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssh_username: Option<::Value<String>>,
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
    #[serde(rename = "Ec2VolumeId")]
    pub ec2_volume_id: ::Value<String>,
    /// Property `MountPoint`.
    #[serde(rename = "MountPoint")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mount_point: Option<::Value<String>>,
    /// Property `Name`.
    #[serde(rename = "Name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<::Value<String>>,
    /// Property `StackId`.
    #[serde(rename = "StackId")]
    pub stack_id: ::Value<String>,
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
        #[serde(rename = "Arn")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub arn: Option<::Value<String>>,
        /// Property `DatabaseName`.
        #[serde(rename = "DatabaseName")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub database_name: Option<::Value<String>>,
        /// Property `Type`.
        #[serde(rename = "Type")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(DataSource);

    /// The [`AWS::OpsWorks::App.EnvironmentVariable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-app-environment.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EnvironmentVariable {
        /// Property `Key`.
        #[serde(rename = "Key")]
        pub key: ::Value<String>,
        /// Property `Secure`.
        #[serde(rename = "Secure")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub secure: Option<::Value<bool>>,
        /// Property `Value`.
        #[serde(rename = "Value")]
        pub value: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(EnvironmentVariable);

    /// The [`AWS::OpsWorks::App.Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-source.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Source {
        /// Property `Password`.
        #[serde(rename = "Password")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub password: Option<::Value<String>>,
        /// Property `Revision`.
        #[serde(rename = "Revision")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub revision: Option<::Value<String>>,
        /// Property `SshKey`.
        #[serde(rename = "SshKey")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ssh_key: Option<::Value<String>>,
        /// Property `Type`.
        #[serde(rename = "Type")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<::Value<String>>,
        /// Property `Url`.
        #[serde(rename = "Url")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub url: Option<::Value<String>>,
        /// Property `Username`.
        #[serde(rename = "Username")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub username: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(Source);

    /// The [`AWS::OpsWorks::App.SslConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-app-sslconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SslConfiguration {
        /// Property `Certificate`.
        #[serde(rename = "Certificate")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub certificate: Option<::Value<String>>,
        /// Property `Chain`.
        #[serde(rename = "Chain")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub chain: Option<::Value<String>>,
        /// Property `PrivateKey`.
        #[serde(rename = "PrivateKey")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub private_key: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(SslConfiguration);
}

pub mod instance {
    //! Property types for the `Instance` resource.

    /// The [`AWS::OpsWorks::Instance.BlockDeviceMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-blockdevicemapping.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct BlockDeviceMapping {
        /// Property `DeviceName`.
        #[serde(rename = "DeviceName")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub device_name: Option<::Value<String>>,
        /// Property `Ebs`.
        #[serde(rename = "Ebs")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ebs: Option<::Value<EbsBlockDevice>>,
        /// Property `NoDevice`.
        #[serde(rename = "NoDevice")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub no_device: Option<::Value<String>>,
        /// Property `VirtualName`.
        #[serde(rename = "VirtualName")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub virtual_name: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(BlockDeviceMapping);

    /// The [`AWS::OpsWorks::Instance.EbsBlockDevice`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-ebsblockdevice.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EbsBlockDevice {
        /// Property `DeleteOnTermination`.
        #[serde(rename = "DeleteOnTermination")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub delete_on_termination: Option<::Value<bool>>,
        /// Property `Iops`.
        #[serde(rename = "Iops")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub iops: Option<::Value<u32>>,
        /// Property `SnapshotId`.
        #[serde(rename = "SnapshotId")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub snapshot_id: Option<::Value<String>>,
        /// Property `VolumeSize`.
        #[serde(rename = "VolumeSize")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub volume_size: Option<::Value<u32>>,
        /// Property `VolumeType`.
        #[serde(rename = "VolumeType")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub volume_type: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(EbsBlockDevice);

    /// The [`AWS::OpsWorks::Instance.TimeBasedAutoScaling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-timebasedautoscaling.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TimeBasedAutoScaling {
        /// Property `Friday`.
        #[serde(rename = "Friday")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub friday: Option<::ValueMap<String>>,
        /// Property `Monday`.
        #[serde(rename = "Monday")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub monday: Option<::ValueMap<String>>,
        /// Property `Saturday`.
        #[serde(rename = "Saturday")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub saturday: Option<::ValueMap<String>>,
        /// Property `Sunday`.
        #[serde(rename = "Sunday")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sunday: Option<::ValueMap<String>>,
        /// Property `Thursday`.
        #[serde(rename = "Thursday")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub thursday: Option<::ValueMap<String>>,
        /// Property `Tuesday`.
        #[serde(rename = "Tuesday")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tuesday: Option<::ValueMap<String>>,
        /// Property `Wednesday`.
        #[serde(rename = "Wednesday")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub wednesday: Option<::ValueMap<String>>,
    }

    cfn_internal__inherit_codec_impls!(TimeBasedAutoScaling);
}

pub mod layer {
    //! Property types for the `Layer` resource.

    /// The [`AWS::OpsWorks::Layer.AutoScalingThresholds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-loadbasedautoscaling-autoscalingthresholds.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AutoScalingThresholds {
        /// Property `CpuThreshold`.
        #[serde(rename = "CpuThreshold")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cpu_threshold: Option<::Value<f64>>,
        /// Property `IgnoreMetricsTime`.
        #[serde(rename = "IgnoreMetricsTime")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ignore_metrics_time: Option<::Value<u32>>,
        /// Property `InstanceCount`.
        #[serde(rename = "InstanceCount")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub instance_count: Option<::Value<u32>>,
        /// Property `LoadThreshold`.
        #[serde(rename = "LoadThreshold")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub load_threshold: Option<::Value<f64>>,
        /// Property `MemoryThreshold`.
        #[serde(rename = "MemoryThreshold")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub memory_threshold: Option<::Value<f64>>,
        /// Property `ThresholdsWaitTime`.
        #[serde(rename = "ThresholdsWaitTime")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub thresholds_wait_time: Option<::Value<u32>>,
    }

    cfn_internal__inherit_codec_impls!(AutoScalingThresholds);

    /// The [`AWS::OpsWorks::Layer.LifecycleEventConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-lifecycleeventconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LifecycleEventConfiguration {
        /// Property `ShutdownEventConfiguration`.
        #[serde(rename = "ShutdownEventConfiguration")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub shutdown_event_configuration: Option<::Value<ShutdownEventConfiguration>>,
    }

    cfn_internal__inherit_codec_impls!(LifecycleEventConfiguration);

    /// The [`AWS::OpsWorks::Layer.LoadBasedAutoScaling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-loadbasedautoscaling.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LoadBasedAutoScaling {
        /// Property `DownScaling`.
        #[serde(rename = "DownScaling")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub down_scaling: Option<::Value<AutoScalingThresholds>>,
        /// Property `Enable`.
        #[serde(rename = "Enable")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enable: Option<::Value<bool>>,
        /// Property `UpScaling`.
        #[serde(rename = "UpScaling")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub up_scaling: Option<::Value<AutoScalingThresholds>>,
    }

    cfn_internal__inherit_codec_impls!(LoadBasedAutoScaling);

    /// The [`AWS::OpsWorks::Layer.Recipes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-recipes.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Recipes {
        /// Property `Configure`.
        #[serde(rename = "Configure")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configure: Option<::ValueList<String>>,
        /// Property `Deploy`.
        #[serde(rename = "Deploy")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub deploy: Option<::ValueList<String>>,
        /// Property `Setup`.
        #[serde(rename = "Setup")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub setup: Option<::ValueList<String>>,
        /// Property `Shutdown`.
        #[serde(rename = "Shutdown")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub shutdown: Option<::ValueList<String>>,
        /// Property `Undeploy`.
        #[serde(rename = "Undeploy")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub undeploy: Option<::ValueList<String>>,
    }

    cfn_internal__inherit_codec_impls!(Recipes);

    /// The [`AWS::OpsWorks::Layer.ShutdownEventConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-lifecycleeventconfiguration-shutdowneventconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ShutdownEventConfiguration {
        /// Property `DelayUntilElbConnectionsDrained`.
        #[serde(rename = "DelayUntilElbConnectionsDrained")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub delay_until_elb_connections_drained: Option<::Value<bool>>,
        /// Property `ExecutionTimeout`.
        #[serde(rename = "ExecutionTimeout")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub execution_timeout: Option<::Value<u32>>,
    }

    cfn_internal__inherit_codec_impls!(ShutdownEventConfiguration);

    /// The [`AWS::OpsWorks::Layer.VolumeConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-volumeconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VolumeConfiguration {
        /// Property `Iops`.
        #[serde(rename = "Iops")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub iops: Option<::Value<u32>>,
        /// Property `MountPoint`.
        #[serde(rename = "MountPoint")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub mount_point: Option<::Value<String>>,
        /// Property `NumberOfDisks`.
        #[serde(rename = "NumberOfDisks")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub number_of_disks: Option<::Value<u32>>,
        /// Property `RaidLevel`.
        #[serde(rename = "RaidLevel")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub raid_level: Option<::Value<u32>>,
        /// Property `Size`.
        #[serde(rename = "Size")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub size: Option<::Value<u32>>,
        /// Property `VolumeType`.
        #[serde(rename = "VolumeType")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub volume_type: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(VolumeConfiguration);
}

pub mod stack {
    //! Property types for the `Stack` resource.

    /// The [`AWS::OpsWorks::Stack.ChefConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-chefconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ChefConfiguration {
        /// Property `BerkshelfVersion`.
        #[serde(rename = "BerkshelfVersion")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub berkshelf_version: Option<::Value<String>>,
        /// Property `ManageBerkshelf`.
        #[serde(rename = "ManageBerkshelf")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub manage_berkshelf: Option<::Value<bool>>,
    }

    cfn_internal__inherit_codec_impls!(ChefConfiguration);

    /// The [`AWS::OpsWorks::Stack.ElasticIp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-elasticip.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ElasticIp {
        /// Property `Ip`.
        #[serde(rename = "Ip")]
        pub ip: ::Value<String>,
        /// Property `Name`.
        #[serde(rename = "Name")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(ElasticIp);

    /// The [`AWS::OpsWorks::Stack.RdsDbInstance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-rdsdbinstance.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RdsDbInstance {
        /// Property `DbPassword`.
        #[serde(rename = "DbPassword")]
        pub db_password: ::Value<String>,
        /// Property `DbUser`.
        #[serde(rename = "DbUser")]
        pub db_user: ::Value<String>,
        /// Property `RdsDbInstanceArn`.
        #[serde(rename = "RdsDbInstanceArn")]
        pub rds_db_instance_arn: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(RdsDbInstance);

    /// The [`AWS::OpsWorks::Stack.Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-source.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Source {
        /// Property `Password`.
        #[serde(rename = "Password")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub password: Option<::Value<String>>,
        /// Property `Revision`.
        #[serde(rename = "Revision")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub revision: Option<::Value<String>>,
        /// Property `SshKey`.
        #[serde(rename = "SshKey")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ssh_key: Option<::Value<String>>,
        /// Property `Type`.
        #[serde(rename = "Type")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<::Value<String>>,
        /// Property `Url`.
        #[serde(rename = "Url")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub url: Option<::Value<String>>,
        /// Property `Username`.
        #[serde(rename = "Username")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub username: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(Source);

    /// The [`AWS::OpsWorks::Stack.StackConfigurationManager`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-stackconfigmanager.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StackConfigurationManager {
        /// Property `Name`.
        #[serde(rename = "Name")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<::Value<String>>,
        /// Property `Version`.
        #[serde(rename = "Version")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub version: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(StackConfigurationManager);
}
