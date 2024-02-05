//! Types for the `GlobalAccelerator` service.

/// The [`AWS::GlobalAccelerator::Accelerator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-globalaccelerator-accelerator.html) resource type.
#[derive(Debug, Default)]
pub struct Accelerator {
    properties: AcceleratorProperties
}

/// Properties for the `Accelerator` resource.
#[derive(Debug, Default)]
pub struct AcceleratorProperties {
    /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-globalaccelerator-accelerator.html#cfn-globalaccelerator-accelerator-enabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enabled: Option<::Value<bool>>,
    /// Property [`IpAddressType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-globalaccelerator-accelerator.html#cfn-globalaccelerator-accelerator-ipaddresstype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ip_address_type: Option<::Value<String>>,
    /// Property [`IpAddresses`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-globalaccelerator-accelerator.html#cfn-globalaccelerator-accelerator-ipaddresses).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ip_addresses: Option<::ValueList<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-globalaccelerator-accelerator.html#cfn-globalaccelerator-accelerator-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-globalaccelerator-accelerator.html#cfn-globalaccelerator-accelerator-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for AcceleratorProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref enabled) = self.enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
        }
        if let Some(ref ip_address_type) = self.ip_address_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IpAddressType", ip_address_type)?;
        }
        if let Some(ref ip_addresses) = self.ip_addresses {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IpAddresses", ip_addresses)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AcceleratorProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AcceleratorProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AcceleratorProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AcceleratorProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut enabled: Option<::Value<bool>> = None;
                let mut ip_address_type: Option<::Value<String>> = None;
                let mut ip_addresses: Option<::ValueList<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Enabled" => {
                            enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IpAddressType" => {
                            ip_address_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IpAddresses" => {
                            ip_addresses = ::serde::de::MapAccess::next_value(&mut map)?;
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

                Ok(AcceleratorProperties {
                    enabled: enabled,
                    ip_address_type: ip_address_type,
                    ip_addresses: ip_addresses,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Accelerator {
    type Properties = AcceleratorProperties;
    const TYPE: &'static str = "AWS::GlobalAccelerator::Accelerator";
    fn properties(&self) -> &AcceleratorProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AcceleratorProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Accelerator {}

impl From<AcceleratorProperties> for Accelerator {
    fn from(properties: AcceleratorProperties) -> Accelerator {
        Accelerator { properties }
    }
}

/// The [`AWS::GlobalAccelerator::EndpointGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-globalaccelerator-endpointgroup.html) resource type.
#[derive(Debug, Default)]
pub struct EndpointGroup {
    properties: EndpointGroupProperties
}

/// Properties for the `EndpointGroup` resource.
#[derive(Debug, Default)]
pub struct EndpointGroupProperties {
    /// Property [`EndpointConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-globalaccelerator-endpointgroup.html#cfn-globalaccelerator-endpointgroup-endpointconfigurations).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub endpoint_configurations: Option<::ValueList<self::endpoint_group::EndpointConfiguration>>,
    /// Property [`EndpointGroupRegion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-globalaccelerator-endpointgroup.html#cfn-globalaccelerator-endpointgroup-endpointgroupregion).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub endpoint_group_region: ::Value<String>,
    /// Property [`HealthCheckIntervalSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-globalaccelerator-endpointgroup.html#cfn-globalaccelerator-endpointgroup-healthcheckintervalseconds).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub health_check_interval_seconds: Option<::Value<u32>>,
    /// Property [`HealthCheckPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-globalaccelerator-endpointgroup.html#cfn-globalaccelerator-endpointgroup-healthcheckpath).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub health_check_path: Option<::Value<String>>,
    /// Property [`HealthCheckPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-globalaccelerator-endpointgroup.html#cfn-globalaccelerator-endpointgroup-healthcheckport).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub health_check_port: Option<::Value<u32>>,
    /// Property [`HealthCheckProtocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-globalaccelerator-endpointgroup.html#cfn-globalaccelerator-endpointgroup-healthcheckprotocol).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub health_check_protocol: Option<::Value<String>>,
    /// Property [`ListenerArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-globalaccelerator-endpointgroup.html#cfn-globalaccelerator-endpointgroup-listenerarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub listener_arn: ::Value<String>,
    /// Property [`PortOverrides`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-globalaccelerator-endpointgroup.html#cfn-globalaccelerator-endpointgroup-portoverrides).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub port_overrides: Option<::ValueList<self::endpoint_group::PortOverride>>,
    /// Property [`ThresholdCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-globalaccelerator-endpointgroup.html#cfn-globalaccelerator-endpointgroup-thresholdcount).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub threshold_count: Option<::Value<u32>>,
    /// Property [`TrafficDialPercentage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-globalaccelerator-endpointgroup.html#cfn-globalaccelerator-endpointgroup-trafficdialpercentage).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub traffic_dial_percentage: Option<::Value<f64>>,
}

impl ::serde::Serialize for EndpointGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref endpoint_configurations) = self.endpoint_configurations {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointConfigurations", endpoint_configurations)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointGroupRegion", &self.endpoint_group_region)?;
        if let Some(ref health_check_interval_seconds) = self.health_check_interval_seconds {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthCheckIntervalSeconds", health_check_interval_seconds)?;
        }
        if let Some(ref health_check_path) = self.health_check_path {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthCheckPath", health_check_path)?;
        }
        if let Some(ref health_check_port) = self.health_check_port {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthCheckPort", health_check_port)?;
        }
        if let Some(ref health_check_protocol) = self.health_check_protocol {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HealthCheckProtocol", health_check_protocol)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ListenerArn", &self.listener_arn)?;
        if let Some(ref port_overrides) = self.port_overrides {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PortOverrides", port_overrides)?;
        }
        if let Some(ref threshold_count) = self.threshold_count {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThresholdCount", threshold_count)?;
        }
        if let Some(ref traffic_dial_percentage) = self.traffic_dial_percentage {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TrafficDialPercentage", traffic_dial_percentage)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EndpointGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EndpointGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EndpointGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EndpointGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut endpoint_configurations: Option<::ValueList<self::endpoint_group::EndpointConfiguration>> = None;
                let mut endpoint_group_region: Option<::Value<String>> = None;
                let mut health_check_interval_seconds: Option<::Value<u32>> = None;
                let mut health_check_path: Option<::Value<String>> = None;
                let mut health_check_port: Option<::Value<u32>> = None;
                let mut health_check_protocol: Option<::Value<String>> = None;
                let mut listener_arn: Option<::Value<String>> = None;
                let mut port_overrides: Option<::ValueList<self::endpoint_group::PortOverride>> = None;
                let mut threshold_count: Option<::Value<u32>> = None;
                let mut traffic_dial_percentage: Option<::Value<f64>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "EndpointConfigurations" => {
                            endpoint_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EndpointGroupRegion" => {
                            endpoint_group_region = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HealthCheckIntervalSeconds" => {
                            health_check_interval_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HealthCheckPath" => {
                            health_check_path = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HealthCheckPort" => {
                            health_check_port = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HealthCheckProtocol" => {
                            health_check_protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ListenerArn" => {
                            listener_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PortOverrides" => {
                            port_overrides = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ThresholdCount" => {
                            threshold_count = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TrafficDialPercentage" => {
                            traffic_dial_percentage = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(EndpointGroupProperties {
                    endpoint_configurations: endpoint_configurations,
                    endpoint_group_region: endpoint_group_region.ok_or(::serde::de::Error::missing_field("EndpointGroupRegion"))?,
                    health_check_interval_seconds: health_check_interval_seconds,
                    health_check_path: health_check_path,
                    health_check_port: health_check_port,
                    health_check_protocol: health_check_protocol,
                    listener_arn: listener_arn.ok_or(::serde::de::Error::missing_field("ListenerArn"))?,
                    port_overrides: port_overrides,
                    threshold_count: threshold_count,
                    traffic_dial_percentage: traffic_dial_percentage,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for EndpointGroup {
    type Properties = EndpointGroupProperties;
    const TYPE: &'static str = "AWS::GlobalAccelerator::EndpointGroup";
    fn properties(&self) -> &EndpointGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EndpointGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for EndpointGroup {}

impl From<EndpointGroupProperties> for EndpointGroup {
    fn from(properties: EndpointGroupProperties) -> EndpointGroup {
        EndpointGroup { properties }
    }
}

/// The [`AWS::GlobalAccelerator::Listener`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-globalaccelerator-listener.html) resource type.
#[derive(Debug, Default)]
pub struct Listener {
    properties: ListenerProperties
}

/// Properties for the `Listener` resource.
#[derive(Debug, Default)]
pub struct ListenerProperties {
    /// Property [`AcceleratorArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-globalaccelerator-listener.html#cfn-globalaccelerator-listener-acceleratorarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub accelerator_arn: ::Value<String>,
    /// Property [`ClientAffinity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-globalaccelerator-listener.html#cfn-globalaccelerator-listener-clientaffinity).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub client_affinity: Option<::Value<String>>,
    /// Property [`PortRanges`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-globalaccelerator-listener.html#cfn-globalaccelerator-listener-portranges).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub port_ranges: ::ValueList<self::listener::PortRange>,
    /// Property [`Protocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-globalaccelerator-listener.html#cfn-globalaccelerator-listener-protocol).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub protocol: ::Value<String>,
}

impl ::serde::Serialize for ListenerProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AcceleratorArn", &self.accelerator_arn)?;
        if let Some(ref client_affinity) = self.client_affinity {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientAffinity", client_affinity)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PortRanges", &self.port_ranges)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocol", &self.protocol)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ListenerProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ListenerProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ListenerProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ListenerProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut accelerator_arn: Option<::Value<String>> = None;
                let mut client_affinity: Option<::Value<String>> = None;
                let mut port_ranges: Option<::ValueList<self::listener::PortRange>> = None;
                let mut protocol: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AcceleratorArn" => {
                            accelerator_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClientAffinity" => {
                            client_affinity = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PortRanges" => {
                            port_ranges = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Protocol" => {
                            protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ListenerProperties {
                    accelerator_arn: accelerator_arn.ok_or(::serde::de::Error::missing_field("AcceleratorArn"))?,
                    client_affinity: client_affinity,
                    port_ranges: port_ranges.ok_or(::serde::de::Error::missing_field("PortRanges"))?,
                    protocol: protocol.ok_or(::serde::de::Error::missing_field("Protocol"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Listener {
    type Properties = ListenerProperties;
    const TYPE: &'static str = "AWS::GlobalAccelerator::Listener";
    fn properties(&self) -> &ListenerProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ListenerProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Listener {}

impl From<ListenerProperties> for Listener {
    fn from(properties: ListenerProperties) -> Listener {
        Listener { properties }
    }
}

pub mod endpoint_group {
    //! Property types for the `EndpointGroup` resource.

    /// The [`AWS::GlobalAccelerator::EndpointGroup.EndpointConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-globalaccelerator-endpointgroup-endpointconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct EndpointConfiguration {
        /// Property [`AttachmentArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-globalaccelerator-endpointgroup-endpointconfiguration.html#cfn-globalaccelerator-endpointgroup-endpointconfiguration-attachmentarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attachment_arn: Option<::Value<String>>,
        /// Property [`ClientIPPreservationEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-globalaccelerator-endpointgroup-endpointconfiguration.html#cfn-globalaccelerator-endpointgroup-endpointconfiguration-clientippreservationenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_ip_preservation_enabled: Option<::Value<bool>>,
        /// Property [`EndpointId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-globalaccelerator-endpointgroup-endpointconfiguration.html#cfn-globalaccelerator-endpointgroup-endpointconfiguration-endpointid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub endpoint_id: ::Value<String>,
        /// Property [`Weight`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-globalaccelerator-endpointgroup-endpointconfiguration.html#cfn-globalaccelerator-endpointgroup-endpointconfiguration-weight).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub weight: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for EndpointConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref attachment_arn) = self.attachment_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttachmentArn", attachment_arn)?;
            }
            if let Some(ref client_ip_preservation_enabled) = self.client_ip_preservation_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientIPPreservationEnabled", client_ip_preservation_enabled)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointId", &self.endpoint_id)?;
            if let Some(ref weight) = self.weight {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Weight", weight)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EndpointConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EndpointConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EndpointConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EndpointConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attachment_arn: Option<::Value<String>> = None;
                    let mut client_ip_preservation_enabled: Option<::Value<bool>> = None;
                    let mut endpoint_id: Option<::Value<String>> = None;
                    let mut weight: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AttachmentArn" => {
                                attachment_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ClientIPPreservationEnabled" => {
                                client_ip_preservation_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EndpointId" => {
                                endpoint_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Weight" => {
                                weight = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EndpointConfiguration {
                        attachment_arn: attachment_arn,
                        client_ip_preservation_enabled: client_ip_preservation_enabled,
                        endpoint_id: endpoint_id.ok_or(::serde::de::Error::missing_field("EndpointId"))?,
                        weight: weight,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::GlobalAccelerator::EndpointGroup.PortOverride`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-globalaccelerator-endpointgroup-portoverride.html) property type.
    #[derive(Debug, Default)]
    pub struct PortOverride {
        /// Property [`EndpointPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-globalaccelerator-endpointgroup-portoverride.html#cfn-globalaccelerator-endpointgroup-portoverride-endpointport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub endpoint_port: ::Value<u32>,
        /// Property [`ListenerPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-globalaccelerator-endpointgroup-portoverride.html#cfn-globalaccelerator-endpointgroup-portoverride-listenerport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub listener_port: ::Value<u32>,
    }

    impl ::codec::SerializeValue for PortOverride {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointPort", &self.endpoint_port)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ListenerPort", &self.listener_port)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PortOverride {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PortOverride, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PortOverride;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PortOverride")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut endpoint_port: Option<::Value<u32>> = None;
                    let mut listener_port: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EndpointPort" => {
                                endpoint_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ListenerPort" => {
                                listener_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PortOverride {
                        endpoint_port: endpoint_port.ok_or(::serde::de::Error::missing_field("EndpointPort"))?,
                        listener_port: listener_port.ok_or(::serde::de::Error::missing_field("ListenerPort"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod listener {
    //! Property types for the `Listener` resource.

    /// The [`AWS::GlobalAccelerator::Listener.PortRange`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-globalaccelerator-listener-portrange.html) property type.
    #[derive(Debug, Default)]
    pub struct PortRange {
        /// Property [`FromPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-globalaccelerator-listener-portrange.html#cfn-globalaccelerator-listener-portrange-fromport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub from_port: ::Value<u32>,
        /// Property [`ToPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-globalaccelerator-listener-portrange.html#cfn-globalaccelerator-listener-portrange-toport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub to_port: ::Value<u32>,
    }

    impl ::codec::SerializeValue for PortRange {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FromPort", &self.from_port)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ToPort", &self.to_port)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PortRange {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PortRange, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PortRange;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PortRange")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut from_port: Option<::Value<u32>> = None;
                    let mut to_port: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FromPort" => {
                                from_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ToPort" => {
                                to_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PortRange {
                        from_port: from_port.ok_or(::serde::de::Error::missing_field("FromPort"))?,
                        to_port: to_port.ok_or(::serde::de::Error::missing_field("ToPort"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
