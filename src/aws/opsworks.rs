//! Types for the `OpsWorks` service.

/// The [`AWS::OpsWorks::App`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-app.html) resource type.
#[derive(Debug, Default)]
pub struct App {
    properties: AppProperties
}

/// Properties for the `App` resource.
#[derive(Debug, Default)]
pub struct AppProperties {
    /// Property [`AppSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-app.html#cfn-opsworks-app-appsource).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub app_source: Option<::Value<self::app::Source>>,
    /// Property [`Attributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-app.html#cfn-opsworks-app-attributes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub attributes: Option<::ValueMap<String>>,
    /// Property [`DataSources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-app.html#cfn-opsworks-app-datasources).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub data_sources: Option<::ValueList<self::app::DataSource>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-app.html#cfn-opsworks-app-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Domains`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-app.html#cfn-opsworks-app-domains).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub domains: Option<::ValueList<String>>,
    /// Property [`EnableSsl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-app.html#cfn-opsworks-app-enablessl).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enable_ssl: Option<::Value<bool>>,
    /// Property [`Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-app.html#cfn-opsworks-app-environment).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub environment: Option<::ValueList<self::app::EnvironmentVariable>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-app.html#cfn-opsworks-app-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Shortname`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-app.html#cfn-opsworks-app-shortname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub shortname: Option<::Value<String>>,
    /// Property [`SslConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-app.html#cfn-opsworks-app-sslconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ssl_configuration: Option<::Value<self::app::SslConfiguration>>,
    /// Property [`StackId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-app.html#cfn-opsworks-app-stackid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub stack_id: ::Value<String>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-app.html#cfn-opsworks-app-type).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub type_: ::Value<String>,
}

impl ::serde::Serialize for AppProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref app_source) = self.app_source {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AppSource", app_source)?;
        }
        if let Some(ref attributes) = self.attributes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Attributes", attributes)?;
        }
        if let Some(ref data_sources) = self.data_sources {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataSources", data_sources)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref domains) = self.domains {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Domains", domains)?;
        }
        if let Some(ref enable_ssl) = self.enable_ssl {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableSsl", enable_ssl)?;
        }
        if let Some(ref environment) = self.environment {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Environment", environment)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref shortname) = self.shortname {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Shortname", shortname)?;
        }
        if let Some(ref ssl_configuration) = self.ssl_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SslConfiguration", ssl_configuration)?;
        }
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
                let mut app_source: Option<::Value<self::app::Source>> = None;
                let mut attributes: Option<::ValueMap<String>> = None;
                let mut data_sources: Option<::ValueList<self::app::DataSource>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut domains: Option<::ValueList<String>> = None;
                let mut enable_ssl: Option<::Value<bool>> = None;
                let mut environment: Option<::ValueList<self::app::EnvironmentVariable>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut shortname: Option<::Value<String>> = None;
                let mut ssl_configuration: Option<::Value<self::app::SslConfiguration>> = None;
                let mut stack_id: Option<::Value<String>> = None;
                let mut type_: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AppSource" => {
                            app_source = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Attributes" => {
                            attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DataSources" => {
                            data_sources = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Domains" => {
                            domains = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnableSsl" => {
                            enable_ssl = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Environment" => {
                            environment = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Shortname" => {
                            shortname = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SslConfiguration" => {
                            ssl_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StackId" => {
                            stack_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Type" => {
                            type_ = ::serde::de::MapAccess::next_value(&mut map)?;
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

impl ::Resource for App {
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
#[derive(Debug, Default)]
pub struct ElasticLoadBalancerAttachment {
    properties: ElasticLoadBalancerAttachmentProperties
}

/// Properties for the `ElasticLoadBalancerAttachment` resource.
#[derive(Debug, Default)]
pub struct ElasticLoadBalancerAttachmentProperties {
    /// Property [`ElasticLoadBalancerName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-elbattachment.html#cfn-opsworks-elbattachment-elbname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub elastic_load_balancer_name: ::Value<String>,
    /// Property [`LayerId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-elbattachment.html#cfn-opsworks-elbattachment-layerid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
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
                let mut elastic_load_balancer_name: Option<::Value<String>> = None;
                let mut layer_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ElasticLoadBalancerName" => {
                            elastic_load_balancer_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LayerId" => {
                            layer_id = ::serde::de::MapAccess::next_value(&mut map)?;
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

impl ::Resource for ElasticLoadBalancerAttachment {
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
#[derive(Debug, Default)]
pub struct Instance {
    properties: InstanceProperties
}

/// Properties for the `Instance` resource.
#[derive(Debug, Default)]
pub struct InstanceProperties {
    /// Property [`AgentVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-instance.html#cfn-opsworks-instance-agentversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub agent_version: Option<::Value<String>>,
    /// Property [`AmiId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-instance.html#cfn-opsworks-instance-amiid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ami_id: Option<::Value<String>>,
    /// Property [`Architecture`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-instance.html#cfn-opsworks-instance-architecture).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub architecture: Option<::Value<String>>,
    /// Property [`AutoScalingType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-instance.html#cfn-opsworks-instance-autoscalingtype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub auto_scaling_type: Option<::Value<String>>,
    /// Property [`AvailabilityZone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-instance.html#cfn-opsworks-instance-availabilityzone).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub availability_zone: Option<::Value<String>>,
    /// Property [`BlockDeviceMappings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-instance.html#cfn-opsworks-instance-blockdevicemappings).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub block_device_mappings: Option<::ValueList<self::instance::BlockDeviceMapping>>,
    /// Property [`EbsOptimized`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-instance.html#cfn-opsworks-instance-ebsoptimized).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub ebs_optimized: Option<::Value<bool>>,
    /// Property [`ElasticIps`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-instance.html#cfn-opsworks-instance-elasticips).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub elastic_ips: Option<::ValueList<String>>,
    /// Property [`Hostname`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-instance.html#cfn-opsworks-instance-hostname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub hostname: Option<::Value<String>>,
    /// Property [`InstallUpdatesOnBoot`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-instance.html#cfn-opsworks-instance-installupdatesonboot).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub install_updates_on_boot: Option<::Value<bool>>,
    /// Property [`InstanceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-instance.html#cfn-opsworks-instance-instancetype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_type: ::Value<String>,
    /// Property [`LayerIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-instance.html#cfn-opsworks-instance-layerids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub layer_ids: ::ValueList<String>,
    /// Property [`Os`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-instance.html#cfn-opsworks-instance-os).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub os: Option<::Value<String>>,
    /// Property [`RootDeviceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-instance.html#cfn-opsworks-instance-rootdevicetype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub root_device_type: Option<::Value<String>>,
    /// Property [`SshKeyName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-instance.html#cfn-opsworks-instance-sshkeyname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ssh_key_name: Option<::Value<String>>,
    /// Property [`StackId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-instance.html#cfn-opsworks-instance-stackid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub stack_id: ::Value<String>,
    /// Property [`SubnetId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-instance.html#cfn-opsworks-instance-subnetid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub subnet_id: Option<::Value<String>>,
    /// Property [`Tenancy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-instance.html#cfn-opsworks-instance-tenancy).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tenancy: Option<::Value<String>>,
    /// Property [`TimeBasedAutoScaling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-instance.html#cfn-opsworks-instance-timebasedautoscaling).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub time_based_auto_scaling: Option<::Value<self::instance::TimeBasedAutoScaling>>,
    /// Property [`VirtualizationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-instance.html#cfn-opsworks-instance-virtualizationtype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub virtualization_type: Option<::Value<String>>,
    /// Property [`Volumes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-instance.html#cfn-opsworks-instance-volumes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub volumes: Option<::ValueList<String>>,
}

impl ::serde::Serialize for InstanceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref agent_version) = self.agent_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AgentVersion", agent_version)?;
        }
        if let Some(ref ami_id) = self.ami_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AmiId", ami_id)?;
        }
        if let Some(ref architecture) = self.architecture {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Architecture", architecture)?;
        }
        if let Some(ref auto_scaling_type) = self.auto_scaling_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoScalingType", auto_scaling_type)?;
        }
        if let Some(ref availability_zone) = self.availability_zone {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZone", availability_zone)?;
        }
        if let Some(ref block_device_mappings) = self.block_device_mappings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BlockDeviceMappings", block_device_mappings)?;
        }
        if let Some(ref ebs_optimized) = self.ebs_optimized {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EbsOptimized", ebs_optimized)?;
        }
        if let Some(ref elastic_ips) = self.elastic_ips {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ElasticIps", elastic_ips)?;
        }
        if let Some(ref hostname) = self.hostname {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Hostname", hostname)?;
        }
        if let Some(ref install_updates_on_boot) = self.install_updates_on_boot {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstallUpdatesOnBoot", install_updates_on_boot)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceType", &self.instance_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LayerIds", &self.layer_ids)?;
        if let Some(ref os) = self.os {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Os", os)?;
        }
        if let Some(ref root_device_type) = self.root_device_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RootDeviceType", root_device_type)?;
        }
        if let Some(ref ssh_key_name) = self.ssh_key_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SshKeyName", ssh_key_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StackId", &self.stack_id)?;
        if let Some(ref subnet_id) = self.subnet_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetId", subnet_id)?;
        }
        if let Some(ref tenancy) = self.tenancy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tenancy", tenancy)?;
        }
        if let Some(ref time_based_auto_scaling) = self.time_based_auto_scaling {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeBasedAutoScaling", time_based_auto_scaling)?;
        }
        if let Some(ref virtualization_type) = self.virtualization_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VirtualizationType", virtualization_type)?;
        }
        if let Some(ref volumes) = self.volumes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Volumes", volumes)?;
        }
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
                let mut agent_version: Option<::Value<String>> = None;
                let mut ami_id: Option<::Value<String>> = None;
                let mut architecture: Option<::Value<String>> = None;
                let mut auto_scaling_type: Option<::Value<String>> = None;
                let mut availability_zone: Option<::Value<String>> = None;
                let mut block_device_mappings: Option<::ValueList<self::instance::BlockDeviceMapping>> = None;
                let mut ebs_optimized: Option<::Value<bool>> = None;
                let mut elastic_ips: Option<::ValueList<String>> = None;
                let mut hostname: Option<::Value<String>> = None;
                let mut install_updates_on_boot: Option<::Value<bool>> = None;
                let mut instance_type: Option<::Value<String>> = None;
                let mut layer_ids: Option<::ValueList<String>> = None;
                let mut os: Option<::Value<String>> = None;
                let mut root_device_type: Option<::Value<String>> = None;
                let mut ssh_key_name: Option<::Value<String>> = None;
                let mut stack_id: Option<::Value<String>> = None;
                let mut subnet_id: Option<::Value<String>> = None;
                let mut tenancy: Option<::Value<String>> = None;
                let mut time_based_auto_scaling: Option<::Value<self::instance::TimeBasedAutoScaling>> = None;
                let mut virtualization_type: Option<::Value<String>> = None;
                let mut volumes: Option<::ValueList<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AgentVersion" => {
                            agent_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AmiId" => {
                            ami_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Architecture" => {
                            architecture = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AutoScalingType" => {
                            auto_scaling_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AvailabilityZone" => {
                            availability_zone = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BlockDeviceMappings" => {
                            block_device_mappings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EbsOptimized" => {
                            ebs_optimized = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ElasticIps" => {
                            elastic_ips = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Hostname" => {
                            hostname = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstallUpdatesOnBoot" => {
                            install_updates_on_boot = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceType" => {
                            instance_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LayerIds" => {
                            layer_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Os" => {
                            os = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RootDeviceType" => {
                            root_device_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SshKeyName" => {
                            ssh_key_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StackId" => {
                            stack_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SubnetId" => {
                            subnet_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tenancy" => {
                            tenancy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TimeBasedAutoScaling" => {
                            time_based_auto_scaling = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VirtualizationType" => {
                            virtualization_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Volumes" => {
                            volumes = ::serde::de::MapAccess::next_value(&mut map)?;
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

impl ::Resource for Instance {
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
#[derive(Debug, Default)]
pub struct Layer {
    properties: LayerProperties
}

/// Properties for the `Layer` resource.
#[derive(Debug, Default)]
pub struct LayerProperties {
    /// Property [`Attributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-layer.html#cfn-opsworks-layer-attributes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub attributes: Option<::ValueMap<String>>,
    /// Property [`AutoAssignElasticIps`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-layer.html#cfn-opsworks-layer-autoassignelasticips).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub auto_assign_elastic_ips: ::Value<bool>,
    /// Property [`AutoAssignPublicIps`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-layer.html#cfn-opsworks-layer-autoassignpublicips).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub auto_assign_public_ips: ::Value<bool>,
    /// Property [`CustomInstanceProfileArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-layer.html#cfn-opsworks-layer-custominstanceprofilearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub custom_instance_profile_arn: Option<::Value<String>>,
    /// Property [`CustomJson`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-layer.html#cfn-opsworks-layer-customjson).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub custom_json: Option<::Value<::json::Value>>,
    /// Property [`CustomRecipes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-layer.html#cfn-opsworks-layer-customrecipes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub custom_recipes: Option<::Value<self::layer::Recipes>>,
    /// Property [`CustomSecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-layer.html#cfn-opsworks-layer-customsecuritygroupids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub custom_security_group_ids: Option<::ValueList<String>>,
    /// Property [`EnableAutoHealing`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-layer.html#cfn-opsworks-layer-enableautohealing).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enable_auto_healing: ::Value<bool>,
    /// Property [`InstallUpdatesOnBoot`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-layer.html#cfn-opsworks-layer-installupdatesonboot).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub install_updates_on_boot: Option<::Value<bool>>,
    /// Property [`LifecycleEventConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-layer.html#cfn-opsworks-layer-lifecycleeventconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub lifecycle_event_configuration: Option<::Value<self::layer::LifecycleEventConfiguration>>,
    /// Property [`LoadBasedAutoScaling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-layer.html#cfn-opsworks-layer-loadbasedautoscaling).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub load_based_auto_scaling: Option<::Value<self::layer::LoadBasedAutoScaling>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-layer.html#cfn-opsworks-layer-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Packages`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-layer.html#cfn-opsworks-layer-packages).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub packages: Option<::ValueList<String>>,
    /// Property [`Shortname`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-layer.html#cfn-opsworks-layer-shortname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub shortname: ::Value<String>,
    /// Property [`StackId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-layer.html#cfn-opsworks-layer-stackid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub stack_id: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-layer.html#cfn-opsworks-layer-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-layer.html#cfn-opsworks-layer-type).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub type_: ::Value<String>,
    /// Property [`UseEbsOptimizedInstances`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-layer.html#cfn-opsworks-layer-useebsoptimizedinstances).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub use_ebs_optimized_instances: Option<::Value<bool>>,
    /// Property [`VolumeConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-layer.html#cfn-opsworks-layer-volumeconfigurations).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub volume_configurations: Option<::ValueList<self::layer::VolumeConfiguration>>,
}

impl ::serde::Serialize for LayerProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref attributes) = self.attributes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Attributes", attributes)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoAssignElasticIps", &self.auto_assign_elastic_ips)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoAssignPublicIps", &self.auto_assign_public_ips)?;
        if let Some(ref custom_instance_profile_arn) = self.custom_instance_profile_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomInstanceProfileArn", custom_instance_profile_arn)?;
        }
        if let Some(ref custom_json) = self.custom_json {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomJson", custom_json)?;
        }
        if let Some(ref custom_recipes) = self.custom_recipes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomRecipes", custom_recipes)?;
        }
        if let Some(ref custom_security_group_ids) = self.custom_security_group_ids {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomSecurityGroupIds", custom_security_group_ids)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableAutoHealing", &self.enable_auto_healing)?;
        if let Some(ref install_updates_on_boot) = self.install_updates_on_boot {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstallUpdatesOnBoot", install_updates_on_boot)?;
        }
        if let Some(ref lifecycle_event_configuration) = self.lifecycle_event_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LifecycleEventConfiguration", lifecycle_event_configuration)?;
        }
        if let Some(ref load_based_auto_scaling) = self.load_based_auto_scaling {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoadBasedAutoScaling", load_based_auto_scaling)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref packages) = self.packages {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Packages", packages)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Shortname", &self.shortname)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StackId", &self.stack_id)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
        if let Some(ref use_ebs_optimized_instances) = self.use_ebs_optimized_instances {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UseEbsOptimizedInstances", use_ebs_optimized_instances)?;
        }
        if let Some(ref volume_configurations) = self.volume_configurations {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeConfigurations", volume_configurations)?;
        }
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
                let mut attributes: Option<::ValueMap<String>> = None;
                let mut auto_assign_elastic_ips: Option<::Value<bool>> = None;
                let mut auto_assign_public_ips: Option<::Value<bool>> = None;
                let mut custom_instance_profile_arn: Option<::Value<String>> = None;
                let mut custom_json: Option<::Value<::json::Value>> = None;
                let mut custom_recipes: Option<::Value<self::layer::Recipes>> = None;
                let mut custom_security_group_ids: Option<::ValueList<String>> = None;
                let mut enable_auto_healing: Option<::Value<bool>> = None;
                let mut install_updates_on_boot: Option<::Value<bool>> = None;
                let mut lifecycle_event_configuration: Option<::Value<self::layer::LifecycleEventConfiguration>> = None;
                let mut load_based_auto_scaling: Option<::Value<self::layer::LoadBasedAutoScaling>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut packages: Option<::ValueList<String>> = None;
                let mut shortname: Option<::Value<String>> = None;
                let mut stack_id: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut type_: Option<::Value<String>> = None;
                let mut use_ebs_optimized_instances: Option<::Value<bool>> = None;
                let mut volume_configurations: Option<::ValueList<self::layer::VolumeConfiguration>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Attributes" => {
                            attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AutoAssignElasticIps" => {
                            auto_assign_elastic_ips = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AutoAssignPublicIps" => {
                            auto_assign_public_ips = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CustomInstanceProfileArn" => {
                            custom_instance_profile_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CustomJson" => {
                            custom_json = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CustomRecipes" => {
                            custom_recipes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CustomSecurityGroupIds" => {
                            custom_security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnableAutoHealing" => {
                            enable_auto_healing = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstallUpdatesOnBoot" => {
                            install_updates_on_boot = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LifecycleEventConfiguration" => {
                            lifecycle_event_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LoadBasedAutoScaling" => {
                            load_based_auto_scaling = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Packages" => {
                            packages = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Shortname" => {
                            shortname = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StackId" => {
                            stack_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Type" => {
                            type_ = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UseEbsOptimizedInstances" => {
                            use_ebs_optimized_instances = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VolumeConfigurations" => {
                            volume_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
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

impl ::Resource for Layer {
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
#[derive(Debug, Default)]
pub struct Stack {
    properties: StackProperties
}

/// Properties for the `Stack` resource.
#[derive(Debug, Default)]
pub struct StackProperties {
    /// Property [`AgentVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-stack.html#cfn-opsworks-stack-agentversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub agent_version: Option<::Value<String>>,
    /// Property [`Attributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-stack.html#cfn-opsworks-stack-attributes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub attributes: Option<::ValueMap<String>>,
    /// Property [`ChefConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-stack.html#cfn-opsworks-stack-chefconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub chef_configuration: Option<::Value<self::stack::ChefConfiguration>>,
    /// Property [`CloneAppIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-stack.html#cfn-opsworks-stack-cloneappids).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub clone_app_ids: Option<::ValueList<String>>,
    /// Property [`ClonePermissions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-stack.html#cfn-opsworks-stack-clonepermissions).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub clone_permissions: Option<::Value<bool>>,
    /// Property [`ConfigurationManager`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-stack.html#cfn-opsworks-stack-configmanager).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub configuration_manager: Option<::Value<self::stack::StackConfigurationManager>>,
    /// Property [`CustomCookbooksSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-stack.html#cfn-opsworks-stack-custcookbooksource).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub custom_cookbooks_source: Option<::Value<self::stack::Source>>,
    /// Property [`CustomJson`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-stack.html#cfn-opsworks-stack-custjson).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub custom_json: Option<::Value<::json::Value>>,
    /// Property [`DefaultAvailabilityZone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-stack.html#cfn-opsworks-stack-defaultaz).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub default_availability_zone: Option<::Value<String>>,
    /// Property [`DefaultInstanceProfileArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-stack.html#cfn-opsworks-stack-defaultinstanceprof).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub default_instance_profile_arn: ::Value<String>,
    /// Property [`DefaultOs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-stack.html#cfn-opsworks-stack-defaultos).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub default_os: Option<::Value<String>>,
    /// Property [`DefaultRootDeviceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-stack.html#cfn-opsworks-stack-defaultrootdevicetype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub default_root_device_type: Option<::Value<String>>,
    /// Property [`DefaultSshKeyName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-stack.html#cfn-opsworks-stack-defaultsshkeyname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub default_ssh_key_name: Option<::Value<String>>,
    /// Property [`DefaultSubnetId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-stack.html#defaultsubnet).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub default_subnet_id: Option<::Value<String>>,
    /// Property [`EcsClusterArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-stack.html#cfn-opsworks-stack-ecsclusterarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ecs_cluster_arn: Option<::Value<String>>,
    /// Property [`ElasticIps`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-stack.html#cfn-opsworks-stack-elasticips).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub elastic_ips: Option<::ValueList<self::stack::ElasticIp>>,
    /// Property [`HostnameTheme`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-stack.html#cfn-opsworks-stack-hostnametheme).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub hostname_theme: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-stack.html#cfn-opsworks-stack-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`RdsDbInstances`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-stack.html#cfn-opsworks-stack-rdsdbinstances).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub rds_db_instances: Option<::ValueList<self::stack::RdsDbInstance>>,
    /// Property [`ServiceRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-stack.html#cfn-opsworks-stack-servicerolearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub service_role_arn: ::Value<String>,
    /// Property [`SourceStackId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-stack.html#cfn-opsworks-stack-sourcestackid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub source_stack_id: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-stack.html#cfn-opsworks-stack-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`UseCustomCookbooks`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-stack.html#usecustcookbooks).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub use_custom_cookbooks: Option<::Value<bool>>,
    /// Property [`UseOpsworksSecurityGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-stack.html#cfn-opsworks-stack-useopsworkssecuritygroups).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub use_opsworks_security_groups: Option<::Value<bool>>,
    /// Property [`VpcId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-stack.html#cfn-opsworks-stack-vpcid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub vpc_id: Option<::Value<String>>,
}

impl ::serde::Serialize for StackProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref agent_version) = self.agent_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AgentVersion", agent_version)?;
        }
        if let Some(ref attributes) = self.attributes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Attributes", attributes)?;
        }
        if let Some(ref chef_configuration) = self.chef_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChefConfiguration", chef_configuration)?;
        }
        if let Some(ref clone_app_ids) = self.clone_app_ids {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloneAppIds", clone_app_ids)?;
        }
        if let Some(ref clone_permissions) = self.clone_permissions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClonePermissions", clone_permissions)?;
        }
        if let Some(ref configuration_manager) = self.configuration_manager {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfigurationManager", configuration_manager)?;
        }
        if let Some(ref custom_cookbooks_source) = self.custom_cookbooks_source {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomCookbooksSource", custom_cookbooks_source)?;
        }
        if let Some(ref custom_json) = self.custom_json {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomJson", custom_json)?;
        }
        if let Some(ref default_availability_zone) = self.default_availability_zone {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultAvailabilityZone", default_availability_zone)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultInstanceProfileArn", &self.default_instance_profile_arn)?;
        if let Some(ref default_os) = self.default_os {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultOs", default_os)?;
        }
        if let Some(ref default_root_device_type) = self.default_root_device_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultRootDeviceType", default_root_device_type)?;
        }
        if let Some(ref default_ssh_key_name) = self.default_ssh_key_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultSshKeyName", default_ssh_key_name)?;
        }
        if let Some(ref default_subnet_id) = self.default_subnet_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultSubnetId", default_subnet_id)?;
        }
        if let Some(ref ecs_cluster_arn) = self.ecs_cluster_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EcsClusterArn", ecs_cluster_arn)?;
        }
        if let Some(ref elastic_ips) = self.elastic_ips {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ElasticIps", elastic_ips)?;
        }
        if let Some(ref hostname_theme) = self.hostname_theme {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HostnameTheme", hostname_theme)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref rds_db_instances) = self.rds_db_instances {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RdsDbInstances", rds_db_instances)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceRoleArn", &self.service_role_arn)?;
        if let Some(ref source_stack_id) = self.source_stack_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceStackId", source_stack_id)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref use_custom_cookbooks) = self.use_custom_cookbooks {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UseCustomCookbooks", use_custom_cookbooks)?;
        }
        if let Some(ref use_opsworks_security_groups) = self.use_opsworks_security_groups {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UseOpsworksSecurityGroups", use_opsworks_security_groups)?;
        }
        if let Some(ref vpc_id) = self.vpc_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcId", vpc_id)?;
        }
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
                let mut agent_version: Option<::Value<String>> = None;
                let mut attributes: Option<::ValueMap<String>> = None;
                let mut chef_configuration: Option<::Value<self::stack::ChefConfiguration>> = None;
                let mut clone_app_ids: Option<::ValueList<String>> = None;
                let mut clone_permissions: Option<::Value<bool>> = None;
                let mut configuration_manager: Option<::Value<self::stack::StackConfigurationManager>> = None;
                let mut custom_cookbooks_source: Option<::Value<self::stack::Source>> = None;
                let mut custom_json: Option<::Value<::json::Value>> = None;
                let mut default_availability_zone: Option<::Value<String>> = None;
                let mut default_instance_profile_arn: Option<::Value<String>> = None;
                let mut default_os: Option<::Value<String>> = None;
                let mut default_root_device_type: Option<::Value<String>> = None;
                let mut default_ssh_key_name: Option<::Value<String>> = None;
                let mut default_subnet_id: Option<::Value<String>> = None;
                let mut ecs_cluster_arn: Option<::Value<String>> = None;
                let mut elastic_ips: Option<::ValueList<self::stack::ElasticIp>> = None;
                let mut hostname_theme: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut rds_db_instances: Option<::ValueList<self::stack::RdsDbInstance>> = None;
                let mut service_role_arn: Option<::Value<String>> = None;
                let mut source_stack_id: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut use_custom_cookbooks: Option<::Value<bool>> = None;
                let mut use_opsworks_security_groups: Option<::Value<bool>> = None;
                let mut vpc_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AgentVersion" => {
                            agent_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Attributes" => {
                            attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ChefConfiguration" => {
                            chef_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CloneAppIds" => {
                            clone_app_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClonePermissions" => {
                            clone_permissions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ConfigurationManager" => {
                            configuration_manager = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CustomCookbooksSource" => {
                            custom_cookbooks_source = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CustomJson" => {
                            custom_json = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DefaultAvailabilityZone" => {
                            default_availability_zone = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DefaultInstanceProfileArn" => {
                            default_instance_profile_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DefaultOs" => {
                            default_os = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DefaultRootDeviceType" => {
                            default_root_device_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DefaultSshKeyName" => {
                            default_ssh_key_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DefaultSubnetId" => {
                            default_subnet_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EcsClusterArn" => {
                            ecs_cluster_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ElasticIps" => {
                            elastic_ips = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HostnameTheme" => {
                            hostname_theme = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RdsDbInstances" => {
                            rds_db_instances = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServiceRoleArn" => {
                            service_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceStackId" => {
                            source_stack_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UseCustomCookbooks" => {
                            use_custom_cookbooks = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UseOpsworksSecurityGroups" => {
                            use_opsworks_security_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcId" => {
                            vpc_id = ::serde::de::MapAccess::next_value(&mut map)?;
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

impl ::Resource for Stack {
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
#[derive(Debug, Default)]
pub struct UserProfile {
    properties: UserProfileProperties
}

/// Properties for the `UserProfile` resource.
#[derive(Debug, Default)]
pub struct UserProfileProperties {
    /// Property [`AllowSelfManagement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-userprofile.html#cfn-opsworks-userprofile-allowselfmanagement).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub allow_self_management: Option<::Value<bool>>,
    /// Property [`IamUserArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-userprofile.html#cfn-opsworks-userprofile-iamuserarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub iam_user_arn: ::Value<String>,
    /// Property [`SshPublicKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-userprofile.html#cfn-opsworks-userprofile-sshpublickey).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ssh_public_key: Option<::Value<String>>,
    /// Property [`SshUsername`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-userprofile.html#cfn-opsworks-userprofile-sshusername).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ssh_username: Option<::Value<String>>,
}

impl ::serde::Serialize for UserProfileProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref allow_self_management) = self.allow_self_management {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowSelfManagement", allow_self_management)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IamUserArn", &self.iam_user_arn)?;
        if let Some(ref ssh_public_key) = self.ssh_public_key {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SshPublicKey", ssh_public_key)?;
        }
        if let Some(ref ssh_username) = self.ssh_username {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SshUsername", ssh_username)?;
        }
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
                let mut allow_self_management: Option<::Value<bool>> = None;
                let mut iam_user_arn: Option<::Value<String>> = None;
                let mut ssh_public_key: Option<::Value<String>> = None;
                let mut ssh_username: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AllowSelfManagement" => {
                            allow_self_management = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IamUserArn" => {
                            iam_user_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SshPublicKey" => {
                            ssh_public_key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SshUsername" => {
                            ssh_username = ::serde::de::MapAccess::next_value(&mut map)?;
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

impl ::Resource for UserProfile {
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
#[derive(Debug, Default)]
pub struct Volume {
    properties: VolumeProperties
}

/// Properties for the `Volume` resource.
#[derive(Debug, Default)]
pub struct VolumeProperties {
    /// Property [`Ec2VolumeId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-volume.html#cfn-opsworks-volume-ec2volumeid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub ec2_volume_id: ::Value<String>,
    /// Property [`MountPoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-volume.html#cfn-opsworks-volume-mountpoint).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub mount_point: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-volume.html#cfn-opsworks-volume-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`StackId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-volume.html#cfn-opsworks-volume-stackid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub stack_id: ::Value<String>,
}

impl ::serde::Serialize for VolumeProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ec2VolumeId", &self.ec2_volume_id)?;
        if let Some(ref mount_point) = self.mount_point {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MountPoint", mount_point)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
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
                let mut ec2_volume_id: Option<::Value<String>> = None;
                let mut mount_point: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut stack_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Ec2VolumeId" => {
                            ec2_volume_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MountPoint" => {
                            mount_point = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StackId" => {
                            stack_id = ::serde::de::MapAccess::next_value(&mut map)?;
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

impl ::Resource for Volume {
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
    #[derive(Debug, Default)]
    pub struct DataSource {
        /// Property [`Arn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-app-datasource.html#cfn-opsworks-app-datasource-arn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub arn: Option<::Value<String>>,
        /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-app-datasource.html#cfn-opsworks-app-datasource-databasename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_name: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-app-datasource.html#cfn-opsworks-app-datasource-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub type_: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DataSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref arn) = self.arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arn", arn)?;
            }
            if let Some(ref database_name) = self.database_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", database_name)?;
            }
            if let Some(ref type_) = self.type_ {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", type_)?;
            }
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
                    let mut arn: Option<::Value<String>> = None;
                    let mut database_name: Option<::Value<String>> = None;
                    let mut type_: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Arn" => {
                                arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabaseName" => {
                                database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                type_ = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct EnvironmentVariable {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-app-environment.html#cfn-opsworks-app-environment-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Secure`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-app-environment.html#cfn-opsworks-app-environment-secure).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secure: Option<::Value<bool>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-app-environment.html#value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for EnvironmentVariable {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            if let Some(ref secure) = self.secure {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Secure", secure)?;
            }
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
                    let mut key: Option<::Value<String>> = None;
                    let mut secure: Option<::Value<bool>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Secure" => {
                                secure = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct Source {
        /// Property [`Password`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-source.html#cfn-opsworks-custcookbooksource-pw).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub password: Option<::Value<String>>,
        /// Property [`Revision`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-source.html#cfn-opsworks-custcookbooksource-revision).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub revision: Option<::Value<String>>,
        /// Property [`SshKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-source.html#cfn-opsworks-custcookbooksource-sshkey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ssh_key: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-source.html#cfn-opsworks-custcookbooksource-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub type_: Option<::Value<String>>,
        /// Property [`Url`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-source.html#cfn-opsworks-custcookbooksource-url).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub url: Option<::Value<String>>,
        /// Property [`Username`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-source.html#cfn-opsworks-custcookbooksource-username).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub username: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Source {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref password) = self.password {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Password", password)?;
            }
            if let Some(ref revision) = self.revision {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Revision", revision)?;
            }
            if let Some(ref ssh_key) = self.ssh_key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SshKey", ssh_key)?;
            }
            if let Some(ref type_) = self.type_ {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", type_)?;
            }
            if let Some(ref url) = self.url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Url", url)?;
            }
            if let Some(ref username) = self.username {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Username", username)?;
            }
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
                    let mut password: Option<::Value<String>> = None;
                    let mut revision: Option<::Value<String>> = None;
                    let mut ssh_key: Option<::Value<String>> = None;
                    let mut type_: Option<::Value<String>> = None;
                    let mut url: Option<::Value<String>> = None;
                    let mut username: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Password" => {
                                password = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Revision" => {
                                revision = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SshKey" => {
                                ssh_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                type_ = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Url" => {
                                url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Username" => {
                                username = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct SslConfiguration {
        /// Property [`Certificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-app-sslconfiguration.html#cfn-opsworks-app-sslconfig-certificate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub certificate: Option<::Value<String>>,
        /// Property [`Chain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-app-sslconfiguration.html#cfn-opsworks-app-sslconfig-chain).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub chain: Option<::Value<String>>,
        /// Property [`PrivateKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-app-sslconfiguration.html#cfn-opsworks-app-sslconfig-privatekey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub private_key: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SslConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref certificate) = self.certificate {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Certificate", certificate)?;
            }
            if let Some(ref chain) = self.chain {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Chain", chain)?;
            }
            if let Some(ref private_key) = self.private_key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrivateKey", private_key)?;
            }
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
                    let mut certificate: Option<::Value<String>> = None;
                    let mut chain: Option<::Value<String>> = None;
                    let mut private_key: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Certificate" => {
                                certificate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Chain" => {
                                chain = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PrivateKey" => {
                                private_key = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct BlockDeviceMapping {
        /// Property [`DeviceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-blockdevicemapping.html#cfn-opsworks-instance-blockdevicemapping-devicename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub device_name: Option<::Value<String>>,
        /// Property [`Ebs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-blockdevicemapping.html#cfn-opsworks-instance-blockdevicemapping-ebs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ebs: Option<::Value<EbsBlockDevice>>,
        /// Property [`NoDevice`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-blockdevicemapping.html#cfn-opsworks-instance-blockdevicemapping-nodevice).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub no_device: Option<::Value<String>>,
        /// Property [`VirtualName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-blockdevicemapping.html#cfn-opsworks-instance-blockdevicemapping-virtualname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub virtual_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for BlockDeviceMapping {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref device_name) = self.device_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceName", device_name)?;
            }
            if let Some(ref ebs) = self.ebs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ebs", ebs)?;
            }
            if let Some(ref no_device) = self.no_device {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NoDevice", no_device)?;
            }
            if let Some(ref virtual_name) = self.virtual_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VirtualName", virtual_name)?;
            }
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
                    let mut device_name: Option<::Value<String>> = None;
                    let mut ebs: Option<::Value<EbsBlockDevice>> = None;
                    let mut no_device: Option<::Value<String>> = None;
                    let mut virtual_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DeviceName" => {
                                device_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Ebs" => {
                                ebs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NoDevice" => {
                                no_device = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VirtualName" => {
                                virtual_name = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct EbsBlockDevice {
        /// Property [`DeleteOnTermination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-ebsblockdevice.html#cfn-opsworks-instance-ebsblockdevice-deleteontermination).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub delete_on_termination: Option<::Value<bool>>,
        /// Property [`Iops`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-ebsblockdevice.html#cfn-opsworks-instance-ebsblockdevice-iops).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub iops: Option<::Value<u32>>,
        /// Property [`SnapshotId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-ebsblockdevice.html#cfn-opsworks-instance-ebsblockdevice-snapshotid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub snapshot_id: Option<::Value<String>>,
        /// Property [`VolumeSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-ebsblockdevice.html#cfn-opsworks-instance-ebsblockdevice-volumesize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub volume_size: Option<::Value<u32>>,
        /// Property [`VolumeType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-ebsblockdevice.html#cfn-opsworks-instance-ebsblockdevice-volumetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub volume_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EbsBlockDevice {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref delete_on_termination) = self.delete_on_termination {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeleteOnTermination", delete_on_termination)?;
            }
            if let Some(ref iops) = self.iops {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Iops", iops)?;
            }
            if let Some(ref snapshot_id) = self.snapshot_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotId", snapshot_id)?;
            }
            if let Some(ref volume_size) = self.volume_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeSize", volume_size)?;
            }
            if let Some(ref volume_type) = self.volume_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeType", volume_type)?;
            }
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
                    let mut delete_on_termination: Option<::Value<bool>> = None;
                    let mut iops: Option<::Value<u32>> = None;
                    let mut snapshot_id: Option<::Value<String>> = None;
                    let mut volume_size: Option<::Value<u32>> = None;
                    let mut volume_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DeleteOnTermination" => {
                                delete_on_termination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Iops" => {
                                iops = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SnapshotId" => {
                                snapshot_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VolumeSize" => {
                                volume_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VolumeType" => {
                                volume_type = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct TimeBasedAutoScaling {
        /// Property [`Friday`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-timebasedautoscaling.html#cfn-opsworks-instance-timebasedautoscaling-friday).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub friday: Option<::ValueMap<String>>,
        /// Property [`Monday`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-timebasedautoscaling.html#cfn-opsworks-instance-timebasedautoscaling-monday).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub monday: Option<::ValueMap<String>>,
        /// Property [`Saturday`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-timebasedautoscaling.html#cfn-opsworks-instance-timebasedautoscaling-saturday).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub saturday: Option<::ValueMap<String>>,
        /// Property [`Sunday`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-timebasedautoscaling.html#cfn-opsworks-instance-timebasedautoscaling-sunday).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sunday: Option<::ValueMap<String>>,
        /// Property [`Thursday`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-timebasedautoscaling.html#cfn-opsworks-instance-timebasedautoscaling-thursday).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub thursday: Option<::ValueMap<String>>,
        /// Property [`Tuesday`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-timebasedautoscaling.html#cfn-opsworks-instance-timebasedautoscaling-tuesday).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tuesday: Option<::ValueMap<String>>,
        /// Property [`Wednesday`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-timebasedautoscaling.html#cfn-opsworks-instance-timebasedautoscaling-wednesday).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub wednesday: Option<::ValueMap<String>>,
    }

    impl ::codec::SerializeValue for TimeBasedAutoScaling {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref friday) = self.friday {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Friday", friday)?;
            }
            if let Some(ref monday) = self.monday {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Monday", monday)?;
            }
            if let Some(ref saturday) = self.saturday {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Saturday", saturday)?;
            }
            if let Some(ref sunday) = self.sunday {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Sunday", sunday)?;
            }
            if let Some(ref thursday) = self.thursday {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Thursday", thursday)?;
            }
            if let Some(ref tuesday) = self.tuesday {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tuesday", tuesday)?;
            }
            if let Some(ref wednesday) = self.wednesday {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Wednesday", wednesday)?;
            }
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
                    let mut friday: Option<::ValueMap<String>> = None;
                    let mut monday: Option<::ValueMap<String>> = None;
                    let mut saturday: Option<::ValueMap<String>> = None;
                    let mut sunday: Option<::ValueMap<String>> = None;
                    let mut thursday: Option<::ValueMap<String>> = None;
                    let mut tuesday: Option<::ValueMap<String>> = None;
                    let mut wednesday: Option<::ValueMap<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Friday" => {
                                friday = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Monday" => {
                                monday = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Saturday" => {
                                saturday = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Sunday" => {
                                sunday = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Thursday" => {
                                thursday = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Tuesday" => {
                                tuesday = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Wednesday" => {
                                wednesday = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct AutoScalingThresholds {
        /// Property [`CpuThreshold`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-loadbasedautoscaling-autoscalingthresholds.html#cfn-opsworks-layer-loadbasedautoscaling-autoscalingthresholds-cputhreshold).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cpu_threshold: Option<::Value<f64>>,
        /// Property [`IgnoreMetricsTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-loadbasedautoscaling-autoscalingthresholds.html#cfn-opsworks-layer-loadbasedautoscaling-autoscalingthresholds-ignoremetricstime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ignore_metrics_time: Option<::Value<u32>>,
        /// Property [`InstanceCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-loadbasedautoscaling-autoscalingthresholds.html#cfn-opsworks-layer-loadbasedautoscaling-autoscalingthresholds-instancecount).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub instance_count: Option<::Value<u32>>,
        /// Property [`LoadThreshold`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-loadbasedautoscaling-autoscalingthresholds.html#cfn-opsworks-layer-loadbasedautoscaling-autoscalingthresholds-loadthreshold).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub load_threshold: Option<::Value<f64>>,
        /// Property [`MemoryThreshold`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-loadbasedautoscaling-autoscalingthresholds.html#cfn-opsworks-layer-loadbasedautoscaling-autoscalingthresholds-memorythreshold).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub memory_threshold: Option<::Value<f64>>,
        /// Property [`ThresholdsWaitTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-loadbasedautoscaling-autoscalingthresholds.html#cfn-opsworks-layer-loadbasedautoscaling-autoscalingthresholds-thresholdwaittime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub thresholds_wait_time: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for AutoScalingThresholds {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cpu_threshold) = self.cpu_threshold {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CpuThreshold", cpu_threshold)?;
            }
            if let Some(ref ignore_metrics_time) = self.ignore_metrics_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IgnoreMetricsTime", ignore_metrics_time)?;
            }
            if let Some(ref instance_count) = self.instance_count {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceCount", instance_count)?;
            }
            if let Some(ref load_threshold) = self.load_threshold {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoadThreshold", load_threshold)?;
            }
            if let Some(ref memory_threshold) = self.memory_threshold {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MemoryThreshold", memory_threshold)?;
            }
            if let Some(ref thresholds_wait_time) = self.thresholds_wait_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThresholdsWaitTime", thresholds_wait_time)?;
            }
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
                    let mut cpu_threshold: Option<::Value<f64>> = None;
                    let mut ignore_metrics_time: Option<::Value<u32>> = None;
                    let mut instance_count: Option<::Value<u32>> = None;
                    let mut load_threshold: Option<::Value<f64>> = None;
                    let mut memory_threshold: Option<::Value<f64>> = None;
                    let mut thresholds_wait_time: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CpuThreshold" => {
                                cpu_threshold = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IgnoreMetricsTime" => {
                                ignore_metrics_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InstanceCount" => {
                                instance_count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LoadThreshold" => {
                                load_threshold = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MemoryThreshold" => {
                                memory_threshold = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ThresholdsWaitTime" => {
                                thresholds_wait_time = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct LifecycleEventConfiguration {
        /// Property [`ShutdownEventConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-lifecycleeventconfiguration.html#cfn-opsworks-layer-lifecycleconfiguration-shutdowneventconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub shutdown_event_configuration: Option<::Value<ShutdownEventConfiguration>>,
    }

    impl ::codec::SerializeValue for LifecycleEventConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref shutdown_event_configuration) = self.shutdown_event_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ShutdownEventConfiguration", shutdown_event_configuration)?;
            }
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
                    let mut shutdown_event_configuration: Option<::Value<ShutdownEventConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ShutdownEventConfiguration" => {
                                shutdown_event_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct LoadBasedAutoScaling {
        /// Property [`DownScaling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-loadbasedautoscaling.html#cfn-opsworks-layer-loadbasedautoscaling-downscaling).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub down_scaling: Option<::Value<AutoScalingThresholds>>,
        /// Property [`Enable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-loadbasedautoscaling.html#cfn-opsworks-layer-loadbasedautoscaling-enable).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable: Option<::Value<bool>>,
        /// Property [`UpScaling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-loadbasedautoscaling.html#cfn-opsworks-layer-loadbasedautoscaling-upscaling).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub up_scaling: Option<::Value<AutoScalingThresholds>>,
    }

    impl ::codec::SerializeValue for LoadBasedAutoScaling {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref down_scaling) = self.down_scaling {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DownScaling", down_scaling)?;
            }
            if let Some(ref enable) = self.enable {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enable", enable)?;
            }
            if let Some(ref up_scaling) = self.up_scaling {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UpScaling", up_scaling)?;
            }
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
                    let mut down_scaling: Option<::Value<AutoScalingThresholds>> = None;
                    let mut enable: Option<::Value<bool>> = None;
                    let mut up_scaling: Option<::Value<AutoScalingThresholds>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DownScaling" => {
                                down_scaling = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Enable" => {
                                enable = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UpScaling" => {
                                up_scaling = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct Recipes {
        /// Property [`Configure`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-recipes.html#cfn-opsworks-layer-customrecipes-configure).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub configure: Option<::ValueList<String>>,
        /// Property [`Deploy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-recipes.html#cfn-opsworks-layer-customrecipes-deploy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub deploy: Option<::ValueList<String>>,
        /// Property [`Setup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-recipes.html#cfn-opsworks-layer-customrecipes-setup).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub setup: Option<::ValueList<String>>,
        /// Property [`Shutdown`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-recipes.html#cfn-opsworks-layer-customrecipes-shutdown).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub shutdown: Option<::ValueList<String>>,
        /// Property [`Undeploy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-recipes.html#cfn-opsworks-layer-customrecipes-undeploy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub undeploy: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for Recipes {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref configure) = self.configure {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Configure", configure)?;
            }
            if let Some(ref deploy) = self.deploy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Deploy", deploy)?;
            }
            if let Some(ref setup) = self.setup {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Setup", setup)?;
            }
            if let Some(ref shutdown) = self.shutdown {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Shutdown", shutdown)?;
            }
            if let Some(ref undeploy) = self.undeploy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Undeploy", undeploy)?;
            }
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
                    let mut configure: Option<::ValueList<String>> = None;
                    let mut deploy: Option<::ValueList<String>> = None;
                    let mut setup: Option<::ValueList<String>> = None;
                    let mut shutdown: Option<::ValueList<String>> = None;
                    let mut undeploy: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Configure" => {
                                configure = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Deploy" => {
                                deploy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Setup" => {
                                setup = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Shutdown" => {
                                shutdown = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Undeploy" => {
                                undeploy = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct ShutdownEventConfiguration {
        /// Property [`DelayUntilElbConnectionsDrained`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-lifecycleeventconfiguration-shutdowneventconfiguration.html#cfn-opsworks-layer-lifecycleconfiguration-shutdowneventconfiguration-delayuntilelbconnectionsdrained).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub delay_until_elb_connections_drained: Option<::Value<bool>>,
        /// Property [`ExecutionTimeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-lifecycleeventconfiguration-shutdowneventconfiguration.html#cfn-opsworks-layer-lifecycleconfiguration-shutdowneventconfiguration-executiontimeout).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub execution_timeout: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for ShutdownEventConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref delay_until_elb_connections_drained) = self.delay_until_elb_connections_drained {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DelayUntilElbConnectionsDrained", delay_until_elb_connections_drained)?;
            }
            if let Some(ref execution_timeout) = self.execution_timeout {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExecutionTimeout", execution_timeout)?;
            }
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
                    let mut delay_until_elb_connections_drained: Option<::Value<bool>> = None;
                    let mut execution_timeout: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DelayUntilElbConnectionsDrained" => {
                                delay_until_elb_connections_drained = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExecutionTimeout" => {
                                execution_timeout = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct VolumeConfiguration {
        /// Property [`Iops`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-volumeconfiguration.html#cfn-opsworks-layer-volconfig-iops).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub iops: Option<::Value<u32>>,
        /// Property [`MountPoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-volumeconfiguration.html#cfn-opsworks-layer-volconfig-mountpoint).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mount_point: Option<::Value<String>>,
        /// Property [`NumberOfDisks`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-volumeconfiguration.html#cfn-opsworks-layer-volconfig-numberofdisks).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub number_of_disks: Option<::Value<u32>>,
        /// Property [`RaidLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-volumeconfiguration.html#cfn-opsworks-layer-volconfig-raidlevel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub raid_level: Option<::Value<u32>>,
        /// Property [`Size`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-volumeconfiguration.html#cfn-opsworks-layer-volconfig-size).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub size: Option<::Value<u32>>,
        /// Property [`VolumeType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-volumeconfiguration.html#cfn-opsworks-layer-volconfig-volumetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub volume_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for VolumeConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref iops) = self.iops {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Iops", iops)?;
            }
            if let Some(ref mount_point) = self.mount_point {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MountPoint", mount_point)?;
            }
            if let Some(ref number_of_disks) = self.number_of_disks {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumberOfDisks", number_of_disks)?;
            }
            if let Some(ref raid_level) = self.raid_level {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RaidLevel", raid_level)?;
            }
            if let Some(ref size) = self.size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Size", size)?;
            }
            if let Some(ref volume_type) = self.volume_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeType", volume_type)?;
            }
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
                    let mut iops: Option<::Value<u32>> = None;
                    let mut mount_point: Option<::Value<String>> = None;
                    let mut number_of_disks: Option<::Value<u32>> = None;
                    let mut raid_level: Option<::Value<u32>> = None;
                    let mut size: Option<::Value<u32>> = None;
                    let mut volume_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Iops" => {
                                iops = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MountPoint" => {
                                mount_point = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NumberOfDisks" => {
                                number_of_disks = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RaidLevel" => {
                                raid_level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Size" => {
                                size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VolumeType" => {
                                volume_type = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct ChefConfiguration {
        /// Property [`BerkshelfVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-chefconfiguration.html#cfn-opsworks-chefconfiguration-berkshelfversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub berkshelf_version: Option<::Value<String>>,
        /// Property [`ManageBerkshelf`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-chefconfiguration.html#cfn-opsworks-chefconfiguration-berkshelfversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub manage_berkshelf: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for ChefConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref berkshelf_version) = self.berkshelf_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BerkshelfVersion", berkshelf_version)?;
            }
            if let Some(ref manage_berkshelf) = self.manage_berkshelf {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManageBerkshelf", manage_berkshelf)?;
            }
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
                    let mut berkshelf_version: Option<::Value<String>> = None;
                    let mut manage_berkshelf: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BerkshelfVersion" => {
                                berkshelf_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ManageBerkshelf" => {
                                manage_berkshelf = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct ElasticIp {
        /// Property [`Ip`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-elasticip.html#cfn-opsworks-stack-elasticip-ip).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ip: ::Value<String>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-elasticip.html#cfn-opsworks-stack-elasticip-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ElasticIp {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ip", &self.ip)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
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
                    let mut ip: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Ip" => {
                                ip = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct RdsDbInstance {
        /// Property [`DbPassword`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-rdsdbinstance.html#cfn-opsworks-stack-rdsdbinstance-dbpassword).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub db_password: ::Value<String>,
        /// Property [`DbUser`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-rdsdbinstance.html#cfn-opsworks-stack-rdsdbinstance-dbuser).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub db_user: ::Value<String>,
        /// Property [`RdsDbInstanceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-rdsdbinstance.html#cfn-opsworks-stack-rdsdbinstance-rdsdbinstancearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
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
                    let mut db_password: Option<::Value<String>> = None;
                    let mut db_user: Option<::Value<String>> = None;
                    let mut rds_db_instance_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DbPassword" => {
                                db_password = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DbUser" => {
                                db_user = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RdsDbInstanceArn" => {
                                rds_db_instance_arn = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct Source {
        /// Property [`Password`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-source.html#cfn-opsworks-custcookbooksource-password).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub password: Option<::Value<String>>,
        /// Property [`Revision`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-source.html#cfn-opsworks-custcookbooksource-revision).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub revision: Option<::Value<String>>,
        /// Property [`SshKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-source.html#cfn-opsworks-custcookbooksource-sshkey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ssh_key: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-source.html#cfn-opsworks-custcookbooksource-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub type_: Option<::Value<String>>,
        /// Property [`Url`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-source.html#cfn-opsworks-custcookbooksource-url).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub url: Option<::Value<String>>,
        /// Property [`Username`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-source.html#cfn-opsworks-custcookbooksource-username).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub username: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Source {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref password) = self.password {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Password", password)?;
            }
            if let Some(ref revision) = self.revision {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Revision", revision)?;
            }
            if let Some(ref ssh_key) = self.ssh_key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SshKey", ssh_key)?;
            }
            if let Some(ref type_) = self.type_ {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", type_)?;
            }
            if let Some(ref url) = self.url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Url", url)?;
            }
            if let Some(ref username) = self.username {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Username", username)?;
            }
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
                    let mut password: Option<::Value<String>> = None;
                    let mut revision: Option<::Value<String>> = None;
                    let mut ssh_key: Option<::Value<String>> = None;
                    let mut type_: Option<::Value<String>> = None;
                    let mut url: Option<::Value<String>> = None;
                    let mut username: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Password" => {
                                password = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Revision" => {
                                revision = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SshKey" => {
                                ssh_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                type_ = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Url" => {
                                url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Username" => {
                                username = ::serde::de::MapAccess::next_value(&mut map)?;
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
    #[derive(Debug, Default)]
    pub struct StackConfigurationManager {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-stackconfigmanager.html#cfn-opsworks-configmanager-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Version`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-stackconfigmanager.html#cfn-opsworks-configmanager-version).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub version: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for StackConfigurationManager {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref version) = self.version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", version)?;
            }
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
                    let mut name: Option<::Value<String>> = None;
                    let mut version: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Version" => {
                                version = ::serde::de::MapAccess::next_value(&mut map)?;
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
