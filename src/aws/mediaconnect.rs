//! Types for the `MediaConnect` service.

/// The [`AWS::MediaConnect::Bridge`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-bridge.html) resource type.
#[derive(Debug, Default)]
pub struct Bridge {
    properties: BridgeProperties
}

/// Properties for the `Bridge` resource.
#[derive(Debug, Default)]
pub struct BridgeProperties {
    /// Property [`EgressGatewayBridge`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-bridge.html#cfn-mediaconnect-bridge-egressgatewaybridge).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub egress_gateway_bridge: Option<::Value<self::bridge::EgressGatewayBridge>>,
    /// Property [`IngressGatewayBridge`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-bridge.html#cfn-mediaconnect-bridge-ingressgatewaybridge).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ingress_gateway_bridge: Option<::Value<self::bridge::IngressGatewayBridge>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-bridge.html#cfn-mediaconnect-bridge-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Outputs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-bridge.html#cfn-mediaconnect-bridge-outputs).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub outputs: Option<::ValueList<self::bridge::BridgeOutput>>,
    /// Property [`PlacementArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-bridge.html#cfn-mediaconnect-bridge-placementarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub placement_arn: ::Value<String>,
    /// Property [`SourceFailoverConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-bridge.html#cfn-mediaconnect-bridge-sourcefailoverconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub source_failover_config: Option<::Value<self::bridge::FailoverConfig>>,
    /// Property [`Sources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-bridge.html#cfn-mediaconnect-bridge-sources).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sources: ::ValueList<self::bridge::BridgeSource>,
}

impl ::serde::Serialize for BridgeProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref egress_gateway_bridge) = self.egress_gateway_bridge {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EgressGatewayBridge", egress_gateway_bridge)?;
        }
        if let Some(ref ingress_gateway_bridge) = self.ingress_gateway_bridge {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IngressGatewayBridge", ingress_gateway_bridge)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref outputs) = self.outputs {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Outputs", outputs)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PlacementArn", &self.placement_arn)?;
        if let Some(ref source_failover_config) = self.source_failover_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceFailoverConfig", source_failover_config)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Sources", &self.sources)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for BridgeProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<BridgeProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = BridgeProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type BridgeProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut egress_gateway_bridge: Option<::Value<self::bridge::EgressGatewayBridge>> = None;
                let mut ingress_gateway_bridge: Option<::Value<self::bridge::IngressGatewayBridge>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut outputs: Option<::ValueList<self::bridge::BridgeOutput>> = None;
                let mut placement_arn: Option<::Value<String>> = None;
                let mut source_failover_config: Option<::Value<self::bridge::FailoverConfig>> = None;
                let mut sources: Option<::ValueList<self::bridge::BridgeSource>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "EgressGatewayBridge" => {
                            egress_gateway_bridge = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IngressGatewayBridge" => {
                            ingress_gateway_bridge = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Outputs" => {
                            outputs = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PlacementArn" => {
                            placement_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceFailoverConfig" => {
                            source_failover_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Sources" => {
                            sources = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(BridgeProperties {
                    egress_gateway_bridge: egress_gateway_bridge,
                    ingress_gateway_bridge: ingress_gateway_bridge,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    outputs: outputs,
                    placement_arn: placement_arn.ok_or(::serde::de::Error::missing_field("PlacementArn"))?,
                    source_failover_config: source_failover_config,
                    sources: sources.ok_or(::serde::de::Error::missing_field("Sources"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Bridge {
    type Properties = BridgeProperties;
    const TYPE: &'static str = "AWS::MediaConnect::Bridge";
    fn properties(&self) -> &BridgeProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut BridgeProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Bridge {}

impl From<BridgeProperties> for Bridge {
    fn from(properties: BridgeProperties) -> Bridge {
        Bridge { properties }
    }
}

/// The [`AWS::MediaConnect::BridgeOutput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-bridgeoutput.html) resource type.
#[derive(Debug, Default)]
pub struct BridgeOutput {
    properties: BridgeOutputProperties
}

/// Properties for the `BridgeOutput` resource.
#[derive(Debug, Default)]
pub struct BridgeOutputProperties {
    /// Property [`BridgeArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-bridgeoutput.html#cfn-mediaconnect-bridgeoutput-bridgearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub bridge_arn: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-bridgeoutput.html#cfn-mediaconnect-bridgeoutput-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`NetworkOutput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-bridgeoutput.html#cfn-mediaconnect-bridgeoutput-networkoutput).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub network_output: ::Value<self::bridge_output::BridgeNetworkOutput>,
}

impl ::serde::Serialize for BridgeOutputProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BridgeArn", &self.bridge_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkOutput", &self.network_output)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for BridgeOutputProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<BridgeOutputProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = BridgeOutputProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type BridgeOutputProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut bridge_arn: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut network_output: Option<::Value<self::bridge_output::BridgeNetworkOutput>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "BridgeArn" => {
                            bridge_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NetworkOutput" => {
                            network_output = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(BridgeOutputProperties {
                    bridge_arn: bridge_arn.ok_or(::serde::de::Error::missing_field("BridgeArn"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    network_output: network_output.ok_or(::serde::de::Error::missing_field("NetworkOutput"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for BridgeOutput {
    type Properties = BridgeOutputProperties;
    const TYPE: &'static str = "AWS::MediaConnect::BridgeOutput";
    fn properties(&self) -> &BridgeOutputProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut BridgeOutputProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for BridgeOutput {}

impl From<BridgeOutputProperties> for BridgeOutput {
    fn from(properties: BridgeOutputProperties) -> BridgeOutput {
        BridgeOutput { properties }
    }
}

/// The [`AWS::MediaConnect::BridgeSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-bridgesource.html) resource type.
#[derive(Debug, Default)]
pub struct BridgeSource {
    properties: BridgeSourceProperties
}

/// Properties for the `BridgeSource` resource.
#[derive(Debug, Default)]
pub struct BridgeSourceProperties {
    /// Property [`BridgeArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-bridgesource.html#cfn-mediaconnect-bridgesource-bridgearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub bridge_arn: ::Value<String>,
    /// Property [`FlowSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-bridgesource.html#cfn-mediaconnect-bridgesource-flowsource).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub flow_source: Option<::Value<self::bridge_source::BridgeFlowSource>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-bridgesource.html#cfn-mediaconnect-bridgesource-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`NetworkSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-bridgesource.html#cfn-mediaconnect-bridgesource-networksource).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub network_source: Option<::Value<self::bridge_source::BridgeNetworkSource>>,
}

impl ::serde::Serialize for BridgeSourceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BridgeArn", &self.bridge_arn)?;
        if let Some(ref flow_source) = self.flow_source {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FlowSource", flow_source)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref network_source) = self.network_source {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkSource", network_source)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for BridgeSourceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<BridgeSourceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = BridgeSourceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type BridgeSourceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut bridge_arn: Option<::Value<String>> = None;
                let mut flow_source: Option<::Value<self::bridge_source::BridgeFlowSource>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut network_source: Option<::Value<self::bridge_source::BridgeNetworkSource>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "BridgeArn" => {
                            bridge_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FlowSource" => {
                            flow_source = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NetworkSource" => {
                            network_source = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(BridgeSourceProperties {
                    bridge_arn: bridge_arn.ok_or(::serde::de::Error::missing_field("BridgeArn"))?,
                    flow_source: flow_source,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    network_source: network_source,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for BridgeSource {
    type Properties = BridgeSourceProperties;
    const TYPE: &'static str = "AWS::MediaConnect::BridgeSource";
    fn properties(&self) -> &BridgeSourceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut BridgeSourceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for BridgeSource {}

impl From<BridgeSourceProperties> for BridgeSource {
    fn from(properties: BridgeSourceProperties) -> BridgeSource {
        BridgeSource { properties }
    }
}

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
    /// Property [`MinLatency`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowoutput.html#cfn-mediaconnect-flowoutput-minlatency).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub min_latency: Option<::Value<u32>>,
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
        if let Some(ref min_latency) = self.min_latency {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinLatency", min_latency)?;
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
                let mut min_latency: Option<::Value<u32>> = None;
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
                        "MinLatency" => {
                            min_latency = ::serde::de::MapAccess::next_value(&mut map)?;
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
                    min_latency: min_latency,
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
    /// Property [`GatewayBridgeSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowsource.html#cfn-mediaconnect-flowsource-gatewaybridgesource).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub gateway_bridge_source: Option<::Value<self::flow_source::GatewayBridgeSource>>,
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
    /// Property [`MinLatency`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowsource.html#cfn-mediaconnect-flowsource-minlatency).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub min_latency: Option<::Value<u32>>,
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
    /// Property [`SenderControlPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowsource.html#cfn-mediaconnect-flowsource-sendercontrolport).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sender_control_port: Option<::Value<u32>>,
    /// Property [`SenderIpAddress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowsource.html#cfn-mediaconnect-flowsource-senderipaddress).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sender_ip_address: Option<::Value<String>>,
    /// Property [`SourceListenerAddress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowsource.html#cfn-mediaconnect-flowsource-sourcelisteneraddress).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub source_listener_address: Option<::Value<String>>,
    /// Property [`SourceListenerPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-flowsource.html#cfn-mediaconnect-flowsource-sourcelistenerport).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub source_listener_port: Option<::Value<u32>>,
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
        if let Some(ref gateway_bridge_source) = self.gateway_bridge_source {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GatewayBridgeSource", gateway_bridge_source)?;
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
        if let Some(ref min_latency) = self.min_latency {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinLatency", min_latency)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref protocol) = self.protocol {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocol", protocol)?;
        }
        if let Some(ref sender_control_port) = self.sender_control_port {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SenderControlPort", sender_control_port)?;
        }
        if let Some(ref sender_ip_address) = self.sender_ip_address {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SenderIpAddress", sender_ip_address)?;
        }
        if let Some(ref source_listener_address) = self.source_listener_address {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceListenerAddress", source_listener_address)?;
        }
        if let Some(ref source_listener_port) = self.source_listener_port {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceListenerPort", source_listener_port)?;
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
                let mut gateway_bridge_source: Option<::Value<self::flow_source::GatewayBridgeSource>> = None;
                let mut ingest_port: Option<::Value<u32>> = None;
                let mut max_bitrate: Option<::Value<u32>> = None;
                let mut max_latency: Option<::Value<u32>> = None;
                let mut min_latency: Option<::Value<u32>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut protocol: Option<::Value<String>> = None;
                let mut sender_control_port: Option<::Value<u32>> = None;
                let mut sender_ip_address: Option<::Value<String>> = None;
                let mut source_listener_address: Option<::Value<String>> = None;
                let mut source_listener_port: Option<::Value<u32>> = None;
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
                        "GatewayBridgeSource" => {
                            gateway_bridge_source = ::serde::de::MapAccess::next_value(&mut map)?;
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
                        "MinLatency" => {
                            min_latency = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Protocol" => {
                            protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SenderControlPort" => {
                            sender_control_port = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SenderIpAddress" => {
                            sender_ip_address = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceListenerAddress" => {
                            source_listener_address = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceListenerPort" => {
                            source_listener_port = ::serde::de::MapAccess::next_value(&mut map)?;
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
                    gateway_bridge_source: gateway_bridge_source,
                    ingest_port: ingest_port,
                    max_bitrate: max_bitrate,
                    max_latency: max_latency,
                    min_latency: min_latency,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    protocol: protocol,
                    sender_control_port: sender_control_port,
                    sender_ip_address: sender_ip_address,
                    source_listener_address: source_listener_address,
                    source_listener_port: source_listener_port,
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

/// The [`AWS::MediaConnect::Gateway`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-gateway.html) resource type.
#[derive(Debug, Default)]
pub struct Gateway {
    properties: GatewayProperties
}

/// Properties for the `Gateway` resource.
#[derive(Debug, Default)]
pub struct GatewayProperties {
    /// Property [`EgressCidrBlocks`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-gateway.html#cfn-mediaconnect-gateway-egresscidrblocks).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub egress_cidr_blocks: ::ValueList<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-gateway.html#cfn-mediaconnect-gateway-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Networks`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconnect-gateway.html#cfn-mediaconnect-gateway-networks).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub networks: ::ValueList<self::gateway::GatewayNetwork>,
}

impl ::serde::Serialize for GatewayProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EgressCidrBlocks", &self.egress_cidr_blocks)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Networks", &self.networks)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for GatewayProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<GatewayProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = GatewayProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type GatewayProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut egress_cidr_blocks: Option<::ValueList<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut networks: Option<::ValueList<self::gateway::GatewayNetwork>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "EgressCidrBlocks" => {
                            egress_cidr_blocks = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Networks" => {
                            networks = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(GatewayProperties {
                    egress_cidr_blocks: egress_cidr_blocks.ok_or(::serde::de::Error::missing_field("EgressCidrBlocks"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    networks: networks.ok_or(::serde::de::Error::missing_field("Networks"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Gateway {
    type Properties = GatewayProperties;
    const TYPE: &'static str = "AWS::MediaConnect::Gateway";
    fn properties(&self) -> &GatewayProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut GatewayProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Gateway {}

impl From<GatewayProperties> for Gateway {
    fn from(properties: GatewayProperties) -> Gateway {
        Gateway { properties }
    }
}

pub mod bridge {
    //! Property types for the `Bridge` resource.

    /// The [`AWS::MediaConnect::Bridge.BridgeFlowSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-bridgeflowsource.html) property type.
    #[derive(Debug, Default)]
    pub struct BridgeFlowSource {
        /// Property [`FlowArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-bridgeflowsource.html#cfn-mediaconnect-bridge-bridgeflowsource-flowarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub flow_arn: ::Value<String>,
        /// Property [`FlowVpcInterfaceAttachment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-bridgeflowsource.html#cfn-mediaconnect-bridge-bridgeflowsource-flowvpcinterfaceattachment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub flow_vpc_interface_attachment: Option<::Value<VpcInterfaceAttachment>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-bridgeflowsource.html#cfn-mediaconnect-bridge-bridgeflowsource-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for BridgeFlowSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FlowArn", &self.flow_arn)?;
            if let Some(ref flow_vpc_interface_attachment) = self.flow_vpc_interface_attachment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FlowVpcInterfaceAttachment", flow_vpc_interface_attachment)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BridgeFlowSource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BridgeFlowSource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BridgeFlowSource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BridgeFlowSource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut flow_arn: Option<::Value<String>> = None;
                    let mut flow_vpc_interface_attachment: Option<::Value<VpcInterfaceAttachment>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FlowArn" => {
                                flow_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FlowVpcInterfaceAttachment" => {
                                flow_vpc_interface_attachment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BridgeFlowSource {
                        flow_arn: flow_arn.ok_or(::serde::de::Error::missing_field("FlowArn"))?,
                        flow_vpc_interface_attachment: flow_vpc_interface_attachment,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaConnect::Bridge.BridgeNetworkOutput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-bridgenetworkoutput.html) property type.
    #[derive(Debug, Default)]
    pub struct BridgeNetworkOutput {
        /// Property [`IpAddress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-bridgenetworkoutput.html#cfn-mediaconnect-bridge-bridgenetworkoutput-ipaddress).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ip_address: ::Value<String>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-bridgenetworkoutput.html#cfn-mediaconnect-bridge-bridgenetworkoutput-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`NetworkName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-bridgenetworkoutput.html#cfn-mediaconnect-bridge-bridgenetworkoutput-networkname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub network_name: ::Value<String>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-bridgenetworkoutput.html#cfn-mediaconnect-bridge-bridgenetworkoutput-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: ::Value<u32>,
        /// Property [`Protocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-bridgenetworkoutput.html#cfn-mediaconnect-bridge-bridgenetworkoutput-protocol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub protocol: ::Value<String>,
        /// Property [`Ttl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-bridgenetworkoutput.html#cfn-mediaconnect-bridge-bridgenetworkoutput-ttl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ttl: ::Value<u32>,
    }

    impl ::codec::SerializeValue for BridgeNetworkOutput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IpAddress", &self.ip_address)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkName", &self.network_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", &self.port)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocol", &self.protocol)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ttl", &self.ttl)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BridgeNetworkOutput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BridgeNetworkOutput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BridgeNetworkOutput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BridgeNetworkOutput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ip_address: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut network_name: Option<::Value<String>> = None;
                    let mut port: Option<::Value<u32>> = None;
                    let mut protocol: Option<::Value<String>> = None;
                    let mut ttl: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "IpAddress" => {
                                ip_address = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NetworkName" => {
                                network_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Protocol" => {
                                protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Ttl" => {
                                ttl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BridgeNetworkOutput {
                        ip_address: ip_address.ok_or(::serde::de::Error::missing_field("IpAddress"))?,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        network_name: network_name.ok_or(::serde::de::Error::missing_field("NetworkName"))?,
                        port: port.ok_or(::serde::de::Error::missing_field("Port"))?,
                        protocol: protocol.ok_or(::serde::de::Error::missing_field("Protocol"))?,
                        ttl: ttl.ok_or(::serde::de::Error::missing_field("Ttl"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaConnect::Bridge.BridgeNetworkSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-bridgenetworksource.html) property type.
    #[derive(Debug, Default)]
    pub struct BridgeNetworkSource {
        /// Property [`MulticastIp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-bridgenetworksource.html#cfn-mediaconnect-bridge-bridgenetworksource-multicastip).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub multicast_ip: ::Value<String>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-bridgenetworksource.html#cfn-mediaconnect-bridge-bridgenetworksource-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`NetworkName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-bridgenetworksource.html#cfn-mediaconnect-bridge-bridgenetworksource-networkname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub network_name: ::Value<String>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-bridgenetworksource.html#cfn-mediaconnect-bridge-bridgenetworksource-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: ::Value<u32>,
        /// Property [`Protocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-bridgenetworksource.html#cfn-mediaconnect-bridge-bridgenetworksource-protocol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub protocol: ::Value<String>,
    }

    impl ::codec::SerializeValue for BridgeNetworkSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MulticastIp", &self.multicast_ip)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkName", &self.network_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", &self.port)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocol", &self.protocol)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BridgeNetworkSource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BridgeNetworkSource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BridgeNetworkSource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BridgeNetworkSource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut multicast_ip: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut network_name: Option<::Value<String>> = None;
                    let mut port: Option<::Value<u32>> = None;
                    let mut protocol: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MulticastIp" => {
                                multicast_ip = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NetworkName" => {
                                network_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Protocol" => {
                                protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BridgeNetworkSource {
                        multicast_ip: multicast_ip.ok_or(::serde::de::Error::missing_field("MulticastIp"))?,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        network_name: network_name.ok_or(::serde::de::Error::missing_field("NetworkName"))?,
                        port: port.ok_or(::serde::de::Error::missing_field("Port"))?,
                        protocol: protocol.ok_or(::serde::de::Error::missing_field("Protocol"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaConnect::Bridge.BridgeOutput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-bridgeoutput.html) property type.
    #[derive(Debug, Default)]
    pub struct BridgeOutput {
        /// Property [`NetworkOutput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-bridgeoutput.html#cfn-mediaconnect-bridge-bridgeoutput-networkoutput).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub network_output: Option<::Value<BridgeNetworkOutput>>,
    }

    impl ::codec::SerializeValue for BridgeOutput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref network_output) = self.network_output {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkOutput", network_output)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BridgeOutput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BridgeOutput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BridgeOutput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BridgeOutput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut network_output: Option<::Value<BridgeNetworkOutput>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "NetworkOutput" => {
                                network_output = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BridgeOutput {
                        network_output: network_output,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaConnect::Bridge.BridgeSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-bridgesource.html) property type.
    #[derive(Debug, Default)]
    pub struct BridgeSource {
        /// Property [`FlowSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-bridgesource.html#cfn-mediaconnect-bridge-bridgesource-flowsource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub flow_source: Option<::Value<BridgeFlowSource>>,
        /// Property [`NetworkSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-bridgesource.html#cfn-mediaconnect-bridge-bridgesource-networksource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub network_source: Option<::Value<BridgeNetworkSource>>,
    }

    impl ::codec::SerializeValue for BridgeSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref flow_source) = self.flow_source {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FlowSource", flow_source)?;
            }
            if let Some(ref network_source) = self.network_source {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkSource", network_source)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BridgeSource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BridgeSource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BridgeSource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BridgeSource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut flow_source: Option<::Value<BridgeFlowSource>> = None;
                    let mut network_source: Option<::Value<BridgeNetworkSource>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FlowSource" => {
                                flow_source = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NetworkSource" => {
                                network_source = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BridgeSource {
                        flow_source: flow_source,
                        network_source: network_source,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaConnect::Bridge.EgressGatewayBridge`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-egressgatewaybridge.html) property type.
    #[derive(Debug, Default)]
    pub struct EgressGatewayBridge {
        /// Property [`MaxBitrate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-egressgatewaybridge.html#cfn-mediaconnect-bridge-egressgatewaybridge-maxbitrate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_bitrate: ::Value<u32>,
    }

    impl ::codec::SerializeValue for EgressGatewayBridge {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxBitrate", &self.max_bitrate)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EgressGatewayBridge {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EgressGatewayBridge, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EgressGatewayBridge;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EgressGatewayBridge")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut max_bitrate: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxBitrate" => {
                                max_bitrate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EgressGatewayBridge {
                        max_bitrate: max_bitrate.ok_or(::serde::de::Error::missing_field("MaxBitrate"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaConnect::Bridge.FailoverConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-failoverconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct FailoverConfig {
        /// Property [`FailoverMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-failoverconfig.html#cfn-mediaconnect-bridge-failoverconfig-failovermode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub failover_mode: ::Value<String>,
        /// Property [`SourcePriority`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-failoverconfig.html#cfn-mediaconnect-bridge-failoverconfig-sourcepriority).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_priority: Option<::Value<SourcePriority>>,
        /// Property [`State`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-failoverconfig.html#cfn-mediaconnect-bridge-failoverconfig-state).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub state: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for FailoverConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FailoverMode", &self.failover_mode)?;
            if let Some(ref source_priority) = self.source_priority {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourcePriority", source_priority)?;
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
                    let mut failover_mode: Option<::Value<String>> = None;
                    let mut source_priority: Option<::Value<SourcePriority>> = None;
                    let mut state: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FailoverMode" => {
                                failover_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourcePriority" => {
                                source_priority = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "State" => {
                                state = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FailoverConfig {
                        failover_mode: failover_mode.ok_or(::serde::de::Error::missing_field("FailoverMode"))?,
                        source_priority: source_priority,
                        state: state,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaConnect::Bridge.IngressGatewayBridge`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-ingressgatewaybridge.html) property type.
    #[derive(Debug, Default)]
    pub struct IngressGatewayBridge {
        /// Property [`MaxBitrate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-ingressgatewaybridge.html#cfn-mediaconnect-bridge-ingressgatewaybridge-maxbitrate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_bitrate: ::Value<u32>,
        /// Property [`MaxOutputs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-ingressgatewaybridge.html#cfn-mediaconnect-bridge-ingressgatewaybridge-maxoutputs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_outputs: ::Value<u32>,
    }

    impl ::codec::SerializeValue for IngressGatewayBridge {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxBitrate", &self.max_bitrate)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxOutputs", &self.max_outputs)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IngressGatewayBridge {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IngressGatewayBridge, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IngressGatewayBridge;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IngressGatewayBridge")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut max_bitrate: Option<::Value<u32>> = None;
                    let mut max_outputs: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxBitrate" => {
                                max_bitrate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxOutputs" => {
                                max_outputs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IngressGatewayBridge {
                        max_bitrate: max_bitrate.ok_or(::serde::de::Error::missing_field("MaxBitrate"))?,
                        max_outputs: max_outputs.ok_or(::serde::de::Error::missing_field("MaxOutputs"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaConnect::Bridge.SourcePriority`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-sourcepriority.html) property type.
    #[derive(Debug, Default)]
    pub struct SourcePriority {
        /// Property [`PrimarySource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-sourcepriority.html#cfn-mediaconnect-bridge-sourcepriority-primarysource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub primary_source: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SourcePriority {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref primary_source) = self.primary_source {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrimarySource", primary_source)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SourcePriority {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SourcePriority, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SourcePriority;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SourcePriority")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut primary_source: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PrimarySource" => {
                                primary_source = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SourcePriority {
                        primary_source: primary_source,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaConnect::Bridge.VpcInterfaceAttachment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-vpcinterfaceattachment.html) property type.
    #[derive(Debug, Default)]
    pub struct VpcInterfaceAttachment {
        /// Property [`VpcInterfaceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridge-vpcinterfaceattachment.html#cfn-mediaconnect-bridge-vpcinterfaceattachment-vpcinterfacename).
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

pub mod bridge_output {
    //! Property types for the `BridgeOutput` resource.

    /// The [`AWS::MediaConnect::BridgeOutput.BridgeNetworkOutput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridgeoutput-bridgenetworkoutput.html) property type.
    #[derive(Debug, Default)]
    pub struct BridgeNetworkOutput {
        /// Property [`IpAddress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridgeoutput-bridgenetworkoutput.html#cfn-mediaconnect-bridgeoutput-bridgenetworkoutput-ipaddress).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ip_address: ::Value<String>,
        /// Property [`NetworkName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridgeoutput-bridgenetworkoutput.html#cfn-mediaconnect-bridgeoutput-bridgenetworkoutput-networkname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub network_name: ::Value<String>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridgeoutput-bridgenetworkoutput.html#cfn-mediaconnect-bridgeoutput-bridgenetworkoutput-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: ::Value<u32>,
        /// Property [`Protocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridgeoutput-bridgenetworkoutput.html#cfn-mediaconnect-bridgeoutput-bridgenetworkoutput-protocol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub protocol: ::Value<String>,
        /// Property [`Ttl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridgeoutput-bridgenetworkoutput.html#cfn-mediaconnect-bridgeoutput-bridgenetworkoutput-ttl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ttl: ::Value<u32>,
    }

    impl ::codec::SerializeValue for BridgeNetworkOutput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IpAddress", &self.ip_address)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkName", &self.network_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", &self.port)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocol", &self.protocol)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ttl", &self.ttl)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BridgeNetworkOutput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BridgeNetworkOutput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BridgeNetworkOutput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BridgeNetworkOutput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ip_address: Option<::Value<String>> = None;
                    let mut network_name: Option<::Value<String>> = None;
                    let mut port: Option<::Value<u32>> = None;
                    let mut protocol: Option<::Value<String>> = None;
                    let mut ttl: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "IpAddress" => {
                                ip_address = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NetworkName" => {
                                network_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Protocol" => {
                                protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Ttl" => {
                                ttl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BridgeNetworkOutput {
                        ip_address: ip_address.ok_or(::serde::de::Error::missing_field("IpAddress"))?,
                        network_name: network_name.ok_or(::serde::de::Error::missing_field("NetworkName"))?,
                        port: port.ok_or(::serde::de::Error::missing_field("Port"))?,
                        protocol: protocol.ok_or(::serde::de::Error::missing_field("Protocol"))?,
                        ttl: ttl.ok_or(::serde::de::Error::missing_field("Ttl"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod bridge_source {
    //! Property types for the `BridgeSource` resource.

    /// The [`AWS::MediaConnect::BridgeSource.BridgeFlowSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridgesource-bridgeflowsource.html) property type.
    #[derive(Debug, Default)]
    pub struct BridgeFlowSource {
        /// Property [`FlowArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridgesource-bridgeflowsource.html#cfn-mediaconnect-bridgesource-bridgeflowsource-flowarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub flow_arn: ::Value<String>,
        /// Property [`FlowVpcInterfaceAttachment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridgesource-bridgeflowsource.html#cfn-mediaconnect-bridgesource-bridgeflowsource-flowvpcinterfaceattachment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub flow_vpc_interface_attachment: Option<::Value<VpcInterfaceAttachment>>,
    }

    impl ::codec::SerializeValue for BridgeFlowSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FlowArn", &self.flow_arn)?;
            if let Some(ref flow_vpc_interface_attachment) = self.flow_vpc_interface_attachment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FlowVpcInterfaceAttachment", flow_vpc_interface_attachment)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BridgeFlowSource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BridgeFlowSource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BridgeFlowSource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BridgeFlowSource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut flow_arn: Option<::Value<String>> = None;
                    let mut flow_vpc_interface_attachment: Option<::Value<VpcInterfaceAttachment>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FlowArn" => {
                                flow_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FlowVpcInterfaceAttachment" => {
                                flow_vpc_interface_attachment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BridgeFlowSource {
                        flow_arn: flow_arn.ok_or(::serde::de::Error::missing_field("FlowArn"))?,
                        flow_vpc_interface_attachment: flow_vpc_interface_attachment,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaConnect::BridgeSource.BridgeNetworkSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridgesource-bridgenetworksource.html) property type.
    #[derive(Debug, Default)]
    pub struct BridgeNetworkSource {
        /// Property [`MulticastIp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridgesource-bridgenetworksource.html#cfn-mediaconnect-bridgesource-bridgenetworksource-multicastip).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub multicast_ip: ::Value<String>,
        /// Property [`NetworkName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridgesource-bridgenetworksource.html#cfn-mediaconnect-bridgesource-bridgenetworksource-networkname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub network_name: ::Value<String>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridgesource-bridgenetworksource.html#cfn-mediaconnect-bridgesource-bridgenetworksource-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: ::Value<u32>,
        /// Property [`Protocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridgesource-bridgenetworksource.html#cfn-mediaconnect-bridgesource-bridgenetworksource-protocol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub protocol: ::Value<String>,
    }

    impl ::codec::SerializeValue for BridgeNetworkSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MulticastIp", &self.multicast_ip)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkName", &self.network_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", &self.port)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocol", &self.protocol)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BridgeNetworkSource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BridgeNetworkSource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BridgeNetworkSource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BridgeNetworkSource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut multicast_ip: Option<::Value<String>> = None;
                    let mut network_name: Option<::Value<String>> = None;
                    let mut port: Option<::Value<u32>> = None;
                    let mut protocol: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MulticastIp" => {
                                multicast_ip = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NetworkName" => {
                                network_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Protocol" => {
                                protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BridgeNetworkSource {
                        multicast_ip: multicast_ip.ok_or(::serde::de::Error::missing_field("MulticastIp"))?,
                        network_name: network_name.ok_or(::serde::de::Error::missing_field("NetworkName"))?,
                        port: port.ok_or(::serde::de::Error::missing_field("Port"))?,
                        protocol: protocol.ok_or(::serde::de::Error::missing_field("Protocol"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaConnect::BridgeSource.VpcInterfaceAttachment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridgesource-vpcinterfaceattachment.html) property type.
    #[derive(Debug, Default)]
    pub struct VpcInterfaceAttachment {
        /// Property [`VpcInterfaceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-bridgesource-vpcinterfaceattachment.html#cfn-mediaconnect-bridgesource-vpcinterfaceattachment-vpcinterfacename).
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

pub mod flow {
    //! Property types for the `Flow` resource.

    /// The [`AWS::MediaConnect::Flow.Encryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-encryption.html) property type.
    #[derive(Debug, Default)]
    pub struct Encryption {
        /// Property [`Algorithm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-encryption.html#cfn-mediaconnect-flow-encryption-algorithm).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub algorithm: Option<::Value<String>>,
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
            if let Some(ref algorithm) = self.algorithm {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Algorithm", algorithm)?;
            }
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
                        algorithm: algorithm,
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
        /// Property [`FailoverMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-failoverconfig.html#cfn-mediaconnect-flow-failoverconfig-failovermode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub failover_mode: Option<::Value<String>>,
        /// Property [`RecoveryWindow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-failoverconfig.html#cfn-mediaconnect-flow-failoverconfig-recoverywindow).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub recovery_window: Option<::Value<u32>>,
        /// Property [`SourcePriority`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-failoverconfig.html#cfn-mediaconnect-flow-failoverconfig-sourcepriority).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_priority: Option<::Value<SourcePriority>>,
        /// Property [`State`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-failoverconfig.html#cfn-mediaconnect-flow-failoverconfig-state).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub state: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for FailoverConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref failover_mode) = self.failover_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FailoverMode", failover_mode)?;
            }
            if let Some(ref recovery_window) = self.recovery_window {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecoveryWindow", recovery_window)?;
            }
            if let Some(ref source_priority) = self.source_priority {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourcePriority", source_priority)?;
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
                    let mut failover_mode: Option<::Value<String>> = None;
                    let mut recovery_window: Option<::Value<u32>> = None;
                    let mut source_priority: Option<::Value<SourcePriority>> = None;
                    let mut state: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FailoverMode" => {
                                failover_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RecoveryWindow" => {
                                recovery_window = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourcePriority" => {
                                source_priority = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "State" => {
                                state = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FailoverConfig {
                        failover_mode: failover_mode,
                        recovery_window: recovery_window,
                        source_priority: source_priority,
                        state: state,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaConnect::Flow.GatewayBridgeSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-gatewaybridgesource.html) property type.
    #[derive(Debug, Default)]
    pub struct GatewayBridgeSource {
        /// Property [`BridgeArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-gatewaybridgesource.html#cfn-mediaconnect-flow-gatewaybridgesource-bridgearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bridge_arn: ::Value<String>,
        /// Property [`VpcInterfaceAttachment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-gatewaybridgesource.html#cfn-mediaconnect-flow-gatewaybridgesource-vpcinterfaceattachment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vpc_interface_attachment: Option<::Value<VpcInterfaceAttachment>>,
    }

    impl ::codec::SerializeValue for GatewayBridgeSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BridgeArn", &self.bridge_arn)?;
            if let Some(ref vpc_interface_attachment) = self.vpc_interface_attachment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcInterfaceAttachment", vpc_interface_attachment)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GatewayBridgeSource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GatewayBridgeSource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GatewayBridgeSource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GatewayBridgeSource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bridge_arn: Option<::Value<String>> = None;
                    let mut vpc_interface_attachment: Option<::Value<VpcInterfaceAttachment>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BridgeArn" => {
                                bridge_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpcInterfaceAttachment" => {
                                vpc_interface_attachment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GatewayBridgeSource {
                        bridge_arn: bridge_arn.ok_or(::serde::de::Error::missing_field("BridgeArn"))?,
                        vpc_interface_attachment: vpc_interface_attachment,
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
        /// Property [`GatewayBridgeSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-source.html#cfn-mediaconnect-flow-source-gatewaybridgesource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub gateway_bridge_source: Option<::Value<GatewayBridgeSource>>,
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
        /// Property [`MinLatency`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-source.html#cfn-mediaconnect-flow-source-minlatency).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min_latency: Option<::Value<u32>>,
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
        /// Property [`SenderControlPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-source.html#cfn-mediaconnect-flow-source-sendercontrolport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sender_control_port: Option<::Value<u32>>,
        /// Property [`SenderIpAddress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-source.html#cfn-mediaconnect-flow-source-senderipaddress).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sender_ip_address: Option<::Value<String>>,
        /// Property [`SourceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-source.html#cfn-mediaconnect-flow-source-sourcearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_arn: Option<::Value<String>>,
        /// Property [`SourceIngestPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-source.html#cfn-mediaconnect-flow-source-sourceingestport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_ingest_port: Option<::Value<String>>,
        /// Property [`SourceListenerAddress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-source.html#cfn-mediaconnect-flow-source-sourcelisteneraddress).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_listener_address: Option<::Value<String>>,
        /// Property [`SourceListenerPort`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-source.html#cfn-mediaconnect-flow-source-sourcelistenerport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_listener_port: Option<::Value<u32>>,
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
            if let Some(ref gateway_bridge_source) = self.gateway_bridge_source {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GatewayBridgeSource", gateway_bridge_source)?;
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
            if let Some(ref min_latency) = self.min_latency {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinLatency", min_latency)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref protocol) = self.protocol {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocol", protocol)?;
            }
            if let Some(ref sender_control_port) = self.sender_control_port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SenderControlPort", sender_control_port)?;
            }
            if let Some(ref sender_ip_address) = self.sender_ip_address {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SenderIpAddress", sender_ip_address)?;
            }
            if let Some(ref source_arn) = self.source_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceArn", source_arn)?;
            }
            if let Some(ref source_ingest_port) = self.source_ingest_port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceIngestPort", source_ingest_port)?;
            }
            if let Some(ref source_listener_address) = self.source_listener_address {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceListenerAddress", source_listener_address)?;
            }
            if let Some(ref source_listener_port) = self.source_listener_port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceListenerPort", source_listener_port)?;
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
                    let mut gateway_bridge_source: Option<::Value<GatewayBridgeSource>> = None;
                    let mut ingest_ip: Option<::Value<String>> = None;
                    let mut ingest_port: Option<::Value<u32>> = None;
                    let mut max_bitrate: Option<::Value<u32>> = None;
                    let mut max_latency: Option<::Value<u32>> = None;
                    let mut min_latency: Option<::Value<u32>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut protocol: Option<::Value<String>> = None;
                    let mut sender_control_port: Option<::Value<u32>> = None;
                    let mut sender_ip_address: Option<::Value<String>> = None;
                    let mut source_arn: Option<::Value<String>> = None;
                    let mut source_ingest_port: Option<::Value<String>> = None;
                    let mut source_listener_address: Option<::Value<String>> = None;
                    let mut source_listener_port: Option<::Value<u32>> = None;
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
                            "GatewayBridgeSource" => {
                                gateway_bridge_source = ::serde::de::MapAccess::next_value(&mut map)?;
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
                            "MinLatency" => {
                                min_latency = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Protocol" => {
                                protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SenderControlPort" => {
                                sender_control_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SenderIpAddress" => {
                                sender_ip_address = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceArn" => {
                                source_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceIngestPort" => {
                                source_ingest_port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceListenerAddress" => {
                                source_listener_address = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceListenerPort" => {
                                source_listener_port = ::serde::de::MapAccess::next_value(&mut map)?;
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
                        gateway_bridge_source: gateway_bridge_source,
                        ingest_ip: ingest_ip,
                        ingest_port: ingest_port,
                        max_bitrate: max_bitrate,
                        max_latency: max_latency,
                        min_latency: min_latency,
                        name: name,
                        protocol: protocol,
                        sender_control_port: sender_control_port,
                        sender_ip_address: sender_ip_address,
                        source_arn: source_arn,
                        source_ingest_port: source_ingest_port,
                        source_listener_address: source_listener_address,
                        source_listener_port: source_listener_port,
                        stream_id: stream_id,
                        vpc_interface_name: vpc_interface_name,
                        whitelist_cidr: whitelist_cidr,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaConnect::Flow.SourcePriority`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-sourcepriority.html) property type.
    #[derive(Debug, Default)]
    pub struct SourcePriority {
        /// Property [`PrimarySource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-sourcepriority.html#cfn-mediaconnect-flow-sourcepriority-primarysource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub primary_source: ::Value<String>,
    }

    impl ::codec::SerializeValue for SourcePriority {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrimarySource", &self.primary_source)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SourcePriority {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SourcePriority, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SourcePriority;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SourcePriority")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut primary_source: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PrimarySource" => {
                                primary_source = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SourcePriority {
                        primary_source: primary_source.ok_or(::serde::de::Error::missing_field("PrimarySource"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaConnect::Flow.VpcInterfaceAttachment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-vpcinterfaceattachment.html) property type.
    #[derive(Debug, Default)]
    pub struct VpcInterfaceAttachment {
        /// Property [`VpcInterfaceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flow-vpcinterfaceattachment.html#cfn-mediaconnect-flow-vpcinterfaceattachment-vpcinterfacename).
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
        pub algorithm: Option<::Value<String>>,
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
            if let Some(ref algorithm) = self.algorithm {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Algorithm", algorithm)?;
            }
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
                        algorithm: algorithm,
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
        pub algorithm: Option<::Value<String>>,
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
            if let Some(ref algorithm) = self.algorithm {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Algorithm", algorithm)?;
            }
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
                        algorithm: algorithm,
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

    /// The [`AWS::MediaConnect::FlowSource.GatewayBridgeSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowsource-gatewaybridgesource.html) property type.
    #[derive(Debug, Default)]
    pub struct GatewayBridgeSource {
        /// Property [`BridgeArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowsource-gatewaybridgesource.html#cfn-mediaconnect-flowsource-gatewaybridgesource-bridgearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bridge_arn: ::Value<String>,
        /// Property [`VpcInterfaceAttachment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowsource-gatewaybridgesource.html#cfn-mediaconnect-flowsource-gatewaybridgesource-vpcinterfaceattachment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vpc_interface_attachment: Option<::Value<VpcInterfaceAttachment>>,
    }

    impl ::codec::SerializeValue for GatewayBridgeSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BridgeArn", &self.bridge_arn)?;
            if let Some(ref vpc_interface_attachment) = self.vpc_interface_attachment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcInterfaceAttachment", vpc_interface_attachment)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GatewayBridgeSource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GatewayBridgeSource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GatewayBridgeSource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GatewayBridgeSource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bridge_arn: Option<::Value<String>> = None;
                    let mut vpc_interface_attachment: Option<::Value<VpcInterfaceAttachment>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BridgeArn" => {
                                bridge_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpcInterfaceAttachment" => {
                                vpc_interface_attachment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GatewayBridgeSource {
                        bridge_arn: bridge_arn.ok_or(::serde::de::Error::missing_field("BridgeArn"))?,
                        vpc_interface_attachment: vpc_interface_attachment,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::MediaConnect::FlowSource.VpcInterfaceAttachment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowsource-vpcinterfaceattachment.html) property type.
    #[derive(Debug, Default)]
    pub struct VpcInterfaceAttachment {
        /// Property [`VpcInterfaceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-flowsource-vpcinterfaceattachment.html#cfn-mediaconnect-flowsource-vpcinterfaceattachment-vpcinterfacename).
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

pub mod gateway {
    //! Property types for the `Gateway` resource.

    /// The [`AWS::MediaConnect::Gateway.GatewayNetwork`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-gateway-gatewaynetwork.html) property type.
    #[derive(Debug, Default)]
    pub struct GatewayNetwork {
        /// Property [`CidrBlock`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-gateway-gatewaynetwork.html#cfn-mediaconnect-gateway-gatewaynetwork-cidrblock).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub cidr_block: ::Value<String>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconnect-gateway-gatewaynetwork.html#cfn-mediaconnect-gateway-gatewaynetwork-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for GatewayNetwork {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CidrBlock", &self.cidr_block)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GatewayNetwork {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GatewayNetwork, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GatewayNetwork;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GatewayNetwork")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cidr_block: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CidrBlock" => {
                                cidr_block = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GatewayNetwork {
                        cidr_block: cidr_block.ok_or(::serde::de::Error::missing_field("CidrBlock"))?,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
