/// The [`AWS::OpsWorks::App`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-app.html) resource.
#[derive(Serialize, Deserialize)]
pub struct App {
    properties: AppProperties
}

/// Properties for the `App` resource.
#[derive(Serialize, Deserialize)]
pub struct AppProperties {
    #[serde(rename="AppSource")]
    pub app_source: (),
    #[serde(rename="Attributes")]
    pub attributes: (),
    #[serde(rename="DataSources")]
    pub data_sources: (),
    #[serde(rename="Description")]
    pub description: (),
    #[serde(rename="Domains")]
    pub domains: (),
    #[serde(rename="EnableSsl")]
    pub enable_ssl: (),
    #[serde(rename="Environment")]
    pub environment: (),
    #[serde(rename="Name")]
    pub name: (),
    #[serde(rename="Shortname")]
    pub shortname: (),
    #[serde(rename="SslConfiguration")]
    pub ssl_configuration: (),
    #[serde(rename="StackId")]
    pub stack_id: (),
    #[serde(rename="Type")]
    pub type_: (),
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

impl From<AppProperties> for App {
    fn from(properties: AppProperties) -> App {
        App { properties }
    }
}

/// The [`AWS::OpsWorks::ElasticLoadBalancerAttachment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-elbattachment.html) resource.
#[derive(Serialize, Deserialize)]
pub struct ElasticLoadBalancerAttachment {
    properties: ElasticLoadBalancerAttachmentProperties
}

/// Properties for the `ElasticLoadBalancerAttachment` resource.
#[derive(Serialize, Deserialize)]
pub struct ElasticLoadBalancerAttachmentProperties {
    #[serde(rename="ElasticLoadBalancerName")]
    pub elastic_load_balancer_name: (),
    #[serde(rename="LayerId")]
    pub layer_id: (),
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

impl From<ElasticLoadBalancerAttachmentProperties> for ElasticLoadBalancerAttachment {
    fn from(properties: ElasticLoadBalancerAttachmentProperties) -> ElasticLoadBalancerAttachment {
        ElasticLoadBalancerAttachment { properties }
    }
}

/// The [`AWS::OpsWorks::Instance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-instance.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Instance {
    properties: InstanceProperties
}

/// Properties for the `Instance` resource.
#[derive(Serialize, Deserialize)]
pub struct InstanceProperties {
    #[serde(rename="AgentVersion")]
    pub agent_version: (),
    #[serde(rename="AmiId")]
    pub ami_id: (),
    #[serde(rename="Architecture")]
    pub architecture: (),
    #[serde(rename="AutoScalingType")]
    pub auto_scaling_type: (),
    #[serde(rename="AvailabilityZone")]
    pub availability_zone: (),
    #[serde(rename="BlockDeviceMappings")]
    pub block_device_mappings: (),
    #[serde(rename="EbsOptimized")]
    pub ebs_optimized: (),
    #[serde(rename="ElasticIps")]
    pub elastic_ips: (),
    #[serde(rename="Hostname")]
    pub hostname: (),
    #[serde(rename="InstallUpdatesOnBoot")]
    pub install_updates_on_boot: (),
    #[serde(rename="InstanceType")]
    pub instance_type: (),
    #[serde(rename="LayerIds")]
    pub layer_ids: (),
    #[serde(rename="Os")]
    pub os: (),
    #[serde(rename="RootDeviceType")]
    pub root_device_type: (),
    #[serde(rename="SshKeyName")]
    pub ssh_key_name: (),
    #[serde(rename="StackId")]
    pub stack_id: (),
    #[serde(rename="SubnetId")]
    pub subnet_id: (),
    #[serde(rename="Tenancy")]
    pub tenancy: (),
    #[serde(rename="TimeBasedAutoScaling")]
    pub time_based_auto_scaling: (),
    #[serde(rename="VirtualizationType")]
    pub virtualization_type: (),
    #[serde(rename="Volumes")]
    pub volumes: (),
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

impl From<InstanceProperties> for Instance {
    fn from(properties: InstanceProperties) -> Instance {
        Instance { properties }
    }
}

/// The [`AWS::OpsWorks::Layer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-layer.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Layer {
    properties: LayerProperties
}

/// Properties for the `Layer` resource.
#[derive(Serialize, Deserialize)]
pub struct LayerProperties {
    #[serde(rename="Attributes")]
    pub attributes: (),
    #[serde(rename="AutoAssignElasticIps")]
    pub auto_assign_elastic_ips: (),
    #[serde(rename="AutoAssignPublicIps")]
    pub auto_assign_public_ips: (),
    #[serde(rename="CustomInstanceProfileArn")]
    pub custom_instance_profile_arn: (),
    #[serde(rename="CustomJson")]
    pub custom_json: (),
    #[serde(rename="CustomRecipes")]
    pub custom_recipes: (),
    #[serde(rename="CustomSecurityGroupIds")]
    pub custom_security_group_ids: (),
    #[serde(rename="EnableAutoHealing")]
    pub enable_auto_healing: (),
    #[serde(rename="InstallUpdatesOnBoot")]
    pub install_updates_on_boot: (),
    #[serde(rename="LifecycleEventConfiguration")]
    pub lifecycle_event_configuration: (),
    #[serde(rename="LoadBasedAutoScaling")]
    pub load_based_auto_scaling: (),
    #[serde(rename="Name")]
    pub name: (),
    #[serde(rename="Packages")]
    pub packages: (),
    #[serde(rename="Shortname")]
    pub shortname: (),
    #[serde(rename="StackId")]
    pub stack_id: (),
    #[serde(rename="Tags")]
    pub tags: (),
    #[serde(rename="Type")]
    pub type_: (),
    #[serde(rename="UseEbsOptimizedInstances")]
    pub use_ebs_optimized_instances: (),
    #[serde(rename="VolumeConfigurations")]
    pub volume_configurations: (),
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

impl From<LayerProperties> for Layer {
    fn from(properties: LayerProperties) -> Layer {
        Layer { properties }
    }
}

/// The [`AWS::OpsWorks::Stack`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-stack.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Stack {
    properties: StackProperties
}

/// Properties for the `Stack` resource.
#[derive(Serialize, Deserialize)]
pub struct StackProperties {
    #[serde(rename="AgentVersion")]
    pub agent_version: (),
    #[serde(rename="Attributes")]
    pub attributes: (),
    #[serde(rename="ChefConfiguration")]
    pub chef_configuration: (),
    #[serde(rename="CloneAppIds")]
    pub clone_app_ids: (),
    #[serde(rename="ClonePermissions")]
    pub clone_permissions: (),
    #[serde(rename="ConfigurationManager")]
    pub configuration_manager: (),
    #[serde(rename="CustomCookbooksSource")]
    pub custom_cookbooks_source: (),
    #[serde(rename="CustomJson")]
    pub custom_json: (),
    #[serde(rename="DefaultAvailabilityZone")]
    pub default_availability_zone: (),
    #[serde(rename="DefaultInstanceProfileArn")]
    pub default_instance_profile_arn: (),
    #[serde(rename="DefaultOs")]
    pub default_os: (),
    #[serde(rename="DefaultRootDeviceType")]
    pub default_root_device_type: (),
    #[serde(rename="DefaultSshKeyName")]
    pub default_ssh_key_name: (),
    #[serde(rename="DefaultSubnetId")]
    pub default_subnet_id: (),
    #[serde(rename="EcsClusterArn")]
    pub ecs_cluster_arn: (),
    #[serde(rename="ElasticIps")]
    pub elastic_ips: (),
    #[serde(rename="HostnameTheme")]
    pub hostname_theme: (),
    #[serde(rename="Name")]
    pub name: (),
    #[serde(rename="RdsDbInstances")]
    pub rds_db_instances: (),
    #[serde(rename="ServiceRoleArn")]
    pub service_role_arn: (),
    #[serde(rename="SourceStackId")]
    pub source_stack_id: (),
    #[serde(rename="Tags")]
    pub tags: (),
    #[serde(rename="UseCustomCookbooks")]
    pub use_custom_cookbooks: (),
    #[serde(rename="UseOpsworksSecurityGroups")]
    pub use_opsworks_security_groups: (),
    #[serde(rename="VpcId")]
    pub vpc_id: (),
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

impl From<StackProperties> for Stack {
    fn from(properties: StackProperties) -> Stack {
        Stack { properties }
    }
}

/// The [`AWS::OpsWorks::UserProfile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-userprofile.html) resource.
#[derive(Serialize, Deserialize)]
pub struct UserProfile {
    properties: UserProfileProperties
}

/// Properties for the `UserProfile` resource.
#[derive(Serialize, Deserialize)]
pub struct UserProfileProperties {
    #[serde(rename="AllowSelfManagement")]
    pub allow_self_management: (),
    #[serde(rename="IamUserArn")]
    pub iam_user_arn: (),
    #[serde(rename="SshPublicKey")]
    pub ssh_public_key: (),
    #[serde(rename="SshUsername")]
    pub ssh_username: (),
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

impl From<UserProfileProperties> for UserProfile {
    fn from(properties: UserProfileProperties) -> UserProfile {
        UserProfile { properties }
    }
}

/// The [`AWS::OpsWorks::Volume`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-volume.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Volume {
    properties: VolumeProperties
}

/// Properties for the `Volume` resource.
#[derive(Serialize, Deserialize)]
pub struct VolumeProperties {
    #[serde(rename="Ec2VolumeId")]
    pub ec2_volume_id: (),
    #[serde(rename="MountPoint")]
    pub mount_point: (),
    #[serde(rename="Name")]
    pub name: (),
    #[serde(rename="StackId")]
    pub stack_id: (),
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

impl From<VolumeProperties> for Volume {
    fn from(properties: VolumeProperties) -> Volume {
        Volume { properties }
    }
}

