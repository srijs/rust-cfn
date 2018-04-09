//! Types for the `OpsWorks` service.

/// The [`AWS::OpsWorks::App`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-app.html) resource type.
#[derive(Debug)]
pub struct App {
    properties: AppProperties
}

/// Properties for the `App` resource.
#[derive(Debug)]
pub struct AppProperties {
    /// Property `AppSource`.
    pub app_source: Option<::Value<self::app::Source>>,
    /// Property `Attributes`.
    pub attributes: Option<::ValueMap<String>>,
    /// Property `DataSources`.
    pub data_sources: Option<::ValueList<self::app::DataSource>>,
    /// Property `Description`.
    pub description: Option<::Value<String>>,
    /// Property `Domains`.
    pub domains: Option<::ValueList<String>>,
    /// Property `EnableSsl`.
    pub enable_ssl: Option<::Value<bool>>,
    /// Property `Environment`.
    pub environment: Option<::ValueList<self::app::EnvironmentVariable>>,
    /// Property `Name`.
    pub name: ::Value<String>,
    /// Property `Shortname`.
    pub shortname: Option<::Value<String>>,
    /// Property `SslConfiguration`.
    pub ssl_configuration: Option<::Value<self::app::SslConfiguration>>,
    /// Property `StackId`.
    pub stack_id: ::Value<String>,
    /// Property `Type`.
    pub type_: ::Value<String>,
}

impl ::serde::Serialize for AppProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AppSource", &self.app_source)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Attributes", &self.attributes)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataSources", &self.data_sources)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Domains", &self.domains)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableSsl", &self.enable_ssl)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Environment", &self.environment)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Shortname", &self.shortname)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SslConfiguration", &self.ssl_configuration)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StackId", &self.stack_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AppProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AppProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AppProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AppProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut app_source = None;
                let mut attributes = None;
                let mut data_sources = None;
                let mut description = None;
                let mut domains = None;
                let mut enable_ssl = None;
                let mut environment = None;
                let mut name = None;
                let mut shortname = None;
                let mut ssl_configuration = None;
                let mut stack_id = None;
                let mut type_ = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AppSource" => {
                            app_source = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Attributes" => {
                            attributes = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DataSources" => {
                            data_sources = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Description" => {
                            description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Domains" => {
                            domains = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EnableSsl" => {
                            enable_ssl = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Environment" => {
                            environment = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Shortname" => {
                            shortname = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SslConfiguration" => {
                            ssl_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "StackId" => {
                            stack_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Type" => {
                            type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(AppProperties {
                    app_source: app_source,
                    attributes: attributes,
                    data_sources: data_sources,
                    description: description,
                    domains: domains,
                    enable_ssl: enable_ssl,
                    environment: environment,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    shortname: shortname,
                    ssl_configuration: ssl_configuration,
                    stack_id: stack_id.ok_or(::serde::de::Error::missing_field("StackId"))?,
                    type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct ElasticLoadBalancerAttachmentProperties {
    /// Property `ElasticLoadBalancerName`.
    pub elastic_load_balancer_name: ::Value<String>,
    /// Property `LayerId`.
    pub layer_id: ::Value<String>,
}

impl ::serde::Serialize for ElasticLoadBalancerAttachmentProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ElasticLoadBalancerName", &self.elastic_load_balancer_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LayerId", &self.layer_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ElasticLoadBalancerAttachmentProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ElasticLoadBalancerAttachmentProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ElasticLoadBalancerAttachmentProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ElasticLoadBalancerAttachmentProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut elastic_load_balancer_name = None;
                let mut layer_id = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ElasticLoadBalancerName" => {
                            elastic_load_balancer_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "LayerId" => {
                            layer_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(ElasticLoadBalancerAttachmentProperties {
                    elastic_load_balancer_name: elastic_load_balancer_name.ok_or(::serde::de::Error::missing_field("ElasticLoadBalancerName"))?,
                    layer_id: layer_id.ok_or(::serde::de::Error::missing_field("LayerId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct InstanceProperties {
    /// Property `AgentVersion`.
    pub agent_version: Option<::Value<String>>,
    /// Property `AmiId`.
    pub ami_id: Option<::Value<String>>,
    /// Property `Architecture`.
    pub architecture: Option<::Value<String>>,
    /// Property `AutoScalingType`.
    pub auto_scaling_type: Option<::Value<String>>,
    /// Property `AvailabilityZone`.
    pub availability_zone: Option<::Value<String>>,
    /// Property `BlockDeviceMappings`.
    pub block_device_mappings: Option<::ValueList<self::instance::BlockDeviceMapping>>,
    /// Property `EbsOptimized`.
    pub ebs_optimized: Option<::Value<bool>>,
    /// Property `ElasticIps`.
    pub elastic_ips: Option<::ValueList<String>>,
    /// Property `Hostname`.
    pub hostname: Option<::Value<String>>,
    /// Property `InstallUpdatesOnBoot`.
    pub install_updates_on_boot: Option<::Value<bool>>,
    /// Property `InstanceType`.
    pub instance_type: ::Value<String>,
    /// Property `LayerIds`.
    pub layer_ids: ::ValueList<String>,
    /// Property `Os`.
    pub os: Option<::Value<String>>,
    /// Property `RootDeviceType`.
    pub root_device_type: Option<::Value<String>>,
    /// Property `SshKeyName`.
    pub ssh_key_name: Option<::Value<String>>,
    /// Property `StackId`.
    pub stack_id: ::Value<String>,
    /// Property `SubnetId`.
    pub subnet_id: Option<::Value<String>>,
    /// Property `Tenancy`.
    pub tenancy: Option<::Value<String>>,
    /// Property `TimeBasedAutoScaling`.
    pub time_based_auto_scaling: Option<::Value<self::instance::TimeBasedAutoScaling>>,
    /// Property `VirtualizationType`.
    pub virtualization_type: Option<::Value<String>>,
    /// Property `Volumes`.
    pub volumes: Option<::ValueList<String>>,
}

impl ::serde::Serialize for InstanceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AgentVersion", &self.agent_version)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AmiId", &self.ami_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Architecture", &self.architecture)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoScalingType", &self.auto_scaling_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZone", &self.availability_zone)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BlockDeviceMappings", &self.block_device_mappings)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EbsOptimized", &self.ebs_optimized)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ElasticIps", &self.elastic_ips)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Hostname", &self.hostname)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstallUpdatesOnBoot", &self.install_updates_on_boot)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceType", &self.instance_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LayerIds", &self.layer_ids)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Os", &self.os)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RootDeviceType", &self.root_device_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SshKeyName", &self.ssh_key_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StackId", &self.stack_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetId", &self.subnet_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tenancy", &self.tenancy)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeBasedAutoScaling", &self.time_based_auto_scaling)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VirtualizationType", &self.virtualization_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Volumes", &self.volumes)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for InstanceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<InstanceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = InstanceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type InstanceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut agent_version = None;
                let mut ami_id = None;
                let mut architecture = None;
                let mut auto_scaling_type = None;
                let mut availability_zone = None;
                let mut block_device_mappings = None;
                let mut ebs_optimized = None;
                let mut elastic_ips = None;
                let mut hostname = None;
                let mut install_updates_on_boot = None;
                let mut instance_type = None;
                let mut layer_ids = None;
                let mut os = None;
                let mut root_device_type = None;
                let mut ssh_key_name = None;
                let mut stack_id = None;
                let mut subnet_id = None;
                let mut tenancy = None;
                let mut time_based_auto_scaling = None;
                let mut virtualization_type = None;
                let mut volumes = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AgentVersion" => {
                            agent_version = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "AmiId" => {
                            ami_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Architecture" => {
                            architecture = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "AutoScalingType" => {
                            auto_scaling_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "AvailabilityZone" => {
                            availability_zone = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "BlockDeviceMappings" => {
                            block_device_mappings = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EbsOptimized" => {
                            ebs_optimized = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ElasticIps" => {
                            elastic_ips = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Hostname" => {
                            hostname = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "InstallUpdatesOnBoot" => {
                            install_updates_on_boot = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "InstanceType" => {
                            instance_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "LayerIds" => {
                            layer_ids = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Os" => {
                            os = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RootDeviceType" => {
                            root_device_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SshKeyName" => {
                            ssh_key_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "StackId" => {
                            stack_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SubnetId" => {
                            subnet_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tenancy" => {
                            tenancy = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "TimeBasedAutoScaling" => {
                            time_based_auto_scaling = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "VirtualizationType" => {
                            virtualization_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Volumes" => {
                            volumes = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(InstanceProperties {
                    agent_version: agent_version,
                    ami_id: ami_id,
                    architecture: architecture,
                    auto_scaling_type: auto_scaling_type,
                    availability_zone: availability_zone,
                    block_device_mappings: block_device_mappings,
                    ebs_optimized: ebs_optimized,
                    elastic_ips: elastic_ips,
                    hostname: hostname,
                    install_updates_on_boot: install_updates_on_boot,
                    instance_type: instance_type.ok_or(::serde::de::Error::missing_field("InstanceType"))?,
                    layer_ids: layer_ids.ok_or(::serde::de::Error::missing_field("LayerIds"))?,
                    os: os,
                    root_device_type: root_device_type,
                    ssh_key_name: ssh_key_name,
                    stack_id: stack_id.ok_or(::serde::de::Error::missing_field("StackId"))?,
                    subnet_id: subnet_id,
                    tenancy: tenancy,
                    time_based_auto_scaling: time_based_auto_scaling,
                    virtualization_type: virtualization_type,
                    volumes: volumes,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct LayerProperties {
    /// Property `Attributes`.
    pub attributes: Option<::ValueMap<String>>,
    /// Property `AutoAssignElasticIps`.
    pub auto_assign_elastic_ips: ::Value<bool>,
    /// Property `AutoAssignPublicIps`.
    pub auto_assign_public_ips: ::Value<bool>,
    /// Property `CustomInstanceProfileArn`.
    pub custom_instance_profile_arn: Option<::Value<String>>,
    /// Property `CustomJson`.
    pub custom_json: Option<::Value<::json::Value>>,
    /// Property `CustomRecipes`.
    pub custom_recipes: Option<::Value<self::layer::Recipes>>,
    /// Property `CustomSecurityGroupIds`.
    pub custom_security_group_ids: Option<::ValueList<String>>,
    /// Property `EnableAutoHealing`.
    pub enable_auto_healing: ::Value<bool>,
    /// Property `InstallUpdatesOnBoot`.
    pub install_updates_on_boot: Option<::Value<bool>>,
    /// Property `LifecycleEventConfiguration`.
    pub lifecycle_event_configuration: Option<::Value<self::layer::LifecycleEventConfiguration>>,
    /// Property `LoadBasedAutoScaling`.
    pub load_based_auto_scaling: Option<::Value<self::layer::LoadBasedAutoScaling>>,
    /// Property `Name`.
    pub name: ::Value<String>,
    /// Property `Packages`.
    pub packages: Option<::ValueList<String>>,
    /// Property `Shortname`.
    pub shortname: ::Value<String>,
    /// Property `StackId`.
    pub stack_id: ::Value<String>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `Type`.
    pub type_: ::Value<String>,
    /// Property `UseEbsOptimizedInstances`.
    pub use_ebs_optimized_instances: Option<::Value<bool>>,
    /// Property `VolumeConfigurations`.
    pub volume_configurations: Option<::ValueList<self::layer::VolumeConfiguration>>,
}

impl ::serde::Serialize for LayerProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Attributes", &self.attributes)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoAssignElasticIps", &self.auto_assign_elastic_ips)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoAssignPublicIps", &self.auto_assign_public_ips)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomInstanceProfileArn", &self.custom_instance_profile_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomJson", &self.custom_json)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomRecipes", &self.custom_recipes)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomSecurityGroupIds", &self.custom_security_group_ids)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableAutoHealing", &self.enable_auto_healing)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstallUpdatesOnBoot", &self.install_updates_on_boot)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LifecycleEventConfiguration", &self.lifecycle_event_configuration)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoadBasedAutoScaling", &self.load_based_auto_scaling)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Packages", &self.packages)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Shortname", &self.shortname)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StackId", &self.stack_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "UseEbsOptimizedInstances", &self.use_ebs_optimized_instances)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeConfigurations", &self.volume_configurations)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LayerProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LayerProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LayerProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LayerProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut attributes = None;
                let mut auto_assign_elastic_ips = None;
                let mut auto_assign_public_ips = None;
                let mut custom_instance_profile_arn = None;
                let mut custom_json = None;
                let mut custom_recipes = None;
                let mut custom_security_group_ids = None;
                let mut enable_auto_healing = None;
                let mut install_updates_on_boot = None;
                let mut lifecycle_event_configuration = None;
                let mut load_based_auto_scaling = None;
                let mut name = None;
                let mut packages = None;
                let mut shortname = None;
                let mut stack_id = None;
                let mut tags = None;
                let mut type_ = None;
                let mut use_ebs_optimized_instances = None;
                let mut volume_configurations = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Attributes" => {
                            attributes = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "AutoAssignElasticIps" => {
                            auto_assign_elastic_ips = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "AutoAssignPublicIps" => {
                            auto_assign_public_ips = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "CustomInstanceProfileArn" => {
                            custom_instance_profile_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "CustomJson" => {
                            custom_json = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "CustomRecipes" => {
                            custom_recipes = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "CustomSecurityGroupIds" => {
                            custom_security_group_ids = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EnableAutoHealing" => {
                            enable_auto_healing = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "InstallUpdatesOnBoot" => {
                            install_updates_on_boot = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "LifecycleEventConfiguration" => {
                            lifecycle_event_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "LoadBasedAutoScaling" => {
                            load_based_auto_scaling = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Packages" => {
                            packages = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Shortname" => {
                            shortname = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "StackId" => {
                            stack_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Type" => {
                            type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "UseEbsOptimizedInstances" => {
                            use_ebs_optimized_instances = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "VolumeConfigurations" => {
                            volume_configurations = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(LayerProperties {
                    attributes: attributes,
                    auto_assign_elastic_ips: auto_assign_elastic_ips.ok_or(::serde::de::Error::missing_field("AutoAssignElasticIps"))?,
                    auto_assign_public_ips: auto_assign_public_ips.ok_or(::serde::de::Error::missing_field("AutoAssignPublicIps"))?,
                    custom_instance_profile_arn: custom_instance_profile_arn,
                    custom_json: custom_json,
                    custom_recipes: custom_recipes,
                    custom_security_group_ids: custom_security_group_ids,
                    enable_auto_healing: enable_auto_healing.ok_or(::serde::de::Error::missing_field("EnableAutoHealing"))?,
                    install_updates_on_boot: install_updates_on_boot,
                    lifecycle_event_configuration: lifecycle_event_configuration,
                    load_based_auto_scaling: load_based_auto_scaling,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    packages: packages,
                    shortname: shortname.ok_or(::serde::de::Error::missing_field("Shortname"))?,
                    stack_id: stack_id.ok_or(::serde::de::Error::missing_field("StackId"))?,
                    tags: tags,
                    type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                    use_ebs_optimized_instances: use_ebs_optimized_instances,
                    volume_configurations: volume_configurations,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct StackProperties {
    /// Property `AgentVersion`.
    pub agent_version: Option<::Value<String>>,
    /// Property `Attributes`.
    pub attributes: Option<::ValueMap<String>>,
    /// Property `ChefConfiguration`.
    pub chef_configuration: Option<::Value<self::stack::ChefConfiguration>>,
    /// Property `CloneAppIds`.
    pub clone_app_ids: Option<::ValueList<String>>,
    /// Property `ClonePermissions`.
    pub clone_permissions: Option<::Value<bool>>,
    /// Property `ConfigurationManager`.
    pub configuration_manager: Option<::Value<self::stack::StackConfigurationManager>>,
    /// Property `CustomCookbooksSource`.
    pub custom_cookbooks_source: Option<::Value<self::stack::Source>>,
    /// Property `CustomJson`.
    pub custom_json: Option<::Value<::json::Value>>,
    /// Property `DefaultAvailabilityZone`.
    pub default_availability_zone: Option<::Value<String>>,
    /// Property `DefaultInstanceProfileArn`.
    pub default_instance_profile_arn: ::Value<String>,
    /// Property `DefaultOs`.
    pub default_os: Option<::Value<String>>,
    /// Property `DefaultRootDeviceType`.
    pub default_root_device_type: Option<::Value<String>>,
    /// Property `DefaultSshKeyName`.
    pub default_ssh_key_name: Option<::Value<String>>,
    /// Property `DefaultSubnetId`.
    pub default_subnet_id: Option<::Value<String>>,
    /// Property `EcsClusterArn`.
    pub ecs_cluster_arn: Option<::Value<String>>,
    /// Property `ElasticIps`.
    pub elastic_ips: Option<::ValueList<self::stack::ElasticIp>>,
    /// Property `HostnameTheme`.
    pub hostname_theme: Option<::Value<String>>,
    /// Property `Name`.
    pub name: ::Value<String>,
    /// Property `RdsDbInstances`.
    pub rds_db_instances: Option<::ValueList<self::stack::RdsDbInstance>>,
    /// Property `ServiceRoleArn`.
    pub service_role_arn: ::Value<String>,
    /// Property `SourceStackId`.
    pub source_stack_id: Option<::Value<String>>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `UseCustomCookbooks`.
    pub use_custom_cookbooks: Option<::Value<bool>>,
    /// Property `UseOpsworksSecurityGroups`.
    pub use_opsworks_security_groups: Option<::Value<bool>>,
    /// Property `VpcId`.
    pub vpc_id: Option<::Value<String>>,
}

impl ::serde::Serialize for StackProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AgentVersion", &self.agent_version)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Attributes", &self.attributes)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChefConfiguration", &self.chef_configuration)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloneAppIds", &self.clone_app_ids)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClonePermissions", &self.clone_permissions)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfigurationManager", &self.configuration_manager)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomCookbooksSource", &self.custom_cookbooks_source)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomJson", &self.custom_json)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultAvailabilityZone", &self.default_availability_zone)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultInstanceProfileArn", &self.default_instance_profile_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultOs", &self.default_os)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultRootDeviceType", &self.default_root_device_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultSshKeyName", &self.default_ssh_key_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultSubnetId", &self.default_subnet_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EcsClusterArn", &self.ecs_cluster_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ElasticIps", &self.elastic_ips)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "HostnameTheme", &self.hostname_theme)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RdsDbInstances", &self.rds_db_instances)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceRoleArn", &self.service_role_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceStackId", &self.source_stack_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "UseCustomCookbooks", &self.use_custom_cookbooks)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "UseOpsworksSecurityGroups", &self.use_opsworks_security_groups)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcId", &self.vpc_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for StackProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<StackProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = StackProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type StackProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut agent_version = None;
                let mut attributes = None;
                let mut chef_configuration = None;
                let mut clone_app_ids = None;
                let mut clone_permissions = None;
                let mut configuration_manager = None;
                let mut custom_cookbooks_source = None;
                let mut custom_json = None;
                let mut default_availability_zone = None;
                let mut default_instance_profile_arn = None;
                let mut default_os = None;
                let mut default_root_device_type = None;
                let mut default_ssh_key_name = None;
                let mut default_subnet_id = None;
                let mut ecs_cluster_arn = None;
                let mut elastic_ips = None;
                let mut hostname_theme = None;
                let mut name = None;
                let mut rds_db_instances = None;
                let mut service_role_arn = None;
                let mut source_stack_id = None;
                let mut tags = None;
                let mut use_custom_cookbooks = None;
                let mut use_opsworks_security_groups = None;
                let mut vpc_id = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AgentVersion" => {
                            agent_version = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Attributes" => {
                            attributes = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ChefConfiguration" => {
                            chef_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "CloneAppIds" => {
                            clone_app_ids = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ClonePermissions" => {
                            clone_permissions = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ConfigurationManager" => {
                            configuration_manager = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "CustomCookbooksSource" => {
                            custom_cookbooks_source = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "CustomJson" => {
                            custom_json = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DefaultAvailabilityZone" => {
                            default_availability_zone = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DefaultInstanceProfileArn" => {
                            default_instance_profile_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DefaultOs" => {
                            default_os = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DefaultRootDeviceType" => {
                            default_root_device_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DefaultSshKeyName" => {
                            default_ssh_key_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DefaultSubnetId" => {
                            default_subnet_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EcsClusterArn" => {
                            ecs_cluster_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ElasticIps" => {
                            elastic_ips = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "HostnameTheme" => {
                            hostname_theme = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RdsDbInstances" => {
                            rds_db_instances = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ServiceRoleArn" => {
                            service_role_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SourceStackId" => {
                            source_stack_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "UseCustomCookbooks" => {
                            use_custom_cookbooks = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "UseOpsworksSecurityGroups" => {
                            use_opsworks_security_groups = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "VpcId" => {
                            vpc_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(StackProperties {
                    agent_version: agent_version,
                    attributes: attributes,
                    chef_configuration: chef_configuration,
                    clone_app_ids: clone_app_ids,
                    clone_permissions: clone_permissions,
                    configuration_manager: configuration_manager,
                    custom_cookbooks_source: custom_cookbooks_source,
                    custom_json: custom_json,
                    default_availability_zone: default_availability_zone,
                    default_instance_profile_arn: default_instance_profile_arn.ok_or(::serde::de::Error::missing_field("DefaultInstanceProfileArn"))?,
                    default_os: default_os,
                    default_root_device_type: default_root_device_type,
                    default_ssh_key_name: default_ssh_key_name,
                    default_subnet_id: default_subnet_id,
                    ecs_cluster_arn: ecs_cluster_arn,
                    elastic_ips: elastic_ips,
                    hostname_theme: hostname_theme,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    rds_db_instances: rds_db_instances,
                    service_role_arn: service_role_arn.ok_or(::serde::de::Error::missing_field("ServiceRoleArn"))?,
                    source_stack_id: source_stack_id,
                    tags: tags,
                    use_custom_cookbooks: use_custom_cookbooks,
                    use_opsworks_security_groups: use_opsworks_security_groups,
                    vpc_id: vpc_id,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct UserProfileProperties {
    /// Property `AllowSelfManagement`.
    pub allow_self_management: Option<::Value<bool>>,
    /// Property `IamUserArn`.
    pub iam_user_arn: ::Value<String>,
    /// Property `SshPublicKey`.
    pub ssh_public_key: Option<::Value<String>>,
    /// Property `SshUsername`.
    pub ssh_username: Option<::Value<String>>,
}

impl ::serde::Serialize for UserProfileProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowSelfManagement", &self.allow_self_management)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IamUserArn", &self.iam_user_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SshPublicKey", &self.ssh_public_key)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SshUsername", &self.ssh_username)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for UserProfileProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<UserProfileProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = UserProfileProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type UserProfileProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut allow_self_management = None;
                let mut iam_user_arn = None;
                let mut ssh_public_key = None;
                let mut ssh_username = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AllowSelfManagement" => {
                            allow_self_management = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "IamUserArn" => {
                            iam_user_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SshPublicKey" => {
                            ssh_public_key = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SshUsername" => {
                            ssh_username = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(UserProfileProperties {
                    allow_self_management: allow_self_management,
                    iam_user_arn: iam_user_arn.ok_or(::serde::de::Error::missing_field("IamUserArn"))?,
                    ssh_public_key: ssh_public_key,
                    ssh_username: ssh_username,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct VolumeProperties {
    /// Property `Ec2VolumeId`.
    pub ec2_volume_id: ::Value<String>,
    /// Property `MountPoint`.
    pub mount_point: Option<::Value<String>>,
    /// Property `Name`.
    pub name: Option<::Value<String>>,
    /// Property `StackId`.
    pub stack_id: ::Value<String>,
}

impl ::serde::Serialize for VolumeProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ec2VolumeId", &self.ec2_volume_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MountPoint", &self.mount_point)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StackId", &self.stack_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for VolumeProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<VolumeProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = VolumeProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type VolumeProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut ec2_volume_id = None;
                let mut mount_point = None;
                let mut name = None;
                let mut stack_id = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Ec2VolumeId" => {
                            ec2_volume_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "MountPoint" => {
                            mount_point = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "StackId" => {
                            stack_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(VolumeProperties {
                    ec2_volume_id: ec2_volume_id.ok_or(::serde::de::Error::missing_field("Ec2VolumeId"))?,
                    mount_point: mount_point,
                    name: name,
                    stack_id: stack_id.ok_or(::serde::de::Error::missing_field("StackId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
    #[derive(Debug)]
    pub struct DataSource {
        /// Property `Arn`.
        pub arn: Option<::Value<String>>,
        /// Property `DatabaseName`.
        pub database_name: Option<::Value<String>>,
        /// Property `Type`.
        pub type_: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DataSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arn", &self.arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", &self.database_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataSource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataSource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataSource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataSource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arn = None;
                    let mut database_name = None;
                    let mut type_ = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Arn" => {
                                arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "DatabaseName" => {
                                database_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Type" => {
                                type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(DataSource {
                        arn: arn,
                        database_name: database_name,
                        type_: type_,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::OpsWorks::App.EnvironmentVariable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-app-environment.html) property type.
    #[derive(Debug)]
    pub struct EnvironmentVariable {
        /// Property `Key`.
        pub key: ::Value<String>,
        /// Property `Secure`.
        pub secure: Option<::Value<bool>>,
        /// Property `Value`.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for EnvironmentVariable {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Secure", &self.secure)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EnvironmentVariable {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EnvironmentVariable, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EnvironmentVariable;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EnvironmentVariable")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key = None;
                    let mut secure = None;
                    let mut value = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Secure" => {
                                secure = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Value" => {
                                value = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(EnvironmentVariable {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        secure: secure,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::OpsWorks::App.Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-source.html) property type.
    #[derive(Debug)]
    pub struct Source {
        /// Property `Password`.
        pub password: Option<::Value<String>>,
        /// Property `Revision`.
        pub revision: Option<::Value<String>>,
        /// Property `SshKey`.
        pub ssh_key: Option<::Value<String>>,
        /// Property `Type`.
        pub type_: Option<::Value<String>>,
        /// Property `Url`.
        pub url: Option<::Value<String>>,
        /// Property `Username`.
        pub username: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Source {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Password", &self.password)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Revision", &self.revision)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SshKey", &self.ssh_key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Url", &self.url)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Username", &self.username)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Source {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Source, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Source;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Source")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut password = None;
                    let mut revision = None;
                    let mut ssh_key = None;
                    let mut type_ = None;
                    let mut url = None;
                    let mut username = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Password" => {
                                password = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Revision" => {
                                revision = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SshKey" => {
                                ssh_key = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Type" => {
                                type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Url" => {
                                url = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Username" => {
                                username = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(Source {
                        password: password,
                        revision: revision,
                        ssh_key: ssh_key,
                        type_: type_,
                        url: url,
                        username: username,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::OpsWorks::App.SslConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-app-sslconfiguration.html) property type.
    #[derive(Debug)]
    pub struct SslConfiguration {
        /// Property `Certificate`.
        pub certificate: Option<::Value<String>>,
        /// Property `Chain`.
        pub chain: Option<::Value<String>>,
        /// Property `PrivateKey`.
        pub private_key: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SslConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Certificate", &self.certificate)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Chain", &self.chain)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrivateKey", &self.private_key)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SslConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SslConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SslConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SslConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut certificate = None;
                    let mut chain = None;
                    let mut private_key = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Certificate" => {
                                certificate = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Chain" => {
                                chain = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "PrivateKey" => {
                                private_key = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(SslConfiguration {
                        certificate: certificate,
                        chain: chain,
                        private_key: private_key,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod instance {
    //! Property types for the `Instance` resource.

    /// The [`AWS::OpsWorks::Instance.BlockDeviceMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-blockdevicemapping.html) property type.
    #[derive(Debug)]
    pub struct BlockDeviceMapping {
        /// Property `DeviceName`.
        pub device_name: Option<::Value<String>>,
        /// Property `Ebs`.
        pub ebs: Option<::Value<EbsBlockDevice>>,
        /// Property `NoDevice`.
        pub no_device: Option<::Value<String>>,
        /// Property `VirtualName`.
        pub virtual_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for BlockDeviceMapping {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceName", &self.device_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ebs", &self.ebs)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NoDevice", &self.no_device)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VirtualName", &self.virtual_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BlockDeviceMapping {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BlockDeviceMapping, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BlockDeviceMapping;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BlockDeviceMapping")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut device_name = None;
                    let mut ebs = None;
                    let mut no_device = None;
                    let mut virtual_name = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DeviceName" => {
                                device_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Ebs" => {
                                ebs = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "NoDevice" => {
                                no_device = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "VirtualName" => {
                                virtual_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(BlockDeviceMapping {
                        device_name: device_name,
                        ebs: ebs,
                        no_device: no_device,
                        virtual_name: virtual_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::OpsWorks::Instance.EbsBlockDevice`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-ebsblockdevice.html) property type.
    #[derive(Debug)]
    pub struct EbsBlockDevice {
        /// Property `DeleteOnTermination`.
        pub delete_on_termination: Option<::Value<bool>>,
        /// Property `Iops`.
        pub iops: Option<::Value<u32>>,
        /// Property `SnapshotId`.
        pub snapshot_id: Option<::Value<String>>,
        /// Property `VolumeSize`.
        pub volume_size: Option<::Value<u32>>,
        /// Property `VolumeType`.
        pub volume_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EbsBlockDevice {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeleteOnTermination", &self.delete_on_termination)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Iops", &self.iops)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotId", &self.snapshot_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeSize", &self.volume_size)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeType", &self.volume_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EbsBlockDevice {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EbsBlockDevice, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EbsBlockDevice;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EbsBlockDevice")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut delete_on_termination = None;
                    let mut iops = None;
                    let mut snapshot_id = None;
                    let mut volume_size = None;
                    let mut volume_type = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DeleteOnTermination" => {
                                delete_on_termination = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Iops" => {
                                iops = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SnapshotId" => {
                                snapshot_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "VolumeSize" => {
                                volume_size = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "VolumeType" => {
                                volume_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(EbsBlockDevice {
                        delete_on_termination: delete_on_termination,
                        iops: iops,
                        snapshot_id: snapshot_id,
                        volume_size: volume_size,
                        volume_type: volume_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::OpsWorks::Instance.TimeBasedAutoScaling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-timebasedautoscaling.html) property type.
    #[derive(Debug)]
    pub struct TimeBasedAutoScaling {
        /// Property `Friday`.
        pub friday: Option<::ValueMap<String>>,
        /// Property `Monday`.
        pub monday: Option<::ValueMap<String>>,
        /// Property `Saturday`.
        pub saturday: Option<::ValueMap<String>>,
        /// Property `Sunday`.
        pub sunday: Option<::ValueMap<String>>,
        /// Property `Thursday`.
        pub thursday: Option<::ValueMap<String>>,
        /// Property `Tuesday`.
        pub tuesday: Option<::ValueMap<String>>,
        /// Property `Wednesday`.
        pub wednesday: Option<::ValueMap<String>>,
    }

    impl ::codec::SerializeValue for TimeBasedAutoScaling {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Friday", &self.friday)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Monday", &self.monday)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Saturday", &self.saturday)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Sunday", &self.sunday)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Thursday", &self.thursday)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tuesday", &self.tuesday)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Wednesday", &self.wednesday)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TimeBasedAutoScaling {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TimeBasedAutoScaling, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TimeBasedAutoScaling;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TimeBasedAutoScaling")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut friday = None;
                    let mut monday = None;
                    let mut saturday = None;
                    let mut sunday = None;
                    let mut thursday = None;
                    let mut tuesday = None;
                    let mut wednesday = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Friday" => {
                                friday = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Monday" => {
                                monday = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Saturday" => {
                                saturday = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Sunday" => {
                                sunday = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Thursday" => {
                                thursday = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Tuesday" => {
                                tuesday = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Wednesday" => {
                                wednesday = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(TimeBasedAutoScaling {
                        friday: friday,
                        monday: monday,
                        saturday: saturday,
                        sunday: sunday,
                        thursday: thursday,
                        tuesday: tuesday,
                        wednesday: wednesday,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod layer {
    //! Property types for the `Layer` resource.

    /// The [`AWS::OpsWorks::Layer.AutoScalingThresholds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-loadbasedautoscaling-autoscalingthresholds.html) property type.
    #[derive(Debug)]
    pub struct AutoScalingThresholds {
        /// Property `CpuThreshold`.
        pub cpu_threshold: Option<::Value<f64>>,
        /// Property `IgnoreMetricsTime`.
        pub ignore_metrics_time: Option<::Value<u32>>,
        /// Property `InstanceCount`.
        pub instance_count: Option<::Value<u32>>,
        /// Property `LoadThreshold`.
        pub load_threshold: Option<::Value<f64>>,
        /// Property `MemoryThreshold`.
        pub memory_threshold: Option<::Value<f64>>,
        /// Property `ThresholdsWaitTime`.
        pub thresholds_wait_time: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for AutoScalingThresholds {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CpuThreshold", &self.cpu_threshold)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IgnoreMetricsTime", &self.ignore_metrics_time)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceCount", &self.instance_count)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoadThreshold", &self.load_threshold)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MemoryThreshold", &self.memory_threshold)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThresholdsWaitTime", &self.thresholds_wait_time)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AutoScalingThresholds {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AutoScalingThresholds, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AutoScalingThresholds;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AutoScalingThresholds")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cpu_threshold = None;
                    let mut ignore_metrics_time = None;
                    let mut instance_count = None;
                    let mut load_threshold = None;
                    let mut memory_threshold = None;
                    let mut thresholds_wait_time = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CpuThreshold" => {
                                cpu_threshold = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "IgnoreMetricsTime" => {
                                ignore_metrics_time = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "InstanceCount" => {
                                instance_count = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "LoadThreshold" => {
                                load_threshold = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "MemoryThreshold" => {
                                memory_threshold = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ThresholdsWaitTime" => {
                                thresholds_wait_time = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(AutoScalingThresholds {
                        cpu_threshold: cpu_threshold,
                        ignore_metrics_time: ignore_metrics_time,
                        instance_count: instance_count,
                        load_threshold: load_threshold,
                        memory_threshold: memory_threshold,
                        thresholds_wait_time: thresholds_wait_time,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::OpsWorks::Layer.LifecycleEventConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-lifecycleeventconfiguration.html) property type.
    #[derive(Debug)]
    pub struct LifecycleEventConfiguration {
        /// Property `ShutdownEventConfiguration`.
        pub shutdown_event_configuration: Option<::Value<ShutdownEventConfiguration>>,
    }

    impl ::codec::SerializeValue for LifecycleEventConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ShutdownEventConfiguration", &self.shutdown_event_configuration)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LifecycleEventConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LifecycleEventConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LifecycleEventConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LifecycleEventConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut shutdown_event_configuration = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ShutdownEventConfiguration" => {
                                shutdown_event_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(LifecycleEventConfiguration {
                        shutdown_event_configuration: shutdown_event_configuration,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::OpsWorks::Layer.LoadBasedAutoScaling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-loadbasedautoscaling.html) property type.
    #[derive(Debug)]
    pub struct LoadBasedAutoScaling {
        /// Property `DownScaling`.
        pub down_scaling: Option<::Value<AutoScalingThresholds>>,
        /// Property `Enable`.
        pub enable: Option<::Value<bool>>,
        /// Property `UpScaling`.
        pub up_scaling: Option<::Value<AutoScalingThresholds>>,
    }

    impl ::codec::SerializeValue for LoadBasedAutoScaling {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DownScaling", &self.down_scaling)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enable", &self.enable)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UpScaling", &self.up_scaling)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LoadBasedAutoScaling {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LoadBasedAutoScaling, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LoadBasedAutoScaling;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LoadBasedAutoScaling")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut down_scaling = None;
                    let mut enable = None;
                    let mut up_scaling = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DownScaling" => {
                                down_scaling = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Enable" => {
                                enable = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "UpScaling" => {
                                up_scaling = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(LoadBasedAutoScaling {
                        down_scaling: down_scaling,
                        enable: enable,
                        up_scaling: up_scaling,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::OpsWorks::Layer.Recipes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-recipes.html) property type.
    #[derive(Debug)]
    pub struct Recipes {
        /// Property `Configure`.
        pub configure: Option<::ValueList<String>>,
        /// Property `Deploy`.
        pub deploy: Option<::ValueList<String>>,
        /// Property `Setup`.
        pub setup: Option<::ValueList<String>>,
        /// Property `Shutdown`.
        pub shutdown: Option<::ValueList<String>>,
        /// Property `Undeploy`.
        pub undeploy: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for Recipes {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Configure", &self.configure)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Deploy", &self.deploy)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Setup", &self.setup)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Shutdown", &self.shutdown)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Undeploy", &self.undeploy)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Recipes {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Recipes, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Recipes;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Recipes")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut configure = None;
                    let mut deploy = None;
                    let mut setup = None;
                    let mut shutdown = None;
                    let mut undeploy = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Configure" => {
                                configure = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Deploy" => {
                                deploy = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Setup" => {
                                setup = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Shutdown" => {
                                shutdown = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Undeploy" => {
                                undeploy = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(Recipes {
                        configure: configure,
                        deploy: deploy,
                        setup: setup,
                        shutdown: shutdown,
                        undeploy: undeploy,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::OpsWorks::Layer.ShutdownEventConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-lifecycleeventconfiguration-shutdowneventconfiguration.html) property type.
    #[derive(Debug)]
    pub struct ShutdownEventConfiguration {
        /// Property `DelayUntilElbConnectionsDrained`.
        pub delay_until_elb_connections_drained: Option<::Value<bool>>,
        /// Property `ExecutionTimeout`.
        pub execution_timeout: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for ShutdownEventConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DelayUntilElbConnectionsDrained", &self.delay_until_elb_connections_drained)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExecutionTimeout", &self.execution_timeout)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ShutdownEventConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ShutdownEventConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ShutdownEventConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ShutdownEventConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut delay_until_elb_connections_drained = None;
                    let mut execution_timeout = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DelayUntilElbConnectionsDrained" => {
                                delay_until_elb_connections_drained = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ExecutionTimeout" => {
                                execution_timeout = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(ShutdownEventConfiguration {
                        delay_until_elb_connections_drained: delay_until_elb_connections_drained,
                        execution_timeout: execution_timeout,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::OpsWorks::Layer.VolumeConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-volumeconfiguration.html) property type.
    #[derive(Debug)]
    pub struct VolumeConfiguration {
        /// Property `Iops`.
        pub iops: Option<::Value<u32>>,
        /// Property `MountPoint`.
        pub mount_point: Option<::Value<String>>,
        /// Property `NumberOfDisks`.
        pub number_of_disks: Option<::Value<u32>>,
        /// Property `RaidLevel`.
        pub raid_level: Option<::Value<u32>>,
        /// Property `Size`.
        pub size: Option<::Value<u32>>,
        /// Property `VolumeType`.
        pub volume_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for VolumeConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Iops", &self.iops)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MountPoint", &self.mount_point)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumberOfDisks", &self.number_of_disks)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RaidLevel", &self.raid_level)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Size", &self.size)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeType", &self.volume_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VolumeConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VolumeConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VolumeConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VolumeConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut iops = None;
                    let mut mount_point = None;
                    let mut number_of_disks = None;
                    let mut raid_level = None;
                    let mut size = None;
                    let mut volume_type = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Iops" => {
                                iops = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "MountPoint" => {
                                mount_point = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "NumberOfDisks" => {
                                number_of_disks = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RaidLevel" => {
                                raid_level = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Size" => {
                                size = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "VolumeType" => {
                                volume_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(VolumeConfiguration {
                        iops: iops,
                        mount_point: mount_point,
                        number_of_disks: number_of_disks,
                        raid_level: raid_level,
                        size: size,
                        volume_type: volume_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod stack {
    //! Property types for the `Stack` resource.

    /// The [`AWS::OpsWorks::Stack.ChefConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-chefconfiguration.html) property type.
    #[derive(Debug)]
    pub struct ChefConfiguration {
        /// Property `BerkshelfVersion`.
        pub berkshelf_version: Option<::Value<String>>,
        /// Property `ManageBerkshelf`.
        pub manage_berkshelf: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for ChefConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BerkshelfVersion", &self.berkshelf_version)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManageBerkshelf", &self.manage_berkshelf)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ChefConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ChefConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ChefConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ChefConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut berkshelf_version = None;
                    let mut manage_berkshelf = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BerkshelfVersion" => {
                                berkshelf_version = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ManageBerkshelf" => {
                                manage_berkshelf = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(ChefConfiguration {
                        berkshelf_version: berkshelf_version,
                        manage_berkshelf: manage_berkshelf,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::OpsWorks::Stack.ElasticIp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-elasticip.html) property type.
    #[derive(Debug)]
    pub struct ElasticIp {
        /// Property `Ip`.
        pub ip: ::Value<String>,
        /// Property `Name`.
        pub name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ElasticIp {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ip", &self.ip)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ElasticIp {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ElasticIp, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ElasticIp;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ElasticIp")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ip = None;
                    let mut name = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Ip" => {
                                ip = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Name" => {
                                name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(ElasticIp {
                        ip: ip.ok_or(::serde::de::Error::missing_field("Ip"))?,
                        name: name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::OpsWorks::Stack.RdsDbInstance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-rdsdbinstance.html) property type.
    #[derive(Debug)]
    pub struct RdsDbInstance {
        /// Property `DbPassword`.
        pub db_password: ::Value<String>,
        /// Property `DbUser`.
        pub db_user: ::Value<String>,
        /// Property `RdsDbInstanceArn`.
        pub rds_db_instance_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for RdsDbInstance {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DbPassword", &self.db_password)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DbUser", &self.db_user)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RdsDbInstanceArn", &self.rds_db_instance_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RdsDbInstance {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RdsDbInstance, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RdsDbInstance;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RdsDbInstance")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut db_password = None;
                    let mut db_user = None;
                    let mut rds_db_instance_arn = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DbPassword" => {
                                db_password = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "DbUser" => {
                                db_user = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RdsDbInstanceArn" => {
                                rds_db_instance_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(RdsDbInstance {
                        db_password: db_password.ok_or(::serde::de::Error::missing_field("DbPassword"))?,
                        db_user: db_user.ok_or(::serde::de::Error::missing_field("DbUser"))?,
                        rds_db_instance_arn: rds_db_instance_arn.ok_or(::serde::de::Error::missing_field("RdsDbInstanceArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::OpsWorks::Stack.Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-source.html) property type.
    #[derive(Debug)]
    pub struct Source {
        /// Property `Password`.
        pub password: Option<::Value<String>>,
        /// Property `Revision`.
        pub revision: Option<::Value<String>>,
        /// Property `SshKey`.
        pub ssh_key: Option<::Value<String>>,
        /// Property `Type`.
        pub type_: Option<::Value<String>>,
        /// Property `Url`.
        pub url: Option<::Value<String>>,
        /// Property `Username`.
        pub username: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Source {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Password", &self.password)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Revision", &self.revision)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SshKey", &self.ssh_key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Url", &self.url)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Username", &self.username)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Source {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Source, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Source;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Source")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut password = None;
                    let mut revision = None;
                    let mut ssh_key = None;
                    let mut type_ = None;
                    let mut url = None;
                    let mut username = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Password" => {
                                password = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Revision" => {
                                revision = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SshKey" => {
                                ssh_key = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Type" => {
                                type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Url" => {
                                url = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Username" => {
                                username = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(Source {
                        password: password,
                        revision: revision,
                        ssh_key: ssh_key,
                        type_: type_,
                        url: url,
                        username: username,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::OpsWorks::Stack.StackConfigurationManager`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-stackconfigmanager.html) property type.
    #[derive(Debug)]
    pub struct StackConfigurationManager {
        /// Property `Name`.
        pub name: Option<::Value<String>>,
        /// Property `Version`.
        pub version: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for StackConfigurationManager {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", &self.version)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StackConfigurationManager {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StackConfigurationManager, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StackConfigurationManager;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StackConfigurationManager")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name = None;
                    let mut version = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Version" => {
                                version = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(StackConfigurationManager {
                        name: name,
                        version: version,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
