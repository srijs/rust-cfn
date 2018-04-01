//! Types for the `ServiceDiscovery` service.

/// The [`AWS::ServiceDiscovery::Instance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicediscovery-instance.html) resource type.
#[derive(Debug)]
pub struct Instance {
    properties: InstanceProperties
}

/// Properties for the `Instance` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct InstanceProperties {
    /// Property `InstanceAttributes`.
    #[serde(rename = "InstanceAttributes")]
    pub instance_attributes: ::Value<::json::Value>,
    /// Property `InstanceId`.
    #[serde(rename = "InstanceId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<::Value<String>>,
    /// Property `ServiceId`.
    #[serde(rename = "ServiceId")]
    pub service_id: ::Value<String>,
}

impl<'a> ::Resource<'a> for Instance {
    type Properties = InstanceProperties;
    const TYPE: &'static str = "AWS::ServiceDiscovery::Instance";
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

/// The [`AWS::ServiceDiscovery::PrivateDnsNamespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicediscovery-privatednsnamespace.html) resource type.
#[derive(Debug)]
pub struct PrivateDnsNamespace {
    properties: PrivateDnsNamespaceProperties
}

/// Properties for the `PrivateDnsNamespace` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct PrivateDnsNamespaceProperties {
    /// Property `Description`.
    #[serde(rename = "Description")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<::Value<String>>,
    /// Property `Name`.
    #[serde(rename = "Name")]
    pub name: ::Value<String>,
    /// Property `Vpc`.
    #[serde(rename = "Vpc")]
    pub vpc: ::Value<String>,
}

impl<'a> ::Resource<'a> for PrivateDnsNamespace {
    type Properties = PrivateDnsNamespaceProperties;
    const TYPE: &'static str = "AWS::ServiceDiscovery::PrivateDnsNamespace";
    fn properties(&self) -> &PrivateDnsNamespaceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PrivateDnsNamespaceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for PrivateDnsNamespace {}

impl From<PrivateDnsNamespaceProperties> for PrivateDnsNamespace {
    fn from(properties: PrivateDnsNamespaceProperties) -> PrivateDnsNamespace {
        PrivateDnsNamespace { properties }
    }
}

/// The [`AWS::ServiceDiscovery::PublicDnsNamespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicediscovery-publicdnsnamespace.html) resource type.
#[derive(Debug)]
pub struct PublicDnsNamespace {
    properties: PublicDnsNamespaceProperties
}

/// Properties for the `PublicDnsNamespace` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct PublicDnsNamespaceProperties {
    /// Property `Description`.
    #[serde(rename = "Description")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<::Value<String>>,
    /// Property `Name`.
    #[serde(rename = "Name")]
    pub name: ::Value<String>,
}

impl<'a> ::Resource<'a> for PublicDnsNamespace {
    type Properties = PublicDnsNamespaceProperties;
    const TYPE: &'static str = "AWS::ServiceDiscovery::PublicDnsNamespace";
    fn properties(&self) -> &PublicDnsNamespaceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PublicDnsNamespaceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for PublicDnsNamespace {}

impl From<PublicDnsNamespaceProperties> for PublicDnsNamespace {
    fn from(properties: PublicDnsNamespaceProperties) -> PublicDnsNamespace {
        PublicDnsNamespace { properties }
    }
}

/// The [`AWS::ServiceDiscovery::Service`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicediscovery-service.html) resource type.
#[derive(Debug)]
pub struct Service {
    properties: ServiceProperties
}

/// Properties for the `Service` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceProperties {
    /// Property `Description`.
    #[serde(rename = "Description")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<::Value<String>>,
    /// Property `DnsConfig`.
    #[serde(rename = "DnsConfig")]
    pub dns_config: ::Value<self::service::DnsConfig>,
    /// Property `HealthCheckConfig`.
    #[serde(rename = "HealthCheckConfig")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health_check_config: Option<::Value<self::service::HealthCheckConfig>>,
    /// Property `Name`.
    #[serde(rename = "Name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<::Value<String>>,
}

impl<'a> ::Resource<'a> for Service {
    type Properties = ServiceProperties;
    const TYPE: &'static str = "AWS::ServiceDiscovery::Service";
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

pub mod service {
    //! Property types for the `Service` resource.

    /// The [`AWS::ServiceDiscovery::Service.DnsConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-servicediscovery-service-dnsconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DnsConfig {
        /// Property `DnsRecords`.
        #[serde(rename = "DnsRecords")]
        pub dns_records: ::ValueList<DnsRecord>,
        /// Property `NamespaceId`.
        #[serde(rename = "NamespaceId")]
        pub namespace_id: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(DnsConfig);

    /// The [`AWS::ServiceDiscovery::Service.DnsRecord`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-servicediscovery-service-dnsrecord.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DnsRecord {
        /// Property `TTL`.
        #[serde(rename = "TTL")]
        pub ttl: ::Value<String>,
        /// Property `Type`.
        #[serde(rename = "Type")]
        pub type_: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(DnsRecord);

    /// The [`AWS::ServiceDiscovery::Service.HealthCheckConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-servicediscovery-service-healthcheckconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct HealthCheckConfig {
        /// Property `FailureThreshold`.
        #[serde(rename = "FailureThreshold")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub failure_threshold: Option<::Value<f64>>,
        /// Property `ResourcePath`.
        #[serde(rename = "ResourcePath")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource_path: Option<::Value<String>>,
        /// Property `Type`.
        #[serde(rename = "Type")]
        pub type_: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(HealthCheckConfig);
}
