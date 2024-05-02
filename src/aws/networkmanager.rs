//! Types for the `NetworkManager` service.

/// The [`AWS::NetworkManager::ConnectAttachment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-connectattachment.html) resource type.
#[derive(Debug, Default)]
pub struct ConnectAttachment {
    properties: ConnectAttachmentProperties
}

/// Properties for the `ConnectAttachment` resource.
#[derive(Debug, Default)]
pub struct ConnectAttachmentProperties {
    /// Property [`CoreNetworkId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-connectattachment.html#cfn-networkmanager-connectattachment-corenetworkid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub core_network_id: ::Value<String>,
    /// Property [`EdgeLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-connectattachment.html#cfn-networkmanager-connectattachment-edgelocation).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub edge_location: ::Value<String>,
    /// Property [`Options`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-connectattachment.html#cfn-networkmanager-connectattachment-options).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub options: ::Value<self::connect_attachment::ConnectAttachmentOptions>,
    /// Property [`ProposedSegmentChange`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-connectattachment.html#cfn-networkmanager-connectattachment-proposedsegmentchange).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub proposed_segment_change: Option<::Value<self::connect_attachment::ProposedSegmentChange>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-connectattachment.html#cfn-networkmanager-connectattachment-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TransportAttachmentId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-connectattachment.html#cfn-networkmanager-connectattachment-transportattachmentid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub transport_attachment_id: ::Value<String>,
}

impl ::serde::Serialize for ConnectAttachmentProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CoreNetworkId", &self.core_network_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EdgeLocation", &self.edge_location)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Options", &self.options)?;
        if let Some(ref proposed_segment_change) = self.proposed_segment_change {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProposedSegmentChange", proposed_segment_change)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TransportAttachmentId", &self.transport_attachment_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ConnectAttachmentProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ConnectAttachmentProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ConnectAttachmentProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ConnectAttachmentProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut core_network_id: Option<::Value<String>> = None;
                let mut edge_location: Option<::Value<String>> = None;
                let mut options: Option<::Value<self::connect_attachment::ConnectAttachmentOptions>> = None;
                let mut proposed_segment_change: Option<::Value<self::connect_attachment::ProposedSegmentChange>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut transport_attachment_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CoreNetworkId" => {
                            core_network_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EdgeLocation" => {
                            edge_location = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Options" => {
                            options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProposedSegmentChange" => {
                            proposed_segment_change = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TransportAttachmentId" => {
                            transport_attachment_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ConnectAttachmentProperties {
                    core_network_id: core_network_id.ok_or(::serde::de::Error::missing_field("CoreNetworkId"))?,
                    edge_location: edge_location.ok_or(::serde::de::Error::missing_field("EdgeLocation"))?,
                    options: options.ok_or(::serde::de::Error::missing_field("Options"))?,
                    proposed_segment_change: proposed_segment_change,
                    tags: tags,
                    transport_attachment_id: transport_attachment_id.ok_or(::serde::de::Error::missing_field("TransportAttachmentId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ConnectAttachment {
    type Properties = ConnectAttachmentProperties;
    const TYPE: &'static str = "AWS::NetworkManager::ConnectAttachment";
    fn properties(&self) -> &ConnectAttachmentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ConnectAttachmentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ConnectAttachment {}

impl From<ConnectAttachmentProperties> for ConnectAttachment {
    fn from(properties: ConnectAttachmentProperties) -> ConnectAttachment {
        ConnectAttachment { properties }
    }
}

/// The [`AWS::NetworkManager::ConnectPeer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-connectpeer.html) resource type.
#[derive(Debug, Default)]
pub struct ConnectPeer {
    properties: ConnectPeerProperties
}

/// Properties for the `ConnectPeer` resource.
#[derive(Debug, Default)]
pub struct ConnectPeerProperties {
    /// Property [`BgpOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-connectpeer.html#cfn-networkmanager-connectpeer-bgpoptions).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub bgp_options: Option<::Value<self::connect_peer::BgpOptions>>,
    /// Property [`ConnectAttachmentId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-connectpeer.html#cfn-networkmanager-connectpeer-connectattachmentid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub connect_attachment_id: ::Value<String>,
    /// Property [`CoreNetworkAddress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-connectpeer.html#cfn-networkmanager-connectpeer-corenetworkaddress).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub core_network_address: Option<::Value<String>>,
    /// Property [`InsideCidrBlocks`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-connectpeer.html#cfn-networkmanager-connectpeer-insidecidrblocks).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub inside_cidr_blocks: Option<::ValueList<String>>,
    /// Property [`PeerAddress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-connectpeer.html#cfn-networkmanager-connectpeer-peeraddress).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub peer_address: ::Value<String>,
    /// Property [`SubnetArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-connectpeer.html#cfn-networkmanager-connectpeer-subnetarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub subnet_arn: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-connectpeer.html#cfn-networkmanager-connectpeer-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ConnectPeerProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref bgp_options) = self.bgp_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BgpOptions", bgp_options)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectAttachmentId", &self.connect_attachment_id)?;
        if let Some(ref core_network_address) = self.core_network_address {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CoreNetworkAddress", core_network_address)?;
        }
        if let Some(ref inside_cidr_blocks) = self.inside_cidr_blocks {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InsideCidrBlocks", inside_cidr_blocks)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PeerAddress", &self.peer_address)?;
        if let Some(ref subnet_arn) = self.subnet_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetArn", subnet_arn)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ConnectPeerProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ConnectPeerProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ConnectPeerProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ConnectPeerProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut bgp_options: Option<::Value<self::connect_peer::BgpOptions>> = None;
                let mut connect_attachment_id: Option<::Value<String>> = None;
                let mut core_network_address: Option<::Value<String>> = None;
                let mut inside_cidr_blocks: Option<::ValueList<String>> = None;
                let mut peer_address: Option<::Value<String>> = None;
                let mut subnet_arn: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "BgpOptions" => {
                            bgp_options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ConnectAttachmentId" => {
                            connect_attachment_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CoreNetworkAddress" => {
                            core_network_address = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InsideCidrBlocks" => {
                            inside_cidr_blocks = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PeerAddress" => {
                            peer_address = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SubnetArn" => {
                            subnet_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ConnectPeerProperties {
                    bgp_options: bgp_options,
                    connect_attachment_id: connect_attachment_id.ok_or(::serde::de::Error::missing_field("ConnectAttachmentId"))?,
                    core_network_address: core_network_address,
                    inside_cidr_blocks: inside_cidr_blocks,
                    peer_address: peer_address.ok_or(::serde::de::Error::missing_field("PeerAddress"))?,
                    subnet_arn: subnet_arn,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ConnectPeer {
    type Properties = ConnectPeerProperties;
    const TYPE: &'static str = "AWS::NetworkManager::ConnectPeer";
    fn properties(&self) -> &ConnectPeerProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ConnectPeerProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ConnectPeer {}

impl From<ConnectPeerProperties> for ConnectPeer {
    fn from(properties: ConnectPeerProperties) -> ConnectPeer {
        ConnectPeer { properties }
    }
}

/// The [`AWS::NetworkManager::CoreNetwork`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-corenetwork.html) resource type.
#[derive(Debug, Default)]
pub struct CoreNetwork {
    properties: CoreNetworkProperties
}

/// Properties for the `CoreNetwork` resource.
#[derive(Debug, Default)]
pub struct CoreNetworkProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-corenetwork.html#cfn-networkmanager-corenetwork-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`GlobalNetworkId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-corenetwork.html#cfn-networkmanager-corenetwork-globalnetworkid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub global_network_id: ::Value<String>,
    /// Property [`PolicyDocument`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-corenetwork.html#cfn-networkmanager-corenetwork-policydocument).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub policy_document: Option<::Value<::json::Value>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-corenetwork.html#cfn-networkmanager-corenetwork-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for CoreNetworkProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "GlobalNetworkId", &self.global_network_id)?;
        if let Some(ref policy_document) = self.policy_document {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyDocument", policy_document)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CoreNetworkProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CoreNetworkProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CoreNetworkProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CoreNetworkProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut global_network_id: Option<::Value<String>> = None;
                let mut policy_document: Option<::Value<::json::Value>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GlobalNetworkId" => {
                            global_network_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PolicyDocument" => {
                            policy_document = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CoreNetworkProperties {
                    description: description,
                    global_network_id: global_network_id.ok_or(::serde::de::Error::missing_field("GlobalNetworkId"))?,
                    policy_document: policy_document,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for CoreNetwork {
    type Properties = CoreNetworkProperties;
    const TYPE: &'static str = "AWS::NetworkManager::CoreNetwork";
    fn properties(&self) -> &CoreNetworkProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CoreNetworkProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for CoreNetwork {}

impl From<CoreNetworkProperties> for CoreNetwork {
    fn from(properties: CoreNetworkProperties) -> CoreNetwork {
        CoreNetwork { properties }
    }
}

/// The [`AWS::NetworkManager::CustomerGatewayAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-customergatewayassociation.html) resource type.
#[derive(Debug, Default)]
pub struct CustomerGatewayAssociation {
    properties: CustomerGatewayAssociationProperties
}

/// Properties for the `CustomerGatewayAssociation` resource.
#[derive(Debug, Default)]
pub struct CustomerGatewayAssociationProperties {
    /// Property [`CustomerGatewayArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-customergatewayassociation.html#cfn-networkmanager-customergatewayassociation-customergatewayarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub customer_gateway_arn: ::Value<String>,
    /// Property [`DeviceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-customergatewayassociation.html#cfn-networkmanager-customergatewayassociation-deviceid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub device_id: ::Value<String>,
    /// Property [`GlobalNetworkId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-customergatewayassociation.html#cfn-networkmanager-customergatewayassociation-globalnetworkid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub global_network_id: ::Value<String>,
    /// Property [`LinkId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-customergatewayassociation.html#cfn-networkmanager-customergatewayassociation-linkid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub link_id: Option<::Value<String>>,
}

impl ::serde::Serialize for CustomerGatewayAssociationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomerGatewayArn", &self.customer_gateway_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceId", &self.device_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "GlobalNetworkId", &self.global_network_id)?;
        if let Some(ref link_id) = self.link_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LinkId", link_id)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CustomerGatewayAssociationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomerGatewayAssociationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CustomerGatewayAssociationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CustomerGatewayAssociationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut customer_gateway_arn: Option<::Value<String>> = None;
                let mut device_id: Option<::Value<String>> = None;
                let mut global_network_id: Option<::Value<String>> = None;
                let mut link_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CustomerGatewayArn" => {
                            customer_gateway_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DeviceId" => {
                            device_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GlobalNetworkId" => {
                            global_network_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LinkId" => {
                            link_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CustomerGatewayAssociationProperties {
                    customer_gateway_arn: customer_gateway_arn.ok_or(::serde::de::Error::missing_field("CustomerGatewayArn"))?,
                    device_id: device_id.ok_or(::serde::de::Error::missing_field("DeviceId"))?,
                    global_network_id: global_network_id.ok_or(::serde::de::Error::missing_field("GlobalNetworkId"))?,
                    link_id: link_id,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for CustomerGatewayAssociation {
    type Properties = CustomerGatewayAssociationProperties;
    const TYPE: &'static str = "AWS::NetworkManager::CustomerGatewayAssociation";
    fn properties(&self) -> &CustomerGatewayAssociationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CustomerGatewayAssociationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for CustomerGatewayAssociation {}

impl From<CustomerGatewayAssociationProperties> for CustomerGatewayAssociation {
    fn from(properties: CustomerGatewayAssociationProperties) -> CustomerGatewayAssociation {
        CustomerGatewayAssociation { properties }
    }
}

/// The [`AWS::NetworkManager::Device`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-device.html) resource type.
#[derive(Debug, Default)]
pub struct Device {
    properties: DeviceProperties
}

/// Properties for the `Device` resource.
#[derive(Debug, Default)]
pub struct DeviceProperties {
    /// Property [`AWSLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-device.html#cfn-networkmanager-device-awslocation).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub aws_location: Option<::Value<self::device::AWSLocation>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-device.html#cfn-networkmanager-device-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`GlobalNetworkId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-device.html#cfn-networkmanager-device-globalnetworkid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub global_network_id: ::Value<String>,
    /// Property [`Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-device.html#cfn-networkmanager-device-location).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub location: Option<::Value<self::device::Location>>,
    /// Property [`Model`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-device.html#cfn-networkmanager-device-model).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub model: Option<::Value<String>>,
    /// Property [`SerialNumber`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-device.html#cfn-networkmanager-device-serialnumber).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub serial_number: Option<::Value<String>>,
    /// Property [`SiteId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-device.html#cfn-networkmanager-device-siteid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub site_id: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-device.html#cfn-networkmanager-device-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-device.html#cfn-networkmanager-device-type).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub r#type: Option<::Value<String>>,
    /// Property [`Vendor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-device.html#cfn-networkmanager-device-vendor).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub vendor: Option<::Value<String>>,
}

impl ::serde::Serialize for DeviceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref aws_location) = self.aws_location {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AWSLocation", aws_location)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "GlobalNetworkId", &self.global_network_id)?;
        if let Some(ref location) = self.location {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Location", location)?;
        }
        if let Some(ref model) = self.model {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Model", model)?;
        }
        if let Some(ref serial_number) = self.serial_number {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SerialNumber", serial_number)?;
        }
        if let Some(ref site_id) = self.site_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SiteId", site_id)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref r#type) = self.r#type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
        }
        if let Some(ref vendor) = self.vendor {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Vendor", vendor)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DeviceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DeviceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DeviceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DeviceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut aws_location: Option<::Value<self::device::AWSLocation>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut global_network_id: Option<::Value<String>> = None;
                let mut location: Option<::Value<self::device::Location>> = None;
                let mut model: Option<::Value<String>> = None;
                let mut serial_number: Option<::Value<String>> = None;
                let mut site_id: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut r#type: Option<::Value<String>> = None;
                let mut vendor: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AWSLocation" => {
                            aws_location = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GlobalNetworkId" => {
                            global_network_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Location" => {
                            location = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Model" => {
                            model = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SerialNumber" => {
                            serial_number = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SiteId" => {
                            site_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Type" => {
                            r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Vendor" => {
                            vendor = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DeviceProperties {
                    aws_location: aws_location,
                    description: description,
                    global_network_id: global_network_id.ok_or(::serde::de::Error::missing_field("GlobalNetworkId"))?,
                    location: location,
                    model: model,
                    serial_number: serial_number,
                    site_id: site_id,
                    tags: tags,
                    r#type: r#type,
                    vendor: vendor,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Device {
    type Properties = DeviceProperties;
    const TYPE: &'static str = "AWS::NetworkManager::Device";
    fn properties(&self) -> &DeviceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DeviceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Device {}

impl From<DeviceProperties> for Device {
    fn from(properties: DeviceProperties) -> Device {
        Device { properties }
    }
}

/// The [`AWS::NetworkManager::GlobalNetwork`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-globalnetwork.html) resource type.
#[derive(Debug, Default)]
pub struct GlobalNetwork {
    properties: GlobalNetworkProperties
}

/// Properties for the `GlobalNetwork` resource.
#[derive(Debug, Default)]
pub struct GlobalNetworkProperties {
    /// Property [`CreatedAt`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-globalnetwork.html#cfn-networkmanager-globalnetwork-createdat).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub created_at: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-globalnetwork.html#cfn-networkmanager-globalnetwork-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`State`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-globalnetwork.html#cfn-networkmanager-globalnetwork-state).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub state: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-globalnetwork.html#cfn-networkmanager-globalnetwork-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for GlobalNetworkProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref created_at) = self.created_at {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CreatedAt", created_at)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref state) = self.state {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "State", state)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for GlobalNetworkProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<GlobalNetworkProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = GlobalNetworkProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type GlobalNetworkProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut created_at: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut state: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CreatedAt" => {
                            created_at = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "State" => {
                            state = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(GlobalNetworkProperties {
                    created_at: created_at,
                    description: description,
                    state: state,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for GlobalNetwork {
    type Properties = GlobalNetworkProperties;
    const TYPE: &'static str = "AWS::NetworkManager::GlobalNetwork";
    fn properties(&self) -> &GlobalNetworkProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut GlobalNetworkProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for GlobalNetwork {}

impl From<GlobalNetworkProperties> for GlobalNetwork {
    fn from(properties: GlobalNetworkProperties) -> GlobalNetwork {
        GlobalNetwork { properties }
    }
}

/// The [`AWS::NetworkManager::Link`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-link.html) resource type.
#[derive(Debug, Default)]
pub struct Link {
    properties: LinkProperties
}

/// Properties for the `Link` resource.
#[derive(Debug, Default)]
pub struct LinkProperties {
    /// Property [`Bandwidth`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-link.html#cfn-networkmanager-link-bandwidth).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub bandwidth: ::Value<self::link::Bandwidth>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-link.html#cfn-networkmanager-link-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`GlobalNetworkId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-link.html#cfn-networkmanager-link-globalnetworkid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub global_network_id: ::Value<String>,
    /// Property [`Provider`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-link.html#cfn-networkmanager-link-provider).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub provider: Option<::Value<String>>,
    /// Property [`SiteId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-link.html#cfn-networkmanager-link-siteid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub site_id: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-link.html#cfn-networkmanager-link-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-link.html#cfn-networkmanager-link-type).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub r#type: Option<::Value<String>>,
}

impl ::serde::Serialize for LinkProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bandwidth", &self.bandwidth)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "GlobalNetworkId", &self.global_network_id)?;
        if let Some(ref provider) = self.provider {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Provider", provider)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SiteId", &self.site_id)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref r#type) = self.r#type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LinkProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LinkProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LinkProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LinkProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut bandwidth: Option<::Value<self::link::Bandwidth>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut global_network_id: Option<::Value<String>> = None;
                let mut provider: Option<::Value<String>> = None;
                let mut site_id: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut r#type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Bandwidth" => {
                            bandwidth = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GlobalNetworkId" => {
                            global_network_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Provider" => {
                            provider = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SiteId" => {
                            site_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Type" => {
                            r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(LinkProperties {
                    bandwidth: bandwidth.ok_or(::serde::de::Error::missing_field("Bandwidth"))?,
                    description: description,
                    global_network_id: global_network_id.ok_or(::serde::de::Error::missing_field("GlobalNetworkId"))?,
                    provider: provider,
                    site_id: site_id.ok_or(::serde::de::Error::missing_field("SiteId"))?,
                    tags: tags,
                    r#type: r#type,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Link {
    type Properties = LinkProperties;
    const TYPE: &'static str = "AWS::NetworkManager::Link";
    fn properties(&self) -> &LinkProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LinkProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Link {}

impl From<LinkProperties> for Link {
    fn from(properties: LinkProperties) -> Link {
        Link { properties }
    }
}

/// The [`AWS::NetworkManager::LinkAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-linkassociation.html) resource type.
#[derive(Debug, Default)]
pub struct LinkAssociation {
    properties: LinkAssociationProperties
}

/// Properties for the `LinkAssociation` resource.
#[derive(Debug, Default)]
pub struct LinkAssociationProperties {
    /// Property [`DeviceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-linkassociation.html#cfn-networkmanager-linkassociation-deviceid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub device_id: ::Value<String>,
    /// Property [`GlobalNetworkId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-linkassociation.html#cfn-networkmanager-linkassociation-globalnetworkid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub global_network_id: ::Value<String>,
    /// Property [`LinkId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-linkassociation.html#cfn-networkmanager-linkassociation-linkid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub link_id: ::Value<String>,
}

impl ::serde::Serialize for LinkAssociationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceId", &self.device_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "GlobalNetworkId", &self.global_network_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LinkId", &self.link_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LinkAssociationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LinkAssociationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LinkAssociationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LinkAssociationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut device_id: Option<::Value<String>> = None;
                let mut global_network_id: Option<::Value<String>> = None;
                let mut link_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DeviceId" => {
                            device_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GlobalNetworkId" => {
                            global_network_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LinkId" => {
                            link_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(LinkAssociationProperties {
                    device_id: device_id.ok_or(::serde::de::Error::missing_field("DeviceId"))?,
                    global_network_id: global_network_id.ok_or(::serde::de::Error::missing_field("GlobalNetworkId"))?,
                    link_id: link_id.ok_or(::serde::de::Error::missing_field("LinkId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for LinkAssociation {
    type Properties = LinkAssociationProperties;
    const TYPE: &'static str = "AWS::NetworkManager::LinkAssociation";
    fn properties(&self) -> &LinkAssociationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LinkAssociationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for LinkAssociation {}

impl From<LinkAssociationProperties> for LinkAssociation {
    fn from(properties: LinkAssociationProperties) -> LinkAssociation {
        LinkAssociation { properties }
    }
}

/// The [`AWS::NetworkManager::Site`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-site.html) resource type.
#[derive(Debug, Default)]
pub struct Site {
    properties: SiteProperties
}

/// Properties for the `Site` resource.
#[derive(Debug, Default)]
pub struct SiteProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-site.html#cfn-networkmanager-site-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`GlobalNetworkId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-site.html#cfn-networkmanager-site-globalnetworkid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub global_network_id: ::Value<String>,
    /// Property [`Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-site.html#cfn-networkmanager-site-location).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub location: Option<::Value<self::site::Location>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-site.html#cfn-networkmanager-site-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for SiteProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "GlobalNetworkId", &self.global_network_id)?;
        if let Some(ref location) = self.location {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Location", location)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SiteProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SiteProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SiteProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SiteProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut global_network_id: Option<::Value<String>> = None;
                let mut location: Option<::Value<self::site::Location>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GlobalNetworkId" => {
                            global_network_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Location" => {
                            location = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SiteProperties {
                    description: description,
                    global_network_id: global_network_id.ok_or(::serde::de::Error::missing_field("GlobalNetworkId"))?,
                    location: location,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Site {
    type Properties = SiteProperties;
    const TYPE: &'static str = "AWS::NetworkManager::Site";
    fn properties(&self) -> &SiteProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SiteProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Site {}

impl From<SiteProperties> for Site {
    fn from(properties: SiteProperties) -> Site {
        Site { properties }
    }
}

/// The [`AWS::NetworkManager::SiteToSiteVpnAttachment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-sitetositevpnattachment.html) resource type.
#[derive(Debug, Default)]
pub struct SiteToSiteVpnAttachment {
    properties: SiteToSiteVpnAttachmentProperties
}

/// Properties for the `SiteToSiteVpnAttachment` resource.
#[derive(Debug, Default)]
pub struct SiteToSiteVpnAttachmentProperties {
    /// Property [`CoreNetworkId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-sitetositevpnattachment.html#cfn-networkmanager-sitetositevpnattachment-corenetworkid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub core_network_id: ::Value<String>,
    /// Property [`ProposedSegmentChange`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-sitetositevpnattachment.html#cfn-networkmanager-sitetositevpnattachment-proposedsegmentchange).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub proposed_segment_change: Option<::Value<self::site_to_site_vpn_attachment::ProposedSegmentChange>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-sitetositevpnattachment.html#cfn-networkmanager-sitetositevpnattachment-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`VpnConnectionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-sitetositevpnattachment.html#cfn-networkmanager-sitetositevpnattachment-vpnconnectionarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub vpn_connection_arn: ::Value<String>,
}

impl ::serde::Serialize for SiteToSiteVpnAttachmentProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CoreNetworkId", &self.core_network_id)?;
        if let Some(ref proposed_segment_change) = self.proposed_segment_change {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProposedSegmentChange", proposed_segment_change)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpnConnectionArn", &self.vpn_connection_arn)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SiteToSiteVpnAttachmentProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SiteToSiteVpnAttachmentProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SiteToSiteVpnAttachmentProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SiteToSiteVpnAttachmentProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut core_network_id: Option<::Value<String>> = None;
                let mut proposed_segment_change: Option<::Value<self::site_to_site_vpn_attachment::ProposedSegmentChange>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut vpn_connection_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CoreNetworkId" => {
                            core_network_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProposedSegmentChange" => {
                            proposed_segment_change = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpnConnectionArn" => {
                            vpn_connection_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SiteToSiteVpnAttachmentProperties {
                    core_network_id: core_network_id.ok_or(::serde::de::Error::missing_field("CoreNetworkId"))?,
                    proposed_segment_change: proposed_segment_change,
                    tags: tags,
                    vpn_connection_arn: vpn_connection_arn.ok_or(::serde::de::Error::missing_field("VpnConnectionArn"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for SiteToSiteVpnAttachment {
    type Properties = SiteToSiteVpnAttachmentProperties;
    const TYPE: &'static str = "AWS::NetworkManager::SiteToSiteVpnAttachment";
    fn properties(&self) -> &SiteToSiteVpnAttachmentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SiteToSiteVpnAttachmentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SiteToSiteVpnAttachment {}

impl From<SiteToSiteVpnAttachmentProperties> for SiteToSiteVpnAttachment {
    fn from(properties: SiteToSiteVpnAttachmentProperties) -> SiteToSiteVpnAttachment {
        SiteToSiteVpnAttachment { properties }
    }
}

/// The [`AWS::NetworkManager::TransitGatewayPeering`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-transitgatewaypeering.html) resource type.
#[derive(Debug, Default)]
pub struct TransitGatewayPeering {
    properties: TransitGatewayPeeringProperties
}

/// Properties for the `TransitGatewayPeering` resource.
#[derive(Debug, Default)]
pub struct TransitGatewayPeeringProperties {
    /// Property [`CoreNetworkId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-transitgatewaypeering.html#cfn-networkmanager-transitgatewaypeering-corenetworkid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub core_network_id: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-transitgatewaypeering.html#cfn-networkmanager-transitgatewaypeering-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TransitGatewayArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-transitgatewaypeering.html#cfn-networkmanager-transitgatewaypeering-transitgatewayarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub transit_gateway_arn: ::Value<String>,
}

impl ::serde::Serialize for TransitGatewayPeeringProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CoreNetworkId", &self.core_network_id)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TransitGatewayArn", &self.transit_gateway_arn)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for TransitGatewayPeeringProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<TransitGatewayPeeringProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TransitGatewayPeeringProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type TransitGatewayPeeringProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut core_network_id: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut transit_gateway_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CoreNetworkId" => {
                            core_network_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TransitGatewayArn" => {
                            transit_gateway_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(TransitGatewayPeeringProperties {
                    core_network_id: core_network_id.ok_or(::serde::de::Error::missing_field("CoreNetworkId"))?,
                    tags: tags,
                    transit_gateway_arn: transit_gateway_arn.ok_or(::serde::de::Error::missing_field("TransitGatewayArn"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for TransitGatewayPeering {
    type Properties = TransitGatewayPeeringProperties;
    const TYPE: &'static str = "AWS::NetworkManager::TransitGatewayPeering";
    fn properties(&self) -> &TransitGatewayPeeringProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TransitGatewayPeeringProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for TransitGatewayPeering {}

impl From<TransitGatewayPeeringProperties> for TransitGatewayPeering {
    fn from(properties: TransitGatewayPeeringProperties) -> TransitGatewayPeering {
        TransitGatewayPeering { properties }
    }
}

/// The [`AWS::NetworkManager::TransitGatewayRegistration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-transitgatewayregistration.html) resource type.
#[derive(Debug, Default)]
pub struct TransitGatewayRegistration {
    properties: TransitGatewayRegistrationProperties
}

/// Properties for the `TransitGatewayRegistration` resource.
#[derive(Debug, Default)]
pub struct TransitGatewayRegistrationProperties {
    /// Property [`GlobalNetworkId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-transitgatewayregistration.html#cfn-networkmanager-transitgatewayregistration-globalnetworkid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub global_network_id: ::Value<String>,
    /// Property [`TransitGatewayArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-transitgatewayregistration.html#cfn-networkmanager-transitgatewayregistration-transitgatewayarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub transit_gateway_arn: ::Value<String>,
}

impl ::serde::Serialize for TransitGatewayRegistrationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "GlobalNetworkId", &self.global_network_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TransitGatewayArn", &self.transit_gateway_arn)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for TransitGatewayRegistrationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<TransitGatewayRegistrationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TransitGatewayRegistrationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type TransitGatewayRegistrationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut global_network_id: Option<::Value<String>> = None;
                let mut transit_gateway_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "GlobalNetworkId" => {
                            global_network_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TransitGatewayArn" => {
                            transit_gateway_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(TransitGatewayRegistrationProperties {
                    global_network_id: global_network_id.ok_or(::serde::de::Error::missing_field("GlobalNetworkId"))?,
                    transit_gateway_arn: transit_gateway_arn.ok_or(::serde::de::Error::missing_field("TransitGatewayArn"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for TransitGatewayRegistration {
    type Properties = TransitGatewayRegistrationProperties;
    const TYPE: &'static str = "AWS::NetworkManager::TransitGatewayRegistration";
    fn properties(&self) -> &TransitGatewayRegistrationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TransitGatewayRegistrationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for TransitGatewayRegistration {}

impl From<TransitGatewayRegistrationProperties> for TransitGatewayRegistration {
    fn from(properties: TransitGatewayRegistrationProperties) -> TransitGatewayRegistration {
        TransitGatewayRegistration { properties }
    }
}

/// The [`AWS::NetworkManager::TransitGatewayRouteTableAttachment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-transitgatewayroutetableattachment.html) resource type.
#[derive(Debug, Default)]
pub struct TransitGatewayRouteTableAttachment {
    properties: TransitGatewayRouteTableAttachmentProperties
}

/// Properties for the `TransitGatewayRouteTableAttachment` resource.
#[derive(Debug, Default)]
pub struct TransitGatewayRouteTableAttachmentProperties {
    /// Property [`PeeringId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-transitgatewayroutetableattachment.html#cfn-networkmanager-transitgatewayroutetableattachment-peeringid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub peering_id: ::Value<String>,
    /// Property [`ProposedSegmentChange`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-transitgatewayroutetableattachment.html#cfn-networkmanager-transitgatewayroutetableattachment-proposedsegmentchange).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub proposed_segment_change: Option<::Value<self::transit_gateway_route_table_attachment::ProposedSegmentChange>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-transitgatewayroutetableattachment.html#cfn-networkmanager-transitgatewayroutetableattachment-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TransitGatewayRouteTableArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-transitgatewayroutetableattachment.html#cfn-networkmanager-transitgatewayroutetableattachment-transitgatewayroutetablearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub transit_gateway_route_table_arn: ::Value<String>,
}

impl ::serde::Serialize for TransitGatewayRouteTableAttachmentProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PeeringId", &self.peering_id)?;
        if let Some(ref proposed_segment_change) = self.proposed_segment_change {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProposedSegmentChange", proposed_segment_change)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TransitGatewayRouteTableArn", &self.transit_gateway_route_table_arn)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for TransitGatewayRouteTableAttachmentProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<TransitGatewayRouteTableAttachmentProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TransitGatewayRouteTableAttachmentProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type TransitGatewayRouteTableAttachmentProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut peering_id: Option<::Value<String>> = None;
                let mut proposed_segment_change: Option<::Value<self::transit_gateway_route_table_attachment::ProposedSegmentChange>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut transit_gateway_route_table_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "PeeringId" => {
                            peering_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProposedSegmentChange" => {
                            proposed_segment_change = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TransitGatewayRouteTableArn" => {
                            transit_gateway_route_table_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(TransitGatewayRouteTableAttachmentProperties {
                    peering_id: peering_id.ok_or(::serde::de::Error::missing_field("PeeringId"))?,
                    proposed_segment_change: proposed_segment_change,
                    tags: tags,
                    transit_gateway_route_table_arn: transit_gateway_route_table_arn.ok_or(::serde::de::Error::missing_field("TransitGatewayRouteTableArn"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for TransitGatewayRouteTableAttachment {
    type Properties = TransitGatewayRouteTableAttachmentProperties;
    const TYPE: &'static str = "AWS::NetworkManager::TransitGatewayRouteTableAttachment";
    fn properties(&self) -> &TransitGatewayRouteTableAttachmentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TransitGatewayRouteTableAttachmentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for TransitGatewayRouteTableAttachment {}

impl From<TransitGatewayRouteTableAttachmentProperties> for TransitGatewayRouteTableAttachment {
    fn from(properties: TransitGatewayRouteTableAttachmentProperties) -> TransitGatewayRouteTableAttachment {
        TransitGatewayRouteTableAttachment { properties }
    }
}

/// The [`AWS::NetworkManager::VpcAttachment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-vpcattachment.html) resource type.
#[derive(Debug, Default)]
pub struct VpcAttachment {
    properties: VpcAttachmentProperties
}

/// Properties for the `VpcAttachment` resource.
#[derive(Debug, Default)]
pub struct VpcAttachmentProperties {
    /// Property [`CoreNetworkId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-vpcattachment.html#cfn-networkmanager-vpcattachment-corenetworkid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub core_network_id: ::Value<String>,
    /// Property [`Options`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-vpcattachment.html#cfn-networkmanager-vpcattachment-options).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub options: Option<::Value<self::vpc_attachment::VpcOptions>>,
    /// Property [`ProposedSegmentChange`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-vpcattachment.html#cfn-networkmanager-vpcattachment-proposedsegmentchange).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub proposed_segment_change: Option<::Value<self::vpc_attachment::ProposedSegmentChange>>,
    /// Property [`SubnetArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-vpcattachment.html#cfn-networkmanager-vpcattachment-subnetarns).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub subnet_arns: ::ValueList<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-vpcattachment.html#cfn-networkmanager-vpcattachment-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`VpcArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-vpcattachment.html#cfn-networkmanager-vpcattachment-vpcarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub vpc_arn: ::Value<String>,
}

impl ::serde::Serialize for VpcAttachmentProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CoreNetworkId", &self.core_network_id)?;
        if let Some(ref options) = self.options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Options", options)?;
        }
        if let Some(ref proposed_segment_change) = self.proposed_segment_change {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProposedSegmentChange", proposed_segment_change)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetArns", &self.subnet_arns)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcArn", &self.vpc_arn)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for VpcAttachmentProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<VpcAttachmentProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = VpcAttachmentProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type VpcAttachmentProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut core_network_id: Option<::Value<String>> = None;
                let mut options: Option<::Value<self::vpc_attachment::VpcOptions>> = None;
                let mut proposed_segment_change: Option<::Value<self::vpc_attachment::ProposedSegmentChange>> = None;
                let mut subnet_arns: Option<::ValueList<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut vpc_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CoreNetworkId" => {
                            core_network_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Options" => {
                            options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProposedSegmentChange" => {
                            proposed_segment_change = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SubnetArns" => {
                            subnet_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcArn" => {
                            vpc_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(VpcAttachmentProperties {
                    core_network_id: core_network_id.ok_or(::serde::de::Error::missing_field("CoreNetworkId"))?,
                    options: options,
                    proposed_segment_change: proposed_segment_change,
                    subnet_arns: subnet_arns.ok_or(::serde::de::Error::missing_field("SubnetArns"))?,
                    tags: tags,
                    vpc_arn: vpc_arn.ok_or(::serde::de::Error::missing_field("VpcArn"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for VpcAttachment {
    type Properties = VpcAttachmentProperties;
    const TYPE: &'static str = "AWS::NetworkManager::VpcAttachment";
    fn properties(&self) -> &VpcAttachmentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VpcAttachmentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for VpcAttachment {}

impl From<VpcAttachmentProperties> for VpcAttachment {
    fn from(properties: VpcAttachmentProperties) -> VpcAttachment {
        VpcAttachment { properties }
    }
}

pub mod connect_attachment {
    //! Property types for the `ConnectAttachment` resource.

    /// The [`AWS::NetworkManager::ConnectAttachment.ConnectAttachmentOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-connectattachment-connectattachmentoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct ConnectAttachmentOptions {
        /// Property [`Protocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-connectattachment-connectattachmentoptions.html#cfn-networkmanager-connectattachment-connectattachmentoptions-protocol).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub protocol: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ConnectAttachmentOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref protocol) = self.protocol {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocol", protocol)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConnectAttachmentOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConnectAttachmentOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConnectAttachmentOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConnectAttachmentOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut protocol: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Protocol" => {
                                protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConnectAttachmentOptions {
                        protocol: protocol,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkManager::ConnectAttachment.ProposedSegmentChange`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-connectattachment-proposedsegmentchange.html) property type.
    #[derive(Debug, Default)]
    pub struct ProposedSegmentChange {
        /// Property [`AttachmentPolicyRuleNumber`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-connectattachment-proposedsegmentchange.html#cfn-networkmanager-connectattachment-proposedsegmentchange-attachmentpolicyrulenumber).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attachment_policy_rule_number: Option<::Value<u32>>,
        /// Property [`SegmentName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-connectattachment-proposedsegmentchange.html#cfn-networkmanager-connectattachment-proposedsegmentchange-segmentname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub segment_name: Option<::Value<String>>,
        /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-connectattachment-proposedsegmentchange.html#cfn-networkmanager-connectattachment-proposedsegmentchange-tags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tags: Option<::ValueList<::Tag>>,
    }

    impl ::codec::SerializeValue for ProposedSegmentChange {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref attachment_policy_rule_number) = self.attachment_policy_rule_number {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttachmentPolicyRuleNumber", attachment_policy_rule_number)?;
            }
            if let Some(ref segment_name) = self.segment_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SegmentName", segment_name)?;
            }
            if let Some(ref tags) = self.tags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ProposedSegmentChange {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ProposedSegmentChange, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ProposedSegmentChange;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ProposedSegmentChange")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attachment_policy_rule_number: Option<::Value<u32>> = None;
                    let mut segment_name: Option<::Value<String>> = None;
                    let mut tags: Option<::ValueList<::Tag>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AttachmentPolicyRuleNumber" => {
                                attachment_policy_rule_number = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SegmentName" => {
                                segment_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Tags" => {
                                tags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ProposedSegmentChange {
                        attachment_policy_rule_number: attachment_policy_rule_number,
                        segment_name: segment_name,
                        tags: tags,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod connect_peer {
    //! Property types for the `ConnectPeer` resource.

    /// The [`AWS::NetworkManager::ConnectPeer.BgpOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-connectpeer-bgpoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct BgpOptions {
        /// Property [`PeerAsn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-connectpeer-bgpoptions.html#cfn-networkmanager-connectpeer-bgpoptions-peerasn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub peer_asn: Option<::Value<f64>>,
    }

    impl ::codec::SerializeValue for BgpOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref peer_asn) = self.peer_asn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PeerAsn", peer_asn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BgpOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BgpOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BgpOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BgpOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut peer_asn: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PeerAsn" => {
                                peer_asn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BgpOptions {
                        peer_asn: peer_asn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkManager::ConnectPeer.ConnectPeerBgpConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-connectpeer-connectpeerbgpconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ConnectPeerBgpConfiguration {
        /// Property [`CoreNetworkAddress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-connectpeer-connectpeerbgpconfiguration.html#cfn-networkmanager-connectpeer-connectpeerbgpconfiguration-corenetworkaddress).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub core_network_address: Option<::Value<String>>,
        /// Property [`CoreNetworkAsn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-connectpeer-connectpeerbgpconfiguration.html#cfn-networkmanager-connectpeer-connectpeerbgpconfiguration-corenetworkasn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub core_network_asn: Option<::Value<f64>>,
        /// Property [`PeerAddress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-connectpeer-connectpeerbgpconfiguration.html#cfn-networkmanager-connectpeer-connectpeerbgpconfiguration-peeraddress).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub peer_address: Option<::Value<String>>,
        /// Property [`PeerAsn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-connectpeer-connectpeerbgpconfiguration.html#cfn-networkmanager-connectpeer-connectpeerbgpconfiguration-peerasn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub peer_asn: Option<::Value<f64>>,
    }

    impl ::codec::SerializeValue for ConnectPeerBgpConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref core_network_address) = self.core_network_address {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CoreNetworkAddress", core_network_address)?;
            }
            if let Some(ref core_network_asn) = self.core_network_asn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CoreNetworkAsn", core_network_asn)?;
            }
            if let Some(ref peer_address) = self.peer_address {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PeerAddress", peer_address)?;
            }
            if let Some(ref peer_asn) = self.peer_asn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PeerAsn", peer_asn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConnectPeerBgpConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConnectPeerBgpConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConnectPeerBgpConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConnectPeerBgpConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut core_network_address: Option<::Value<String>> = None;
                    let mut core_network_asn: Option<::Value<f64>> = None;
                    let mut peer_address: Option<::Value<String>> = None;
                    let mut peer_asn: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CoreNetworkAddress" => {
                                core_network_address = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CoreNetworkAsn" => {
                                core_network_asn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PeerAddress" => {
                                peer_address = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PeerAsn" => {
                                peer_asn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConnectPeerBgpConfiguration {
                        core_network_address: core_network_address,
                        core_network_asn: core_network_asn,
                        peer_address: peer_address,
                        peer_asn: peer_asn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkManager::ConnectPeer.ConnectPeerConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-connectpeer-connectpeerconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ConnectPeerConfiguration {
        /// Property [`BgpConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-connectpeer-connectpeerconfiguration.html#cfn-networkmanager-connectpeer-connectpeerconfiguration-bgpconfigurations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bgp_configurations: Option<::ValueList<ConnectPeerBgpConfiguration>>,
        /// Property [`CoreNetworkAddress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-connectpeer-connectpeerconfiguration.html#cfn-networkmanager-connectpeer-connectpeerconfiguration-corenetworkaddress).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub core_network_address: Option<::Value<String>>,
        /// Property [`InsideCidrBlocks`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-connectpeer-connectpeerconfiguration.html#cfn-networkmanager-connectpeer-connectpeerconfiguration-insidecidrblocks).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub inside_cidr_blocks: Option<::ValueList<String>>,
        /// Property [`PeerAddress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-connectpeer-connectpeerconfiguration.html#cfn-networkmanager-connectpeer-connectpeerconfiguration-peeraddress).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub peer_address: Option<::Value<String>>,
        /// Property [`Protocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-connectpeer-connectpeerconfiguration.html#cfn-networkmanager-connectpeer-connectpeerconfiguration-protocol).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub protocol: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ConnectPeerConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref bgp_configurations) = self.bgp_configurations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BgpConfigurations", bgp_configurations)?;
            }
            if let Some(ref core_network_address) = self.core_network_address {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CoreNetworkAddress", core_network_address)?;
            }
            if let Some(ref inside_cidr_blocks) = self.inside_cidr_blocks {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InsideCidrBlocks", inside_cidr_blocks)?;
            }
            if let Some(ref peer_address) = self.peer_address {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PeerAddress", peer_address)?;
            }
            if let Some(ref protocol) = self.protocol {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocol", protocol)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConnectPeerConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConnectPeerConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConnectPeerConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConnectPeerConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bgp_configurations: Option<::ValueList<ConnectPeerBgpConfiguration>> = None;
                    let mut core_network_address: Option<::Value<String>> = None;
                    let mut inside_cidr_blocks: Option<::ValueList<String>> = None;
                    let mut peer_address: Option<::Value<String>> = None;
                    let mut protocol: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BgpConfigurations" => {
                                bgp_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CoreNetworkAddress" => {
                                core_network_address = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InsideCidrBlocks" => {
                                inside_cidr_blocks = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PeerAddress" => {
                                peer_address = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Protocol" => {
                                protocol = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConnectPeerConfiguration {
                        bgp_configurations: bgp_configurations,
                        core_network_address: core_network_address,
                        inside_cidr_blocks: inside_cidr_blocks,
                        peer_address: peer_address,
                        protocol: protocol,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod core_network {
    //! Property types for the `CoreNetwork` resource.

    /// The [`AWS::NetworkManager::CoreNetwork.CoreNetworkEdge`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-corenetwork-corenetworkedge.html) property type.
    #[derive(Debug, Default)]
    pub struct CoreNetworkEdge {
        /// Property [`Asn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-corenetwork-corenetworkedge.html#cfn-networkmanager-corenetwork-corenetworkedge-asn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub asn: Option<::Value<f64>>,
        /// Property [`EdgeLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-corenetwork-corenetworkedge.html#cfn-networkmanager-corenetwork-corenetworkedge-edgelocation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub edge_location: Option<::Value<String>>,
        /// Property [`InsideCidrBlocks`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-corenetwork-corenetworkedge.html#cfn-networkmanager-corenetwork-corenetworkedge-insidecidrblocks).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub inside_cidr_blocks: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for CoreNetworkEdge {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref asn) = self.asn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Asn", asn)?;
            }
            if let Some(ref edge_location) = self.edge_location {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EdgeLocation", edge_location)?;
            }
            if let Some(ref inside_cidr_blocks) = self.inside_cidr_blocks {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InsideCidrBlocks", inside_cidr_blocks)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CoreNetworkEdge {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CoreNetworkEdge, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CoreNetworkEdge;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CoreNetworkEdge")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut asn: Option<::Value<f64>> = None;
                    let mut edge_location: Option<::Value<String>> = None;
                    let mut inside_cidr_blocks: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Asn" => {
                                asn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EdgeLocation" => {
                                edge_location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InsideCidrBlocks" => {
                                inside_cidr_blocks = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CoreNetworkEdge {
                        asn: asn,
                        edge_location: edge_location,
                        inside_cidr_blocks: inside_cidr_blocks,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkManager::CoreNetwork.CoreNetworkSegment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-corenetwork-corenetworksegment.html) property type.
    #[derive(Debug, Default)]
    pub struct CoreNetworkSegment {
        /// Property [`EdgeLocations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-corenetwork-corenetworksegment.html#cfn-networkmanager-corenetwork-corenetworksegment-edgelocations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub edge_locations: Option<::ValueList<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-corenetwork-corenetworksegment.html#cfn-networkmanager-corenetwork-corenetworksegment-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`SharedSegments`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-corenetwork-corenetworksegment.html#cfn-networkmanager-corenetwork-corenetworksegment-sharedsegments).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub shared_segments: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for CoreNetworkSegment {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref edge_locations) = self.edge_locations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EdgeLocations", edge_locations)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref shared_segments) = self.shared_segments {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SharedSegments", shared_segments)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CoreNetworkSegment {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CoreNetworkSegment, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CoreNetworkSegment;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CoreNetworkSegment")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut edge_locations: Option<::ValueList<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut shared_segments: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EdgeLocations" => {
                                edge_locations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SharedSegments" => {
                                shared_segments = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CoreNetworkSegment {
                        edge_locations: edge_locations,
                        name: name,
                        shared_segments: shared_segments,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod device {
    //! Property types for the `Device` resource.

    /// The [`AWS::NetworkManager::Device.AWSLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-device-awslocation.html) property type.
    #[derive(Debug, Default)]
    pub struct AWSLocation {
        /// Property [`SubnetArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-device-awslocation.html#cfn-networkmanager-device-awslocation-subnetarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subnet_arn: Option<::Value<String>>,
        /// Property [`Zone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-device-awslocation.html#cfn-networkmanager-device-awslocation-zone).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub zone: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AWSLocation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref subnet_arn) = self.subnet_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetArn", subnet_arn)?;
            }
            if let Some(ref zone) = self.zone {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Zone", zone)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AWSLocation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AWSLocation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AWSLocation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AWSLocation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut subnet_arn: Option<::Value<String>> = None;
                    let mut zone: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SubnetArn" => {
                                subnet_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Zone" => {
                                zone = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AWSLocation {
                        subnet_arn: subnet_arn,
                        zone: zone,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkManager::Device.Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-device-location.html) property type.
    #[derive(Debug, Default)]
    pub struct Location {
        /// Property [`Address`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-device-location.html#cfn-networkmanager-device-location-address).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub address: Option<::Value<String>>,
        /// Property [`Latitude`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-device-location.html#cfn-networkmanager-device-location-latitude).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub latitude: Option<::Value<String>>,
        /// Property [`Longitude`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-device-location.html#cfn-networkmanager-device-location-longitude).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub longitude: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Location {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref address) = self.address {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Address", address)?;
            }
            if let Some(ref latitude) = self.latitude {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Latitude", latitude)?;
            }
            if let Some(ref longitude) = self.longitude {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Longitude", longitude)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Location {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Location, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Location;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Location")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut address: Option<::Value<String>> = None;
                    let mut latitude: Option<::Value<String>> = None;
                    let mut longitude: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Address" => {
                                address = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Latitude" => {
                                latitude = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Longitude" => {
                                longitude = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Location {
                        address: address,
                        latitude: latitude,
                        longitude: longitude,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod link {
    //! Property types for the `Link` resource.

    /// The [`AWS::NetworkManager::Link.Bandwidth`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-link-bandwidth.html) property type.
    #[derive(Debug, Default)]
    pub struct Bandwidth {
        /// Property [`DownloadSpeed`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-link-bandwidth.html#cfn-networkmanager-link-bandwidth-downloadspeed).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub download_speed: Option<::Value<u32>>,
        /// Property [`UploadSpeed`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-link-bandwidth.html#cfn-networkmanager-link-bandwidth-uploadspeed).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub upload_speed: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for Bandwidth {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref download_speed) = self.download_speed {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DownloadSpeed", download_speed)?;
            }
            if let Some(ref upload_speed) = self.upload_speed {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UploadSpeed", upload_speed)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Bandwidth {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Bandwidth, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Bandwidth;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Bandwidth")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut download_speed: Option<::Value<u32>> = None;
                    let mut upload_speed: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DownloadSpeed" => {
                                download_speed = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UploadSpeed" => {
                                upload_speed = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Bandwidth {
                        download_speed: download_speed,
                        upload_speed: upload_speed,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod site {
    //! Property types for the `Site` resource.

    /// The [`AWS::NetworkManager::Site.Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-site-location.html) property type.
    #[derive(Debug, Default)]
    pub struct Location {
        /// Property [`Address`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-site-location.html#cfn-networkmanager-site-location-address).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub address: Option<::Value<String>>,
        /// Property [`Latitude`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-site-location.html#cfn-networkmanager-site-location-latitude).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub latitude: Option<::Value<String>>,
        /// Property [`Longitude`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-site-location.html#cfn-networkmanager-site-location-longitude).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub longitude: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Location {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref address) = self.address {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Address", address)?;
            }
            if let Some(ref latitude) = self.latitude {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Latitude", latitude)?;
            }
            if let Some(ref longitude) = self.longitude {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Longitude", longitude)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Location {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Location, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Location;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Location")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut address: Option<::Value<String>> = None;
                    let mut latitude: Option<::Value<String>> = None;
                    let mut longitude: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Address" => {
                                address = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Latitude" => {
                                latitude = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Longitude" => {
                                longitude = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Location {
                        address: address,
                        latitude: latitude,
                        longitude: longitude,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod site_to_site_vpn_attachment {
    //! Property types for the `SiteToSiteVpnAttachment` resource.

    /// The [`AWS::NetworkManager::SiteToSiteVpnAttachment.ProposedSegmentChange`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-sitetositevpnattachment-proposedsegmentchange.html) property type.
    #[derive(Debug, Default)]
    pub struct ProposedSegmentChange {
        /// Property [`AttachmentPolicyRuleNumber`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-sitetositevpnattachment-proposedsegmentchange.html#cfn-networkmanager-sitetositevpnattachment-proposedsegmentchange-attachmentpolicyrulenumber).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attachment_policy_rule_number: Option<::Value<u32>>,
        /// Property [`SegmentName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-sitetositevpnattachment-proposedsegmentchange.html#cfn-networkmanager-sitetositevpnattachment-proposedsegmentchange-segmentname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub segment_name: Option<::Value<String>>,
        /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-sitetositevpnattachment-proposedsegmentchange.html#cfn-networkmanager-sitetositevpnattachment-proposedsegmentchange-tags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tags: Option<::ValueList<::Tag>>,
    }

    impl ::codec::SerializeValue for ProposedSegmentChange {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref attachment_policy_rule_number) = self.attachment_policy_rule_number {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttachmentPolicyRuleNumber", attachment_policy_rule_number)?;
            }
            if let Some(ref segment_name) = self.segment_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SegmentName", segment_name)?;
            }
            if let Some(ref tags) = self.tags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ProposedSegmentChange {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ProposedSegmentChange, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ProposedSegmentChange;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ProposedSegmentChange")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attachment_policy_rule_number: Option<::Value<u32>> = None;
                    let mut segment_name: Option<::Value<String>> = None;
                    let mut tags: Option<::ValueList<::Tag>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AttachmentPolicyRuleNumber" => {
                                attachment_policy_rule_number = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SegmentName" => {
                                segment_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Tags" => {
                                tags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ProposedSegmentChange {
                        attachment_policy_rule_number: attachment_policy_rule_number,
                        segment_name: segment_name,
                        tags: tags,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod transit_gateway_route_table_attachment {
    //! Property types for the `TransitGatewayRouteTableAttachment` resource.

    /// The [`AWS::NetworkManager::TransitGatewayRouteTableAttachment.ProposedSegmentChange`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-transitgatewayroutetableattachment-proposedsegmentchange.html) property type.
    #[derive(Debug, Default)]
    pub struct ProposedSegmentChange {
        /// Property [`AttachmentPolicyRuleNumber`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-transitgatewayroutetableattachment-proposedsegmentchange.html#cfn-networkmanager-transitgatewayroutetableattachment-proposedsegmentchange-attachmentpolicyrulenumber).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attachment_policy_rule_number: Option<::Value<u32>>,
        /// Property [`SegmentName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-transitgatewayroutetableattachment-proposedsegmentchange.html#cfn-networkmanager-transitgatewayroutetableattachment-proposedsegmentchange-segmentname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub segment_name: Option<::Value<String>>,
        /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-transitgatewayroutetableattachment-proposedsegmentchange.html#cfn-networkmanager-transitgatewayroutetableattachment-proposedsegmentchange-tags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tags: Option<::ValueList<::Tag>>,
    }

    impl ::codec::SerializeValue for ProposedSegmentChange {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref attachment_policy_rule_number) = self.attachment_policy_rule_number {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttachmentPolicyRuleNumber", attachment_policy_rule_number)?;
            }
            if let Some(ref segment_name) = self.segment_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SegmentName", segment_name)?;
            }
            if let Some(ref tags) = self.tags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ProposedSegmentChange {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ProposedSegmentChange, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ProposedSegmentChange;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ProposedSegmentChange")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attachment_policy_rule_number: Option<::Value<u32>> = None;
                    let mut segment_name: Option<::Value<String>> = None;
                    let mut tags: Option<::ValueList<::Tag>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AttachmentPolicyRuleNumber" => {
                                attachment_policy_rule_number = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SegmentName" => {
                                segment_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Tags" => {
                                tags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ProposedSegmentChange {
                        attachment_policy_rule_number: attachment_policy_rule_number,
                        segment_name: segment_name,
                        tags: tags,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod vpc_attachment {
    //! Property types for the `VpcAttachment` resource.

    /// The [`AWS::NetworkManager::VpcAttachment.ProposedSegmentChange`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-vpcattachment-proposedsegmentchange.html) property type.
    #[derive(Debug, Default)]
    pub struct ProposedSegmentChange {
        /// Property [`AttachmentPolicyRuleNumber`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-vpcattachment-proposedsegmentchange.html#cfn-networkmanager-vpcattachment-proposedsegmentchange-attachmentpolicyrulenumber).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attachment_policy_rule_number: Option<::Value<u32>>,
        /// Property [`SegmentName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-vpcattachment-proposedsegmentchange.html#cfn-networkmanager-vpcattachment-proposedsegmentchange-segmentname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub segment_name: Option<::Value<String>>,
        /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-vpcattachment-proposedsegmentchange.html#cfn-networkmanager-vpcattachment-proposedsegmentchange-tags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tags: Option<::ValueList<::Tag>>,
    }

    impl ::codec::SerializeValue for ProposedSegmentChange {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref attachment_policy_rule_number) = self.attachment_policy_rule_number {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttachmentPolicyRuleNumber", attachment_policy_rule_number)?;
            }
            if let Some(ref segment_name) = self.segment_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SegmentName", segment_name)?;
            }
            if let Some(ref tags) = self.tags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ProposedSegmentChange {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ProposedSegmentChange, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ProposedSegmentChange;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ProposedSegmentChange")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attachment_policy_rule_number: Option<::Value<u32>> = None;
                    let mut segment_name: Option<::Value<String>> = None;
                    let mut tags: Option<::ValueList<::Tag>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AttachmentPolicyRuleNumber" => {
                                attachment_policy_rule_number = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SegmentName" => {
                                segment_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Tags" => {
                                tags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ProposedSegmentChange {
                        attachment_policy_rule_number: attachment_policy_rule_number,
                        segment_name: segment_name,
                        tags: tags,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::NetworkManager::VpcAttachment.VpcOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-vpcattachment-vpcoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct VpcOptions {
        /// Property [`ApplianceModeSupport`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-vpcattachment-vpcoptions.html#cfn-networkmanager-vpcattachment-vpcoptions-appliancemodesupport).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub appliance_mode_support: Option<::Value<bool>>,
        /// Property [`Ipv6Support`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-networkmanager-vpcattachment-vpcoptions.html#cfn-networkmanager-vpcattachment-vpcoptions-ipv6support).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ipv6_support: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for VpcOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref appliance_mode_support) = self.appliance_mode_support {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplianceModeSupport", appliance_mode_support)?;
            }
            if let Some(ref ipv6_support) = self.ipv6_support {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ipv6Support", ipv6_support)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VpcOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VpcOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VpcOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VpcOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut appliance_mode_support: Option<::Value<bool>> = None;
                    let mut ipv6_support: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ApplianceModeSupport" => {
                                appliance_mode_support = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Ipv6Support" => {
                                ipv6_support = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VpcOptions {
                        appliance_mode_support: appliance_mode_support,
                        ipv6_support: ipv6_support,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
