/// The [`AWS::ECS::Cluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-cluster.html) resource type.
pub struct Cluster {
    properties: ClusterProperties
}

/// Properties for the `Cluster` resource.
#[derive(Serialize, Deserialize)]
pub struct ClusterProperties {
    #[serde(rename="ClusterName")]
    pub cluster_name: String,
}

impl<'a> ::Resource<'a> for Cluster {
    type Properties = ClusterProperties;
    const TYPE: &'static str = "AWS::ECS::Cluster";
    fn properties(&self) -> &ClusterProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ClusterProperties {
        &mut self.properties
    }
}

impl From<ClusterProperties> for Cluster {
    fn from(properties: ClusterProperties) -> Cluster {
        Cluster { properties }
    }
}

/// The [`AWS::ECS::Service`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-service.html) resource type.
pub struct Service {
    properties: ServiceProperties
}

/// Properties for the `Service` resource.
#[derive(Serialize, Deserialize)]
pub struct ServiceProperties {
    #[serde(rename="Cluster")]
    pub cluster: String,
    #[serde(rename="DeploymentConfiguration")]
    pub deployment_configuration: self::service::DeploymentConfiguration,
    #[serde(rename="DesiredCount")]
    pub desired_count: u32,
    #[serde(rename="HealthCheckGracePeriodSeconds")]
    pub health_check_grace_period_seconds: u32,
    #[serde(rename="LaunchType")]
    pub launch_type: String,
    #[serde(rename="LoadBalancers")]
    pub load_balancers: Vec<self::service::LoadBalancer>,
    #[serde(rename="NetworkConfiguration")]
    pub network_configuration: self::service::NetworkConfiguration,
    #[serde(rename="PlacementConstraints")]
    pub placement_constraints: Vec<self::service::PlacementConstraint>,
    #[serde(rename="PlacementStrategies")]
    pub placement_strategies: Vec<self::service::PlacementStrategy>,
    #[serde(rename="PlatformVersion")]
    pub platform_version: String,
    #[serde(rename="Role")]
    pub role: String,
    #[serde(rename="ServiceName")]
    pub service_name: String,
    #[serde(rename="TaskDefinition")]
    pub task_definition: String,
}

impl<'a> ::Resource<'a> for Service {
    type Properties = ServiceProperties;
    const TYPE: &'static str = "AWS::ECS::Service";
    fn properties(&self) -> &ServiceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ServiceProperties {
        &mut self.properties
    }
}

impl From<ServiceProperties> for Service {
    fn from(properties: ServiceProperties) -> Service {
        Service { properties }
    }
}

/// The [`AWS::ECS::TaskDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-taskdefinition.html) resource type.
pub struct TaskDefinition {
    properties: TaskDefinitionProperties
}

/// Properties for the `TaskDefinition` resource.
#[derive(Serialize, Deserialize)]
pub struct TaskDefinitionProperties {
    #[serde(rename="ContainerDefinitions")]
    pub container_definitions: Vec<self::task_definition::ContainerDefinition>,
    #[serde(rename="Cpu")]
    pub cpu: String,
    #[serde(rename="ExecutionRoleArn")]
    pub execution_role_arn: String,
    #[serde(rename="Family")]
    pub family: String,
    #[serde(rename="Memory")]
    pub memory: String,
    #[serde(rename="NetworkMode")]
    pub network_mode: String,
    #[serde(rename="PlacementConstraints")]
    pub placement_constraints: Vec<self::task_definition::TaskDefinitionPlacementConstraint>,
    #[serde(rename="RequiresCompatibilities")]
    pub requires_compatibilities: Vec<String>,
    #[serde(rename="TaskRoleArn")]
    pub task_role_arn: String,
    #[serde(rename="Volumes")]
    pub volumes: Vec<self::task_definition::Volume>,
}

impl<'a> ::Resource<'a> for TaskDefinition {
    type Properties = TaskDefinitionProperties;
    const TYPE: &'static str = "AWS::ECS::TaskDefinition";
    fn properties(&self) -> &TaskDefinitionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TaskDefinitionProperties {
        &mut self.properties
    }
}

impl From<TaskDefinitionProperties> for TaskDefinition {
    fn from(properties: TaskDefinitionProperties) -> TaskDefinition {
        TaskDefinition { properties }
    }
}

pub mod service {
    /// The [`AWS::ECS::Service.AwsVpcConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-awsvpcconfiguration.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct AwsVpcConfiguration {
        #[serde(rename="AssignPublicIp")]
        pub assign_public_ip: String,
        #[serde(rename="SecurityGroups")]
        pub security_groups: Vec<String>,
        #[serde(rename="Subnets")]
        pub subnets: Vec<String>,
    }

    /// The [`AWS::ECS::Service.DeploymentConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-deploymentconfiguration.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct DeploymentConfiguration {
        #[serde(rename="MaximumPercent")]
        pub maximum_percent: u32,
        #[serde(rename="MinimumHealthyPercent")]
        pub minimum_healthy_percent: u32,
    }

    /// The [`AWS::ECS::Service.LoadBalancer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-loadbalancers.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct LoadBalancer {
        #[serde(rename="ContainerName")]
        pub container_name: String,
        #[serde(rename="ContainerPort")]
        pub container_port: u32,
        #[serde(rename="LoadBalancerName")]
        pub load_balancer_name: String,
        #[serde(rename="TargetGroupArn")]
        pub target_group_arn: String,
    }

    /// The [`AWS::ECS::Service.NetworkConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-networkconfiguration.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct NetworkConfiguration {
        #[serde(rename="AwsvpcConfiguration")]
        pub awsvpc_configuration: AwsVpcConfiguration,
    }

    /// The [`AWS::ECS::Service.PlacementConstraint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-placementconstraint.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct PlacementConstraint {
        #[serde(rename="Expression")]
        pub expression: String,
        #[serde(rename="Type")]
        pub type_: String,
    }

    /// The [`AWS::ECS::Service.PlacementStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-placementstrategy.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct PlacementStrategy {
        #[serde(rename="Field")]
        pub field: String,
        #[serde(rename="Type")]
        pub type_: String,
    }

}

pub mod task_definition {
    /// The [`AWS::ECS::TaskDefinition.ContainerDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinitions.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct ContainerDefinition {
        #[serde(rename="Command")]
        pub command: Vec<String>,
        #[serde(rename="Cpu")]
        pub cpu: u32,
        #[serde(rename="DisableNetworking")]
        pub disable_networking: bool,
        #[serde(rename="DnsSearchDomains")]
        pub dns_search_domains: Vec<String>,
        #[serde(rename="DnsServers")]
        pub dns_servers: Vec<String>,
        #[serde(rename="DockerLabels")]
        pub docker_labels: ::std::collections::HashMap<String, String>,
        #[serde(rename="DockerSecurityOptions")]
        pub docker_security_options: Vec<String>,
        #[serde(rename="EntryPoint")]
        pub entry_point: Vec<String>,
        #[serde(rename="Environment")]
        pub environment: Vec<KeyValuePair>,
        #[serde(rename="Essential")]
        pub essential: bool,
        #[serde(rename="ExtraHosts")]
        pub extra_hosts: Vec<HostEntry>,
        #[serde(rename="Hostname")]
        pub hostname: String,
        #[serde(rename="Image")]
        pub image: String,
        #[serde(rename="Links")]
        pub links: Vec<String>,
        #[serde(rename="LinuxParameters")]
        pub linux_parameters: LinuxParameters,
        #[serde(rename="LogConfiguration")]
        pub log_configuration: LogConfiguration,
        #[serde(rename="Memory")]
        pub memory: u32,
        #[serde(rename="MemoryReservation")]
        pub memory_reservation: u32,
        #[serde(rename="MountPoints")]
        pub mount_points: Vec<MountPoint>,
        #[serde(rename="Name")]
        pub name: String,
        #[serde(rename="PortMappings")]
        pub port_mappings: Vec<PortMapping>,
        #[serde(rename="Privileged")]
        pub privileged: bool,
        #[serde(rename="ReadonlyRootFilesystem")]
        pub readonly_root_filesystem: bool,
        #[serde(rename="Ulimits")]
        pub ulimits: Vec<Ulimit>,
        #[serde(rename="User")]
        pub user: String,
        #[serde(rename="VolumesFrom")]
        pub volumes_from: Vec<VolumeFrom>,
        #[serde(rename="WorkingDirectory")]
        pub working_directory: String,
    }

    /// The [`AWS::ECS::TaskDefinition.Device`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-device.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Device {
        #[serde(rename="ContainerPath")]
        pub container_path: String,
        #[serde(rename="HostPath")]
        pub host_path: String,
        #[serde(rename="Permissions")]
        pub permissions: Vec<String>,
    }

    /// The [`AWS::ECS::TaskDefinition.HostEntry`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinitions-hostentry.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct HostEntry {
        #[serde(rename="Hostname")]
        pub hostname: String,
        #[serde(rename="IpAddress")]
        pub ip_address: String,
    }

    /// The [`AWS::ECS::TaskDefinition.HostVolumeProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-volumes-host.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct HostVolumeProperties {
        #[serde(rename="SourcePath")]
        pub source_path: String,
    }

    /// The [`AWS::ECS::TaskDefinition.KernelCapabilities`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-kernelcapabilities.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct KernelCapabilities {
        #[serde(rename="Add")]
        pub add: Vec<String>,
        #[serde(rename="Drop")]
        pub drop: Vec<String>,
    }

    /// The [`AWS::ECS::TaskDefinition.KeyValuePair`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinitions-environment.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct KeyValuePair {
        #[serde(rename="Name")]
        pub name: String,
        #[serde(rename="Value")]
        pub value: String,
    }

    /// The [`AWS::ECS::TaskDefinition.LinuxParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-linuxparameters.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct LinuxParameters {
        #[serde(rename="Capabilities")]
        pub capabilities: KernelCapabilities,
        #[serde(rename="Devices")]
        pub devices: Vec<Device>,
        #[serde(rename="InitProcessEnabled")]
        pub init_process_enabled: bool,
    }

    /// The [`AWS::ECS::TaskDefinition.LogConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinitions-logconfiguration.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct LogConfiguration {
        #[serde(rename="LogDriver")]
        pub log_driver: String,
        #[serde(rename="Options")]
        pub options: ::std::collections::HashMap<String, String>,
    }

    /// The [`AWS::ECS::TaskDefinition.MountPoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinitions-mountpoints.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct MountPoint {
        #[serde(rename="ContainerPath")]
        pub container_path: String,
        #[serde(rename="ReadOnly")]
        pub read_only: bool,
        #[serde(rename="SourceVolume")]
        pub source_volume: String,
    }

    /// The [`AWS::ECS::TaskDefinition.PortMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinitions-portmappings.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct PortMapping {
        #[serde(rename="ContainerPort")]
        pub container_port: u32,
        #[serde(rename="HostPort")]
        pub host_port: u32,
        #[serde(rename="Protocol")]
        pub protocol: String,
    }

    /// The [`AWS::ECS::TaskDefinition.TaskDefinitionPlacementConstraint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-taskdefinitionplacementconstraint.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct TaskDefinitionPlacementConstraint {
        #[serde(rename="Expression")]
        pub expression: String,
        #[serde(rename="Type")]
        pub type_: String,
    }

    /// The [`AWS::ECS::TaskDefinition.Ulimit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinitions-ulimit.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Ulimit {
        #[serde(rename="HardLimit")]
        pub hard_limit: u32,
        #[serde(rename="Name")]
        pub name: String,
        #[serde(rename="SoftLimit")]
        pub soft_limit: u32,
    }

    /// The [`AWS::ECS::TaskDefinition.Volume`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-volumes.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Volume {
        #[serde(rename="Host")]
        pub host: HostVolumeProperties,
        #[serde(rename="Name")]
        pub name: String,
    }

    /// The [`AWS::ECS::TaskDefinition.VolumeFrom`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinitions-volumesfrom.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct VolumeFrom {
        #[serde(rename="ReadOnly")]
        pub read_only: bool,
        #[serde(rename="SourceContainer")]
        pub source_container: String,
    }

}

