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
    #[serde(rename = "ClusterName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<::Value<String>>,
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
    #[serde(rename = "Cluster")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster: Option<::Value<String>>,
    /// Property `DeploymentConfiguration`.
    #[serde(rename = "DeploymentConfiguration")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployment_configuration: Option<::Value<self::service::DeploymentConfiguration>>,
    /// Property `DesiredCount`.
    #[serde(rename = "DesiredCount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desired_count: Option<::Value<u32>>,
    /// Property `HealthCheckGracePeriodSeconds`.
    #[serde(rename = "HealthCheckGracePeriodSeconds")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health_check_grace_period_seconds: Option<::Value<u32>>,
    /// Property `LaunchType`.
    #[serde(rename = "LaunchType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<::Value<String>>,
    /// Property `LoadBalancers`.
    #[serde(rename = "LoadBalancers")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_balancers: Option<::ValueList<self::service::LoadBalancer>>,
    /// Property `NetworkConfiguration`.
    #[serde(rename = "NetworkConfiguration")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<::Value<self::service::NetworkConfiguration>>,
    /// Property `PlacementConstraints`.
    #[serde(rename = "PlacementConstraints")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placement_constraints: Option<::ValueList<self::service::PlacementConstraint>>,
    /// Property `PlacementStrategies`.
    #[serde(rename = "PlacementStrategies")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placement_strategies: Option<::ValueList<self::service::PlacementStrategy>>,
    /// Property `PlatformVersion`.
    #[serde(rename = "PlatformVersion")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<::Value<String>>,
    /// Property `Role`.
    #[serde(rename = "Role")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<::Value<String>>,
    /// Property `ServiceName`.
    #[serde(rename = "ServiceName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_name: Option<::Value<String>>,
    /// Property `TaskDefinition`.
    #[serde(rename = "TaskDefinition")]
    pub task_definition: ::Value<String>,
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
    #[serde(rename = "ContainerDefinitions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container_definitions: Option<::ValueList<self::task_definition::ContainerDefinition>>,
    /// Property `Cpu`.
    #[serde(rename = "Cpu")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<::Value<String>>,
    /// Property `ExecutionRoleArn`.
    #[serde(rename = "ExecutionRoleArn")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<::Value<String>>,
    /// Property `Family`.
    #[serde(rename = "Family")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<::Value<String>>,
    /// Property `Memory`.
    #[serde(rename = "Memory")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<::Value<String>>,
    /// Property `NetworkMode`.
    #[serde(rename = "NetworkMode")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network_mode: Option<::Value<String>>,
    /// Property `PlacementConstraints`.
    #[serde(rename = "PlacementConstraints")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placement_constraints: Option<::ValueList<self::task_definition::TaskDefinitionPlacementConstraint>>,
    /// Property `RequiresCompatibilities`.
    #[serde(rename = "RequiresCompatibilities")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requires_compatibilities: Option<::ValueList<String>>,
    /// Property `TaskRoleArn`.
    #[serde(rename = "TaskRoleArn")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_role_arn: Option<::Value<String>>,
    /// Property `Volumes`.
    #[serde(rename = "Volumes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volumes: Option<::ValueList<self::task_definition::Volume>>,
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
        #[serde(rename = "AssignPublicIp")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub assign_public_ip: Option<::Value<String>>,
        /// Property `SecurityGroups`.
        #[serde(rename = "SecurityGroups")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub security_groups: Option<::ValueList<String>>,
        /// Property `Subnets`.
        #[serde(rename = "Subnets")]
        pub subnets: ::ValueList<String>,
    }

    cfn_internal__inherit_codec_impls!(AwsVpcConfiguration);

    /// The [`AWS::ECS::Service.DeploymentConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-deploymentconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DeploymentConfiguration {
        /// Property `MaximumPercent`.
        #[serde(rename = "MaximumPercent")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub maximum_percent: Option<::Value<u32>>,
        /// Property `MinimumHealthyPercent`.
        #[serde(rename = "MinimumHealthyPercent")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub minimum_healthy_percent: Option<::Value<u32>>,
    }

    cfn_internal__inherit_codec_impls!(DeploymentConfiguration);

    /// The [`AWS::ECS::Service.LoadBalancer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-loadbalancers.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LoadBalancer {
        /// Property `ContainerName`.
        #[serde(rename = "ContainerName")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub container_name: Option<::Value<String>>,
        /// Property `ContainerPort`.
        #[serde(rename = "ContainerPort")]
        pub container_port: ::Value<u32>,
        /// Property `LoadBalancerName`.
        #[serde(rename = "LoadBalancerName")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub load_balancer_name: Option<::Value<String>>,
        /// Property `TargetGroupArn`.
        #[serde(rename = "TargetGroupArn")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub target_group_arn: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(LoadBalancer);

    /// The [`AWS::ECS::Service.NetworkConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-networkconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct NetworkConfiguration {
        /// Property `AwsvpcConfiguration`.
        #[serde(rename = "AwsvpcConfiguration")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub awsvpc_configuration: Option<::Value<AwsVpcConfiguration>>,
    }

    cfn_internal__inherit_codec_impls!(NetworkConfiguration);

    /// The [`AWS::ECS::Service.PlacementConstraint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-placementconstraint.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PlacementConstraint {
        /// Property `Expression`.
        #[serde(rename = "Expression")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub expression: Option<::Value<String>>,
        /// Property `Type`.
        #[serde(rename = "Type")]
        pub type_: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(PlacementConstraint);

    /// The [`AWS::ECS::Service.PlacementStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-placementstrategy.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PlacementStrategy {
        /// Property `Field`.
        #[serde(rename = "Field")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub field: Option<::Value<String>>,
        /// Property `Type`.
        #[serde(rename = "Type")]
        pub type_: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(PlacementStrategy);
}

pub mod task_definition {
    //! Property types for the `TaskDefinition` resource.

    /// The [`AWS::ECS::TaskDefinition.ContainerDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinitions.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ContainerDefinition {
        /// Property `Command`.
        #[serde(rename = "Command")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub command: Option<::ValueList<String>>,
        /// Property `Cpu`.
        #[serde(rename = "Cpu")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cpu: Option<::Value<u32>>,
        /// Property `DisableNetworking`.
        #[serde(rename = "DisableNetworking")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub disable_networking: Option<::Value<bool>>,
        /// Property `DnsSearchDomains`.
        #[serde(rename = "DnsSearchDomains")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub dns_search_domains: Option<::ValueList<String>>,
        /// Property `DnsServers`.
        #[serde(rename = "DnsServers")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub dns_servers: Option<::ValueList<String>>,
        /// Property `DockerLabels`.
        #[serde(rename = "DockerLabels")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub docker_labels: Option<::std::collections::HashMap<String, ::Value<String>>>,
        /// Property `DockerSecurityOptions`.
        #[serde(rename = "DockerSecurityOptions")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub docker_security_options: Option<::ValueList<String>>,
        /// Property `EntryPoint`.
        #[serde(rename = "EntryPoint")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub entry_point: Option<::ValueList<String>>,
        /// Property `Environment`.
        #[serde(rename = "Environment")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub environment: Option<::ValueList<KeyValuePair>>,
        /// Property `Essential`.
        #[serde(rename = "Essential")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub essential: Option<::Value<bool>>,
        /// Property `ExtraHosts`.
        #[serde(rename = "ExtraHosts")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub extra_hosts: Option<::ValueList<HostEntry>>,
        /// Property `Hostname`.
        #[serde(rename = "Hostname")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub hostname: Option<::Value<String>>,
        /// Property `Image`.
        #[serde(rename = "Image")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub image: Option<::Value<String>>,
        /// Property `Links`.
        #[serde(rename = "Links")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub links: Option<::ValueList<String>>,
        /// Property `LinuxParameters`.
        #[serde(rename = "LinuxParameters")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub linux_parameters: Option<::Value<LinuxParameters>>,
        /// Property `LogConfiguration`.
        #[serde(rename = "LogConfiguration")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub log_configuration: Option<::Value<LogConfiguration>>,
        /// Property `Memory`.
        #[serde(rename = "Memory")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub memory: Option<::Value<u32>>,
        /// Property `MemoryReservation`.
        #[serde(rename = "MemoryReservation")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub memory_reservation: Option<::Value<u32>>,
        /// Property `MountPoints`.
        #[serde(rename = "MountPoints")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub mount_points: Option<::ValueList<MountPoint>>,
        /// Property `Name`.
        #[serde(rename = "Name")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<::Value<String>>,
        /// Property `PortMappings`.
        #[serde(rename = "PortMappings")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub port_mappings: Option<::ValueList<PortMapping>>,
        /// Property `Privileged`.
        #[serde(rename = "Privileged")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub privileged: Option<::Value<bool>>,
        /// Property `ReadonlyRootFilesystem`.
        #[serde(rename = "ReadonlyRootFilesystem")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub readonly_root_filesystem: Option<::Value<bool>>,
        /// Property `Ulimits`.
        #[serde(rename = "Ulimits")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ulimits: Option<::ValueList<Ulimit>>,
        /// Property `User`.
        #[serde(rename = "User")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub user: Option<::Value<String>>,
        /// Property `VolumesFrom`.
        #[serde(rename = "VolumesFrom")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub volumes_from: Option<::ValueList<VolumeFrom>>,
        /// Property `WorkingDirectory`.
        #[serde(rename = "WorkingDirectory")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub working_directory: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(ContainerDefinition);

    /// The [`AWS::ECS::TaskDefinition.Device`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-device.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Device {
        /// Property `ContainerPath`.
        #[serde(rename = "ContainerPath")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub container_path: Option<::Value<String>>,
        /// Property `HostPath`.
        #[serde(rename = "HostPath")]
        pub host_path: ::Value<String>,
        /// Property `Permissions`.
        #[serde(rename = "Permissions")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub permissions: Option<::ValueList<String>>,
    }

    cfn_internal__inherit_codec_impls!(Device);

    /// The [`AWS::ECS::TaskDefinition.HostEntry`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinitions-hostentry.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct HostEntry {
        /// Property `Hostname`.
        #[serde(rename = "Hostname")]
        pub hostname: ::Value<String>,
        /// Property `IpAddress`.
        #[serde(rename = "IpAddress")]
        pub ip_address: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(HostEntry);

    /// The [`AWS::ECS::TaskDefinition.HostVolumeProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-volumes-host.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct HostVolumeProperties {
        /// Property `SourcePath`.
        #[serde(rename = "SourcePath")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub source_path: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(HostVolumeProperties);

    /// The [`AWS::ECS::TaskDefinition.KernelCapabilities`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-kernelcapabilities.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct KernelCapabilities {
        /// Property `Add`.
        #[serde(rename = "Add")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub add: Option<::ValueList<String>>,
        /// Property `Drop`.
        #[serde(rename = "Drop")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub drop: Option<::ValueList<String>>,
    }

    cfn_internal__inherit_codec_impls!(KernelCapabilities);

    /// The [`AWS::ECS::TaskDefinition.KeyValuePair`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinitions-environment.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct KeyValuePair {
        /// Property `Name`.
        #[serde(rename = "Name")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<::Value<String>>,
        /// Property `Value`.
        #[serde(rename = "Value")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub value: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(KeyValuePair);

    /// The [`AWS::ECS::TaskDefinition.LinuxParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-linuxparameters.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LinuxParameters {
        /// Property `Capabilities`.
        #[serde(rename = "Capabilities")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub capabilities: Option<::Value<KernelCapabilities>>,
        /// Property `Devices`.
        #[serde(rename = "Devices")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub devices: Option<::ValueList<Device>>,
        /// Property `InitProcessEnabled`.
        #[serde(rename = "InitProcessEnabled")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub init_process_enabled: Option<::Value<bool>>,
    }

    cfn_internal__inherit_codec_impls!(LinuxParameters);

    /// The [`AWS::ECS::TaskDefinition.LogConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinitions-logconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LogConfiguration {
        /// Property `LogDriver`.
        #[serde(rename = "LogDriver")]
        pub log_driver: ::Value<String>,
        /// Property `Options`.
        #[serde(rename = "Options")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub options: Option<::std::collections::HashMap<String, ::Value<String>>>,
    }

    cfn_internal__inherit_codec_impls!(LogConfiguration);

    /// The [`AWS::ECS::TaskDefinition.MountPoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinitions-mountpoints.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MountPoint {
        /// Property `ContainerPath`.
        #[serde(rename = "ContainerPath")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub container_path: Option<::Value<String>>,
        /// Property `ReadOnly`.
        #[serde(rename = "ReadOnly")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub read_only: Option<::Value<bool>>,
        /// Property `SourceVolume`.
        #[serde(rename = "SourceVolume")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub source_volume: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(MountPoint);

    /// The [`AWS::ECS::TaskDefinition.PortMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinitions-portmappings.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PortMapping {
        /// Property `ContainerPort`.
        #[serde(rename = "ContainerPort")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub container_port: Option<::Value<u32>>,
        /// Property `HostPort`.
        #[serde(rename = "HostPort")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub host_port: Option<::Value<u32>>,
        /// Property `Protocol`.
        #[serde(rename = "Protocol")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub protocol: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(PortMapping);

    /// The [`AWS::ECS::TaskDefinition.TaskDefinitionPlacementConstraint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-taskdefinitionplacementconstraint.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TaskDefinitionPlacementConstraint {
        /// Property `Expression`.
        #[serde(rename = "Expression")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub expression: Option<::Value<String>>,
        /// Property `Type`.
        #[serde(rename = "Type")]
        pub type_: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(TaskDefinitionPlacementConstraint);

    /// The [`AWS::ECS::TaskDefinition.Ulimit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinitions-ulimit.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Ulimit {
        /// Property `HardLimit`.
        #[serde(rename = "HardLimit")]
        pub hard_limit: ::Value<u32>,
        /// Property `Name`.
        #[serde(rename = "Name")]
        pub name: ::Value<String>,
        /// Property `SoftLimit`.
        #[serde(rename = "SoftLimit")]
        pub soft_limit: ::Value<u32>,
    }

    cfn_internal__inherit_codec_impls!(Ulimit);

    /// The [`AWS::ECS::TaskDefinition.Volume`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-volumes.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Volume {
        /// Property `Host`.
        #[serde(rename = "Host")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub host: Option<::Value<HostVolumeProperties>>,
        /// Property `Name`.
        #[serde(rename = "Name")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(Volume);

    /// The [`AWS::ECS::TaskDefinition.VolumeFrom`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinitions-volumesfrom.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VolumeFrom {
        /// Property `ReadOnly`.
        #[serde(rename = "ReadOnly")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub read_only: Option<::Value<bool>>,
        /// Property `SourceContainer`.
        #[serde(rename = "SourceContainer")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub source_container: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(VolumeFrom);
}
