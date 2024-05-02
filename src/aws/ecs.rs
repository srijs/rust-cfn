//! Types for the `ECS` service.

/// The [`AWS::ECS::CapacityProvider`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-capacityprovider.html) resource type.
#[derive(Debug, Default)]
pub struct CapacityProvider {
    properties: CapacityProviderProperties
}

/// Properties for the `CapacityProvider` resource.
#[derive(Debug, Default)]
pub struct CapacityProviderProperties {
    /// Property [`AutoScalingGroupProvider`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-capacityprovider.html#cfn-ecs-capacityprovider-autoscalinggroupprovider).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub auto_scaling_group_provider: ::Value<self::capacity_provider::AutoScalingGroupProvider>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-capacityprovider.html#cfn-ecs-capacityprovider-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-capacityprovider.html#cfn-ecs-capacityprovider-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for CapacityProviderProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoScalingGroupProvider", &self.auto_scaling_group_provider)?;
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CapacityProviderProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CapacityProviderProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CapacityProviderProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CapacityProviderProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut auto_scaling_group_provider: Option<::Value<self::capacity_provider::AutoScalingGroupProvider>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AutoScalingGroupProvider" => {
                            auto_scaling_group_provider = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CapacityProviderProperties {
                    auto_scaling_group_provider: auto_scaling_group_provider.ok_or(::serde::de::Error::missing_field("AutoScalingGroupProvider"))?,
                    name: name,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for CapacityProvider {
    type Properties = CapacityProviderProperties;
    const TYPE: &'static str = "AWS::ECS::CapacityProvider";
    fn properties(&self) -> &CapacityProviderProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CapacityProviderProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for CapacityProvider {}

impl From<CapacityProviderProperties> for CapacityProvider {
    fn from(properties: CapacityProviderProperties) -> CapacityProvider {
        CapacityProvider { properties }
    }
}

/// The [`AWS::ECS::Cluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-cluster.html) resource type.
#[derive(Debug, Default)]
pub struct Cluster {
    properties: ClusterProperties
}

/// Properties for the `Cluster` resource.
#[derive(Debug, Default)]
pub struct ClusterProperties {
    /// Property [`CapacityProviders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-cluster.html#cfn-ecs-cluster-capacityproviders).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub capacity_providers: Option<::ValueList<String>>,
    /// Property [`ClusterName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-cluster.html#cfn-ecs-cluster-clustername).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub cluster_name: Option<::Value<String>>,
    /// Property [`ClusterSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-cluster.html#cfn-ecs-cluster-clustersettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cluster_settings: Option<::ValueList<self::cluster::ClusterSettings>>,
    /// Property [`Configuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-cluster.html#cfn-ecs-cluster-configuration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub configuration: Option<::Value<self::cluster::ClusterConfiguration>>,
    /// Property [`DefaultCapacityProviderStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-cluster.html#cfn-ecs-cluster-defaultcapacityproviderstrategy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub default_capacity_provider_strategy: Option<::ValueList<self::cluster::CapacityProviderStrategyItem>>,
    /// Property [`ServiceConnectDefaults`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-cluster.html#cfn-ecs-cluster-serviceconnectdefaults).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub service_connect_defaults: Option<::Value<self::cluster::ServiceConnectDefaults>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-cluster.html#cfn-ecs-cluster-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ClusterProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref capacity_providers) = self.capacity_providers {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CapacityProviders", capacity_providers)?;
        }
        if let Some(ref cluster_name) = self.cluster_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterName", cluster_name)?;
        }
        if let Some(ref cluster_settings) = self.cluster_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterSettings", cluster_settings)?;
        }
        if let Some(ref configuration) = self.configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Configuration", configuration)?;
        }
        if let Some(ref default_capacity_provider_strategy) = self.default_capacity_provider_strategy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultCapacityProviderStrategy", default_capacity_provider_strategy)?;
        }
        if let Some(ref service_connect_defaults) = self.service_connect_defaults {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceConnectDefaults", service_connect_defaults)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
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
                let mut capacity_providers: Option<::ValueList<String>> = None;
                let mut cluster_name: Option<::Value<String>> = None;
                let mut cluster_settings: Option<::ValueList<self::cluster::ClusterSettings>> = None;
                let mut configuration: Option<::Value<self::cluster::ClusterConfiguration>> = None;
                let mut default_capacity_provider_strategy: Option<::ValueList<self::cluster::CapacityProviderStrategyItem>> = None;
                let mut service_connect_defaults: Option<::Value<self::cluster::ServiceConnectDefaults>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CapacityProviders" => {
                            capacity_providers = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClusterName" => {
                            cluster_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClusterSettings" => {
                            cluster_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Configuration" => {
                            configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DefaultCapacityProviderStrategy" => {
                            default_capacity_provider_strategy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServiceConnectDefaults" => {
                            service_connect_defaults = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ClusterProperties {
                    capacity_providers: capacity_providers,
                    cluster_name: cluster_name,
                    cluster_settings: cluster_settings,
                    configuration: configuration,
                    default_capacity_provider_strategy: default_capacity_provider_strategy,
                    service_connect_defaults: service_connect_defaults,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Cluster {
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

/// The [`AWS::ECS::ClusterCapacityProviderAssociations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-clustercapacityproviderassociations.html) resource type.
#[derive(Debug, Default)]
pub struct ClusterCapacityProviderAssociations {
    properties: ClusterCapacityProviderAssociationsProperties
}

/// Properties for the `ClusterCapacityProviderAssociations` resource.
#[derive(Debug, Default)]
pub struct ClusterCapacityProviderAssociationsProperties {
    /// Property [`CapacityProviders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-clustercapacityproviderassociations.html#cfn-ecs-clustercapacityproviderassociations-capacityproviders).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub capacity_providers: ::ValueList<String>,
    /// Property [`Cluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-clustercapacityproviderassociations.html#cfn-ecs-clustercapacityproviderassociations-cluster).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub cluster: ::Value<String>,
    /// Property [`DefaultCapacityProviderStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-clustercapacityproviderassociations.html#cfn-ecs-clustercapacityproviderassociations-defaultcapacityproviderstrategy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub default_capacity_provider_strategy: ::ValueList<self::cluster_capacity_provider_associations::CapacityProviderStrategy>,
}

impl ::serde::Serialize for ClusterCapacityProviderAssociationsProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CapacityProviders", &self.capacity_providers)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cluster", &self.cluster)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultCapacityProviderStrategy", &self.default_capacity_provider_strategy)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ClusterCapacityProviderAssociationsProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ClusterCapacityProviderAssociationsProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ClusterCapacityProviderAssociationsProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ClusterCapacityProviderAssociationsProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut capacity_providers: Option<::ValueList<String>> = None;
                let mut cluster: Option<::Value<String>> = None;
                let mut default_capacity_provider_strategy: Option<::ValueList<self::cluster_capacity_provider_associations::CapacityProviderStrategy>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CapacityProviders" => {
                            capacity_providers = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Cluster" => {
                            cluster = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DefaultCapacityProviderStrategy" => {
                            default_capacity_provider_strategy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ClusterCapacityProviderAssociationsProperties {
                    capacity_providers: capacity_providers.ok_or(::serde::de::Error::missing_field("CapacityProviders"))?,
                    cluster: cluster.ok_or(::serde::de::Error::missing_field("Cluster"))?,
                    default_capacity_provider_strategy: default_capacity_provider_strategy.ok_or(::serde::de::Error::missing_field("DefaultCapacityProviderStrategy"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ClusterCapacityProviderAssociations {
    type Properties = ClusterCapacityProviderAssociationsProperties;
    const TYPE: &'static str = "AWS::ECS::ClusterCapacityProviderAssociations";
    fn properties(&self) -> &ClusterCapacityProviderAssociationsProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ClusterCapacityProviderAssociationsProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ClusterCapacityProviderAssociations {}

impl From<ClusterCapacityProviderAssociationsProperties> for ClusterCapacityProviderAssociations {
    fn from(properties: ClusterCapacityProviderAssociationsProperties) -> ClusterCapacityProviderAssociations {
        ClusterCapacityProviderAssociations { properties }
    }
}

/// The [`AWS::ECS::PrimaryTaskSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-primarytaskset.html) resource type.
#[derive(Debug, Default)]
pub struct PrimaryTaskSet {
    properties: PrimaryTaskSetProperties
}

/// Properties for the `PrimaryTaskSet` resource.
#[derive(Debug, Default)]
pub struct PrimaryTaskSetProperties {
    /// Property [`Cluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-primarytaskset.html#cfn-ecs-primarytaskset-cluster).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub cluster: ::Value<String>,
    /// Property [`Service`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-primarytaskset.html#cfn-ecs-primarytaskset-service).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub service: ::Value<String>,
    /// Property [`TaskSetId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-primarytaskset.html#cfn-ecs-primarytaskset-tasksetid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub task_set_id: ::Value<String>,
}

impl ::serde::Serialize for PrimaryTaskSetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cluster", &self.cluster)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Service", &self.service)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TaskSetId", &self.task_set_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PrimaryTaskSetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PrimaryTaskSetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PrimaryTaskSetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PrimaryTaskSetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut cluster: Option<::Value<String>> = None;
                let mut service: Option<::Value<String>> = None;
                let mut task_set_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Cluster" => {
                            cluster = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Service" => {
                            service = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TaskSetId" => {
                            task_set_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PrimaryTaskSetProperties {
                    cluster: cluster.ok_or(::serde::de::Error::missing_field("Cluster"))?,
                    service: service.ok_or(::serde::de::Error::missing_field("Service"))?,
                    task_set_id: task_set_id.ok_or(::serde::de::Error::missing_field("TaskSetId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for PrimaryTaskSet {
    type Properties = PrimaryTaskSetProperties;
    const TYPE: &'static str = "AWS::ECS::PrimaryTaskSet";
    fn properties(&self) -> &PrimaryTaskSetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PrimaryTaskSetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for PrimaryTaskSet {}

impl From<PrimaryTaskSetProperties> for PrimaryTaskSet {
    fn from(properties: PrimaryTaskSetProperties) -> PrimaryTaskSet {
        PrimaryTaskSet { properties }
    }
}

/// The [`AWS::ECS::Service`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-service.html) resource type.
#[derive(Debug, Default)]
pub struct Service {
    properties: ServiceProperties
}

/// Properties for the `Service` resource.
#[derive(Debug, Default)]
pub struct ServiceProperties {
    /// Property [`CapacityProviderStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-service.html#cfn-ecs-service-capacityproviderstrategy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub capacity_provider_strategy: Option<::ValueList<self::service::CapacityProviderStrategyItem>>,
    /// Property [`Cluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-service.html#cfn-ecs-service-cluster).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub cluster: Option<::Value<String>>,
    /// Property [`DeploymentConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-service.html#cfn-ecs-service-deploymentconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub deployment_configuration: Option<::Value<self::service::DeploymentConfiguration>>,
    /// Property [`DeploymentController`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-service.html#cfn-ecs-service-deploymentcontroller).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub deployment_controller: Option<::Value<self::service::DeploymentController>>,
    /// Property [`DesiredCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-service.html#cfn-ecs-service-desiredcount).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub desired_count: Option<::Value<u32>>,
    /// Property [`EnableECSManagedTags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-service.html#cfn-ecs-service-enableecsmanagedtags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enable_ecs_managed_tags: Option<::Value<bool>>,
    /// Property [`EnableExecuteCommand`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-service.html#cfn-ecs-service-enableexecutecommand).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enable_execute_command: Option<::Value<bool>>,
    /// Property [`HealthCheckGracePeriodSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-service.html#cfn-ecs-service-healthcheckgraceperiodseconds).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub health_check_grace_period_seconds: Option<::Value<u32>>,
    /// Property [`LaunchType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-service.html#cfn-ecs-service-launchtype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub launch_type: Option<::Value<String>>,
    /// Property [`LoadBalancers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-service.html#cfn-ecs-service-loadbalancers).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub load_balancers: Option<::ValueList<self::service::LoadBalancer>>,
    /// Property [`NetworkConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-service.html#cfn-ecs-service-networkconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub network_configuration: Option<::Value<self::service::NetworkConfiguration>>,
    /// Property [`PlacementConstraints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-service.html#cfn-ecs-service-placementconstraints).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub placement_constraints: Option<::ValueList<self::service::PlacementConstraint>>,
    /// Property [`PlacementStrategies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-service.html#cfn-ecs-service-placementstrategies).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub placement_strategies: Option<::ValueList<self::service::PlacementStrategy>>,
    /// Property [`PlatformVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-service.html#cfn-ecs-service-platformversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub platform_version: Option<::Value<String>>,
    /// Property [`PropagateTags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-service.html#cfn-ecs-service-propagatetags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub propagate_tags: Option<::Value<String>>,
    /// Property [`Role`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-service.html#cfn-ecs-service-role).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub role: Option<::Value<String>>,
    /// Property [`SchedulingStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-service.html#cfn-ecs-service-schedulingstrategy).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub scheduling_strategy: Option<::Value<String>>,
    /// Property [`ServiceConnectConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-service.html#cfn-ecs-service-serviceconnectconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub service_connect_configuration: Option<::Value<self::service::ServiceConnectConfiguration>>,
    /// Property [`ServiceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-service.html#cfn-ecs-service-servicename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub service_name: Option<::Value<String>>,
    /// Property [`ServiceRegistries`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-service.html#cfn-ecs-service-serviceregistries).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub service_registries: Option<::ValueList<self::service::ServiceRegistry>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-service.html#cfn-ecs-service-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TaskDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-service.html#cfn-ecs-service-taskdefinition).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub task_definition: Option<::Value<String>>,
    /// Property [`VolumeConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-service.html#cfn-ecs-service-volumeconfigurations).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub volume_configurations: Option<::ValueList<self::service::ServiceVolumeConfiguration>>,
}

impl ::serde::Serialize for ServiceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref capacity_provider_strategy) = self.capacity_provider_strategy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CapacityProviderStrategy", capacity_provider_strategy)?;
        }
        if let Some(ref cluster) = self.cluster {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cluster", cluster)?;
        }
        if let Some(ref deployment_configuration) = self.deployment_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeploymentConfiguration", deployment_configuration)?;
        }
        if let Some(ref deployment_controller) = self.deployment_controller {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeploymentController", deployment_controller)?;
        }
        if let Some(ref desired_count) = self.desired_count {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DesiredCount", desired_count)?;
        }
        if let Some(ref enable_ecs_managed_tags) = self.enable_ecs_managed_tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableECSManagedTags", enable_ecs_managed_tags)?;
        }
        if let Some(ref enable_execute_command) = self.enable_execute_command {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableExecuteCommand", enable_execute_command)?;
        }
        if let Some(ref health_check_grace_period_seconds) = self.health_check_grace_period_seconds {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthCheckGracePeriodSeconds", health_check_grace_period_seconds)?;
        }
        if let Some(ref launch_type) = self.launch_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LaunchType", launch_type)?;
        }
        if let Some(ref load_balancers) = self.load_balancers {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoadBalancers", load_balancers)?;
        }
        if let Some(ref network_configuration) = self.network_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkConfiguration", network_configuration)?;
        }
        if let Some(ref placement_constraints) = self.placement_constraints {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PlacementConstraints", placement_constraints)?;
        }
        if let Some(ref placement_strategies) = self.placement_strategies {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PlacementStrategies", placement_strategies)?;
        }
        if let Some(ref platform_version) = self.platform_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PlatformVersion", platform_version)?;
        }
        if let Some(ref propagate_tags) = self.propagate_tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PropagateTags", propagate_tags)?;
        }
        if let Some(ref role) = self.role {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Role", role)?;
        }
        if let Some(ref scheduling_strategy) = self.scheduling_strategy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SchedulingStrategy", scheduling_strategy)?;
        }
        if let Some(ref service_connect_configuration) = self.service_connect_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceConnectConfiguration", service_connect_configuration)?;
        }
        if let Some(ref service_name) = self.service_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceName", service_name)?;
        }
        if let Some(ref service_registries) = self.service_registries {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceRegistries", service_registries)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref task_definition) = self.task_definition {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TaskDefinition", task_definition)?;
        }
        if let Some(ref volume_configurations) = self.volume_configurations {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeConfigurations", volume_configurations)?;
        }
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
                let mut capacity_provider_strategy: Option<::ValueList<self::service::CapacityProviderStrategyItem>> = None;
                let mut cluster: Option<::Value<String>> = None;
                let mut deployment_configuration: Option<::Value<self::service::DeploymentConfiguration>> = None;
                let mut deployment_controller: Option<::Value<self::service::DeploymentController>> = None;
                let mut desired_count: Option<::Value<u32>> = None;
                let mut enable_ecs_managed_tags: Option<::Value<bool>> = None;
                let mut enable_execute_command: Option<::Value<bool>> = None;
                let mut health_check_grace_period_seconds: Option<::Value<u32>> = None;
                let mut launch_type: Option<::Value<String>> = None;
                let mut load_balancers: Option<::ValueList<self::service::LoadBalancer>> = None;
                let mut network_configuration: Option<::Value<self::service::NetworkConfiguration>> = None;
                let mut placement_constraints: Option<::ValueList<self::service::PlacementConstraint>> = None;
                let mut placement_strategies: Option<::ValueList<self::service::PlacementStrategy>> = None;
                let mut platform_version: Option<::Value<String>> = None;
                let mut propagate_tags: Option<::Value<String>> = None;
                let mut role: Option<::Value<String>> = None;
                let mut scheduling_strategy: Option<::Value<String>> = None;
                let mut service_connect_configuration: Option<::Value<self::service::ServiceConnectConfiguration>> = None;
                let mut service_name: Option<::Value<String>> = None;
                let mut service_registries: Option<::ValueList<self::service::ServiceRegistry>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut task_definition: Option<::Value<String>> = None;
                let mut volume_configurations: Option<::ValueList<self::service::ServiceVolumeConfiguration>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CapacityProviderStrategy" => {
                            capacity_provider_strategy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Cluster" => {
                            cluster = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DeploymentConfiguration" => {
                            deployment_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DeploymentController" => {
                            deployment_controller = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DesiredCount" => {
                            desired_count = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnableECSManagedTags" => {
                            enable_ecs_managed_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnableExecuteCommand" => {
                            enable_execute_command = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HealthCheckGracePeriodSeconds" => {
                            health_check_grace_period_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LaunchType" => {
                            launch_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LoadBalancers" => {
                            load_balancers = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NetworkConfiguration" => {
                            network_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PlacementConstraints" => {
                            placement_constraints = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PlacementStrategies" => {
                            placement_strategies = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PlatformVersion" => {
                            platform_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PropagateTags" => {
                            propagate_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Role" => {
                            role = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SchedulingStrategy" => {
                            scheduling_strategy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServiceConnectConfiguration" => {
                            service_connect_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServiceName" => {
                            service_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServiceRegistries" => {
                            service_registries = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TaskDefinition" => {
                            task_definition = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VolumeConfigurations" => {
                            volume_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ServiceProperties {
                    capacity_provider_strategy: capacity_provider_strategy,
                    cluster: cluster,
                    deployment_configuration: deployment_configuration,
                    deployment_controller: deployment_controller,
                    desired_count: desired_count,
                    enable_ecs_managed_tags: enable_ecs_managed_tags,
                    enable_execute_command: enable_execute_command,
                    health_check_grace_period_seconds: health_check_grace_period_seconds,
                    launch_type: launch_type,
                    load_balancers: load_balancers,
                    network_configuration: network_configuration,
                    placement_constraints: placement_constraints,
                    placement_strategies: placement_strategies,
                    platform_version: platform_version,
                    propagate_tags: propagate_tags,
                    role: role,
                    scheduling_strategy: scheduling_strategy,
                    service_connect_configuration: service_connect_configuration,
                    service_name: service_name,
                    service_registries: service_registries,
                    tags: tags,
                    task_definition: task_definition,
                    volume_configurations: volume_configurations,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Service {
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
#[derive(Debug, Default)]
pub struct TaskDefinition {
    properties: TaskDefinitionProperties
}

/// Properties for the `TaskDefinition` resource.
#[derive(Debug, Default)]
pub struct TaskDefinitionProperties {
    /// Property [`ContainerDefinitions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-taskdefinition.html#cfn-ecs-taskdefinition-containerdefinitions).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub container_definitions: Option<::ValueList<self::task_definition::ContainerDefinition>>,
    /// Property [`Cpu`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-taskdefinition.html#cfn-ecs-taskdefinition-cpu).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub cpu: Option<::Value<String>>,
    /// Property [`EphemeralStorage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-taskdefinition.html#cfn-ecs-taskdefinition-ephemeralstorage).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub ephemeral_storage: Option<::Value<self::task_definition::EphemeralStorage>>,
    /// Property [`ExecutionRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-taskdefinition.html#cfn-ecs-taskdefinition-executionrolearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub execution_role_arn: Option<::Value<String>>,
    /// Property [`Family`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-taskdefinition.html#cfn-ecs-taskdefinition-family).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub family: Option<::Value<String>>,
    /// Property [`InferenceAccelerators`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-taskdefinition.html#cfn-ecs-taskdefinition-inferenceaccelerators).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub inference_accelerators: Option<::ValueList<self::task_definition::InferenceAccelerator>>,
    /// Property [`IpcMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-taskdefinition.html#cfn-ecs-taskdefinition-ipcmode).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub ipc_mode: Option<::Value<String>>,
    /// Property [`Memory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-taskdefinition.html#cfn-ecs-taskdefinition-memory).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub memory: Option<::Value<String>>,
    /// Property [`NetworkMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-taskdefinition.html#cfn-ecs-taskdefinition-networkmode).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub network_mode: Option<::Value<String>>,
    /// Property [`PidMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-taskdefinition.html#cfn-ecs-taskdefinition-pidmode).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub pid_mode: Option<::Value<String>>,
    /// Property [`PlacementConstraints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-taskdefinition.html#cfn-ecs-taskdefinition-placementconstraints).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub placement_constraints: Option<::ValueList<self::task_definition::TaskDefinitionPlacementConstraint>>,
    /// Property [`ProxyConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-taskdefinition.html#cfn-ecs-taskdefinition-proxyconfiguration).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub proxy_configuration: Option<::Value<self::task_definition::ProxyConfiguration>>,
    /// Property [`RequiresCompatibilities`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-taskdefinition.html#cfn-ecs-taskdefinition-requirescompatibilities).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub requires_compatibilities: Option<::ValueList<String>>,
    /// Property [`RuntimePlatform`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-taskdefinition.html#cfn-ecs-taskdefinition-runtimeplatform).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub runtime_platform: Option<::Value<self::task_definition::RuntimePlatform>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-taskdefinition.html#cfn-ecs-taskdefinition-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TaskRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-taskdefinition.html#cfn-ecs-taskdefinition-taskrolearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub task_role_arn: Option<::Value<String>>,
    /// Property [`Volumes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-taskdefinition.html#cfn-ecs-taskdefinition-volumes).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub volumes: Option<::ValueList<self::task_definition::Volume>>,
}

impl ::serde::Serialize for TaskDefinitionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref container_definitions) = self.container_definitions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerDefinitions", container_definitions)?;
        }
        if let Some(ref cpu) = self.cpu {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cpu", cpu)?;
        }
        if let Some(ref ephemeral_storage) = self.ephemeral_storage {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EphemeralStorage", ephemeral_storage)?;
        }
        if let Some(ref execution_role_arn) = self.execution_role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExecutionRoleArn", execution_role_arn)?;
        }
        if let Some(ref family) = self.family {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Family", family)?;
        }
        if let Some(ref inference_accelerators) = self.inference_accelerators {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InferenceAccelerators", inference_accelerators)?;
        }
        if let Some(ref ipc_mode) = self.ipc_mode {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IpcMode", ipc_mode)?;
        }
        if let Some(ref memory) = self.memory {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Memory", memory)?;
        }
        if let Some(ref network_mode) = self.network_mode {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkMode", network_mode)?;
        }
        if let Some(ref pid_mode) = self.pid_mode {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PidMode", pid_mode)?;
        }
        if let Some(ref placement_constraints) = self.placement_constraints {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PlacementConstraints", placement_constraints)?;
        }
        if let Some(ref proxy_configuration) = self.proxy_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProxyConfiguration", proxy_configuration)?;
        }
        if let Some(ref requires_compatibilities) = self.requires_compatibilities {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequiresCompatibilities", requires_compatibilities)?;
        }
        if let Some(ref runtime_platform) = self.runtime_platform {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuntimePlatform", runtime_platform)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref task_role_arn) = self.task_role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TaskRoleArn", task_role_arn)?;
        }
        if let Some(ref volumes) = self.volumes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Volumes", volumes)?;
        }
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
                let mut container_definitions: Option<::ValueList<self::task_definition::ContainerDefinition>> = None;
                let mut cpu: Option<::Value<String>> = None;
                let mut ephemeral_storage: Option<::Value<self::task_definition::EphemeralStorage>> = None;
                let mut execution_role_arn: Option<::Value<String>> = None;
                let mut family: Option<::Value<String>> = None;
                let mut inference_accelerators: Option<::ValueList<self::task_definition::InferenceAccelerator>> = None;
                let mut ipc_mode: Option<::Value<String>> = None;
                let mut memory: Option<::Value<String>> = None;
                let mut network_mode: Option<::Value<String>> = None;
                let mut pid_mode: Option<::Value<String>> = None;
                let mut placement_constraints: Option<::ValueList<self::task_definition::TaskDefinitionPlacementConstraint>> = None;
                let mut proxy_configuration: Option<::Value<self::task_definition::ProxyConfiguration>> = None;
                let mut requires_compatibilities: Option<::ValueList<String>> = None;
                let mut runtime_platform: Option<::Value<self::task_definition::RuntimePlatform>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut task_role_arn: Option<::Value<String>> = None;
                let mut volumes: Option<::ValueList<self::task_definition::Volume>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ContainerDefinitions" => {
                            container_definitions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Cpu" => {
                            cpu = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EphemeralStorage" => {
                            ephemeral_storage = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ExecutionRoleArn" => {
                            execution_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Family" => {
                            family = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InferenceAccelerators" => {
                            inference_accelerators = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IpcMode" => {
                            ipc_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Memory" => {
                            memory = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NetworkMode" => {
                            network_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PidMode" => {
                            pid_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PlacementConstraints" => {
                            placement_constraints = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProxyConfiguration" => {
                            proxy_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RequiresCompatibilities" => {
                            requires_compatibilities = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RuntimePlatform" => {
                            runtime_platform = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TaskRoleArn" => {
                            task_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Volumes" => {
                            volumes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(TaskDefinitionProperties {
                    container_definitions: container_definitions,
                    cpu: cpu,
                    ephemeral_storage: ephemeral_storage,
                    execution_role_arn: execution_role_arn,
                    family: family,
                    inference_accelerators: inference_accelerators,
                    ipc_mode: ipc_mode,
                    memory: memory,
                    network_mode: network_mode,
                    pid_mode: pid_mode,
                    placement_constraints: placement_constraints,
                    proxy_configuration: proxy_configuration,
                    requires_compatibilities: requires_compatibilities,
                    runtime_platform: runtime_platform,
                    tags: tags,
                    task_role_arn: task_role_arn,
                    volumes: volumes,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for TaskDefinition {
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

/// The [`AWS::ECS::TaskSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-taskset.html) resource type.
#[derive(Debug, Default)]
pub struct TaskSet {
    properties: TaskSetProperties
}

/// Properties for the `TaskSet` resource.
#[derive(Debug, Default)]
pub struct TaskSetProperties {
    /// Property [`Cluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-taskset.html#cfn-ecs-taskset-cluster).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub cluster: ::Value<String>,
    /// Property [`ExternalId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-taskset.html#cfn-ecs-taskset-externalid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub external_id: Option<::Value<String>>,
    /// Property [`LaunchType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-taskset.html#cfn-ecs-taskset-launchtype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub launch_type: Option<::Value<String>>,
    /// Property [`LoadBalancers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-taskset.html#cfn-ecs-taskset-loadbalancers).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub load_balancers: Option<::ValueList<self::task_set::LoadBalancer>>,
    /// Property [`NetworkConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-taskset.html#cfn-ecs-taskset-networkconfiguration).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub network_configuration: Option<::Value<self::task_set::NetworkConfiguration>>,
    /// Property [`PlatformVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-taskset.html#cfn-ecs-taskset-platformversion).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub platform_version: Option<::Value<String>>,
    /// Property [`Scale`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-taskset.html#cfn-ecs-taskset-scale).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub scale: Option<::Value<self::task_set::Scale>>,
    /// Property [`Service`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-taskset.html#cfn-ecs-taskset-service).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub service: ::Value<String>,
    /// Property [`ServiceRegistries`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-taskset.html#cfn-ecs-taskset-serviceregistries).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub service_registries: Option<::ValueList<self::task_set::ServiceRegistry>>,
    /// Property [`TaskDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-taskset.html#cfn-ecs-taskset-taskdefinition).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub task_definition: ::Value<String>,
}

impl ::serde::Serialize for TaskSetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cluster", &self.cluster)?;
        if let Some(ref external_id) = self.external_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExternalId", external_id)?;
        }
        if let Some(ref launch_type) = self.launch_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LaunchType", launch_type)?;
        }
        if let Some(ref load_balancers) = self.load_balancers {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoadBalancers", load_balancers)?;
        }
        if let Some(ref network_configuration) = self.network_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkConfiguration", network_configuration)?;
        }
        if let Some(ref platform_version) = self.platform_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PlatformVersion", platform_version)?;
        }
        if let Some(ref scale) = self.scale {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scale", scale)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Service", &self.service)?;
        if let Some(ref service_registries) = self.service_registries {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceRegistries", service_registries)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TaskDefinition", &self.task_definition)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for TaskSetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<TaskSetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TaskSetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type TaskSetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut cluster: Option<::Value<String>> = None;
                let mut external_id: Option<::Value<String>> = None;
                let mut launch_type: Option<::Value<String>> = None;
                let mut load_balancers: Option<::ValueList<self::task_set::LoadBalancer>> = None;
                let mut network_configuration: Option<::Value<self::task_set::NetworkConfiguration>> = None;
                let mut platform_version: Option<::Value<String>> = None;
                let mut scale: Option<::Value<self::task_set::Scale>> = None;
                let mut service: Option<::Value<String>> = None;
                let mut service_registries: Option<::ValueList<self::task_set::ServiceRegistry>> = None;
                let mut task_definition: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Cluster" => {
                            cluster = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ExternalId" => {
                            external_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LaunchType" => {
                            launch_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LoadBalancers" => {
                            load_balancers = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NetworkConfiguration" => {
                            network_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PlatformVersion" => {
                            platform_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Scale" => {
                            scale = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Service" => {
                            service = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServiceRegistries" => {
                            service_registries = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TaskDefinition" => {
                            task_definition = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(TaskSetProperties {
                    cluster: cluster.ok_or(::serde::de::Error::missing_field("Cluster"))?,
                    external_id: external_id,
                    launch_type: launch_type,
                    load_balancers: load_balancers,
                    network_configuration: network_configuration,
                    platform_version: platform_version,
                    scale: scale,
                    service: service.ok_or(::serde::de::Error::missing_field("Service"))?,
                    service_registries: service_registries,
                    task_definition: task_definition.ok_or(::serde::de::Error::missing_field("TaskDefinition"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for TaskSet {
    type Properties = TaskSetProperties;
    const TYPE: &'static str = "AWS::ECS::TaskSet";
    fn properties(&self) -> &TaskSetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TaskSetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for TaskSet {}

impl From<TaskSetProperties> for TaskSet {
    fn from(properties: TaskSetProperties) -> TaskSet {
        TaskSet { properties }
    }
}

pub mod capacity_provider {
    //! Property types for the `CapacityProvider` resource.

    /// The [`AWS::ECS::CapacityProvider.AutoScalingGroupProvider`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-capacityprovider-autoscalinggroupprovider.html) property type.
    #[derive(Debug, Default)]
    pub struct AutoScalingGroupProvider {
        /// Property [`AutoScalingGroupArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-capacityprovider-autoscalinggroupprovider.html#cfn-ecs-capacityprovider-autoscalinggroupprovider-autoscalinggrouparn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub auto_scaling_group_arn: ::Value<String>,
        /// Property [`ManagedDraining`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-capacityprovider-autoscalinggroupprovider.html#cfn-ecs-capacityprovider-autoscalinggroupprovider-manageddraining).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub managed_draining: Option<::Value<String>>,
        /// Property [`ManagedScaling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-capacityprovider-autoscalinggroupprovider.html#cfn-ecs-capacityprovider-autoscalinggroupprovider-managedscaling).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub managed_scaling: Option<::Value<ManagedScaling>>,
        /// Property [`ManagedTerminationProtection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-capacityprovider-autoscalinggroupprovider.html#cfn-ecs-capacityprovider-autoscalinggroupprovider-managedterminationprotection).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub managed_termination_protection: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AutoScalingGroupProvider {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoScalingGroupArn", &self.auto_scaling_group_arn)?;
            if let Some(ref managed_draining) = self.managed_draining {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManagedDraining", managed_draining)?;
            }
            if let Some(ref managed_scaling) = self.managed_scaling {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManagedScaling", managed_scaling)?;
            }
            if let Some(ref managed_termination_protection) = self.managed_termination_protection {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManagedTerminationProtection", managed_termination_protection)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AutoScalingGroupProvider {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AutoScalingGroupProvider, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AutoScalingGroupProvider;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AutoScalingGroupProvider")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut auto_scaling_group_arn: Option<::Value<String>> = None;
                    let mut managed_draining: Option<::Value<String>> = None;
                    let mut managed_scaling: Option<::Value<ManagedScaling>> = None;
                    let mut managed_termination_protection: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AutoScalingGroupArn" => {
                                auto_scaling_group_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ManagedDraining" => {
                                managed_draining = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ManagedScaling" => {
                                managed_scaling = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ManagedTerminationProtection" => {
                                managed_termination_protection = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AutoScalingGroupProvider {
                        auto_scaling_group_arn: auto_scaling_group_arn.ok_or(::serde::de::Error::missing_field("AutoScalingGroupArn"))?,
                        managed_draining: managed_draining,
                        managed_scaling: managed_scaling,
                        managed_termination_protection: managed_termination_protection,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::CapacityProvider.ManagedScaling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-capacityprovider-managedscaling.html) property type.
    #[derive(Debug, Default)]
    pub struct ManagedScaling {
        /// Property [`InstanceWarmupPeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-capacityprovider-managedscaling.html#cfn-ecs-capacityprovider-managedscaling-instancewarmupperiod).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub instance_warmup_period: Option<::Value<u32>>,
        /// Property [`MaximumScalingStepSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-capacityprovider-managedscaling.html#cfn-ecs-capacityprovider-managedscaling-maximumscalingstepsize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub maximum_scaling_step_size: Option<::Value<u32>>,
        /// Property [`MinimumScalingStepSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-capacityprovider-managedscaling.html#cfn-ecs-capacityprovider-managedscaling-minimumscalingstepsize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub minimum_scaling_step_size: Option<::Value<u32>>,
        /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-capacityprovider-managedscaling.html#cfn-ecs-capacityprovider-managedscaling-status).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub status: Option<::Value<String>>,
        /// Property [`TargetCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-capacityprovider-managedscaling.html#cfn-ecs-capacityprovider-managedscaling-targetcapacity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_capacity: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for ManagedScaling {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref instance_warmup_period) = self.instance_warmup_period {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceWarmupPeriod", instance_warmup_period)?;
            }
            if let Some(ref maximum_scaling_step_size) = self.maximum_scaling_step_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumScalingStepSize", maximum_scaling_step_size)?;
            }
            if let Some(ref minimum_scaling_step_size) = self.minimum_scaling_step_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinimumScalingStepSize", minimum_scaling_step_size)?;
            }
            if let Some(ref status) = self.status {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", status)?;
            }
            if let Some(ref target_capacity) = self.target_capacity {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetCapacity", target_capacity)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ManagedScaling {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ManagedScaling, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ManagedScaling;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ManagedScaling")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut instance_warmup_period: Option<::Value<u32>> = None;
                    let mut maximum_scaling_step_size: Option<::Value<u32>> = None;
                    let mut minimum_scaling_step_size: Option<::Value<u32>> = None;
                    let mut status: Option<::Value<String>> = None;
                    let mut target_capacity: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InstanceWarmupPeriod" => {
                                instance_warmup_period = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaximumScalingStepSize" => {
                                maximum_scaling_step_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinimumScalingStepSize" => {
                                minimum_scaling_step_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Status" => {
                                status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetCapacity" => {
                                target_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ManagedScaling {
                        instance_warmup_period: instance_warmup_period,
                        maximum_scaling_step_size: maximum_scaling_step_size,
                        minimum_scaling_step_size: minimum_scaling_step_size,
                        status: status,
                        target_capacity: target_capacity,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod cluster {
    //! Property types for the `Cluster` resource.

    /// The [`AWS::ECS::Cluster.CapacityProviderStrategyItem`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-cluster-capacityproviderstrategyitem.html) property type.
    #[derive(Debug, Default)]
    pub struct CapacityProviderStrategyItem {
        /// Property [`Base`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-cluster-capacityproviderstrategyitem.html#cfn-ecs-cluster-capacityproviderstrategyitem-base).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub base: Option<::Value<u32>>,
        /// Property [`CapacityProvider`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-cluster-capacityproviderstrategyitem.html#cfn-ecs-cluster-capacityproviderstrategyitem-capacityprovider).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub capacity_provider: Option<::Value<String>>,
        /// Property [`Weight`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-cluster-capacityproviderstrategyitem.html#cfn-ecs-cluster-capacityproviderstrategyitem-weight).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub weight: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for CapacityProviderStrategyItem {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref base) = self.base {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Base", base)?;
            }
            if let Some(ref capacity_provider) = self.capacity_provider {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CapacityProvider", capacity_provider)?;
            }
            if let Some(ref weight) = self.weight {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Weight", weight)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CapacityProviderStrategyItem {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CapacityProviderStrategyItem, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CapacityProviderStrategyItem;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CapacityProviderStrategyItem")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut base: Option<::Value<u32>> = None;
                    let mut capacity_provider: Option<::Value<String>> = None;
                    let mut weight: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Base" => {
                                base = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CapacityProvider" => {
                                capacity_provider = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Weight" => {
                                weight = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CapacityProviderStrategyItem {
                        base: base,
                        capacity_provider: capacity_provider,
                        weight: weight,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::Cluster.ClusterConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-cluster-clusterconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ClusterConfiguration {
        /// Property [`ExecuteCommandConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-cluster-clusterconfiguration.html#cfn-ecs-cluster-clusterconfiguration-executecommandconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub execute_command_configuration: Option<::Value<ExecuteCommandConfiguration>>,
    }

    impl ::codec::SerializeValue for ClusterConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref execute_command_configuration) = self.execute_command_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExecuteCommandConfiguration", execute_command_configuration)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ClusterConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ClusterConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ClusterConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ClusterConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut execute_command_configuration: Option<::Value<ExecuteCommandConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ExecuteCommandConfiguration" => {
                                execute_command_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ClusterConfiguration {
                        execute_command_configuration: execute_command_configuration,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::Cluster.ClusterSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-cluster-clustersettings.html) property type.
    #[derive(Debug, Default)]
    pub struct ClusterSettings {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-cluster-clustersettings.html#cfn-ecs-cluster-clustersettings-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-cluster-clustersettings.html#cfn-ecs-cluster-clustersettings-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ClusterSettings {
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

    impl ::codec::DeserializeValue for ClusterSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ClusterSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ClusterSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ClusterSettings")
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

                    Ok(ClusterSettings {
                        name: name,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::Cluster.ExecuteCommandConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-cluster-executecommandconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ExecuteCommandConfiguration {
        /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-cluster-executecommandconfiguration.html#cfn-ecs-cluster-executecommandconfiguration-kmskeyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kms_key_id: Option<::Value<String>>,
        /// Property [`LogConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-cluster-executecommandconfiguration.html#cfn-ecs-cluster-executecommandconfiguration-logconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_configuration: Option<::Value<ExecuteCommandLogConfiguration>>,
        /// Property [`Logging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-cluster-executecommandconfiguration.html#cfn-ecs-cluster-executecommandconfiguration-logging).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub logging: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ExecuteCommandConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref kms_key_id) = self.kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
            }
            if let Some(ref log_configuration) = self.log_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogConfiguration", log_configuration)?;
            }
            if let Some(ref logging) = self.logging {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Logging", logging)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ExecuteCommandConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ExecuteCommandConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ExecuteCommandConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ExecuteCommandConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut kms_key_id: Option<::Value<String>> = None;
                    let mut log_configuration: Option<::Value<ExecuteCommandLogConfiguration>> = None;
                    let mut logging: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KmsKeyId" => {
                                kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogConfiguration" => {
                                log_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Logging" => {
                                logging = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ExecuteCommandConfiguration {
                        kms_key_id: kms_key_id,
                        log_configuration: log_configuration,
                        logging: logging,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::Cluster.ExecuteCommandLogConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-cluster-executecommandlogconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ExecuteCommandLogConfiguration {
        /// Property [`CloudWatchEncryptionEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-cluster-executecommandlogconfiguration.html#cfn-ecs-cluster-executecommandlogconfiguration-cloudwatchencryptionenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloud_watch_encryption_enabled: Option<::Value<bool>>,
        /// Property [`CloudWatchLogGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-cluster-executecommandlogconfiguration.html#cfn-ecs-cluster-executecommandlogconfiguration-cloudwatchloggroupname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloud_watch_log_group_name: Option<::Value<String>>,
        /// Property [`S3BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-cluster-executecommandlogconfiguration.html#cfn-ecs-cluster-executecommandlogconfiguration-s3bucketname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_bucket_name: Option<::Value<String>>,
        /// Property [`S3EncryptionEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-cluster-executecommandlogconfiguration.html#cfn-ecs-cluster-executecommandlogconfiguration-s3encryptionenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_encryption_enabled: Option<::Value<bool>>,
        /// Property [`S3KeyPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-cluster-executecommandlogconfiguration.html#cfn-ecs-cluster-executecommandlogconfiguration-s3keyprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_key_prefix: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ExecuteCommandLogConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cloud_watch_encryption_enabled) = self.cloud_watch_encryption_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchEncryptionEnabled", cloud_watch_encryption_enabled)?;
            }
            if let Some(ref cloud_watch_log_group_name) = self.cloud_watch_log_group_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLogGroupName", cloud_watch_log_group_name)?;
            }
            if let Some(ref s3_bucket_name) = self.s3_bucket_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3BucketName", s3_bucket_name)?;
            }
            if let Some(ref s3_encryption_enabled) = self.s3_encryption_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3EncryptionEnabled", s3_encryption_enabled)?;
            }
            if let Some(ref s3_key_prefix) = self.s3_key_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3KeyPrefix", s3_key_prefix)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ExecuteCommandLogConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ExecuteCommandLogConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ExecuteCommandLogConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ExecuteCommandLogConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cloud_watch_encryption_enabled: Option<::Value<bool>> = None;
                    let mut cloud_watch_log_group_name: Option<::Value<String>> = None;
                    let mut s3_bucket_name: Option<::Value<String>> = None;
                    let mut s3_encryption_enabled: Option<::Value<bool>> = None;
                    let mut s3_key_prefix: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CloudWatchEncryptionEnabled" => {
                                cloud_watch_encryption_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CloudWatchLogGroupName" => {
                                cloud_watch_log_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3BucketName" => {
                                s3_bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3EncryptionEnabled" => {
                                s3_encryption_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3KeyPrefix" => {
                                s3_key_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ExecuteCommandLogConfiguration {
                        cloud_watch_encryption_enabled: cloud_watch_encryption_enabled,
                        cloud_watch_log_group_name: cloud_watch_log_group_name,
                        s3_bucket_name: s3_bucket_name,
                        s3_encryption_enabled: s3_encryption_enabled,
                        s3_key_prefix: s3_key_prefix,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::Cluster.ServiceConnectDefaults`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-cluster-serviceconnectdefaults.html) property type.
    #[derive(Debug, Default)]
    pub struct ServiceConnectDefaults {
        /// Property [`Namespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-cluster-serviceconnectdefaults.html#cfn-ecs-cluster-serviceconnectdefaults-namespace).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub namespace: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ServiceConnectDefaults {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref namespace) = self.namespace {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Namespace", namespace)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ServiceConnectDefaults {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ServiceConnectDefaults, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ServiceConnectDefaults;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ServiceConnectDefaults")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut namespace: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Namespace" => {
                                namespace = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ServiceConnectDefaults {
                        namespace: namespace,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod cluster_capacity_provider_associations {
    //! Property types for the `ClusterCapacityProviderAssociations` resource.

    /// The [`AWS::ECS::ClusterCapacityProviderAssociations.CapacityProviderStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-clustercapacityproviderassociations-capacityproviderstrategy.html) property type.
    #[derive(Debug, Default)]
    pub struct CapacityProviderStrategy {
        /// Property [`Base`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-clustercapacityproviderassociations-capacityproviderstrategy.html#cfn-ecs-clustercapacityproviderassociations-capacityproviderstrategy-base).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub base: Option<::Value<u32>>,
        /// Property [`CapacityProvider`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-clustercapacityproviderassociations-capacityproviderstrategy.html#cfn-ecs-clustercapacityproviderassociations-capacityproviderstrategy-capacityprovider).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub capacity_provider: ::Value<String>,
        /// Property [`Weight`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-clustercapacityproviderassociations-capacityproviderstrategy.html#cfn-ecs-clustercapacityproviderassociations-capacityproviderstrategy-weight).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub weight: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for CapacityProviderStrategy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref base) = self.base {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Base", base)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CapacityProvider", &self.capacity_provider)?;
            if let Some(ref weight) = self.weight {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Weight", weight)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CapacityProviderStrategy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CapacityProviderStrategy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CapacityProviderStrategy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CapacityProviderStrategy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut base: Option<::Value<u32>> = None;
                    let mut capacity_provider: Option<::Value<String>> = None;
                    let mut weight: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Base" => {
                                base = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CapacityProvider" => {
                                capacity_provider = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Weight" => {
                                weight = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CapacityProviderStrategy {
                        base: base,
                        capacity_provider: capacity_provider.ok_or(::serde::de::Error::missing_field("CapacityProvider"))?,
                        weight: weight,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod service {
    //! Property types for the `Service` resource.

    /// The [`AWS::ECS::Service.AwsVpcConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-awsvpcconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct AwsVpcConfiguration {
        /// Property [`AssignPublicIp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-awsvpcconfiguration.html#cfn-ecs-service-awsvpcconfiguration-assignpublicip).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub assign_public_ip: Option<::Value<String>>,
        /// Property [`SecurityGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-awsvpcconfiguration.html#cfn-ecs-service-awsvpcconfiguration-securitygroups).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_groups: Option<::ValueList<String>>,
        /// Property [`Subnets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-awsvpcconfiguration.html#cfn-ecs-service-awsvpcconfiguration-subnets).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subnets: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for AwsVpcConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref assign_public_ip) = self.assign_public_ip {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssignPublicIp", assign_public_ip)?;
            }
            if let Some(ref security_groups) = self.security_groups {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroups", security_groups)?;
            }
            if let Some(ref subnets) = self.subnets {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subnets", subnets)?;
            }
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
                    let mut assign_public_ip: Option<::Value<String>> = None;
                    let mut security_groups: Option<::ValueList<String>> = None;
                    let mut subnets: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AssignPublicIp" => {
                                assign_public_ip = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecurityGroups" => {
                                security_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Subnets" => {
                                subnets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AwsVpcConfiguration {
                        assign_public_ip: assign_public_ip,
                        security_groups: security_groups,
                        subnets: subnets,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::Service.CapacityProviderStrategyItem`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-capacityproviderstrategyitem.html) property type.
    #[derive(Debug, Default)]
    pub struct CapacityProviderStrategyItem {
        /// Property [`Base`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-capacityproviderstrategyitem.html#cfn-ecs-service-capacityproviderstrategyitem-base).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub base: Option<::Value<u32>>,
        /// Property [`CapacityProvider`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-capacityproviderstrategyitem.html#cfn-ecs-service-capacityproviderstrategyitem-capacityprovider).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub capacity_provider: Option<::Value<String>>,
        /// Property [`Weight`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-capacityproviderstrategyitem.html#cfn-ecs-service-capacityproviderstrategyitem-weight).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub weight: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for CapacityProviderStrategyItem {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref base) = self.base {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Base", base)?;
            }
            if let Some(ref capacity_provider) = self.capacity_provider {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CapacityProvider", capacity_provider)?;
            }
            if let Some(ref weight) = self.weight {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Weight", weight)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CapacityProviderStrategyItem {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CapacityProviderStrategyItem, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CapacityProviderStrategyItem;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CapacityProviderStrategyItem")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut base: Option<::Value<u32>> = None;
                    let mut capacity_provider: Option<::Value<String>> = None;
                    let mut weight: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Base" => {
                                base = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CapacityProvider" => {
                                capacity_provider = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Weight" => {
                                weight = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CapacityProviderStrategyItem {
                        base: base,
                        capacity_provider: capacity_provider,
                        weight: weight,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::Service.DeploymentAlarms`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-deploymentalarms.html) property type.
    #[derive(Debug, Default)]
    pub struct DeploymentAlarms {
        /// Property [`AlarmNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-deploymentalarms.html#cfn-ecs-service-deploymentalarms-alarmnames).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub alarm_names: ::ValueList<String>,
        /// Property [`Enable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-deploymentalarms.html#cfn-ecs-service-deploymentalarms-enable).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable: ::Value<bool>,
        /// Property [`Rollback`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-deploymentalarms.html#cfn-ecs-service-deploymentalarms-rollback).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rollback: ::Value<bool>,
    }

    impl ::codec::SerializeValue for DeploymentAlarms {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AlarmNames", &self.alarm_names)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enable", &self.enable)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Rollback", &self.rollback)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DeploymentAlarms {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DeploymentAlarms, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DeploymentAlarms;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DeploymentAlarms")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut alarm_names: Option<::ValueList<String>> = None;
                    let mut enable: Option<::Value<bool>> = None;
                    let mut rollback: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AlarmNames" => {
                                alarm_names = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Enable" => {
                                enable = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Rollback" => {
                                rollback = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DeploymentAlarms {
                        alarm_names: alarm_names.ok_or(::serde::de::Error::missing_field("AlarmNames"))?,
                        enable: enable.ok_or(::serde::de::Error::missing_field("Enable"))?,
                        rollback: rollback.ok_or(::serde::de::Error::missing_field("Rollback"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::Service.DeploymentCircuitBreaker`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-deploymentcircuitbreaker.html) property type.
    #[derive(Debug, Default)]
    pub struct DeploymentCircuitBreaker {
        /// Property [`Enable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-deploymentcircuitbreaker.html#cfn-ecs-service-deploymentcircuitbreaker-enable).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable: ::Value<bool>,
        /// Property [`Rollback`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-deploymentcircuitbreaker.html#cfn-ecs-service-deploymentcircuitbreaker-rollback).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rollback: ::Value<bool>,
    }

    impl ::codec::SerializeValue for DeploymentCircuitBreaker {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enable", &self.enable)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Rollback", &self.rollback)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DeploymentCircuitBreaker {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DeploymentCircuitBreaker, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DeploymentCircuitBreaker;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DeploymentCircuitBreaker")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enable: Option<::Value<bool>> = None;
                    let mut rollback: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enable" => {
                                enable = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Rollback" => {
                                rollback = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DeploymentCircuitBreaker {
                        enable: enable.ok_or(::serde::de::Error::missing_field("Enable"))?,
                        rollback: rollback.ok_or(::serde::de::Error::missing_field("Rollback"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::Service.DeploymentConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-deploymentconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct DeploymentConfiguration {
        /// Property [`Alarms`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-deploymentconfiguration.html#cfn-ecs-service-deploymentconfiguration-alarms).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub alarms: Option<::Value<DeploymentAlarms>>,
        /// Property [`DeploymentCircuitBreaker`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-deploymentconfiguration.html#cfn-ecs-service-deploymentconfiguration-deploymentcircuitbreaker).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub deployment_circuit_breaker: Option<::Value<DeploymentCircuitBreaker>>,
        /// Property [`MaximumPercent`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-deploymentconfiguration.html#cfn-ecs-service-deploymentconfiguration-maximumpercent).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub maximum_percent: Option<::Value<u32>>,
        /// Property [`MinimumHealthyPercent`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-deploymentconfiguration.html#cfn-ecs-service-deploymentconfiguration-minimumhealthypercent).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub minimum_healthy_percent: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for DeploymentConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref alarms) = self.alarms {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Alarms", alarms)?;
            }
            if let Some(ref deployment_circuit_breaker) = self.deployment_circuit_breaker {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeploymentCircuitBreaker", deployment_circuit_breaker)?;
            }
            if let Some(ref maximum_percent) = self.maximum_percent {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumPercent", maximum_percent)?;
            }
            if let Some(ref minimum_healthy_percent) = self.minimum_healthy_percent {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinimumHealthyPercent", minimum_healthy_percent)?;
            }
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
                    let mut alarms: Option<::Value<DeploymentAlarms>> = None;
                    let mut deployment_circuit_breaker: Option<::Value<DeploymentCircuitBreaker>> = None;
                    let mut maximum_percent: Option<::Value<u32>> = None;
                    let mut minimum_healthy_percent: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Alarms" => {
                                alarms = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeploymentCircuitBreaker" => {
                                deployment_circuit_breaker = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaximumPercent" => {
                                maximum_percent = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinimumHealthyPercent" => {
                                minimum_healthy_percent = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DeploymentConfiguration {
                        alarms: alarms,
                        deployment_circuit_breaker: deployment_circuit_breaker,
                        maximum_percent: maximum_percent,
                        minimum_healthy_percent: minimum_healthy_percent,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::Service.DeploymentController`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-deploymentcontroller.html) property type.
    #[derive(Debug, Default)]
    pub struct DeploymentController {
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-deploymentcontroller.html#cfn-ecs-service-deploymentcontroller-type).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub r#type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DeploymentController {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref r#type) = self.r#type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DeploymentController {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DeploymentController, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DeploymentController;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DeploymentController")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DeploymentController {
                        r#type: r#type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::Service.EBSTagSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-ebstagspecification.html) property type.
    #[derive(Debug, Default)]
    pub struct EBSTagSpecification {
        /// Property [`PropagateTags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-ebstagspecification.html#cfn-ecs-service-ebstagspecification-propagatetags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub propagate_tags: Option<::Value<String>>,
        /// Property [`ResourceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-ebstagspecification.html#cfn-ecs-service-ebstagspecification-resourcetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_type: ::Value<String>,
        /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-ebstagspecification.html#cfn-ecs-service-ebstagspecification-tags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tags: Option<::ValueList<::Tag>>,
    }

    impl ::codec::SerializeValue for EBSTagSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref propagate_tags) = self.propagate_tags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PropagateTags", propagate_tags)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceType", &self.resource_type)?;
            if let Some(ref tags) = self.tags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EBSTagSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EBSTagSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EBSTagSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EBSTagSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut propagate_tags: Option<::Value<String>> = None;
                    let mut resource_type: Option<::Value<String>> = None;
                    let mut tags: Option<::ValueList<::Tag>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PropagateTags" => {
                                propagate_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceType" => {
                                resource_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Tags" => {
                                tags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EBSTagSpecification {
                        propagate_tags: propagate_tags,
                        resource_type: resource_type.ok_or(::serde::de::Error::missing_field("ResourceType"))?,
                        tags: tags,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::Service.LoadBalancer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-loadbalancer.html) property type.
    #[derive(Debug, Default)]
    pub struct LoadBalancer {
        /// Property [`ContainerName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-loadbalancer.html#cfn-ecs-service-loadbalancer-containername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub container_name: Option<::Value<String>>,
        /// Property [`ContainerPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-loadbalancer.html#cfn-ecs-service-loadbalancer-containerport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub container_port: Option<::Value<u32>>,
        /// Property [`LoadBalancerName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-loadbalancer.html#cfn-ecs-service-loadbalancer-loadbalancername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub load_balancer_name: Option<::Value<String>>,
        /// Property [`TargetGroupArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-loadbalancer.html#cfn-ecs-service-loadbalancer-targetgrouparn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_group_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LoadBalancer {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref container_name) = self.container_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerName", container_name)?;
            }
            if let Some(ref container_port) = self.container_port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerPort", container_port)?;
            }
            if let Some(ref load_balancer_name) = self.load_balancer_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoadBalancerName", load_balancer_name)?;
            }
            if let Some(ref target_group_arn) = self.target_group_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetGroupArn", target_group_arn)?;
            }
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
                    let mut container_name: Option<::Value<String>> = None;
                    let mut container_port: Option<::Value<u32>> = None;
                    let mut load_balancer_name: Option<::Value<String>> = None;
                    let mut target_group_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContainerName" => {
                                container_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ContainerPort" => {
                                container_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LoadBalancerName" => {
                                load_balancer_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetGroupArn" => {
                                target_group_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LoadBalancer {
                        container_name: container_name,
                        container_port: container_port,
                        load_balancer_name: load_balancer_name,
                        target_group_arn: target_group_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::Service.LogConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-logconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct LogConfiguration {
        /// Property [`LogDriver`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-logconfiguration.html#cfn-ecs-service-logconfiguration-logdriver).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_driver: Option<::Value<String>>,
        /// Property [`Options`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-logconfiguration.html#cfn-ecs-service-logconfiguration-options).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub options: Option<::ValueMap<String>>,
        /// Property [`SecretOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-logconfiguration.html#cfn-ecs-service-logconfiguration-secretoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secret_options: Option<::ValueList<Secret>>,
    }

    impl ::codec::SerializeValue for LogConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref log_driver) = self.log_driver {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogDriver", log_driver)?;
            }
            if let Some(ref options) = self.options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Options", options)?;
            }
            if let Some(ref secret_options) = self.secret_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretOptions", secret_options)?;
            }
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
                    let mut log_driver: Option<::Value<String>> = None;
                    let mut options: Option<::ValueMap<String>> = None;
                    let mut secret_options: Option<::ValueList<Secret>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LogDriver" => {
                                log_driver = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Options" => {
                                options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretOptions" => {
                                secret_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LogConfiguration {
                        log_driver: log_driver,
                        options: options,
                        secret_options: secret_options,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::Service.NetworkConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-networkconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct NetworkConfiguration {
        /// Property [`AwsvpcConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-networkconfiguration.html#cfn-ecs-service-networkconfiguration-awsvpcconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub awsvpc_configuration: Option<::Value<AwsVpcConfiguration>>,
    }

    impl ::codec::SerializeValue for NetworkConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref awsvpc_configuration) = self.awsvpc_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AwsvpcConfiguration", awsvpc_configuration)?;
            }
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
                    let mut awsvpc_configuration: Option<::Value<AwsVpcConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AwsvpcConfiguration" => {
                                awsvpc_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct PlacementConstraint {
        /// Property [`Expression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-placementconstraint.html#cfn-ecs-service-placementconstraint-expression).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub expression: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-placementconstraint.html#cfn-ecs-service-placementconstraint-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for PlacementConstraint {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref expression) = self.expression {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Expression", expression)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
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
                    let mut expression: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Expression" => {
                                expression = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PlacementConstraint {
                        expression: expression,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::Service.PlacementStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-placementstrategy.html) property type.
    #[derive(Debug, Default)]
    pub struct PlacementStrategy {
        /// Property [`Field`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-placementstrategy.html#cfn-ecs-service-placementstrategy-field).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-placementstrategy.html#cfn-ecs-service-placementstrategy-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for PlacementStrategy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref field) = self.field {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Field", field)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
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
                    let mut field: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Field" => {
                                field = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PlacementStrategy {
                        field: field,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::Service.Secret`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-secret.html) property type.
    #[derive(Debug, Default)]
    pub struct Secret {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-secret.html#cfn-ecs-service-secret-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`ValueFrom`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-secret.html#cfn-ecs-service-secret-valuefrom).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value_from: ::Value<String>,
    }

    impl ::codec::SerializeValue for Secret {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ValueFrom", &self.value_from)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Secret {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Secret, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Secret;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Secret")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut value_from: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ValueFrom" => {
                                value_from = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Secret {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        value_from: value_from.ok_or(::serde::de::Error::missing_field("ValueFrom"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::Service.ServiceConnectClientAlias`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-serviceconnectclientalias.html) property type.
    #[derive(Debug, Default)]
    pub struct ServiceConnectClientAlias {
        /// Property [`DnsName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-serviceconnectclientalias.html#cfn-ecs-service-serviceconnectclientalias-dnsname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dns_name: Option<::Value<String>>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-serviceconnectclientalias.html#cfn-ecs-service-serviceconnectclientalias-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: ::Value<u32>,
    }

    impl ::codec::SerializeValue for ServiceConnectClientAlias {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref dns_name) = self.dns_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DnsName", dns_name)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", &self.port)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ServiceConnectClientAlias {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ServiceConnectClientAlias, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ServiceConnectClientAlias;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ServiceConnectClientAlias")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dns_name: Option<::Value<String>> = None;
                    let mut port: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DnsName" => {
                                dns_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ServiceConnectClientAlias {
                        dns_name: dns_name,
                        port: port.ok_or(::serde::de::Error::missing_field("Port"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::Service.ServiceConnectConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-serviceconnectconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ServiceConnectConfiguration {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-serviceconnectconfiguration.html#cfn-ecs-service-serviceconnectconfiguration-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: ::Value<bool>,
        /// Property [`LogConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-serviceconnectconfiguration.html#cfn-ecs-service-serviceconnectconfiguration-logconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_configuration: Option<::Value<LogConfiguration>>,
        /// Property [`Namespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-serviceconnectconfiguration.html#cfn-ecs-service-serviceconnectconfiguration-namespace).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub namespace: Option<::Value<String>>,
        /// Property [`Services`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-serviceconnectconfiguration.html#cfn-ecs-service-serviceconnectconfiguration-services).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub services: Option<::ValueList<ServiceConnectService>>,
    }

    impl ::codec::SerializeValue for ServiceConnectConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            if let Some(ref log_configuration) = self.log_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogConfiguration", log_configuration)?;
            }
            if let Some(ref namespace) = self.namespace {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Namespace", namespace)?;
            }
            if let Some(ref services) = self.services {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Services", services)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ServiceConnectConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ServiceConnectConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ServiceConnectConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ServiceConnectConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut log_configuration: Option<::Value<LogConfiguration>> = None;
                    let mut namespace: Option<::Value<String>> = None;
                    let mut services: Option<::ValueList<ServiceConnectService>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogConfiguration" => {
                                log_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Namespace" => {
                                namespace = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Services" => {
                                services = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ServiceConnectConfiguration {
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                        log_configuration: log_configuration,
                        namespace: namespace,
                        services: services,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::Service.ServiceConnectService`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-serviceconnectservice.html) property type.
    #[derive(Debug, Default)]
    pub struct ServiceConnectService {
        /// Property [`ClientAliases`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-serviceconnectservice.html#cfn-ecs-service-serviceconnectservice-clientaliases).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_aliases: Option<::ValueList<ServiceConnectClientAlias>>,
        /// Property [`DiscoveryName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-serviceconnectservice.html#cfn-ecs-service-serviceconnectservice-discoveryname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub discovery_name: Option<::Value<String>>,
        /// Property [`IngressPortOverride`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-serviceconnectservice.html#cfn-ecs-service-serviceconnectservice-ingressportoverride).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ingress_port_override: Option<::Value<u32>>,
        /// Property [`PortName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-serviceconnectservice.html#cfn-ecs-service-serviceconnectservice-portname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port_name: ::Value<String>,
        /// Property [`Timeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-serviceconnectservice.html#cfn-ecs-service-serviceconnectservice-timeout).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timeout: Option<::Value<TimeoutConfiguration>>,
        /// Property [`Tls`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-serviceconnectservice.html#cfn-ecs-service-serviceconnectservice-tls).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tls: Option<::Value<ServiceConnectTlsConfiguration>>,
    }

    impl ::codec::SerializeValue for ServiceConnectService {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref client_aliases) = self.client_aliases {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientAliases", client_aliases)?;
            }
            if let Some(ref discovery_name) = self.discovery_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DiscoveryName", discovery_name)?;
            }
            if let Some(ref ingress_port_override) = self.ingress_port_override {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IngressPortOverride", ingress_port_override)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PortName", &self.port_name)?;
            if let Some(ref timeout) = self.timeout {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Timeout", timeout)?;
            }
            if let Some(ref tls) = self.tls {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tls", tls)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ServiceConnectService {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ServiceConnectService, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ServiceConnectService;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ServiceConnectService")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut client_aliases: Option<::ValueList<ServiceConnectClientAlias>> = None;
                    let mut discovery_name: Option<::Value<String>> = None;
                    let mut ingress_port_override: Option<::Value<u32>> = None;
                    let mut port_name: Option<::Value<String>> = None;
                    let mut timeout: Option<::Value<TimeoutConfiguration>> = None;
                    let mut tls: Option<::Value<ServiceConnectTlsConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ClientAliases" => {
                                client_aliases = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DiscoveryName" => {
                                discovery_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IngressPortOverride" => {
                                ingress_port_override = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PortName" => {
                                port_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Timeout" => {
                                timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Tls" => {
                                tls = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ServiceConnectService {
                        client_aliases: client_aliases,
                        discovery_name: discovery_name,
                        ingress_port_override: ingress_port_override,
                        port_name: port_name.ok_or(::serde::de::Error::missing_field("PortName"))?,
                        timeout: timeout,
                        tls: tls,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::Service.ServiceConnectTlsCertificateAuthority`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-serviceconnecttlscertificateauthority.html) property type.
    #[derive(Debug, Default)]
    pub struct ServiceConnectTlsCertificateAuthority {
        /// Property [`AwsPcaAuthorityArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-serviceconnecttlscertificateauthority.html#cfn-ecs-service-serviceconnecttlscertificateauthority-awspcaauthorityarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aws_pca_authority_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ServiceConnectTlsCertificateAuthority {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref aws_pca_authority_arn) = self.aws_pca_authority_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AwsPcaAuthorityArn", aws_pca_authority_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ServiceConnectTlsCertificateAuthority {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ServiceConnectTlsCertificateAuthority, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ServiceConnectTlsCertificateAuthority;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ServiceConnectTlsCertificateAuthority")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut aws_pca_authority_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AwsPcaAuthorityArn" => {
                                aws_pca_authority_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ServiceConnectTlsCertificateAuthority {
                        aws_pca_authority_arn: aws_pca_authority_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::Service.ServiceConnectTlsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-serviceconnecttlsconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ServiceConnectTlsConfiguration {
        /// Property [`IssuerCertificateAuthority`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-serviceconnecttlsconfiguration.html#cfn-ecs-service-serviceconnecttlsconfiguration-issuercertificateauthority).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub issuer_certificate_authority: ::Value<ServiceConnectTlsCertificateAuthority>,
        /// Property [`KmsKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-serviceconnecttlsconfiguration.html#cfn-ecs-service-serviceconnecttlsconfiguration-kmskey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kms_key: Option<::Value<String>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-serviceconnecttlsconfiguration.html#cfn-ecs-service-serviceconnecttlsconfiguration-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ServiceConnectTlsConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IssuerCertificateAuthority", &self.issuer_certificate_authority)?;
            if let Some(ref kms_key) = self.kms_key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKey", kms_key)?;
            }
            if let Some(ref role_arn) = self.role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", role_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ServiceConnectTlsConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ServiceConnectTlsConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ServiceConnectTlsConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ServiceConnectTlsConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut issuer_certificate_authority: Option<::Value<ServiceConnectTlsCertificateAuthority>> = None;
                    let mut kms_key: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "IssuerCertificateAuthority" => {
                                issuer_certificate_authority = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KmsKey" => {
                                kms_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ServiceConnectTlsConfiguration {
                        issuer_certificate_authority: issuer_certificate_authority.ok_or(::serde::de::Error::missing_field("IssuerCertificateAuthority"))?,
                        kms_key: kms_key,
                        role_arn: role_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::Service.ServiceManagedEBSVolumeConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-servicemanagedebsvolumeconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ServiceManagedEBSVolumeConfiguration {
        /// Property [`Encrypted`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-servicemanagedebsvolumeconfiguration.html#cfn-ecs-service-servicemanagedebsvolumeconfiguration-encrypted).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encrypted: Option<::Value<bool>>,
        /// Property [`FilesystemType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-servicemanagedebsvolumeconfiguration.html#cfn-ecs-service-servicemanagedebsvolumeconfiguration-filesystemtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub filesystem_type: Option<::Value<String>>,
        /// Property [`Iops`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-servicemanagedebsvolumeconfiguration.html#cfn-ecs-service-servicemanagedebsvolumeconfiguration-iops).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub iops: Option<::Value<u32>>,
        /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-servicemanagedebsvolumeconfiguration.html#cfn-ecs-service-servicemanagedebsvolumeconfiguration-kmskeyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kms_key_id: Option<::Value<String>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-servicemanagedebsvolumeconfiguration.html#cfn-ecs-service-servicemanagedebsvolumeconfiguration-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`SizeInGiB`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-servicemanagedebsvolumeconfiguration.html#cfn-ecs-service-servicemanagedebsvolumeconfiguration-sizeingib).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub size_in_gi_b: Option<::Value<u32>>,
        /// Property [`SnapshotId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-servicemanagedebsvolumeconfiguration.html#cfn-ecs-service-servicemanagedebsvolumeconfiguration-snapshotid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub snapshot_id: Option<::Value<String>>,
        /// Property [`TagSpecifications`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-servicemanagedebsvolumeconfiguration.html#cfn-ecs-service-servicemanagedebsvolumeconfiguration-tagspecifications).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tag_specifications: Option<::ValueList<EBSTagSpecification>>,
        /// Property [`Throughput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-servicemanagedebsvolumeconfiguration.html#cfn-ecs-service-servicemanagedebsvolumeconfiguration-throughput).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub throughput: Option<::Value<u32>>,
        /// Property [`VolumeType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-servicemanagedebsvolumeconfiguration.html#cfn-ecs-service-servicemanagedebsvolumeconfiguration-volumetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub volume_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ServiceManagedEBSVolumeConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref encrypted) = self.encrypted {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Encrypted", encrypted)?;
            }
            if let Some(ref filesystem_type) = self.filesystem_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FilesystemType", filesystem_type)?;
            }
            if let Some(ref iops) = self.iops {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Iops", iops)?;
            }
            if let Some(ref kms_key_id) = self.kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            if let Some(ref size_in_gi_b) = self.size_in_gi_b {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SizeInGiB", size_in_gi_b)?;
            }
            if let Some(ref snapshot_id) = self.snapshot_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotId", snapshot_id)?;
            }
            if let Some(ref tag_specifications) = self.tag_specifications {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TagSpecifications", tag_specifications)?;
            }
            if let Some(ref throughput) = self.throughput {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Throughput", throughput)?;
            }
            if let Some(ref volume_type) = self.volume_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeType", volume_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ServiceManagedEBSVolumeConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ServiceManagedEBSVolumeConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ServiceManagedEBSVolumeConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ServiceManagedEBSVolumeConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut encrypted: Option<::Value<bool>> = None;
                    let mut filesystem_type: Option<::Value<String>> = None;
                    let mut iops: Option<::Value<u32>> = None;
                    let mut kms_key_id: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut size_in_gi_b: Option<::Value<u32>> = None;
                    let mut snapshot_id: Option<::Value<String>> = None;
                    let mut tag_specifications: Option<::ValueList<EBSTagSpecification>> = None;
                    let mut throughput: Option<::Value<u32>> = None;
                    let mut volume_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Encrypted" => {
                                encrypted = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FilesystemType" => {
                                filesystem_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Iops" => {
                                iops = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KmsKeyId" => {
                                kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SizeInGiB" => {
                                size_in_gi_b = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SnapshotId" => {
                                snapshot_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TagSpecifications" => {
                                tag_specifications = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Throughput" => {
                                throughput = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VolumeType" => {
                                volume_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ServiceManagedEBSVolumeConfiguration {
                        encrypted: encrypted,
                        filesystem_type: filesystem_type,
                        iops: iops,
                        kms_key_id: kms_key_id,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                        size_in_gi_b: size_in_gi_b,
                        snapshot_id: snapshot_id,
                        tag_specifications: tag_specifications,
                        throughput: throughput,
                        volume_type: volume_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::Service.ServiceRegistry`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-serviceregistry.html) property type.
    #[derive(Debug, Default)]
    pub struct ServiceRegistry {
        /// Property [`ContainerName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-serviceregistry.html#cfn-ecs-service-serviceregistry-containername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub container_name: Option<::Value<String>>,
        /// Property [`ContainerPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-serviceregistry.html#cfn-ecs-service-serviceregistry-containerport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub container_port: Option<::Value<u32>>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-serviceregistry.html#cfn-ecs-service-serviceregistry-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: Option<::Value<u32>>,
        /// Property [`RegistryArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-serviceregistry.html#cfn-ecs-service-serviceregistry-registryarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub registry_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ServiceRegistry {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref container_name) = self.container_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerName", container_name)?;
            }
            if let Some(ref container_port) = self.container_port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerPort", container_port)?;
            }
            if let Some(ref port) = self.port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
            }
            if let Some(ref registry_arn) = self.registry_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RegistryArn", registry_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ServiceRegistry {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ServiceRegistry, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ServiceRegistry;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ServiceRegistry")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut container_name: Option<::Value<String>> = None;
                    let mut container_port: Option<::Value<u32>> = None;
                    let mut port: Option<::Value<u32>> = None;
                    let mut registry_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContainerName" => {
                                container_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ContainerPort" => {
                                container_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RegistryArn" => {
                                registry_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ServiceRegistry {
                        container_name: container_name,
                        container_port: container_port,
                        port: port,
                        registry_arn: registry_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::Service.ServiceVolumeConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-servicevolumeconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ServiceVolumeConfiguration {
        /// Property [`ManagedEBSVolume`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-servicevolumeconfiguration.html#cfn-ecs-service-servicevolumeconfiguration-managedebsvolume).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub managed_ebs_volume: Option<::Value<ServiceManagedEBSVolumeConfiguration>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-servicevolumeconfiguration.html#cfn-ecs-service-servicevolumeconfiguration-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for ServiceVolumeConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref managed_ebs_volume) = self.managed_ebs_volume {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManagedEBSVolume", managed_ebs_volume)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ServiceVolumeConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ServiceVolumeConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ServiceVolumeConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ServiceVolumeConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut managed_ebs_volume: Option<::Value<ServiceManagedEBSVolumeConfiguration>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ManagedEBSVolume" => {
                                managed_ebs_volume = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ServiceVolumeConfiguration {
                        managed_ebs_volume: managed_ebs_volume,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::Service.TimeoutConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-timeoutconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct TimeoutConfiguration {
        /// Property [`IdleTimeoutSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-timeoutconfiguration.html#cfn-ecs-service-timeoutconfiguration-idletimeoutseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub idle_timeout_seconds: Option<::Value<u32>>,
        /// Property [`PerRequestTimeoutSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-timeoutconfiguration.html#cfn-ecs-service-timeoutconfiguration-perrequesttimeoutseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub per_request_timeout_seconds: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for TimeoutConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref idle_timeout_seconds) = self.idle_timeout_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdleTimeoutSeconds", idle_timeout_seconds)?;
            }
            if let Some(ref per_request_timeout_seconds) = self.per_request_timeout_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PerRequestTimeoutSeconds", per_request_timeout_seconds)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TimeoutConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TimeoutConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TimeoutConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TimeoutConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut idle_timeout_seconds: Option<::Value<u32>> = None;
                    let mut per_request_timeout_seconds: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "IdleTimeoutSeconds" => {
                                idle_timeout_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PerRequestTimeoutSeconds" => {
                                per_request_timeout_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TimeoutConfiguration {
                        idle_timeout_seconds: idle_timeout_seconds,
                        per_request_timeout_seconds: per_request_timeout_seconds,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod task_definition {
    //! Property types for the `TaskDefinition` resource.

    /// The [`AWS::ECS::TaskDefinition.AuthorizationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-authorizationconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct AuthorizationConfig {
        /// Property [`AccessPointId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-authorizationconfig.html#cfn-ecs-taskdefinition-authorizationconfig-accesspointid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub access_point_id: Option<::Value<String>>,
        /// Property [`IAM`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-authorizationconfig.html#cfn-ecs-taskdefinition-authorizationconfig-iam).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub iam: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AuthorizationConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref access_point_id) = self.access_point_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessPointId", access_point_id)?;
            }
            if let Some(ref iam) = self.iam {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IAM", iam)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AuthorizationConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AuthorizationConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AuthorizationConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AuthorizationConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut access_point_id: Option<::Value<String>> = None;
                    let mut iam: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AccessPointId" => {
                                access_point_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IAM" => {
                                iam = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AuthorizationConfig {
                        access_point_id: access_point_id,
                        iam: iam,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::TaskDefinition.ContainerDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinition.html) property type.
    #[derive(Debug, Default)]
    pub struct ContainerDefinition {
        /// Property [`Command`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinition.html#cfn-ecs-taskdefinition-containerdefinition-command).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub command: Option<::ValueList<String>>,
        /// Property [`Cpu`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinition.html#cfn-ecs-taskdefinition-containerdefinition-cpu).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub cpu: Option<::Value<u32>>,
        /// Property [`DependsOn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinition.html#cfn-ecs-taskdefinition-containerdefinition-dependson).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub depends_on: Option<::ValueList<ContainerDependency>>,
        /// Property [`DisableNetworking`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinition.html#cfn-ecs-taskdefinition-containerdefinition-disablenetworking).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub disable_networking: Option<::Value<bool>>,
        /// Property [`DnsSearchDomains`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinition.html#cfn-ecs-taskdefinition-containerdefinition-dnssearchdomains).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub dns_search_domains: Option<::ValueList<String>>,
        /// Property [`DnsServers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinition.html#cfn-ecs-taskdefinition-containerdefinition-dnsservers).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub dns_servers: Option<::ValueList<String>>,
        /// Property [`DockerLabels`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinition.html#cfn-ecs-taskdefinition-containerdefinition-dockerlabels).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub docker_labels: Option<::ValueMap<String>>,
        /// Property [`DockerSecurityOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinition.html#cfn-ecs-taskdefinition-containerdefinition-dockersecurityoptions).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub docker_security_options: Option<::ValueList<String>>,
        /// Property [`EntryPoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinition.html#cfn-ecs-taskdefinition-containerdefinition-entrypoint).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub entry_point: Option<::ValueList<String>>,
        /// Property [`Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinition.html#cfn-ecs-taskdefinition-containerdefinition-environment).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub environment: Option<::ValueList<KeyValuePair>>,
        /// Property [`EnvironmentFiles`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinition.html#cfn-ecs-taskdefinition-containerdefinition-environmentfiles).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub environment_files: Option<::ValueList<EnvironmentFile>>,
        /// Property [`Essential`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinition.html#cfn-ecs-taskdefinition-containerdefinition-essential).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub essential: Option<::Value<bool>>,
        /// Property [`ExtraHosts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinition.html#cfn-ecs-taskdefinition-containerdefinition-extrahosts).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub extra_hosts: Option<::ValueList<HostEntry>>,
        /// Property [`FirelensConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinition.html#cfn-ecs-taskdefinition-containerdefinition-firelensconfiguration).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub firelens_configuration: Option<::Value<FirelensConfiguration>>,
        /// Property [`HealthCheck`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinition.html#cfn-ecs-taskdefinition-containerdefinition-healthcheck).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub health_check: Option<::Value<HealthCheck>>,
        /// Property [`Hostname`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinition.html#cfn-ecs-taskdefinition-containerdefinition-hostname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub hostname: Option<::Value<String>>,
        /// Property [`Image`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinition.html#cfn-ecs-taskdefinition-containerdefinition-image).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub image: ::Value<String>,
        /// Property [`Interactive`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinition.html#cfn-ecs-taskdefinition-containerdefinition-interactive).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub interactive: Option<::Value<bool>>,
        /// Property [`Links`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinition.html#cfn-ecs-taskdefinition-containerdefinition-links).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub links: Option<::ValueList<String>>,
        /// Property [`LinuxParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinition.html#cfn-ecs-taskdefinition-containerdefinition-linuxparameters).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub linux_parameters: Option<::Value<LinuxParameters>>,
        /// Property [`LogConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinition.html#cfn-ecs-taskdefinition-containerdefinition-logconfiguration).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub log_configuration: Option<::Value<LogConfiguration>>,
        /// Property [`Memory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinition.html#cfn-ecs-taskdefinition-containerdefinition-memory).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub memory: Option<::Value<u32>>,
        /// Property [`MemoryReservation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinition.html#cfn-ecs-taskdefinition-containerdefinition-memoryreservation).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub memory_reservation: Option<::Value<u32>>,
        /// Property [`MountPoints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinition.html#cfn-ecs-taskdefinition-containerdefinition-mountpoints).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub mount_points: Option<::ValueList<MountPoint>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinition.html#cfn-ecs-taskdefinition-containerdefinition-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`PortMappings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinition.html#cfn-ecs-taskdefinition-containerdefinition-portmappings).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub port_mappings: Option<::ValueList<PortMapping>>,
        /// Property [`Privileged`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinition.html#cfn-ecs-taskdefinition-containerdefinition-privileged).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub privileged: Option<::Value<bool>>,
        /// Property [`PseudoTerminal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinition.html#cfn-ecs-taskdefinition-containerdefinition-pseudoterminal).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub pseudo_terminal: Option<::Value<bool>>,
        /// Property [`ReadonlyRootFilesystem`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinition.html#cfn-ecs-taskdefinition-containerdefinition-readonlyrootfilesystem).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub readonly_root_filesystem: Option<::Value<bool>>,
        /// Property [`RepositoryCredentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinition.html#cfn-ecs-taskdefinition-containerdefinition-repositorycredentials).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub repository_credentials: Option<::Value<RepositoryCredentials>>,
        /// Property [`ResourceRequirements`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinition.html#cfn-ecs-taskdefinition-containerdefinition-resourcerequirements).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub resource_requirements: Option<::ValueList<ResourceRequirement>>,
        /// Property [`Secrets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinition.html#cfn-ecs-taskdefinition-containerdefinition-secrets).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub secrets: Option<::ValueList<Secret>>,
        /// Property [`StartTimeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinition.html#cfn-ecs-taskdefinition-containerdefinition-starttimeout).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub start_timeout: Option<::Value<u32>>,
        /// Property [`StopTimeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinition.html#cfn-ecs-taskdefinition-containerdefinition-stoptimeout).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub stop_timeout: Option<::Value<u32>>,
        /// Property [`SystemControls`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinition.html#cfn-ecs-taskdefinition-containerdefinition-systemcontrols).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub system_controls: Option<::ValueList<SystemControl>>,
        /// Property [`Ulimits`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinition.html#cfn-ecs-taskdefinition-containerdefinition-ulimits).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub ulimits: Option<::ValueList<Ulimit>>,
        /// Property [`User`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinition.html#cfn-ecs-taskdefinition-containerdefinition-user).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub user: Option<::Value<String>>,
        /// Property [`VolumesFrom`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinition.html#cfn-ecs-taskdefinition-containerdefinition-volumesfrom).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub volumes_from: Option<::ValueList<VolumeFrom>>,
        /// Property [`WorkingDirectory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinition.html#cfn-ecs-taskdefinition-containerdefinition-workingdirectory).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub working_directory: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ContainerDefinition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref command) = self.command {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Command", command)?;
            }
            if let Some(ref cpu) = self.cpu {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cpu", cpu)?;
            }
            if let Some(ref depends_on) = self.depends_on {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DependsOn", depends_on)?;
            }
            if let Some(ref disable_networking) = self.disable_networking {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisableNetworking", disable_networking)?;
            }
            if let Some(ref dns_search_domains) = self.dns_search_domains {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DnsSearchDomains", dns_search_domains)?;
            }
            if let Some(ref dns_servers) = self.dns_servers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DnsServers", dns_servers)?;
            }
            if let Some(ref docker_labels) = self.docker_labels {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DockerLabels", docker_labels)?;
            }
            if let Some(ref docker_security_options) = self.docker_security_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DockerSecurityOptions", docker_security_options)?;
            }
            if let Some(ref entry_point) = self.entry_point {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EntryPoint", entry_point)?;
            }
            if let Some(ref environment) = self.environment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Environment", environment)?;
            }
            if let Some(ref environment_files) = self.environment_files {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnvironmentFiles", environment_files)?;
            }
            if let Some(ref essential) = self.essential {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Essential", essential)?;
            }
            if let Some(ref extra_hosts) = self.extra_hosts {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExtraHosts", extra_hosts)?;
            }
            if let Some(ref firelens_configuration) = self.firelens_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FirelensConfiguration", firelens_configuration)?;
            }
            if let Some(ref health_check) = self.health_check {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthCheck", health_check)?;
            }
            if let Some(ref hostname) = self.hostname {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Hostname", hostname)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Image", &self.image)?;
            if let Some(ref interactive) = self.interactive {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Interactive", interactive)?;
            }
            if let Some(ref links) = self.links {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Links", links)?;
            }
            if let Some(ref linux_parameters) = self.linux_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LinuxParameters", linux_parameters)?;
            }
            if let Some(ref log_configuration) = self.log_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogConfiguration", log_configuration)?;
            }
            if let Some(ref memory) = self.memory {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Memory", memory)?;
            }
            if let Some(ref memory_reservation) = self.memory_reservation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MemoryReservation", memory_reservation)?;
            }
            if let Some(ref mount_points) = self.mount_points {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MountPoints", mount_points)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            if let Some(ref port_mappings) = self.port_mappings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PortMappings", port_mappings)?;
            }
            if let Some(ref privileged) = self.privileged {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Privileged", privileged)?;
            }
            if let Some(ref pseudo_terminal) = self.pseudo_terminal {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PseudoTerminal", pseudo_terminal)?;
            }
            if let Some(ref readonly_root_filesystem) = self.readonly_root_filesystem {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReadonlyRootFilesystem", readonly_root_filesystem)?;
            }
            if let Some(ref repository_credentials) = self.repository_credentials {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RepositoryCredentials", repository_credentials)?;
            }
            if let Some(ref resource_requirements) = self.resource_requirements {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceRequirements", resource_requirements)?;
            }
            if let Some(ref secrets) = self.secrets {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Secrets", secrets)?;
            }
            if let Some(ref start_timeout) = self.start_timeout {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartTimeout", start_timeout)?;
            }
            if let Some(ref stop_timeout) = self.stop_timeout {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StopTimeout", stop_timeout)?;
            }
            if let Some(ref system_controls) = self.system_controls {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SystemControls", system_controls)?;
            }
            if let Some(ref ulimits) = self.ulimits {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ulimits", ulimits)?;
            }
            if let Some(ref user) = self.user {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "User", user)?;
            }
            if let Some(ref volumes_from) = self.volumes_from {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumesFrom", volumes_from)?;
            }
            if let Some(ref working_directory) = self.working_directory {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkingDirectory", working_directory)?;
            }
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
                    let mut command: Option<::ValueList<String>> = None;
                    let mut cpu: Option<::Value<u32>> = None;
                    let mut depends_on: Option<::ValueList<ContainerDependency>> = None;
                    let mut disable_networking: Option<::Value<bool>> = None;
                    let mut dns_search_domains: Option<::ValueList<String>> = None;
                    let mut dns_servers: Option<::ValueList<String>> = None;
                    let mut docker_labels: Option<::ValueMap<String>> = None;
                    let mut docker_security_options: Option<::ValueList<String>> = None;
                    let mut entry_point: Option<::ValueList<String>> = None;
                    let mut environment: Option<::ValueList<KeyValuePair>> = None;
                    let mut environment_files: Option<::ValueList<EnvironmentFile>> = None;
                    let mut essential: Option<::Value<bool>> = None;
                    let mut extra_hosts: Option<::ValueList<HostEntry>> = None;
                    let mut firelens_configuration: Option<::Value<FirelensConfiguration>> = None;
                    let mut health_check: Option<::Value<HealthCheck>> = None;
                    let mut hostname: Option<::Value<String>> = None;
                    let mut image: Option<::Value<String>> = None;
                    let mut interactive: Option<::Value<bool>> = None;
                    let mut links: Option<::ValueList<String>> = None;
                    let mut linux_parameters: Option<::Value<LinuxParameters>> = None;
                    let mut log_configuration: Option<::Value<LogConfiguration>> = None;
                    let mut memory: Option<::Value<u32>> = None;
                    let mut memory_reservation: Option<::Value<u32>> = None;
                    let mut mount_points: Option<::ValueList<MountPoint>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut port_mappings: Option<::ValueList<PortMapping>> = None;
                    let mut privileged: Option<::Value<bool>> = None;
                    let mut pseudo_terminal: Option<::Value<bool>> = None;
                    let mut readonly_root_filesystem: Option<::Value<bool>> = None;
                    let mut repository_credentials: Option<::Value<RepositoryCredentials>> = None;
                    let mut resource_requirements: Option<::ValueList<ResourceRequirement>> = None;
                    let mut secrets: Option<::ValueList<Secret>> = None;
                    let mut start_timeout: Option<::Value<u32>> = None;
                    let mut stop_timeout: Option<::Value<u32>> = None;
                    let mut system_controls: Option<::ValueList<SystemControl>> = None;
                    let mut ulimits: Option<::ValueList<Ulimit>> = None;
                    let mut user: Option<::Value<String>> = None;
                    let mut volumes_from: Option<::ValueList<VolumeFrom>> = None;
                    let mut working_directory: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Command" => {
                                command = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Cpu" => {
                                cpu = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DependsOn" => {
                                depends_on = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DisableNetworking" => {
                                disable_networking = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DnsSearchDomains" => {
                                dns_search_domains = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DnsServers" => {
                                dns_servers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DockerLabels" => {
                                docker_labels = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DockerSecurityOptions" => {
                                docker_security_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EntryPoint" => {
                                entry_point = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Environment" => {
                                environment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnvironmentFiles" => {
                                environment_files = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Essential" => {
                                essential = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExtraHosts" => {
                                extra_hosts = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FirelensConfiguration" => {
                                firelens_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HealthCheck" => {
                                health_check = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Hostname" => {
                                hostname = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Image" => {
                                image = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Interactive" => {
                                interactive = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Links" => {
                                links = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LinuxParameters" => {
                                linux_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogConfiguration" => {
                                log_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Memory" => {
                                memory = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MemoryReservation" => {
                                memory_reservation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MountPoints" => {
                                mount_points = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PortMappings" => {
                                port_mappings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Privileged" => {
                                privileged = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PseudoTerminal" => {
                                pseudo_terminal = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReadonlyRootFilesystem" => {
                                readonly_root_filesystem = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RepositoryCredentials" => {
                                repository_credentials = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceRequirements" => {
                                resource_requirements = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Secrets" => {
                                secrets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StartTimeout" => {
                                start_timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StopTimeout" => {
                                stop_timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SystemControls" => {
                                system_controls = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Ulimits" => {
                                ulimits = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "User" => {
                                user = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VolumesFrom" => {
                                volumes_from = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WorkingDirectory" => {
                                working_directory = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ContainerDefinition {
                        command: command,
                        cpu: cpu,
                        depends_on: depends_on,
                        disable_networking: disable_networking,
                        dns_search_domains: dns_search_domains,
                        dns_servers: dns_servers,
                        docker_labels: docker_labels,
                        docker_security_options: docker_security_options,
                        entry_point: entry_point,
                        environment: environment,
                        environment_files: environment_files,
                        essential: essential,
                        extra_hosts: extra_hosts,
                        firelens_configuration: firelens_configuration,
                        health_check: health_check,
                        hostname: hostname,
                        image: image.ok_or(::serde::de::Error::missing_field("Image"))?,
                        interactive: interactive,
                        links: links,
                        linux_parameters: linux_parameters,
                        log_configuration: log_configuration,
                        memory: memory,
                        memory_reservation: memory_reservation,
                        mount_points: mount_points,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        port_mappings: port_mappings,
                        privileged: privileged,
                        pseudo_terminal: pseudo_terminal,
                        readonly_root_filesystem: readonly_root_filesystem,
                        repository_credentials: repository_credentials,
                        resource_requirements: resource_requirements,
                        secrets: secrets,
                        start_timeout: start_timeout,
                        stop_timeout: stop_timeout,
                        system_controls: system_controls,
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

    /// The [`AWS::ECS::TaskDefinition.ContainerDependency`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdependency.html) property type.
    #[derive(Debug, Default)]
    pub struct ContainerDependency {
        /// Property [`Condition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdependency.html#cfn-ecs-taskdefinition-containerdependency-condition).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub condition: Option<::Value<String>>,
        /// Property [`ContainerName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdependency.html#cfn-ecs-taskdefinition-containerdependency-containername).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub container_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ContainerDependency {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref condition) = self.condition {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Condition", condition)?;
            }
            if let Some(ref container_name) = self.container_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerName", container_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ContainerDependency {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ContainerDependency, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ContainerDependency;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ContainerDependency")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut condition: Option<::Value<String>> = None;
                    let mut container_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Condition" => {
                                condition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ContainerName" => {
                                container_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ContainerDependency {
                        condition: condition,
                        container_name: container_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::TaskDefinition.Device`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-device.html) property type.
    #[derive(Debug, Default)]
    pub struct Device {
        /// Property [`ContainerPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-device.html#cfn-ecs-taskdefinition-device-containerpath).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub container_path: Option<::Value<String>>,
        /// Property [`HostPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-device.html#cfn-ecs-taskdefinition-device-hostpath).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub host_path: Option<::Value<String>>,
        /// Property [`Permissions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-device.html#cfn-ecs-taskdefinition-device-permissions).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub permissions: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for Device {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref container_path) = self.container_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerPath", container_path)?;
            }
            if let Some(ref host_path) = self.host_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HostPath", host_path)?;
            }
            if let Some(ref permissions) = self.permissions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Permissions", permissions)?;
            }
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
                    let mut container_path: Option<::Value<String>> = None;
                    let mut host_path: Option<::Value<String>> = None;
                    let mut permissions: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContainerPath" => {
                                container_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HostPath" => {
                                host_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Permissions" => {
                                permissions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Device {
                        container_path: container_path,
                        host_path: host_path,
                        permissions: permissions,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::TaskDefinition.DockerVolumeConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-dockervolumeconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct DockerVolumeConfiguration {
        /// Property [`Autoprovision`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-dockervolumeconfiguration.html#cfn-ecs-taskdefinition-dockervolumeconfiguration-autoprovision).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub autoprovision: Option<::Value<bool>>,
        /// Property [`Driver`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-dockervolumeconfiguration.html#cfn-ecs-taskdefinition-dockervolumeconfiguration-driver).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub driver: Option<::Value<String>>,
        /// Property [`DriverOpts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-dockervolumeconfiguration.html#cfn-ecs-taskdefinition-dockervolumeconfiguration-driveropts).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub driver_opts: Option<::ValueMap<String>>,
        /// Property [`Labels`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-dockervolumeconfiguration.html#cfn-ecs-taskdefinition-dockervolumeconfiguration-labels).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub labels: Option<::ValueMap<String>>,
        /// Property [`Scope`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-dockervolumeconfiguration.html#cfn-ecs-taskdefinition-dockervolumeconfiguration-scope).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub scope: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DockerVolumeConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref autoprovision) = self.autoprovision {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Autoprovision", autoprovision)?;
            }
            if let Some(ref driver) = self.driver {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Driver", driver)?;
            }
            if let Some(ref driver_opts) = self.driver_opts {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DriverOpts", driver_opts)?;
            }
            if let Some(ref labels) = self.labels {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Labels", labels)?;
            }
            if let Some(ref scope) = self.scope {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scope", scope)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DockerVolumeConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DockerVolumeConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DockerVolumeConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DockerVolumeConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut autoprovision: Option<::Value<bool>> = None;
                    let mut driver: Option<::Value<String>> = None;
                    let mut driver_opts: Option<::ValueMap<String>> = None;
                    let mut labels: Option<::ValueMap<String>> = None;
                    let mut scope: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Autoprovision" => {
                                autoprovision = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Driver" => {
                                driver = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DriverOpts" => {
                                driver_opts = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Labels" => {
                                labels = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Scope" => {
                                scope = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DockerVolumeConfiguration {
                        autoprovision: autoprovision,
                        driver: driver,
                        driver_opts: driver_opts,
                        labels: labels,
                        scope: scope,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::TaskDefinition.EFSVolumeConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-efsvolumeconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct EFSVolumeConfiguration {
        /// Property [`AuthorizationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-efsvolumeconfiguration.html#cfn-ecs-taskdefinition-efsvolumeconfiguration-authorizationconfig).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub authorization_config: Option<::Value<AuthorizationConfig>>,
        /// Property [`FilesystemId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-efsvolumeconfiguration.html#cfn-ecs-taskdefinition-efsvolumeconfiguration-filesystemid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub filesystem_id: ::Value<String>,
        /// Property [`RootDirectory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-efsvolumeconfiguration.html#cfn-ecs-taskdefinition-efsvolumeconfiguration-rootdirectory).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub root_directory: Option<::Value<String>>,
        /// Property [`TransitEncryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-efsvolumeconfiguration.html#cfn-ecs-taskdefinition-efsvolumeconfiguration-transitencryption).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub transit_encryption: Option<::Value<String>>,
        /// Property [`TransitEncryptionPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-efsvolumeconfiguration.html#cfn-ecs-taskdefinition-efsvolumeconfiguration-transitencryptionport).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub transit_encryption_port: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for EFSVolumeConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref authorization_config) = self.authorization_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthorizationConfig", authorization_config)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FilesystemId", &self.filesystem_id)?;
            if let Some(ref root_directory) = self.root_directory {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RootDirectory", root_directory)?;
            }
            if let Some(ref transit_encryption) = self.transit_encryption {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TransitEncryption", transit_encryption)?;
            }
            if let Some(ref transit_encryption_port) = self.transit_encryption_port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TransitEncryptionPort", transit_encryption_port)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EFSVolumeConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EFSVolumeConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EFSVolumeConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EFSVolumeConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut authorization_config: Option<::Value<AuthorizationConfig>> = None;
                    let mut filesystem_id: Option<::Value<String>> = None;
                    let mut root_directory: Option<::Value<String>> = None;
                    let mut transit_encryption: Option<::Value<String>> = None;
                    let mut transit_encryption_port: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AuthorizationConfig" => {
                                authorization_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FilesystemId" => {
                                filesystem_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RootDirectory" => {
                                root_directory = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TransitEncryption" => {
                                transit_encryption = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TransitEncryptionPort" => {
                                transit_encryption_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EFSVolumeConfiguration {
                        authorization_config: authorization_config,
                        filesystem_id: filesystem_id.ok_or(::serde::de::Error::missing_field("FilesystemId"))?,
                        root_directory: root_directory,
                        transit_encryption: transit_encryption,
                        transit_encryption_port: transit_encryption_port,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::TaskDefinition.EnvironmentFile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-environmentfile.html) property type.
    #[derive(Debug, Default)]
    pub struct EnvironmentFile {
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-environmentfile.html#cfn-ecs-taskdefinition-environmentfile-type).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub r#type: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-environmentfile.html#cfn-ecs-taskdefinition-environmentfile-value).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EnvironmentFile {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref r#type) = self.r#type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EnvironmentFile {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EnvironmentFile, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EnvironmentFile;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EnvironmentFile")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut r#type: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EnvironmentFile {
                        r#type: r#type,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::TaskDefinition.EphemeralStorage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-ephemeralstorage.html) property type.
    #[derive(Debug, Default)]
    pub struct EphemeralStorage {
        /// Property [`SizeInGiB`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-ephemeralstorage.html#cfn-ecs-taskdefinition-ephemeralstorage-sizeingib).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub size_in_gi_b: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for EphemeralStorage {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref size_in_gi_b) = self.size_in_gi_b {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SizeInGiB", size_in_gi_b)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EphemeralStorage {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EphemeralStorage, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EphemeralStorage;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EphemeralStorage")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut size_in_gi_b: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SizeInGiB" => {
                                size_in_gi_b = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EphemeralStorage {
                        size_in_gi_b: size_in_gi_b,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::TaskDefinition.FirelensConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-firelensconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct FirelensConfiguration {
        /// Property [`Options`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-firelensconfiguration.html#cfn-ecs-taskdefinition-firelensconfiguration-options).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub options: Option<::ValueMap<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-firelensconfiguration.html#cfn-ecs-taskdefinition-firelensconfiguration-type).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub r#type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for FirelensConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref options) = self.options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Options", options)?;
            }
            if let Some(ref r#type) = self.r#type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FirelensConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FirelensConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FirelensConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FirelensConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut options: Option<::ValueMap<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Options" => {
                                options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FirelensConfiguration {
                        options: options,
                        r#type: r#type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::TaskDefinition.HealthCheck`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-healthcheck.html) property type.
    #[derive(Debug, Default)]
    pub struct HealthCheck {
        /// Property [`Command`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-healthcheck.html#cfn-ecs-taskdefinition-healthcheck-command).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub command: Option<::ValueList<String>>,
        /// Property [`Interval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-healthcheck.html#cfn-ecs-taskdefinition-healthcheck-interval).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub interval: Option<::Value<u32>>,
        /// Property [`Retries`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-healthcheck.html#cfn-ecs-taskdefinition-healthcheck-retries).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub retries: Option<::Value<u32>>,
        /// Property [`StartPeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-healthcheck.html#cfn-ecs-taskdefinition-healthcheck-startperiod).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub start_period: Option<::Value<u32>>,
        /// Property [`Timeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-healthcheck.html#cfn-ecs-taskdefinition-healthcheck-timeout).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub timeout: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for HealthCheck {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref command) = self.command {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Command", command)?;
            }
            if let Some(ref interval) = self.interval {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Interval", interval)?;
            }
            if let Some(ref retries) = self.retries {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Retries", retries)?;
            }
            if let Some(ref start_period) = self.start_period {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartPeriod", start_period)?;
            }
            if let Some(ref timeout) = self.timeout {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Timeout", timeout)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HealthCheck {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HealthCheck, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HealthCheck;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HealthCheck")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut command: Option<::ValueList<String>> = None;
                    let mut interval: Option<::Value<u32>> = None;
                    let mut retries: Option<::Value<u32>> = None;
                    let mut start_period: Option<::Value<u32>> = None;
                    let mut timeout: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Command" => {
                                command = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Interval" => {
                                interval = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Retries" => {
                                retries = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StartPeriod" => {
                                start_period = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Timeout" => {
                                timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HealthCheck {
                        command: command,
                        interval: interval,
                        retries: retries,
                        start_period: start_period,
                        timeout: timeout,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::TaskDefinition.HostEntry`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-hostentry.html) property type.
    #[derive(Debug, Default)]
    pub struct HostEntry {
        /// Property [`Hostname`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-hostentry.html#cfn-ecs-taskdefinition-hostentry-hostname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub hostname: Option<::Value<String>>,
        /// Property [`IpAddress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-hostentry.html#cfn-ecs-taskdefinition-hostentry-ipaddress).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub ip_address: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for HostEntry {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref hostname) = self.hostname {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Hostname", hostname)?;
            }
            if let Some(ref ip_address) = self.ip_address {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IpAddress", ip_address)?;
            }
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
                    let mut hostname: Option<::Value<String>> = None;
                    let mut ip_address: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Hostname" => {
                                hostname = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IpAddress" => {
                                ip_address = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HostEntry {
                        hostname: hostname,
                        ip_address: ip_address,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::TaskDefinition.HostVolumeProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-hostvolumeproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct HostVolumeProperties {
        /// Property [`SourcePath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-hostvolumeproperties.html#cfn-ecs-taskdefinition-hostvolumeproperties-sourcepath).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub source_path: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for HostVolumeProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref source_path) = self.source_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourcePath", source_path)?;
            }
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
                    let mut source_path: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SourcePath" => {
                                source_path = ::serde::de::MapAccess::next_value(&mut map)?;
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

    /// The [`AWS::ECS::TaskDefinition.InferenceAccelerator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-inferenceaccelerator.html) property type.
    #[derive(Debug, Default)]
    pub struct InferenceAccelerator {
        /// Property [`DeviceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-inferenceaccelerator.html#cfn-ecs-taskdefinition-inferenceaccelerator-devicename).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub device_name: Option<::Value<String>>,
        /// Property [`DeviceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-inferenceaccelerator.html#cfn-ecs-taskdefinition-inferenceaccelerator-devicetype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub device_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for InferenceAccelerator {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref device_name) = self.device_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceName", device_name)?;
            }
            if let Some(ref device_type) = self.device_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceType", device_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InferenceAccelerator {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InferenceAccelerator, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InferenceAccelerator;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InferenceAccelerator")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut device_name: Option<::Value<String>> = None;
                    let mut device_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DeviceName" => {
                                device_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeviceType" => {
                                device_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InferenceAccelerator {
                        device_name: device_name,
                        device_type: device_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::TaskDefinition.KernelCapabilities`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-kernelcapabilities.html) property type.
    #[derive(Debug, Default)]
    pub struct KernelCapabilities {
        /// Property [`Add`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-kernelcapabilities.html#cfn-ecs-taskdefinition-kernelcapabilities-add).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub add: Option<::ValueList<String>>,
        /// Property [`Drop`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-kernelcapabilities.html#cfn-ecs-taskdefinition-kernelcapabilities-drop).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub drop: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for KernelCapabilities {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref add) = self.add {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Add", add)?;
            }
            if let Some(ref drop) = self.drop {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Drop", drop)?;
            }
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
                    let mut add: Option<::ValueList<String>> = None;
                    let mut drop: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Add" => {
                                add = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Drop" => {
                                drop = ::serde::de::MapAccess::next_value(&mut map)?;
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

    /// The [`AWS::ECS::TaskDefinition.KeyValuePair`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-keyvaluepair.html) property type.
    #[derive(Debug, Default)]
    pub struct KeyValuePair {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-keyvaluepair.html#cfn-ecs-taskdefinition-keyvaluepair-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-keyvaluepair.html#cfn-ecs-taskdefinition-keyvaluepair-value).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for KeyValuePair {
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

    impl ::codec::DeserializeValue for KeyValuePair {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KeyValuePair, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KeyValuePair;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KeyValuePair")
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
    #[derive(Debug, Default)]
    pub struct LinuxParameters {
        /// Property [`Capabilities`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-linuxparameters.html#cfn-ecs-taskdefinition-linuxparameters-capabilities).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub capabilities: Option<::Value<KernelCapabilities>>,
        /// Property [`Devices`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-linuxparameters.html#cfn-ecs-taskdefinition-linuxparameters-devices).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub devices: Option<::ValueList<Device>>,
        /// Property [`InitProcessEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-linuxparameters.html#cfn-ecs-taskdefinition-linuxparameters-initprocessenabled).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub init_process_enabled: Option<::Value<bool>>,
        /// Property [`MaxSwap`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-linuxparameters.html#cfn-ecs-taskdefinition-linuxparameters-maxswap).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub max_swap: Option<::Value<u32>>,
        /// Property [`SharedMemorySize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-linuxparameters.html#cfn-ecs-taskdefinition-linuxparameters-sharedmemorysize).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub shared_memory_size: Option<::Value<u32>>,
        /// Property [`Swappiness`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-linuxparameters.html#cfn-ecs-taskdefinition-linuxparameters-swappiness).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub swappiness: Option<::Value<u32>>,
        /// Property [`Tmpfs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-linuxparameters.html#cfn-ecs-taskdefinition-linuxparameters-tmpfs).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub tmpfs: Option<::ValueList<Tmpfs>>,
    }

    impl ::codec::SerializeValue for LinuxParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref capabilities) = self.capabilities {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Capabilities", capabilities)?;
            }
            if let Some(ref devices) = self.devices {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Devices", devices)?;
            }
            if let Some(ref init_process_enabled) = self.init_process_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InitProcessEnabled", init_process_enabled)?;
            }
            if let Some(ref max_swap) = self.max_swap {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxSwap", max_swap)?;
            }
            if let Some(ref shared_memory_size) = self.shared_memory_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SharedMemorySize", shared_memory_size)?;
            }
            if let Some(ref swappiness) = self.swappiness {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Swappiness", swappiness)?;
            }
            if let Some(ref tmpfs) = self.tmpfs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tmpfs", tmpfs)?;
            }
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
                    let mut capabilities: Option<::Value<KernelCapabilities>> = None;
                    let mut devices: Option<::ValueList<Device>> = None;
                    let mut init_process_enabled: Option<::Value<bool>> = None;
                    let mut max_swap: Option<::Value<u32>> = None;
                    let mut shared_memory_size: Option<::Value<u32>> = None;
                    let mut swappiness: Option<::Value<u32>> = None;
                    let mut tmpfs: Option<::ValueList<Tmpfs>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Capabilities" => {
                                capabilities = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Devices" => {
                                devices = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InitProcessEnabled" => {
                                init_process_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxSwap" => {
                                max_swap = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SharedMemorySize" => {
                                shared_memory_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Swappiness" => {
                                swappiness = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Tmpfs" => {
                                tmpfs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LinuxParameters {
                        capabilities: capabilities,
                        devices: devices,
                        init_process_enabled: init_process_enabled,
                        max_swap: max_swap,
                        shared_memory_size: shared_memory_size,
                        swappiness: swappiness,
                        tmpfs: tmpfs,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::TaskDefinition.LogConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-logconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct LogConfiguration {
        /// Property [`LogDriver`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-logconfiguration.html#cfn-ecs-taskdefinition-logconfiguration-logdriver).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub log_driver: ::Value<String>,
        /// Property [`Options`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-logconfiguration.html#cfn-ecs-taskdefinition-logconfiguration-options).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub options: Option<::ValueMap<String>>,
        /// Property [`SecretOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-logconfiguration.html#cfn-ecs-taskdefinition-logconfiguration-secretoptions).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub secret_options: Option<::ValueList<Secret>>,
    }

    impl ::codec::SerializeValue for LogConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogDriver", &self.log_driver)?;
            if let Some(ref options) = self.options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Options", options)?;
            }
            if let Some(ref secret_options) = self.secret_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretOptions", secret_options)?;
            }
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
                    let mut log_driver: Option<::Value<String>> = None;
                    let mut options: Option<::ValueMap<String>> = None;
                    let mut secret_options: Option<::ValueList<Secret>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LogDriver" => {
                                log_driver = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Options" => {
                                options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretOptions" => {
                                secret_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LogConfiguration {
                        log_driver: log_driver.ok_or(::serde::de::Error::missing_field("LogDriver"))?,
                        options: options,
                        secret_options: secret_options,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::TaskDefinition.MountPoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-mountpoint.html) property type.
    #[derive(Debug, Default)]
    pub struct MountPoint {
        /// Property [`ContainerPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-mountpoint.html#cfn-ecs-taskdefinition-mountpoint-containerpath).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub container_path: Option<::Value<String>>,
        /// Property [`ReadOnly`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-mountpoint.html#cfn-ecs-taskdefinition-mountpoint-readonly).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub read_only: Option<::Value<bool>>,
        /// Property [`SourceVolume`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-mountpoint.html#cfn-ecs-taskdefinition-mountpoint-sourcevolume).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub source_volume: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for MountPoint {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref container_path) = self.container_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerPath", container_path)?;
            }
            if let Some(ref read_only) = self.read_only {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReadOnly", read_only)?;
            }
            if let Some(ref source_volume) = self.source_volume {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceVolume", source_volume)?;
            }
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
                    let mut container_path: Option<::Value<String>> = None;
                    let mut read_only: Option<::Value<bool>> = None;
                    let mut source_volume: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContainerPath" => {
                                container_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReadOnly" => {
                                read_only = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceVolume" => {
                                source_volume = ::serde::de::MapAccess::next_value(&mut map)?;
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

    /// The [`AWS::ECS::TaskDefinition.PortMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-portmapping.html) property type.
    #[derive(Debug, Default)]
    pub struct PortMapping {
        /// Property [`AppProtocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-portmapping.html#cfn-ecs-taskdefinition-portmapping-appprotocol).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub app_protocol: Option<::Value<String>>,
        /// Property [`ContainerPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-portmapping.html#cfn-ecs-taskdefinition-portmapping-containerport).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub container_port: Option<::Value<u32>>,
        /// Property [`ContainerPortRange`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-portmapping.html#cfn-ecs-taskdefinition-portmapping-containerportrange).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub container_port_range: Option<::Value<String>>,
        /// Property [`HostPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-portmapping.html#cfn-ecs-taskdefinition-portmapping-hostport).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub host_port: Option<::Value<u32>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-portmapping.html#cfn-ecs-taskdefinition-portmapping-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Protocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-portmapping.html#cfn-ecs-taskdefinition-portmapping-protocol).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub protocol: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for PortMapping {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref app_protocol) = self.app_protocol {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AppProtocol", app_protocol)?;
            }
            if let Some(ref container_port) = self.container_port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerPort", container_port)?;
            }
            if let Some(ref container_port_range) = self.container_port_range {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerPortRange", container_port_range)?;
            }
            if let Some(ref host_port) = self.host_port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HostPort", host_port)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref protocol) = self.protocol {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocol", protocol)?;
            }
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
                    let mut app_protocol: Option<::Value<String>> = None;
                    let mut container_port: Option<::Value<u32>> = None;
                    let mut container_port_range: Option<::Value<String>> = None;
                    let mut host_port: Option<::Value<u32>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut protocol: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AppProtocol" => {
                                app_protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ContainerPort" => {
                                container_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ContainerPortRange" => {
                                container_port_range = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HostPort" => {
                                host_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Protocol" => {
                                protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PortMapping {
                        app_protocol: app_protocol,
                        container_port: container_port,
                        container_port_range: container_port_range,
                        host_port: host_port,
                        name: name,
                        protocol: protocol,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::TaskDefinition.ProxyConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-proxyconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ProxyConfiguration {
        /// Property [`ContainerName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-proxyconfiguration.html#cfn-ecs-taskdefinition-proxyconfiguration-containername).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub container_name: ::Value<String>,
        /// Property [`ProxyConfigurationProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-proxyconfiguration.html#cfn-ecs-taskdefinition-proxyconfiguration-proxyconfigurationproperties).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub proxy_configuration_properties: Option<::ValueList<KeyValuePair>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-proxyconfiguration.html#cfn-ecs-taskdefinition-proxyconfiguration-type).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub r#type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ProxyConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerName", &self.container_name)?;
            if let Some(ref proxy_configuration_properties) = self.proxy_configuration_properties {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProxyConfigurationProperties", proxy_configuration_properties)?;
            }
            if let Some(ref r#type) = self.r#type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ProxyConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ProxyConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ProxyConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ProxyConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut container_name: Option<::Value<String>> = None;
                    let mut proxy_configuration_properties: Option<::ValueList<KeyValuePair>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContainerName" => {
                                container_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProxyConfigurationProperties" => {
                                proxy_configuration_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ProxyConfiguration {
                        container_name: container_name.ok_or(::serde::de::Error::missing_field("ContainerName"))?,
                        proxy_configuration_properties: proxy_configuration_properties,
                        r#type: r#type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::TaskDefinition.RepositoryCredentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-repositorycredentials.html) property type.
    #[derive(Debug, Default)]
    pub struct RepositoryCredentials {
        /// Property [`CredentialsParameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-repositorycredentials.html#cfn-ecs-taskdefinition-repositorycredentials-credentialsparameter).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub credentials_parameter: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for RepositoryCredentials {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref credentials_parameter) = self.credentials_parameter {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CredentialsParameter", credentials_parameter)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RepositoryCredentials {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RepositoryCredentials, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RepositoryCredentials;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RepositoryCredentials")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut credentials_parameter: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CredentialsParameter" => {
                                credentials_parameter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RepositoryCredentials {
                        credentials_parameter: credentials_parameter,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::TaskDefinition.ResourceRequirement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-resourcerequirement.html) property type.
    #[derive(Debug, Default)]
    pub struct ResourceRequirement {
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-resourcerequirement.html#cfn-ecs-taskdefinition-resourcerequirement-type).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub r#type: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-resourcerequirement.html#cfn-ecs-taskdefinition-resourcerequirement-value).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for ResourceRequirement {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResourceRequirement {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourceRequirement, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResourceRequirement;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResourceRequirement")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut r#type: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResourceRequirement {
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::TaskDefinition.RuntimePlatform`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-runtimeplatform.html) property type.
    #[derive(Debug, Default)]
    pub struct RuntimePlatform {
        /// Property [`CpuArchitecture`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-runtimeplatform.html#cfn-ecs-taskdefinition-runtimeplatform-cpuarchitecture).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub cpu_architecture: Option<::Value<String>>,
        /// Property [`OperatingSystemFamily`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-runtimeplatform.html#cfn-ecs-taskdefinition-runtimeplatform-operatingsystemfamily).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub operating_system_family: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for RuntimePlatform {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cpu_architecture) = self.cpu_architecture {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CpuArchitecture", cpu_architecture)?;
            }
            if let Some(ref operating_system_family) = self.operating_system_family {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OperatingSystemFamily", operating_system_family)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RuntimePlatform {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RuntimePlatform, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RuntimePlatform;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RuntimePlatform")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cpu_architecture: Option<::Value<String>> = None;
                    let mut operating_system_family: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CpuArchitecture" => {
                                cpu_architecture = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OperatingSystemFamily" => {
                                operating_system_family = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RuntimePlatform {
                        cpu_architecture: cpu_architecture,
                        operating_system_family: operating_system_family,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::TaskDefinition.Secret`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-secret.html) property type.
    #[derive(Debug, Default)]
    pub struct Secret {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-secret.html#cfn-ecs-taskdefinition-secret-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`ValueFrom`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-secret.html#cfn-ecs-taskdefinition-secret-valuefrom).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub value_from: ::Value<String>,
    }

    impl ::codec::SerializeValue for Secret {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ValueFrom", &self.value_from)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Secret {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Secret, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Secret;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Secret")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut value_from: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ValueFrom" => {
                                value_from = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Secret {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        value_from: value_from.ok_or(::serde::de::Error::missing_field("ValueFrom"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::TaskDefinition.SystemControl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-systemcontrol.html) property type.
    #[derive(Debug, Default)]
    pub struct SystemControl {
        /// Property [`Namespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-systemcontrol.html#cfn-ecs-taskdefinition-systemcontrol-namespace).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub namespace: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-systemcontrol.html#cfn-ecs-taskdefinition-systemcontrol-value).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SystemControl {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref namespace) = self.namespace {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Namespace", namespace)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SystemControl {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SystemControl, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SystemControl;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SystemControl")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut namespace: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Namespace" => {
                                namespace = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SystemControl {
                        namespace: namespace,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::TaskDefinition.TaskDefinitionPlacementConstraint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-taskdefinitionplacementconstraint.html) property type.
    #[derive(Debug, Default)]
    pub struct TaskDefinitionPlacementConstraint {
        /// Property [`Expression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-taskdefinitionplacementconstraint.html#cfn-ecs-taskdefinition-taskdefinitionplacementconstraint-expression).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub expression: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-taskdefinitionplacementconstraint.html#cfn-ecs-taskdefinition-taskdefinitionplacementconstraint-type).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for TaskDefinitionPlacementConstraint {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref expression) = self.expression {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Expression", expression)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
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
                    let mut expression: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Expression" => {
                                expression = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TaskDefinitionPlacementConstraint {
                        expression: expression,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::TaskDefinition.Tmpfs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-tmpfs.html) property type.
    #[derive(Debug, Default)]
    pub struct Tmpfs {
        /// Property [`ContainerPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-tmpfs.html#cfn-ecs-taskdefinition-tmpfs-containerpath).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub container_path: Option<::Value<String>>,
        /// Property [`MountOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-tmpfs.html#cfn-ecs-taskdefinition-tmpfs-mountoptions).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub mount_options: Option<::ValueList<String>>,
        /// Property [`Size`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-tmpfs.html#cfn-ecs-taskdefinition-tmpfs-size).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub size: ::Value<u32>,
    }

    impl ::codec::SerializeValue for Tmpfs {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref container_path) = self.container_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerPath", container_path)?;
            }
            if let Some(ref mount_options) = self.mount_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MountOptions", mount_options)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Size", &self.size)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Tmpfs {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Tmpfs, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Tmpfs;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Tmpfs")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut container_path: Option<::Value<String>> = None;
                    let mut mount_options: Option<::ValueList<String>> = None;
                    let mut size: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContainerPath" => {
                                container_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MountOptions" => {
                                mount_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Size" => {
                                size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Tmpfs {
                        container_path: container_path,
                        mount_options: mount_options,
                        size: size.ok_or(::serde::de::Error::missing_field("Size"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::TaskDefinition.Ulimit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-ulimit.html) property type.
    #[derive(Debug, Default)]
    pub struct Ulimit {
        /// Property [`HardLimit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-ulimit.html#cfn-ecs-taskdefinition-ulimit-hardlimit).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub hard_limit: ::Value<u32>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-ulimit.html#cfn-ecs-taskdefinition-ulimit-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`SoftLimit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-ulimit.html#cfn-ecs-taskdefinition-ulimit-softlimit).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
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
                    let mut hard_limit: Option<::Value<u32>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut soft_limit: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HardLimit" => {
                                hard_limit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SoftLimit" => {
                                soft_limit = ::serde::de::MapAccess::next_value(&mut map)?;
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

    /// The [`AWS::ECS::TaskDefinition.Volume`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-volume.html) property type.
    #[derive(Debug, Default)]
    pub struct Volume {
        /// Property [`ConfiguredAtLaunch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-volume.html#cfn-ecs-taskdefinition-volume-configuredatlaunch).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub configured_at_launch: Option<::Value<bool>>,
        /// Property [`DockerVolumeConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-volume.html#cfn-ecs-taskdefinition-volume-dockervolumeconfiguration).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub docker_volume_configuration: Option<::Value<DockerVolumeConfiguration>>,
        /// Property [`EFSVolumeConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-volume.html#cfn-ecs-taskdefinition-volume-efsvolumeconfiguration).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub efs_volume_configuration: Option<::Value<EFSVolumeConfiguration>>,
        /// Property [`Host`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-volume.html#cfn-ecs-taskdefinition-volume-host).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub host: Option<::Value<HostVolumeProperties>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-volume.html#cfn-ecs-taskdefinition-volume-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Volume {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref configured_at_launch) = self.configured_at_launch {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfiguredAtLaunch", configured_at_launch)?;
            }
            if let Some(ref docker_volume_configuration) = self.docker_volume_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DockerVolumeConfiguration", docker_volume_configuration)?;
            }
            if let Some(ref efs_volume_configuration) = self.efs_volume_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EFSVolumeConfiguration", efs_volume_configuration)?;
            }
            if let Some(ref host) = self.host {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Host", host)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
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
                    let mut configured_at_launch: Option<::Value<bool>> = None;
                    let mut docker_volume_configuration: Option<::Value<DockerVolumeConfiguration>> = None;
                    let mut efs_volume_configuration: Option<::Value<EFSVolumeConfiguration>> = None;
                    let mut host: Option<::Value<HostVolumeProperties>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConfiguredAtLaunch" => {
                                configured_at_launch = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DockerVolumeConfiguration" => {
                                docker_volume_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EFSVolumeConfiguration" => {
                                efs_volume_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Host" => {
                                host = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Volume {
                        configured_at_launch: configured_at_launch,
                        docker_volume_configuration: docker_volume_configuration,
                        efs_volume_configuration: efs_volume_configuration,
                        host: host,
                        name: name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::TaskDefinition.VolumeFrom`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-volumefrom.html) property type.
    #[derive(Debug, Default)]
    pub struct VolumeFrom {
        /// Property [`ReadOnly`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-volumefrom.html#cfn-ecs-taskdefinition-volumefrom-readonly).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub read_only: Option<::Value<bool>>,
        /// Property [`SourceContainer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-volumefrom.html#cfn-ecs-taskdefinition-volumefrom-sourcecontainer).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub source_container: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for VolumeFrom {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref read_only) = self.read_only {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReadOnly", read_only)?;
            }
            if let Some(ref source_container) = self.source_container {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceContainer", source_container)?;
            }
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
                    let mut read_only: Option<::Value<bool>> = None;
                    let mut source_container: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ReadOnly" => {
                                read_only = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceContainer" => {
                                source_container = ::serde::de::MapAccess::next_value(&mut map)?;
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

pub mod task_set {
    //! Property types for the `TaskSet` resource.

    /// The [`AWS::ECS::TaskSet.AwsVpcConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskset-awsvpcconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct AwsVpcConfiguration {
        /// Property [`AssignPublicIp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskset-awsvpcconfiguration.html#cfn-ecs-taskset-awsvpcconfiguration-assignpublicip).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub assign_public_ip: Option<::Value<String>>,
        /// Property [`SecurityGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskset-awsvpcconfiguration.html#cfn-ecs-taskset-awsvpcconfiguration-securitygroups).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub security_groups: Option<::ValueList<String>>,
        /// Property [`Subnets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskset-awsvpcconfiguration.html#cfn-ecs-taskset-awsvpcconfiguration-subnets).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub subnets: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for AwsVpcConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref assign_public_ip) = self.assign_public_ip {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssignPublicIp", assign_public_ip)?;
            }
            if let Some(ref security_groups) = self.security_groups {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroups", security_groups)?;
            }
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
                    let mut assign_public_ip: Option<::Value<String>> = None;
                    let mut security_groups: Option<::ValueList<String>> = None;
                    let mut subnets: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AssignPublicIp" => {
                                assign_public_ip = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecurityGroups" => {
                                security_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Subnets" => {
                                subnets = ::serde::de::MapAccess::next_value(&mut map)?;
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

    /// The [`AWS::ECS::TaskSet.LoadBalancer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskset-loadbalancer.html) property type.
    #[derive(Debug, Default)]
    pub struct LoadBalancer {
        /// Property [`ContainerName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskset-loadbalancer.html#cfn-ecs-taskset-loadbalancer-containername).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub container_name: Option<::Value<String>>,
        /// Property [`ContainerPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskset-loadbalancer.html#cfn-ecs-taskset-loadbalancer-containerport).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub container_port: Option<::Value<u32>>,
        /// Property [`TargetGroupArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskset-loadbalancer.html#cfn-ecs-taskset-loadbalancer-targetgrouparn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub target_group_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LoadBalancer {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref container_name) = self.container_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerName", container_name)?;
            }
            if let Some(ref container_port) = self.container_port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerPort", container_port)?;
            }
            if let Some(ref target_group_arn) = self.target_group_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetGroupArn", target_group_arn)?;
            }
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
                    let mut container_name: Option<::Value<String>> = None;
                    let mut container_port: Option<::Value<u32>> = None;
                    let mut target_group_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContainerName" => {
                                container_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ContainerPort" => {
                                container_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetGroupArn" => {
                                target_group_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LoadBalancer {
                        container_name: container_name,
                        container_port: container_port,
                        target_group_arn: target_group_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::TaskSet.NetworkConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskset-networkconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct NetworkConfiguration {
        /// Property [`AwsVpcConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskset-networkconfiguration.html#cfn-ecs-taskset-networkconfiguration-awsvpcconfiguration).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub aws_vpc_configuration: Option<::Value<AwsVpcConfiguration>>,
    }

    impl ::codec::SerializeValue for NetworkConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref aws_vpc_configuration) = self.aws_vpc_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AwsVpcConfiguration", aws_vpc_configuration)?;
            }
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
                    let mut aws_vpc_configuration: Option<::Value<AwsVpcConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AwsVpcConfiguration" => {
                                aws_vpc_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NetworkConfiguration {
                        aws_vpc_configuration: aws_vpc_configuration,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::TaskSet.Scale`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskset-scale.html) property type.
    #[derive(Debug, Default)]
    pub struct Scale {
        /// Property [`Unit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskset-scale.html#cfn-ecs-taskset-scale-unit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unit: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskset-scale.html#cfn-ecs-taskset-scale-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<f64>>,
    }

    impl ::codec::SerializeValue for Scale {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref unit) = self.unit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Unit", unit)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Scale {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Scale, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Scale;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Scale")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut unit: Option<::Value<String>> = None;
                    let mut value: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Unit" => {
                                unit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Scale {
                        unit: unit,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ECS::TaskSet.ServiceRegistry`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskset-serviceregistry.html) property type.
    #[derive(Debug, Default)]
    pub struct ServiceRegistry {
        /// Property [`ContainerName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskset-serviceregistry.html#cfn-ecs-taskset-serviceregistry-containername).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub container_name: Option<::Value<String>>,
        /// Property [`ContainerPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskset-serviceregistry.html#cfn-ecs-taskset-serviceregistry-containerport).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub container_port: Option<::Value<u32>>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskset-serviceregistry.html#cfn-ecs-taskset-serviceregistry-port).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub port: Option<::Value<u32>>,
        /// Property [`RegistryArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskset-serviceregistry.html#cfn-ecs-taskset-serviceregistry-registryarn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub registry_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ServiceRegistry {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref container_name) = self.container_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerName", container_name)?;
            }
            if let Some(ref container_port) = self.container_port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerPort", container_port)?;
            }
            if let Some(ref port) = self.port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
            }
            if let Some(ref registry_arn) = self.registry_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RegistryArn", registry_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ServiceRegistry {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ServiceRegistry, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ServiceRegistry;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ServiceRegistry")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut container_name: Option<::Value<String>> = None;
                    let mut container_port: Option<::Value<u32>> = None;
                    let mut port: Option<::Value<u32>> = None;
                    let mut registry_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContainerName" => {
                                container_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ContainerPort" => {
                                container_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RegistryArn" => {
                                registry_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ServiceRegistry {
                        container_name: container_name,
                        container_port: container_port,
                        port: port,
                        registry_arn: registry_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
