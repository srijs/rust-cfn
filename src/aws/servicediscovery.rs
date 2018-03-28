/// The [`AWS::ServiceDiscovery::Instance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicediscovery-instance.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Instance {
    properties: InstanceProperties
}

/// Properties for the `Instance` resource.
#[derive(Serialize, Deserialize)]
pub struct InstanceProperties {
    #[serde(rename="InstanceAttributes")]
    pub instance_attributes: (),
    #[serde(rename="InstanceId")]
    pub instance_id: (),
    #[serde(rename="ServiceId")]
    pub service_id: (),
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

impl From<InstanceProperties> for Instance {
    fn from(properties: InstanceProperties) -> Instance {
        Instance { properties }
    }
}

/// The [`AWS::ServiceDiscovery::PrivateDnsNamespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicediscovery-privatednsnamespace.html) resource.
#[derive(Serialize, Deserialize)]
pub struct PrivateDnsNamespace {
    properties: PrivateDnsNamespaceProperties
}

/// Properties for the `PrivateDnsNamespace` resource.
#[derive(Serialize, Deserialize)]
pub struct PrivateDnsNamespaceProperties {
    #[serde(rename="Description")]
    pub description: (),
    #[serde(rename="Name")]
    pub name: (),
    #[serde(rename="Vpc")]
    pub vpc: (),
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

impl From<PrivateDnsNamespaceProperties> for PrivateDnsNamespace {
    fn from(properties: PrivateDnsNamespaceProperties) -> PrivateDnsNamespace {
        PrivateDnsNamespace { properties }
    }
}

/// The [`AWS::ServiceDiscovery::PublicDnsNamespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicediscovery-publicdnsnamespace.html) resource.
#[derive(Serialize, Deserialize)]
pub struct PublicDnsNamespace {
    properties: PublicDnsNamespaceProperties
}

/// Properties for the `PublicDnsNamespace` resource.
#[derive(Serialize, Deserialize)]
pub struct PublicDnsNamespaceProperties {
    #[serde(rename="Description")]
    pub description: (),
    #[serde(rename="Name")]
    pub name: (),
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

impl From<PublicDnsNamespaceProperties> for PublicDnsNamespace {
    fn from(properties: PublicDnsNamespaceProperties) -> PublicDnsNamespace {
        PublicDnsNamespace { properties }
    }
}

/// The [`AWS::ServiceDiscovery::Service`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicediscovery-service.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Service {
    properties: ServiceProperties
}

/// Properties for the `Service` resource.
#[derive(Serialize, Deserialize)]
pub struct ServiceProperties {
    #[serde(rename="Description")]
    pub description: (),
    #[serde(rename="DnsConfig")]
    pub dns_config: (),
    #[serde(rename="HealthCheckConfig")]
    pub health_check_config: (),
    #[serde(rename="Name")]
    pub name: (),
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

impl From<ServiceProperties> for Service {
    fn from(properties: ServiceProperties) -> Service {
        Service { properties }
    }
}

