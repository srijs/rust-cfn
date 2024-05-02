//! Types for the `IoTFleetWise` service.

/// The [`AWS::IoTFleetWise::Campaign`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-campaign.html) resource type.
#[derive(Debug, Default)]
pub struct Campaign {
    properties: CampaignProperties
}

/// Properties for the `Campaign` resource.
#[derive(Debug, Default)]
pub struct CampaignProperties {
    /// Property [`Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-campaign.html#cfn-iotfleetwise-campaign-action).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub action: ::Value<String>,
    /// Property [`CollectionScheme`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-campaign.html#cfn-iotfleetwise-campaign-collectionscheme).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub collection_scheme: ::Value<self::campaign::CollectionScheme>,
    /// Property [`Compression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-campaign.html#cfn-iotfleetwise-campaign-compression).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub compression: Option<::Value<String>>,
    /// Property [`DataDestinationConfigs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-campaign.html#cfn-iotfleetwise-campaign-datadestinationconfigs).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub data_destination_configs: Option<::ValueList<self::campaign::DataDestinationConfig>>,
    /// Property [`DataExtraDimensions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-campaign.html#cfn-iotfleetwise-campaign-dataextradimensions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub data_extra_dimensions: Option<::ValueList<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-campaign.html#cfn-iotfleetwise-campaign-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`DiagnosticsMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-campaign.html#cfn-iotfleetwise-campaign-diagnosticsmode).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub diagnostics_mode: Option<::Value<String>>,
    /// Property [`ExpiryTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-campaign.html#cfn-iotfleetwise-campaign-expirytime).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub expiry_time: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-campaign.html#cfn-iotfleetwise-campaign-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`PostTriggerCollectionDuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-campaign.html#cfn-iotfleetwise-campaign-posttriggercollectionduration).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub post_trigger_collection_duration: Option<::Value<f64>>,
    /// Property [`Priority`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-campaign.html#cfn-iotfleetwise-campaign-priority).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub priority: Option<::Value<u32>>,
    /// Property [`SignalCatalogArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-campaign.html#cfn-iotfleetwise-campaign-signalcatalogarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub signal_catalog_arn: ::Value<String>,
    /// Property [`SignalsToCollect`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-campaign.html#cfn-iotfleetwise-campaign-signalstocollect).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub signals_to_collect: Option<::ValueList<self::campaign::SignalInformation>>,
    /// Property [`SpoolingMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-campaign.html#cfn-iotfleetwise-campaign-spoolingmode).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub spooling_mode: Option<::Value<String>>,
    /// Property [`StartTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-campaign.html#cfn-iotfleetwise-campaign-starttime).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub start_time: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-campaign.html#cfn-iotfleetwise-campaign-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TargetArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-campaign.html#cfn-iotfleetwise-campaign-targetarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub target_arn: ::Value<String>,
}

impl ::serde::Serialize for CampaignProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Action", &self.action)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CollectionScheme", &self.collection_scheme)?;
        if let Some(ref compression) = self.compression {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Compression", compression)?;
        }
        if let Some(ref data_destination_configs) = self.data_destination_configs {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataDestinationConfigs", data_destination_configs)?;
        }
        if let Some(ref data_extra_dimensions) = self.data_extra_dimensions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataExtraDimensions", data_extra_dimensions)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref diagnostics_mode) = self.diagnostics_mode {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DiagnosticsMode", diagnostics_mode)?;
        }
        if let Some(ref expiry_time) = self.expiry_time {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExpiryTime", expiry_time)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref post_trigger_collection_duration) = self.post_trigger_collection_duration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PostTriggerCollectionDuration", post_trigger_collection_duration)?;
        }
        if let Some(ref priority) = self.priority {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Priority", priority)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SignalCatalogArn", &self.signal_catalog_arn)?;
        if let Some(ref signals_to_collect) = self.signals_to_collect {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SignalsToCollect", signals_to_collect)?;
        }
        if let Some(ref spooling_mode) = self.spooling_mode {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SpoolingMode", spooling_mode)?;
        }
        if let Some(ref start_time) = self.start_time {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartTime", start_time)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetArn", &self.target_arn)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CampaignProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CampaignProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CampaignProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CampaignProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut action: Option<::Value<String>> = None;
                let mut collection_scheme: Option<::Value<self::campaign::CollectionScheme>> = None;
                let mut compression: Option<::Value<String>> = None;
                let mut data_destination_configs: Option<::ValueList<self::campaign::DataDestinationConfig>> = None;
                let mut data_extra_dimensions: Option<::ValueList<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut diagnostics_mode: Option<::Value<String>> = None;
                let mut expiry_time: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut post_trigger_collection_duration: Option<::Value<f64>> = None;
                let mut priority: Option<::Value<u32>> = None;
                let mut signal_catalog_arn: Option<::Value<String>> = None;
                let mut signals_to_collect: Option<::ValueList<self::campaign::SignalInformation>> = None;
                let mut spooling_mode: Option<::Value<String>> = None;
                let mut start_time: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut target_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Action" => {
                            action = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CollectionScheme" => {
                            collection_scheme = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Compression" => {
                            compression = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DataDestinationConfigs" => {
                            data_destination_configs = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DataExtraDimensions" => {
                            data_extra_dimensions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DiagnosticsMode" => {
                            diagnostics_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ExpiryTime" => {
                            expiry_time = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PostTriggerCollectionDuration" => {
                            post_trigger_collection_duration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Priority" => {
                            priority = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SignalCatalogArn" => {
                            signal_catalog_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SignalsToCollect" => {
                            signals_to_collect = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SpoolingMode" => {
                            spooling_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StartTime" => {
                            start_time = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetArn" => {
                            target_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CampaignProperties {
                    action: action.ok_or(::serde::de::Error::missing_field("Action"))?,
                    collection_scheme: collection_scheme.ok_or(::serde::de::Error::missing_field("CollectionScheme"))?,
                    compression: compression,
                    data_destination_configs: data_destination_configs,
                    data_extra_dimensions: data_extra_dimensions,
                    description: description,
                    diagnostics_mode: diagnostics_mode,
                    expiry_time: expiry_time,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    post_trigger_collection_duration: post_trigger_collection_duration,
                    priority: priority,
                    signal_catalog_arn: signal_catalog_arn.ok_or(::serde::de::Error::missing_field("SignalCatalogArn"))?,
                    signals_to_collect: signals_to_collect,
                    spooling_mode: spooling_mode,
                    start_time: start_time,
                    tags: tags,
                    target_arn: target_arn.ok_or(::serde::de::Error::missing_field("TargetArn"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Campaign {
    type Properties = CampaignProperties;
    const TYPE: &'static str = "AWS::IoTFleetWise::Campaign";
    fn properties(&self) -> &CampaignProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CampaignProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Campaign {}

impl From<CampaignProperties> for Campaign {
    fn from(properties: CampaignProperties) -> Campaign {
        Campaign { properties }
    }
}

/// The [`AWS::IoTFleetWise::DecoderManifest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-decodermanifest.html) resource type.
#[derive(Debug, Default)]
pub struct DecoderManifest {
    properties: DecoderManifestProperties
}

/// Properties for the `DecoderManifest` resource.
#[derive(Debug, Default)]
pub struct DecoderManifestProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-decodermanifest.html#cfn-iotfleetwise-decodermanifest-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`ModelManifestArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-decodermanifest.html#cfn-iotfleetwise-decodermanifest-modelmanifestarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub model_manifest_arn: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-decodermanifest.html#cfn-iotfleetwise-decodermanifest-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`NetworkInterfaces`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-decodermanifest.html#cfn-iotfleetwise-decodermanifest-networkinterfaces).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub network_interfaces: Option<::ValueList<self::decoder_manifest::NetworkInterfacesItems>>,
    /// Property [`SignalDecoders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-decodermanifest.html#cfn-iotfleetwise-decodermanifest-signaldecoders).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub signal_decoders: Option<::ValueList<self::decoder_manifest::SignalDecodersItems>>,
    /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-decodermanifest.html#cfn-iotfleetwise-decodermanifest-status).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub status: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-decodermanifest.html#cfn-iotfleetwise-decodermanifest-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for DecoderManifestProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelManifestArn", &self.model_manifest_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref network_interfaces) = self.network_interfaces {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkInterfaces", network_interfaces)?;
        }
        if let Some(ref signal_decoders) = self.signal_decoders {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SignalDecoders", signal_decoders)?;
        }
        if let Some(ref status) = self.status {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", status)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DecoderManifestProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DecoderManifestProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DecoderManifestProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DecoderManifestProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut model_manifest_arn: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut network_interfaces: Option<::ValueList<self::decoder_manifest::NetworkInterfacesItems>> = None;
                let mut signal_decoders: Option<::ValueList<self::decoder_manifest::SignalDecodersItems>> = None;
                let mut status: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ModelManifestArn" => {
                            model_manifest_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NetworkInterfaces" => {
                            network_interfaces = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SignalDecoders" => {
                            signal_decoders = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Status" => {
                            status = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DecoderManifestProperties {
                    description: description,
                    model_manifest_arn: model_manifest_arn.ok_or(::serde::de::Error::missing_field("ModelManifestArn"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    network_interfaces: network_interfaces,
                    signal_decoders: signal_decoders,
                    status: status,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DecoderManifest {
    type Properties = DecoderManifestProperties;
    const TYPE: &'static str = "AWS::IoTFleetWise::DecoderManifest";
    fn properties(&self) -> &DecoderManifestProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DecoderManifestProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DecoderManifest {}

impl From<DecoderManifestProperties> for DecoderManifest {
    fn from(properties: DecoderManifestProperties) -> DecoderManifest {
        DecoderManifest { properties }
    }
}

/// The [`AWS::IoTFleetWise::Fleet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-fleet.html) resource type.
#[derive(Debug, Default)]
pub struct Fleet {
    properties: FleetProperties
}

/// Properties for the `Fleet` resource.
#[derive(Debug, Default)]
pub struct FleetProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-fleet.html#cfn-iotfleetwise-fleet-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-fleet.html#cfn-iotfleetwise-fleet-id).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub id: ::Value<String>,
    /// Property [`SignalCatalogArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-fleet.html#cfn-iotfleetwise-fleet-signalcatalogarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub signal_catalog_arn: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-fleet.html#cfn-iotfleetwise-fleet-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for FleetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SignalCatalogArn", &self.signal_catalog_arn)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for FleetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<FleetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = FleetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type FleetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut id: Option<::Value<String>> = None;
                let mut signal_catalog_arn: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Id" => {
                            id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SignalCatalogArn" => {
                            signal_catalog_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(FleetProperties {
                    description: description,
                    id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                    signal_catalog_arn: signal_catalog_arn.ok_or(::serde::de::Error::missing_field("SignalCatalogArn"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Fleet {
    type Properties = FleetProperties;
    const TYPE: &'static str = "AWS::IoTFleetWise::Fleet";
    fn properties(&self) -> &FleetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut FleetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Fleet {}

impl From<FleetProperties> for Fleet {
    fn from(properties: FleetProperties) -> Fleet {
        Fleet { properties }
    }
}

/// The [`AWS::IoTFleetWise::ModelManifest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-modelmanifest.html) resource type.
#[derive(Debug, Default)]
pub struct ModelManifest {
    properties: ModelManifestProperties
}

/// Properties for the `ModelManifest` resource.
#[derive(Debug, Default)]
pub struct ModelManifestProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-modelmanifest.html#cfn-iotfleetwise-modelmanifest-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-modelmanifest.html#cfn-iotfleetwise-modelmanifest-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Nodes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-modelmanifest.html#cfn-iotfleetwise-modelmanifest-nodes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub nodes: Option<::ValueList<String>>,
    /// Property [`SignalCatalogArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-modelmanifest.html#cfn-iotfleetwise-modelmanifest-signalcatalogarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub signal_catalog_arn: ::Value<String>,
    /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-modelmanifest.html#cfn-iotfleetwise-modelmanifest-status).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub status: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-modelmanifest.html#cfn-iotfleetwise-modelmanifest-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ModelManifestProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref nodes) = self.nodes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Nodes", nodes)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SignalCatalogArn", &self.signal_catalog_arn)?;
        if let Some(ref status) = self.status {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", status)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ModelManifestProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ModelManifestProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ModelManifestProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ModelManifestProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut nodes: Option<::ValueList<String>> = None;
                let mut signal_catalog_arn: Option<::Value<String>> = None;
                let mut status: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Nodes" => {
                            nodes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SignalCatalogArn" => {
                            signal_catalog_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Status" => {
                            status = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ModelManifestProperties {
                    description: description,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    nodes: nodes,
                    signal_catalog_arn: signal_catalog_arn.ok_or(::serde::de::Error::missing_field("SignalCatalogArn"))?,
                    status: status,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ModelManifest {
    type Properties = ModelManifestProperties;
    const TYPE: &'static str = "AWS::IoTFleetWise::ModelManifest";
    fn properties(&self) -> &ModelManifestProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ModelManifestProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ModelManifest {}

impl From<ModelManifestProperties> for ModelManifest {
    fn from(properties: ModelManifestProperties) -> ModelManifest {
        ModelManifest { properties }
    }
}

/// The [`AWS::IoTFleetWise::SignalCatalog`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-signalcatalog.html) resource type.
#[derive(Debug, Default)]
pub struct SignalCatalog {
    properties: SignalCatalogProperties
}

/// Properties for the `SignalCatalog` resource.
#[derive(Debug, Default)]
pub struct SignalCatalogProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-signalcatalog.html#cfn-iotfleetwise-signalcatalog-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-signalcatalog.html#cfn-iotfleetwise-signalcatalog-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`NodeCounts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-signalcatalog.html#cfn-iotfleetwise-signalcatalog-nodecounts).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub node_counts: Option<::Value<self::signal_catalog::NodeCounts>>,
    /// Property [`Nodes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-signalcatalog.html#cfn-iotfleetwise-signalcatalog-nodes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub nodes: Option<::ValueList<self::signal_catalog::Node>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-signalcatalog.html#cfn-iotfleetwise-signalcatalog-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for SignalCatalogProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref node_counts) = self.node_counts {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NodeCounts", node_counts)?;
        }
        if let Some(ref nodes) = self.nodes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Nodes", nodes)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SignalCatalogProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SignalCatalogProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SignalCatalogProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SignalCatalogProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut node_counts: Option<::Value<self::signal_catalog::NodeCounts>> = None;
                let mut nodes: Option<::ValueList<self::signal_catalog::Node>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NodeCounts" => {
                            node_counts = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Nodes" => {
                            nodes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SignalCatalogProperties {
                    description: description,
                    name: name,
                    node_counts: node_counts,
                    nodes: nodes,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for SignalCatalog {
    type Properties = SignalCatalogProperties;
    const TYPE: &'static str = "AWS::IoTFleetWise::SignalCatalog";
    fn properties(&self) -> &SignalCatalogProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SignalCatalogProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SignalCatalog {}

impl From<SignalCatalogProperties> for SignalCatalog {
    fn from(properties: SignalCatalogProperties) -> SignalCatalog {
        SignalCatalog { properties }
    }
}

/// The [`AWS::IoTFleetWise::Vehicle`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-vehicle.html) resource type.
#[derive(Debug, Default)]
pub struct Vehicle {
    properties: VehicleProperties
}

/// Properties for the `Vehicle` resource.
#[derive(Debug, Default)]
pub struct VehicleProperties {
    /// Property [`AssociationBehavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-vehicle.html#cfn-iotfleetwise-vehicle-associationbehavior).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub association_behavior: Option<::Value<String>>,
    /// Property [`Attributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-vehicle.html#cfn-iotfleetwise-vehicle-attributes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub attributes: Option<::ValueMap<String>>,
    /// Property [`DecoderManifestArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-vehicle.html#cfn-iotfleetwise-vehicle-decodermanifestarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub decoder_manifest_arn: ::Value<String>,
    /// Property [`ModelManifestArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-vehicle.html#cfn-iotfleetwise-vehicle-modelmanifestarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub model_manifest_arn: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-vehicle.html#cfn-iotfleetwise-vehicle-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-vehicle.html#cfn-iotfleetwise-vehicle-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for VehicleProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref association_behavior) = self.association_behavior {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssociationBehavior", association_behavior)?;
        }
        if let Some(ref attributes) = self.attributes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Attributes", attributes)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DecoderManifestArn", &self.decoder_manifest_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelManifestArn", &self.model_manifest_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for VehicleProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<VehicleProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = VehicleProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type VehicleProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut association_behavior: Option<::Value<String>> = None;
                let mut attributes: Option<::ValueMap<String>> = None;
                let mut decoder_manifest_arn: Option<::Value<String>> = None;
                let mut model_manifest_arn: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AssociationBehavior" => {
                            association_behavior = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Attributes" => {
                            attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DecoderManifestArn" => {
                            decoder_manifest_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ModelManifestArn" => {
                            model_manifest_arn = ::serde::de::MapAccess::next_value(&mut map)?;
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

                Ok(VehicleProperties {
                    association_behavior: association_behavior,
                    attributes: attributes,
                    decoder_manifest_arn: decoder_manifest_arn.ok_or(::serde::de::Error::missing_field("DecoderManifestArn"))?,
                    model_manifest_arn: model_manifest_arn.ok_or(::serde::de::Error::missing_field("ModelManifestArn"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Vehicle {
    type Properties = VehicleProperties;
    const TYPE: &'static str = "AWS::IoTFleetWise::Vehicle";
    fn properties(&self) -> &VehicleProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut VehicleProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Vehicle {}

impl From<VehicleProperties> for Vehicle {
    fn from(properties: VehicleProperties) -> Vehicle {
        Vehicle { properties }
    }
}

pub mod campaign {
    //! Property types for the `Campaign` resource.

    /// The [`AWS::IoTFleetWise::Campaign.CollectionScheme`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-collectionscheme.html) property type.
    #[derive(Debug, Default)]
    pub struct CollectionScheme {
        /// Property [`ConditionBasedCollectionScheme`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-collectionscheme.html#cfn-iotfleetwise-campaign-collectionscheme-conditionbasedcollectionscheme).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub condition_based_collection_scheme: Option<::Value<ConditionBasedCollectionScheme>>,
        /// Property [`TimeBasedCollectionScheme`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-collectionscheme.html#cfn-iotfleetwise-campaign-collectionscheme-timebasedcollectionscheme).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub time_based_collection_scheme: Option<::Value<TimeBasedCollectionScheme>>,
    }

    impl ::codec::SerializeValue for CollectionScheme {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref condition_based_collection_scheme) = self.condition_based_collection_scheme {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConditionBasedCollectionScheme", condition_based_collection_scheme)?;
            }
            if let Some(ref time_based_collection_scheme) = self.time_based_collection_scheme {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeBasedCollectionScheme", time_based_collection_scheme)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CollectionScheme {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CollectionScheme, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CollectionScheme;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CollectionScheme")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut condition_based_collection_scheme: Option<::Value<ConditionBasedCollectionScheme>> = None;
                    let mut time_based_collection_scheme: Option<::Value<TimeBasedCollectionScheme>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConditionBasedCollectionScheme" => {
                                condition_based_collection_scheme = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimeBasedCollectionScheme" => {
                                time_based_collection_scheme = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CollectionScheme {
                        condition_based_collection_scheme: condition_based_collection_scheme,
                        time_based_collection_scheme: time_based_collection_scheme,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTFleetWise::Campaign.ConditionBasedCollectionScheme`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-conditionbasedcollectionscheme.html) property type.
    #[derive(Debug, Default)]
    pub struct ConditionBasedCollectionScheme {
        /// Property [`ConditionLanguageVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-conditionbasedcollectionscheme.html#cfn-iotfleetwise-campaign-conditionbasedcollectionscheme-conditionlanguageversion).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub condition_language_version: Option<::Value<u32>>,
        /// Property [`Expression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-conditionbasedcollectionscheme.html#cfn-iotfleetwise-campaign-conditionbasedcollectionscheme-expression).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub expression: ::Value<String>,
        /// Property [`MinimumTriggerIntervalMs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-conditionbasedcollectionscheme.html#cfn-iotfleetwise-campaign-conditionbasedcollectionscheme-minimumtriggerintervalms).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub minimum_trigger_interval_ms: Option<::Value<f64>>,
        /// Property [`TriggerMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-conditionbasedcollectionscheme.html#cfn-iotfleetwise-campaign-conditionbasedcollectionscheme-triggermode).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub trigger_mode: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ConditionBasedCollectionScheme {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref condition_language_version) = self.condition_language_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConditionLanguageVersion", condition_language_version)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Expression", &self.expression)?;
            if let Some(ref minimum_trigger_interval_ms) = self.minimum_trigger_interval_ms {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinimumTriggerIntervalMs", minimum_trigger_interval_ms)?;
            }
            if let Some(ref trigger_mode) = self.trigger_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TriggerMode", trigger_mode)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConditionBasedCollectionScheme {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConditionBasedCollectionScheme, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConditionBasedCollectionScheme;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConditionBasedCollectionScheme")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut condition_language_version: Option<::Value<u32>> = None;
                    let mut expression: Option<::Value<String>> = None;
                    let mut minimum_trigger_interval_ms: Option<::Value<f64>> = None;
                    let mut trigger_mode: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConditionLanguageVersion" => {
                                condition_language_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Expression" => {
                                expression = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinimumTriggerIntervalMs" => {
                                minimum_trigger_interval_ms = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TriggerMode" => {
                                trigger_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConditionBasedCollectionScheme {
                        condition_language_version: condition_language_version,
                        expression: expression.ok_or(::serde::de::Error::missing_field("Expression"))?,
                        minimum_trigger_interval_ms: minimum_trigger_interval_ms,
                        trigger_mode: trigger_mode,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTFleetWise::Campaign.DataDestinationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-datadestinationconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct DataDestinationConfig {
        /// Property [`S3Config`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-datadestinationconfig.html#cfn-iotfleetwise-campaign-datadestinationconfig-s3config).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_config: Option<::Value<S3Config>>,
        /// Property [`TimestreamConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-datadestinationconfig.html#cfn-iotfleetwise-campaign-datadestinationconfig-timestreamconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timestream_config: Option<::Value<TimestreamConfig>>,
    }

    impl ::codec::SerializeValue for DataDestinationConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref s3_config) = self.s3_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Config", s3_config)?;
            }
            if let Some(ref timestream_config) = self.timestream_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimestreamConfig", timestream_config)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataDestinationConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataDestinationConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataDestinationConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataDestinationConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut s3_config: Option<::Value<S3Config>> = None;
                    let mut timestream_config: Option<::Value<TimestreamConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3Config" => {
                                s3_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimestreamConfig" => {
                                timestream_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataDestinationConfig {
                        s3_config: s3_config,
                        timestream_config: timestream_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTFleetWise::Campaign.S3Config`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-s3config.html) property type.
    #[derive(Debug, Default)]
    pub struct S3Config {
        /// Property [`BucketArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-s3config.html#cfn-iotfleetwise-campaign-s3config-bucketarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_arn: ::Value<String>,
        /// Property [`DataFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-s3config.html#cfn-iotfleetwise-campaign-s3config-dataformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_format: Option<::Value<String>>,
        /// Property [`Prefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-s3config.html#cfn-iotfleetwise-campaign-s3config-prefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub prefix: Option<::Value<String>>,
        /// Property [`StorageCompressionFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-s3config.html#cfn-iotfleetwise-campaign-s3config-storagecompressionformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub storage_compression_format: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for S3Config {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketArn", &self.bucket_arn)?;
            if let Some(ref data_format) = self.data_format {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataFormat", data_format)?;
            }
            if let Some(ref prefix) = self.prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Prefix", prefix)?;
            }
            if let Some(ref storage_compression_format) = self.storage_compression_format {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StorageCompressionFormat", storage_compression_format)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3Config {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3Config, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3Config;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3Config")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket_arn: Option<::Value<String>> = None;
                    let mut data_format: Option<::Value<String>> = None;
                    let mut prefix: Option<::Value<String>> = None;
                    let mut storage_compression_format: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketArn" => {
                                bucket_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataFormat" => {
                                data_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Prefix" => {
                                prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StorageCompressionFormat" => {
                                storage_compression_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Config {
                        bucket_arn: bucket_arn.ok_or(::serde::de::Error::missing_field("BucketArn"))?,
                        data_format: data_format,
                        prefix: prefix,
                        storage_compression_format: storage_compression_format,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTFleetWise::Campaign.SignalInformation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-signalinformation.html) property type.
    #[derive(Debug, Default)]
    pub struct SignalInformation {
        /// Property [`MaxSampleCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-signalinformation.html#cfn-iotfleetwise-campaign-signalinformation-maxsamplecount).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_sample_count: Option<::Value<f64>>,
        /// Property [`MinimumSamplingIntervalMs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-signalinformation.html#cfn-iotfleetwise-campaign-signalinformation-minimumsamplingintervalms).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub minimum_sampling_interval_ms: Option<::Value<f64>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-signalinformation.html#cfn-iotfleetwise-campaign-signalinformation-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for SignalInformation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref max_sample_count) = self.max_sample_count {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxSampleCount", max_sample_count)?;
            }
            if let Some(ref minimum_sampling_interval_ms) = self.minimum_sampling_interval_ms {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinimumSamplingIntervalMs", minimum_sampling_interval_ms)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SignalInformation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SignalInformation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SignalInformation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SignalInformation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut max_sample_count: Option<::Value<f64>> = None;
                    let mut minimum_sampling_interval_ms: Option<::Value<f64>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxSampleCount" => {
                                max_sample_count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinimumSamplingIntervalMs" => {
                                minimum_sampling_interval_ms = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SignalInformation {
                        max_sample_count: max_sample_count,
                        minimum_sampling_interval_ms: minimum_sampling_interval_ms,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTFleetWise::Campaign.TimeBasedCollectionScheme`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-timebasedcollectionscheme.html) property type.
    #[derive(Debug, Default)]
    pub struct TimeBasedCollectionScheme {
        /// Property [`PeriodMs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-timebasedcollectionscheme.html#cfn-iotfleetwise-campaign-timebasedcollectionscheme-periodms).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub period_ms: ::Value<f64>,
    }

    impl ::codec::SerializeValue for TimeBasedCollectionScheme {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PeriodMs", &self.period_ms)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TimeBasedCollectionScheme {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TimeBasedCollectionScheme, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TimeBasedCollectionScheme;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TimeBasedCollectionScheme")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut period_ms: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PeriodMs" => {
                                period_ms = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TimeBasedCollectionScheme {
                        period_ms: period_ms.ok_or(::serde::de::Error::missing_field("PeriodMs"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTFleetWise::Campaign.TimestreamConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-timestreamconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct TimestreamConfig {
        /// Property [`ExecutionRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-timestreamconfig.html#cfn-iotfleetwise-campaign-timestreamconfig-executionrolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub execution_role_arn: ::Value<String>,
        /// Property [`TimestreamTableArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-timestreamconfig.html#cfn-iotfleetwise-campaign-timestreamconfig-timestreamtablearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timestream_table_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for TimestreamConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExecutionRoleArn", &self.execution_role_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimestreamTableArn", &self.timestream_table_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TimestreamConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TimestreamConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TimestreamConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TimestreamConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut execution_role_arn: Option<::Value<String>> = None;
                    let mut timestream_table_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ExecutionRoleArn" => {
                                execution_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimestreamTableArn" => {
                                timestream_table_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TimestreamConfig {
                        execution_role_arn: execution_role_arn.ok_or(::serde::de::Error::missing_field("ExecutionRoleArn"))?,
                        timestream_table_arn: timestream_table_arn.ok_or(::serde::de::Error::missing_field("TimestreamTableArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod decoder_manifest {
    //! Property types for the `DecoderManifest` resource.

    /// The [`AWS::IoTFleetWise::DecoderManifest.CanInterface`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-caninterface.html) property type.
    #[derive(Debug, Default)]
    pub struct CanInterface {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-caninterface.html#cfn-iotfleetwise-decodermanifest-caninterface-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`ProtocolName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-caninterface.html#cfn-iotfleetwise-decodermanifest-caninterface-protocolname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub protocol_name: Option<::Value<String>>,
        /// Property [`ProtocolVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-caninterface.html#cfn-iotfleetwise-decodermanifest-caninterface-protocolversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub protocol_version: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CanInterface {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            if let Some(ref protocol_name) = self.protocol_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProtocolName", protocol_name)?;
            }
            if let Some(ref protocol_version) = self.protocol_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProtocolVersion", protocol_version)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CanInterface {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CanInterface, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CanInterface;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CanInterface")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut protocol_name: Option<::Value<String>> = None;
                    let mut protocol_version: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProtocolName" => {
                                protocol_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProtocolVersion" => {
                                protocol_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CanInterface {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        protocol_name: protocol_name,
                        protocol_version: protocol_version,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTFleetWise::DecoderManifest.CanSignal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-cansignal.html) property type.
    #[derive(Debug, Default)]
    pub struct CanSignal {
        /// Property [`Factor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-cansignal.html#cfn-iotfleetwise-decodermanifest-cansignal-factor).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub factor: ::Value<String>,
        /// Property [`IsBigEndian`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-cansignal.html#cfn-iotfleetwise-decodermanifest-cansignal-isbigendian).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub is_big_endian: ::Value<String>,
        /// Property [`IsSigned`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-cansignal.html#cfn-iotfleetwise-decodermanifest-cansignal-issigned).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub is_signed: ::Value<String>,
        /// Property [`Length`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-cansignal.html#cfn-iotfleetwise-decodermanifest-cansignal-length).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub length: ::Value<String>,
        /// Property [`MessageId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-cansignal.html#cfn-iotfleetwise-decodermanifest-cansignal-messageid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub message_id: ::Value<String>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-cansignal.html#cfn-iotfleetwise-decodermanifest-cansignal-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Offset`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-cansignal.html#cfn-iotfleetwise-decodermanifest-cansignal-offset).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub offset: ::Value<String>,
        /// Property [`StartBit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-cansignal.html#cfn-iotfleetwise-decodermanifest-cansignal-startbit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub start_bit: ::Value<String>,
    }

    impl ::codec::SerializeValue for CanSignal {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Factor", &self.factor)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsBigEndian", &self.is_big_endian)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsSigned", &self.is_signed)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Length", &self.length)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MessageId", &self.message_id)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Offset", &self.offset)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartBit", &self.start_bit)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CanSignal {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CanSignal, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CanSignal;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CanSignal")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut factor: Option<::Value<String>> = None;
                    let mut is_big_endian: Option<::Value<String>> = None;
                    let mut is_signed: Option<::Value<String>> = None;
                    let mut length: Option<::Value<String>> = None;
                    let mut message_id: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut offset: Option<::Value<String>> = None;
                    let mut start_bit: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Factor" => {
                                factor = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IsBigEndian" => {
                                is_big_endian = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IsSigned" => {
                                is_signed = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Length" => {
                                length = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MessageId" => {
                                message_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Offset" => {
                                offset = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StartBit" => {
                                start_bit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CanSignal {
                        factor: factor.ok_or(::serde::de::Error::missing_field("Factor"))?,
                        is_big_endian: is_big_endian.ok_or(::serde::de::Error::missing_field("IsBigEndian"))?,
                        is_signed: is_signed.ok_or(::serde::de::Error::missing_field("IsSigned"))?,
                        length: length.ok_or(::serde::de::Error::missing_field("Length"))?,
                        message_id: message_id.ok_or(::serde::de::Error::missing_field("MessageId"))?,
                        name: name,
                        offset: offset.ok_or(::serde::de::Error::missing_field("Offset"))?,
                        start_bit: start_bit.ok_or(::serde::de::Error::missing_field("StartBit"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTFleetWise::DecoderManifest.NetworkInterfacesItems`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-networkinterfacesitems.html) property type.
    #[derive(Debug, Default)]
    pub struct NetworkInterfacesItems {
        /// Property [`CanInterface`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-networkinterfacesitems.html#cfn-iotfleetwise-decodermanifest-networkinterfacesitems-caninterface).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub can_interface: Option<::Value<CanInterface>>,
        /// Property [`InterfaceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-networkinterfacesitems.html#cfn-iotfleetwise-decodermanifest-networkinterfacesitems-interfaceid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub interface_id: ::Value<String>,
        /// Property [`ObdInterface`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-networkinterfacesitems.html#cfn-iotfleetwise-decodermanifest-networkinterfacesitems-obdinterface).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub obd_interface: Option<::Value<ObdInterface>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-networkinterfacesitems.html#cfn-iotfleetwise-decodermanifest-networkinterfacesitems-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for NetworkInterfacesItems {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref can_interface) = self.can_interface {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CanInterface", can_interface)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InterfaceId", &self.interface_id)?;
            if let Some(ref obd_interface) = self.obd_interface {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ObdInterface", obd_interface)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NetworkInterfacesItems {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NetworkInterfacesItems, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NetworkInterfacesItems;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NetworkInterfacesItems")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut can_interface: Option<::Value<CanInterface>> = None;
                    let mut interface_id: Option<::Value<String>> = None;
                    let mut obd_interface: Option<::Value<ObdInterface>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CanInterface" => {
                                can_interface = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InterfaceId" => {
                                interface_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ObdInterface" => {
                                obd_interface = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NetworkInterfacesItems {
                        can_interface: can_interface,
                        interface_id: interface_id.ok_or(::serde::de::Error::missing_field("InterfaceId"))?,
                        obd_interface: obd_interface,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTFleetWise::DecoderManifest.ObdInterface`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-obdinterface.html) property type.
    #[derive(Debug, Default)]
    pub struct ObdInterface {
        /// Property [`DtcRequestIntervalSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-obdinterface.html#cfn-iotfleetwise-decodermanifest-obdinterface-dtcrequestintervalseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dtc_request_interval_seconds: Option<::Value<String>>,
        /// Property [`HasTransmissionEcu`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-obdinterface.html#cfn-iotfleetwise-decodermanifest-obdinterface-hastransmissionecu).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub has_transmission_ecu: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-obdinterface.html#cfn-iotfleetwise-decodermanifest-obdinterface-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`ObdStandard`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-obdinterface.html#cfn-iotfleetwise-decodermanifest-obdinterface-obdstandard).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub obd_standard: Option<::Value<String>>,
        /// Property [`PidRequestIntervalSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-obdinterface.html#cfn-iotfleetwise-decodermanifest-obdinterface-pidrequestintervalseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pid_request_interval_seconds: Option<::Value<String>>,
        /// Property [`RequestMessageId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-obdinterface.html#cfn-iotfleetwise-decodermanifest-obdinterface-requestmessageid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub request_message_id: ::Value<String>,
        /// Property [`UseExtendedIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-obdinterface.html#cfn-iotfleetwise-decodermanifest-obdinterface-useextendedids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub use_extended_ids: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ObdInterface {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref dtc_request_interval_seconds) = self.dtc_request_interval_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DtcRequestIntervalSeconds", dtc_request_interval_seconds)?;
            }
            if let Some(ref has_transmission_ecu) = self.has_transmission_ecu {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HasTransmissionEcu", has_transmission_ecu)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            if let Some(ref obd_standard) = self.obd_standard {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ObdStandard", obd_standard)?;
            }
            if let Some(ref pid_request_interval_seconds) = self.pid_request_interval_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PidRequestIntervalSeconds", pid_request_interval_seconds)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequestMessageId", &self.request_message_id)?;
            if let Some(ref use_extended_ids) = self.use_extended_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UseExtendedIds", use_extended_ids)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ObdInterface {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ObdInterface, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ObdInterface;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ObdInterface")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dtc_request_interval_seconds: Option<::Value<String>> = None;
                    let mut has_transmission_ecu: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut obd_standard: Option<::Value<String>> = None;
                    let mut pid_request_interval_seconds: Option<::Value<String>> = None;
                    let mut request_message_id: Option<::Value<String>> = None;
                    let mut use_extended_ids: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DtcRequestIntervalSeconds" => {
                                dtc_request_interval_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HasTransmissionEcu" => {
                                has_transmission_ecu = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ObdStandard" => {
                                obd_standard = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PidRequestIntervalSeconds" => {
                                pid_request_interval_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RequestMessageId" => {
                                request_message_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UseExtendedIds" => {
                                use_extended_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ObdInterface {
                        dtc_request_interval_seconds: dtc_request_interval_seconds,
                        has_transmission_ecu: has_transmission_ecu,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        obd_standard: obd_standard,
                        pid_request_interval_seconds: pid_request_interval_seconds,
                        request_message_id: request_message_id.ok_or(::serde::de::Error::missing_field("RequestMessageId"))?,
                        use_extended_ids: use_extended_ids,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTFleetWise::DecoderManifest.ObdSignal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-obdsignal.html) property type.
    #[derive(Debug, Default)]
    pub struct ObdSignal {
        /// Property [`BitMaskLength`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-obdsignal.html#cfn-iotfleetwise-decodermanifest-obdsignal-bitmasklength).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bit_mask_length: Option<::Value<String>>,
        /// Property [`BitRightShift`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-obdsignal.html#cfn-iotfleetwise-decodermanifest-obdsignal-bitrightshift).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bit_right_shift: Option<::Value<String>>,
        /// Property [`ByteLength`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-obdsignal.html#cfn-iotfleetwise-decodermanifest-obdsignal-bytelength).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub byte_length: ::Value<String>,
        /// Property [`Offset`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-obdsignal.html#cfn-iotfleetwise-decodermanifest-obdsignal-offset).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub offset: ::Value<String>,
        /// Property [`Pid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-obdsignal.html#cfn-iotfleetwise-decodermanifest-obdsignal-pid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pid: ::Value<String>,
        /// Property [`PidResponseLength`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-obdsignal.html#cfn-iotfleetwise-decodermanifest-obdsignal-pidresponselength).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pid_response_length: ::Value<String>,
        /// Property [`Scaling`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-obdsignal.html#cfn-iotfleetwise-decodermanifest-obdsignal-scaling).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scaling: ::Value<String>,
        /// Property [`ServiceMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-obdsignal.html#cfn-iotfleetwise-decodermanifest-obdsignal-servicemode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub service_mode: ::Value<String>,
        /// Property [`StartByte`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-obdsignal.html#cfn-iotfleetwise-decodermanifest-obdsignal-startbyte).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub start_byte: ::Value<String>,
    }

    impl ::codec::SerializeValue for ObdSignal {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref bit_mask_length) = self.bit_mask_length {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BitMaskLength", bit_mask_length)?;
            }
            if let Some(ref bit_right_shift) = self.bit_right_shift {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BitRightShift", bit_right_shift)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ByteLength", &self.byte_length)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Offset", &self.offset)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Pid", &self.pid)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PidResponseLength", &self.pid_response_length)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scaling", &self.scaling)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceMode", &self.service_mode)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartByte", &self.start_byte)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ObdSignal {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ObdSignal, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ObdSignal;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ObdSignal")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bit_mask_length: Option<::Value<String>> = None;
                    let mut bit_right_shift: Option<::Value<String>> = None;
                    let mut byte_length: Option<::Value<String>> = None;
                    let mut offset: Option<::Value<String>> = None;
                    let mut pid: Option<::Value<String>> = None;
                    let mut pid_response_length: Option<::Value<String>> = None;
                    let mut scaling: Option<::Value<String>> = None;
                    let mut service_mode: Option<::Value<String>> = None;
                    let mut start_byte: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BitMaskLength" => {
                                bit_mask_length = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BitRightShift" => {
                                bit_right_shift = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ByteLength" => {
                                byte_length = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Offset" => {
                                offset = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Pid" => {
                                pid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PidResponseLength" => {
                                pid_response_length = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Scaling" => {
                                scaling = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServiceMode" => {
                                service_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StartByte" => {
                                start_byte = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ObdSignal {
                        bit_mask_length: bit_mask_length,
                        bit_right_shift: bit_right_shift,
                        byte_length: byte_length.ok_or(::serde::de::Error::missing_field("ByteLength"))?,
                        offset: offset.ok_or(::serde::de::Error::missing_field("Offset"))?,
                        pid: pid.ok_or(::serde::de::Error::missing_field("Pid"))?,
                        pid_response_length: pid_response_length.ok_or(::serde::de::Error::missing_field("PidResponseLength"))?,
                        scaling: scaling.ok_or(::serde::de::Error::missing_field("Scaling"))?,
                        service_mode: service_mode.ok_or(::serde::de::Error::missing_field("ServiceMode"))?,
                        start_byte: start_byte.ok_or(::serde::de::Error::missing_field("StartByte"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTFleetWise::DecoderManifest.SignalDecodersItems`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-signaldecodersitems.html) property type.
    #[derive(Debug, Default)]
    pub struct SignalDecodersItems {
        /// Property [`CanSignal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-signaldecodersitems.html#cfn-iotfleetwise-decodermanifest-signaldecodersitems-cansignal).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub can_signal: Option<::Value<CanSignal>>,
        /// Property [`FullyQualifiedName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-signaldecodersitems.html#cfn-iotfleetwise-decodermanifest-signaldecodersitems-fullyqualifiedname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub fully_qualified_name: ::Value<String>,
        /// Property [`InterfaceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-signaldecodersitems.html#cfn-iotfleetwise-decodermanifest-signaldecodersitems-interfaceid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub interface_id: ::Value<String>,
        /// Property [`ObdSignal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-signaldecodersitems.html#cfn-iotfleetwise-decodermanifest-signaldecodersitems-obdsignal).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub obd_signal: Option<::Value<ObdSignal>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-signaldecodersitems.html#cfn-iotfleetwise-decodermanifest-signaldecodersitems-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for SignalDecodersItems {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref can_signal) = self.can_signal {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CanSignal", can_signal)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FullyQualifiedName", &self.fully_qualified_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InterfaceId", &self.interface_id)?;
            if let Some(ref obd_signal) = self.obd_signal {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ObdSignal", obd_signal)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SignalDecodersItems {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SignalDecodersItems, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SignalDecodersItems;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SignalDecodersItems")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut can_signal: Option<::Value<CanSignal>> = None;
                    let mut fully_qualified_name: Option<::Value<String>> = None;
                    let mut interface_id: Option<::Value<String>> = None;
                    let mut obd_signal: Option<::Value<ObdSignal>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CanSignal" => {
                                can_signal = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FullyQualifiedName" => {
                                fully_qualified_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InterfaceId" => {
                                interface_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ObdSignal" => {
                                obd_signal = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SignalDecodersItems {
                        can_signal: can_signal,
                        fully_qualified_name: fully_qualified_name.ok_or(::serde::de::Error::missing_field("FullyQualifiedName"))?,
                        interface_id: interface_id.ok_or(::serde::de::Error::missing_field("InterfaceId"))?,
                        obd_signal: obd_signal,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod signal_catalog {
    //! Property types for the `SignalCatalog` resource.

    /// The [`AWS::IoTFleetWise::SignalCatalog.Actuator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-actuator.html) property type.
    #[derive(Debug, Default)]
    pub struct Actuator {
        /// Property [`AllowedValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-actuator.html#cfn-iotfleetwise-signalcatalog-actuator-allowedvalues).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allowed_values: Option<::ValueList<String>>,
        /// Property [`AssignedValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-actuator.html#cfn-iotfleetwise-signalcatalog-actuator-assignedvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub assigned_value: Option<::Value<String>>,
        /// Property [`DataType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-actuator.html#cfn-iotfleetwise-signalcatalog-actuator-datatype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_type: ::Value<String>,
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-actuator.html#cfn-iotfleetwise-signalcatalog-actuator-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`FullyQualifiedName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-actuator.html#cfn-iotfleetwise-signalcatalog-actuator-fullyqualifiedname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub fully_qualified_name: ::Value<String>,
        /// Property [`Max`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-actuator.html#cfn-iotfleetwise-signalcatalog-actuator-max).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max: Option<::Value<f64>>,
        /// Property [`Min`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-actuator.html#cfn-iotfleetwise-signalcatalog-actuator-min).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min: Option<::Value<f64>>,
        /// Property [`Unit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-actuator.html#cfn-iotfleetwise-signalcatalog-actuator-unit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unit: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Actuator {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref allowed_values) = self.allowed_values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowedValues", allowed_values)?;
            }
            if let Some(ref assigned_value) = self.assigned_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssignedValue", assigned_value)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataType", &self.data_type)?;
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FullyQualifiedName", &self.fully_qualified_name)?;
            if let Some(ref max) = self.max {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Max", max)?;
            }
            if let Some(ref min) = self.min {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Min", min)?;
            }
            if let Some(ref unit) = self.unit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Unit", unit)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Actuator {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Actuator, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Actuator;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Actuator")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allowed_values: Option<::ValueList<String>> = None;
                    let mut assigned_value: Option<::Value<String>> = None;
                    let mut data_type: Option<::Value<String>> = None;
                    let mut description: Option<::Value<String>> = None;
                    let mut fully_qualified_name: Option<::Value<String>> = None;
                    let mut max: Option<::Value<f64>> = None;
                    let mut min: Option<::Value<f64>> = None;
                    let mut unit: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllowedValues" => {
                                allowed_values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AssignedValue" => {
                                assigned_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataType" => {
                                data_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FullyQualifiedName" => {
                                fully_qualified_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Max" => {
                                max = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Min" => {
                                min = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Unit" => {
                                unit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Actuator {
                        allowed_values: allowed_values,
                        assigned_value: assigned_value,
                        data_type: data_type.ok_or(::serde::de::Error::missing_field("DataType"))?,
                        description: description,
                        fully_qualified_name: fully_qualified_name.ok_or(::serde::de::Error::missing_field("FullyQualifiedName"))?,
                        max: max,
                        min: min,
                        unit: unit,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTFleetWise::SignalCatalog.Attribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-attribute.html) property type.
    #[derive(Debug, Default)]
    pub struct Attribute {
        /// Property [`AllowedValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-attribute.html#cfn-iotfleetwise-signalcatalog-attribute-allowedvalues).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allowed_values: Option<::ValueList<String>>,
        /// Property [`AssignedValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-attribute.html#cfn-iotfleetwise-signalcatalog-attribute-assignedvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub assigned_value: Option<::Value<String>>,
        /// Property [`DataType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-attribute.html#cfn-iotfleetwise-signalcatalog-attribute-datatype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_type: ::Value<String>,
        /// Property [`DefaultValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-attribute.html#cfn-iotfleetwise-signalcatalog-attribute-defaultvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_value: Option<::Value<String>>,
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-attribute.html#cfn-iotfleetwise-signalcatalog-attribute-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`FullyQualifiedName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-attribute.html#cfn-iotfleetwise-signalcatalog-attribute-fullyqualifiedname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub fully_qualified_name: ::Value<String>,
        /// Property [`Max`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-attribute.html#cfn-iotfleetwise-signalcatalog-attribute-max).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max: Option<::Value<f64>>,
        /// Property [`Min`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-attribute.html#cfn-iotfleetwise-signalcatalog-attribute-min).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min: Option<::Value<f64>>,
        /// Property [`Unit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-attribute.html#cfn-iotfleetwise-signalcatalog-attribute-unit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unit: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Attribute {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref allowed_values) = self.allowed_values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowedValues", allowed_values)?;
            }
            if let Some(ref assigned_value) = self.assigned_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssignedValue", assigned_value)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataType", &self.data_type)?;
            if let Some(ref default_value) = self.default_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultValue", default_value)?;
            }
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FullyQualifiedName", &self.fully_qualified_name)?;
            if let Some(ref max) = self.max {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Max", max)?;
            }
            if let Some(ref min) = self.min {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Min", min)?;
            }
            if let Some(ref unit) = self.unit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Unit", unit)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Attribute {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Attribute, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Attribute;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Attribute")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allowed_values: Option<::ValueList<String>> = None;
                    let mut assigned_value: Option<::Value<String>> = None;
                    let mut data_type: Option<::Value<String>> = None;
                    let mut default_value: Option<::Value<String>> = None;
                    let mut description: Option<::Value<String>> = None;
                    let mut fully_qualified_name: Option<::Value<String>> = None;
                    let mut max: Option<::Value<f64>> = None;
                    let mut min: Option<::Value<f64>> = None;
                    let mut unit: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllowedValues" => {
                                allowed_values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AssignedValue" => {
                                assigned_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataType" => {
                                data_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DefaultValue" => {
                                default_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FullyQualifiedName" => {
                                fully_qualified_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Max" => {
                                max = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Min" => {
                                min = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Unit" => {
                                unit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Attribute {
                        allowed_values: allowed_values,
                        assigned_value: assigned_value,
                        data_type: data_type.ok_or(::serde::de::Error::missing_field("DataType"))?,
                        default_value: default_value,
                        description: description,
                        fully_qualified_name: fully_qualified_name.ok_or(::serde::de::Error::missing_field("FullyQualifiedName"))?,
                        max: max,
                        min: min,
                        unit: unit,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTFleetWise::SignalCatalog.Branch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-branch.html) property type.
    #[derive(Debug, Default)]
    pub struct Branch {
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-branch.html#cfn-iotfleetwise-signalcatalog-branch-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`FullyQualifiedName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-branch.html#cfn-iotfleetwise-signalcatalog-branch-fullyqualifiedname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub fully_qualified_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for Branch {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FullyQualifiedName", &self.fully_qualified_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Branch {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Branch, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Branch;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Branch")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut description: Option<::Value<String>> = None;
                    let mut fully_qualified_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FullyQualifiedName" => {
                                fully_qualified_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Branch {
                        description: description,
                        fully_qualified_name: fully_qualified_name.ok_or(::serde::de::Error::missing_field("FullyQualifiedName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTFleetWise::SignalCatalog.Node`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-node.html) property type.
    #[derive(Debug, Default)]
    pub struct Node {
        /// Property [`Actuator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-node.html#cfn-iotfleetwise-signalcatalog-node-actuator).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub actuator: Option<::Value<Actuator>>,
        /// Property [`Attribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-node.html#cfn-iotfleetwise-signalcatalog-node-attribute).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attribute: Option<::Value<Attribute>>,
        /// Property [`Branch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-node.html#cfn-iotfleetwise-signalcatalog-node-branch).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub branch: Option<::Value<Branch>>,
        /// Property [`Sensor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-node.html#cfn-iotfleetwise-signalcatalog-node-sensor).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sensor: Option<::Value<Sensor>>,
    }

    impl ::codec::SerializeValue for Node {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref actuator) = self.actuator {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Actuator", actuator)?;
            }
            if let Some(ref attribute) = self.attribute {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Attribute", attribute)?;
            }
            if let Some(ref branch) = self.branch {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Branch", branch)?;
            }
            if let Some(ref sensor) = self.sensor {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Sensor", sensor)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Node {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Node, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Node;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Node")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut actuator: Option<::Value<Actuator>> = None;
                    let mut attribute: Option<::Value<Attribute>> = None;
                    let mut branch: Option<::Value<Branch>> = None;
                    let mut sensor: Option<::Value<Sensor>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Actuator" => {
                                actuator = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Attribute" => {
                                attribute = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Branch" => {
                                branch = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Sensor" => {
                                sensor = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Node {
                        actuator: actuator,
                        attribute: attribute,
                        branch: branch,
                        sensor: sensor,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTFleetWise::SignalCatalog.NodeCounts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-nodecounts.html) property type.
    #[derive(Debug, Default)]
    pub struct NodeCounts {
        /// Property [`TotalActuators`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-nodecounts.html#cfn-iotfleetwise-signalcatalog-nodecounts-totalactuators).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub total_actuators: Option<::Value<f64>>,
        /// Property [`TotalAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-nodecounts.html#cfn-iotfleetwise-signalcatalog-nodecounts-totalattributes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub total_attributes: Option<::Value<f64>>,
        /// Property [`TotalBranches`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-nodecounts.html#cfn-iotfleetwise-signalcatalog-nodecounts-totalbranches).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub total_branches: Option<::Value<f64>>,
        /// Property [`TotalNodes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-nodecounts.html#cfn-iotfleetwise-signalcatalog-nodecounts-totalnodes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub total_nodes: Option<::Value<f64>>,
        /// Property [`TotalSensors`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-nodecounts.html#cfn-iotfleetwise-signalcatalog-nodecounts-totalsensors).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub total_sensors: Option<::Value<f64>>,
    }

    impl ::codec::SerializeValue for NodeCounts {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref total_actuators) = self.total_actuators {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TotalActuators", total_actuators)?;
            }
            if let Some(ref total_attributes) = self.total_attributes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TotalAttributes", total_attributes)?;
            }
            if let Some(ref total_branches) = self.total_branches {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TotalBranches", total_branches)?;
            }
            if let Some(ref total_nodes) = self.total_nodes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TotalNodes", total_nodes)?;
            }
            if let Some(ref total_sensors) = self.total_sensors {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TotalSensors", total_sensors)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NodeCounts {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NodeCounts, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NodeCounts;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NodeCounts")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut total_actuators: Option<::Value<f64>> = None;
                    let mut total_attributes: Option<::Value<f64>> = None;
                    let mut total_branches: Option<::Value<f64>> = None;
                    let mut total_nodes: Option<::Value<f64>> = None;
                    let mut total_sensors: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TotalActuators" => {
                                total_actuators = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TotalAttributes" => {
                                total_attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TotalBranches" => {
                                total_branches = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TotalNodes" => {
                                total_nodes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TotalSensors" => {
                                total_sensors = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NodeCounts {
                        total_actuators: total_actuators,
                        total_attributes: total_attributes,
                        total_branches: total_branches,
                        total_nodes: total_nodes,
                        total_sensors: total_sensors,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoTFleetWise::SignalCatalog.Sensor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-sensor.html) property type.
    #[derive(Debug, Default)]
    pub struct Sensor {
        /// Property [`AllowedValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-sensor.html#cfn-iotfleetwise-signalcatalog-sensor-allowedvalues).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allowed_values: Option<::ValueList<String>>,
        /// Property [`DataType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-sensor.html#cfn-iotfleetwise-signalcatalog-sensor-datatype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_type: ::Value<String>,
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-sensor.html#cfn-iotfleetwise-signalcatalog-sensor-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`FullyQualifiedName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-sensor.html#cfn-iotfleetwise-signalcatalog-sensor-fullyqualifiedname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub fully_qualified_name: ::Value<String>,
        /// Property [`Max`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-sensor.html#cfn-iotfleetwise-signalcatalog-sensor-max).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max: Option<::Value<f64>>,
        /// Property [`Min`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-sensor.html#cfn-iotfleetwise-signalcatalog-sensor-min).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min: Option<::Value<f64>>,
        /// Property [`Unit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-sensor.html#cfn-iotfleetwise-signalcatalog-sensor-unit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unit: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Sensor {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref allowed_values) = self.allowed_values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowedValues", allowed_values)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataType", &self.data_type)?;
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FullyQualifiedName", &self.fully_qualified_name)?;
            if let Some(ref max) = self.max {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Max", max)?;
            }
            if let Some(ref min) = self.min {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Min", min)?;
            }
            if let Some(ref unit) = self.unit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Unit", unit)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Sensor {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Sensor, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Sensor;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Sensor")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allowed_values: Option<::ValueList<String>> = None;
                    let mut data_type: Option<::Value<String>> = None;
                    let mut description: Option<::Value<String>> = None;
                    let mut fully_qualified_name: Option<::Value<String>> = None;
                    let mut max: Option<::Value<f64>> = None;
                    let mut min: Option<::Value<f64>> = None;
                    let mut unit: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllowedValues" => {
                                allowed_values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataType" => {
                                data_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FullyQualifiedName" => {
                                fully_qualified_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Max" => {
                                max = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Min" => {
                                min = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Unit" => {
                                unit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Sensor {
                        allowed_values: allowed_values,
                        data_type: data_type.ok_or(::serde::de::Error::missing_field("DataType"))?,
                        description: description,
                        fully_qualified_name: fully_qualified_name.ok_or(::serde::de::Error::missing_field("FullyQualifiedName"))?,
                        max: max,
                        min: min,
                        unit: unit,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
