//! Types for the `GroundStation` service.

/// The [`AWS::GroundStation::Config`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-groundstation-config.html) resource type.
#[derive(Debug, Default)]
pub struct Config {
    properties: ConfigProperties
}

/// Properties for the `Config` resource.
#[derive(Debug, Default)]
pub struct ConfigProperties {
    /// Property [`ConfigData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-groundstation-config.html#cfn-groundstation-config-configdata).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub config_data: ::Value<self::config::ConfigData>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-groundstation-config.html#cfn-groundstation-config-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-groundstation-config.html#cfn-groundstation-config-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ConfigProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfigData", &self.config_data)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ConfigProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ConfigProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ConfigProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ConfigProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut config_data: Option<::Value<self::config::ConfigData>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ConfigData" => {
                            config_data = ::serde::de::MapAccess::next_value(&mut map)?;
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

                Ok(ConfigProperties {
                    config_data: config_data.ok_or(::serde::de::Error::missing_field("ConfigData"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Config {
    type Properties = ConfigProperties;
    const TYPE: &'static str = "AWS::GroundStation::Config";
    fn properties(&self) -> &ConfigProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ConfigProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Config {}

impl From<ConfigProperties> for Config {
    fn from(properties: ConfigProperties) -> Config {
        Config { properties }
    }
}

/// The [`AWS::GroundStation::DataflowEndpointGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-groundstation-dataflowendpointgroup.html) resource type.
#[derive(Debug, Default)]
pub struct DataflowEndpointGroup {
    properties: DataflowEndpointGroupProperties
}

/// Properties for the `DataflowEndpointGroup` resource.
#[derive(Debug, Default)]
pub struct DataflowEndpointGroupProperties {
    /// Property [`EndpointDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-groundstation-dataflowendpointgroup.html#cfn-groundstation-dataflowendpointgroup-endpointdetails).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub endpoint_details: ::ValueList<self::dataflow_endpoint_group::EndpointDetails>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-groundstation-dataflowendpointgroup.html#cfn-groundstation-dataflowendpointgroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for DataflowEndpointGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointDetails", &self.endpoint_details)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DataflowEndpointGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DataflowEndpointGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DataflowEndpointGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DataflowEndpointGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut endpoint_details: Option<::ValueList<self::dataflow_endpoint_group::EndpointDetails>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "EndpointDetails" => {
                            endpoint_details = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DataflowEndpointGroupProperties {
                    endpoint_details: endpoint_details.ok_or(::serde::de::Error::missing_field("EndpointDetails"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DataflowEndpointGroup {
    type Properties = DataflowEndpointGroupProperties;
    const TYPE: &'static str = "AWS::GroundStation::DataflowEndpointGroup";
    fn properties(&self) -> &DataflowEndpointGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DataflowEndpointGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DataflowEndpointGroup {}

impl From<DataflowEndpointGroupProperties> for DataflowEndpointGroup {
    fn from(properties: DataflowEndpointGroupProperties) -> DataflowEndpointGroup {
        DataflowEndpointGroup { properties }
    }
}

/// The [`AWS::GroundStation::MissionProfile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-groundstation-missionprofile.html) resource type.
#[derive(Debug, Default)]
pub struct MissionProfile {
    properties: MissionProfileProperties
}

/// Properties for the `MissionProfile` resource.
#[derive(Debug, Default)]
pub struct MissionProfileProperties {
    /// Property [`ContactPostPassDurationSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-groundstation-missionprofile.html#cfn-groundstation-missionprofile-contactpostpassdurationseconds).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub contact_post_pass_duration_seconds: Option<::Value<u32>>,
    /// Property [`ContactPrePassDurationSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-groundstation-missionprofile.html#cfn-groundstation-missionprofile-contactprepassdurationseconds).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub contact_pre_pass_duration_seconds: Option<::Value<u32>>,
    /// Property [`DataflowEdges`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-groundstation-missionprofile.html#cfn-groundstation-missionprofile-dataflowedges).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub dataflow_edges: ::ValueList<self::mission_profile::DataflowEdge>,
    /// Property [`MinimumViableContactDurationSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-groundstation-missionprofile.html#cfn-groundstation-missionprofile-minimumviablecontactdurationseconds).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub minimum_viable_contact_duration_seconds: ::Value<u32>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-groundstation-missionprofile.html#cfn-groundstation-missionprofile-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-groundstation-missionprofile.html#cfn-groundstation-missionprofile-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TrackingConfigArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-groundstation-missionprofile.html#cfn-groundstation-missionprofile-trackingconfigarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tracking_config_arn: ::Value<String>,
}

impl ::serde::Serialize for MissionProfileProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref contact_post_pass_duration_seconds) = self.contact_post_pass_duration_seconds {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContactPostPassDurationSeconds", contact_post_pass_duration_seconds)?;
        }
        if let Some(ref contact_pre_pass_duration_seconds) = self.contact_pre_pass_duration_seconds {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContactPrePassDurationSeconds", contact_pre_pass_duration_seconds)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataflowEdges", &self.dataflow_edges)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinimumViableContactDurationSeconds", &self.minimum_viable_contact_duration_seconds)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TrackingConfigArn", &self.tracking_config_arn)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for MissionProfileProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<MissionProfileProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = MissionProfileProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type MissionProfileProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut contact_post_pass_duration_seconds: Option<::Value<u32>> = None;
                let mut contact_pre_pass_duration_seconds: Option<::Value<u32>> = None;
                let mut dataflow_edges: Option<::ValueList<self::mission_profile::DataflowEdge>> = None;
                let mut minimum_viable_contact_duration_seconds: Option<::Value<u32>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut tracking_config_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ContactPostPassDurationSeconds" => {
                            contact_post_pass_duration_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ContactPrePassDurationSeconds" => {
                            contact_pre_pass_duration_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DataflowEdges" => {
                            dataflow_edges = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MinimumViableContactDurationSeconds" => {
                            minimum_viable_contact_duration_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TrackingConfigArn" => {
                            tracking_config_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(MissionProfileProperties {
                    contact_post_pass_duration_seconds: contact_post_pass_duration_seconds,
                    contact_pre_pass_duration_seconds: contact_pre_pass_duration_seconds,
                    dataflow_edges: dataflow_edges.ok_or(::serde::de::Error::missing_field("DataflowEdges"))?,
                    minimum_viable_contact_duration_seconds: minimum_viable_contact_duration_seconds.ok_or(::serde::de::Error::missing_field("MinimumViableContactDurationSeconds"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    tags: tags,
                    tracking_config_arn: tracking_config_arn.ok_or(::serde::de::Error::missing_field("TrackingConfigArn"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for MissionProfile {
    type Properties = MissionProfileProperties;
    const TYPE: &'static str = "AWS::GroundStation::MissionProfile";
    fn properties(&self) -> &MissionProfileProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut MissionProfileProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for MissionProfile {}

impl From<MissionProfileProperties> for MissionProfile {
    fn from(properties: MissionProfileProperties) -> MissionProfile {
        MissionProfile { properties }
    }
}

pub mod config {
    //! Property types for the `Config` resource.

    /// The [`AWS::GroundStation::Config.AntennaDownlinkConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-antennadownlinkconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct AntennaDownlinkConfig {
        /// Property [`SpectrumConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-antennadownlinkconfig.html#cfn-groundstation-config-antennadownlinkconfig-spectrumconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub spectrum_config: Option<::Value<SpectrumConfig>>,
    }

    impl ::codec::SerializeValue for AntennaDownlinkConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref spectrum_config) = self.spectrum_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SpectrumConfig", spectrum_config)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AntennaDownlinkConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AntennaDownlinkConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AntennaDownlinkConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AntennaDownlinkConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut spectrum_config: Option<::Value<SpectrumConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SpectrumConfig" => {
                                spectrum_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AntennaDownlinkConfig {
                        spectrum_config: spectrum_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::GroundStation::Config.AntennaDownlinkDemodDecodeConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-antennadownlinkdemoddecodeconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct AntennaDownlinkDemodDecodeConfig {
        /// Property [`DecodeConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-antennadownlinkdemoddecodeconfig.html#cfn-groundstation-config-antennadownlinkdemoddecodeconfig-decodeconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub decode_config: Option<::Value<DecodeConfig>>,
        /// Property [`DemodulationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-antennadownlinkdemoddecodeconfig.html#cfn-groundstation-config-antennadownlinkdemoddecodeconfig-demodulationconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub demodulation_config: Option<::Value<DemodulationConfig>>,
        /// Property [`SpectrumConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-antennadownlinkdemoddecodeconfig.html#cfn-groundstation-config-antennadownlinkdemoddecodeconfig-spectrumconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub spectrum_config: Option<::Value<SpectrumConfig>>,
    }

    impl ::codec::SerializeValue for AntennaDownlinkDemodDecodeConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref decode_config) = self.decode_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DecodeConfig", decode_config)?;
            }
            if let Some(ref demodulation_config) = self.demodulation_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DemodulationConfig", demodulation_config)?;
            }
            if let Some(ref spectrum_config) = self.spectrum_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SpectrumConfig", spectrum_config)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AntennaDownlinkDemodDecodeConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AntennaDownlinkDemodDecodeConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AntennaDownlinkDemodDecodeConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AntennaDownlinkDemodDecodeConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut decode_config: Option<::Value<DecodeConfig>> = None;
                    let mut demodulation_config: Option<::Value<DemodulationConfig>> = None;
                    let mut spectrum_config: Option<::Value<SpectrumConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DecodeConfig" => {
                                decode_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DemodulationConfig" => {
                                demodulation_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SpectrumConfig" => {
                                spectrum_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AntennaDownlinkDemodDecodeConfig {
                        decode_config: decode_config,
                        demodulation_config: demodulation_config,
                        spectrum_config: spectrum_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::GroundStation::Config.AntennaUplinkConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-antennauplinkconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct AntennaUplinkConfig {
        /// Property [`SpectrumConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-antennauplinkconfig.html#cfn-groundstation-config-antennauplinkconfig-spectrumconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub spectrum_config: Option<::Value<UplinkSpectrumConfig>>,
        /// Property [`TargetEirp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-antennauplinkconfig.html#cfn-groundstation-config-antennauplinkconfig-targeteirp).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_eirp: Option<::Value<Eirp>>,
        /// Property [`TransmitDisabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-antennauplinkconfig.html#cfn-groundstation-config-antennauplinkconfig-transmitdisabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub transmit_disabled: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for AntennaUplinkConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref spectrum_config) = self.spectrum_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SpectrumConfig", spectrum_config)?;
            }
            if let Some(ref target_eirp) = self.target_eirp {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetEirp", target_eirp)?;
            }
            if let Some(ref transmit_disabled) = self.transmit_disabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TransmitDisabled", transmit_disabled)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AntennaUplinkConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AntennaUplinkConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AntennaUplinkConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AntennaUplinkConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut spectrum_config: Option<::Value<UplinkSpectrumConfig>> = None;
                    let mut target_eirp: Option<::Value<Eirp>> = None;
                    let mut transmit_disabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SpectrumConfig" => {
                                spectrum_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetEirp" => {
                                target_eirp = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TransmitDisabled" => {
                                transmit_disabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AntennaUplinkConfig {
                        spectrum_config: spectrum_config,
                        target_eirp: target_eirp,
                        transmit_disabled: transmit_disabled,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::GroundStation::Config.ConfigData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-configdata.html) property type.
    #[derive(Debug, Default)]
    pub struct ConfigData {
        /// Property [`AntennaDownlinkConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-configdata.html#cfn-groundstation-config-configdata-antennadownlinkconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub antenna_downlink_config: Option<::Value<AntennaDownlinkConfig>>,
        /// Property [`AntennaDownlinkDemodDecodeConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-configdata.html#cfn-groundstation-config-configdata-antennadownlinkdemoddecodeconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub antenna_downlink_demod_decode_config: Option<::Value<AntennaDownlinkDemodDecodeConfig>>,
        /// Property [`AntennaUplinkConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-configdata.html#cfn-groundstation-config-configdata-antennauplinkconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub antenna_uplink_config: Option<::Value<AntennaUplinkConfig>>,
        /// Property [`DataflowEndpointConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-configdata.html#cfn-groundstation-config-configdata-dataflowendpointconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dataflow_endpoint_config: Option<::Value<DataflowEndpointConfig>>,
        /// Property [`S3RecordingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-configdata.html#cfn-groundstation-config-configdata-s3recordingconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_recording_config: Option<::Value<S3RecordingConfig>>,
        /// Property [`TrackingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-configdata.html#cfn-groundstation-config-configdata-trackingconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tracking_config: Option<::Value<TrackingConfig>>,
        /// Property [`UplinkEchoConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-configdata.html#cfn-groundstation-config-configdata-uplinkechoconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub uplink_echo_config: Option<::Value<UplinkEchoConfig>>,
    }

    impl ::codec::SerializeValue for ConfigData {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref antenna_downlink_config) = self.antenna_downlink_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AntennaDownlinkConfig", antenna_downlink_config)?;
            }
            if let Some(ref antenna_downlink_demod_decode_config) = self.antenna_downlink_demod_decode_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AntennaDownlinkDemodDecodeConfig", antenna_downlink_demod_decode_config)?;
            }
            if let Some(ref antenna_uplink_config) = self.antenna_uplink_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AntennaUplinkConfig", antenna_uplink_config)?;
            }
            if let Some(ref dataflow_endpoint_config) = self.dataflow_endpoint_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataflowEndpointConfig", dataflow_endpoint_config)?;
            }
            if let Some(ref s3_recording_config) = self.s3_recording_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3RecordingConfig", s3_recording_config)?;
            }
            if let Some(ref tracking_config) = self.tracking_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TrackingConfig", tracking_config)?;
            }
            if let Some(ref uplink_echo_config) = self.uplink_echo_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UplinkEchoConfig", uplink_echo_config)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConfigData {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConfigData, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConfigData;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConfigData")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut antenna_downlink_config: Option<::Value<AntennaDownlinkConfig>> = None;
                    let mut antenna_downlink_demod_decode_config: Option<::Value<AntennaDownlinkDemodDecodeConfig>> = None;
                    let mut antenna_uplink_config: Option<::Value<AntennaUplinkConfig>> = None;
                    let mut dataflow_endpoint_config: Option<::Value<DataflowEndpointConfig>> = None;
                    let mut s3_recording_config: Option<::Value<S3RecordingConfig>> = None;
                    let mut tracking_config: Option<::Value<TrackingConfig>> = None;
                    let mut uplink_echo_config: Option<::Value<UplinkEchoConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AntennaDownlinkConfig" => {
                                antenna_downlink_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AntennaDownlinkDemodDecodeConfig" => {
                                antenna_downlink_demod_decode_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AntennaUplinkConfig" => {
                                antenna_uplink_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataflowEndpointConfig" => {
                                dataflow_endpoint_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3RecordingConfig" => {
                                s3_recording_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TrackingConfig" => {
                                tracking_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UplinkEchoConfig" => {
                                uplink_echo_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConfigData {
                        antenna_downlink_config: antenna_downlink_config,
                        antenna_downlink_demod_decode_config: antenna_downlink_demod_decode_config,
                        antenna_uplink_config: antenna_uplink_config,
                        dataflow_endpoint_config: dataflow_endpoint_config,
                        s3_recording_config: s3_recording_config,
                        tracking_config: tracking_config,
                        uplink_echo_config: uplink_echo_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::GroundStation::Config.DataflowEndpointConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-dataflowendpointconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct DataflowEndpointConfig {
        /// Property [`DataflowEndpointName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-dataflowendpointconfig.html#cfn-groundstation-config-dataflowendpointconfig-dataflowendpointname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dataflow_endpoint_name: Option<::Value<String>>,
        /// Property [`DataflowEndpointRegion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-dataflowendpointconfig.html#cfn-groundstation-config-dataflowendpointconfig-dataflowendpointregion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dataflow_endpoint_region: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DataflowEndpointConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref dataflow_endpoint_name) = self.dataflow_endpoint_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataflowEndpointName", dataflow_endpoint_name)?;
            }
            if let Some(ref dataflow_endpoint_region) = self.dataflow_endpoint_region {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataflowEndpointRegion", dataflow_endpoint_region)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataflowEndpointConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataflowEndpointConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataflowEndpointConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataflowEndpointConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dataflow_endpoint_name: Option<::Value<String>> = None;
                    let mut dataflow_endpoint_region: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataflowEndpointName" => {
                                dataflow_endpoint_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataflowEndpointRegion" => {
                                dataflow_endpoint_region = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataflowEndpointConfig {
                        dataflow_endpoint_name: dataflow_endpoint_name,
                        dataflow_endpoint_region: dataflow_endpoint_region,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::GroundStation::Config.DecodeConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-decodeconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct DecodeConfig {
        /// Property [`UnvalidatedJSON`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-decodeconfig.html#cfn-groundstation-config-decodeconfig-unvalidatedjson).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unvalidated_json: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DecodeConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref unvalidated_json) = self.unvalidated_json {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UnvalidatedJSON", unvalidated_json)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DecodeConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DecodeConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DecodeConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DecodeConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut unvalidated_json: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "UnvalidatedJSON" => {
                                unvalidated_json = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DecodeConfig {
                        unvalidated_json: unvalidated_json,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::GroundStation::Config.DemodulationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-demodulationconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct DemodulationConfig {
        /// Property [`UnvalidatedJSON`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-demodulationconfig.html#cfn-groundstation-config-demodulationconfig-unvalidatedjson).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unvalidated_json: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DemodulationConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref unvalidated_json) = self.unvalidated_json {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UnvalidatedJSON", unvalidated_json)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DemodulationConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DemodulationConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DemodulationConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DemodulationConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut unvalidated_json: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "UnvalidatedJSON" => {
                                unvalidated_json = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DemodulationConfig {
                        unvalidated_json: unvalidated_json,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::GroundStation::Config.Eirp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-eirp.html) property type.
    #[derive(Debug, Default)]
    pub struct Eirp {
        /// Property [`Units`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-eirp.html#cfn-groundstation-config-eirp-units).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub units: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-eirp.html#cfn-groundstation-config-eirp-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<f64>>,
    }

    impl ::codec::SerializeValue for Eirp {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref units) = self.units {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Units", units)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Eirp {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Eirp, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Eirp;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Eirp")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut units: Option<::Value<String>> = None;
                    let mut value: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Units" => {
                                units = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Eirp {
                        units: units,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::GroundStation::Config.Frequency`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-frequency.html) property type.
    #[derive(Debug, Default)]
    pub struct Frequency {
        /// Property [`Units`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-frequency.html#cfn-groundstation-config-frequency-units).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub units: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-frequency.html#cfn-groundstation-config-frequency-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<f64>>,
    }

    impl ::codec::SerializeValue for Frequency {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref units) = self.units {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Units", units)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Frequency {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Frequency, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Frequency;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Frequency")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut units: Option<::Value<String>> = None;
                    let mut value: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Units" => {
                                units = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Frequency {
                        units: units,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::GroundStation::Config.FrequencyBandwidth`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-frequencybandwidth.html) property type.
    #[derive(Debug, Default)]
    pub struct FrequencyBandwidth {
        /// Property [`Units`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-frequencybandwidth.html#cfn-groundstation-config-frequencybandwidth-units).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub units: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-frequencybandwidth.html#cfn-groundstation-config-frequencybandwidth-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<f64>>,
    }

    impl ::codec::SerializeValue for FrequencyBandwidth {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref units) = self.units {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Units", units)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FrequencyBandwidth {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FrequencyBandwidth, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FrequencyBandwidth;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FrequencyBandwidth")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut units: Option<::Value<String>> = None;
                    let mut value: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Units" => {
                                units = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FrequencyBandwidth {
                        units: units,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::GroundStation::Config.S3RecordingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-s3recordingconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct S3RecordingConfig {
        /// Property [`BucketArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-s3recordingconfig.html#cfn-groundstation-config-s3recordingconfig-bucketarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_arn: Option<::Value<String>>,
        /// Property [`Prefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-s3recordingconfig.html#cfn-groundstation-config-s3recordingconfig-prefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub prefix: Option<::Value<String>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-s3recordingconfig.html#cfn-groundstation-config-s3recordingconfig-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for S3RecordingConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref bucket_arn) = self.bucket_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketArn", bucket_arn)?;
            }
            if let Some(ref prefix) = self.prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Prefix", prefix)?;
            }
            if let Some(ref role_arn) = self.role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", role_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3RecordingConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3RecordingConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3RecordingConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3RecordingConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket_arn: Option<::Value<String>> = None;
                    let mut prefix: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketArn" => {
                                bucket_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Prefix" => {
                                prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3RecordingConfig {
                        bucket_arn: bucket_arn,
                        prefix: prefix,
                        role_arn: role_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::GroundStation::Config.SpectrumConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-spectrumconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct SpectrumConfig {
        /// Property [`Bandwidth`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-spectrumconfig.html#cfn-groundstation-config-spectrumconfig-bandwidth).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bandwidth: Option<::Value<FrequencyBandwidth>>,
        /// Property [`CenterFrequency`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-spectrumconfig.html#cfn-groundstation-config-spectrumconfig-centerfrequency).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub center_frequency: Option<::Value<Frequency>>,
        /// Property [`Polarization`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-spectrumconfig.html#cfn-groundstation-config-spectrumconfig-polarization).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub polarization: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SpectrumConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref bandwidth) = self.bandwidth {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bandwidth", bandwidth)?;
            }
            if let Some(ref center_frequency) = self.center_frequency {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CenterFrequency", center_frequency)?;
            }
            if let Some(ref polarization) = self.polarization {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Polarization", polarization)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SpectrumConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SpectrumConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SpectrumConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SpectrumConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bandwidth: Option<::Value<FrequencyBandwidth>> = None;
                    let mut center_frequency: Option<::Value<Frequency>> = None;
                    let mut polarization: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Bandwidth" => {
                                bandwidth = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CenterFrequency" => {
                                center_frequency = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Polarization" => {
                                polarization = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SpectrumConfig {
                        bandwidth: bandwidth,
                        center_frequency: center_frequency,
                        polarization: polarization,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::GroundStation::Config.TrackingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-trackingconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct TrackingConfig {
        /// Property [`Autotrack`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-trackingconfig.html#cfn-groundstation-config-trackingconfig-autotrack).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub autotrack: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for TrackingConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref autotrack) = self.autotrack {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Autotrack", autotrack)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TrackingConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TrackingConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TrackingConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TrackingConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut autotrack: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Autotrack" => {
                                autotrack = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TrackingConfig {
                        autotrack: autotrack,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::GroundStation::Config.UplinkEchoConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-uplinkechoconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct UplinkEchoConfig {
        /// Property [`AntennaUplinkConfigArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-uplinkechoconfig.html#cfn-groundstation-config-uplinkechoconfig-antennauplinkconfigarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub antenna_uplink_config_arn: Option<::Value<String>>,
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-uplinkechoconfig.html#cfn-groundstation-config-uplinkechoconfig-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for UplinkEchoConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref antenna_uplink_config_arn) = self.antenna_uplink_config_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AntennaUplinkConfigArn", antenna_uplink_config_arn)?;
            }
            if let Some(ref enabled) = self.enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for UplinkEchoConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<UplinkEchoConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = UplinkEchoConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type UplinkEchoConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut antenna_uplink_config_arn: Option<::Value<String>> = None;
                    let mut enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AntennaUplinkConfigArn" => {
                                antenna_uplink_config_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(UplinkEchoConfig {
                        antenna_uplink_config_arn: antenna_uplink_config_arn,
                        enabled: enabled,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::GroundStation::Config.UplinkSpectrumConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-uplinkspectrumconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct UplinkSpectrumConfig {
        /// Property [`CenterFrequency`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-uplinkspectrumconfig.html#cfn-groundstation-config-uplinkspectrumconfig-centerfrequency).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub center_frequency: Option<::Value<Frequency>>,
        /// Property [`Polarization`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-config-uplinkspectrumconfig.html#cfn-groundstation-config-uplinkspectrumconfig-polarization).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub polarization: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for UplinkSpectrumConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref center_frequency) = self.center_frequency {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CenterFrequency", center_frequency)?;
            }
            if let Some(ref polarization) = self.polarization {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Polarization", polarization)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for UplinkSpectrumConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<UplinkSpectrumConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = UplinkSpectrumConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type UplinkSpectrumConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut center_frequency: Option<::Value<Frequency>> = None;
                    let mut polarization: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CenterFrequency" => {
                                center_frequency = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Polarization" => {
                                polarization = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(UplinkSpectrumConfig {
                        center_frequency: center_frequency,
                        polarization: polarization,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod dataflow_endpoint_group {
    //! Property types for the `DataflowEndpointGroup` resource.

    /// The [`AWS::GroundStation::DataflowEndpointGroup.DataflowEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-dataflowendpointgroup-dataflowendpoint.html) property type.
    #[derive(Debug, Default)]
    pub struct DataflowEndpoint {
        /// Property [`Address`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-dataflowendpointgroup-dataflowendpoint.html#cfn-groundstation-dataflowendpointgroup-dataflowendpoint-address).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub address: Option<::Value<SocketAddress>>,
        /// Property [`Mtu`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-dataflowendpointgroup-dataflowendpoint.html#cfn-groundstation-dataflowendpointgroup-dataflowendpoint-mtu).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mtu: Option<::Value<u32>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-dataflowendpointgroup-dataflowendpoint.html#cfn-groundstation-dataflowendpointgroup-dataflowendpoint-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DataflowEndpoint {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref address) = self.address {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Address", address)?;
            }
            if let Some(ref mtu) = self.mtu {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Mtu", mtu)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataflowEndpoint {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataflowEndpoint, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataflowEndpoint;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataflowEndpoint")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut address: Option<::Value<SocketAddress>> = None;
                    let mut mtu: Option<::Value<u32>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Address" => {
                                address = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Mtu" => {
                                mtu = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataflowEndpoint {
                        address: address,
                        mtu: mtu,
                        name: name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::GroundStation::DataflowEndpointGroup.EndpointDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-dataflowendpointgroup-endpointdetails.html) property type.
    #[derive(Debug, Default)]
    pub struct EndpointDetails {
        /// Property [`Endpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-dataflowendpointgroup-endpointdetails.html#cfn-groundstation-dataflowendpointgroup-endpointdetails-endpoint).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub endpoint: Option<::Value<DataflowEndpoint>>,
        /// Property [`SecurityDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-dataflowendpointgroup-endpointdetails.html#cfn-groundstation-dataflowendpointgroup-endpointdetails-securitydetails).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_details: Option<::Value<SecurityDetails>>,
    }

    impl ::codec::SerializeValue for EndpointDetails {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref endpoint) = self.endpoint {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Endpoint", endpoint)?;
            }
            if let Some(ref security_details) = self.security_details {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityDetails", security_details)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EndpointDetails {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EndpointDetails, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EndpointDetails;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EndpointDetails")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut endpoint: Option<::Value<DataflowEndpoint>> = None;
                    let mut security_details: Option<::Value<SecurityDetails>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Endpoint" => {
                                endpoint = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecurityDetails" => {
                                security_details = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EndpointDetails {
                        endpoint: endpoint,
                        security_details: security_details,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::GroundStation::DataflowEndpointGroup.SecurityDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-dataflowendpointgroup-securitydetails.html) property type.
    #[derive(Debug, Default)]
    pub struct SecurityDetails {
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-dataflowendpointgroup-securitydetails.html#cfn-groundstation-dataflowendpointgroup-securitydetails-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: Option<::Value<String>>,
        /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-dataflowendpointgroup-securitydetails.html#cfn-groundstation-dataflowendpointgroup-securitydetails-securitygroupids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_group_ids: Option<::ValueList<String>>,
        /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-dataflowendpointgroup-securitydetails.html#cfn-groundstation-dataflowendpointgroup-securitydetails-subnetids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subnet_ids: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for SecurityDetails {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref role_arn) = self.role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", role_arn)?;
            }
            if let Some(ref security_group_ids) = self.security_group_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", security_group_ids)?;
            }
            if let Some(ref subnet_ids) = self.subnet_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", subnet_ids)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SecurityDetails {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SecurityDetails, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SecurityDetails;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SecurityDetails")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut security_group_ids: Option<::ValueList<String>> = None;
                    let mut subnet_ids: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecurityGroupIds" => {
                                security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubnetIds" => {
                                subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SecurityDetails {
                        role_arn: role_arn,
                        security_group_ids: security_group_ids,
                        subnet_ids: subnet_ids,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::GroundStation::DataflowEndpointGroup.SocketAddress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-dataflowendpointgroup-socketaddress.html) property type.
    #[derive(Debug, Default)]
    pub struct SocketAddress {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-dataflowendpointgroup-socketaddress.html#cfn-groundstation-dataflowendpointgroup-socketaddress-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-dataflowendpointgroup-socketaddress.html#cfn-groundstation-dataflowendpointgroup-socketaddress-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for SocketAddress {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref port) = self.port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SocketAddress {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SocketAddress, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SocketAddress;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SocketAddress")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut port: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SocketAddress {
                        name: name,
                        port: port,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod mission_profile {
    //! Property types for the `MissionProfile` resource.

    /// The [`AWS::GroundStation::MissionProfile.DataflowEdge`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-missionprofile-dataflowedge.html) property type.
    #[derive(Debug, Default)]
    pub struct DataflowEdge {
        /// Property [`Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-missionprofile-dataflowedge.html#cfn-groundstation-missionprofile-dataflowedge-destination).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination: Option<::Value<String>>,
        /// Property [`Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-groundstation-missionprofile-dataflowedge.html#cfn-groundstation-missionprofile-dataflowedge-source).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DataflowEdge {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref destination) = self.destination {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Destination", destination)?;
            }
            if let Some(ref source) = self.source {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Source", source)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataflowEdge {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataflowEdge, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataflowEdge;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataflowEdge")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destination: Option<::Value<String>> = None;
                    let mut source: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Destination" => {
                                destination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Source" => {
                                source = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataflowEdge {
                        destination: destination,
                        source: source,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
