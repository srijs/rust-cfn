//! Types for the `ECS` service.

/// The [`AWS::ECS::Cluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-cluster.html) resource type.
#[derive(Debug)]
pub struct Cluster {
    properties: ClusterProperties
}

/// Properties for the `Cluster` resource.
#[derive(Debug)]
pub struct ClusterProperties {
    /// Property `ClusterName`.
    pub cluster_name: Option<::Value<String>>,
}

impl ::serde::Serialize for ClusterProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterName", &self.cluster_name)?;
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
                let mut cluster_name = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ClusterName" => {
                            cluster_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(ClusterProperties {
                    cluster_name: cluster_name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct ServiceProperties {
    /// Property `Cluster`.
    pub cluster: Option<::Value<String>>,
    /// Property `DeploymentConfiguration`.
    pub deployment_configuration: Option<::Value<self::service::DeploymentConfiguration>>,
    /// Property `DesiredCount`.
    pub desired_count: Option<::Value<u32>>,
    /// Property `HealthCheckGracePeriodSeconds`.
    pub health_check_grace_period_seconds: Option<::Value<u32>>,
    /// Property `LaunchType`.
    pub launch_type: Option<::Value<String>>,
    /// Property `LoadBalancers`.
    pub load_balancers: Option<::ValueList<self::service::LoadBalancer>>,
    /// Property `NetworkConfiguration`.
    pub network_configuration: Option<::Value<self::service::NetworkConfiguration>>,
    /// Property `PlacementConstraints`.
    pub placement_constraints: Option<::ValueList<self::service::PlacementConstraint>>,
    /// Property `PlacementStrategies`.
    pub placement_strategies: Option<::ValueList<self::service::PlacementStrategy>>,
    /// Property `PlatformVersion`.
    pub platform_version: Option<::Value<String>>,
    /// Property `Role`.
    pub role: Option<::Value<String>>,
    /// Property `ServiceName`.
    pub service_name: Option<::Value<String>>,
    /// Property `TaskDefinition`.
    pub task_definition: ::Value<String>,
}

impl ::serde::Serialize for ServiceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cluster", &self.cluster)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeploymentConfiguration", &self.deployment_configuration)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DesiredCount", &self.desired_count)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthCheckGracePeriodSeconds", &self.health_check_grace_period_seconds)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LaunchType", &self.launch_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoadBalancers", &self.load_balancers)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkConfiguration", &self.network_configuration)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PlacementConstraints", &self.placement_constraints)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PlacementStrategies", &self.placement_strategies)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PlatformVersion", &self.platform_version)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Role", &self.role)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceName", &self.service_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TaskDefinition", &self.task_definition)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ServiceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ServiceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ServiceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ServiceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut cluster = None;
                let mut deployment_configuration = None;
                let mut desired_count = None;
                let mut health_check_grace_period_seconds = None;
                let mut launch_type = None;
                let mut load_balancers = None;
                let mut network_configuration = None;
                let mut placement_constraints = None;
                let mut placement_strategies = None;
                let mut platform_version = None;
                let mut role = None;
                let mut service_name = None;
                let mut task_definition = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Cluster" => {
                            cluster = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DeploymentConfiguration" => {
                            deployment_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DesiredCount" => {
                            desired_count = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "HealthCheckGracePeriodSeconds" => {
                            health_check_grace_period_seconds = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "LaunchType" => {
                            launch_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "LoadBalancers" => {
                            load_balancers = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "NetworkConfiguration" => {
                            network_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "PlacementConstraints" => {
                            placement_constraints = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "PlacementStrategies" => {
                            placement_strategies = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "PlatformVersion" => {
                            platform_version = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Role" => {
                            role = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ServiceName" => {
                            service_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "TaskDefinition" => {
                            task_definition = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(ServiceProperties {
                    cluster: cluster,
                    deployment_configuration: deployment_configuration,
                    desired_count: desired_count,
                    health_check_grace_period_seconds: health_check_grace_period_seconds,
                    launch_type: launch_type,
                    load_balancers: load_balancers,
                    network_configuration: network_configuration,
                    placement_constraints: placement_constraints,
                    placement_strategies: placement_strategies,
                    platform_version: platform_version,
                    role: role,
                    service_name: service_name,
                    task_definition: task_definition.ok_or(::serde::de::Error::missing_field("TaskDefinition"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct TaskDefinitionProperties {
    /// Property `ContainerDefinitions`.
    pub container_definitions: Option<::ValueList<self::task_definition::ContainerDefinition>>,
    /// Property `Cpu`.
    pub cpu: Option<::Value<String>>,
    /// Property `ExecutionRoleArn`.
    pub execution_role_arn: Option<::Value<String>>,
    /// Property `Family`.
    pub family: Option<::Value<String>>,
    /// Property `Memory`.
    pub memory: Option<::Value<String>>,
    /// Property `NetworkMode`.
    pub network_mode: Option<::Value<String>>,
    /// Property `PlacementConstraints`.
    pub placement_constraints: Option<::ValueList<self::task_definition::TaskDefinitionPlacementConstraint>>,
    /// Property `RequiresCompatibilities`.
    pub requires_compatibilities: Option<::ValueList<String>>,
    /// Property `TaskRoleArn`.
    pub task_role_arn: Option<::Value<String>>,
    /// Property `Volumes`.
    pub volumes: Option<::ValueList<self::task_definition::Volume>>,
}

impl ::serde::Serialize for TaskDefinitionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerDefinitions", &self.container_definitions)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cpu", &self.cpu)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExecutionRoleArn", &self.execution_role_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Family", &self.family)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Memory", &self.memory)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkMode", &self.network_mode)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PlacementConstraints", &self.placement_constraints)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequiresCompatibilities", &self.requires_compatibilities)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TaskRoleArn", &self.task_role_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Volumes", &self.volumes)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for TaskDefinitionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<TaskDefinitionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TaskDefinitionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type TaskDefinitionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut container_definitions = None;
                let mut cpu = None;
                let mut execution_role_arn = None;
                let mut family = None;
                let mut memory = None;
                let mut network_mode = None;
                let mut placement_constraints = None;
                let mut requires_compatibilities = None;
                let mut task_role_arn = None;
                let mut volumes = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ContainerDefinitions" => {
                            container_definitions = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Cpu" => {
                            cpu = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ExecutionRoleArn" => {
                            execution_role_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Family" => {
                            family = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Memory" => {
                            memory = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "NetworkMode" => {
                            network_mode = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "PlacementConstraints" => {
                            placement_constraints = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RequiresCompatibilities" => {
                            requires_compatibilities = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "TaskRoleArn" => {
                            task_role_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Volumes" => {
                            volumes = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(TaskDefinitionProperties {
                    container_definitions: container_definitions,
                    cpu: cpu,
                    execution_role_arn: execution_role_arn,
                    family: family,
                    memory: memory,
                    network_mode: network_mode,
                    placement_constraints: placement_constraints,
                    requires_compatibilities: requires_compatibilities,
                    task_role_arn: task_role_arn,
                    volumes: volumes,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
    #[derive(Debug)]
    pub struct AwsVpcConfiguration {
        /// Property `AssignPublicIp`.
        pub assign_public_ip: Option<::Value<String>>,
        /// Property `SecurityGroups`.
        pub security_groups: Option<::ValueList<String>>,
        /// Property `Subnets`.
        pub subnets: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for AwsVpcConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssignPublicIp", &self.assign_public_ip)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroups", &self.security_groups)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subnets", &self.subnets)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AwsVpcConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AwsVpcConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AwsVpcConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AwsVpcConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut assign_public_ip = None;
                    let mut security_groups = None;
                    let mut subnets = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AssignPublicIp" => {
                                assign_public_ip = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SecurityGroups" => {
                                security_groups = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Subnets" => {
                                subnets = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(AwsVpcConfiguration {
                        assign_public_ip: assign_public_ip,
                        security_groups: security_groups,
                        subnets: subnets.ok_or(::serde::de::Error::missing_field("Subnets"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::Service.DeploymentConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-deploymentconfiguration.html) property type.
    #[derive(Debug)]
    pub struct DeploymentConfiguration {
        /// Property `MaximumPercent`.
        pub maximum_percent: Option<::Value<u32>>,
        /// Property `MinimumHealthyPercent`.
        pub minimum_healthy_percent: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for DeploymentConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumPercent", &self.maximum_percent)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinimumHealthyPercent", &self.minimum_healthy_percent)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DeploymentConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DeploymentConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DeploymentConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DeploymentConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut maximum_percent = None;
                    let mut minimum_healthy_percent = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaximumPercent" => {
                                maximum_percent = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "MinimumHealthyPercent" => {
                                minimum_healthy_percent = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(DeploymentConfiguration {
                        maximum_percent: maximum_percent,
                        minimum_healthy_percent: minimum_healthy_percent,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::Service.LoadBalancer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-loadbalancers.html) property type.
    #[derive(Debug)]
    pub struct LoadBalancer {
        /// Property `ContainerName`.
        pub container_name: Option<::Value<String>>,
        /// Property `ContainerPort`.
        pub container_port: ::Value<u32>,
        /// Property `LoadBalancerName`.
        pub load_balancer_name: Option<::Value<String>>,
        /// Property `TargetGroupArn`.
        pub target_group_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LoadBalancer {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerName", &self.container_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerPort", &self.container_port)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoadBalancerName", &self.load_balancer_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetGroupArn", &self.target_group_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LoadBalancer {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LoadBalancer, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LoadBalancer;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LoadBalancer")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut container_name = None;
                    let mut container_port = None;
                    let mut load_balancer_name = None;
                    let mut target_group_arn = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContainerName" => {
                                container_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ContainerPort" => {
                                container_port = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "LoadBalancerName" => {
                                load_balancer_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "TargetGroupArn" => {
                                target_group_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(LoadBalancer {
                        container_name: container_name,
                        container_port: container_port.ok_or(::serde::de::Error::missing_field("ContainerPort"))?,
                        load_balancer_name: load_balancer_name,
                        target_group_arn: target_group_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::Service.NetworkConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-networkconfiguration.html) property type.
    #[derive(Debug)]
    pub struct NetworkConfiguration {
        /// Property `AwsvpcConfiguration`.
        pub awsvpc_configuration: Option<::Value<AwsVpcConfiguration>>,
    }

    impl ::codec::SerializeValue for NetworkConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AwsvpcConfiguration", &self.awsvpc_configuration)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NetworkConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NetworkConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NetworkConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NetworkConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut awsvpc_configuration = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AwsvpcConfiguration" => {
                                awsvpc_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(NetworkConfiguration {
                        awsvpc_configuration: awsvpc_configuration,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::Service.PlacementConstraint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-placementconstraint.html) property type.
    #[derive(Debug)]
    pub struct PlacementConstraint {
        /// Property `Expression`.
        pub expression: Option<::Value<String>>,
        /// Property `Type`.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for PlacementConstraint {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Expression", &self.expression)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PlacementConstraint {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PlacementConstraint, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PlacementConstraint;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PlacementConstraint")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut expression = None;
                    let mut type_ = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Expression" => {
                                expression = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Type" => {
                                type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(PlacementConstraint {
                        expression: expression,
                        type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::Service.PlacementStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-placementstrategy.html) property type.
    #[derive(Debug)]
    pub struct PlacementStrategy {
        /// Property `Field`.
        pub field: Option<::Value<String>>,
        /// Property `Type`.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for PlacementStrategy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Field", &self.field)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PlacementStrategy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PlacementStrategy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PlacementStrategy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PlacementStrategy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut field = None;
                    let mut type_ = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Field" => {
                                field = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Type" => {
                                type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(PlacementStrategy {
                        field: field,
                        type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod task_definition {
    //! Property types for the `TaskDefinition` resource.

    /// The [`AWS::ECS::TaskDefinition.ContainerDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinitions.html) property type.
    #[derive(Debug)]
    pub struct ContainerDefinition {
        /// Property `Command`.
        pub command: Option<::ValueList<String>>,
        /// Property `Cpu`.
        pub cpu: Option<::Value<u32>>,
        /// Property `DisableNetworking`.
        pub disable_networking: Option<::Value<bool>>,
        /// Property `DnsSearchDomains`.
        pub dns_search_domains: Option<::ValueList<String>>,
        /// Property `DnsServers`.
        pub dns_servers: Option<::ValueList<String>>,
        /// Property `DockerLabels`.
        pub docker_labels: Option<::ValueMap<String>>,
        /// Property `DockerSecurityOptions`.
        pub docker_security_options: Option<::ValueList<String>>,
        /// Property `EntryPoint`.
        pub entry_point: Option<::ValueList<String>>,
        /// Property `Environment`.
        pub environment: Option<::ValueList<KeyValuePair>>,
        /// Property `Essential`.
        pub essential: Option<::Value<bool>>,
        /// Property `ExtraHosts`.
        pub extra_hosts: Option<::ValueList<HostEntry>>,
        /// Property `Hostname`.
        pub hostname: Option<::Value<String>>,
        /// Property `Image`.
        pub image: Option<::Value<String>>,
        /// Property `Links`.
        pub links: Option<::ValueList<String>>,
        /// Property `LinuxParameters`.
        pub linux_parameters: Option<::Value<LinuxParameters>>,
        /// Property `LogConfiguration`.
        pub log_configuration: Option<::Value<LogConfiguration>>,
        /// Property `Memory`.
        pub memory: Option<::Value<u32>>,
        /// Property `MemoryReservation`.
        pub memory_reservation: Option<::Value<u32>>,
        /// Property `MountPoints`.
        pub mount_points: Option<::ValueList<MountPoint>>,
        /// Property `Name`.
        pub name: Option<::Value<String>>,
        /// Property `PortMappings`.
        pub port_mappings: Option<::ValueList<PortMapping>>,
        /// Property `Privileged`.
        pub privileged: Option<::Value<bool>>,
        /// Property `ReadonlyRootFilesystem`.
        pub readonly_root_filesystem: Option<::Value<bool>>,
        /// Property `Ulimits`.
        pub ulimits: Option<::ValueList<Ulimit>>,
        /// Property `User`.
        pub user: Option<::Value<String>>,
        /// Property `VolumesFrom`.
        pub volumes_from: Option<::ValueList<VolumeFrom>>,
        /// Property `WorkingDirectory`.
        pub working_directory: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ContainerDefinition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Command", &self.command)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cpu", &self.cpu)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisableNetworking", &self.disable_networking)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DnsSearchDomains", &self.dns_search_domains)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DnsServers", &self.dns_servers)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DockerLabels", &self.docker_labels)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DockerSecurityOptions", &self.docker_security_options)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EntryPoint", &self.entry_point)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Environment", &self.environment)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Essential", &self.essential)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExtraHosts", &self.extra_hosts)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Hostname", &self.hostname)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Image", &self.image)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Links", &self.links)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LinuxParameters", &self.linux_parameters)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogConfiguration", &self.log_configuration)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Memory", &self.memory)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MemoryReservation", &self.memory_reservation)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MountPoints", &self.mount_points)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PortMappings", &self.port_mappings)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Privileged", &self.privileged)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReadonlyRootFilesystem", &self.readonly_root_filesystem)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ulimits", &self.ulimits)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "User", &self.user)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumesFrom", &self.volumes_from)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkingDirectory", &self.working_directory)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ContainerDefinition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ContainerDefinition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ContainerDefinition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ContainerDefinition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut command = None;
                    let mut cpu = None;
                    let mut disable_networking = None;
                    let mut dns_search_domains = None;
                    let mut dns_servers = None;
                    let mut docker_labels = None;
                    let mut docker_security_options = None;
                    let mut entry_point = None;
                    let mut environment = None;
                    let mut essential = None;
                    let mut extra_hosts = None;
                    let mut hostname = None;
                    let mut image = None;
                    let mut links = None;
                    let mut linux_parameters = None;
                    let mut log_configuration = None;
                    let mut memory = None;
                    let mut memory_reservation = None;
                    let mut mount_points = None;
                    let mut name = None;
                    let mut port_mappings = None;
                    let mut privileged = None;
                    let mut readonly_root_filesystem = None;
                    let mut ulimits = None;
                    let mut user = None;
                    let mut volumes_from = None;
                    let mut working_directory = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Command" => {
                                command = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Cpu" => {
                                cpu = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "DisableNetworking" => {
                                disable_networking = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "DnsSearchDomains" => {
                                dns_search_domains = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "DnsServers" => {
                                dns_servers = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "DockerLabels" => {
                                docker_labels = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "DockerSecurityOptions" => {
                                docker_security_options = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "EntryPoint" => {
                                entry_point = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Environment" => {
                                environment = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Essential" => {
                                essential = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ExtraHosts" => {
                                extra_hosts = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Hostname" => {
                                hostname = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Image" => {
                                image = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Links" => {
                                links = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "LinuxParameters" => {
                                linux_parameters = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "LogConfiguration" => {
                                log_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Memory" => {
                                memory = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "MemoryReservation" => {
                                memory_reservation = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "MountPoints" => {
                                mount_points = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Name" => {
                                name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "PortMappings" => {
                                port_mappings = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Privileged" => {
                                privileged = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ReadonlyRootFilesystem" => {
                                readonly_root_filesystem = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Ulimits" => {
                                ulimits = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "User" => {
                                user = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "VolumesFrom" => {
                                volumes_from = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "WorkingDirectory" => {
                                working_directory = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(ContainerDefinition {
                        command: command,
                        cpu: cpu,
                        disable_networking: disable_networking,
                        dns_search_domains: dns_search_domains,
                        dns_servers: dns_servers,
                        docker_labels: docker_labels,
                        docker_security_options: docker_security_options,
                        entry_point: entry_point,
                        environment: environment,
                        essential: essential,
                        extra_hosts: extra_hosts,
                        hostname: hostname,
                        image: image,
                        links: links,
                        linux_parameters: linux_parameters,
                        log_configuration: log_configuration,
                        memory: memory,
                        memory_reservation: memory_reservation,
                        mount_points: mount_points,
                        name: name,
                        port_mappings: port_mappings,
                        privileged: privileged,
                        readonly_root_filesystem: readonly_root_filesystem,
                        ulimits: ulimits,
                        user: user,
                        volumes_from: volumes_from,
                        working_directory: working_directory,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::TaskDefinition.Device`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-device.html) property type.
    #[derive(Debug)]
    pub struct Device {
        /// Property `ContainerPath`.
        pub container_path: Option<::Value<String>>,
        /// Property `HostPath`.
        pub host_path: ::Value<String>,
        /// Property `Permissions`.
        pub permissions: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for Device {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerPath", &self.container_path)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HostPath", &self.host_path)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Permissions", &self.permissions)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Device {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Device, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Device;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Device")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut container_path = None;
                    let mut host_path = None;
                    let mut permissions = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContainerPath" => {
                                container_path = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "HostPath" => {
                                host_path = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Permissions" => {
                                permissions = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(Device {
                        container_path: container_path,
                        host_path: host_path.ok_or(::serde::de::Error::missing_field("HostPath"))?,
                        permissions: permissions,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::TaskDefinition.HostEntry`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinitions-hostentry.html) property type.
    #[derive(Debug)]
    pub struct HostEntry {
        /// Property `Hostname`.
        pub hostname: ::Value<String>,
        /// Property `IpAddress`.
        pub ip_address: ::Value<String>,
    }

    impl ::codec::SerializeValue for HostEntry {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Hostname", &self.hostname)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IpAddress", &self.ip_address)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HostEntry {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HostEntry, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HostEntry;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HostEntry")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut hostname = None;
                    let mut ip_address = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Hostname" => {
                                hostname = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "IpAddress" => {
                                ip_address = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(HostEntry {
                        hostname: hostname.ok_or(::serde::de::Error::missing_field("Hostname"))?,
                        ip_address: ip_address.ok_or(::serde::de::Error::missing_field("IpAddress"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::TaskDefinition.HostVolumeProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-volumes-host.html) property type.
    #[derive(Debug)]
    pub struct HostVolumeProperties {
        /// Property `SourcePath`.
        pub source_path: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for HostVolumeProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourcePath", &self.source_path)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HostVolumeProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HostVolumeProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HostVolumeProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HostVolumeProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut source_path = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SourcePath" => {
                                source_path = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(HostVolumeProperties {
                        source_path: source_path,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::TaskDefinition.KernelCapabilities`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-kernelcapabilities.html) property type.
    #[derive(Debug)]
    pub struct KernelCapabilities {
        /// Property `Add`.
        pub add: Option<::ValueList<String>>,
        /// Property `Drop`.
        pub drop: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for KernelCapabilities {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Add", &self.add)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Drop", &self.drop)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KernelCapabilities {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KernelCapabilities, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KernelCapabilities;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KernelCapabilities")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut add = None;
                    let mut drop = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Add" => {
                                add = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Drop" => {
                                drop = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(KernelCapabilities {
                        add: add,
                        drop: drop,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::TaskDefinition.KeyValuePair`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinitions-environment.html) property type.
    #[derive(Debug)]
    pub struct KeyValuePair {
        /// Property `Name`.
        pub name: Option<::Value<String>>,
        /// Property `Value`.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for KeyValuePair {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KeyValuePair {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KeyValuePair, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KeyValuePair;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KeyValuePair")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name = None;
                    let mut value = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Value" => {
                                value = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(KeyValuePair {
                        name: name,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::TaskDefinition.LinuxParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-linuxparameters.html) property type.
    #[derive(Debug)]
    pub struct LinuxParameters {
        /// Property `Capabilities`.
        pub capabilities: Option<::Value<KernelCapabilities>>,
        /// Property `Devices`.
        pub devices: Option<::ValueList<Device>>,
        /// Property `InitProcessEnabled`.
        pub init_process_enabled: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for LinuxParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Capabilities", &self.capabilities)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Devices", &self.devices)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InitProcessEnabled", &self.init_process_enabled)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LinuxParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LinuxParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LinuxParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LinuxParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut capabilities = None;
                    let mut devices = None;
                    let mut init_process_enabled = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Capabilities" => {
                                capabilities = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Devices" => {
                                devices = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "InitProcessEnabled" => {
                                init_process_enabled = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(LinuxParameters {
                        capabilities: capabilities,
                        devices: devices,
                        init_process_enabled: init_process_enabled,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::TaskDefinition.LogConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinitions-logconfiguration.html) property type.
    #[derive(Debug)]
    pub struct LogConfiguration {
        /// Property `LogDriver`.
        pub log_driver: ::Value<String>,
        /// Property `Options`.
        pub options: Option<::ValueMap<String>>,
    }

    impl ::codec::SerializeValue for LogConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogDriver", &self.log_driver)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Options", &self.options)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LogConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LogConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LogConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LogConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut log_driver = None;
                    let mut options = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LogDriver" => {
                                log_driver = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Options" => {
                                options = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(LogConfiguration {
                        log_driver: log_driver.ok_or(::serde::de::Error::missing_field("LogDriver"))?,
                        options: options,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::TaskDefinition.MountPoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinitions-mountpoints.html) property type.
    #[derive(Debug)]
    pub struct MountPoint {
        /// Property `ContainerPath`.
        pub container_path: Option<::Value<String>>,
        /// Property `ReadOnly`.
        pub read_only: Option<::Value<bool>>,
        /// Property `SourceVolume`.
        pub source_volume: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for MountPoint {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerPath", &self.container_path)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReadOnly", &self.read_only)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceVolume", &self.source_volume)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MountPoint {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MountPoint, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MountPoint;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MountPoint")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut container_path = None;
                    let mut read_only = None;
                    let mut source_volume = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContainerPath" => {
                                container_path = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ReadOnly" => {
                                read_only = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SourceVolume" => {
                                source_volume = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(MountPoint {
                        container_path: container_path,
                        read_only: read_only,
                        source_volume: source_volume,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::TaskDefinition.PortMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinitions-portmappings.html) property type.
    #[derive(Debug)]
    pub struct PortMapping {
        /// Property `ContainerPort`.
        pub container_port: Option<::Value<u32>>,
        /// Property `HostPort`.
        pub host_port: Option<::Value<u32>>,
        /// Property `Protocol`.
        pub protocol: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for PortMapping {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerPort", &self.container_port)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HostPort", &self.host_port)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocol", &self.protocol)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PortMapping {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PortMapping, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PortMapping;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PortMapping")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut container_port = None;
                    let mut host_port = None;
                    let mut protocol = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContainerPort" => {
                                container_port = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "HostPort" => {
                                host_port = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Protocol" => {
                                protocol = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(PortMapping {
                        container_port: container_port,
                        host_port: host_port,
                        protocol: protocol,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::TaskDefinition.TaskDefinitionPlacementConstraint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-taskdefinitionplacementconstraint.html) property type.
    #[derive(Debug)]
    pub struct TaskDefinitionPlacementConstraint {
        /// Property `Expression`.
        pub expression: Option<::Value<String>>,
        /// Property `Type`.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for TaskDefinitionPlacementConstraint {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Expression", &self.expression)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TaskDefinitionPlacementConstraint {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TaskDefinitionPlacementConstraint, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TaskDefinitionPlacementConstraint;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TaskDefinitionPlacementConstraint")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut expression = None;
                    let mut type_ = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Expression" => {
                                expression = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Type" => {
                                type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(TaskDefinitionPlacementConstraint {
                        expression: expression,
                        type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::TaskDefinition.Ulimit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinitions-ulimit.html) property type.
    #[derive(Debug)]
    pub struct Ulimit {
        /// Property `HardLimit`.
        pub hard_limit: ::Value<u32>,
        /// Property `Name`.
        pub name: ::Value<String>,
        /// Property `SoftLimit`.
        pub soft_limit: ::Value<u32>,
    }

    impl ::codec::SerializeValue for Ulimit {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HardLimit", &self.hard_limit)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SoftLimit", &self.soft_limit)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Ulimit {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Ulimit, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Ulimit;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Ulimit")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut hard_limit = None;
                    let mut name = None;
                    let mut soft_limit = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HardLimit" => {
                                hard_limit = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Name" => {
                                name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SoftLimit" => {
                                soft_limit = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(Ulimit {
                        hard_limit: hard_limit.ok_or(::serde::de::Error::missing_field("HardLimit"))?,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        soft_limit: soft_limit.ok_or(::serde::de::Error::missing_field("SoftLimit"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::TaskDefinition.Volume`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-volumes.html) property type.
    #[derive(Debug)]
    pub struct Volume {
        /// Property `Host`.
        pub host: Option<::Value<HostVolumeProperties>>,
        /// Property `Name`.
        pub name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Volume {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Host", &self.host)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Volume {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Volume, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Volume;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Volume")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut host = None;
                    let mut name = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Host" => {
                                host = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Name" => {
                                name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(Volume {
                        host: host,
                        name: name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::TaskDefinition.VolumeFrom`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinitions-volumesfrom.html) property type.
    #[derive(Debug)]
    pub struct VolumeFrom {
        /// Property `ReadOnly`.
        pub read_only: Option<::Value<bool>>,
        /// Property `SourceContainer`.
        pub source_container: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for VolumeFrom {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReadOnly", &self.read_only)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceContainer", &self.source_container)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VolumeFrom {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VolumeFrom, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VolumeFrom;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VolumeFrom")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut read_only = None;
                    let mut source_container = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ReadOnly" => {
                                read_only = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SourceContainer" => {
                                source_container = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(VolumeFrom {
                        read_only: read_only,
                        source_container: source_container,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
