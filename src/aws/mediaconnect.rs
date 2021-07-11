//! Types for the `MediaConnect` service.

/// The [`AWS::MediaConnect::Flow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flow.html) resource type.
#[derive(Debug, Default)]
pub struct Flow {
    properties: FlowProperties
}

/// Properties for the `Flow` resource.
#[derive(Debug, Default)]
pub struct FlowProperties {
    /// Property [`AvailabilityZone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flow.html#cfn-mediaconnect-flow-availabilityzone).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub availability_zone: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flow.html#cfn-mediaconnect-flow-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flow.html#cfn-mediaconnect-flow-source).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub source: ::Value<self::flow::Source>,
    /// Property [`SourceFailoverConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flow.html#cfn-mediaconnect-flow-sourcefailoverconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub source_failover_config: Option<::Value<self::flow::FailoverConfig>>,
}

impl ::serde::Serialize for FlowProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref availability_zone) = self.availability_zone {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZone", availability_zone)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Source", &self.source)?;
        if let Some(ref source_failover_config) = self.source_failover_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceFailoverConfig", source_failover_config)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for FlowProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<FlowProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = FlowProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type FlowProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut availability_zone: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut source: Option<::Value<self::flow::Source>> = None;
                let mut source_failover_config: Option<::Value<self::flow::FailoverConfig>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AvailabilityZone" => {
                            availability_zone = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Source" => {
                            source = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceFailoverConfig" => {
                            source_failover_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(FlowProperties {
                    availability_zone: availability_zone,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    source: source.ok_or(::serde::de::Error::missing_field("Source"))?,
                    source_failover_config: source_failover_config,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Flow {
    type Properties = FlowProperties;
    const TYPE: &'static str = "AWS::MediaConnect::Flow";
    fn properties(&self) -> &FlowProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut FlowProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Flow {}

impl From<FlowProperties> for Flow {
    fn from(properties: FlowProperties) -> Flow {
        Flow { properties }
    }
}

/// The [`AWS::MediaConnect::FlowEntitlement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowentitlement.html) resource type.
#[derive(Debug, Default)]
pub struct FlowEntitlement {
    properties: FlowEntitlementProperties
}

/// Properties for the `FlowEntitlement` resource.
#[derive(Debug, Default)]
pub struct FlowEntitlementProperties {
    /// Property [`DataTransferSubscriberFeePercent`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowentitlement.html#cfn-mediaconnect-flowentitlement-datatransfersubscriberfeepercent).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub data_transfer_subscriber_fee_percent: Option<::Value<u32>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowentitlement.html#cfn-mediaconnect-flowentitlement-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: ::Value<String>,
    /// Property [`Encryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowentitlement.html#cfn-mediaconnect-flowentitlement-encryption).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub encryption: Option<::Value<self::flow_entitlement::Encryption>>,
    /// Property [`EntitlementStatus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowentitlement.html#cfn-mediaconnect-flowentitlement-entitlementstatus).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub entitlement_status: Option<::Value<String>>,
    /// Property [`FlowArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowentitlement.html#cfn-mediaconnect-flowentitlement-flowarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub flow_arn: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowentitlement.html#cfn-mediaconnect-flowentitlement-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Subscribers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowentitlement.html#cfn-mediaconnect-flowentitlement-subscribers).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub subscribers: ::ValueList<String>,
}

impl ::serde::Serialize for FlowEntitlementProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref data_transfer_subscriber_fee_percent) = self.data_transfer_subscriber_fee_percent {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataTransferSubscriberFeePercent", data_transfer_subscriber_fee_percent)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        if let Some(ref encryption) = self.encryption {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Encryption", encryption)?;
        }
        if let Some(ref entitlement_status) = self.entitlement_status {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EntitlementStatus", entitlement_status)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FlowArn", &self.flow_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subscribers", &self.subscribers)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for FlowEntitlementProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<FlowEntitlementProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = FlowEntitlementProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type FlowEntitlementProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut data_transfer_subscriber_fee_percent: Option<::Value<u32>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut encryption: Option<::Value<self::flow_entitlement::Encryption>> = None;
                let mut entitlement_status: Option<::Value<String>> = None;
                let mut flow_arn: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut subscribers: Option<::ValueList<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DataTransferSubscriberFeePercent" => {
                            data_transfer_subscriber_fee_percent = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Encryption" => {
                            encryption = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EntitlementStatus" => {
                            entitlement_status = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FlowArn" => {
                            flow_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Subscribers" => {
                            subscribers = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(FlowEntitlementProperties {
                    data_transfer_subscriber_fee_percent: data_transfer_subscriber_fee_percent,
                    description: description.ok_or(::serde::de::Error::missing_field("Description"))?,
                    encryption: encryption,
                    entitlement_status: entitlement_status,
                    flow_arn: flow_arn.ok_or(::serde::de::Error::missing_field("FlowArn"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    subscribers: subscribers.ok_or(::serde::de::Error::missing_field("Subscribers"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for FlowEntitlement {
    type Properties = FlowEntitlementProperties;
    const TYPE: &'static str = "AWS::MediaConnect::FlowEntitlement";
    fn properties(&self) -> &FlowEntitlementProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut FlowEntitlementProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for FlowEntitlement {}

impl From<FlowEntitlementProperties> for FlowEntitlement {
    fn from(properties: FlowEntitlementProperties) -> FlowEntitlement {
        FlowEntitlement { properties }
    }
}

/// The [`AWS::MediaConnect::FlowOutput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowoutput.html) resource type.
#[derive(Debug, Default)]
pub struct FlowOutput {
    properties: FlowOutputProperties
}

/// Properties for the `FlowOutput` resource.
#[derive(Debug, Default)]
pub struct FlowOutputProperties {
    /// Property [`CidrAllowList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowoutput.html#cfn-mediaconnect-flowoutput-cidrallowlist).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cidr_allow_list: Option<::ValueList<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowoutput.html#cfn-mediaconnect-flowoutput-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowoutput.html#cfn-mediaconnect-flowoutput-destination).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub destination: Option<::Value<String>>,
    /// Property [`Encryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowoutput.html#cfn-mediaconnect-flowoutput-encryption).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub encryption: Option<::Value<self::flow_output::Encryption>>,
    /// Property [`FlowArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowoutput.html#cfn-mediaconnect-flowoutput-flowarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub flow_arn: ::Value<String>,
    /// Property [`MaxLatency`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowoutput.html#cfn-mediaconnect-flowoutput-maxlatency).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub max_latency: Option<::Value<u32>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowoutput.html#cfn-mediaconnect-flowoutput-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowoutput.html#cfn-mediaconnect-flowoutput-port).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub port: Option<::Value<u32>>,
    /// Property [`Protocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowoutput.html#cfn-mediaconnect-flowoutput-protocol).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub protocol: ::Value<String>,
    /// Property [`RemoteId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowoutput.html#cfn-mediaconnect-flowoutput-remoteid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub remote_id: Option<::Value<String>>,
    /// Property [`SmoothingLatency`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowoutput.html#cfn-mediaconnect-flowoutput-smoothinglatency).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub smoothing_latency: Option<::Value<u32>>,
    /// Property [`StreamId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowoutput.html#cfn-mediaconnect-flowoutput-streamid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub stream_id: Option<::Value<String>>,
    /// Property [`VpcInterfaceAttachment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowoutput.html#cfn-mediaconnect-flowoutput-vpcinterfaceattachment).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub vpc_interface_attachment: Option<::Value<self::flow_output::VpcInterfaceAttachment>>,
}

impl ::serde::Serialize for FlowOutputProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref cidr_allow_list) = self.cidr_allow_list {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CidrAllowList", cidr_allow_list)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref destination) = self.destination {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Destination", destination)?;
        }
        if let Some(ref encryption) = self.encryption {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Encryption", encryption)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FlowArn", &self.flow_arn)?;
        if let Some(ref max_latency) = self.max_latency {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxLatency", max_latency)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref port) = self.port {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocol", &self.protocol)?;
        if let Some(ref remote_id) = self.remote_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RemoteId", remote_id)?;
        }
        if let Some(ref smoothing_latency) = self.smoothing_latency {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SmoothingLatency", smoothing_latency)?;
        }
        if let Some(ref stream_id) = self.stream_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamId", stream_id)?;
        }
        if let Some(ref vpc_interface_attachment) = self.vpc_interface_attachment {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcInterfaceAttachment", vpc_interface_attachment)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for FlowOutputProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<FlowOutputProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = FlowOutputProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type FlowOutputProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut cidr_allow_list: Option<::ValueList<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut destination: Option<::Value<String>> = None;
                let mut encryption: Option<::Value<self::flow_output::Encryption>> = None;
                let mut flow_arn: Option<::Value<String>> = None;
                let mut max_latency: Option<::Value<u32>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut port: Option<::Value<u32>> = None;
                let mut protocol: Option<::Value<String>> = None;
                let mut remote_id: Option<::Value<String>> = None;
                let mut smoothing_latency: Option<::Value<u32>> = None;
                let mut stream_id: Option<::Value<String>> = None;
                let mut vpc_interface_attachment: Option<::Value<self::flow_output::VpcInterfaceAttachment>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CidrAllowList" => {
                            cidr_allow_list = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Destination" => {
                            destination = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Encryption" => {
                            encryption = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FlowArn" => {
                            flow_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaxLatency" => {
                            max_latency = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Port" => {
                            port = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Protocol" => {
                            protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RemoteId" => {
                            remote_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SmoothingLatency" => {
                            smoothing_latency = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StreamId" => {
                            stream_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcInterfaceAttachment" => {
                            vpc_interface_attachment = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(FlowOutputProperties {
                    cidr_allow_list: cidr_allow_list,
                    description: description,
                    destination: destination,
                    encryption: encryption,
                    flow_arn: flow_arn.ok_or(::serde::de::Error::missing_field("FlowArn"))?,
                    max_latency: max_latency,
                    name: name,
                    port: port,
                    protocol: protocol.ok_or(::serde::de::Error::missing_field("Protocol"))?,
                    remote_id: remote_id,
                    smoothing_latency: smoothing_latency,
                    stream_id: stream_id,
                    vpc_interface_attachment: vpc_interface_attachment,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for FlowOutput {
    type Properties = FlowOutputProperties;
    const TYPE: &'static str = "AWS::MediaConnect::FlowOutput";
    fn properties(&self) -> &FlowOutputProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut FlowOutputProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for FlowOutput {}

impl From<FlowOutputProperties> for FlowOutput {
    fn from(properties: FlowOutputProperties) -> FlowOutput {
        FlowOutput { properties }
    }
}

/// The [`AWS::MediaConnect::FlowSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowsource.html) resource type.
#[derive(Debug, Default)]
pub struct FlowSource {
    properties: FlowSourceProperties
}

/// Properties for the `FlowSource` resource.
#[derive(Debug, Default)]
pub struct FlowSourceProperties {
    /// Property [`Decryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowsource.html#cfn-mediaconnect-flowsource-decryption).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub decryption: Option<::Value<self::flow_source::Encryption>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowsource.html#cfn-mediaconnect-flowsource-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: ::Value<String>,
    /// Property [`EntitlementArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowsource.html#cfn-mediaconnect-flowsource-entitlementarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub entitlement_arn: Option<::Value<String>>,
    /// Property [`FlowArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowsource.html#cfn-mediaconnect-flowsource-flowarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub flow_arn: Option<::Value<String>>,
    /// Property [`IngestPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowsource.html#cfn-mediaconnect-flowsource-ingestport).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ingest_port: Option<::Value<u32>>,
    /// Property [`MaxBitrate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowsource.html#cfn-mediaconnect-flowsource-maxbitrate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub max_bitrate: Option<::Value<u32>>,
    /// Property [`MaxLatency`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowsource.html#cfn-mediaconnect-flowsource-maxlatency).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub max_latency: Option<::Value<u32>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowsource.html#cfn-mediaconnect-flowsource-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Protocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowsource.html#cfn-mediaconnect-flowsource-protocol).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub protocol: Option<::Value<String>>,
    /// Property [`StreamId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowsource.html#cfn-mediaconnect-flowsource-streamid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub stream_id: Option<::Value<String>>,
    /// Property [`VpcInterfaceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowsource.html#cfn-mediaconnect-flowsource-vpcinterfacename).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub vpc_interface_name: Option<::Value<String>>,
    /// Property [`WhitelistCidr`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowsource.html#cfn-mediaconnect-flowsource-whitelistcidr).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub whitelist_cidr: Option<::Value<String>>,
}

impl ::serde::Serialize for FlowSourceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref decryption) = self.decryption {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Decryption", decryption)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        if let Some(ref entitlement_arn) = self.entitlement_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EntitlementArn", entitlement_arn)?;
        }
        if let Some(ref flow_arn) = self.flow_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FlowArn", flow_arn)?;
        }
        if let Some(ref ingest_port) = self.ingest_port {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IngestPort", ingest_port)?;
        }
        if let Some(ref max_bitrate) = self.max_bitrate {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxBitrate", max_bitrate)?;
        }
        if let Some(ref max_latency) = self.max_latency {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxLatency", max_latency)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref protocol) = self.protocol {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocol", protocol)?;
        }
        if let Some(ref stream_id) = self.stream_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamId", stream_id)?;
        }
        if let Some(ref vpc_interface_name) = self.vpc_interface_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcInterfaceName", vpc_interface_name)?;
        }
        if let Some(ref whitelist_cidr) = self.whitelist_cidr {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WhitelistCidr", whitelist_cidr)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for FlowSourceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<FlowSourceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = FlowSourceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type FlowSourceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut decryption: Option<::Value<self::flow_source::Encryption>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut entitlement_arn: Option<::Value<String>> = None;
                let mut flow_arn: Option<::Value<String>> = None;
                let mut ingest_port: Option<::Value<u32>> = None;
                let mut max_bitrate: Option<::Value<u32>> = None;
                let mut max_latency: Option<::Value<u32>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut protocol: Option<::Value<String>> = None;
                let mut stream_id: Option<::Value<String>> = None;
                let mut vpc_interface_name: Option<::Value<String>> = None;
                let mut whitelist_cidr: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Decryption" => {
                            decryption = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EntitlementArn" => {
                            entitlement_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FlowArn" => {
                            flow_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IngestPort" => {
                            ingest_port = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaxBitrate" => {
                            max_bitrate = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaxLatency" => {
                            max_latency = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Protocol" => {
                            protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StreamId" => {
                            stream_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcInterfaceName" => {
                            vpc_interface_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "WhitelistCidr" => {
                            whitelist_cidr = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(FlowSourceProperties {
                    decryption: decryption,
                    description: description.ok_or(::serde::de::Error::missing_field("Description"))?,
                    entitlement_arn: entitlement_arn,
                    flow_arn: flow_arn,
                    ingest_port: ingest_port,
                    max_bitrate: max_bitrate,
                    max_latency: max_latency,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    protocol: protocol,
                    stream_id: stream_id,
                    vpc_interface_name: vpc_interface_name,
                    whitelist_cidr: whitelist_cidr,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for FlowSource {
    type Properties = FlowSourceProperties;
    const TYPE: &'static str = "AWS::MediaConnect::FlowSource";
    fn properties(&self) -> &FlowSourceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut FlowSourceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for FlowSource {}

impl From<FlowSourceProperties> for FlowSource {
    fn from(properties: FlowSourceProperties) -> FlowSource {
        FlowSource { properties }
    }
}

/// The [`AWS::MediaConnect::FlowVpcInterface`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowvpcinterface.html) resource type.
#[derive(Debug, Default)]
pub struct FlowVpcInterface {
    properties: FlowVpcInterfaceProperties
}

/// Properties for the `FlowVpcInterface` resource.
#[derive(Debug, Default)]
pub struct FlowVpcInterfaceProperties {
    /// Property [`FlowArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowvpcinterface.html#cfn-mediaconnect-flowvpcinterface-flowarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub flow_arn: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowvpcinterface.html#cfn-mediaconnect-flowvpcinterface-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowvpcinterface.html#cfn-mediaconnect-flowvpcinterface-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: ::Value<String>,
    /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowvpcinterface.html#cfn-mediaconnect-flowvpcinterface-securitygroupids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub security_group_ids: ::ValueList<String>,
    /// Property [`SubnetId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowvpcinterface.html#cfn-mediaconnect-flowvpcinterface-subnetid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub subnet_id: ::Value<String>,
}

impl ::serde::Serialize for FlowVpcInterfaceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FlowArn", &self.flow_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", &self.security_group_ids)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetId", &self.subnet_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for FlowVpcInterfaceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<FlowVpcInterfaceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = FlowVpcInterfaceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type FlowVpcInterfaceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut flow_arn: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut security_group_ids: Option<::ValueList<String>> = None;
                let mut subnet_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "FlowArn" => {
                            flow_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecurityGroupIds" => {
                            security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SubnetId" => {
                            subnet_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(FlowVpcInterfaceProperties {
                    flow_arn: flow_arn.ok_or(::serde::de::Error::missing_field("FlowArn"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    security_group_ids: security_group_ids.ok_or(::serde::de::Error::missing_field("SecurityGroupIds"))?,
                    subnet_id: subnet_id.ok_or(::serde::de::Error::missing_field("SubnetId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for FlowVpcInterface {
    type Properties = FlowVpcInterfaceProperties;
    const TYPE: &'static str = "AWS::MediaConnect::FlowVpcInterface";
    fn properties(&self) -> &FlowVpcInterfaceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut FlowVpcInterfaceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for FlowVpcInterface {}

impl From<FlowVpcInterfaceProperties> for FlowVpcInterface {
    fn from(properties: FlowVpcInterfaceProperties) -> FlowVpcInterface {
        FlowVpcInterface { properties }
    }
}

pub mod flow {
    //! Property types for the `Flow` resource.

    /// The [`AWS::MediaConnect::Flow.Encryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-encryption.html) property type.
    #[derive(Debug, Default)]
    pub struct Encryption {
        /// Property [`Algorithm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-encryption.html#cfn-mediaconnect-flow-encryption-algorithm).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub algorithm: ::Value<String>,
        /// Property [`ConstantInitializationVector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-encryption.html#cfn-mediaconnect-flow-encryption-constantinitializationvector).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub constant_initialization_vector: Option<::Value<String>>,
        /// Property [`DeviceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-encryption.html#cfn-mediaconnect-flow-encryption-deviceid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub device_id: Option<::Value<String>>,
        /// Property [`KeyType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-encryption.html#cfn-mediaconnect-flow-encryption-keytype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key_type: Option<::Value<String>>,
        /// Property [`Region`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-encryption.html#cfn-mediaconnect-flow-encryption-region).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub region: Option<::Value<String>>,
        /// Property [`ResourceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-encryption.html#cfn-mediaconnect-flow-encryption-resourceid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_id: Option<::Value<String>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-encryption.html#cfn-mediaconnect-flow-encryption-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`SecretArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-encryption.html#cfn-mediaconnect-flow-encryption-secretarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secret_arn: Option<::Value<String>>,
        /// Property [`Url`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-encryption.html#cfn-mediaconnect-flow-encryption-url).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub url: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Encryption {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Algorithm", &self.algorithm)?;
            if let Some(ref constant_initialization_vector) = self.constant_initialization_vector {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConstantInitializationVector", constant_initialization_vector)?;
            }
            if let Some(ref device_id) = self.device_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceId", device_id)?;
            }
            if let Some(ref key_type) = self.key_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyType", key_type)?;
            }
            if let Some(ref region) = self.region {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Region", region)?;
            }
            if let Some(ref resource_id) = self.resource_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceId", resource_id)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            if let Some(ref secret_arn) = self.secret_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretArn", secret_arn)?;
            }
            if let Some(ref url) = self.url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Url", url)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Encryption {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Encryption, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Encryption;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Encryption")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut algorithm: Option<::Value<String>> = None;
                    let mut constant_initialization_vector: Option<::Value<String>> = None;
                    let mut device_id: Option<::Value<String>> = None;
                    let mut key_type: Option<::Value<String>> = None;
                    let mut region: Option<::Value<String>> = None;
                    let mut resource_id: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut secret_arn: Option<::Value<String>> = None;
                    let mut url: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Algorithm" => {
                                algorithm = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConstantInitializationVector" => {
                                constant_initialization_vector = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeviceId" => {
                                device_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeyType" => {
                                key_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Region" => {
                                region = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceId" => {
                                resource_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretArn" => {
                                secret_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Url" => {
                                url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Encryption {
                        algorithm: algorithm.ok_or(::serde::de::Error::missing_field("Algorithm"))?,
                        constant_initialization_vector: constant_initialization_vector,
                        device_id: device_id,
                        key_type: key_type,
                        region: region,
                        resource_id: resource_id,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                        secret_arn: secret_arn,
                        url: url,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaConnect::Flow.FailoverConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-failoverconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct FailoverConfig {
        /// Property [`RecoveryWindow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-failoverconfig.html#cfn-mediaconnect-flow-failoverconfig-recoverywindow).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub recovery_window: Option<::Value<u32>>,
        /// Property [`State`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-failoverconfig.html#cfn-mediaconnect-flow-failoverconfig-state).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub state: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for FailoverConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref recovery_window) = self.recovery_window {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecoveryWindow", recovery_window)?;
            }
            if let Some(ref state) = self.state {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "State", state)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FailoverConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FailoverConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FailoverConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FailoverConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut recovery_window: Option<::Value<u32>> = None;
                    let mut state: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RecoveryWindow" => {
                                recovery_window = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "State" => {
                                state = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FailoverConfig {
                        recovery_window: recovery_window,
                        state: state,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaConnect::Flow.Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-source.html) property type.
    #[derive(Debug, Default)]
    pub struct Source {
        /// Property [`Decryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-source.html#cfn-mediaconnect-flow-source-decryption).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub decryption: Option<::Value<Encryption>>,
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-source.html#cfn-mediaconnect-flow-source-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`EntitlementArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-source.html#cfn-mediaconnect-flow-source-entitlementarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub entitlement_arn: Option<::Value<String>>,
        /// Property [`IngestIp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-source.html#cfn-mediaconnect-flow-source-ingestip).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ingest_ip: Option<::Value<String>>,
        /// Property [`IngestPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-source.html#cfn-mediaconnect-flow-source-ingestport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ingest_port: Option<::Value<u32>>,
        /// Property [`MaxBitrate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-source.html#cfn-mediaconnect-flow-source-maxbitrate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_bitrate: Option<::Value<u32>>,
        /// Property [`MaxLatency`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-source.html#cfn-mediaconnect-flow-source-maxlatency).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_latency: Option<::Value<u32>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-source.html#cfn-mediaconnect-flow-source-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Protocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-source.html#cfn-mediaconnect-flow-source-protocol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub protocol: Option<::Value<String>>,
        /// Property [`SourceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-source.html#cfn-mediaconnect-flow-source-sourcearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_arn: Option<::Value<String>>,
        /// Property [`StreamId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-source.html#cfn-mediaconnect-flow-source-streamid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stream_id: Option<::Value<String>>,
        /// Property [`VpcInterfaceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-source.html#cfn-mediaconnect-flow-source-vpcinterfacename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vpc_interface_name: Option<::Value<String>>,
        /// Property [`WhitelistCidr`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-source.html#cfn-mediaconnect-flow-source-whitelistcidr).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub whitelist_cidr: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Source {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref decryption) = self.decryption {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Decryption", decryption)?;
            }
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            if let Some(ref entitlement_arn) = self.entitlement_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EntitlementArn", entitlement_arn)?;
            }
            if let Some(ref ingest_ip) = self.ingest_ip {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IngestIp", ingest_ip)?;
            }
            if let Some(ref ingest_port) = self.ingest_port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IngestPort", ingest_port)?;
            }
            if let Some(ref max_bitrate) = self.max_bitrate {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxBitrate", max_bitrate)?;
            }
            if let Some(ref max_latency) = self.max_latency {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxLatency", max_latency)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref protocol) = self.protocol {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocol", protocol)?;
            }
            if let Some(ref source_arn) = self.source_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceArn", source_arn)?;
            }
            if let Some(ref stream_id) = self.stream_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamId", stream_id)?;
            }
            if let Some(ref vpc_interface_name) = self.vpc_interface_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcInterfaceName", vpc_interface_name)?;
            }
            if let Some(ref whitelist_cidr) = self.whitelist_cidr {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WhitelistCidr", whitelist_cidr)?;
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
                    let mut decryption: Option<::Value<Encryption>> = None;
                    let mut description: Option<::Value<String>> = None;
                    let mut entitlement_arn: Option<::Value<String>> = None;
                    let mut ingest_ip: Option<::Value<String>> = None;
                    let mut ingest_port: Option<::Value<u32>> = None;
                    let mut max_bitrate: Option<::Value<u32>> = None;
                    let mut max_latency: Option<::Value<u32>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut protocol: Option<::Value<String>> = None;
                    let mut source_arn: Option<::Value<String>> = None;
                    let mut stream_id: Option<::Value<String>> = None;
                    let mut vpc_interface_name: Option<::Value<String>> = None;
                    let mut whitelist_cidr: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Decryption" => {
                                decryption = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EntitlementArn" => {
                                entitlement_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IngestIp" => {
                                ingest_ip = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IngestPort" => {
                                ingest_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxBitrate" => {
                                max_bitrate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxLatency" => {
                                max_latency = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Protocol" => {
                                protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceArn" => {
                                source_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StreamId" => {
                                stream_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpcInterfaceName" => {
                                vpc_interface_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WhitelistCidr" => {
                                whitelist_cidr = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Source {
                        decryption: decryption,
                        description: description,
                        entitlement_arn: entitlement_arn,
                        ingest_ip: ingest_ip,
                        ingest_port: ingest_port,
                        max_bitrate: max_bitrate,
                        max_latency: max_latency,
                        name: name,
                        protocol: protocol,
                        source_arn: source_arn,
                        stream_id: stream_id,
                        vpc_interface_name: vpc_interface_name,
                        whitelist_cidr: whitelist_cidr,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod flow_entitlement {
    //! Property types for the `FlowEntitlement` resource.

    /// The [`AWS::MediaConnect::FlowEntitlement.Encryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowentitlement-encryption.html) property type.
    #[derive(Debug, Default)]
    pub struct Encryption {
        /// Property [`Algorithm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowentitlement-encryption.html#cfn-mediaconnect-flowentitlement-encryption-algorithm).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub algorithm: ::Value<String>,
        /// Property [`ConstantInitializationVector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowentitlement-encryption.html#cfn-mediaconnect-flowentitlement-encryption-constantinitializationvector).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub constant_initialization_vector: Option<::Value<String>>,
        /// Property [`DeviceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowentitlement-encryption.html#cfn-mediaconnect-flowentitlement-encryption-deviceid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub device_id: Option<::Value<String>>,
        /// Property [`KeyType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowentitlement-encryption.html#cfn-mediaconnect-flowentitlement-encryption-keytype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key_type: Option<::Value<String>>,
        /// Property [`Region`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowentitlement-encryption.html#cfn-mediaconnect-flowentitlement-encryption-region).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub region: Option<::Value<String>>,
        /// Property [`ResourceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowentitlement-encryption.html#cfn-mediaconnect-flowentitlement-encryption-resourceid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_id: Option<::Value<String>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowentitlement-encryption.html#cfn-mediaconnect-flowentitlement-encryption-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`SecretArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowentitlement-encryption.html#cfn-mediaconnect-flowentitlement-encryption-secretarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secret_arn: Option<::Value<String>>,
        /// Property [`Url`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowentitlement-encryption.html#cfn-mediaconnect-flowentitlement-encryption-url).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub url: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Encryption {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Algorithm", &self.algorithm)?;
            if let Some(ref constant_initialization_vector) = self.constant_initialization_vector {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConstantInitializationVector", constant_initialization_vector)?;
            }
            if let Some(ref device_id) = self.device_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceId", device_id)?;
            }
            if let Some(ref key_type) = self.key_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyType", key_type)?;
            }
            if let Some(ref region) = self.region {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Region", region)?;
            }
            if let Some(ref resource_id) = self.resource_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceId", resource_id)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            if let Some(ref secret_arn) = self.secret_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretArn", secret_arn)?;
            }
            if let Some(ref url) = self.url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Url", url)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Encryption {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Encryption, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Encryption;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Encryption")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut algorithm: Option<::Value<String>> = None;
                    let mut constant_initialization_vector: Option<::Value<String>> = None;
                    let mut device_id: Option<::Value<String>> = None;
                    let mut key_type: Option<::Value<String>> = None;
                    let mut region: Option<::Value<String>> = None;
                    let mut resource_id: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut secret_arn: Option<::Value<String>> = None;
                    let mut url: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Algorithm" => {
                                algorithm = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConstantInitializationVector" => {
                                constant_initialization_vector = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeviceId" => {
                                device_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeyType" => {
                                key_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Region" => {
                                region = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceId" => {
                                resource_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretArn" => {
                                secret_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Url" => {
                                url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Encryption {
                        algorithm: algorithm.ok_or(::serde::de::Error::missing_field("Algorithm"))?,
                        constant_initialization_vector: constant_initialization_vector,
                        device_id: device_id,
                        key_type: key_type,
                        region: region,
                        resource_id: resource_id,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                        secret_arn: secret_arn,
                        url: url,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod flow_output {
    //! Property types for the `FlowOutput` resource.

    /// The [`AWS::MediaConnect::FlowOutput.Encryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowoutput-encryption.html) property type.
    #[derive(Debug, Default)]
    pub struct Encryption {
        /// Property [`Algorithm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowoutput-encryption.html#cfn-mediaconnect-flowoutput-encryption-algorithm).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub algorithm: ::Value<String>,
        /// Property [`KeyType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowoutput-encryption.html#cfn-mediaconnect-flowoutput-encryption-keytype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key_type: Option<::Value<String>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowoutput-encryption.html#cfn-mediaconnect-flowoutput-encryption-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`SecretArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowoutput-encryption.html#cfn-mediaconnect-flowoutput-encryption-secretarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secret_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for Encryption {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Algorithm", &self.algorithm)?;
            if let Some(ref key_type) = self.key_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyType", key_type)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretArn", &self.secret_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Encryption {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Encryption, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Encryption;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Encryption")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut algorithm: Option<::Value<String>> = None;
                    let mut key_type: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut secret_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Algorithm" => {
                                algorithm = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeyType" => {
                                key_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretArn" => {
                                secret_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Encryption {
                        algorithm: algorithm.ok_or(::serde::de::Error::missing_field("Algorithm"))?,
                        key_type: key_type,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                        secret_arn: secret_arn.ok_or(::serde::de::Error::missing_field("SecretArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaConnect::FlowOutput.VpcInterfaceAttachment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowoutput-vpcinterfaceattachment.html) property type.
    #[derive(Debug, Default)]
    pub struct VpcInterfaceAttachment {
        /// Property [`VpcInterfaceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowoutput-vpcinterfaceattachment.html#cfn-mediaconnect-flowoutput-vpcinterfaceattachment-vpcinterfacename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vpc_interface_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for VpcInterfaceAttachment {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref vpc_interface_name) = self.vpc_interface_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcInterfaceName", vpc_interface_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VpcInterfaceAttachment {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VpcInterfaceAttachment, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VpcInterfaceAttachment;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VpcInterfaceAttachment")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut vpc_interface_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "VpcInterfaceName" => {
                                vpc_interface_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VpcInterfaceAttachment {
                        vpc_interface_name: vpc_interface_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod flow_source {
    //! Property types for the `FlowSource` resource.

    /// The [`AWS::MediaConnect::FlowSource.Encryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowsource-encryption.html) property type.
    #[derive(Debug, Default)]
    pub struct Encryption {
        /// Property [`Algorithm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowsource-encryption.html#cfn-mediaconnect-flowsource-encryption-algorithm).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub algorithm: ::Value<String>,
        /// Property [`ConstantInitializationVector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowsource-encryption.html#cfn-mediaconnect-flowsource-encryption-constantinitializationvector).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub constant_initialization_vector: Option<::Value<String>>,
        /// Property [`DeviceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowsource-encryption.html#cfn-mediaconnect-flowsource-encryption-deviceid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub device_id: Option<::Value<String>>,
        /// Property [`KeyType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowsource-encryption.html#cfn-mediaconnect-flowsource-encryption-keytype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key_type: Option<::Value<String>>,
        /// Property [`Region`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowsource-encryption.html#cfn-mediaconnect-flowsource-encryption-region).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub region: Option<::Value<String>>,
        /// Property [`ResourceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowsource-encryption.html#cfn-mediaconnect-flowsource-encryption-resourceid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_id: Option<::Value<String>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowsource-encryption.html#cfn-mediaconnect-flowsource-encryption-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`SecretArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowsource-encryption.html#cfn-mediaconnect-flowsource-encryption-secretarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secret_arn: Option<::Value<String>>,
        /// Property [`Url`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowsource-encryption.html#cfn-mediaconnect-flowsource-encryption-url).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub url: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Encryption {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Algorithm", &self.algorithm)?;
            if let Some(ref constant_initialization_vector) = self.constant_initialization_vector {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConstantInitializationVector", constant_initialization_vector)?;
            }
            if let Some(ref device_id) = self.device_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceId", device_id)?;
            }
            if let Some(ref key_type) = self.key_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyType", key_type)?;
            }
            if let Some(ref region) = self.region {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Region", region)?;
            }
            if let Some(ref resource_id) = self.resource_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceId", resource_id)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            if let Some(ref secret_arn) = self.secret_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretArn", secret_arn)?;
            }
            if let Some(ref url) = self.url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Url", url)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Encryption {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Encryption, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Encryption;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Encryption")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut algorithm: Option<::Value<String>> = None;
                    let mut constant_initialization_vector: Option<::Value<String>> = None;
                    let mut device_id: Option<::Value<String>> = None;
                    let mut key_type: Option<::Value<String>> = None;
                    let mut region: Option<::Value<String>> = None;
                    let mut resource_id: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut secret_arn: Option<::Value<String>> = None;
                    let mut url: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Algorithm" => {
                                algorithm = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConstantInitializationVector" => {
                                constant_initialization_vector = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeviceId" => {
                                device_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeyType" => {
                                key_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Region" => {
                                region = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceId" => {
                                resource_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretArn" => {
                                secret_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Url" => {
                                url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Encryption {
                        algorithm: algorithm.ok_or(::serde::de::Error::missing_field("Algorithm"))?,
                        constant_initialization_vector: constant_initialization_vector,
                        device_id: device_id,
                        key_type: key_type,
                        region: region,
                        resource_id: resource_id,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                        secret_arn: secret_arn,
                        url: url,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
