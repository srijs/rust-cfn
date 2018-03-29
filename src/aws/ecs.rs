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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// Property `DeploymentConfiguration`.
    #[serde(rename="DeploymentConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_configuration: Option<self::service::DeploymentConfiguration>,
    /// Property `DesiredCount`.
    #[serde(rename="DesiredCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_count: Option<u32>,
    /// Property `HealthCheckGracePeriodSeconds`.
    #[serde(rename="HealthCheckGracePeriodSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_grace_period_seconds: Option<u32>,
    /// Property `LaunchType`.
    #[serde(rename="LaunchType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,
    /// Property `LoadBalancers`.
    #[serde(rename="LoadBalancers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancers: Option<Vec<self::service::LoadBalancer>>,
    /// Property `NetworkConfiguration`.
    #[serde(rename="NetworkConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<self::service::NetworkConfiguration>,
    /// Property `PlacementConstraints`.
    #[serde(rename="PlacementConstraints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_constraints: Option<Vec<self::service::PlacementConstraint>>,
    /// Property `PlacementStrategies`.
    #[serde(rename="PlacementStrategies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_strategies: Option<Vec<self::service::PlacementStrategy>>,
    /// Property `PlatformVersion`.
    #[serde(rename="PlatformVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    /// Property `Role`.
    #[serde(rename="Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// Property `ServiceName`.
    #[serde(rename="ServiceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_definitions: Option<Vec<self::task_definition::ContainerDefinition>>,
    /// Property `Cpu`.
    #[serde(rename="Cpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    /// Property `ExecutionRoleArn`.
    #[serde(rename="ExecutionRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    /// Property `Family`.
    #[serde(rename="Family")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    /// Property `Memory`.
    #[serde(rename="Memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
    /// Property `NetworkMode`.
    #[serde(rename="NetworkMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_mode: Option<String>,
    /// Property `PlacementConstraints`.
    #[serde(rename="PlacementConstraints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_constraints: Option<Vec<self::task_definition::TaskDefinitionPlacementConstraint>>,
    /// Property `RequiresCompatibilities`.
    #[serde(rename="RequiresCompatibilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_compatibilities: Option<Vec<String>>,
    /// Property `TaskRoleArn`.
    #[serde(rename="TaskRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_role_arn: Option<String>,
    /// Property `Volumes`.
    #[serde(rename="Volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<self::task_definition::Volume>>,
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
        #[serde(skip_serializing_if = "Option::is_none")]
        pub assign_public_ip: Option<String>,
        /// Property `SecurityGroups`.
        #[serde(rename="SecurityGroups")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub security_groups: Option<Vec<String>>,
        /// Property `Subnets`.
        #[serde(rename="Subnets")]
        pub subnets: Vec<String>,
    }

    /// The [`AWS::ECS::Service.DeploymentConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-deploymentconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DeploymentConfiguration {
        /// Property `MaximumPercent`.
        #[serde(rename="MaximumPercent")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub maximum_percent: Option<u32>,
        /// Property `MinimumHealthyPercent`.
        #[serde(rename="MinimumHealthyPercent")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub minimum_healthy_percent: Option<u32>,
    }

    /// The [`AWS::ECS::Service.LoadBalancer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-loadbalancers.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LoadBalancer {
        /// Property `ContainerName`.
        #[serde(rename="ContainerName")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub container_name: Option<String>,
        /// Property `ContainerPort`.
        #[serde(rename="ContainerPort")]
        pub container_port: u32,
        /// Property `LoadBalancerName`.
        #[serde(rename="LoadBalancerName")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub load_balancer_name: Option<String>,
        /// Property `TargetGroupArn`.
        #[serde(rename="TargetGroupArn")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub target_group_arn: Option<String>,
    }

    /// The [`AWS::ECS::Service.NetworkConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-networkconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct NetworkConfiguration {
        /// Property `AwsvpcConfiguration`.
        #[serde(rename="AwsvpcConfiguration")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub awsvpc_configuration: Option<AwsVpcConfiguration>,
    }

    /// The [`AWS::ECS::Service.PlacementConstraint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-placementconstraint.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PlacementConstraint {
        /// Property `Expression`.
        #[serde(rename="Expression")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub expression: Option<String>,
        /// Property `Type`.
        #[serde(rename="Type")]
        pub type_: String,
    }

    /// The [`AWS::ECS::Service.PlacementStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-placementstrategy.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PlacementStrategy {
        /// Property `Field`.
        #[serde(rename="Field")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub field: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none")]
        pub command: Option<Vec<String>>,
        /// Property `Cpu`.
        #[serde(rename="Cpu")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cpu: Option<u32>,
        /// Property `DisableNetworking`.
        #[serde(rename="DisableNetworking")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub disable_networking: Option<bool>,
        /// Property `DnsSearchDomains`.
        #[serde(rename="DnsSearchDomains")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub dns_search_domains: Option<Vec<String>>,
        /// Property `DnsServers`.
        #[serde(rename="DnsServers")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub dns_servers: Option<Vec<String>>,
        /// Property `DockerLabels`.
        #[serde(rename="DockerLabels")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub docker_labels: Option<::std::collections::HashMap<String, String>>,
        /// Property `DockerSecurityOptions`.
        #[serde(rename="DockerSecurityOptions")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub docker_security_options: Option<Vec<String>>,
        /// Property `EntryPoint`.
        #[serde(rename="EntryPoint")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub entry_point: Option<Vec<String>>,
        /// Property `Environment`.
        #[serde(rename="Environment")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub environment: Option<Vec<KeyValuePair>>,
        /// Property `Essential`.
        #[serde(rename="Essential")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub essential: Option<bool>,
        /// Property `ExtraHosts`.
        #[serde(rename="ExtraHosts")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub extra_hosts: Option<Vec<HostEntry>>,
        /// Property `Hostname`.
        #[serde(rename="Hostname")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub hostname: Option<String>,
        /// Property `Image`.
        #[serde(rename="Image")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub image: Option<String>,
        /// Property `Links`.
        #[serde(rename="Links")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub links: Option<Vec<String>>,
        /// Property `LinuxParameters`.
        #[serde(rename="LinuxParameters")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub linux_parameters: Option<LinuxParameters>,
        /// Property `LogConfiguration`.
        #[serde(rename="LogConfiguration")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub log_configuration: Option<LogConfiguration>,
        /// Property `Memory`.
        #[serde(rename="Memory")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub memory: Option<u32>,
        /// Property `MemoryReservation`.
        #[serde(rename="MemoryReservation")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub memory_reservation: Option<u32>,
        /// Property `MountPoints`.
        #[serde(rename="MountPoints")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub mount_points: Option<Vec<MountPoint>>,
        /// Property `Name`.
        #[serde(rename="Name")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        /// Property `PortMappings`.
        #[serde(rename="PortMappings")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub port_mappings: Option<Vec<PortMapping>>,
        /// Property `Privileged`.
        #[serde(rename="Privileged")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub privileged: Option<bool>,
        /// Property `ReadonlyRootFilesystem`.
        #[serde(rename="ReadonlyRootFilesystem")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub readonly_root_filesystem: Option<bool>,
        /// Property `Ulimits`.
        #[serde(rename="Ulimits")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ulimits: Option<Vec<Ulimit>>,
        /// Property `User`.
        #[serde(rename="User")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub user: Option<String>,
        /// Property `VolumesFrom`.
        #[serde(rename="VolumesFrom")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub volumes_from: Option<Vec<VolumeFrom>>,
        /// Property `WorkingDirectory`.
        #[serde(rename="WorkingDirectory")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub working_directory: Option<String>,
    }

    /// The [`AWS::ECS::TaskDefinition.Device`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-device.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Device {
        /// Property `ContainerPath`.
        #[serde(rename="ContainerPath")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub container_path: Option<String>,
        /// Property `HostPath`.
        #[serde(rename="HostPath")]
        pub host_path: String,
        /// Property `Permissions`.
        #[serde(rename="Permissions")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub permissions: Option<Vec<String>>,
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
        #[serde(skip_serializing_if = "Option::is_none")]
        pub source_path: Option<String>,
    }

    /// The [`AWS::ECS::TaskDefinition.KernelCapabilities`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-kernelcapabilities.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct KernelCapabilities {
        /// Property `Add`.
        #[serde(rename="Add")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub add: Option<Vec<String>>,
        /// Property `Drop`.
        #[serde(rename="Drop")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub drop: Option<Vec<String>>,
    }

    /// The [`AWS::ECS::TaskDefinition.KeyValuePair`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinitions-environment.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct KeyValuePair {
        /// Property `Name`.
        #[serde(rename="Name")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        /// Property `Value`.
        #[serde(rename="Value")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    /// The [`AWS::ECS::TaskDefinition.LinuxParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-linuxparameters.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LinuxParameters {
        /// Property `Capabilities`.
        #[serde(rename="Capabilities")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub capabilities: Option<KernelCapabilities>,
        /// Property `Devices`.
        #[serde(rename="Devices")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub devices: Option<Vec<Device>>,
        /// Property `InitProcessEnabled`.
        #[serde(rename="InitProcessEnabled")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub init_process_enabled: Option<bool>,
    }

    /// The [`AWS::ECS::TaskDefinition.LogConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinitions-logconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LogConfiguration {
        /// Property `LogDriver`.
        #[serde(rename="LogDriver")]
        pub log_driver: String,
        /// Property `Options`.
        #[serde(rename="Options")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub options: Option<::std::collections::HashMap<String, String>>,
    }

    /// The [`AWS::ECS::TaskDefinition.MountPoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinitions-mountpoints.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MountPoint {
        /// Property `ContainerPath`.
        #[serde(rename="ContainerPath")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub container_path: Option<String>,
        /// Property `ReadOnly`.
        #[serde(rename="ReadOnly")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub read_only: Option<bool>,
        /// Property `SourceVolume`.
        #[serde(rename="SourceVolume")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub source_volume: Option<String>,
    }

    /// The [`AWS::ECS::TaskDefinition.PortMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinitions-portmappings.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PortMapping {
        /// Property `ContainerPort`.
        #[serde(rename="ContainerPort")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub container_port: Option<u32>,
        /// Property `HostPort`.
        #[serde(rename="HostPort")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub host_port: Option<u32>,
        /// Property `Protocol`.
        #[serde(rename="Protocol")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub protocol: Option<String>,
    }

    /// The [`AWS::ECS::TaskDefinition.TaskDefinitionPlacementConstraint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-taskdefinitionplacementconstraint.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TaskDefinitionPlacementConstraint {
        /// Property `Expression`.
        #[serde(rename="Expression")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub expression: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none")]
        pub host: Option<HostVolumeProperties>,
        /// Property `Name`.
        #[serde(rename="Name")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    /// The [`AWS::ECS::TaskDefinition.VolumeFrom`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinitions-volumesfrom.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VolumeFrom {
        /// Property `ReadOnly`.
        #[serde(rename="ReadOnly")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub read_only: Option<bool>,
        /// Property `SourceContainer`.
        #[serde(rename="SourceContainer")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub source_container: Option<String>,
    }
}
