//! Types for the `ServiceDiscovery` service.

/// The [`AWS::ServiceDiscovery::Instance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicediscovery-instance.html) resource type.
#[derive(Debug)]
pub struct Instance {
    properties: InstanceProperties
}

/// Properties for the `Instance` resource.
#[derive(Debug)]
pub struct InstanceProperties {
    /// Property `InstanceAttributes`.
    pub instance_attributes: ::Value<::json::Value>,
    /// Property `InstanceId`.
    pub instance_id: Option<::Value<String>>,
    /// Property `ServiceId`.
    pub service_id: ::Value<String>,
}

impl ::serde::Serialize for InstanceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceAttributes", &self.instance_attributes)?;
        if let Some(ref instance_id) = self.instance_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceId", instance_id)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceId", &self.service_id)?;
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
                let mut instance_attributes = None;
                let mut instance_id = None;
                let mut service_id = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "InstanceAttributes" => {
                            instance_attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceId" => {
                            instance_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServiceId" => {
                            service_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(InstanceProperties {
                    instance_attributes: instance_attributes.ok_or(::serde::de::Error::missing_field("InstanceAttributes"))?,
                    instance_id: instance_id,
                    service_id: service_id.ok_or(::serde::de::Error::missing_field("ServiceId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Instance {
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
#[derive(Debug)]
pub struct PrivateDnsNamespaceProperties {
    /// Property `Description`.
    pub description: Option<::Value<String>>,
    /// Property `Name`.
    pub name: ::Value<String>,
    /// Property `Vpc`.
    pub vpc: ::Value<String>,
}

impl ::serde::Serialize for PrivateDnsNamespaceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Vpc", &self.vpc)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PrivateDnsNamespaceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PrivateDnsNamespaceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PrivateDnsNamespaceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PrivateDnsNamespaceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description = None;
                let mut name = None;
                let mut vpc = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Vpc" => {
                            vpc = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PrivateDnsNamespaceProperties {
                    description: description,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    vpc: vpc.ok_or(::serde::de::Error::missing_field("Vpc"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for PrivateDnsNamespace {
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
#[derive(Debug)]
pub struct PublicDnsNamespaceProperties {
    /// Property `Description`.
    pub description: Option<::Value<String>>,
    /// Property `Name`.
    pub name: ::Value<String>,
}

impl ::serde::Serialize for PublicDnsNamespaceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PublicDnsNamespaceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PublicDnsNamespaceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PublicDnsNamespaceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PublicDnsNamespaceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description = None;
                let mut name = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PublicDnsNamespaceProperties {
                    description: description,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for PublicDnsNamespace {
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
#[derive(Debug)]
pub struct ServiceProperties {
    /// Property `Description`.
    pub description: Option<::Value<String>>,
    /// Property `DnsConfig`.
    pub dns_config: ::Value<self::service::DnsConfig>,
    /// Property `HealthCheckConfig`.
    pub health_check_config: Option<::Value<self::service::HealthCheckConfig>>,
    /// Property `Name`.
    pub name: Option<::Value<String>>,
}

impl ::serde::Serialize for ServiceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DnsConfig", &self.dns_config)?;
        if let Some(ref health_check_config) = self.health_check_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthCheckConfig", health_check_config)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
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
                let mut description = None;
                let mut dns_config = None;
                let mut health_check_config = None;
                let mut name = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DnsConfig" => {
                            dns_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HealthCheckConfig" => {
                            health_check_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ServiceProperties {
                    description: description,
                    dns_config: dns_config.ok_or(::serde::de::Error::missing_field("DnsConfig"))?,
                    health_check_config: health_check_config,
                    name: name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Service {
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
    #[derive(Debug)]
    pub struct DnsConfig {
        /// Property `DnsRecords`.
        pub dns_records: ::ValueList<DnsRecord>,
        /// Property `NamespaceId`.
        pub namespace_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for DnsConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DnsRecords", &self.dns_records)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NamespaceId", &self.namespace_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DnsConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DnsConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DnsConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DnsConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dns_records = None;
                    let mut namespace_id = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DnsRecords" => {
                                dns_records = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NamespaceId" => {
                                namespace_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DnsConfig {
                        dns_records: dns_records.ok_or(::serde::de::Error::missing_field("DnsRecords"))?,
                        namespace_id: namespace_id.ok_or(::serde::de::Error::missing_field("NamespaceId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ServiceDiscovery::Service.DnsRecord`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-servicediscovery-service-dnsrecord.html) property type.
    #[derive(Debug)]
    pub struct DnsRecord {
        /// Property `TTL`.
        pub ttl: ::Value<String>,
        /// Property `Type`.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for DnsRecord {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TTL", &self.ttl)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DnsRecord {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DnsRecord, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DnsRecord;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DnsRecord")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ttl = None;
                    let mut type_ = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TTL" => {
                                ttl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                type_ = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DnsRecord {
                        ttl: ttl.ok_or(::serde::de::Error::missing_field("TTL"))?,
                        type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ServiceDiscovery::Service.HealthCheckConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-servicediscovery-service-healthcheckconfig.html) property type.
    #[derive(Debug)]
    pub struct HealthCheckConfig {
        /// Property `FailureThreshold`.
        pub failure_threshold: Option<::Value<f64>>,
        /// Property `ResourcePath`.
        pub resource_path: Option<::Value<String>>,
        /// Property `Type`.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for HealthCheckConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref failure_threshold) = self.failure_threshold {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FailureThreshold", failure_threshold)?;
            }
            if let Some(ref resource_path) = self.resource_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourcePath", resource_path)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HealthCheckConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HealthCheckConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HealthCheckConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HealthCheckConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut failure_threshold = None;
                    let mut resource_path = None;
                    let mut type_ = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FailureThreshold" => {
                                failure_threshold = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourcePath" => {
                                resource_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                type_ = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HealthCheckConfig {
                        failure_threshold: failure_threshold,
                        resource_path: resource_path,
                        type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
