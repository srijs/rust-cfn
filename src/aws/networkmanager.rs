//! Types for the `NetworkManager` service.

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
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-globalnetwork.html#cfn-networkmanager-globalnetwork-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-networkmanager-globalnetwork.html#cfn-networkmanager-globalnetwork-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for GlobalNetworkProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
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
                let mut description: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(GlobalNetworkProperties {
                    description: description,
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

pub mod device {
    //! Property types for the `Device` resource.

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
