//! Types for the `IoTWireless` service.

/// The [`AWS::IoTWireless::Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-destination.html) resource type.
#[derive(Debug, Default)]
pub struct Destination {
    properties: DestinationProperties
}

/// Properties for the `Destination` resource.
#[derive(Debug, Default)]
pub struct DestinationProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-destination.html#cfn-iotwireless-destination-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Expression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-destination.html#cfn-iotwireless-destination-expression).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub expression: ::Value<String>,
    /// Property [`ExpressionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-destination.html#cfn-iotwireless-destination-expressiontype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub expression_type: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-destination.html#cfn-iotwireless-destination-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-destination.html#cfn-iotwireless-destination-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-destination.html#cfn-iotwireless-destination-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for DestinationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Expression", &self.expression)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExpressionType", &self.expression_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DestinationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DestinationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DestinationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DestinationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut expression: Option<::Value<String>> = None;
                let mut expression_type: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Expression" => {
                            expression = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ExpressionType" => {
                            expression_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DestinationProperties {
                    description: description,
                    expression: expression.ok_or(::serde::de::Error::missing_field("Expression"))?,
                    expression_type: expression_type.ok_or(::serde::de::Error::missing_field("ExpressionType"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Destination {
    type Properties = DestinationProperties;
    const TYPE: &'static str = "AWS::IoTWireless::Destination";
    fn properties(&self) -> &DestinationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DestinationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Destination {}

impl From<DestinationProperties> for Destination {
    fn from(properties: DestinationProperties) -> Destination {
        Destination { properties }
    }
}

/// The [`AWS::IoTWireless::DeviceProfile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-deviceprofile.html) resource type.
#[derive(Debug, Default)]
pub struct DeviceProfile {
    properties: DeviceProfileProperties
}

/// Properties for the `DeviceProfile` resource.
#[derive(Debug, Default)]
pub struct DeviceProfileProperties {
    /// Property [`LoRaWAN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-deviceprofile.html#cfn-iotwireless-deviceprofile-lorawan).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub lo_ra_wan: Option<::Value<self::device_profile::LoRaWANDeviceProfile>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-deviceprofile.html#cfn-iotwireless-deviceprofile-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-deviceprofile.html#cfn-iotwireless-deviceprofile-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for DeviceProfileProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref lo_ra_wan) = self.lo_ra_wan {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoRaWAN", lo_ra_wan)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DeviceProfileProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DeviceProfileProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DeviceProfileProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DeviceProfileProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut lo_ra_wan: Option<::Value<self::device_profile::LoRaWANDeviceProfile>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "LoRaWAN" => {
                            lo_ra_wan = ::serde::de::MapAccess::next_value(&mut map)?;
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

                Ok(DeviceProfileProperties {
                    lo_ra_wan: lo_ra_wan,
                    name: name,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DeviceProfile {
    type Properties = DeviceProfileProperties;
    const TYPE: &'static str = "AWS::IoTWireless::DeviceProfile";
    fn properties(&self) -> &DeviceProfileProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DeviceProfileProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DeviceProfile {}

impl From<DeviceProfileProperties> for DeviceProfile {
    fn from(properties: DeviceProfileProperties) -> DeviceProfile {
        DeviceProfile { properties }
    }
}

/// The [`AWS::IoTWireless::FuotaTask`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-fuotatask.html) resource type.
#[derive(Debug, Default)]
pub struct FuotaTask {
    properties: FuotaTaskProperties
}

/// Properties for the `FuotaTask` resource.
#[derive(Debug, Default)]
pub struct FuotaTaskProperties {
    /// Property [`AssociateMulticastGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-fuotatask.html#cfn-iotwireless-fuotatask-associatemulticastgroup).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub associate_multicast_group: Option<::Value<String>>,
    /// Property [`AssociateWirelessDevice`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-fuotatask.html#cfn-iotwireless-fuotatask-associatewirelessdevice).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub associate_wireless_device: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-fuotatask.html#cfn-iotwireless-fuotatask-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`DisassociateMulticastGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-fuotatask.html#cfn-iotwireless-fuotatask-disassociatemulticastgroup).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub disassociate_multicast_group: Option<::Value<String>>,
    /// Property [`DisassociateWirelessDevice`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-fuotatask.html#cfn-iotwireless-fuotatask-disassociatewirelessdevice).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub disassociate_wireless_device: Option<::Value<String>>,
    /// Property [`FirmwareUpdateImage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-fuotatask.html#cfn-iotwireless-fuotatask-firmwareupdateimage).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub firmware_update_image: ::Value<String>,
    /// Property [`FirmwareUpdateRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-fuotatask.html#cfn-iotwireless-fuotatask-firmwareupdaterole).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub firmware_update_role: ::Value<String>,
    /// Property [`LoRaWAN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-fuotatask.html#cfn-iotwireless-fuotatask-lorawan).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub lo_ra_wan: ::Value<self::fuota_task::LoRaWAN>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-fuotatask.html#cfn-iotwireless-fuotatask-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-fuotatask.html#cfn-iotwireless-fuotatask-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for FuotaTaskProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref associate_multicast_group) = self.associate_multicast_group {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssociateMulticastGroup", associate_multicast_group)?;
        }
        if let Some(ref associate_wireless_device) = self.associate_wireless_device {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssociateWirelessDevice", associate_wireless_device)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref disassociate_multicast_group) = self.disassociate_multicast_group {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisassociateMulticastGroup", disassociate_multicast_group)?;
        }
        if let Some(ref disassociate_wireless_device) = self.disassociate_wireless_device {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisassociateWirelessDevice", disassociate_wireless_device)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FirmwareUpdateImage", &self.firmware_update_image)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FirmwareUpdateRole", &self.firmware_update_role)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoRaWAN", &self.lo_ra_wan)?;
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for FuotaTaskProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<FuotaTaskProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = FuotaTaskProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type FuotaTaskProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut associate_multicast_group: Option<::Value<String>> = None;
                let mut associate_wireless_device: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut disassociate_multicast_group: Option<::Value<String>> = None;
                let mut disassociate_wireless_device: Option<::Value<String>> = None;
                let mut firmware_update_image: Option<::Value<String>> = None;
                let mut firmware_update_role: Option<::Value<String>> = None;
                let mut lo_ra_wan: Option<::Value<self::fuota_task::LoRaWAN>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AssociateMulticastGroup" => {
                            associate_multicast_group = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AssociateWirelessDevice" => {
                            associate_wireless_device = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DisassociateMulticastGroup" => {
                            disassociate_multicast_group = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DisassociateWirelessDevice" => {
                            disassociate_wireless_device = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FirmwareUpdateImage" => {
                            firmware_update_image = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FirmwareUpdateRole" => {
                            firmware_update_role = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LoRaWAN" => {
                            lo_ra_wan = ::serde::de::MapAccess::next_value(&mut map)?;
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

                Ok(FuotaTaskProperties {
                    associate_multicast_group: associate_multicast_group,
                    associate_wireless_device: associate_wireless_device,
                    description: description,
                    disassociate_multicast_group: disassociate_multicast_group,
                    disassociate_wireless_device: disassociate_wireless_device,
                    firmware_update_image: firmware_update_image.ok_or(::serde::de::Error::missing_field("FirmwareUpdateImage"))?,
                    firmware_update_role: firmware_update_role.ok_or(::serde::de::Error::missing_field("FirmwareUpdateRole"))?,
                    lo_ra_wan: lo_ra_wan.ok_or(::serde::de::Error::missing_field("LoRaWAN"))?,
                    name: name,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for FuotaTask {
    type Properties = FuotaTaskProperties;
    const TYPE: &'static str = "AWS::IoTWireless::FuotaTask";
    fn properties(&self) -> &FuotaTaskProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut FuotaTaskProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for FuotaTask {}

impl From<FuotaTaskProperties> for FuotaTask {
    fn from(properties: FuotaTaskProperties) -> FuotaTask {
        FuotaTask { properties }
    }
}

/// The [`AWS::IoTWireless::MulticastGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-multicastgroup.html) resource type.
#[derive(Debug, Default)]
pub struct MulticastGroup {
    properties: MulticastGroupProperties
}

/// Properties for the `MulticastGroup` resource.
#[derive(Debug, Default)]
pub struct MulticastGroupProperties {
    /// Property [`AssociateWirelessDevice`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-multicastgroup.html#cfn-iotwireless-multicastgroup-associatewirelessdevice).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub associate_wireless_device: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-multicastgroup.html#cfn-iotwireless-multicastgroup-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`DisassociateWirelessDevice`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-multicastgroup.html#cfn-iotwireless-multicastgroup-disassociatewirelessdevice).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub disassociate_wireless_device: Option<::Value<String>>,
    /// Property [`LoRaWAN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-multicastgroup.html#cfn-iotwireless-multicastgroup-lorawan).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub lo_ra_wan: ::Value<self::multicast_group::LoRaWAN>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-multicastgroup.html#cfn-iotwireless-multicastgroup-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-multicastgroup.html#cfn-iotwireless-multicastgroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for MulticastGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref associate_wireless_device) = self.associate_wireless_device {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssociateWirelessDevice", associate_wireless_device)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref disassociate_wireless_device) = self.disassociate_wireless_device {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisassociateWirelessDevice", disassociate_wireless_device)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoRaWAN", &self.lo_ra_wan)?;
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for MulticastGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<MulticastGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = MulticastGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type MulticastGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut associate_wireless_device: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut disassociate_wireless_device: Option<::Value<String>> = None;
                let mut lo_ra_wan: Option<::Value<self::multicast_group::LoRaWAN>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AssociateWirelessDevice" => {
                            associate_wireless_device = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DisassociateWirelessDevice" => {
                            disassociate_wireless_device = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LoRaWAN" => {
                            lo_ra_wan = ::serde::de::MapAccess::next_value(&mut map)?;
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

                Ok(MulticastGroupProperties {
                    associate_wireless_device: associate_wireless_device,
                    description: description,
                    disassociate_wireless_device: disassociate_wireless_device,
                    lo_ra_wan: lo_ra_wan.ok_or(::serde::de::Error::missing_field("LoRaWAN"))?,
                    name: name,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for MulticastGroup {
    type Properties = MulticastGroupProperties;
    const TYPE: &'static str = "AWS::IoTWireless::MulticastGroup";
    fn properties(&self) -> &MulticastGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut MulticastGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for MulticastGroup {}

impl From<MulticastGroupProperties> for MulticastGroup {
    fn from(properties: MulticastGroupProperties) -> MulticastGroup {
        MulticastGroup { properties }
    }
}

/// The [`AWS::IoTWireless::PartnerAccount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-partneraccount.html) resource type.
#[derive(Debug, Default)]
pub struct PartnerAccount {
    properties: PartnerAccountProperties
}

/// Properties for the `PartnerAccount` resource.
#[derive(Debug, Default)]
pub struct PartnerAccountProperties {
    /// Property [`AccountLinked`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-partneraccount.html#cfn-iotwireless-partneraccount-accountlinked).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub account_linked: Option<::Value<bool>>,
    /// Property [`Fingerprint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-partneraccount.html#cfn-iotwireless-partneraccount-fingerprint).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub fingerprint: Option<::Value<String>>,
    /// Property [`PartnerAccountId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-partneraccount.html#cfn-iotwireless-partneraccount-partneraccountid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub partner_account_id: Option<::Value<String>>,
    /// Property [`PartnerType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-partneraccount.html#cfn-iotwireless-partneraccount-partnertype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub partner_type: Option<::Value<String>>,
    /// Property [`Sidewalk`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-partneraccount.html#cfn-iotwireless-partneraccount-sidewalk).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sidewalk: Option<::Value<self::partner_account::SidewalkAccountInfo>>,
    /// Property [`SidewalkUpdate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-partneraccount.html#cfn-iotwireless-partneraccount-sidewalkupdate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sidewalk_update: Option<::Value<self::partner_account::SidewalkUpdateAccount>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-partneraccount.html#cfn-iotwireless-partneraccount-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for PartnerAccountProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref account_linked) = self.account_linked {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccountLinked", account_linked)?;
        }
        if let Some(ref fingerprint) = self.fingerprint {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Fingerprint", fingerprint)?;
        }
        if let Some(ref partner_account_id) = self.partner_account_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PartnerAccountId", partner_account_id)?;
        }
        if let Some(ref partner_type) = self.partner_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PartnerType", partner_type)?;
        }
        if let Some(ref sidewalk) = self.sidewalk {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Sidewalk", sidewalk)?;
        }
        if let Some(ref sidewalk_update) = self.sidewalk_update {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SidewalkUpdate", sidewalk_update)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PartnerAccountProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PartnerAccountProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PartnerAccountProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PartnerAccountProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut account_linked: Option<::Value<bool>> = None;
                let mut fingerprint: Option<::Value<String>> = None;
                let mut partner_account_id: Option<::Value<String>> = None;
                let mut partner_type: Option<::Value<String>> = None;
                let mut sidewalk: Option<::Value<self::partner_account::SidewalkAccountInfo>> = None;
                let mut sidewalk_update: Option<::Value<self::partner_account::SidewalkUpdateAccount>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AccountLinked" => {
                            account_linked = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Fingerprint" => {
                            fingerprint = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PartnerAccountId" => {
                            partner_account_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PartnerType" => {
                            partner_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Sidewalk" => {
                            sidewalk = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SidewalkUpdate" => {
                            sidewalk_update = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PartnerAccountProperties {
                    account_linked: account_linked,
                    fingerprint: fingerprint,
                    partner_account_id: partner_account_id,
                    partner_type: partner_type,
                    sidewalk: sidewalk,
                    sidewalk_update: sidewalk_update,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for PartnerAccount {
    type Properties = PartnerAccountProperties;
    const TYPE: &'static str = "AWS::IoTWireless::PartnerAccount";
    fn properties(&self) -> &PartnerAccountProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PartnerAccountProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for PartnerAccount {}

impl From<PartnerAccountProperties> for PartnerAccount {
    fn from(properties: PartnerAccountProperties) -> PartnerAccount {
        PartnerAccount { properties }
    }
}

/// The [`AWS::IoTWireless::ServiceProfile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-serviceprofile.html) resource type.
#[derive(Debug, Default)]
pub struct ServiceProfile {
    properties: ServiceProfileProperties
}

/// Properties for the `ServiceProfile` resource.
#[derive(Debug, Default)]
pub struct ServiceProfileProperties {
    /// Property [`LoRaWAN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-serviceprofile.html#cfn-iotwireless-serviceprofile-lorawan).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub lo_ra_wan: Option<::Value<self::service_profile::LoRaWANServiceProfile>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-serviceprofile.html#cfn-iotwireless-serviceprofile-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-serviceprofile.html#cfn-iotwireless-serviceprofile-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ServiceProfileProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref lo_ra_wan) = self.lo_ra_wan {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoRaWAN", lo_ra_wan)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ServiceProfileProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ServiceProfileProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ServiceProfileProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ServiceProfileProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut lo_ra_wan: Option<::Value<self::service_profile::LoRaWANServiceProfile>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "LoRaWAN" => {
                            lo_ra_wan = ::serde::de::MapAccess::next_value(&mut map)?;
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

                Ok(ServiceProfileProperties {
                    lo_ra_wan: lo_ra_wan,
                    name: name,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ServiceProfile {
    type Properties = ServiceProfileProperties;
    const TYPE: &'static str = "AWS::IoTWireless::ServiceProfile";
    fn properties(&self) -> &ServiceProfileProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ServiceProfileProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ServiceProfile {}

impl From<ServiceProfileProperties> for ServiceProfile {
    fn from(properties: ServiceProfileProperties) -> ServiceProfile {
        ServiceProfile { properties }
    }
}

/// The [`AWS::IoTWireless::TaskDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-taskdefinition.html) resource type.
#[derive(Debug, Default)]
pub struct TaskDefinition {
    properties: TaskDefinitionProperties
}

/// Properties for the `TaskDefinition` resource.
#[derive(Debug, Default)]
pub struct TaskDefinitionProperties {
    /// Property [`AutoCreateTasks`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-taskdefinition.html#cfn-iotwireless-taskdefinition-autocreatetasks).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub auto_create_tasks: ::Value<bool>,
    /// Property [`LoRaWANUpdateGatewayTaskEntry`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-taskdefinition.html#cfn-iotwireless-taskdefinition-lorawanupdategatewaytaskentry).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub lo_ra_wan_update_gateway_task_entry: Option<::Value<self::task_definition::LoRaWANUpdateGatewayTaskEntry>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-taskdefinition.html#cfn-iotwireless-taskdefinition-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-taskdefinition.html#cfn-iotwireless-taskdefinition-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TaskDefinitionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-taskdefinition.html#cfn-iotwireless-taskdefinition-taskdefinitiontype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub task_definition_type: Option<::Value<String>>,
    /// Property [`Update`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-taskdefinition.html#cfn-iotwireless-taskdefinition-update).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub update: Option<::Value<self::task_definition::UpdateWirelessGatewayTaskCreate>>,
}

impl ::serde::Serialize for TaskDefinitionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoCreateTasks", &self.auto_create_tasks)?;
        if let Some(ref lo_ra_wan_update_gateway_task_entry) = self.lo_ra_wan_update_gateway_task_entry {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoRaWANUpdateGatewayTaskEntry", lo_ra_wan_update_gateway_task_entry)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref task_definition_type) = self.task_definition_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TaskDefinitionType", task_definition_type)?;
        }
        if let Some(ref update) = self.update {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Update", update)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for TaskDefinitionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<TaskDefinitionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TaskDefinitionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type TaskDefinitionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut auto_create_tasks: Option<::Value<bool>> = None;
                let mut lo_ra_wan_update_gateway_task_entry: Option<::Value<self::task_definition::LoRaWANUpdateGatewayTaskEntry>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut task_definition_type: Option<::Value<String>> = None;
                let mut update: Option<::Value<self::task_definition::UpdateWirelessGatewayTaskCreate>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AutoCreateTasks" => {
                            auto_create_tasks = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LoRaWANUpdateGatewayTaskEntry" => {
                            lo_ra_wan_update_gateway_task_entry = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TaskDefinitionType" => {
                            task_definition_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Update" => {
                            update = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(TaskDefinitionProperties {
                    auto_create_tasks: auto_create_tasks.ok_or(::serde::de::Error::missing_field("AutoCreateTasks"))?,
                    lo_ra_wan_update_gateway_task_entry: lo_ra_wan_update_gateway_task_entry,
                    name: name,
                    tags: tags,
                    task_definition_type: task_definition_type,
                    update: update,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for TaskDefinition {
    type Properties = TaskDefinitionProperties;
    const TYPE: &'static str = "AWS::IoTWireless::TaskDefinition";
    fn properties(&self) -> &TaskDefinitionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TaskDefinitionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for TaskDefinition {}

impl From<TaskDefinitionProperties> for TaskDefinition {
    fn from(properties: TaskDefinitionProperties) -> TaskDefinition {
        TaskDefinition { properties }
    }
}

/// The [`AWS::IoTWireless::WirelessDevice`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-wirelessdevice.html) resource type.
#[derive(Debug, Default)]
pub struct WirelessDevice {
    properties: WirelessDeviceProperties
}

/// Properties for the `WirelessDevice` resource.
#[derive(Debug, Default)]
pub struct WirelessDeviceProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-wirelessdevice.html#cfn-iotwireless-wirelessdevice-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`DestinationName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-wirelessdevice.html#cfn-iotwireless-wirelessdevice-destinationname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub destination_name: ::Value<String>,
    /// Property [`LastUplinkReceivedAt`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-wirelessdevice.html#cfn-iotwireless-wirelessdevice-lastuplinkreceivedat).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub last_uplink_received_at: Option<::Value<String>>,
    /// Property [`LoRaWAN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-wirelessdevice.html#cfn-iotwireless-wirelessdevice-lorawan).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub lo_ra_wan: Option<::Value<self::wireless_device::LoRaWANDevice>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-wirelessdevice.html#cfn-iotwireless-wirelessdevice-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-wirelessdevice.html#cfn-iotwireless-wirelessdevice-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`ThingArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-wirelessdevice.html#cfn-iotwireless-wirelessdevice-thingarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub thing_arn: Option<::Value<String>>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-wirelessdevice.html#cfn-iotwireless-wirelessdevice-type).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub r#type: ::Value<String>,
}

impl ::serde::Serialize for WirelessDeviceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationName", &self.destination_name)?;
        if let Some(ref last_uplink_received_at) = self.last_uplink_received_at {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LastUplinkReceivedAt", last_uplink_received_at)?;
        }
        if let Some(ref lo_ra_wan) = self.lo_ra_wan {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoRaWAN", lo_ra_wan)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref thing_arn) = self.thing_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThingArn", thing_arn)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for WirelessDeviceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<WirelessDeviceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = WirelessDeviceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type WirelessDeviceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut destination_name: Option<::Value<String>> = None;
                let mut last_uplink_received_at: Option<::Value<String>> = None;
                let mut lo_ra_wan: Option<::Value<self::wireless_device::LoRaWANDevice>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut thing_arn: Option<::Value<String>> = None;
                let mut r#type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DestinationName" => {
                            destination_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LastUplinkReceivedAt" => {
                            last_uplink_received_at = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LoRaWAN" => {
                            lo_ra_wan = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ThingArn" => {
                            thing_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Type" => {
                            r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(WirelessDeviceProperties {
                    description: description,
                    destination_name: destination_name.ok_or(::serde::de::Error::missing_field("DestinationName"))?,
                    last_uplink_received_at: last_uplink_received_at,
                    lo_ra_wan: lo_ra_wan,
                    name: name,
                    tags: tags,
                    thing_arn: thing_arn,
                    r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for WirelessDevice {
    type Properties = WirelessDeviceProperties;
    const TYPE: &'static str = "AWS::IoTWireless::WirelessDevice";
    fn properties(&self) -> &WirelessDeviceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut WirelessDeviceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for WirelessDevice {}

impl From<WirelessDeviceProperties> for WirelessDevice {
    fn from(properties: WirelessDeviceProperties) -> WirelessDevice {
        WirelessDevice { properties }
    }
}

/// The [`AWS::IoTWireless::WirelessGateway`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-wirelessgateway.html) resource type.
#[derive(Debug, Default)]
pub struct WirelessGateway {
    properties: WirelessGatewayProperties
}

/// Properties for the `WirelessGateway` resource.
#[derive(Debug, Default)]
pub struct WirelessGatewayProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-wirelessgateway.html#cfn-iotwireless-wirelessgateway-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`LastUplinkReceivedAt`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-wirelessgateway.html#cfn-iotwireless-wirelessgateway-lastuplinkreceivedat).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub last_uplink_received_at: Option<::Value<String>>,
    /// Property [`LoRaWAN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-wirelessgateway.html#cfn-iotwireless-wirelessgateway-lorawan).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub lo_ra_wan: ::Value<self::wireless_gateway::LoRaWANGateway>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-wirelessgateway.html#cfn-iotwireless-wirelessgateway-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-wirelessgateway.html#cfn-iotwireless-wirelessgateway-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`ThingArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotwireless-wirelessgateway.html#cfn-iotwireless-wirelessgateway-thingarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub thing_arn: Option<::Value<String>>,
}

impl ::serde::Serialize for WirelessGatewayProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref last_uplink_received_at) = self.last_uplink_received_at {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LastUplinkReceivedAt", last_uplink_received_at)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoRaWAN", &self.lo_ra_wan)?;
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref thing_arn) = self.thing_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThingArn", thing_arn)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for WirelessGatewayProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<WirelessGatewayProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = WirelessGatewayProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type WirelessGatewayProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut last_uplink_received_at: Option<::Value<String>> = None;
                let mut lo_ra_wan: Option<::Value<self::wireless_gateway::LoRaWANGateway>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut thing_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LastUplinkReceivedAt" => {
                            last_uplink_received_at = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LoRaWAN" => {
                            lo_ra_wan = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ThingArn" => {
                            thing_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(WirelessGatewayProperties {
                    description: description,
                    last_uplink_received_at: last_uplink_received_at,
                    lo_ra_wan: lo_ra_wan.ok_or(::serde::de::Error::missing_field("LoRaWAN"))?,
                    name: name,
                    tags: tags,
                    thing_arn: thing_arn,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for WirelessGateway {
    type Properties = WirelessGatewayProperties;
    const TYPE: &'static str = "AWS::IoTWireless::WirelessGateway";
    fn properties(&self) -> &WirelessGatewayProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut WirelessGatewayProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for WirelessGateway {}

impl From<WirelessGatewayProperties> for WirelessGateway {
    fn from(properties: WirelessGatewayProperties) -> WirelessGateway {
        WirelessGateway { properties }
    }
}

pub mod device_profile {
    //! Property types for the `DeviceProfile` resource.

    /// The [`AWS::IoTWireless::DeviceProfile.LoRaWANDeviceProfile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-deviceprofile-lorawandeviceprofile.html) property type.
    #[derive(Debug, Default)]
    pub struct LoRaWANDeviceProfile {
        /// Property [`ClassBTimeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-deviceprofile-lorawandeviceprofile.html#cfn-iotwireless-deviceprofile-lorawandeviceprofile-classbtimeout).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub class_b_timeout: Option<::Value<u32>>,
        /// Property [`ClassCTimeout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-deviceprofile-lorawandeviceprofile.html#cfn-iotwireless-deviceprofile-lorawandeviceprofile-classctimeout).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub class_c_timeout: Option<::Value<u32>>,
        /// Property [`MacVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-deviceprofile-lorawandeviceprofile.html#cfn-iotwireless-deviceprofile-lorawandeviceprofile-macversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mac_version: Option<::Value<String>>,
        /// Property [`MaxDutyCycle`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-deviceprofile-lorawandeviceprofile.html#cfn-iotwireless-deviceprofile-lorawandeviceprofile-maxdutycycle).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_duty_cycle: Option<::Value<u32>>,
        /// Property [`MaxEirp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-deviceprofile-lorawandeviceprofile.html#cfn-iotwireless-deviceprofile-lorawandeviceprofile-maxeirp).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_eirp: Option<::Value<u32>>,
        /// Property [`PingSlotDr`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-deviceprofile-lorawandeviceprofile.html#cfn-iotwireless-deviceprofile-lorawandeviceprofile-pingslotdr).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ping_slot_dr: Option<::Value<u32>>,
        /// Property [`PingSlotFreq`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-deviceprofile-lorawandeviceprofile.html#cfn-iotwireless-deviceprofile-lorawandeviceprofile-pingslotfreq).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ping_slot_freq: Option<::Value<u32>>,
        /// Property [`PingSlotPeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-deviceprofile-lorawandeviceprofile.html#cfn-iotwireless-deviceprofile-lorawandeviceprofile-pingslotperiod).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ping_slot_period: Option<::Value<u32>>,
        /// Property [`RegParamsRevision`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-deviceprofile-lorawandeviceprofile.html#cfn-iotwireless-deviceprofile-lorawandeviceprofile-regparamsrevision).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub reg_params_revision: Option<::Value<String>>,
        /// Property [`RfRegion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-deviceprofile-lorawandeviceprofile.html#cfn-iotwireless-deviceprofile-lorawandeviceprofile-rfregion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rf_region: Option<::Value<String>>,
        /// Property [`Supports32BitFCnt`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-deviceprofile-lorawandeviceprofile.html#cfn-iotwireless-deviceprofile-lorawandeviceprofile-supports32bitfcnt).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub supports32_bit_f_cnt: Option<::Value<bool>>,
        /// Property [`SupportsClassB`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-deviceprofile-lorawandeviceprofile.html#cfn-iotwireless-deviceprofile-lorawandeviceprofile-supportsclassb).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub supports_class_b: Option<::Value<bool>>,
        /// Property [`SupportsClassC`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-deviceprofile-lorawandeviceprofile.html#cfn-iotwireless-deviceprofile-lorawandeviceprofile-supportsclassc).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub supports_class_c: Option<::Value<bool>>,
        /// Property [`SupportsJoin`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-deviceprofile-lorawandeviceprofile.html#cfn-iotwireless-deviceprofile-lorawandeviceprofile-supportsjoin).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub supports_join: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for LoRaWANDeviceProfile {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref class_b_timeout) = self.class_b_timeout {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClassBTimeout", class_b_timeout)?;
            }
            if let Some(ref class_c_timeout) = self.class_c_timeout {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClassCTimeout", class_c_timeout)?;
            }
            if let Some(ref mac_version) = self.mac_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MacVersion", mac_version)?;
            }
            if let Some(ref max_duty_cycle) = self.max_duty_cycle {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxDutyCycle", max_duty_cycle)?;
            }
            if let Some(ref max_eirp) = self.max_eirp {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxEirp", max_eirp)?;
            }
            if let Some(ref ping_slot_dr) = self.ping_slot_dr {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PingSlotDr", ping_slot_dr)?;
            }
            if let Some(ref ping_slot_freq) = self.ping_slot_freq {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PingSlotFreq", ping_slot_freq)?;
            }
            if let Some(ref ping_slot_period) = self.ping_slot_period {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PingSlotPeriod", ping_slot_period)?;
            }
            if let Some(ref reg_params_revision) = self.reg_params_revision {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RegParamsRevision", reg_params_revision)?;
            }
            if let Some(ref rf_region) = self.rf_region {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RfRegion", rf_region)?;
            }
            if let Some(ref supports32_bit_f_cnt) = self.supports32_bit_f_cnt {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Supports32BitFCnt", supports32_bit_f_cnt)?;
            }
            if let Some(ref supports_class_b) = self.supports_class_b {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SupportsClassB", supports_class_b)?;
            }
            if let Some(ref supports_class_c) = self.supports_class_c {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SupportsClassC", supports_class_c)?;
            }
            if let Some(ref supports_join) = self.supports_join {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SupportsJoin", supports_join)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LoRaWANDeviceProfile {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LoRaWANDeviceProfile, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LoRaWANDeviceProfile;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LoRaWANDeviceProfile")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut class_b_timeout: Option<::Value<u32>> = None;
                    let mut class_c_timeout: Option<::Value<u32>> = None;
                    let mut mac_version: Option<::Value<String>> = None;
                    let mut max_duty_cycle: Option<::Value<u32>> = None;
                    let mut max_eirp: Option<::Value<u32>> = None;
                    let mut ping_slot_dr: Option<::Value<u32>> = None;
                    let mut ping_slot_freq: Option<::Value<u32>> = None;
                    let mut ping_slot_period: Option<::Value<u32>> = None;
                    let mut reg_params_revision: Option<::Value<String>> = None;
                    let mut rf_region: Option<::Value<String>> = None;
                    let mut supports32_bit_f_cnt: Option<::Value<bool>> = None;
                    let mut supports_class_b: Option<::Value<bool>> = None;
                    let mut supports_class_c: Option<::Value<bool>> = None;
                    let mut supports_join: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ClassBTimeout" => {
                                class_b_timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ClassCTimeout" => {
                                class_c_timeout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MacVersion" => {
                                mac_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxDutyCycle" => {
                                max_duty_cycle = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxEirp" => {
                                max_eirp = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PingSlotDr" => {
                                ping_slot_dr = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PingSlotFreq" => {
                                ping_slot_freq = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PingSlotPeriod" => {
                                ping_slot_period = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RegParamsRevision" => {
                                reg_params_revision = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RfRegion" => {
                                rf_region = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Supports32BitFCnt" => {
                                supports32_bit_f_cnt = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SupportsClassB" => {
                                supports_class_b = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SupportsClassC" => {
                                supports_class_c = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SupportsJoin" => {
                                supports_join = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LoRaWANDeviceProfile {
                        class_b_timeout: class_b_timeout,
                        class_c_timeout: class_c_timeout,
                        mac_version: mac_version,
                        max_duty_cycle: max_duty_cycle,
                        max_eirp: max_eirp,
                        ping_slot_dr: ping_slot_dr,
                        ping_slot_freq: ping_slot_freq,
                        ping_slot_period: ping_slot_period,
                        reg_params_revision: reg_params_revision,
                        rf_region: rf_region,
                        supports32_bit_f_cnt: supports32_bit_f_cnt,
                        supports_class_b: supports_class_b,
                        supports_class_c: supports_class_c,
                        supports_join: supports_join,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod fuota_task {
    //! Property types for the `FuotaTask` resource.

    /// The [`AWS::IoTWireless::FuotaTask.LoRaWAN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-fuotatask-lorawan.html) property type.
    #[derive(Debug, Default)]
    pub struct LoRaWAN {
        /// Property [`RfRegion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-fuotatask-lorawan.html#cfn-iotwireless-fuotatask-lorawan-rfregion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rf_region: ::Value<String>,
        /// Property [`StartTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-fuotatask-lorawan.html#cfn-iotwireless-fuotatask-lorawan-starttime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub start_time: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LoRaWAN {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RfRegion", &self.rf_region)?;
            if let Some(ref start_time) = self.start_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartTime", start_time)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LoRaWAN {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LoRaWAN, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LoRaWAN;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LoRaWAN")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut rf_region: Option<::Value<String>> = None;
                    let mut start_time: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RfRegion" => {
                                rf_region = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StartTime" => {
                                start_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LoRaWAN {
                        rf_region: rf_region.ok_or(::serde::de::Error::missing_field("RfRegion"))?,
                        start_time: start_time,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod multicast_group {
    //! Property types for the `MulticastGroup` resource.

    /// The [`AWS::IoTWireless::MulticastGroup.LoRaWAN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-multicastgroup-lorawan.html) property type.
    #[derive(Debug, Default)]
    pub struct LoRaWAN {
        /// Property [`DlClass`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-multicastgroup-lorawan.html#cfn-iotwireless-multicastgroup-lorawan-dlclass).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dl_class: ::Value<String>,
        /// Property [`NumberOfDevicesInGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-multicastgroup-lorawan.html#cfn-iotwireless-multicastgroup-lorawan-numberofdevicesingroup).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub number_of_devices_in_group: Option<::Value<u32>>,
        /// Property [`NumberOfDevicesRequested`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-multicastgroup-lorawan.html#cfn-iotwireless-multicastgroup-lorawan-numberofdevicesrequested).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub number_of_devices_requested: Option<::Value<u32>>,
        /// Property [`RfRegion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-multicastgroup-lorawan.html#cfn-iotwireless-multicastgroup-lorawan-rfregion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rf_region: ::Value<String>,
    }

    impl ::codec::SerializeValue for LoRaWAN {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DlClass", &self.dl_class)?;
            if let Some(ref number_of_devices_in_group) = self.number_of_devices_in_group {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumberOfDevicesInGroup", number_of_devices_in_group)?;
            }
            if let Some(ref number_of_devices_requested) = self.number_of_devices_requested {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumberOfDevicesRequested", number_of_devices_requested)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RfRegion", &self.rf_region)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LoRaWAN {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LoRaWAN, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LoRaWAN;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LoRaWAN")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dl_class: Option<::Value<String>> = None;
                    let mut number_of_devices_in_group: Option<::Value<u32>> = None;
                    let mut number_of_devices_requested: Option<::Value<u32>> = None;
                    let mut rf_region: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DlClass" => {
                                dl_class = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NumberOfDevicesInGroup" => {
                                number_of_devices_in_group = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NumberOfDevicesRequested" => {
                                number_of_devices_requested = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RfRegion" => {
                                rf_region = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LoRaWAN {
                        dl_class: dl_class.ok_or(::serde::de::Error::missing_field("DlClass"))?,
                        number_of_devices_in_group: number_of_devices_in_group,
                        number_of_devices_requested: number_of_devices_requested,
                        rf_region: rf_region.ok_or(::serde::de::Error::missing_field("RfRegion"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod partner_account {
    //! Property types for the `PartnerAccount` resource.

    /// The [`AWS::IoTWireless::PartnerAccount.SidewalkAccountInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-partneraccount-sidewalkaccountinfo.html) property type.
    #[derive(Debug, Default)]
    pub struct SidewalkAccountInfo {
        /// Property [`AppServerPrivateKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-partneraccount-sidewalkaccountinfo.html#cfn-iotwireless-partneraccount-sidewalkaccountinfo-appserverprivatekey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub app_server_private_key: ::Value<String>,
    }

    impl ::codec::SerializeValue for SidewalkAccountInfo {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AppServerPrivateKey", &self.app_server_private_key)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SidewalkAccountInfo {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SidewalkAccountInfo, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SidewalkAccountInfo;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SidewalkAccountInfo")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut app_server_private_key: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AppServerPrivateKey" => {
                                app_server_private_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SidewalkAccountInfo {
                        app_server_private_key: app_server_private_key.ok_or(::serde::de::Error::missing_field("AppServerPrivateKey"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTWireless::PartnerAccount.SidewalkUpdateAccount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-partneraccount-sidewalkupdateaccount.html) property type.
    #[derive(Debug, Default)]
    pub struct SidewalkUpdateAccount {
        /// Property [`AppServerPrivateKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-partneraccount-sidewalkupdateaccount.html#cfn-iotwireless-partneraccount-sidewalkupdateaccount-appserverprivatekey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub app_server_private_key: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SidewalkUpdateAccount {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref app_server_private_key) = self.app_server_private_key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AppServerPrivateKey", app_server_private_key)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SidewalkUpdateAccount {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SidewalkUpdateAccount, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SidewalkUpdateAccount;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SidewalkUpdateAccount")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut app_server_private_key: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AppServerPrivateKey" => {
                                app_server_private_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SidewalkUpdateAccount {
                        app_server_private_key: app_server_private_key,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod service_profile {
    //! Property types for the `ServiceProfile` resource.

    /// The [`AWS::IoTWireless::ServiceProfile.LoRaWANServiceProfile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-serviceprofile-lorawanserviceprofile.html) property type.
    #[derive(Debug, Default)]
    pub struct LoRaWANServiceProfile {
        /// Property [`AddGwMetadata`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-serviceprofile-lorawanserviceprofile.html#cfn-iotwireless-serviceprofile-lorawanserviceprofile-addgwmetadata).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub add_gw_metadata: Option<::Value<bool>>,
        /// Property [`ChannelMask`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-serviceprofile-lorawanserviceprofile.html#cfn-iotwireless-serviceprofile-lorawanserviceprofile-channelmask).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub channel_mask: Option<::Value<String>>,
        /// Property [`DevStatusReqFreq`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-serviceprofile-lorawanserviceprofile.html#cfn-iotwireless-serviceprofile-lorawanserviceprofile-devstatusreqfreq).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dev_status_req_freq: Option<::Value<u32>>,
        /// Property [`DlBucketSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-serviceprofile-lorawanserviceprofile.html#cfn-iotwireless-serviceprofile-lorawanserviceprofile-dlbucketsize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dl_bucket_size: Option<::Value<u32>>,
        /// Property [`DlRate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-serviceprofile-lorawanserviceprofile.html#cfn-iotwireless-serviceprofile-lorawanserviceprofile-dlrate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dl_rate: Option<::Value<u32>>,
        /// Property [`DlRatePolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-serviceprofile-lorawanserviceprofile.html#cfn-iotwireless-serviceprofile-lorawanserviceprofile-dlratepolicy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dl_rate_policy: Option<::Value<String>>,
        /// Property [`DrMax`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-serviceprofile-lorawanserviceprofile.html#cfn-iotwireless-serviceprofile-lorawanserviceprofile-drmax).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dr_max: Option<::Value<u32>>,
        /// Property [`DrMin`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-serviceprofile-lorawanserviceprofile.html#cfn-iotwireless-serviceprofile-lorawanserviceprofile-drmin).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dr_min: Option<::Value<u32>>,
        /// Property [`HrAllowed`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-serviceprofile-lorawanserviceprofile.html#cfn-iotwireless-serviceprofile-lorawanserviceprofile-hrallowed).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hr_allowed: Option<::Value<bool>>,
        /// Property [`MinGwDiversity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-serviceprofile-lorawanserviceprofile.html#cfn-iotwireless-serviceprofile-lorawanserviceprofile-mingwdiversity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min_gw_diversity: Option<::Value<u32>>,
        /// Property [`NwkGeoLoc`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-serviceprofile-lorawanserviceprofile.html#cfn-iotwireless-serviceprofile-lorawanserviceprofile-nwkgeoloc).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub nwk_geo_loc: Option<::Value<bool>>,
        /// Property [`PrAllowed`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-serviceprofile-lorawanserviceprofile.html#cfn-iotwireless-serviceprofile-lorawanserviceprofile-prallowed).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pr_allowed: Option<::Value<bool>>,
        /// Property [`RaAllowed`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-serviceprofile-lorawanserviceprofile.html#cfn-iotwireless-serviceprofile-lorawanserviceprofile-raallowed).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ra_allowed: Option<::Value<bool>>,
        /// Property [`ReportDevStatusBattery`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-serviceprofile-lorawanserviceprofile.html#cfn-iotwireless-serviceprofile-lorawanserviceprofile-reportdevstatusbattery).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub report_dev_status_battery: Option<::Value<bool>>,
        /// Property [`ReportDevStatusMargin`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-serviceprofile-lorawanserviceprofile.html#cfn-iotwireless-serviceprofile-lorawanserviceprofile-reportdevstatusmargin).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub report_dev_status_margin: Option<::Value<bool>>,
        /// Property [`TargetPer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-serviceprofile-lorawanserviceprofile.html#cfn-iotwireless-serviceprofile-lorawanserviceprofile-targetper).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_per: Option<::Value<u32>>,
        /// Property [`UlBucketSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-serviceprofile-lorawanserviceprofile.html#cfn-iotwireless-serviceprofile-lorawanserviceprofile-ulbucketsize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ul_bucket_size: Option<::Value<u32>>,
        /// Property [`UlRate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-serviceprofile-lorawanserviceprofile.html#cfn-iotwireless-serviceprofile-lorawanserviceprofile-ulrate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ul_rate: Option<::Value<u32>>,
        /// Property [`UlRatePolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-serviceprofile-lorawanserviceprofile.html#cfn-iotwireless-serviceprofile-lorawanserviceprofile-ulratepolicy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ul_rate_policy: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LoRaWANServiceProfile {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref add_gw_metadata) = self.add_gw_metadata {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AddGwMetadata", add_gw_metadata)?;
            }
            if let Some(ref channel_mask) = self.channel_mask {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChannelMask", channel_mask)?;
            }
            if let Some(ref dev_status_req_freq) = self.dev_status_req_freq {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DevStatusReqFreq", dev_status_req_freq)?;
            }
            if let Some(ref dl_bucket_size) = self.dl_bucket_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DlBucketSize", dl_bucket_size)?;
            }
            if let Some(ref dl_rate) = self.dl_rate {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DlRate", dl_rate)?;
            }
            if let Some(ref dl_rate_policy) = self.dl_rate_policy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DlRatePolicy", dl_rate_policy)?;
            }
            if let Some(ref dr_max) = self.dr_max {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DrMax", dr_max)?;
            }
            if let Some(ref dr_min) = self.dr_min {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DrMin", dr_min)?;
            }
            if let Some(ref hr_allowed) = self.hr_allowed {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HrAllowed", hr_allowed)?;
            }
            if let Some(ref min_gw_diversity) = self.min_gw_diversity {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinGwDiversity", min_gw_diversity)?;
            }
            if let Some(ref nwk_geo_loc) = self.nwk_geo_loc {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NwkGeoLoc", nwk_geo_loc)?;
            }
            if let Some(ref pr_allowed) = self.pr_allowed {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrAllowed", pr_allowed)?;
            }
            if let Some(ref ra_allowed) = self.ra_allowed {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RaAllowed", ra_allowed)?;
            }
            if let Some(ref report_dev_status_battery) = self.report_dev_status_battery {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReportDevStatusBattery", report_dev_status_battery)?;
            }
            if let Some(ref report_dev_status_margin) = self.report_dev_status_margin {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReportDevStatusMargin", report_dev_status_margin)?;
            }
            if let Some(ref target_per) = self.target_per {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetPer", target_per)?;
            }
            if let Some(ref ul_bucket_size) = self.ul_bucket_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UlBucketSize", ul_bucket_size)?;
            }
            if let Some(ref ul_rate) = self.ul_rate {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UlRate", ul_rate)?;
            }
            if let Some(ref ul_rate_policy) = self.ul_rate_policy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UlRatePolicy", ul_rate_policy)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LoRaWANServiceProfile {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LoRaWANServiceProfile, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LoRaWANServiceProfile;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LoRaWANServiceProfile")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut add_gw_metadata: Option<::Value<bool>> = None;
                    let mut channel_mask: Option<::Value<String>> = None;
                    let mut dev_status_req_freq: Option<::Value<u32>> = None;
                    let mut dl_bucket_size: Option<::Value<u32>> = None;
                    let mut dl_rate: Option<::Value<u32>> = None;
                    let mut dl_rate_policy: Option<::Value<String>> = None;
                    let mut dr_max: Option<::Value<u32>> = None;
                    let mut dr_min: Option<::Value<u32>> = None;
                    let mut hr_allowed: Option<::Value<bool>> = None;
                    let mut min_gw_diversity: Option<::Value<u32>> = None;
                    let mut nwk_geo_loc: Option<::Value<bool>> = None;
                    let mut pr_allowed: Option<::Value<bool>> = None;
                    let mut ra_allowed: Option<::Value<bool>> = None;
                    let mut report_dev_status_battery: Option<::Value<bool>> = None;
                    let mut report_dev_status_margin: Option<::Value<bool>> = None;
                    let mut target_per: Option<::Value<u32>> = None;
                    let mut ul_bucket_size: Option<::Value<u32>> = None;
                    let mut ul_rate: Option<::Value<u32>> = None;
                    let mut ul_rate_policy: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AddGwMetadata" => {
                                add_gw_metadata = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ChannelMask" => {
                                channel_mask = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DevStatusReqFreq" => {
                                dev_status_req_freq = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DlBucketSize" => {
                                dl_bucket_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DlRate" => {
                                dl_rate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DlRatePolicy" => {
                                dl_rate_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DrMax" => {
                                dr_max = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DrMin" => {
                                dr_min = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HrAllowed" => {
                                hr_allowed = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinGwDiversity" => {
                                min_gw_diversity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NwkGeoLoc" => {
                                nwk_geo_loc = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PrAllowed" => {
                                pr_allowed = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RaAllowed" => {
                                ra_allowed = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReportDevStatusBattery" => {
                                report_dev_status_battery = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReportDevStatusMargin" => {
                                report_dev_status_margin = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetPer" => {
                                target_per = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UlBucketSize" => {
                                ul_bucket_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UlRate" => {
                                ul_rate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UlRatePolicy" => {
                                ul_rate_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LoRaWANServiceProfile {
                        add_gw_metadata: add_gw_metadata,
                        channel_mask: channel_mask,
                        dev_status_req_freq: dev_status_req_freq,
                        dl_bucket_size: dl_bucket_size,
                        dl_rate: dl_rate,
                        dl_rate_policy: dl_rate_policy,
                        dr_max: dr_max,
                        dr_min: dr_min,
                        hr_allowed: hr_allowed,
                        min_gw_diversity: min_gw_diversity,
                        nwk_geo_loc: nwk_geo_loc,
                        pr_allowed: pr_allowed,
                        ra_allowed: ra_allowed,
                        report_dev_status_battery: report_dev_status_battery,
                        report_dev_status_margin: report_dev_status_margin,
                        target_per: target_per,
                        ul_bucket_size: ul_bucket_size,
                        ul_rate: ul_rate,
                        ul_rate_policy: ul_rate_policy,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod task_definition {
    //! Property types for the `TaskDefinition` resource.

    /// The [`AWS::IoTWireless::TaskDefinition.LoRaWANGatewayVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-taskdefinition-lorawangatewayversion.html) property type.
    #[derive(Debug, Default)]
    pub struct LoRaWANGatewayVersion {
        /// Property [`Model`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-taskdefinition-lorawangatewayversion.html#cfn-iotwireless-taskdefinition-lorawangatewayversion-model).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub model: Option<::Value<String>>,
        /// Property [`PackageVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-taskdefinition-lorawangatewayversion.html#cfn-iotwireless-taskdefinition-lorawangatewayversion-packageversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub package_version: Option<::Value<String>>,
        /// Property [`Station`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-taskdefinition-lorawangatewayversion.html#cfn-iotwireless-taskdefinition-lorawangatewayversion-station).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub station: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LoRaWANGatewayVersion {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref model) = self.model {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Model", model)?;
            }
            if let Some(ref package_version) = self.package_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PackageVersion", package_version)?;
            }
            if let Some(ref station) = self.station {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Station", station)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LoRaWANGatewayVersion {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LoRaWANGatewayVersion, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LoRaWANGatewayVersion;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LoRaWANGatewayVersion")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut model: Option<::Value<String>> = None;
                    let mut package_version: Option<::Value<String>> = None;
                    let mut station: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Model" => {
                                model = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PackageVersion" => {
                                package_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Station" => {
                                station = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LoRaWANGatewayVersion {
                        model: model,
                        package_version: package_version,
                        station: station,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTWireless::TaskDefinition.LoRaWANUpdateGatewayTaskCreate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-taskdefinition-lorawanupdategatewaytaskcreate.html) property type.
    #[derive(Debug, Default)]
    pub struct LoRaWANUpdateGatewayTaskCreate {
        /// Property [`CurrentVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-taskdefinition-lorawanupdategatewaytaskcreate.html#cfn-iotwireless-taskdefinition-lorawanupdategatewaytaskcreate-currentversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub current_version: Option<::Value<LoRaWANGatewayVersion>>,
        /// Property [`SigKeyCrc`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-taskdefinition-lorawanupdategatewaytaskcreate.html#cfn-iotwireless-taskdefinition-lorawanupdategatewaytaskcreate-sigkeycrc).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sig_key_crc: Option<::Value<u32>>,
        /// Property [`UpdateSignature`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-taskdefinition-lorawanupdategatewaytaskcreate.html#cfn-iotwireless-taskdefinition-lorawanupdategatewaytaskcreate-updatesignature).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub update_signature: Option<::Value<String>>,
        /// Property [`UpdateVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-taskdefinition-lorawanupdategatewaytaskcreate.html#cfn-iotwireless-taskdefinition-lorawanupdategatewaytaskcreate-updateversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub update_version: Option<::Value<LoRaWANGatewayVersion>>,
    }

    impl ::codec::SerializeValue for LoRaWANUpdateGatewayTaskCreate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref current_version) = self.current_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CurrentVersion", current_version)?;
            }
            if let Some(ref sig_key_crc) = self.sig_key_crc {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SigKeyCrc", sig_key_crc)?;
            }
            if let Some(ref update_signature) = self.update_signature {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UpdateSignature", update_signature)?;
            }
            if let Some(ref update_version) = self.update_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UpdateVersion", update_version)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LoRaWANUpdateGatewayTaskCreate {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LoRaWANUpdateGatewayTaskCreate, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LoRaWANUpdateGatewayTaskCreate;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LoRaWANUpdateGatewayTaskCreate")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut current_version: Option<::Value<LoRaWANGatewayVersion>> = None;
                    let mut sig_key_crc: Option<::Value<u32>> = None;
                    let mut update_signature: Option<::Value<String>> = None;
                    let mut update_version: Option<::Value<LoRaWANGatewayVersion>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CurrentVersion" => {
                                current_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SigKeyCrc" => {
                                sig_key_crc = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UpdateSignature" => {
                                update_signature = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UpdateVersion" => {
                                update_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LoRaWANUpdateGatewayTaskCreate {
                        current_version: current_version,
                        sig_key_crc: sig_key_crc,
                        update_signature: update_signature,
                        update_version: update_version,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTWireless::TaskDefinition.LoRaWANUpdateGatewayTaskEntry`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-taskdefinition-lorawanupdategatewaytaskentry.html) property type.
    #[derive(Debug, Default)]
    pub struct LoRaWANUpdateGatewayTaskEntry {
        /// Property [`CurrentVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-taskdefinition-lorawanupdategatewaytaskentry.html#cfn-iotwireless-taskdefinition-lorawanupdategatewaytaskentry-currentversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub current_version: Option<::Value<LoRaWANGatewayVersion>>,
        /// Property [`UpdateVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-taskdefinition-lorawanupdategatewaytaskentry.html#cfn-iotwireless-taskdefinition-lorawanupdategatewaytaskentry-updateversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub update_version: Option<::Value<LoRaWANGatewayVersion>>,
    }

    impl ::codec::SerializeValue for LoRaWANUpdateGatewayTaskEntry {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref current_version) = self.current_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CurrentVersion", current_version)?;
            }
            if let Some(ref update_version) = self.update_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UpdateVersion", update_version)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LoRaWANUpdateGatewayTaskEntry {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LoRaWANUpdateGatewayTaskEntry, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LoRaWANUpdateGatewayTaskEntry;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LoRaWANUpdateGatewayTaskEntry")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut current_version: Option<::Value<LoRaWANGatewayVersion>> = None;
                    let mut update_version: Option<::Value<LoRaWANGatewayVersion>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CurrentVersion" => {
                                current_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UpdateVersion" => {
                                update_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LoRaWANUpdateGatewayTaskEntry {
                        current_version: current_version,
                        update_version: update_version,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTWireless::TaskDefinition.UpdateWirelessGatewayTaskCreate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-taskdefinition-updatewirelessgatewaytaskcreate.html) property type.
    #[derive(Debug, Default)]
    pub struct UpdateWirelessGatewayTaskCreate {
        /// Property [`LoRaWAN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-taskdefinition-updatewirelessgatewaytaskcreate.html#cfn-iotwireless-taskdefinition-updatewirelessgatewaytaskcreate-lorawan).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lo_ra_wan: Option<::Value<LoRaWANUpdateGatewayTaskCreate>>,
        /// Property [`UpdateDataRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-taskdefinition-updatewirelessgatewaytaskcreate.html#cfn-iotwireless-taskdefinition-updatewirelessgatewaytaskcreate-updatedatarole).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub update_data_role: Option<::Value<String>>,
        /// Property [`UpdateDataSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-taskdefinition-updatewirelessgatewaytaskcreate.html#cfn-iotwireless-taskdefinition-updatewirelessgatewaytaskcreate-updatedatasource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub update_data_source: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for UpdateWirelessGatewayTaskCreate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref lo_ra_wan) = self.lo_ra_wan {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoRaWAN", lo_ra_wan)?;
            }
            if let Some(ref update_data_role) = self.update_data_role {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UpdateDataRole", update_data_role)?;
            }
            if let Some(ref update_data_source) = self.update_data_source {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UpdateDataSource", update_data_source)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for UpdateWirelessGatewayTaskCreate {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<UpdateWirelessGatewayTaskCreate, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = UpdateWirelessGatewayTaskCreate;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type UpdateWirelessGatewayTaskCreate")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut lo_ra_wan: Option<::Value<LoRaWANUpdateGatewayTaskCreate>> = None;
                    let mut update_data_role: Option<::Value<String>> = None;
                    let mut update_data_source: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LoRaWAN" => {
                                lo_ra_wan = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UpdateDataRole" => {
                                update_data_role = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UpdateDataSource" => {
                                update_data_source = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(UpdateWirelessGatewayTaskCreate {
                        lo_ra_wan: lo_ra_wan,
                        update_data_role: update_data_role,
                        update_data_source: update_data_source,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod wireless_device {
    //! Property types for the `WirelessDevice` resource.

    /// The [`AWS::IoTWireless::WirelessDevice.AbpV10x`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessdevice-abpv10x.html) property type.
    #[derive(Debug, Default)]
    pub struct AbpV10x {
        /// Property [`DevAddr`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessdevice-abpv10x.html#cfn-iotwireless-wirelessdevice-abpv10x-devaddr).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dev_addr: ::Value<String>,
        /// Property [`SessionKeys`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessdevice-abpv10x.html#cfn-iotwireless-wirelessdevice-abpv10x-sessionkeys).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub session_keys: ::Value<SessionKeysAbpV10x>,
    }

    impl ::codec::SerializeValue for AbpV10x {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DevAddr", &self.dev_addr)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SessionKeys", &self.session_keys)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AbpV10x {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AbpV10x, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AbpV10x;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AbpV10x")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dev_addr: Option<::Value<String>> = None;
                    let mut session_keys: Option<::Value<SessionKeysAbpV10x>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DevAddr" => {
                                dev_addr = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SessionKeys" => {
                                session_keys = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AbpV10x {
                        dev_addr: dev_addr.ok_or(::serde::de::Error::missing_field("DevAddr"))?,
                        session_keys: session_keys.ok_or(::serde::de::Error::missing_field("SessionKeys"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTWireless::WirelessDevice.AbpV11`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessdevice-abpv11.html) property type.
    #[derive(Debug, Default)]
    pub struct AbpV11 {
        /// Property [`DevAddr`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessdevice-abpv11.html#cfn-iotwireless-wirelessdevice-abpv11-devaddr).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dev_addr: ::Value<String>,
        /// Property [`SessionKeys`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessdevice-abpv11.html#cfn-iotwireless-wirelessdevice-abpv11-sessionkeys).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub session_keys: ::Value<SessionKeysAbpV11>,
    }

    impl ::codec::SerializeValue for AbpV11 {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DevAddr", &self.dev_addr)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SessionKeys", &self.session_keys)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AbpV11 {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AbpV11, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AbpV11;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AbpV11")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dev_addr: Option<::Value<String>> = None;
                    let mut session_keys: Option<::Value<SessionKeysAbpV11>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DevAddr" => {
                                dev_addr = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SessionKeys" => {
                                session_keys = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AbpV11 {
                        dev_addr: dev_addr.ok_or(::serde::de::Error::missing_field("DevAddr"))?,
                        session_keys: session_keys.ok_or(::serde::de::Error::missing_field("SessionKeys"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTWireless::WirelessDevice.LoRaWANDevice`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessdevice-lorawandevice.html) property type.
    #[derive(Debug, Default)]
    pub struct LoRaWANDevice {
        /// Property [`AbpV10x`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessdevice-lorawandevice.html#cfn-iotwireless-wirelessdevice-lorawandevice-abpv10x).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub abp_v10x: Option<::Value<AbpV10x>>,
        /// Property [`AbpV11`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessdevice-lorawandevice.html#cfn-iotwireless-wirelessdevice-lorawandevice-abpv11).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub abp_v11: Option<::Value<AbpV11>>,
        /// Property [`DevEui`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessdevice-lorawandevice.html#cfn-iotwireless-wirelessdevice-lorawandevice-deveui).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dev_eui: Option<::Value<String>>,
        /// Property [`DeviceProfileId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessdevice-lorawandevice.html#cfn-iotwireless-wirelessdevice-lorawandevice-deviceprofileid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub device_profile_id: Option<::Value<String>>,
        /// Property [`OtaaV10x`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessdevice-lorawandevice.html#cfn-iotwireless-wirelessdevice-lorawandevice-otaav10x).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub otaa_v10x: Option<::Value<OtaaV10x>>,
        /// Property [`OtaaV11`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessdevice-lorawandevice.html#cfn-iotwireless-wirelessdevice-lorawandevice-otaav11).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub otaa_v11: Option<::Value<OtaaV11>>,
        /// Property [`ServiceProfileId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessdevice-lorawandevice.html#cfn-iotwireless-wirelessdevice-lorawandevice-serviceprofileid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub service_profile_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LoRaWANDevice {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref abp_v10x) = self.abp_v10x {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AbpV10x", abp_v10x)?;
            }
            if let Some(ref abp_v11) = self.abp_v11 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AbpV11", abp_v11)?;
            }
            if let Some(ref dev_eui) = self.dev_eui {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DevEui", dev_eui)?;
            }
            if let Some(ref device_profile_id) = self.device_profile_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceProfileId", device_profile_id)?;
            }
            if let Some(ref otaa_v10x) = self.otaa_v10x {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OtaaV10x", otaa_v10x)?;
            }
            if let Some(ref otaa_v11) = self.otaa_v11 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OtaaV11", otaa_v11)?;
            }
            if let Some(ref service_profile_id) = self.service_profile_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceProfileId", service_profile_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LoRaWANDevice {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LoRaWANDevice, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LoRaWANDevice;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LoRaWANDevice")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut abp_v10x: Option<::Value<AbpV10x>> = None;
                    let mut abp_v11: Option<::Value<AbpV11>> = None;
                    let mut dev_eui: Option<::Value<String>> = None;
                    let mut device_profile_id: Option<::Value<String>> = None;
                    let mut otaa_v10x: Option<::Value<OtaaV10x>> = None;
                    let mut otaa_v11: Option<::Value<OtaaV11>> = None;
                    let mut service_profile_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AbpV10x" => {
                                abp_v10x = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AbpV11" => {
                                abp_v11 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DevEui" => {
                                dev_eui = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeviceProfileId" => {
                                device_profile_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OtaaV10x" => {
                                otaa_v10x = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OtaaV11" => {
                                otaa_v11 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServiceProfileId" => {
                                service_profile_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LoRaWANDevice {
                        abp_v10x: abp_v10x,
                        abp_v11: abp_v11,
                        dev_eui: dev_eui,
                        device_profile_id: device_profile_id,
                        otaa_v10x: otaa_v10x,
                        otaa_v11: otaa_v11,
                        service_profile_id: service_profile_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTWireless::WirelessDevice.OtaaV10x`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessdevice-otaav10x.html) property type.
    #[derive(Debug, Default)]
    pub struct OtaaV10x {
        /// Property [`AppEui`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessdevice-otaav10x.html#cfn-iotwireless-wirelessdevice-otaav10x-appeui).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub app_eui: ::Value<String>,
        /// Property [`AppKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessdevice-otaav10x.html#cfn-iotwireless-wirelessdevice-otaav10x-appkey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub app_key: ::Value<String>,
    }

    impl ::codec::SerializeValue for OtaaV10x {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AppEui", &self.app_eui)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AppKey", &self.app_key)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OtaaV10x {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OtaaV10x, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OtaaV10x;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OtaaV10x")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut app_eui: Option<::Value<String>> = None;
                    let mut app_key: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AppEui" => {
                                app_eui = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AppKey" => {
                                app_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OtaaV10x {
                        app_eui: app_eui.ok_or(::serde::de::Error::missing_field("AppEui"))?,
                        app_key: app_key.ok_or(::serde::de::Error::missing_field("AppKey"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTWireless::WirelessDevice.OtaaV11`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessdevice-otaav11.html) property type.
    #[derive(Debug, Default)]
    pub struct OtaaV11 {
        /// Property [`AppKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessdevice-otaav11.html#cfn-iotwireless-wirelessdevice-otaav11-appkey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub app_key: ::Value<String>,
        /// Property [`JoinEui`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessdevice-otaav11.html#cfn-iotwireless-wirelessdevice-otaav11-joineui).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub join_eui: ::Value<String>,
        /// Property [`NwkKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessdevice-otaav11.html#cfn-iotwireless-wirelessdevice-otaav11-nwkkey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub nwk_key: ::Value<String>,
    }

    impl ::codec::SerializeValue for OtaaV11 {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AppKey", &self.app_key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "JoinEui", &self.join_eui)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NwkKey", &self.nwk_key)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OtaaV11 {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OtaaV11, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OtaaV11;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OtaaV11")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut app_key: Option<::Value<String>> = None;
                    let mut join_eui: Option<::Value<String>> = None;
                    let mut nwk_key: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AppKey" => {
                                app_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "JoinEui" => {
                                join_eui = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NwkKey" => {
                                nwk_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OtaaV11 {
                        app_key: app_key.ok_or(::serde::de::Error::missing_field("AppKey"))?,
                        join_eui: join_eui.ok_or(::serde::de::Error::missing_field("JoinEui"))?,
                        nwk_key: nwk_key.ok_or(::serde::de::Error::missing_field("NwkKey"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTWireless::WirelessDevice.SessionKeysAbpV10x`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessdevice-sessionkeysabpv10x.html) property type.
    #[derive(Debug, Default)]
    pub struct SessionKeysAbpV10x {
        /// Property [`AppSKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessdevice-sessionkeysabpv10x.html#cfn-iotwireless-wirelessdevice-sessionkeysabpv10x-appskey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub app_s_key: ::Value<String>,
        /// Property [`NwkSKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessdevice-sessionkeysabpv10x.html#cfn-iotwireless-wirelessdevice-sessionkeysabpv10x-nwkskey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub nwk_s_key: ::Value<String>,
    }

    impl ::codec::SerializeValue for SessionKeysAbpV10x {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AppSKey", &self.app_s_key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NwkSKey", &self.nwk_s_key)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SessionKeysAbpV10x {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SessionKeysAbpV10x, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SessionKeysAbpV10x;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SessionKeysAbpV10x")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut app_s_key: Option<::Value<String>> = None;
                    let mut nwk_s_key: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AppSKey" => {
                                app_s_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NwkSKey" => {
                                nwk_s_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SessionKeysAbpV10x {
                        app_s_key: app_s_key.ok_or(::serde::de::Error::missing_field("AppSKey"))?,
                        nwk_s_key: nwk_s_key.ok_or(::serde::de::Error::missing_field("NwkSKey"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTWireless::WirelessDevice.SessionKeysAbpV11`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessdevice-sessionkeysabpv11.html) property type.
    #[derive(Debug, Default)]
    pub struct SessionKeysAbpV11 {
        /// Property [`AppSKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessdevice-sessionkeysabpv11.html#cfn-iotwireless-wirelessdevice-sessionkeysabpv11-appskey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub app_s_key: ::Value<String>,
        /// Property [`FNwkSIntKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessdevice-sessionkeysabpv11.html#cfn-iotwireless-wirelessdevice-sessionkeysabpv11-fnwksintkey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub f_nwk_s_int_key: ::Value<String>,
        /// Property [`NwkSEncKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessdevice-sessionkeysabpv11.html#cfn-iotwireless-wirelessdevice-sessionkeysabpv11-nwksenckey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub nwk_s_enc_key: ::Value<String>,
        /// Property [`SNwkSIntKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessdevice-sessionkeysabpv11.html#cfn-iotwireless-wirelessdevice-sessionkeysabpv11-snwksintkey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s_nwk_s_int_key: ::Value<String>,
    }

    impl ::codec::SerializeValue for SessionKeysAbpV11 {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AppSKey", &self.app_s_key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FNwkSIntKey", &self.f_nwk_s_int_key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NwkSEncKey", &self.nwk_s_enc_key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SNwkSIntKey", &self.s_nwk_s_int_key)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SessionKeysAbpV11 {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SessionKeysAbpV11, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SessionKeysAbpV11;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SessionKeysAbpV11")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut app_s_key: Option<::Value<String>> = None;
                    let mut f_nwk_s_int_key: Option<::Value<String>> = None;
                    let mut nwk_s_enc_key: Option<::Value<String>> = None;
                    let mut s_nwk_s_int_key: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AppSKey" => {
                                app_s_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FNwkSIntKey" => {
                                f_nwk_s_int_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NwkSEncKey" => {
                                nwk_s_enc_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SNwkSIntKey" => {
                                s_nwk_s_int_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SessionKeysAbpV11 {
                        app_s_key: app_s_key.ok_or(::serde::de::Error::missing_field("AppSKey"))?,
                        f_nwk_s_int_key: f_nwk_s_int_key.ok_or(::serde::de::Error::missing_field("FNwkSIntKey"))?,
                        nwk_s_enc_key: nwk_s_enc_key.ok_or(::serde::de::Error::missing_field("NwkSEncKey"))?,
                        s_nwk_s_int_key: s_nwk_s_int_key.ok_or(::serde::de::Error::missing_field("SNwkSIntKey"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod wireless_gateway {
    //! Property types for the `WirelessGateway` resource.

    /// The [`AWS::IoTWireless::WirelessGateway.LoRaWANGateway`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessgateway-lorawangateway.html) property type.
    #[derive(Debug, Default)]
    pub struct LoRaWANGateway {
        /// Property [`GatewayEui`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessgateway-lorawangateway.html#cfn-iotwireless-wirelessgateway-lorawangateway-gatewayeui).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub gateway_eui: ::Value<String>,
        /// Property [`RfRegion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotwireless-wirelessgateway-lorawangateway.html#cfn-iotwireless-wirelessgateway-lorawangateway-rfregion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rf_region: ::Value<String>,
    }

    impl ::codec::SerializeValue for LoRaWANGateway {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GatewayEui", &self.gateway_eui)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RfRegion", &self.rf_region)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LoRaWANGateway {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LoRaWANGateway, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LoRaWANGateway;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LoRaWANGateway")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut gateway_eui: Option<::Value<String>> = None;
                    let mut rf_region: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "GatewayEui" => {
                                gateway_eui = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RfRegion" => {
                                rf_region = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LoRaWANGateway {
                        gateway_eui: gateway_eui.ok_or(::serde::de::Error::missing_field("GatewayEui"))?,
                        rf_region: rf_region.ok_or(::serde::de::Error::missing_field("RfRegion"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
