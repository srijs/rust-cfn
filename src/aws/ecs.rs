//! Types for the `ECS` service.

/// The [`AWS::ECS::Cluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-cluster.html) resource type.
#[derive(Debug)]
pub struct Cluster {
    properties: ClusterProperties
}

/// Properties for the `Cluster` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterProperties {
    /// Property `ClusterName`.
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

impl ::private::Sealed for Cluster {}

impl From<ClusterProperties> for Cluster {
    fn from(properties: ClusterProperties) -> Cluster {
        Cluster { properties }
    }
}

/// The [`AWS::ECS::Service`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-service.html) resource type.
#[derive(Debug)]
pub struct Service {
    properties: ServiceProperties
}

/// Properties for the `Service` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceProperties {
    /// Property `Cluster`.
    #[serde(rename="Cluster")]
    pub cluster: String,
    /// Property `DeploymentConfiguration`.
    #[serde(rename="DeploymentConfiguration")]
    pub deployment_configuration: self::service::DeploymentConfiguration,
    /// Property `DesiredCount`.
    #[serde(rename="DesiredCount")]
    pub desired_count: u32,
    /// Property `HealthCheckGracePeriodSeconds`.
    #[serde(rename="HealthCheckGracePeriodSeconds")]
    pub health_check_grace_period_seconds: u32,
    /// Property `LaunchType`.
    #[serde(rename="LaunchType")]
    pub launch_type: String,
    /// Property `LoadBalancers`.
    #[serde(rename="LoadBalancers")]
    pub load_balancers: Vec<self::service::LoadBalancer>,
    /// Property `NetworkConfiguration`.
    #[serde(rename="NetworkConfiguration")]
    pub network_configuration: self::service::NetworkConfiguration,
    /// Property `PlacementConstraints`.
    #[serde(rename="PlacementConstraints")]
    pub placement_constraints: Vec<self::service::PlacementConstraint>,
    /// Property `PlacementStrategies`.
    #[serde(rename="PlacementStrategies")]
    pub placement_strategies: Vec<self::service::PlacementStrategy>,
    /// Property `PlatformVersion`.
    #[serde(rename="PlatformVersion")]
    pub platform_version: String,
    /// Property `Role`.
    #[serde(rename="Role")]
    pub role: String,
    /// Property `ServiceName`.
    #[serde(rename="ServiceName")]
    pub service_name: String,
    /// Property `TaskDefinition`.
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

impl ::private::Sealed for Service {}

impl From<ServiceProperties> for Service {
    fn from(properties: ServiceProperties) -> Service {
        Service { properties }
    }
}

/// The [`AWS::ECS::TaskDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-taskdefinition.html) resource type.
#[derive(Debug)]
pub struct TaskDefinition {
    properties: TaskDefinitionProperties
}

/// Properties for the `TaskDefinition` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskDefinitionProperties {
    /// Property `ContainerDefinitions`.
    #[serde(rename="ContainerDefinitions")]
    pub container_definitions: Vec<self::task_definition::ContainerDefinition>,
    /// Property `Cpu`.
    #[serde(rename="Cpu")]
    pub cpu: String,
    /// Property `ExecutionRoleArn`.
    #[serde(rename="ExecutionRoleArn")]
    pub execution_role_arn: String,
    /// Property `Family`.
    #[serde(rename="Family")]
    pub family: String,
    /// Property `Memory`.
    #[serde(rename="Memory")]
    pub memory: String,
    /// Property `NetworkMode`.
    #[serde(rename="NetworkMode")]
    pub network_mode: String,
    /// Property `PlacementConstraints`.
    #[serde(rename="PlacementConstraints")]
    pub placement_constraints: Vec<self::task_definition::TaskDefinitionPlacementConstraint>,
    /// Property `RequiresCompatibilities`.
    #[serde(rename="RequiresCompatibilities")]
    pub requires_compatibilities: Vec<String>,
    /// Property `TaskRoleArn`.
    #[serde(rename="TaskRoleArn")]
    pub task_role_arn: String,
    /// Property `Volumes`.
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

impl ::private::Sealed for TaskDefinition {}

impl From<TaskDefinitionProperties> for TaskDefinition {
    fn from(properties: TaskDefinitionProperties) -> TaskDefinition {
        TaskDefinition { properties }
    }
}

pub mod service {
    //! Property types for the `Service` resource.

    /// The [`AWS::ECS::Service.AwsVpcConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-awsvpcconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AwsVpcConfiguration {
        /// Property `AssignPublicIp`.
        #[serde(rename="AssignPublicIp")]
        pub assign_public_ip: String,
        /// Property `SecurityGroups`.
        #[serde(rename="SecurityGroups")]
        pub security_groups: Vec<String>,
        /// Property `Subnets`.
        #[serde(rename="Subnets")]
        pub subnets: Vec<String>,
    }

    /// The [`AWS::ECS::Service.DeploymentConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-deploymentconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DeploymentConfiguration {
        /// Property `MaximumPercent`.
        #[serde(rename="MaximumPercent")]
        pub maximum_percent: u32,
        /// Property `MinimumHealthyPercent`.
        #[serde(rename="MinimumHealthyPercent")]
        pub minimum_healthy_percent: u32,
    }

    /// The [`AWS::ECS::Service.LoadBalancer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-loadbalancers.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LoadBalancer {
        /// Property `ContainerName`.
        #[serde(rename="ContainerName")]
        pub container_name: String,
        /// Property `ContainerPort`.
        #[serde(rename="ContainerPort")]
        pub container_port: u32,
        /// Property `LoadBalancerName`.
        #[serde(rename="LoadBalancerName")]
        pub load_balancer_name: String,
        /// Property `TargetGroupArn`.
        #[serde(rename="TargetGroupArn")]
        pub target_group_arn: String,
    }

    /// The [`AWS::ECS::Service.NetworkConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-networkconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct NetworkConfiguration {
        /// Property `AwsvpcConfiguration`.
        #[serde(rename="AwsvpcConfiguration")]
        pub awsvpc_configuration: AwsVpcConfiguration,
    }

    /// The [`AWS::ECS::Service.PlacementConstraint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-placementconstraint.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PlacementConstraint {
        /// Property `Expression`.
        #[serde(rename="Expression")]
        pub expression: String,
        /// Property `Type`.
        #[serde(rename="Type")]
        pub type_: String,
    }

    /// The [`AWS::ECS::Service.PlacementStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-placementstrategy.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PlacementStrategy {
        /// Property `Field`.
        #[serde(rename="Field")]
        pub field: String,
        /// Property `Type`.
        #[serde(rename="Type")]
        pub type_: String,
    }
}

pub mod task_definition {
    //! Property types for the `TaskDefinition` resource.

    /// The [`AWS::ECS::TaskDefinition.ContainerDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinitions.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ContainerDefinition {
        /// Property `Command`.
        #[serde(rename="Command")]
        pub command: Vec<String>,
        /// Property `Cpu`.
        #[serde(rename="Cpu")]
        pub cpu: u32,
        /// Property `DisableNetworking`.
        #[serde(rename="DisableNetworking")]
        pub disable_networking: bool,
        /// Property `DnsSearchDomains`.
        #[serde(rename="DnsSearchDomains")]
        pub dns_search_domains: Vec<String>,
        /// Property `DnsServers`.
        #[serde(rename="DnsServers")]
        pub dns_servers: Vec<String>,
        /// Property `DockerLabels`.
        #[serde(rename="DockerLabels")]
        pub docker_labels: ::std::collections::HashMap<String, String>,
        /// Property `DockerSecurityOptions`.
        #[serde(rename="DockerSecurityOptions")]
        pub docker_security_options: Vec<String>,
        /// Property `EntryPoint`.
        #[serde(rename="EntryPoint")]
        pub entry_point: Vec<String>,
        /// Property `Environment`.
        #[serde(rename="Environment")]
        pub environment: Vec<KeyValuePair>,
        /// Property `Essential`.
        #[serde(rename="Essential")]
        pub essential: bool,
        /// Property `ExtraHosts`.
        #[serde(rename="ExtraHosts")]
        pub extra_hosts: Vec<HostEntry>,
        /// Property `Hostname`.
        #[serde(rename="Hostname")]
        pub hostname: String,
        /// Property `Image`.
        #[serde(rename="Image")]
        pub image: String,
        /// Property `Links`.
        #[serde(rename="Links")]
        pub links: Vec<String>,
        /// Property `LinuxParameters`.
        #[serde(rename="LinuxParameters")]
        pub linux_parameters: LinuxParameters,
        /// Property `LogConfiguration`.
        #[serde(rename="LogConfiguration")]
        pub log_configuration: LogConfiguration,
        /// Property `Memory`.
        #[serde(rename="Memory")]
        pub memory: u32,
        /// Property `MemoryReservation`.
        #[serde(rename="MemoryReservation")]
        pub memory_reservation: u32,
        /// Property `MountPoints`.
        #[serde(rename="MountPoints")]
        pub mount_points: Vec<MountPoint>,
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
        /// Property `PortMappings`.
        #[serde(rename="PortMappings")]
        pub port_mappings: Vec<PortMapping>,
        /// Property `Privileged`.
        #[serde(rename="Privileged")]
        pub privileged: bool,
        /// Property `ReadonlyRootFilesystem`.
        #[serde(rename="ReadonlyRootFilesystem")]
        pub readonly_root_filesystem: bool,
        /// Property `Ulimits`.
        #[serde(rename="Ulimits")]
        pub ulimits: Vec<Ulimit>,
        /// Property `User`.
        #[serde(rename="User")]
        pub user: String,
        /// Property `VolumesFrom`.
        #[serde(rename="VolumesFrom")]
        pub volumes_from: Vec<VolumeFrom>,
        /// Property `WorkingDirectory`.
        #[serde(rename="WorkingDirectory")]
        pub working_directory: String,
    }

    /// The [`AWS::ECS::TaskDefinition.Device`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-device.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Device {
        /// Property `ContainerPath`.
        #[serde(rename="ContainerPath")]
        pub container_path: String,
        /// Property `HostPath`.
        #[serde(rename="HostPath")]
        pub host_path: String,
        /// Property `Permissions`.
        #[serde(rename="Permissions")]
        pub permissions: Vec<String>,
    }

    /// The [`AWS::ECS::TaskDefinition.HostEntry`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinitions-hostentry.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct HostEntry {
        /// Property `Hostname`.
        #[serde(rename="Hostname")]
        pub hostname: String,
        /// Property `IpAddress`.
        #[serde(rename="IpAddress")]
        pub ip_address: String,
    }

    /// The [`AWS::ECS::TaskDefinition.HostVolumeProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-volumes-host.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct HostVolumeProperties {
        /// Property `SourcePath`.
        #[serde(rename="SourcePath")]
        pub source_path: String,
    }

    /// The [`AWS::ECS::TaskDefinition.KernelCapabilities`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-kernelcapabilities.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct KernelCapabilities {
        /// Property `Add`.
        #[serde(rename="Add")]
        pub add: Vec<String>,
        /// Property `Drop`.
        #[serde(rename="Drop")]
        pub drop: Vec<String>,
    }

    /// The [`AWS::ECS::TaskDefinition.KeyValuePair`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinitions-environment.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct KeyValuePair {
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
        /// Property `Value`.
        #[serde(rename="Value")]
        pub value: String,
    }

    /// The [`AWS::ECS::TaskDefinition.LinuxParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-linuxparameters.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LinuxParameters {
        /// Property `Capabilities`.
        #[serde(rename="Capabilities")]
        pub capabilities: KernelCapabilities,
        /// Property `Devices`.
        #[serde(rename="Devices")]
        pub devices: Vec<Device>,
        /// Property `InitProcessEnabled`.
        #[serde(rename="InitProcessEnabled")]
        pub init_process_enabled: bool,
    }

    /// The [`AWS::ECS::TaskDefinition.LogConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinitions-logconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LogConfiguration {
        /// Property `LogDriver`.
        #[serde(rename="LogDriver")]
        pub log_driver: String,
        /// Property `Options`.
        #[serde(rename="Options")]
        pub options: ::std::collections::HashMap<String, String>,
    }

    /// The [`AWS::ECS::TaskDefinition.MountPoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinitions-mountpoints.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MountPoint {
        /// Property `ContainerPath`.
        #[serde(rename="ContainerPath")]
        pub container_path: String,
        /// Property `ReadOnly`.
        #[serde(rename="ReadOnly")]
        pub read_only: bool,
        /// Property `SourceVolume`.
        #[serde(rename="SourceVolume")]
        pub source_volume: String,
    }

    /// The [`AWS::ECS::TaskDefinition.PortMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinitions-portmappings.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PortMapping {
        /// Property `ContainerPort`.
        #[serde(rename="ContainerPort")]
        pub container_port: u32,
        /// Property `HostPort`.
        #[serde(rename="HostPort")]
        pub host_port: u32,
        /// Property `Protocol`.
        #[serde(rename="Protocol")]
        pub protocol: String,
    }

    /// The [`AWS::ECS::TaskDefinition.TaskDefinitionPlacementConstraint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-taskdefinitionplacementconstraint.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TaskDefinitionPlacementConstraint {
        /// Property `Expression`.
        #[serde(rename="Expression")]
        pub expression: String,
        /// Property `Type`.
        #[serde(rename="Type")]
        pub type_: String,
    }

    /// The [`AWS::ECS::TaskDefinition.Ulimit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinitions-ulimit.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Ulimit {
        /// Property `HardLimit`.
        #[serde(rename="HardLimit")]
        pub hard_limit: u32,
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
        /// Property `SoftLimit`.
        #[serde(rename="SoftLimit")]
        pub soft_limit: u32,
    }

    /// The [`AWS::ECS::TaskDefinition.Volume`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-volumes.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Volume {
        /// Property `Host`.
        #[serde(rename="Host")]
        pub host: HostVolumeProperties,
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
    }

    /// The [`AWS::ECS::TaskDefinition.VolumeFrom`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinitions-volumesfrom.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VolumeFrom {
        /// Property `ReadOnly`.
        #[serde(rename="ReadOnly")]
        pub read_only: bool,
        /// Property `SourceContainer`.
        #[serde(rename="SourceContainer")]
        pub source_container: String,
    }
}
