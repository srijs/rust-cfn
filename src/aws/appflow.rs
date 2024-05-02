//! Types for the `AppFlow` service.

/// The [`AWS::AppFlow::Connector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appflow-connector.html) resource type.
#[derive(Debug, Default)]
pub struct Connector {
    properties: ConnectorProperties
}

/// Properties for the `Connector` resource.
#[derive(Debug, Default)]
pub struct ConnectorProperties {
    /// Property [`ConnectorLabel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appflow-connector.html#cfn-appflow-connector-connectorlabel).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub connector_label: Option<::Value<String>>,
    /// Property [`ConnectorProvisioningConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appflow-connector.html#cfn-appflow-connector-connectorprovisioningconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub connector_provisioning_config: ::Value<self::connector::ConnectorProvisioningConfig>,
    /// Property [`ConnectorProvisioningType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appflow-connector.html#cfn-appflow-connector-connectorprovisioningtype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub connector_provisioning_type: ::Value<String>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appflow-connector.html#cfn-appflow-connector-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
}

impl ::serde::Serialize for ConnectorProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref connector_label) = self.connector_label {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectorLabel", connector_label)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectorProvisioningConfig", &self.connector_provisioning_config)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectorProvisioningType", &self.connector_provisioning_type)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ConnectorProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ConnectorProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ConnectorProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ConnectorProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut connector_label: Option<::Value<String>> = None;
                let mut connector_provisioning_config: Option<::Value<self::connector::ConnectorProvisioningConfig>> = None;
                let mut connector_provisioning_type: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ConnectorLabel" => {
                            connector_label = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ConnectorProvisioningConfig" => {
                            connector_provisioning_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ConnectorProvisioningType" => {
                            connector_provisioning_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ConnectorProperties {
                    connector_label: connector_label,
                    connector_provisioning_config: connector_provisioning_config.ok_or(::serde::de::Error::missing_field("ConnectorProvisioningConfig"))?,
                    connector_provisioning_type: connector_provisioning_type.ok_or(::serde::de::Error::missing_field("ConnectorProvisioningType"))?,
                    description: description,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Connector {
    type Properties = ConnectorProperties;
    const TYPE: &'static str = "AWS::AppFlow::Connector";
    fn properties(&self) -> &ConnectorProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ConnectorProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Connector {}

impl From<ConnectorProperties> for Connector {
    fn from(properties: ConnectorProperties) -> Connector {
        Connector { properties }
    }
}

/// The [`AWS::AppFlow::ConnectorProfile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appflow-connectorprofile.html) resource type.
#[derive(Debug, Default)]
pub struct ConnectorProfile {
    properties: ConnectorProfileProperties
}

/// Properties for the `ConnectorProfile` resource.
#[derive(Debug, Default)]
pub struct ConnectorProfileProperties {
    /// Property [`ConnectionMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appflow-connectorprofile.html#cfn-appflow-connectorprofile-connectionmode).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub connection_mode: ::Value<String>,
    /// Property [`ConnectorLabel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appflow-connectorprofile.html#cfn-appflow-connectorprofile-connectorlabel).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub connector_label: Option<::Value<String>>,
    /// Property [`ConnectorProfileConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appflow-connectorprofile.html#cfn-appflow-connectorprofile-connectorprofileconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub connector_profile_config: Option<::Value<self::connector_profile::ConnectorProfileConfig>>,
    /// Property [`ConnectorProfileName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appflow-connectorprofile.html#cfn-appflow-connectorprofile-connectorprofilename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub connector_profile_name: ::Value<String>,
    /// Property [`ConnectorType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appflow-connectorprofile.html#cfn-appflow-connectorprofile-connectortype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub connector_type: ::Value<String>,
    /// Property [`KMSArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appflow-connectorprofile.html#cfn-appflow-connectorprofile-kmsarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub kms_arn: Option<::Value<String>>,
}

impl ::serde::Serialize for ConnectorProfileProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectionMode", &self.connection_mode)?;
        if let Some(ref connector_label) = self.connector_label {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectorLabel", connector_label)?;
        }
        if let Some(ref connector_profile_config) = self.connector_profile_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectorProfileConfig", connector_profile_config)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectorProfileName", &self.connector_profile_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectorType", &self.connector_type)?;
        if let Some(ref kms_arn) = self.kms_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KMSArn", kms_arn)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ConnectorProfileProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ConnectorProfileProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ConnectorProfileProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ConnectorProfileProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut connection_mode: Option<::Value<String>> = None;
                let mut connector_label: Option<::Value<String>> = None;
                let mut connector_profile_config: Option<::Value<self::connector_profile::ConnectorProfileConfig>> = None;
                let mut connector_profile_name: Option<::Value<String>> = None;
                let mut connector_type: Option<::Value<String>> = None;
                let mut kms_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ConnectionMode" => {
                            connection_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ConnectorLabel" => {
                            connector_label = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ConnectorProfileConfig" => {
                            connector_profile_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ConnectorProfileName" => {
                            connector_profile_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ConnectorType" => {
                            connector_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KMSArn" => {
                            kms_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ConnectorProfileProperties {
                    connection_mode: connection_mode.ok_or(::serde::de::Error::missing_field("ConnectionMode"))?,
                    connector_label: connector_label,
                    connector_profile_config: connector_profile_config,
                    connector_profile_name: connector_profile_name.ok_or(::serde::de::Error::missing_field("ConnectorProfileName"))?,
                    connector_type: connector_type.ok_or(::serde::de::Error::missing_field("ConnectorType"))?,
                    kms_arn: kms_arn,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ConnectorProfile {
    type Properties = ConnectorProfileProperties;
    const TYPE: &'static str = "AWS::AppFlow::ConnectorProfile";
    fn properties(&self) -> &ConnectorProfileProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ConnectorProfileProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ConnectorProfile {}

impl From<ConnectorProfileProperties> for ConnectorProfile {
    fn from(properties: ConnectorProfileProperties) -> ConnectorProfile {
        ConnectorProfile { properties }
    }
}

/// The [`AWS::AppFlow::Flow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appflow-flow.html) resource type.
#[derive(Debug, Default)]
pub struct Flow {
    properties: FlowProperties
}

/// Properties for the `Flow` resource.
#[derive(Debug, Default)]
pub struct FlowProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appflow-flow.html#cfn-appflow-flow-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`DestinationFlowConfigList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appflow-flow.html#cfn-appflow-flow-destinationflowconfiglist).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub destination_flow_config_list: ::ValueList<self::flow::DestinationFlowConfig>,
    /// Property [`FlowName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appflow-flow.html#cfn-appflow-flow-flowname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub flow_name: ::Value<String>,
    /// Property [`FlowStatus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appflow-flow.html#cfn-appflow-flow-flowstatus).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub flow_status: Option<::Value<String>>,
    /// Property [`KMSArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appflow-flow.html#cfn-appflow-flow-kmsarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kms_arn: Option<::Value<String>>,
    /// Property [`MetadataCatalogConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appflow-flow.html#cfn-appflow-flow-metadatacatalogconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub metadata_catalog_config: Option<::Value<self::flow::MetadataCatalogConfig>>,
    /// Property [`SourceFlowConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appflow-flow.html#cfn-appflow-flow-sourceflowconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub source_flow_config: ::Value<self::flow::SourceFlowConfig>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appflow-flow.html#cfn-appflow-flow-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Tasks`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appflow-flow.html#cfn-appflow-flow-tasks).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tasks: ::ValueList<self::flow::Task>,
    /// Property [`TriggerConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appflow-flow.html#cfn-appflow-flow-triggerconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub trigger_config: ::Value<self::flow::TriggerConfig>,
}

impl ::serde::Serialize for FlowProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationFlowConfigList", &self.destination_flow_config_list)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FlowName", &self.flow_name)?;
        if let Some(ref flow_status) = self.flow_status {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FlowStatus", flow_status)?;
        }
        if let Some(ref kms_arn) = self.kms_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KMSArn", kms_arn)?;
        }
        if let Some(ref metadata_catalog_config) = self.metadata_catalog_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetadataCatalogConfig", metadata_catalog_config)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceFlowConfig", &self.source_flow_config)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tasks", &self.tasks)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TriggerConfig", &self.trigger_config)?;
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
                let mut description: Option<::Value<String>> = None;
                let mut destination_flow_config_list: Option<::ValueList<self::flow::DestinationFlowConfig>> = None;
                let mut flow_name: Option<::Value<String>> = None;
                let mut flow_status: Option<::Value<String>> = None;
                let mut kms_arn: Option<::Value<String>> = None;
                let mut metadata_catalog_config: Option<::Value<self::flow::MetadataCatalogConfig>> = None;
                let mut source_flow_config: Option<::Value<self::flow::SourceFlowConfig>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut tasks: Option<::ValueList<self::flow::Task>> = None;
                let mut trigger_config: Option<::Value<self::flow::TriggerConfig>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DestinationFlowConfigList" => {
                            destination_flow_config_list = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FlowName" => {
                            flow_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FlowStatus" => {
                            flow_status = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KMSArn" => {
                            kms_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MetadataCatalogConfig" => {
                            metadata_catalog_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceFlowConfig" => {
                            source_flow_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tasks" => {
                            tasks = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TriggerConfig" => {
                            trigger_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(FlowProperties {
                    description: description,
                    destination_flow_config_list: destination_flow_config_list.ok_or(::serde::de::Error::missing_field("DestinationFlowConfigList"))?,
                    flow_name: flow_name.ok_or(::serde::de::Error::missing_field("FlowName"))?,
                    flow_status: flow_status,
                    kms_arn: kms_arn,
                    metadata_catalog_config: metadata_catalog_config,
                    source_flow_config: source_flow_config.ok_or(::serde::de::Error::missing_field("SourceFlowConfig"))?,
                    tags: tags,
                    tasks: tasks.ok_or(::serde::de::Error::missing_field("Tasks"))?,
                    trigger_config: trigger_config.ok_or(::serde::de::Error::missing_field("TriggerConfig"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Flow {
    type Properties = FlowProperties;
    const TYPE: &'static str = "AWS::AppFlow::Flow";
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

pub mod connector {
    //! Property types for the `Connector` resource.

    /// The [`AWS::AppFlow::Connector.ConnectorProvisioningConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connector-connectorprovisioningconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ConnectorProvisioningConfig {
        /// Property [`Lambda`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connector-connectorprovisioningconfig.html#cfn-appflow-connector-connectorprovisioningconfig-lambda).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lambda: Option<::Value<LambdaConnectorProvisioningConfig>>,
    }

    impl ::codec::SerializeValue for ConnectorProvisioningConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref lambda) = self.lambda {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Lambda", lambda)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConnectorProvisioningConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConnectorProvisioningConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConnectorProvisioningConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConnectorProvisioningConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut lambda: Option<::Value<LambdaConnectorProvisioningConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Lambda" => {
                                lambda = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConnectorProvisioningConfig {
                        lambda: lambda,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Connector.LambdaConnectorProvisioningConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connector-lambdaconnectorprovisioningconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct LambdaConnectorProvisioningConfig {
        /// Property [`LambdaArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connector-lambdaconnectorprovisioningconfig.html#cfn-appflow-connector-lambdaconnectorprovisioningconfig-lambdaarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lambda_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for LambdaConnectorProvisioningConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaArn", &self.lambda_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LambdaConnectorProvisioningConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LambdaConnectorProvisioningConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LambdaConnectorProvisioningConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LambdaConnectorProvisioningConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut lambda_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LambdaArn" => {
                                lambda_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LambdaConnectorProvisioningConfig {
                        lambda_arn: lambda_arn.ok_or(::serde::de::Error::missing_field("LambdaArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod connector_profile {
    //! Property types for the `ConnectorProfile` resource.

    /// The [`AWS::AppFlow::ConnectorProfile.AmplitudeConnectorProfileCredentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-amplitudeconnectorprofilecredentials.html) property type.
    #[derive(Debug, Default)]
    pub struct AmplitudeConnectorProfileCredentials {
        /// Property [`ApiKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-amplitudeconnectorprofilecredentials.html#cfn-appflow-connectorprofile-amplitudeconnectorprofilecredentials-apikey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub api_key: ::Value<String>,
        /// Property [`SecretKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-amplitudeconnectorprofilecredentials.html#cfn-appflow-connectorprofile-amplitudeconnectorprofilecredentials-secretkey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secret_key: ::Value<String>,
    }

    impl ::codec::SerializeValue for AmplitudeConnectorProfileCredentials {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiKey", &self.api_key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretKey", &self.secret_key)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AmplitudeConnectorProfileCredentials {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AmplitudeConnectorProfileCredentials, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AmplitudeConnectorProfileCredentials;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AmplitudeConnectorProfileCredentials")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut api_key: Option<::Value<String>> = None;
                    let mut secret_key: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ApiKey" => {
                                api_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretKey" => {
                                secret_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AmplitudeConnectorProfileCredentials {
                        api_key: api_key.ok_or(::serde::de::Error::missing_field("ApiKey"))?,
                        secret_key: secret_key.ok_or(::serde::de::Error::missing_field("SecretKey"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.ApiKeyCredentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-apikeycredentials.html) property type.
    #[derive(Debug, Default)]
    pub struct ApiKeyCredentials {
        /// Property [`ApiKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-apikeycredentials.html#cfn-appflow-connectorprofile-apikeycredentials-apikey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub api_key: ::Value<String>,
        /// Property [`ApiSecretKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-apikeycredentials.html#cfn-appflow-connectorprofile-apikeycredentials-apisecretkey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub api_secret_key: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ApiKeyCredentials {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiKey", &self.api_key)?;
            if let Some(ref api_secret_key) = self.api_secret_key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiSecretKey", api_secret_key)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ApiKeyCredentials {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ApiKeyCredentials, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ApiKeyCredentials;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ApiKeyCredentials")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut api_key: Option<::Value<String>> = None;
                    let mut api_secret_key: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ApiKey" => {
                                api_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ApiSecretKey" => {
                                api_secret_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ApiKeyCredentials {
                        api_key: api_key.ok_or(::serde::de::Error::missing_field("ApiKey"))?,
                        api_secret_key: api_secret_key,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.BasicAuthCredentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-basicauthcredentials.html) property type.
    #[derive(Debug, Default)]
    pub struct BasicAuthCredentials {
        /// Property [`Password`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-basicauthcredentials.html#cfn-appflow-connectorprofile-basicauthcredentials-password).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub password: ::Value<String>,
        /// Property [`Username`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-basicauthcredentials.html#cfn-appflow-connectorprofile-basicauthcredentials-username).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub username: ::Value<String>,
    }

    impl ::codec::SerializeValue for BasicAuthCredentials {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Password", &self.password)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Username", &self.username)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BasicAuthCredentials {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BasicAuthCredentials, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BasicAuthCredentials;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BasicAuthCredentials")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut password: Option<::Value<String>> = None;
                    let mut username: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Password" => {
                                password = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Username" => {
                                username = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BasicAuthCredentials {
                        password: password.ok_or(::serde::de::Error::missing_field("Password"))?,
                        username: username.ok_or(::serde::de::Error::missing_field("Username"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.ConnectorOAuthRequest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectoroauthrequest.html) property type.
    #[derive(Debug, Default)]
    pub struct ConnectorOAuthRequest {
        /// Property [`AuthCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectoroauthrequest.html#cfn-appflow-connectorprofile-connectoroauthrequest-authcode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub auth_code: Option<::Value<String>>,
        /// Property [`RedirectUri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectoroauthrequest.html#cfn-appflow-connectorprofile-connectoroauthrequest-redirecturi).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub redirect_uri: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ConnectorOAuthRequest {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref auth_code) = self.auth_code {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthCode", auth_code)?;
            }
            if let Some(ref redirect_uri) = self.redirect_uri {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RedirectUri", redirect_uri)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConnectorOAuthRequest {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConnectorOAuthRequest, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConnectorOAuthRequest;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConnectorOAuthRequest")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut auth_code: Option<::Value<String>> = None;
                    let mut redirect_uri: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AuthCode" => {
                                auth_code = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RedirectUri" => {
                                redirect_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConnectorOAuthRequest {
                        auth_code: auth_code,
                        redirect_uri: redirect_uri,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.ConnectorProfileConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectorprofileconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ConnectorProfileConfig {
        /// Property [`ConnectorProfileCredentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectorprofileconfig.html#cfn-appflow-connectorprofile-connectorprofileconfig-connectorprofilecredentials).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connector_profile_credentials: Option<::Value<ConnectorProfileCredentials>>,
        /// Property [`ConnectorProfileProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectorprofileconfig.html#cfn-appflow-connectorprofile-connectorprofileconfig-connectorprofileproperties).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connector_profile_properties: Option<::Value<ConnectorProfileProperties>>,
    }

    impl ::codec::SerializeValue for ConnectorProfileConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref connector_profile_credentials) = self.connector_profile_credentials {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectorProfileCredentials", connector_profile_credentials)?;
            }
            if let Some(ref connector_profile_properties) = self.connector_profile_properties {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectorProfileProperties", connector_profile_properties)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConnectorProfileConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConnectorProfileConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConnectorProfileConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConnectorProfileConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut connector_profile_credentials: Option<::Value<ConnectorProfileCredentials>> = None;
                    let mut connector_profile_properties: Option<::Value<ConnectorProfileProperties>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConnectorProfileCredentials" => {
                                connector_profile_credentials = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConnectorProfileProperties" => {
                                connector_profile_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConnectorProfileConfig {
                        connector_profile_credentials: connector_profile_credentials,
                        connector_profile_properties: connector_profile_properties,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.ConnectorProfileCredentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectorprofilecredentials.html) property type.
    #[derive(Debug, Default)]
    pub struct ConnectorProfileCredentials {
        /// Property [`Amplitude`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectorprofilecredentials.html#cfn-appflow-connectorprofile-connectorprofilecredentials-amplitude).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub amplitude: Option<::Value<AmplitudeConnectorProfileCredentials>>,
        /// Property [`CustomConnector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectorprofilecredentials.html#cfn-appflow-connectorprofile-connectorprofilecredentials-customconnector).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_connector: Option<::Value<CustomConnectorProfileCredentials>>,
        /// Property [`Datadog`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectorprofilecredentials.html#cfn-appflow-connectorprofile-connectorprofilecredentials-datadog).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub datadog: Option<::Value<DatadogConnectorProfileCredentials>>,
        /// Property [`Dynatrace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectorprofilecredentials.html#cfn-appflow-connectorprofile-connectorprofilecredentials-dynatrace).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dynatrace: Option<::Value<DynatraceConnectorProfileCredentials>>,
        /// Property [`GoogleAnalytics`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectorprofilecredentials.html#cfn-appflow-connectorprofile-connectorprofilecredentials-googleanalytics).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub google_analytics: Option<::Value<GoogleAnalyticsConnectorProfileCredentials>>,
        /// Property [`InforNexus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectorprofilecredentials.html#cfn-appflow-connectorprofile-connectorprofilecredentials-infornexus).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub infor_nexus: Option<::Value<InforNexusConnectorProfileCredentials>>,
        /// Property [`Marketo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectorprofilecredentials.html#cfn-appflow-connectorprofile-connectorprofilecredentials-marketo).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub marketo: Option<::Value<MarketoConnectorProfileCredentials>>,
        /// Property [`Pardot`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectorprofilecredentials.html#cfn-appflow-connectorprofile-connectorprofilecredentials-pardot).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pardot: Option<::Value<PardotConnectorProfileCredentials>>,
        /// Property [`Redshift`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectorprofilecredentials.html#cfn-appflow-connectorprofile-connectorprofilecredentials-redshift).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub redshift: Option<::Value<RedshiftConnectorProfileCredentials>>,
        /// Property [`SAPOData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectorprofilecredentials.html#cfn-appflow-connectorprofile-connectorprofilecredentials-sapodata).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sapo_data: Option<::Value<SAPODataConnectorProfileCredentials>>,
        /// Property [`Salesforce`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectorprofilecredentials.html#cfn-appflow-connectorprofile-connectorprofilecredentials-salesforce).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub salesforce: Option<::Value<SalesforceConnectorProfileCredentials>>,
        /// Property [`ServiceNow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectorprofilecredentials.html#cfn-appflow-connectorprofile-connectorprofilecredentials-servicenow).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub service_now: Option<::Value<ServiceNowConnectorProfileCredentials>>,
        /// Property [`Singular`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectorprofilecredentials.html#cfn-appflow-connectorprofile-connectorprofilecredentials-singular).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub singular: Option<::Value<SingularConnectorProfileCredentials>>,
        /// Property [`Slack`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectorprofilecredentials.html#cfn-appflow-connectorprofile-connectorprofilecredentials-slack).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub slack: Option<::Value<SlackConnectorProfileCredentials>>,
        /// Property [`Snowflake`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectorprofilecredentials.html#cfn-appflow-connectorprofile-connectorprofilecredentials-snowflake).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub snowflake: Option<::Value<SnowflakeConnectorProfileCredentials>>,
        /// Property [`Trendmicro`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectorprofilecredentials.html#cfn-appflow-connectorprofile-connectorprofilecredentials-trendmicro).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub trendmicro: Option<::Value<TrendmicroConnectorProfileCredentials>>,
        /// Property [`Veeva`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectorprofilecredentials.html#cfn-appflow-connectorprofile-connectorprofilecredentials-veeva).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub veeva: Option<::Value<VeevaConnectorProfileCredentials>>,
        /// Property [`Zendesk`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectorprofilecredentials.html#cfn-appflow-connectorprofile-connectorprofilecredentials-zendesk).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub zendesk: Option<::Value<ZendeskConnectorProfileCredentials>>,
    }

    impl ::codec::SerializeValue for ConnectorProfileCredentials {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref amplitude) = self.amplitude {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Amplitude", amplitude)?;
            }
            if let Some(ref custom_connector) = self.custom_connector {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomConnector", custom_connector)?;
            }
            if let Some(ref datadog) = self.datadog {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Datadog", datadog)?;
            }
            if let Some(ref dynatrace) = self.dynatrace {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Dynatrace", dynatrace)?;
            }
            if let Some(ref google_analytics) = self.google_analytics {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GoogleAnalytics", google_analytics)?;
            }
            if let Some(ref infor_nexus) = self.infor_nexus {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InforNexus", infor_nexus)?;
            }
            if let Some(ref marketo) = self.marketo {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Marketo", marketo)?;
            }
            if let Some(ref pardot) = self.pardot {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Pardot", pardot)?;
            }
            if let Some(ref redshift) = self.redshift {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Redshift", redshift)?;
            }
            if let Some(ref sapo_data) = self.sapo_data {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SAPOData", sapo_data)?;
            }
            if let Some(ref salesforce) = self.salesforce {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Salesforce", salesforce)?;
            }
            if let Some(ref service_now) = self.service_now {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceNow", service_now)?;
            }
            if let Some(ref singular) = self.singular {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Singular", singular)?;
            }
            if let Some(ref slack) = self.slack {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Slack", slack)?;
            }
            if let Some(ref snowflake) = self.snowflake {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Snowflake", snowflake)?;
            }
            if let Some(ref trendmicro) = self.trendmicro {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Trendmicro", trendmicro)?;
            }
            if let Some(ref veeva) = self.veeva {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Veeva", veeva)?;
            }
            if let Some(ref zendesk) = self.zendesk {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Zendesk", zendesk)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConnectorProfileCredentials {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConnectorProfileCredentials, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConnectorProfileCredentials;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConnectorProfileCredentials")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut amplitude: Option<::Value<AmplitudeConnectorProfileCredentials>> = None;
                    let mut custom_connector: Option<::Value<CustomConnectorProfileCredentials>> = None;
                    let mut datadog: Option<::Value<DatadogConnectorProfileCredentials>> = None;
                    let mut dynatrace: Option<::Value<DynatraceConnectorProfileCredentials>> = None;
                    let mut google_analytics: Option<::Value<GoogleAnalyticsConnectorProfileCredentials>> = None;
                    let mut infor_nexus: Option<::Value<InforNexusConnectorProfileCredentials>> = None;
                    let mut marketo: Option<::Value<MarketoConnectorProfileCredentials>> = None;
                    let mut pardot: Option<::Value<PardotConnectorProfileCredentials>> = None;
                    let mut redshift: Option<::Value<RedshiftConnectorProfileCredentials>> = None;
                    let mut sapo_data: Option<::Value<SAPODataConnectorProfileCredentials>> = None;
                    let mut salesforce: Option<::Value<SalesforceConnectorProfileCredentials>> = None;
                    let mut service_now: Option<::Value<ServiceNowConnectorProfileCredentials>> = None;
                    let mut singular: Option<::Value<SingularConnectorProfileCredentials>> = None;
                    let mut slack: Option<::Value<SlackConnectorProfileCredentials>> = None;
                    let mut snowflake: Option<::Value<SnowflakeConnectorProfileCredentials>> = None;
                    let mut trendmicro: Option<::Value<TrendmicroConnectorProfileCredentials>> = None;
                    let mut veeva: Option<::Value<VeevaConnectorProfileCredentials>> = None;
                    let mut zendesk: Option<::Value<ZendeskConnectorProfileCredentials>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Amplitude" => {
                                amplitude = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CustomConnector" => {
                                custom_connector = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Datadog" => {
                                datadog = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Dynatrace" => {
                                dynatrace = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GoogleAnalytics" => {
                                google_analytics = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InforNexus" => {
                                infor_nexus = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Marketo" => {
                                marketo = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Pardot" => {
                                pardot = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Redshift" => {
                                redshift = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SAPOData" => {
                                sapo_data = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Salesforce" => {
                                salesforce = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServiceNow" => {
                                service_now = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Singular" => {
                                singular = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Slack" => {
                                slack = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Snowflake" => {
                                snowflake = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Trendmicro" => {
                                trendmicro = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Veeva" => {
                                veeva = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Zendesk" => {
                                zendesk = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConnectorProfileCredentials {
                        amplitude: amplitude,
                        custom_connector: custom_connector,
                        datadog: datadog,
                        dynatrace: dynatrace,
                        google_analytics: google_analytics,
                        infor_nexus: infor_nexus,
                        marketo: marketo,
                        pardot: pardot,
                        redshift: redshift,
                        sapo_data: sapo_data,
                        salesforce: salesforce,
                        service_now: service_now,
                        singular: singular,
                        slack: slack,
                        snowflake: snowflake,
                        trendmicro: trendmicro,
                        veeva: veeva,
                        zendesk: zendesk,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.ConnectorProfileProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectorprofileproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct ConnectorProfileProperties {
        /// Property [`CustomConnector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectorprofileproperties.html#cfn-appflow-connectorprofile-connectorprofileproperties-customconnector).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_connector: Option<::Value<CustomConnectorProfileProperties>>,
        /// Property [`Datadog`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectorprofileproperties.html#cfn-appflow-connectorprofile-connectorprofileproperties-datadog).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub datadog: Option<::Value<DatadogConnectorProfileProperties>>,
        /// Property [`Dynatrace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectorprofileproperties.html#cfn-appflow-connectorprofile-connectorprofileproperties-dynatrace).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dynatrace: Option<::Value<DynatraceConnectorProfileProperties>>,
        /// Property [`InforNexus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectorprofileproperties.html#cfn-appflow-connectorprofile-connectorprofileproperties-infornexus).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub infor_nexus: Option<::Value<InforNexusConnectorProfileProperties>>,
        /// Property [`Marketo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectorprofileproperties.html#cfn-appflow-connectorprofile-connectorprofileproperties-marketo).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub marketo: Option<::Value<MarketoConnectorProfileProperties>>,
        /// Property [`Pardot`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectorprofileproperties.html#cfn-appflow-connectorprofile-connectorprofileproperties-pardot).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pardot: Option<::Value<PardotConnectorProfileProperties>>,
        /// Property [`Redshift`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectorprofileproperties.html#cfn-appflow-connectorprofile-connectorprofileproperties-redshift).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub redshift: Option<::Value<RedshiftConnectorProfileProperties>>,
        /// Property [`SAPOData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectorprofileproperties.html#cfn-appflow-connectorprofile-connectorprofileproperties-sapodata).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sapo_data: Option<::Value<SAPODataConnectorProfileProperties>>,
        /// Property [`Salesforce`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectorprofileproperties.html#cfn-appflow-connectorprofile-connectorprofileproperties-salesforce).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub salesforce: Option<::Value<SalesforceConnectorProfileProperties>>,
        /// Property [`ServiceNow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectorprofileproperties.html#cfn-appflow-connectorprofile-connectorprofileproperties-servicenow).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub service_now: Option<::Value<ServiceNowConnectorProfileProperties>>,
        /// Property [`Slack`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectorprofileproperties.html#cfn-appflow-connectorprofile-connectorprofileproperties-slack).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub slack: Option<::Value<SlackConnectorProfileProperties>>,
        /// Property [`Snowflake`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectorprofileproperties.html#cfn-appflow-connectorprofile-connectorprofileproperties-snowflake).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub snowflake: Option<::Value<SnowflakeConnectorProfileProperties>>,
        /// Property [`Veeva`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectorprofileproperties.html#cfn-appflow-connectorprofile-connectorprofileproperties-veeva).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub veeva: Option<::Value<VeevaConnectorProfileProperties>>,
        /// Property [`Zendesk`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-connectorprofileproperties.html#cfn-appflow-connectorprofile-connectorprofileproperties-zendesk).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub zendesk: Option<::Value<ZendeskConnectorProfileProperties>>,
    }

    impl ::codec::SerializeValue for ConnectorProfileProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref custom_connector) = self.custom_connector {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomConnector", custom_connector)?;
            }
            if let Some(ref datadog) = self.datadog {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Datadog", datadog)?;
            }
            if let Some(ref dynatrace) = self.dynatrace {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Dynatrace", dynatrace)?;
            }
            if let Some(ref infor_nexus) = self.infor_nexus {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InforNexus", infor_nexus)?;
            }
            if let Some(ref marketo) = self.marketo {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Marketo", marketo)?;
            }
            if let Some(ref pardot) = self.pardot {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Pardot", pardot)?;
            }
            if let Some(ref redshift) = self.redshift {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Redshift", redshift)?;
            }
            if let Some(ref sapo_data) = self.sapo_data {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SAPOData", sapo_data)?;
            }
            if let Some(ref salesforce) = self.salesforce {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Salesforce", salesforce)?;
            }
            if let Some(ref service_now) = self.service_now {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceNow", service_now)?;
            }
            if let Some(ref slack) = self.slack {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Slack", slack)?;
            }
            if let Some(ref snowflake) = self.snowflake {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Snowflake", snowflake)?;
            }
            if let Some(ref veeva) = self.veeva {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Veeva", veeva)?;
            }
            if let Some(ref zendesk) = self.zendesk {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Zendesk", zendesk)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConnectorProfileProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConnectorProfileProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConnectorProfileProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConnectorProfileProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut custom_connector: Option<::Value<CustomConnectorProfileProperties>> = None;
                    let mut datadog: Option<::Value<DatadogConnectorProfileProperties>> = None;
                    let mut dynatrace: Option<::Value<DynatraceConnectorProfileProperties>> = None;
                    let mut infor_nexus: Option<::Value<InforNexusConnectorProfileProperties>> = None;
                    let mut marketo: Option<::Value<MarketoConnectorProfileProperties>> = None;
                    let mut pardot: Option<::Value<PardotConnectorProfileProperties>> = None;
                    let mut redshift: Option<::Value<RedshiftConnectorProfileProperties>> = None;
                    let mut sapo_data: Option<::Value<SAPODataConnectorProfileProperties>> = None;
                    let mut salesforce: Option<::Value<SalesforceConnectorProfileProperties>> = None;
                    let mut service_now: Option<::Value<ServiceNowConnectorProfileProperties>> = None;
                    let mut slack: Option<::Value<SlackConnectorProfileProperties>> = None;
                    let mut snowflake: Option<::Value<SnowflakeConnectorProfileProperties>> = None;
                    let mut veeva: Option<::Value<VeevaConnectorProfileProperties>> = None;
                    let mut zendesk: Option<::Value<ZendeskConnectorProfileProperties>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CustomConnector" => {
                                custom_connector = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Datadog" => {
                                datadog = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Dynatrace" => {
                                dynatrace = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InforNexus" => {
                                infor_nexus = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Marketo" => {
                                marketo = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Pardot" => {
                                pardot = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Redshift" => {
                                redshift = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SAPOData" => {
                                sapo_data = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Salesforce" => {
                                salesforce = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServiceNow" => {
                                service_now = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Slack" => {
                                slack = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Snowflake" => {
                                snowflake = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Veeva" => {
                                veeva = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Zendesk" => {
                                zendesk = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConnectorProfileProperties {
                        custom_connector: custom_connector,
                        datadog: datadog,
                        dynatrace: dynatrace,
                        infor_nexus: infor_nexus,
                        marketo: marketo,
                        pardot: pardot,
                        redshift: redshift,
                        sapo_data: sapo_data,
                        salesforce: salesforce,
                        service_now: service_now,
                        slack: slack,
                        snowflake: snowflake,
                        veeva: veeva,
                        zendesk: zendesk,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.CustomAuthCredentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-customauthcredentials.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomAuthCredentials {
        /// Property [`CredentialsMap`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-customauthcredentials.html#cfn-appflow-connectorprofile-customauthcredentials-credentialsmap).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub credentials_map: Option<::ValueMap<String>>,
        /// Property [`CustomAuthenticationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-customauthcredentials.html#cfn-appflow-connectorprofile-customauthcredentials-customauthenticationtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_authentication_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for CustomAuthCredentials {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref credentials_map) = self.credentials_map {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CredentialsMap", credentials_map)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomAuthenticationType", &self.custom_authentication_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomAuthCredentials {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomAuthCredentials, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomAuthCredentials;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomAuthCredentials")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut credentials_map: Option<::ValueMap<String>> = None;
                    let mut custom_authentication_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CredentialsMap" => {
                                credentials_map = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CustomAuthenticationType" => {
                                custom_authentication_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomAuthCredentials {
                        credentials_map: credentials_map,
                        custom_authentication_type: custom_authentication_type.ok_or(::serde::de::Error::missing_field("CustomAuthenticationType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.CustomConnectorProfileCredentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-customconnectorprofilecredentials.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomConnectorProfileCredentials {
        /// Property [`ApiKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-customconnectorprofilecredentials.html#cfn-appflow-connectorprofile-customconnectorprofilecredentials-apikey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub api_key: Option<::Value<ApiKeyCredentials>>,
        /// Property [`AuthenticationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-customconnectorprofilecredentials.html#cfn-appflow-connectorprofile-customconnectorprofilecredentials-authenticationtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub authentication_type: ::Value<String>,
        /// Property [`Basic`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-customconnectorprofilecredentials.html#cfn-appflow-connectorprofile-customconnectorprofilecredentials-basic).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub basic: Option<::Value<BasicAuthCredentials>>,
        /// Property [`Custom`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-customconnectorprofilecredentials.html#cfn-appflow-connectorprofile-customconnectorprofilecredentials-custom).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom: Option<::Value<CustomAuthCredentials>>,
        /// Property [`Oauth2`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-customconnectorprofilecredentials.html#cfn-appflow-connectorprofile-customconnectorprofilecredentials-oauth2).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub oauth2: Option<::Value<OAuth2Credentials>>,
    }

    impl ::codec::SerializeValue for CustomConnectorProfileCredentials {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref api_key) = self.api_key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiKey", api_key)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthenticationType", &self.authentication_type)?;
            if let Some(ref basic) = self.basic {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Basic", basic)?;
            }
            if let Some(ref custom) = self.custom {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Custom", custom)?;
            }
            if let Some(ref oauth2) = self.oauth2 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Oauth2", oauth2)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomConnectorProfileCredentials {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomConnectorProfileCredentials, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomConnectorProfileCredentials;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomConnectorProfileCredentials")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut api_key: Option<::Value<ApiKeyCredentials>> = None;
                    let mut authentication_type: Option<::Value<String>> = None;
                    let mut basic: Option<::Value<BasicAuthCredentials>> = None;
                    let mut custom: Option<::Value<CustomAuthCredentials>> = None;
                    let mut oauth2: Option<::Value<OAuth2Credentials>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ApiKey" => {
                                api_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AuthenticationType" => {
                                authentication_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Basic" => {
                                basic = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Custom" => {
                                custom = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Oauth2" => {
                                oauth2 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomConnectorProfileCredentials {
                        api_key: api_key,
                        authentication_type: authentication_type.ok_or(::serde::de::Error::missing_field("AuthenticationType"))?,
                        basic: basic,
                        custom: custom,
                        oauth2: oauth2,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.CustomConnectorProfileProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-customconnectorprofileproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomConnectorProfileProperties {
        /// Property [`OAuth2Properties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-customconnectorprofileproperties.html#cfn-appflow-connectorprofile-customconnectorprofileproperties-oauth2properties).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub o_auth2_properties: Option<::Value<OAuth2Properties>>,
        /// Property [`ProfileProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-customconnectorprofileproperties.html#cfn-appflow-connectorprofile-customconnectorprofileproperties-profileproperties).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub profile_properties: Option<::ValueMap<String>>,
    }

    impl ::codec::SerializeValue for CustomConnectorProfileProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref o_auth2_properties) = self.o_auth2_properties {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OAuth2Properties", o_auth2_properties)?;
            }
            if let Some(ref profile_properties) = self.profile_properties {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProfileProperties", profile_properties)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomConnectorProfileProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomConnectorProfileProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomConnectorProfileProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomConnectorProfileProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut o_auth2_properties: Option<::Value<OAuth2Properties>> = None;
                    let mut profile_properties: Option<::ValueMap<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "OAuth2Properties" => {
                                o_auth2_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProfileProperties" => {
                                profile_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomConnectorProfileProperties {
                        o_auth2_properties: o_auth2_properties,
                        profile_properties: profile_properties,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.DatadogConnectorProfileCredentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-datadogconnectorprofilecredentials.html) property type.
    #[derive(Debug, Default)]
    pub struct DatadogConnectorProfileCredentials {
        /// Property [`ApiKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-datadogconnectorprofilecredentials.html#cfn-appflow-connectorprofile-datadogconnectorprofilecredentials-apikey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub api_key: ::Value<String>,
        /// Property [`ApplicationKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-datadogconnectorprofilecredentials.html#cfn-appflow-connectorprofile-datadogconnectorprofilecredentials-applicationkey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub application_key: ::Value<String>,
    }

    impl ::codec::SerializeValue for DatadogConnectorProfileCredentials {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiKey", &self.api_key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationKey", &self.application_key)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DatadogConnectorProfileCredentials {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DatadogConnectorProfileCredentials, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DatadogConnectorProfileCredentials;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DatadogConnectorProfileCredentials")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut api_key: Option<::Value<String>> = None;
                    let mut application_key: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ApiKey" => {
                                api_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ApplicationKey" => {
                                application_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DatadogConnectorProfileCredentials {
                        api_key: api_key.ok_or(::serde::de::Error::missing_field("ApiKey"))?,
                        application_key: application_key.ok_or(::serde::de::Error::missing_field("ApplicationKey"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.DatadogConnectorProfileProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-datadogconnectorprofileproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct DatadogConnectorProfileProperties {
        /// Property [`InstanceUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-datadogconnectorprofileproperties.html#cfn-appflow-connectorprofile-datadogconnectorprofileproperties-instanceurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub instance_url: ::Value<String>,
    }

    impl ::codec::SerializeValue for DatadogConnectorProfileProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceUrl", &self.instance_url)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DatadogConnectorProfileProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DatadogConnectorProfileProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DatadogConnectorProfileProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DatadogConnectorProfileProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut instance_url: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InstanceUrl" => {
                                instance_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DatadogConnectorProfileProperties {
                        instance_url: instance_url.ok_or(::serde::de::Error::missing_field("InstanceUrl"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.DynatraceConnectorProfileCredentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-dynatraceconnectorprofilecredentials.html) property type.
    #[derive(Debug, Default)]
    pub struct DynatraceConnectorProfileCredentials {
        /// Property [`ApiToken`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-dynatraceconnectorprofilecredentials.html#cfn-appflow-connectorprofile-dynatraceconnectorprofilecredentials-apitoken).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub api_token: ::Value<String>,
    }

    impl ::codec::SerializeValue for DynatraceConnectorProfileCredentials {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiToken", &self.api_token)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DynatraceConnectorProfileCredentials {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DynatraceConnectorProfileCredentials, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DynatraceConnectorProfileCredentials;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DynatraceConnectorProfileCredentials")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut api_token: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ApiToken" => {
                                api_token = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DynatraceConnectorProfileCredentials {
                        api_token: api_token.ok_or(::serde::de::Error::missing_field("ApiToken"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.DynatraceConnectorProfileProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-dynatraceconnectorprofileproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct DynatraceConnectorProfileProperties {
        /// Property [`InstanceUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-dynatraceconnectorprofileproperties.html#cfn-appflow-connectorprofile-dynatraceconnectorprofileproperties-instanceurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub instance_url: ::Value<String>,
    }

    impl ::codec::SerializeValue for DynatraceConnectorProfileProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceUrl", &self.instance_url)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DynatraceConnectorProfileProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DynatraceConnectorProfileProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DynatraceConnectorProfileProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DynatraceConnectorProfileProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut instance_url: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InstanceUrl" => {
                                instance_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DynatraceConnectorProfileProperties {
                        instance_url: instance_url.ok_or(::serde::de::Error::missing_field("InstanceUrl"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.GoogleAnalyticsConnectorProfileCredentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-googleanalyticsconnectorprofilecredentials.html) property type.
    #[derive(Debug, Default)]
    pub struct GoogleAnalyticsConnectorProfileCredentials {
        /// Property [`AccessToken`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-googleanalyticsconnectorprofilecredentials.html#cfn-appflow-connectorprofile-googleanalyticsconnectorprofilecredentials-accesstoken).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub access_token: Option<::Value<String>>,
        /// Property [`ClientId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-googleanalyticsconnectorprofilecredentials.html#cfn-appflow-connectorprofile-googleanalyticsconnectorprofilecredentials-clientid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_id: ::Value<String>,
        /// Property [`ClientSecret`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-googleanalyticsconnectorprofilecredentials.html#cfn-appflow-connectorprofile-googleanalyticsconnectorprofilecredentials-clientsecret).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_secret: ::Value<String>,
        /// Property [`ConnectorOAuthRequest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-googleanalyticsconnectorprofilecredentials.html#cfn-appflow-connectorprofile-googleanalyticsconnectorprofilecredentials-connectoroauthrequest).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connector_o_auth_request: Option<::Value<ConnectorOAuthRequest>>,
        /// Property [`RefreshToken`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-googleanalyticsconnectorprofilecredentials.html#cfn-appflow-connectorprofile-googleanalyticsconnectorprofilecredentials-refreshtoken).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub refresh_token: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for GoogleAnalyticsConnectorProfileCredentials {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref access_token) = self.access_token {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessToken", access_token)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientId", &self.client_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientSecret", &self.client_secret)?;
            if let Some(ref connector_o_auth_request) = self.connector_o_auth_request {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectorOAuthRequest", connector_o_auth_request)?;
            }
            if let Some(ref refresh_token) = self.refresh_token {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RefreshToken", refresh_token)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GoogleAnalyticsConnectorProfileCredentials {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GoogleAnalyticsConnectorProfileCredentials, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GoogleAnalyticsConnectorProfileCredentials;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GoogleAnalyticsConnectorProfileCredentials")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut access_token: Option<::Value<String>> = None;
                    let mut client_id: Option<::Value<String>> = None;
                    let mut client_secret: Option<::Value<String>> = None;
                    let mut connector_o_auth_request: Option<::Value<ConnectorOAuthRequest>> = None;
                    let mut refresh_token: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AccessToken" => {
                                access_token = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ClientId" => {
                                client_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ClientSecret" => {
                                client_secret = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConnectorOAuthRequest" => {
                                connector_o_auth_request = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RefreshToken" => {
                                refresh_token = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GoogleAnalyticsConnectorProfileCredentials {
                        access_token: access_token,
                        client_id: client_id.ok_or(::serde::de::Error::missing_field("ClientId"))?,
                        client_secret: client_secret.ok_or(::serde::de::Error::missing_field("ClientSecret"))?,
                        connector_o_auth_request: connector_o_auth_request,
                        refresh_token: refresh_token,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.InforNexusConnectorProfileCredentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-infornexusconnectorprofilecredentials.html) property type.
    #[derive(Debug, Default)]
    pub struct InforNexusConnectorProfileCredentials {
        /// Property [`AccessKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-infornexusconnectorprofilecredentials.html#cfn-appflow-connectorprofile-infornexusconnectorprofilecredentials-accesskeyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub access_key_id: ::Value<String>,
        /// Property [`Datakey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-infornexusconnectorprofilecredentials.html#cfn-appflow-connectorprofile-infornexusconnectorprofilecredentials-datakey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub datakey: ::Value<String>,
        /// Property [`SecretAccessKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-infornexusconnectorprofilecredentials.html#cfn-appflow-connectorprofile-infornexusconnectorprofilecredentials-secretaccesskey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secret_access_key: ::Value<String>,
        /// Property [`UserId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-infornexusconnectorprofilecredentials.html#cfn-appflow-connectorprofile-infornexusconnectorprofilecredentials-userid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for InforNexusConnectorProfileCredentials {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessKeyId", &self.access_key_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Datakey", &self.datakey)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretAccessKey", &self.secret_access_key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserId", &self.user_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InforNexusConnectorProfileCredentials {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InforNexusConnectorProfileCredentials, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InforNexusConnectorProfileCredentials;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InforNexusConnectorProfileCredentials")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut access_key_id: Option<::Value<String>> = None;
                    let mut datakey: Option<::Value<String>> = None;
                    let mut secret_access_key: Option<::Value<String>> = None;
                    let mut user_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AccessKeyId" => {
                                access_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Datakey" => {
                                datakey = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretAccessKey" => {
                                secret_access_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserId" => {
                                user_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InforNexusConnectorProfileCredentials {
                        access_key_id: access_key_id.ok_or(::serde::de::Error::missing_field("AccessKeyId"))?,
                        datakey: datakey.ok_or(::serde::de::Error::missing_field("Datakey"))?,
                        secret_access_key: secret_access_key.ok_or(::serde::de::Error::missing_field("SecretAccessKey"))?,
                        user_id: user_id.ok_or(::serde::de::Error::missing_field("UserId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.InforNexusConnectorProfileProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-infornexusconnectorprofileproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct InforNexusConnectorProfileProperties {
        /// Property [`InstanceUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-infornexusconnectorprofileproperties.html#cfn-appflow-connectorprofile-infornexusconnectorprofileproperties-instanceurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub instance_url: ::Value<String>,
    }

    impl ::codec::SerializeValue for InforNexusConnectorProfileProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceUrl", &self.instance_url)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InforNexusConnectorProfileProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InforNexusConnectorProfileProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InforNexusConnectorProfileProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InforNexusConnectorProfileProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut instance_url: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InstanceUrl" => {
                                instance_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InforNexusConnectorProfileProperties {
                        instance_url: instance_url.ok_or(::serde::de::Error::missing_field("InstanceUrl"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.MarketoConnectorProfileCredentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-marketoconnectorprofilecredentials.html) property type.
    #[derive(Debug, Default)]
    pub struct MarketoConnectorProfileCredentials {
        /// Property [`AccessToken`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-marketoconnectorprofilecredentials.html#cfn-appflow-connectorprofile-marketoconnectorprofilecredentials-accesstoken).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub access_token: Option<::Value<String>>,
        /// Property [`ClientId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-marketoconnectorprofilecredentials.html#cfn-appflow-connectorprofile-marketoconnectorprofilecredentials-clientid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_id: ::Value<String>,
        /// Property [`ClientSecret`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-marketoconnectorprofilecredentials.html#cfn-appflow-connectorprofile-marketoconnectorprofilecredentials-clientsecret).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_secret: ::Value<String>,
        /// Property [`ConnectorOAuthRequest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-marketoconnectorprofilecredentials.html#cfn-appflow-connectorprofile-marketoconnectorprofilecredentials-connectoroauthrequest).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connector_o_auth_request: Option<::Value<ConnectorOAuthRequest>>,
    }

    impl ::codec::SerializeValue for MarketoConnectorProfileCredentials {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref access_token) = self.access_token {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessToken", access_token)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientId", &self.client_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientSecret", &self.client_secret)?;
            if let Some(ref connector_o_auth_request) = self.connector_o_auth_request {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectorOAuthRequest", connector_o_auth_request)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MarketoConnectorProfileCredentials {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MarketoConnectorProfileCredentials, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MarketoConnectorProfileCredentials;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MarketoConnectorProfileCredentials")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut access_token: Option<::Value<String>> = None;
                    let mut client_id: Option<::Value<String>> = None;
                    let mut client_secret: Option<::Value<String>> = None;
                    let mut connector_o_auth_request: Option<::Value<ConnectorOAuthRequest>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AccessToken" => {
                                access_token = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ClientId" => {
                                client_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ClientSecret" => {
                                client_secret = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConnectorOAuthRequest" => {
                                connector_o_auth_request = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MarketoConnectorProfileCredentials {
                        access_token: access_token,
                        client_id: client_id.ok_or(::serde::de::Error::missing_field("ClientId"))?,
                        client_secret: client_secret.ok_or(::serde::de::Error::missing_field("ClientSecret"))?,
                        connector_o_auth_request: connector_o_auth_request,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.MarketoConnectorProfileProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-marketoconnectorprofileproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct MarketoConnectorProfileProperties {
        /// Property [`InstanceUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-marketoconnectorprofileproperties.html#cfn-appflow-connectorprofile-marketoconnectorprofileproperties-instanceurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub instance_url: ::Value<String>,
    }

    impl ::codec::SerializeValue for MarketoConnectorProfileProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceUrl", &self.instance_url)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MarketoConnectorProfileProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MarketoConnectorProfileProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MarketoConnectorProfileProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MarketoConnectorProfileProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut instance_url: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InstanceUrl" => {
                                instance_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MarketoConnectorProfileProperties {
                        instance_url: instance_url.ok_or(::serde::de::Error::missing_field("InstanceUrl"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.OAuth2Credentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-oauth2credentials.html) property type.
    #[derive(Debug, Default)]
    pub struct OAuth2Credentials {
        /// Property [`AccessToken`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-oauth2credentials.html#cfn-appflow-connectorprofile-oauth2credentials-accesstoken).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub access_token: Option<::Value<String>>,
        /// Property [`ClientId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-oauth2credentials.html#cfn-appflow-connectorprofile-oauth2credentials-clientid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_id: Option<::Value<String>>,
        /// Property [`ClientSecret`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-oauth2credentials.html#cfn-appflow-connectorprofile-oauth2credentials-clientsecret).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_secret: Option<::Value<String>>,
        /// Property [`OAuthRequest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-oauth2credentials.html#cfn-appflow-connectorprofile-oauth2credentials-oauthrequest).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub o_auth_request: Option<::Value<ConnectorOAuthRequest>>,
        /// Property [`RefreshToken`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-oauth2credentials.html#cfn-appflow-connectorprofile-oauth2credentials-refreshtoken).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub refresh_token: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for OAuth2Credentials {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref access_token) = self.access_token {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessToken", access_token)?;
            }
            if let Some(ref client_id) = self.client_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientId", client_id)?;
            }
            if let Some(ref client_secret) = self.client_secret {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientSecret", client_secret)?;
            }
            if let Some(ref o_auth_request) = self.o_auth_request {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OAuthRequest", o_auth_request)?;
            }
            if let Some(ref refresh_token) = self.refresh_token {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RefreshToken", refresh_token)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OAuth2Credentials {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OAuth2Credentials, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OAuth2Credentials;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OAuth2Credentials")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut access_token: Option<::Value<String>> = None;
                    let mut client_id: Option<::Value<String>> = None;
                    let mut client_secret: Option<::Value<String>> = None;
                    let mut o_auth_request: Option<::Value<ConnectorOAuthRequest>> = None;
                    let mut refresh_token: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AccessToken" => {
                                access_token = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ClientId" => {
                                client_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ClientSecret" => {
                                client_secret = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OAuthRequest" => {
                                o_auth_request = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RefreshToken" => {
                                refresh_token = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OAuth2Credentials {
                        access_token: access_token,
                        client_id: client_id,
                        client_secret: client_secret,
                        o_auth_request: o_auth_request,
                        refresh_token: refresh_token,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.OAuth2Properties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-oauth2properties.html) property type.
    #[derive(Debug, Default)]
    pub struct OAuth2Properties {
        /// Property [`OAuth2GrantType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-oauth2properties.html#cfn-appflow-connectorprofile-oauth2properties-oauth2granttype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub o_auth2_grant_type: Option<::Value<String>>,
        /// Property [`TokenUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-oauth2properties.html#cfn-appflow-connectorprofile-oauth2properties-tokenurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub token_url: Option<::Value<String>>,
        /// Property [`TokenUrlCustomProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-oauth2properties.html#cfn-appflow-connectorprofile-oauth2properties-tokenurlcustomproperties).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub token_url_custom_properties: Option<::ValueMap<String>>,
    }

    impl ::codec::SerializeValue for OAuth2Properties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref o_auth2_grant_type) = self.o_auth2_grant_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OAuth2GrantType", o_auth2_grant_type)?;
            }
            if let Some(ref token_url) = self.token_url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TokenUrl", token_url)?;
            }
            if let Some(ref token_url_custom_properties) = self.token_url_custom_properties {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TokenUrlCustomProperties", token_url_custom_properties)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OAuth2Properties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OAuth2Properties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OAuth2Properties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OAuth2Properties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut o_auth2_grant_type: Option<::Value<String>> = None;
                    let mut token_url: Option<::Value<String>> = None;
                    let mut token_url_custom_properties: Option<::ValueMap<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "OAuth2GrantType" => {
                                o_auth2_grant_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TokenUrl" => {
                                token_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TokenUrlCustomProperties" => {
                                token_url_custom_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OAuth2Properties {
                        o_auth2_grant_type: o_auth2_grant_type,
                        token_url: token_url,
                        token_url_custom_properties: token_url_custom_properties,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.OAuthCredentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-oauthcredentials.html) property type.
    #[derive(Debug, Default)]
    pub struct OAuthCredentials {
        /// Property [`AccessToken`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-oauthcredentials.html#cfn-appflow-connectorprofile-oauthcredentials-accesstoken).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub access_token: Option<::Value<String>>,
        /// Property [`ClientId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-oauthcredentials.html#cfn-appflow-connectorprofile-oauthcredentials-clientid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_id: Option<::Value<String>>,
        /// Property [`ClientSecret`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-oauthcredentials.html#cfn-appflow-connectorprofile-oauthcredentials-clientsecret).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_secret: Option<::Value<String>>,
        /// Property [`ConnectorOAuthRequest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-oauthcredentials.html#cfn-appflow-connectorprofile-oauthcredentials-connectoroauthrequest).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connector_o_auth_request: Option<::Value<ConnectorOAuthRequest>>,
        /// Property [`RefreshToken`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-oauthcredentials.html#cfn-appflow-connectorprofile-oauthcredentials-refreshtoken).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub refresh_token: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for OAuthCredentials {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref access_token) = self.access_token {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessToken", access_token)?;
            }
            if let Some(ref client_id) = self.client_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientId", client_id)?;
            }
            if let Some(ref client_secret) = self.client_secret {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientSecret", client_secret)?;
            }
            if let Some(ref connector_o_auth_request) = self.connector_o_auth_request {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectorOAuthRequest", connector_o_auth_request)?;
            }
            if let Some(ref refresh_token) = self.refresh_token {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RefreshToken", refresh_token)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OAuthCredentials {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OAuthCredentials, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OAuthCredentials;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OAuthCredentials")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut access_token: Option<::Value<String>> = None;
                    let mut client_id: Option<::Value<String>> = None;
                    let mut client_secret: Option<::Value<String>> = None;
                    let mut connector_o_auth_request: Option<::Value<ConnectorOAuthRequest>> = None;
                    let mut refresh_token: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AccessToken" => {
                                access_token = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ClientId" => {
                                client_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ClientSecret" => {
                                client_secret = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConnectorOAuthRequest" => {
                                connector_o_auth_request = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RefreshToken" => {
                                refresh_token = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OAuthCredentials {
                        access_token: access_token,
                        client_id: client_id,
                        client_secret: client_secret,
                        connector_o_auth_request: connector_o_auth_request,
                        refresh_token: refresh_token,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.OAuthProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-oauthproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct OAuthProperties {
        /// Property [`AuthCodeUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-oauthproperties.html#cfn-appflow-connectorprofile-oauthproperties-authcodeurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub auth_code_url: Option<::Value<String>>,
        /// Property [`OAuthScopes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-oauthproperties.html#cfn-appflow-connectorprofile-oauthproperties-oauthscopes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub o_auth_scopes: Option<::ValueList<String>>,
        /// Property [`TokenUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-oauthproperties.html#cfn-appflow-connectorprofile-oauthproperties-tokenurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub token_url: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for OAuthProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref auth_code_url) = self.auth_code_url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthCodeUrl", auth_code_url)?;
            }
            if let Some(ref o_auth_scopes) = self.o_auth_scopes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OAuthScopes", o_auth_scopes)?;
            }
            if let Some(ref token_url) = self.token_url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TokenUrl", token_url)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OAuthProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OAuthProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OAuthProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OAuthProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut auth_code_url: Option<::Value<String>> = None;
                    let mut o_auth_scopes: Option<::ValueList<String>> = None;
                    let mut token_url: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AuthCodeUrl" => {
                                auth_code_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OAuthScopes" => {
                                o_auth_scopes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TokenUrl" => {
                                token_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OAuthProperties {
                        auth_code_url: auth_code_url,
                        o_auth_scopes: o_auth_scopes,
                        token_url: token_url,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.PardotConnectorProfileCredentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-pardotconnectorprofilecredentials.html) property type.
    #[derive(Debug, Default)]
    pub struct PardotConnectorProfileCredentials {
        /// Property [`AccessToken`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-pardotconnectorprofilecredentials.html#cfn-appflow-connectorprofile-pardotconnectorprofilecredentials-accesstoken).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub access_token: Option<::Value<String>>,
        /// Property [`ClientCredentialsArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-pardotconnectorprofilecredentials.html#cfn-appflow-connectorprofile-pardotconnectorprofilecredentials-clientcredentialsarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_credentials_arn: Option<::Value<String>>,
        /// Property [`ConnectorOAuthRequest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-pardotconnectorprofilecredentials.html#cfn-appflow-connectorprofile-pardotconnectorprofilecredentials-connectoroauthrequest).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connector_o_auth_request: Option<::Value<ConnectorOAuthRequest>>,
        /// Property [`RefreshToken`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-pardotconnectorprofilecredentials.html#cfn-appflow-connectorprofile-pardotconnectorprofilecredentials-refreshtoken).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub refresh_token: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for PardotConnectorProfileCredentials {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref access_token) = self.access_token {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessToken", access_token)?;
            }
            if let Some(ref client_credentials_arn) = self.client_credentials_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientCredentialsArn", client_credentials_arn)?;
            }
            if let Some(ref connector_o_auth_request) = self.connector_o_auth_request {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectorOAuthRequest", connector_o_auth_request)?;
            }
            if let Some(ref refresh_token) = self.refresh_token {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RefreshToken", refresh_token)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PardotConnectorProfileCredentials {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PardotConnectorProfileCredentials, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PardotConnectorProfileCredentials;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PardotConnectorProfileCredentials")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut access_token: Option<::Value<String>> = None;
                    let mut client_credentials_arn: Option<::Value<String>> = None;
                    let mut connector_o_auth_request: Option<::Value<ConnectorOAuthRequest>> = None;
                    let mut refresh_token: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AccessToken" => {
                                access_token = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ClientCredentialsArn" => {
                                client_credentials_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConnectorOAuthRequest" => {
                                connector_o_auth_request = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RefreshToken" => {
                                refresh_token = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PardotConnectorProfileCredentials {
                        access_token: access_token,
                        client_credentials_arn: client_credentials_arn,
                        connector_o_auth_request: connector_o_auth_request,
                        refresh_token: refresh_token,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.PardotConnectorProfileProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-pardotconnectorprofileproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct PardotConnectorProfileProperties {
        /// Property [`BusinessUnitId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-pardotconnectorprofileproperties.html#cfn-appflow-connectorprofile-pardotconnectorprofileproperties-businessunitid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub business_unit_id: ::Value<String>,
        /// Property [`InstanceUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-pardotconnectorprofileproperties.html#cfn-appflow-connectorprofile-pardotconnectorprofileproperties-instanceurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub instance_url: Option<::Value<String>>,
        /// Property [`IsSandboxEnvironment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-pardotconnectorprofileproperties.html#cfn-appflow-connectorprofile-pardotconnectorprofileproperties-issandboxenvironment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub is_sandbox_environment: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for PardotConnectorProfileProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BusinessUnitId", &self.business_unit_id)?;
            if let Some(ref instance_url) = self.instance_url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceUrl", instance_url)?;
            }
            if let Some(ref is_sandbox_environment) = self.is_sandbox_environment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsSandboxEnvironment", is_sandbox_environment)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PardotConnectorProfileProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PardotConnectorProfileProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PardotConnectorProfileProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PardotConnectorProfileProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut business_unit_id: Option<::Value<String>> = None;
                    let mut instance_url: Option<::Value<String>> = None;
                    let mut is_sandbox_environment: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BusinessUnitId" => {
                                business_unit_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InstanceUrl" => {
                                instance_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IsSandboxEnvironment" => {
                                is_sandbox_environment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PardotConnectorProfileProperties {
                        business_unit_id: business_unit_id.ok_or(::serde::de::Error::missing_field("BusinessUnitId"))?,
                        instance_url: instance_url,
                        is_sandbox_environment: is_sandbox_environment,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.RedshiftConnectorProfileCredentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-redshiftconnectorprofilecredentials.html) property type.
    #[derive(Debug, Default)]
    pub struct RedshiftConnectorProfileCredentials {
        /// Property [`Password`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-redshiftconnectorprofilecredentials.html#cfn-appflow-connectorprofile-redshiftconnectorprofilecredentials-password).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub password: Option<::Value<String>>,
        /// Property [`Username`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-redshiftconnectorprofilecredentials.html#cfn-appflow-connectorprofile-redshiftconnectorprofilecredentials-username).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub username: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for RedshiftConnectorProfileCredentials {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref password) = self.password {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Password", password)?;
            }
            if let Some(ref username) = self.username {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Username", username)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RedshiftConnectorProfileCredentials {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RedshiftConnectorProfileCredentials, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RedshiftConnectorProfileCredentials;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RedshiftConnectorProfileCredentials")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut password: Option<::Value<String>> = None;
                    let mut username: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Password" => {
                                password = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Username" => {
                                username = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RedshiftConnectorProfileCredentials {
                        password: password,
                        username: username,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.RedshiftConnectorProfileProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-redshiftconnectorprofileproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct RedshiftConnectorProfileProperties {
        /// Property [`BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-redshiftconnectorprofileproperties.html#cfn-appflow-connectorprofile-redshiftconnectorprofileproperties-bucketname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_name: ::Value<String>,
        /// Property [`BucketPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-redshiftconnectorprofileproperties.html#cfn-appflow-connectorprofile-redshiftconnectorprofileproperties-bucketprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_prefix: Option<::Value<String>>,
        /// Property [`ClusterIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-redshiftconnectorprofileproperties.html#cfn-appflow-connectorprofile-redshiftconnectorprofileproperties-clusteridentifier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cluster_identifier: Option<::Value<String>>,
        /// Property [`DataApiRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-redshiftconnectorprofileproperties.html#cfn-appflow-connectorprofile-redshiftconnectorprofileproperties-dataapirolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_api_role_arn: Option<::Value<String>>,
        /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-redshiftconnectorprofileproperties.html#cfn-appflow-connectorprofile-redshiftconnectorprofileproperties-databasename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_name: Option<::Value<String>>,
        /// Property [`DatabaseUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-redshiftconnectorprofileproperties.html#cfn-appflow-connectorprofile-redshiftconnectorprofileproperties-databaseurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_url: Option<::Value<String>>,
        /// Property [`IsRedshiftServerless`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-redshiftconnectorprofileproperties.html#cfn-appflow-connectorprofile-redshiftconnectorprofileproperties-isredshiftserverless).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub is_redshift_serverless: Option<::Value<bool>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-redshiftconnectorprofileproperties.html#cfn-appflow-connectorprofile-redshiftconnectorprofileproperties-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`WorkgroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-redshiftconnectorprofileproperties.html#cfn-appflow-connectorprofile-redshiftconnectorprofileproperties-workgroupname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub workgroup_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for RedshiftConnectorProfileProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketName", &self.bucket_name)?;
            if let Some(ref bucket_prefix) = self.bucket_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketPrefix", bucket_prefix)?;
            }
            if let Some(ref cluster_identifier) = self.cluster_identifier {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterIdentifier", cluster_identifier)?;
            }
            if let Some(ref data_api_role_arn) = self.data_api_role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataApiRoleArn", data_api_role_arn)?;
            }
            if let Some(ref database_name) = self.database_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", database_name)?;
            }
            if let Some(ref database_url) = self.database_url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseUrl", database_url)?;
            }
            if let Some(ref is_redshift_serverless) = self.is_redshift_serverless {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsRedshiftServerless", is_redshift_serverless)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            if let Some(ref workgroup_name) = self.workgroup_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkgroupName", workgroup_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RedshiftConnectorProfileProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RedshiftConnectorProfileProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RedshiftConnectorProfileProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RedshiftConnectorProfileProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket_name: Option<::Value<String>> = None;
                    let mut bucket_prefix: Option<::Value<String>> = None;
                    let mut cluster_identifier: Option<::Value<String>> = None;
                    let mut data_api_role_arn: Option<::Value<String>> = None;
                    let mut database_name: Option<::Value<String>> = None;
                    let mut database_url: Option<::Value<String>> = None;
                    let mut is_redshift_serverless: Option<::Value<bool>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut workgroup_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketName" => {
                                bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BucketPrefix" => {
                                bucket_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ClusterIdentifier" => {
                                cluster_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataApiRoleArn" => {
                                data_api_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabaseName" => {
                                database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabaseUrl" => {
                                database_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IsRedshiftServerless" => {
                                is_redshift_serverless = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WorkgroupName" => {
                                workgroup_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RedshiftConnectorProfileProperties {
                        bucket_name: bucket_name.ok_or(::serde::de::Error::missing_field("BucketName"))?,
                        bucket_prefix: bucket_prefix,
                        cluster_identifier: cluster_identifier,
                        data_api_role_arn: data_api_role_arn,
                        database_name: database_name,
                        database_url: database_url,
                        is_redshift_serverless: is_redshift_serverless,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                        workgroup_name: workgroup_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.SAPODataConnectorProfileCredentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-sapodataconnectorprofilecredentials.html) property type.
    #[derive(Debug, Default)]
    pub struct SAPODataConnectorProfileCredentials {
        /// Property [`BasicAuthCredentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-sapodataconnectorprofilecredentials.html#cfn-appflow-connectorprofile-sapodataconnectorprofilecredentials-basicauthcredentials).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub basic_auth_credentials: Option<::Value<BasicAuthCredentials>>,
        /// Property [`OAuthCredentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-sapodataconnectorprofilecredentials.html#cfn-appflow-connectorprofile-sapodataconnectorprofilecredentials-oauthcredentials).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub o_auth_credentials: Option<::Value<OAuthCredentials>>,
    }

    impl ::codec::SerializeValue for SAPODataConnectorProfileCredentials {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref basic_auth_credentials) = self.basic_auth_credentials {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BasicAuthCredentials", basic_auth_credentials)?;
            }
            if let Some(ref o_auth_credentials) = self.o_auth_credentials {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OAuthCredentials", o_auth_credentials)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SAPODataConnectorProfileCredentials {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SAPODataConnectorProfileCredentials, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SAPODataConnectorProfileCredentials;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SAPODataConnectorProfileCredentials")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut basic_auth_credentials: Option<::Value<BasicAuthCredentials>> = None;
                    let mut o_auth_credentials: Option<::Value<OAuthCredentials>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BasicAuthCredentials" => {
                                basic_auth_credentials = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OAuthCredentials" => {
                                o_auth_credentials = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SAPODataConnectorProfileCredentials {
                        basic_auth_credentials: basic_auth_credentials,
                        o_auth_credentials: o_auth_credentials,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.SAPODataConnectorProfileProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-sapodataconnectorprofileproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct SAPODataConnectorProfileProperties {
        /// Property [`ApplicationHostUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-sapodataconnectorprofileproperties.html#cfn-appflow-connectorprofile-sapodataconnectorprofileproperties-applicationhosturl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub application_host_url: Option<::Value<String>>,
        /// Property [`ApplicationServicePath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-sapodataconnectorprofileproperties.html#cfn-appflow-connectorprofile-sapodataconnectorprofileproperties-applicationservicepath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub application_service_path: Option<::Value<String>>,
        /// Property [`ClientNumber`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-sapodataconnectorprofileproperties.html#cfn-appflow-connectorprofile-sapodataconnectorprofileproperties-clientnumber).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_number: Option<::Value<String>>,
        /// Property [`DisableSSO`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-sapodataconnectorprofileproperties.html#cfn-appflow-connectorprofile-sapodataconnectorprofileproperties-disablesso).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub disable_sso: Option<::Value<bool>>,
        /// Property [`LogonLanguage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-sapodataconnectorprofileproperties.html#cfn-appflow-connectorprofile-sapodataconnectorprofileproperties-logonlanguage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub logon_language: Option<::Value<String>>,
        /// Property [`OAuthProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-sapodataconnectorprofileproperties.html#cfn-appflow-connectorprofile-sapodataconnectorprofileproperties-oauthproperties).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub o_auth_properties: Option<::Value<OAuthProperties>>,
        /// Property [`PortNumber`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-sapodataconnectorprofileproperties.html#cfn-appflow-connectorprofile-sapodataconnectorprofileproperties-portnumber).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port_number: Option<::Value<u32>>,
        /// Property [`PrivateLinkServiceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-sapodataconnectorprofileproperties.html#cfn-appflow-connectorprofile-sapodataconnectorprofileproperties-privatelinkservicename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub private_link_service_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SAPODataConnectorProfileProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref application_host_url) = self.application_host_url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationHostUrl", application_host_url)?;
            }
            if let Some(ref application_service_path) = self.application_service_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationServicePath", application_service_path)?;
            }
            if let Some(ref client_number) = self.client_number {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientNumber", client_number)?;
            }
            if let Some(ref disable_sso) = self.disable_sso {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisableSSO", disable_sso)?;
            }
            if let Some(ref logon_language) = self.logon_language {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogonLanguage", logon_language)?;
            }
            if let Some(ref o_auth_properties) = self.o_auth_properties {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OAuthProperties", o_auth_properties)?;
            }
            if let Some(ref port_number) = self.port_number {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PortNumber", port_number)?;
            }
            if let Some(ref private_link_service_name) = self.private_link_service_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrivateLinkServiceName", private_link_service_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SAPODataConnectorProfileProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SAPODataConnectorProfileProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SAPODataConnectorProfileProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SAPODataConnectorProfileProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut application_host_url: Option<::Value<String>> = None;
                    let mut application_service_path: Option<::Value<String>> = None;
                    let mut client_number: Option<::Value<String>> = None;
                    let mut disable_sso: Option<::Value<bool>> = None;
                    let mut logon_language: Option<::Value<String>> = None;
                    let mut o_auth_properties: Option<::Value<OAuthProperties>> = None;
                    let mut port_number: Option<::Value<u32>> = None;
                    let mut private_link_service_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ApplicationHostUrl" => {
                                application_host_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ApplicationServicePath" => {
                                application_service_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ClientNumber" => {
                                client_number = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DisableSSO" => {
                                disable_sso = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogonLanguage" => {
                                logon_language = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OAuthProperties" => {
                                o_auth_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PortNumber" => {
                                port_number = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PrivateLinkServiceName" => {
                                private_link_service_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SAPODataConnectorProfileProperties {
                        application_host_url: application_host_url,
                        application_service_path: application_service_path,
                        client_number: client_number,
                        disable_sso: disable_sso,
                        logon_language: logon_language,
                        o_auth_properties: o_auth_properties,
                        port_number: port_number,
                        private_link_service_name: private_link_service_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.SalesforceConnectorProfileCredentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-salesforceconnectorprofilecredentials.html) property type.
    #[derive(Debug, Default)]
    pub struct SalesforceConnectorProfileCredentials {
        /// Property [`AccessToken`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-salesforceconnectorprofilecredentials.html#cfn-appflow-connectorprofile-salesforceconnectorprofilecredentials-accesstoken).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub access_token: Option<::Value<String>>,
        /// Property [`ClientCredentialsArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-salesforceconnectorprofilecredentials.html#cfn-appflow-connectorprofile-salesforceconnectorprofilecredentials-clientcredentialsarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_credentials_arn: Option<::Value<String>>,
        /// Property [`ConnectorOAuthRequest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-salesforceconnectorprofilecredentials.html#cfn-appflow-connectorprofile-salesforceconnectorprofilecredentials-connectoroauthrequest).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connector_o_auth_request: Option<::Value<ConnectorOAuthRequest>>,
        /// Property [`JwtToken`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-salesforceconnectorprofilecredentials.html#cfn-appflow-connectorprofile-salesforceconnectorprofilecredentials-jwttoken).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub jwt_token: Option<::Value<String>>,
        /// Property [`OAuth2GrantType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-salesforceconnectorprofilecredentials.html#cfn-appflow-connectorprofile-salesforceconnectorprofilecredentials-oauth2granttype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub o_auth2_grant_type: Option<::Value<String>>,
        /// Property [`RefreshToken`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-salesforceconnectorprofilecredentials.html#cfn-appflow-connectorprofile-salesforceconnectorprofilecredentials-refreshtoken).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub refresh_token: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SalesforceConnectorProfileCredentials {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref access_token) = self.access_token {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessToken", access_token)?;
            }
            if let Some(ref client_credentials_arn) = self.client_credentials_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientCredentialsArn", client_credentials_arn)?;
            }
            if let Some(ref connector_o_auth_request) = self.connector_o_auth_request {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectorOAuthRequest", connector_o_auth_request)?;
            }
            if let Some(ref jwt_token) = self.jwt_token {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "JwtToken", jwt_token)?;
            }
            if let Some(ref o_auth2_grant_type) = self.o_auth2_grant_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OAuth2GrantType", o_auth2_grant_type)?;
            }
            if let Some(ref refresh_token) = self.refresh_token {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RefreshToken", refresh_token)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SalesforceConnectorProfileCredentials {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SalesforceConnectorProfileCredentials, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SalesforceConnectorProfileCredentials;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SalesforceConnectorProfileCredentials")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut access_token: Option<::Value<String>> = None;
                    let mut client_credentials_arn: Option<::Value<String>> = None;
                    let mut connector_o_auth_request: Option<::Value<ConnectorOAuthRequest>> = None;
                    let mut jwt_token: Option<::Value<String>> = None;
                    let mut o_auth2_grant_type: Option<::Value<String>> = None;
                    let mut refresh_token: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AccessToken" => {
                                access_token = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ClientCredentialsArn" => {
                                client_credentials_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConnectorOAuthRequest" => {
                                connector_o_auth_request = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "JwtToken" => {
                                jwt_token = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OAuth2GrantType" => {
                                o_auth2_grant_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RefreshToken" => {
                                refresh_token = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SalesforceConnectorProfileCredentials {
                        access_token: access_token,
                        client_credentials_arn: client_credentials_arn,
                        connector_o_auth_request: connector_o_auth_request,
                        jwt_token: jwt_token,
                        o_auth2_grant_type: o_auth2_grant_type,
                        refresh_token: refresh_token,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.SalesforceConnectorProfileProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-salesforceconnectorprofileproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct SalesforceConnectorProfileProperties {
        /// Property [`InstanceUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-salesforceconnectorprofileproperties.html#cfn-appflow-connectorprofile-salesforceconnectorprofileproperties-instanceurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub instance_url: Option<::Value<String>>,
        /// Property [`isSandboxEnvironment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-salesforceconnectorprofileproperties.html#cfn-appflow-connectorprofile-salesforceconnectorprofileproperties-issandboxenvironment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub is_sandbox_environment: Option<::Value<bool>>,
        /// Property [`usePrivateLinkForMetadataAndAuthorization`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-salesforceconnectorprofileproperties.html#cfn-appflow-connectorprofile-salesforceconnectorprofileproperties-useprivatelinkformetadataandauthorization).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub use_private_link_for_metadata_and_authorization: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for SalesforceConnectorProfileProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref instance_url) = self.instance_url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceUrl", instance_url)?;
            }
            if let Some(ref is_sandbox_environment) = self.is_sandbox_environment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "isSandboxEnvironment", is_sandbox_environment)?;
            }
            if let Some(ref use_private_link_for_metadata_and_authorization) = self.use_private_link_for_metadata_and_authorization {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "usePrivateLinkForMetadataAndAuthorization", use_private_link_for_metadata_and_authorization)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SalesforceConnectorProfileProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SalesforceConnectorProfileProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SalesforceConnectorProfileProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SalesforceConnectorProfileProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut instance_url: Option<::Value<String>> = None;
                    let mut is_sandbox_environment: Option<::Value<bool>> = None;
                    let mut use_private_link_for_metadata_and_authorization: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InstanceUrl" => {
                                instance_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "isSandboxEnvironment" => {
                                is_sandbox_environment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "usePrivateLinkForMetadataAndAuthorization" => {
                                use_private_link_for_metadata_and_authorization = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SalesforceConnectorProfileProperties {
                        instance_url: instance_url,
                        is_sandbox_environment: is_sandbox_environment,
                        use_private_link_for_metadata_and_authorization: use_private_link_for_metadata_and_authorization,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.ServiceNowConnectorProfileCredentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-servicenowconnectorprofilecredentials.html) property type.
    #[derive(Debug, Default)]
    pub struct ServiceNowConnectorProfileCredentials {
        /// Property [`OAuth2Credentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-servicenowconnectorprofilecredentials.html#cfn-appflow-connectorprofile-servicenowconnectorprofilecredentials-oauth2credentials).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub o_auth2_credentials: Option<::Value<OAuth2Credentials>>,
        /// Property [`Password`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-servicenowconnectorprofilecredentials.html#cfn-appflow-connectorprofile-servicenowconnectorprofilecredentials-password).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub password: Option<::Value<String>>,
        /// Property [`Username`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-servicenowconnectorprofilecredentials.html#cfn-appflow-connectorprofile-servicenowconnectorprofilecredentials-username).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub username: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ServiceNowConnectorProfileCredentials {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref o_auth2_credentials) = self.o_auth2_credentials {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OAuth2Credentials", o_auth2_credentials)?;
            }
            if let Some(ref password) = self.password {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Password", password)?;
            }
            if let Some(ref username) = self.username {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Username", username)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ServiceNowConnectorProfileCredentials {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ServiceNowConnectorProfileCredentials, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ServiceNowConnectorProfileCredentials;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ServiceNowConnectorProfileCredentials")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut o_auth2_credentials: Option<::Value<OAuth2Credentials>> = None;
                    let mut password: Option<::Value<String>> = None;
                    let mut username: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "OAuth2Credentials" => {
                                o_auth2_credentials = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Password" => {
                                password = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Username" => {
                                username = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ServiceNowConnectorProfileCredentials {
                        o_auth2_credentials: o_auth2_credentials,
                        password: password,
                        username: username,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.ServiceNowConnectorProfileProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-servicenowconnectorprofileproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct ServiceNowConnectorProfileProperties {
        /// Property [`InstanceUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-servicenowconnectorprofileproperties.html#cfn-appflow-connectorprofile-servicenowconnectorprofileproperties-instanceurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub instance_url: ::Value<String>,
    }

    impl ::codec::SerializeValue for ServiceNowConnectorProfileProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceUrl", &self.instance_url)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ServiceNowConnectorProfileProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ServiceNowConnectorProfileProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ServiceNowConnectorProfileProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ServiceNowConnectorProfileProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut instance_url: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InstanceUrl" => {
                                instance_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ServiceNowConnectorProfileProperties {
                        instance_url: instance_url.ok_or(::serde::de::Error::missing_field("InstanceUrl"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.SingularConnectorProfileCredentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-singularconnectorprofilecredentials.html) property type.
    #[derive(Debug, Default)]
    pub struct SingularConnectorProfileCredentials {
        /// Property [`ApiKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-singularconnectorprofilecredentials.html#cfn-appflow-connectorprofile-singularconnectorprofilecredentials-apikey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub api_key: ::Value<String>,
    }

    impl ::codec::SerializeValue for SingularConnectorProfileCredentials {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiKey", &self.api_key)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SingularConnectorProfileCredentials {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SingularConnectorProfileCredentials, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SingularConnectorProfileCredentials;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SingularConnectorProfileCredentials")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut api_key: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ApiKey" => {
                                api_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SingularConnectorProfileCredentials {
                        api_key: api_key.ok_or(::serde::de::Error::missing_field("ApiKey"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.SlackConnectorProfileCredentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-slackconnectorprofilecredentials.html) property type.
    #[derive(Debug, Default)]
    pub struct SlackConnectorProfileCredentials {
        /// Property [`AccessToken`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-slackconnectorprofilecredentials.html#cfn-appflow-connectorprofile-slackconnectorprofilecredentials-accesstoken).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub access_token: Option<::Value<String>>,
        /// Property [`ClientId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-slackconnectorprofilecredentials.html#cfn-appflow-connectorprofile-slackconnectorprofilecredentials-clientid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_id: ::Value<String>,
        /// Property [`ClientSecret`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-slackconnectorprofilecredentials.html#cfn-appflow-connectorprofile-slackconnectorprofilecredentials-clientsecret).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_secret: ::Value<String>,
        /// Property [`ConnectorOAuthRequest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-slackconnectorprofilecredentials.html#cfn-appflow-connectorprofile-slackconnectorprofilecredentials-connectoroauthrequest).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connector_o_auth_request: Option<::Value<ConnectorOAuthRequest>>,
    }

    impl ::codec::SerializeValue for SlackConnectorProfileCredentials {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref access_token) = self.access_token {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessToken", access_token)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientId", &self.client_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientSecret", &self.client_secret)?;
            if let Some(ref connector_o_auth_request) = self.connector_o_auth_request {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectorOAuthRequest", connector_o_auth_request)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SlackConnectorProfileCredentials {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SlackConnectorProfileCredentials, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SlackConnectorProfileCredentials;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SlackConnectorProfileCredentials")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut access_token: Option<::Value<String>> = None;
                    let mut client_id: Option<::Value<String>> = None;
                    let mut client_secret: Option<::Value<String>> = None;
                    let mut connector_o_auth_request: Option<::Value<ConnectorOAuthRequest>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AccessToken" => {
                                access_token = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ClientId" => {
                                client_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ClientSecret" => {
                                client_secret = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConnectorOAuthRequest" => {
                                connector_o_auth_request = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SlackConnectorProfileCredentials {
                        access_token: access_token,
                        client_id: client_id.ok_or(::serde::de::Error::missing_field("ClientId"))?,
                        client_secret: client_secret.ok_or(::serde::de::Error::missing_field("ClientSecret"))?,
                        connector_o_auth_request: connector_o_auth_request,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.SlackConnectorProfileProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-slackconnectorprofileproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct SlackConnectorProfileProperties {
        /// Property [`InstanceUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-slackconnectorprofileproperties.html#cfn-appflow-connectorprofile-slackconnectorprofileproperties-instanceurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub instance_url: ::Value<String>,
    }

    impl ::codec::SerializeValue for SlackConnectorProfileProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceUrl", &self.instance_url)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SlackConnectorProfileProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SlackConnectorProfileProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SlackConnectorProfileProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SlackConnectorProfileProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut instance_url: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InstanceUrl" => {
                                instance_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SlackConnectorProfileProperties {
                        instance_url: instance_url.ok_or(::serde::de::Error::missing_field("InstanceUrl"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.SnowflakeConnectorProfileCredentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-snowflakeconnectorprofilecredentials.html) property type.
    #[derive(Debug, Default)]
    pub struct SnowflakeConnectorProfileCredentials {
        /// Property [`Password`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-snowflakeconnectorprofilecredentials.html#cfn-appflow-connectorprofile-snowflakeconnectorprofilecredentials-password).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub password: ::Value<String>,
        /// Property [`Username`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-snowflakeconnectorprofilecredentials.html#cfn-appflow-connectorprofile-snowflakeconnectorprofilecredentials-username).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub username: ::Value<String>,
    }

    impl ::codec::SerializeValue for SnowflakeConnectorProfileCredentials {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Password", &self.password)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Username", &self.username)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SnowflakeConnectorProfileCredentials {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SnowflakeConnectorProfileCredentials, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SnowflakeConnectorProfileCredentials;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SnowflakeConnectorProfileCredentials")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut password: Option<::Value<String>> = None;
                    let mut username: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Password" => {
                                password = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Username" => {
                                username = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SnowflakeConnectorProfileCredentials {
                        password: password.ok_or(::serde::de::Error::missing_field("Password"))?,
                        username: username.ok_or(::serde::de::Error::missing_field("Username"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.SnowflakeConnectorProfileProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-snowflakeconnectorprofileproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct SnowflakeConnectorProfileProperties {
        /// Property [`AccountName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-snowflakeconnectorprofileproperties.html#cfn-appflow-connectorprofile-snowflakeconnectorprofileproperties-accountname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub account_name: Option<::Value<String>>,
        /// Property [`BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-snowflakeconnectorprofileproperties.html#cfn-appflow-connectorprofile-snowflakeconnectorprofileproperties-bucketname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_name: ::Value<String>,
        /// Property [`BucketPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-snowflakeconnectorprofileproperties.html#cfn-appflow-connectorprofile-snowflakeconnectorprofileproperties-bucketprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_prefix: Option<::Value<String>>,
        /// Property [`PrivateLinkServiceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-snowflakeconnectorprofileproperties.html#cfn-appflow-connectorprofile-snowflakeconnectorprofileproperties-privatelinkservicename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub private_link_service_name: Option<::Value<String>>,
        /// Property [`Region`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-snowflakeconnectorprofileproperties.html#cfn-appflow-connectorprofile-snowflakeconnectorprofileproperties-region).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub region: Option<::Value<String>>,
        /// Property [`Stage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-snowflakeconnectorprofileproperties.html#cfn-appflow-connectorprofile-snowflakeconnectorprofileproperties-stage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stage: ::Value<String>,
        /// Property [`Warehouse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-snowflakeconnectorprofileproperties.html#cfn-appflow-connectorprofile-snowflakeconnectorprofileproperties-warehouse).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub warehouse: ::Value<String>,
    }

    impl ::codec::SerializeValue for SnowflakeConnectorProfileProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref account_name) = self.account_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccountName", account_name)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketName", &self.bucket_name)?;
            if let Some(ref bucket_prefix) = self.bucket_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketPrefix", bucket_prefix)?;
            }
            if let Some(ref private_link_service_name) = self.private_link_service_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrivateLinkServiceName", private_link_service_name)?;
            }
            if let Some(ref region) = self.region {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Region", region)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Stage", &self.stage)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Warehouse", &self.warehouse)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SnowflakeConnectorProfileProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SnowflakeConnectorProfileProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SnowflakeConnectorProfileProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SnowflakeConnectorProfileProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut account_name: Option<::Value<String>> = None;
                    let mut bucket_name: Option<::Value<String>> = None;
                    let mut bucket_prefix: Option<::Value<String>> = None;
                    let mut private_link_service_name: Option<::Value<String>> = None;
                    let mut region: Option<::Value<String>> = None;
                    let mut stage: Option<::Value<String>> = None;
                    let mut warehouse: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AccountName" => {
                                account_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BucketName" => {
                                bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BucketPrefix" => {
                                bucket_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PrivateLinkServiceName" => {
                                private_link_service_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Region" => {
                                region = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Stage" => {
                                stage = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Warehouse" => {
                                warehouse = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SnowflakeConnectorProfileProperties {
                        account_name: account_name,
                        bucket_name: bucket_name.ok_or(::serde::de::Error::missing_field("BucketName"))?,
                        bucket_prefix: bucket_prefix,
                        private_link_service_name: private_link_service_name,
                        region: region,
                        stage: stage.ok_or(::serde::de::Error::missing_field("Stage"))?,
                        warehouse: warehouse.ok_or(::serde::de::Error::missing_field("Warehouse"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.TrendmicroConnectorProfileCredentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-trendmicroconnectorprofilecredentials.html) property type.
    #[derive(Debug, Default)]
    pub struct TrendmicroConnectorProfileCredentials {
        /// Property [`ApiSecretKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-trendmicroconnectorprofilecredentials.html#cfn-appflow-connectorprofile-trendmicroconnectorprofilecredentials-apisecretkey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub api_secret_key: ::Value<String>,
    }

    impl ::codec::SerializeValue for TrendmicroConnectorProfileCredentials {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiSecretKey", &self.api_secret_key)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TrendmicroConnectorProfileCredentials {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TrendmicroConnectorProfileCredentials, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TrendmicroConnectorProfileCredentials;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TrendmicroConnectorProfileCredentials")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut api_secret_key: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ApiSecretKey" => {
                                api_secret_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TrendmicroConnectorProfileCredentials {
                        api_secret_key: api_secret_key.ok_or(::serde::de::Error::missing_field("ApiSecretKey"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.VeevaConnectorProfileCredentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-veevaconnectorprofilecredentials.html) property type.
    #[derive(Debug, Default)]
    pub struct VeevaConnectorProfileCredentials {
        /// Property [`Password`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-veevaconnectorprofilecredentials.html#cfn-appflow-connectorprofile-veevaconnectorprofilecredentials-password).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub password: ::Value<String>,
        /// Property [`Username`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-veevaconnectorprofilecredentials.html#cfn-appflow-connectorprofile-veevaconnectorprofilecredentials-username).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub username: ::Value<String>,
    }

    impl ::codec::SerializeValue for VeevaConnectorProfileCredentials {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Password", &self.password)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Username", &self.username)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VeevaConnectorProfileCredentials {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VeevaConnectorProfileCredentials, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VeevaConnectorProfileCredentials;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VeevaConnectorProfileCredentials")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut password: Option<::Value<String>> = None;
                    let mut username: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Password" => {
                                password = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Username" => {
                                username = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VeevaConnectorProfileCredentials {
                        password: password.ok_or(::serde::de::Error::missing_field("Password"))?,
                        username: username.ok_or(::serde::de::Error::missing_field("Username"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.VeevaConnectorProfileProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-veevaconnectorprofileproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct VeevaConnectorProfileProperties {
        /// Property [`InstanceUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-veevaconnectorprofileproperties.html#cfn-appflow-connectorprofile-veevaconnectorprofileproperties-instanceurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub instance_url: ::Value<String>,
    }

    impl ::codec::SerializeValue for VeevaConnectorProfileProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceUrl", &self.instance_url)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VeevaConnectorProfileProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VeevaConnectorProfileProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VeevaConnectorProfileProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VeevaConnectorProfileProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut instance_url: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InstanceUrl" => {
                                instance_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VeevaConnectorProfileProperties {
                        instance_url: instance_url.ok_or(::serde::de::Error::missing_field("InstanceUrl"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.ZendeskConnectorProfileCredentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-zendeskconnectorprofilecredentials.html) property type.
    #[derive(Debug, Default)]
    pub struct ZendeskConnectorProfileCredentials {
        /// Property [`AccessToken`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-zendeskconnectorprofilecredentials.html#cfn-appflow-connectorprofile-zendeskconnectorprofilecredentials-accesstoken).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub access_token: Option<::Value<String>>,
        /// Property [`ClientId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-zendeskconnectorprofilecredentials.html#cfn-appflow-connectorprofile-zendeskconnectorprofilecredentials-clientid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_id: ::Value<String>,
        /// Property [`ClientSecret`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-zendeskconnectorprofilecredentials.html#cfn-appflow-connectorprofile-zendeskconnectorprofilecredentials-clientsecret).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_secret: ::Value<String>,
        /// Property [`ConnectorOAuthRequest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-zendeskconnectorprofilecredentials.html#cfn-appflow-connectorprofile-zendeskconnectorprofilecredentials-connectoroauthrequest).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connector_o_auth_request: Option<::Value<ConnectorOAuthRequest>>,
    }

    impl ::codec::SerializeValue for ZendeskConnectorProfileCredentials {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref access_token) = self.access_token {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessToken", access_token)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientId", &self.client_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientSecret", &self.client_secret)?;
            if let Some(ref connector_o_auth_request) = self.connector_o_auth_request {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectorOAuthRequest", connector_o_auth_request)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ZendeskConnectorProfileCredentials {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ZendeskConnectorProfileCredentials, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ZendeskConnectorProfileCredentials;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ZendeskConnectorProfileCredentials")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut access_token: Option<::Value<String>> = None;
                    let mut client_id: Option<::Value<String>> = None;
                    let mut client_secret: Option<::Value<String>> = None;
                    let mut connector_o_auth_request: Option<::Value<ConnectorOAuthRequest>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AccessToken" => {
                                access_token = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ClientId" => {
                                client_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ClientSecret" => {
                                client_secret = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConnectorOAuthRequest" => {
                                connector_o_auth_request = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ZendeskConnectorProfileCredentials {
                        access_token: access_token,
                        client_id: client_id.ok_or(::serde::de::Error::missing_field("ClientId"))?,
                        client_secret: client_secret.ok_or(::serde::de::Error::missing_field("ClientSecret"))?,
                        connector_o_auth_request: connector_o_auth_request,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::ConnectorProfile.ZendeskConnectorProfileProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-zendeskconnectorprofileproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct ZendeskConnectorProfileProperties {
        /// Property [`InstanceUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-connectorprofile-zendeskconnectorprofileproperties.html#cfn-appflow-connectorprofile-zendeskconnectorprofileproperties-instanceurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub instance_url: ::Value<String>,
    }

    impl ::codec::SerializeValue for ZendeskConnectorProfileProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceUrl", &self.instance_url)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ZendeskConnectorProfileProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ZendeskConnectorProfileProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ZendeskConnectorProfileProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ZendeskConnectorProfileProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut instance_url: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InstanceUrl" => {
                                instance_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ZendeskConnectorProfileProperties {
                        instance_url: instance_url.ok_or(::serde::de::Error::missing_field("InstanceUrl"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod flow {
    //! Property types for the `Flow` resource.

    /// The [`AWS::AppFlow::Flow.AggregationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-aggregationconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct AggregationConfig {
        /// Property [`AggregationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-aggregationconfig.html#cfn-appflow-flow-aggregationconfig-aggregationtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aggregation_type: Option<::Value<String>>,
        /// Property [`TargetFileSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-aggregationconfig.html#cfn-appflow-flow-aggregationconfig-targetfilesize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_file_size: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for AggregationConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref aggregation_type) = self.aggregation_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AggregationType", aggregation_type)?;
            }
            if let Some(ref target_file_size) = self.target_file_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetFileSize", target_file_size)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AggregationConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AggregationConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AggregationConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AggregationConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut aggregation_type: Option<::Value<String>> = None;
                    let mut target_file_size: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AggregationType" => {
                                aggregation_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetFileSize" => {
                                target_file_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AggregationConfig {
                        aggregation_type: aggregation_type,
                        target_file_size: target_file_size,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.AmplitudeSourceProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-amplitudesourceproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct AmplitudeSourceProperties {
        /// Property [`Object`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-amplitudesourceproperties.html#cfn-appflow-flow-amplitudesourceproperties-object).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object: ::Value<String>,
    }

    impl ::codec::SerializeValue for AmplitudeSourceProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Object", &self.object)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AmplitudeSourceProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AmplitudeSourceProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AmplitudeSourceProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AmplitudeSourceProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut object: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Object" => {
                                object = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AmplitudeSourceProperties {
                        object: object.ok_or(::serde::de::Error::missing_field("Object"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.ConnectorOperator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-connectoroperator.html) property type.
    #[derive(Debug, Default)]
    pub struct ConnectorOperator {
        /// Property [`Amplitude`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-connectoroperator.html#cfn-appflow-flow-connectoroperator-amplitude).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub amplitude: Option<::Value<String>>,
        /// Property [`CustomConnector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-connectoroperator.html#cfn-appflow-flow-connectoroperator-customconnector).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_connector: Option<::Value<String>>,
        /// Property [`Datadog`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-connectoroperator.html#cfn-appflow-flow-connectoroperator-datadog).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub datadog: Option<::Value<String>>,
        /// Property [`Dynatrace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-connectoroperator.html#cfn-appflow-flow-connectoroperator-dynatrace).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dynatrace: Option<::Value<String>>,
        /// Property [`GoogleAnalytics`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-connectoroperator.html#cfn-appflow-flow-connectoroperator-googleanalytics).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub google_analytics: Option<::Value<String>>,
        /// Property [`InforNexus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-connectoroperator.html#cfn-appflow-flow-connectoroperator-infornexus).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub infor_nexus: Option<::Value<String>>,
        /// Property [`Marketo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-connectoroperator.html#cfn-appflow-flow-connectoroperator-marketo).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub marketo: Option<::Value<String>>,
        /// Property [`Pardot`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-connectoroperator.html#cfn-appflow-flow-connectoroperator-pardot).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pardot: Option<::Value<String>>,
        /// Property [`S3`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-connectoroperator.html#cfn-appflow-flow-connectoroperator-s3).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3: Option<::Value<String>>,
        /// Property [`SAPOData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-connectoroperator.html#cfn-appflow-flow-connectoroperator-sapodata).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sapo_data: Option<::Value<String>>,
        /// Property [`Salesforce`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-connectoroperator.html#cfn-appflow-flow-connectoroperator-salesforce).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub salesforce: Option<::Value<String>>,
        /// Property [`ServiceNow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-connectoroperator.html#cfn-appflow-flow-connectoroperator-servicenow).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub service_now: Option<::Value<String>>,
        /// Property [`Singular`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-connectoroperator.html#cfn-appflow-flow-connectoroperator-singular).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub singular: Option<::Value<String>>,
        /// Property [`Slack`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-connectoroperator.html#cfn-appflow-flow-connectoroperator-slack).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub slack: Option<::Value<String>>,
        /// Property [`Trendmicro`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-connectoroperator.html#cfn-appflow-flow-connectoroperator-trendmicro).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub trendmicro: Option<::Value<String>>,
        /// Property [`Veeva`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-connectoroperator.html#cfn-appflow-flow-connectoroperator-veeva).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub veeva: Option<::Value<String>>,
        /// Property [`Zendesk`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-connectoroperator.html#cfn-appflow-flow-connectoroperator-zendesk).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub zendesk: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ConnectorOperator {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref amplitude) = self.amplitude {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Amplitude", amplitude)?;
            }
            if let Some(ref custom_connector) = self.custom_connector {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomConnector", custom_connector)?;
            }
            if let Some(ref datadog) = self.datadog {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Datadog", datadog)?;
            }
            if let Some(ref dynatrace) = self.dynatrace {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Dynatrace", dynatrace)?;
            }
            if let Some(ref google_analytics) = self.google_analytics {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GoogleAnalytics", google_analytics)?;
            }
            if let Some(ref infor_nexus) = self.infor_nexus {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InforNexus", infor_nexus)?;
            }
            if let Some(ref marketo) = self.marketo {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Marketo", marketo)?;
            }
            if let Some(ref pardot) = self.pardot {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Pardot", pardot)?;
            }
            if let Some(ref s3) = self.s3 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3", s3)?;
            }
            if let Some(ref sapo_data) = self.sapo_data {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SAPOData", sapo_data)?;
            }
            if let Some(ref salesforce) = self.salesforce {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Salesforce", salesforce)?;
            }
            if let Some(ref service_now) = self.service_now {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceNow", service_now)?;
            }
            if let Some(ref singular) = self.singular {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Singular", singular)?;
            }
            if let Some(ref slack) = self.slack {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Slack", slack)?;
            }
            if let Some(ref trendmicro) = self.trendmicro {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Trendmicro", trendmicro)?;
            }
            if let Some(ref veeva) = self.veeva {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Veeva", veeva)?;
            }
            if let Some(ref zendesk) = self.zendesk {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Zendesk", zendesk)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConnectorOperator {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConnectorOperator, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConnectorOperator;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConnectorOperator")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut amplitude: Option<::Value<String>> = None;
                    let mut custom_connector: Option<::Value<String>> = None;
                    let mut datadog: Option<::Value<String>> = None;
                    let mut dynatrace: Option<::Value<String>> = None;
                    let mut google_analytics: Option<::Value<String>> = None;
                    let mut infor_nexus: Option<::Value<String>> = None;
                    let mut marketo: Option<::Value<String>> = None;
                    let mut pardot: Option<::Value<String>> = None;
                    let mut s3: Option<::Value<String>> = None;
                    let mut sapo_data: Option<::Value<String>> = None;
                    let mut salesforce: Option<::Value<String>> = None;
                    let mut service_now: Option<::Value<String>> = None;
                    let mut singular: Option<::Value<String>> = None;
                    let mut slack: Option<::Value<String>> = None;
                    let mut trendmicro: Option<::Value<String>> = None;
                    let mut veeva: Option<::Value<String>> = None;
                    let mut zendesk: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Amplitude" => {
                                amplitude = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CustomConnector" => {
                                custom_connector = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Datadog" => {
                                datadog = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Dynatrace" => {
                                dynatrace = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GoogleAnalytics" => {
                                google_analytics = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InforNexus" => {
                                infor_nexus = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Marketo" => {
                                marketo = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Pardot" => {
                                pardot = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3" => {
                                s3 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SAPOData" => {
                                sapo_data = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Salesforce" => {
                                salesforce = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServiceNow" => {
                                service_now = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Singular" => {
                                singular = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Slack" => {
                                slack = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Trendmicro" => {
                                trendmicro = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Veeva" => {
                                veeva = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Zendesk" => {
                                zendesk = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConnectorOperator {
                        amplitude: amplitude,
                        custom_connector: custom_connector,
                        datadog: datadog,
                        dynatrace: dynatrace,
                        google_analytics: google_analytics,
                        infor_nexus: infor_nexus,
                        marketo: marketo,
                        pardot: pardot,
                        s3: s3,
                        sapo_data: sapo_data,
                        salesforce: salesforce,
                        service_now: service_now,
                        singular: singular,
                        slack: slack,
                        trendmicro: trendmicro,
                        veeva: veeva,
                        zendesk: zendesk,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.CustomConnectorDestinationProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-customconnectordestinationproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomConnectorDestinationProperties {
        /// Property [`CustomProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-customconnectordestinationproperties.html#cfn-appflow-flow-customconnectordestinationproperties-customproperties).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_properties: Option<::ValueMap<String>>,
        /// Property [`EntityName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-customconnectordestinationproperties.html#cfn-appflow-flow-customconnectordestinationproperties-entityname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub entity_name: ::Value<String>,
        /// Property [`ErrorHandlingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-customconnectordestinationproperties.html#cfn-appflow-flow-customconnectordestinationproperties-errorhandlingconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub error_handling_config: Option<::Value<ErrorHandlingConfig>>,
        /// Property [`IdFieldNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-customconnectordestinationproperties.html#cfn-appflow-flow-customconnectordestinationproperties-idfieldnames).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id_field_names: Option<::ValueList<String>>,
        /// Property [`WriteOperationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-customconnectordestinationproperties.html#cfn-appflow-flow-customconnectordestinationproperties-writeoperationtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub write_operation_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CustomConnectorDestinationProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref custom_properties) = self.custom_properties {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomProperties", custom_properties)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EntityName", &self.entity_name)?;
            if let Some(ref error_handling_config) = self.error_handling_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ErrorHandlingConfig", error_handling_config)?;
            }
            if let Some(ref id_field_names) = self.id_field_names {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdFieldNames", id_field_names)?;
            }
            if let Some(ref write_operation_type) = self.write_operation_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WriteOperationType", write_operation_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomConnectorDestinationProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomConnectorDestinationProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomConnectorDestinationProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomConnectorDestinationProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut custom_properties: Option<::ValueMap<String>> = None;
                    let mut entity_name: Option<::Value<String>> = None;
                    let mut error_handling_config: Option<::Value<ErrorHandlingConfig>> = None;
                    let mut id_field_names: Option<::ValueList<String>> = None;
                    let mut write_operation_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CustomProperties" => {
                                custom_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EntityName" => {
                                entity_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ErrorHandlingConfig" => {
                                error_handling_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IdFieldNames" => {
                                id_field_names = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WriteOperationType" => {
                                write_operation_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomConnectorDestinationProperties {
                        custom_properties: custom_properties,
                        entity_name: entity_name.ok_or(::serde::de::Error::missing_field("EntityName"))?,
                        error_handling_config: error_handling_config,
                        id_field_names: id_field_names,
                        write_operation_type: write_operation_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.CustomConnectorSourceProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-customconnectorsourceproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomConnectorSourceProperties {
        /// Property [`CustomProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-customconnectorsourceproperties.html#cfn-appflow-flow-customconnectorsourceproperties-customproperties).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_properties: Option<::ValueMap<String>>,
        /// Property [`DataTransferApi`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-customconnectorsourceproperties.html#cfn-appflow-flow-customconnectorsourceproperties-datatransferapi).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_transfer_api: Option<::Value<DataTransferApi>>,
        /// Property [`EntityName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-customconnectorsourceproperties.html#cfn-appflow-flow-customconnectorsourceproperties-entityname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub entity_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for CustomConnectorSourceProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref custom_properties) = self.custom_properties {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomProperties", custom_properties)?;
            }
            if let Some(ref data_transfer_api) = self.data_transfer_api {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataTransferApi", data_transfer_api)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EntityName", &self.entity_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomConnectorSourceProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomConnectorSourceProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomConnectorSourceProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomConnectorSourceProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut custom_properties: Option<::ValueMap<String>> = None;
                    let mut data_transfer_api: Option<::Value<DataTransferApi>> = None;
                    let mut entity_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CustomProperties" => {
                                custom_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataTransferApi" => {
                                data_transfer_api = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EntityName" => {
                                entity_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomConnectorSourceProperties {
                        custom_properties: custom_properties,
                        data_transfer_api: data_transfer_api,
                        entity_name: entity_name.ok_or(::serde::de::Error::missing_field("EntityName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.DataTransferApi`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-datatransferapi.html) property type.
    #[derive(Debug, Default)]
    pub struct DataTransferApi {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-datatransferapi.html#cfn-appflow-flow-datatransferapi-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-datatransferapi.html#cfn-appflow-flow-datatransferapi-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for DataTransferApi {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataTransferApi {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataTransferApi, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataTransferApi;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataTransferApi")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataTransferApi {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.DatadogSourceProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-datadogsourceproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct DatadogSourceProperties {
        /// Property [`Object`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-datadogsourceproperties.html#cfn-appflow-flow-datadogsourceproperties-object).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object: ::Value<String>,
    }

    impl ::codec::SerializeValue for DatadogSourceProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Object", &self.object)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DatadogSourceProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DatadogSourceProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DatadogSourceProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DatadogSourceProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut object: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Object" => {
                                object = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DatadogSourceProperties {
                        object: object.ok_or(::serde::de::Error::missing_field("Object"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.DestinationConnectorProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-destinationconnectorproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct DestinationConnectorProperties {
        /// Property [`CustomConnector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-destinationconnectorproperties.html#cfn-appflow-flow-destinationconnectorproperties-customconnector).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_connector: Option<::Value<CustomConnectorDestinationProperties>>,
        /// Property [`EventBridge`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-destinationconnectorproperties.html#cfn-appflow-flow-destinationconnectorproperties-eventbridge).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub event_bridge: Option<::Value<EventBridgeDestinationProperties>>,
        /// Property [`LookoutMetrics`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-destinationconnectorproperties.html#cfn-appflow-flow-destinationconnectorproperties-lookoutmetrics).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lookout_metrics: Option<::Value<LookoutMetricsDestinationProperties>>,
        /// Property [`Marketo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-destinationconnectorproperties.html#cfn-appflow-flow-destinationconnectorproperties-marketo).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub marketo: Option<::Value<MarketoDestinationProperties>>,
        /// Property [`Redshift`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-destinationconnectorproperties.html#cfn-appflow-flow-destinationconnectorproperties-redshift).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub redshift: Option<::Value<RedshiftDestinationProperties>>,
        /// Property [`S3`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-destinationconnectorproperties.html#cfn-appflow-flow-destinationconnectorproperties-s3).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3: Option<::Value<S3DestinationProperties>>,
        /// Property [`SAPOData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-destinationconnectorproperties.html#cfn-appflow-flow-destinationconnectorproperties-sapodata).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sapo_data: Option<::Value<SAPODataDestinationProperties>>,
        /// Property [`Salesforce`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-destinationconnectorproperties.html#cfn-appflow-flow-destinationconnectorproperties-salesforce).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub salesforce: Option<::Value<SalesforceDestinationProperties>>,
        /// Property [`Snowflake`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-destinationconnectorproperties.html#cfn-appflow-flow-destinationconnectorproperties-snowflake).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub snowflake: Option<::Value<SnowflakeDestinationProperties>>,
        /// Property [`Upsolver`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-destinationconnectorproperties.html#cfn-appflow-flow-destinationconnectorproperties-upsolver).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub upsolver: Option<::Value<UpsolverDestinationProperties>>,
        /// Property [`Zendesk`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-destinationconnectorproperties.html#cfn-appflow-flow-destinationconnectorproperties-zendesk).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub zendesk: Option<::Value<ZendeskDestinationProperties>>,
    }

    impl ::codec::SerializeValue for DestinationConnectorProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref custom_connector) = self.custom_connector {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomConnector", custom_connector)?;
            }
            if let Some(ref event_bridge) = self.event_bridge {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventBridge", event_bridge)?;
            }
            if let Some(ref lookout_metrics) = self.lookout_metrics {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LookoutMetrics", lookout_metrics)?;
            }
            if let Some(ref marketo) = self.marketo {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Marketo", marketo)?;
            }
            if let Some(ref redshift) = self.redshift {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Redshift", redshift)?;
            }
            if let Some(ref s3) = self.s3 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3", s3)?;
            }
            if let Some(ref sapo_data) = self.sapo_data {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SAPOData", sapo_data)?;
            }
            if let Some(ref salesforce) = self.salesforce {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Salesforce", salesforce)?;
            }
            if let Some(ref snowflake) = self.snowflake {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Snowflake", snowflake)?;
            }
            if let Some(ref upsolver) = self.upsolver {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Upsolver", upsolver)?;
            }
            if let Some(ref zendesk) = self.zendesk {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Zendesk", zendesk)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DestinationConnectorProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DestinationConnectorProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DestinationConnectorProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DestinationConnectorProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut custom_connector: Option<::Value<CustomConnectorDestinationProperties>> = None;
                    let mut event_bridge: Option<::Value<EventBridgeDestinationProperties>> = None;
                    let mut lookout_metrics: Option<::Value<LookoutMetricsDestinationProperties>> = None;
                    let mut marketo: Option<::Value<MarketoDestinationProperties>> = None;
                    let mut redshift: Option<::Value<RedshiftDestinationProperties>> = None;
                    let mut s3: Option<::Value<S3DestinationProperties>> = None;
                    let mut sapo_data: Option<::Value<SAPODataDestinationProperties>> = None;
                    let mut salesforce: Option<::Value<SalesforceDestinationProperties>> = None;
                    let mut snowflake: Option<::Value<SnowflakeDestinationProperties>> = None;
                    let mut upsolver: Option<::Value<UpsolverDestinationProperties>> = None;
                    let mut zendesk: Option<::Value<ZendeskDestinationProperties>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CustomConnector" => {
                                custom_connector = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EventBridge" => {
                                event_bridge = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LookoutMetrics" => {
                                lookout_metrics = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Marketo" => {
                                marketo = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Redshift" => {
                                redshift = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3" => {
                                s3 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SAPOData" => {
                                sapo_data = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Salesforce" => {
                                salesforce = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Snowflake" => {
                                snowflake = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Upsolver" => {
                                upsolver = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Zendesk" => {
                                zendesk = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DestinationConnectorProperties {
                        custom_connector: custom_connector,
                        event_bridge: event_bridge,
                        lookout_metrics: lookout_metrics,
                        marketo: marketo,
                        redshift: redshift,
                        s3: s3,
                        sapo_data: sapo_data,
                        salesforce: salesforce,
                        snowflake: snowflake,
                        upsolver: upsolver,
                        zendesk: zendesk,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.DestinationFlowConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-destinationflowconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct DestinationFlowConfig {
        /// Property [`ApiVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-destinationflowconfig.html#cfn-appflow-flow-destinationflowconfig-apiversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub api_version: Option<::Value<String>>,
        /// Property [`ConnectorProfileName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-destinationflowconfig.html#cfn-appflow-flow-destinationflowconfig-connectorprofilename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connector_profile_name: Option<::Value<String>>,
        /// Property [`ConnectorType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-destinationflowconfig.html#cfn-appflow-flow-destinationflowconfig-connectortype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connector_type: ::Value<String>,
        /// Property [`DestinationConnectorProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-destinationflowconfig.html#cfn-appflow-flow-destinationflowconfig-destinationconnectorproperties).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination_connector_properties: ::Value<DestinationConnectorProperties>,
    }

    impl ::codec::SerializeValue for DestinationFlowConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref api_version) = self.api_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiVersion", api_version)?;
            }
            if let Some(ref connector_profile_name) = self.connector_profile_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectorProfileName", connector_profile_name)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectorType", &self.connector_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationConnectorProperties", &self.destination_connector_properties)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DestinationFlowConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DestinationFlowConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DestinationFlowConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DestinationFlowConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut api_version: Option<::Value<String>> = None;
                    let mut connector_profile_name: Option<::Value<String>> = None;
                    let mut connector_type: Option<::Value<String>> = None;
                    let mut destination_connector_properties: Option<::Value<DestinationConnectorProperties>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ApiVersion" => {
                                api_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConnectorProfileName" => {
                                connector_profile_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConnectorType" => {
                                connector_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DestinationConnectorProperties" => {
                                destination_connector_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DestinationFlowConfig {
                        api_version: api_version,
                        connector_profile_name: connector_profile_name,
                        connector_type: connector_type.ok_or(::serde::de::Error::missing_field("ConnectorType"))?,
                        destination_connector_properties: destination_connector_properties.ok_or(::serde::de::Error::missing_field("DestinationConnectorProperties"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.DynatraceSourceProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-dynatracesourceproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct DynatraceSourceProperties {
        /// Property [`Object`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-dynatracesourceproperties.html#cfn-appflow-flow-dynatracesourceproperties-object).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object: ::Value<String>,
    }

    impl ::codec::SerializeValue for DynatraceSourceProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Object", &self.object)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DynatraceSourceProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DynatraceSourceProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DynatraceSourceProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DynatraceSourceProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut object: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Object" => {
                                object = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DynatraceSourceProperties {
                        object: object.ok_or(::serde::de::Error::missing_field("Object"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.ErrorHandlingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-errorhandlingconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ErrorHandlingConfig {
        /// Property [`BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-errorhandlingconfig.html#cfn-appflow-flow-errorhandlingconfig-bucketname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_name: Option<::Value<String>>,
        /// Property [`BucketPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-errorhandlingconfig.html#cfn-appflow-flow-errorhandlingconfig-bucketprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_prefix: Option<::Value<String>>,
        /// Property [`FailOnFirstError`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-errorhandlingconfig.html#cfn-appflow-flow-errorhandlingconfig-failonfirsterror).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub fail_on_first_error: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for ErrorHandlingConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref bucket_name) = self.bucket_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketName", bucket_name)?;
            }
            if let Some(ref bucket_prefix) = self.bucket_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketPrefix", bucket_prefix)?;
            }
            if let Some(ref fail_on_first_error) = self.fail_on_first_error {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FailOnFirstError", fail_on_first_error)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ErrorHandlingConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ErrorHandlingConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ErrorHandlingConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ErrorHandlingConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket_name: Option<::Value<String>> = None;
                    let mut bucket_prefix: Option<::Value<String>> = None;
                    let mut fail_on_first_error: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketName" => {
                                bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BucketPrefix" => {
                                bucket_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FailOnFirstError" => {
                                fail_on_first_error = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ErrorHandlingConfig {
                        bucket_name: bucket_name,
                        bucket_prefix: bucket_prefix,
                        fail_on_first_error: fail_on_first_error,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.EventBridgeDestinationProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-eventbridgedestinationproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct EventBridgeDestinationProperties {
        /// Property [`ErrorHandlingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-eventbridgedestinationproperties.html#cfn-appflow-flow-eventbridgedestinationproperties-errorhandlingconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub error_handling_config: Option<::Value<ErrorHandlingConfig>>,
        /// Property [`Object`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-eventbridgedestinationproperties.html#cfn-appflow-flow-eventbridgedestinationproperties-object).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object: ::Value<String>,
    }

    impl ::codec::SerializeValue for EventBridgeDestinationProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref error_handling_config) = self.error_handling_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ErrorHandlingConfig", error_handling_config)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Object", &self.object)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EventBridgeDestinationProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EventBridgeDestinationProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EventBridgeDestinationProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EventBridgeDestinationProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut error_handling_config: Option<::Value<ErrorHandlingConfig>> = None;
                    let mut object: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ErrorHandlingConfig" => {
                                error_handling_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Object" => {
                                object = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EventBridgeDestinationProperties {
                        error_handling_config: error_handling_config,
                        object: object.ok_or(::serde::de::Error::missing_field("Object"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.GlueDataCatalog`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-gluedatacatalog.html) property type.
    #[derive(Debug, Default)]
    pub struct GlueDataCatalog {
        /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-gluedatacatalog.html#cfn-appflow-flow-gluedatacatalog-databasename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_name: ::Value<String>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-gluedatacatalog.html#cfn-appflow-flow-gluedatacatalog-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`TablePrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-gluedatacatalog.html#cfn-appflow-flow-gluedatacatalog-tableprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub table_prefix: ::Value<String>,
    }

    impl ::codec::SerializeValue for GlueDataCatalog {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", &self.database_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TablePrefix", &self.table_prefix)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GlueDataCatalog {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GlueDataCatalog, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GlueDataCatalog;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GlueDataCatalog")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut database_name: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut table_prefix: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DatabaseName" => {
                                database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TablePrefix" => {
                                table_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GlueDataCatalog {
                        database_name: database_name.ok_or(::serde::de::Error::missing_field("DatabaseName"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                        table_prefix: table_prefix.ok_or(::serde::de::Error::missing_field("TablePrefix"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.GoogleAnalyticsSourceProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-googleanalyticssourceproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct GoogleAnalyticsSourceProperties {
        /// Property [`Object`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-googleanalyticssourceproperties.html#cfn-appflow-flow-googleanalyticssourceproperties-object).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object: ::Value<String>,
    }

    impl ::codec::SerializeValue for GoogleAnalyticsSourceProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Object", &self.object)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GoogleAnalyticsSourceProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GoogleAnalyticsSourceProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GoogleAnalyticsSourceProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GoogleAnalyticsSourceProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut object: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Object" => {
                                object = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GoogleAnalyticsSourceProperties {
                        object: object.ok_or(::serde::de::Error::missing_field("Object"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.IncrementalPullConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-incrementalpullconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct IncrementalPullConfig {
        /// Property [`DatetimeTypeFieldName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-incrementalpullconfig.html#cfn-appflow-flow-incrementalpullconfig-datetimetypefieldname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub datetime_type_field_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for IncrementalPullConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref datetime_type_field_name) = self.datetime_type_field_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatetimeTypeFieldName", datetime_type_field_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IncrementalPullConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IncrementalPullConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IncrementalPullConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IncrementalPullConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut datetime_type_field_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DatetimeTypeFieldName" => {
                                datetime_type_field_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IncrementalPullConfig {
                        datetime_type_field_name: datetime_type_field_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.InforNexusSourceProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-infornexussourceproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct InforNexusSourceProperties {
        /// Property [`Object`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-infornexussourceproperties.html#cfn-appflow-flow-infornexussourceproperties-object).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object: ::Value<String>,
    }

    impl ::codec::SerializeValue for InforNexusSourceProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Object", &self.object)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InforNexusSourceProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InforNexusSourceProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InforNexusSourceProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InforNexusSourceProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut object: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Object" => {
                                object = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InforNexusSourceProperties {
                        object: object.ok_or(::serde::de::Error::missing_field("Object"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.LookoutMetricsDestinationProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-lookoutmetricsdestinationproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct LookoutMetricsDestinationProperties {
        /// Property [`Object`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-lookoutmetricsdestinationproperties.html#cfn-appflow-flow-lookoutmetricsdestinationproperties-object).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LookoutMetricsDestinationProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref object) = self.object {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Object", object)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LookoutMetricsDestinationProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LookoutMetricsDestinationProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LookoutMetricsDestinationProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LookoutMetricsDestinationProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut object: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Object" => {
                                object = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LookoutMetricsDestinationProperties {
                        object: object,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.MarketoDestinationProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-marketodestinationproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct MarketoDestinationProperties {
        /// Property [`ErrorHandlingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-marketodestinationproperties.html#cfn-appflow-flow-marketodestinationproperties-errorhandlingconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub error_handling_config: Option<::Value<ErrorHandlingConfig>>,
        /// Property [`Object`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-marketodestinationproperties.html#cfn-appflow-flow-marketodestinationproperties-object).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object: ::Value<String>,
    }

    impl ::codec::SerializeValue for MarketoDestinationProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref error_handling_config) = self.error_handling_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ErrorHandlingConfig", error_handling_config)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Object", &self.object)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MarketoDestinationProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MarketoDestinationProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MarketoDestinationProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MarketoDestinationProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut error_handling_config: Option<::Value<ErrorHandlingConfig>> = None;
                    let mut object: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ErrorHandlingConfig" => {
                                error_handling_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Object" => {
                                object = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MarketoDestinationProperties {
                        error_handling_config: error_handling_config,
                        object: object.ok_or(::serde::de::Error::missing_field("Object"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.MarketoSourceProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-marketosourceproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct MarketoSourceProperties {
        /// Property [`Object`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-marketosourceproperties.html#cfn-appflow-flow-marketosourceproperties-object).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object: ::Value<String>,
    }

    impl ::codec::SerializeValue for MarketoSourceProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Object", &self.object)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MarketoSourceProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MarketoSourceProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MarketoSourceProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MarketoSourceProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut object: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Object" => {
                                object = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MarketoSourceProperties {
                        object: object.ok_or(::serde::de::Error::missing_field("Object"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.MetadataCatalogConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-metadatacatalogconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct MetadataCatalogConfig {
        /// Property [`GlueDataCatalog`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-metadatacatalogconfig.html#cfn-appflow-flow-metadatacatalogconfig-gluedatacatalog).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub glue_data_catalog: Option<::Value<GlueDataCatalog>>,
    }

    impl ::codec::SerializeValue for MetadataCatalogConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref glue_data_catalog) = self.glue_data_catalog {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GlueDataCatalog", glue_data_catalog)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MetadataCatalogConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MetadataCatalogConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MetadataCatalogConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MetadataCatalogConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut glue_data_catalog: Option<::Value<GlueDataCatalog>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "GlueDataCatalog" => {
                                glue_data_catalog = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MetadataCatalogConfig {
                        glue_data_catalog: glue_data_catalog,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.PardotSourceProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-pardotsourceproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct PardotSourceProperties {
        /// Property [`Object`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-pardotsourceproperties.html#cfn-appflow-flow-pardotsourceproperties-object).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object: ::Value<String>,
    }

    impl ::codec::SerializeValue for PardotSourceProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Object", &self.object)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PardotSourceProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PardotSourceProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PardotSourceProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PardotSourceProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut object: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Object" => {
                                object = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PardotSourceProperties {
                        object: object.ok_or(::serde::de::Error::missing_field("Object"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.PrefixConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-prefixconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct PrefixConfig {
        /// Property [`PathPrefixHierarchy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-prefixconfig.html#cfn-appflow-flow-prefixconfig-pathprefixhierarchy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub path_prefix_hierarchy: Option<::ValueList<String>>,
        /// Property [`PrefixFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-prefixconfig.html#cfn-appflow-flow-prefixconfig-prefixformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub prefix_format: Option<::Value<String>>,
        /// Property [`PrefixType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-prefixconfig.html#cfn-appflow-flow-prefixconfig-prefixtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub prefix_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for PrefixConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref path_prefix_hierarchy) = self.path_prefix_hierarchy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PathPrefixHierarchy", path_prefix_hierarchy)?;
            }
            if let Some(ref prefix_format) = self.prefix_format {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrefixFormat", prefix_format)?;
            }
            if let Some(ref prefix_type) = self.prefix_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrefixType", prefix_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PrefixConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PrefixConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PrefixConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PrefixConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut path_prefix_hierarchy: Option<::ValueList<String>> = None;
                    let mut prefix_format: Option<::Value<String>> = None;
                    let mut prefix_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PathPrefixHierarchy" => {
                                path_prefix_hierarchy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PrefixFormat" => {
                                prefix_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PrefixType" => {
                                prefix_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PrefixConfig {
                        path_prefix_hierarchy: path_prefix_hierarchy,
                        prefix_format: prefix_format,
                        prefix_type: prefix_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.RedshiftDestinationProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-redshiftdestinationproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct RedshiftDestinationProperties {
        /// Property [`BucketPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-redshiftdestinationproperties.html#cfn-appflow-flow-redshiftdestinationproperties-bucketprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_prefix: Option<::Value<String>>,
        /// Property [`ErrorHandlingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-redshiftdestinationproperties.html#cfn-appflow-flow-redshiftdestinationproperties-errorhandlingconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub error_handling_config: Option<::Value<ErrorHandlingConfig>>,
        /// Property [`IntermediateBucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-redshiftdestinationproperties.html#cfn-appflow-flow-redshiftdestinationproperties-intermediatebucketname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub intermediate_bucket_name: ::Value<String>,
        /// Property [`Object`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-redshiftdestinationproperties.html#cfn-appflow-flow-redshiftdestinationproperties-object).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object: ::Value<String>,
    }

    impl ::codec::SerializeValue for RedshiftDestinationProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref bucket_prefix) = self.bucket_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketPrefix", bucket_prefix)?;
            }
            if let Some(ref error_handling_config) = self.error_handling_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ErrorHandlingConfig", error_handling_config)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntermediateBucketName", &self.intermediate_bucket_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Object", &self.object)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RedshiftDestinationProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RedshiftDestinationProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RedshiftDestinationProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RedshiftDestinationProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket_prefix: Option<::Value<String>> = None;
                    let mut error_handling_config: Option<::Value<ErrorHandlingConfig>> = None;
                    let mut intermediate_bucket_name: Option<::Value<String>> = None;
                    let mut object: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketPrefix" => {
                                bucket_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ErrorHandlingConfig" => {
                                error_handling_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IntermediateBucketName" => {
                                intermediate_bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Object" => {
                                object = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RedshiftDestinationProperties {
                        bucket_prefix: bucket_prefix,
                        error_handling_config: error_handling_config,
                        intermediate_bucket_name: intermediate_bucket_name.ok_or(::serde::de::Error::missing_field("IntermediateBucketName"))?,
                        object: object.ok_or(::serde::de::Error::missing_field("Object"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.S3DestinationProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-s3destinationproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct S3DestinationProperties {
        /// Property [`BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-s3destinationproperties.html#cfn-appflow-flow-s3destinationproperties-bucketname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_name: ::Value<String>,
        /// Property [`BucketPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-s3destinationproperties.html#cfn-appflow-flow-s3destinationproperties-bucketprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_prefix: Option<::Value<String>>,
        /// Property [`S3OutputFormatConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-s3destinationproperties.html#cfn-appflow-flow-s3destinationproperties-s3outputformatconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_output_format_config: Option<::Value<S3OutputFormatConfig>>,
    }

    impl ::codec::SerializeValue for S3DestinationProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketName", &self.bucket_name)?;
            if let Some(ref bucket_prefix) = self.bucket_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketPrefix", bucket_prefix)?;
            }
            if let Some(ref s3_output_format_config) = self.s3_output_format_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3OutputFormatConfig", s3_output_format_config)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3DestinationProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3DestinationProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3DestinationProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3DestinationProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket_name: Option<::Value<String>> = None;
                    let mut bucket_prefix: Option<::Value<String>> = None;
                    let mut s3_output_format_config: Option<::Value<S3OutputFormatConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketName" => {
                                bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BucketPrefix" => {
                                bucket_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3OutputFormatConfig" => {
                                s3_output_format_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3DestinationProperties {
                        bucket_name: bucket_name.ok_or(::serde::de::Error::missing_field("BucketName"))?,
                        bucket_prefix: bucket_prefix,
                        s3_output_format_config: s3_output_format_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.S3InputFormatConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-s3inputformatconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct S3InputFormatConfig {
        /// Property [`S3InputFileType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-s3inputformatconfig.html#cfn-appflow-flow-s3inputformatconfig-s3inputfiletype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_input_file_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for S3InputFormatConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref s3_input_file_type) = self.s3_input_file_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3InputFileType", s3_input_file_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3InputFormatConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3InputFormatConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3InputFormatConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3InputFormatConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut s3_input_file_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3InputFileType" => {
                                s3_input_file_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3InputFormatConfig {
                        s3_input_file_type: s3_input_file_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.S3OutputFormatConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-s3outputformatconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct S3OutputFormatConfig {
        /// Property [`AggregationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-s3outputformatconfig.html#cfn-appflow-flow-s3outputformatconfig-aggregationconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aggregation_config: Option<::Value<AggregationConfig>>,
        /// Property [`FileType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-s3outputformatconfig.html#cfn-appflow-flow-s3outputformatconfig-filetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub file_type: Option<::Value<String>>,
        /// Property [`PrefixConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-s3outputformatconfig.html#cfn-appflow-flow-s3outputformatconfig-prefixconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub prefix_config: Option<::Value<PrefixConfig>>,
        /// Property [`PreserveSourceDataTyping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-s3outputformatconfig.html#cfn-appflow-flow-s3outputformatconfig-preservesourcedatatyping).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub preserve_source_data_typing: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for S3OutputFormatConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref aggregation_config) = self.aggregation_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AggregationConfig", aggregation_config)?;
            }
            if let Some(ref file_type) = self.file_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FileType", file_type)?;
            }
            if let Some(ref prefix_config) = self.prefix_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrefixConfig", prefix_config)?;
            }
            if let Some(ref preserve_source_data_typing) = self.preserve_source_data_typing {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreserveSourceDataTyping", preserve_source_data_typing)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3OutputFormatConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3OutputFormatConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3OutputFormatConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3OutputFormatConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut aggregation_config: Option<::Value<AggregationConfig>> = None;
                    let mut file_type: Option<::Value<String>> = None;
                    let mut prefix_config: Option<::Value<PrefixConfig>> = None;
                    let mut preserve_source_data_typing: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AggregationConfig" => {
                                aggregation_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FileType" => {
                                file_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PrefixConfig" => {
                                prefix_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PreserveSourceDataTyping" => {
                                preserve_source_data_typing = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3OutputFormatConfig {
                        aggregation_config: aggregation_config,
                        file_type: file_type,
                        prefix_config: prefix_config,
                        preserve_source_data_typing: preserve_source_data_typing,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.S3SourceProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-s3sourceproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct S3SourceProperties {
        /// Property [`BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-s3sourceproperties.html#cfn-appflow-flow-s3sourceproperties-bucketname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_name: ::Value<String>,
        /// Property [`BucketPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-s3sourceproperties.html#cfn-appflow-flow-s3sourceproperties-bucketprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_prefix: ::Value<String>,
        /// Property [`S3InputFormatConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-s3sourceproperties.html#cfn-appflow-flow-s3sourceproperties-s3inputformatconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_input_format_config: Option<::Value<S3InputFormatConfig>>,
    }

    impl ::codec::SerializeValue for S3SourceProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketName", &self.bucket_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketPrefix", &self.bucket_prefix)?;
            if let Some(ref s3_input_format_config) = self.s3_input_format_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3InputFormatConfig", s3_input_format_config)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3SourceProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3SourceProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3SourceProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3SourceProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket_name: Option<::Value<String>> = None;
                    let mut bucket_prefix: Option<::Value<String>> = None;
                    let mut s3_input_format_config: Option<::Value<S3InputFormatConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketName" => {
                                bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BucketPrefix" => {
                                bucket_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3InputFormatConfig" => {
                                s3_input_format_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3SourceProperties {
                        bucket_name: bucket_name.ok_or(::serde::de::Error::missing_field("BucketName"))?,
                        bucket_prefix: bucket_prefix.ok_or(::serde::de::Error::missing_field("BucketPrefix"))?,
                        s3_input_format_config: s3_input_format_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.SAPODataDestinationProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sapodatadestinationproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct SAPODataDestinationProperties {
        /// Property [`ErrorHandlingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sapodatadestinationproperties.html#cfn-appflow-flow-sapodatadestinationproperties-errorhandlingconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub error_handling_config: Option<::Value<ErrorHandlingConfig>>,
        /// Property [`IdFieldNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sapodatadestinationproperties.html#cfn-appflow-flow-sapodatadestinationproperties-idfieldnames).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id_field_names: Option<::ValueList<String>>,
        /// Property [`ObjectPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sapodatadestinationproperties.html#cfn-appflow-flow-sapodatadestinationproperties-objectpath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object_path: ::Value<String>,
        /// Property [`SuccessResponseHandlingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sapodatadestinationproperties.html#cfn-appflow-flow-sapodatadestinationproperties-successresponsehandlingconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub success_response_handling_config: Option<::Value<SuccessResponseHandlingConfig>>,
        /// Property [`WriteOperationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sapodatadestinationproperties.html#cfn-appflow-flow-sapodatadestinationproperties-writeoperationtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub write_operation_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SAPODataDestinationProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref error_handling_config) = self.error_handling_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ErrorHandlingConfig", error_handling_config)?;
            }
            if let Some(ref id_field_names) = self.id_field_names {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdFieldNames", id_field_names)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ObjectPath", &self.object_path)?;
            if let Some(ref success_response_handling_config) = self.success_response_handling_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SuccessResponseHandlingConfig", success_response_handling_config)?;
            }
            if let Some(ref write_operation_type) = self.write_operation_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WriteOperationType", write_operation_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SAPODataDestinationProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SAPODataDestinationProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SAPODataDestinationProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SAPODataDestinationProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut error_handling_config: Option<::Value<ErrorHandlingConfig>> = None;
                    let mut id_field_names: Option<::ValueList<String>> = None;
                    let mut object_path: Option<::Value<String>> = None;
                    let mut success_response_handling_config: Option<::Value<SuccessResponseHandlingConfig>> = None;
                    let mut write_operation_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ErrorHandlingConfig" => {
                                error_handling_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IdFieldNames" => {
                                id_field_names = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ObjectPath" => {
                                object_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SuccessResponseHandlingConfig" => {
                                success_response_handling_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WriteOperationType" => {
                                write_operation_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SAPODataDestinationProperties {
                        error_handling_config: error_handling_config,
                        id_field_names: id_field_names,
                        object_path: object_path.ok_or(::serde::de::Error::missing_field("ObjectPath"))?,
                        success_response_handling_config: success_response_handling_config,
                        write_operation_type: write_operation_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.SAPODataPaginationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sapodatapaginationconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct SAPODataPaginationConfig {
        /// Property [`maxPageSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sapodatapaginationconfig.html#cfn-appflow-flow-sapodatapaginationconfig-maxpagesize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_page_size: ::Value<u32>,
    }

    impl ::codec::SerializeValue for SAPODataPaginationConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "maxPageSize", &self.max_page_size)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SAPODataPaginationConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SAPODataPaginationConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SAPODataPaginationConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SAPODataPaginationConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut max_page_size: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "maxPageSize" => {
                                max_page_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SAPODataPaginationConfig {
                        max_page_size: max_page_size.ok_or(::serde::de::Error::missing_field("maxPageSize"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.SAPODataParallelismConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sapodataparallelismconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct SAPODataParallelismConfig {
        /// Property [`maxParallelism`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sapodataparallelismconfig.html#cfn-appflow-flow-sapodataparallelismconfig-maxparallelism).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_parallelism: ::Value<u32>,
    }

    impl ::codec::SerializeValue for SAPODataParallelismConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "maxParallelism", &self.max_parallelism)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SAPODataParallelismConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SAPODataParallelismConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SAPODataParallelismConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SAPODataParallelismConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut max_parallelism: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "maxParallelism" => {
                                max_parallelism = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SAPODataParallelismConfig {
                        max_parallelism: max_parallelism.ok_or(::serde::de::Error::missing_field("maxParallelism"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.SAPODataSourceProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sapodatasourceproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct SAPODataSourceProperties {
        /// Property [`ObjectPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sapodatasourceproperties.html#cfn-appflow-flow-sapodatasourceproperties-objectpath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object_path: ::Value<String>,
        /// Property [`paginationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sapodatasourceproperties.html#cfn-appflow-flow-sapodatasourceproperties-paginationconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pagination_config: Option<::Value<SAPODataPaginationConfig>>,
        /// Property [`parallelismConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sapodatasourceproperties.html#cfn-appflow-flow-sapodatasourceproperties-parallelismconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parallelism_config: Option<::Value<SAPODataParallelismConfig>>,
    }

    impl ::codec::SerializeValue for SAPODataSourceProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ObjectPath", &self.object_path)?;
            if let Some(ref pagination_config) = self.pagination_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "paginationConfig", pagination_config)?;
            }
            if let Some(ref parallelism_config) = self.parallelism_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "parallelismConfig", parallelism_config)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SAPODataSourceProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SAPODataSourceProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SAPODataSourceProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SAPODataSourceProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut object_path: Option<::Value<String>> = None;
                    let mut pagination_config: Option<::Value<SAPODataPaginationConfig>> = None;
                    let mut parallelism_config: Option<::Value<SAPODataParallelismConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ObjectPath" => {
                                object_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "paginationConfig" => {
                                pagination_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "parallelismConfig" => {
                                parallelism_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SAPODataSourceProperties {
                        object_path: object_path.ok_or(::serde::de::Error::missing_field("ObjectPath"))?,
                        pagination_config: pagination_config,
                        parallelism_config: parallelism_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.SalesforceDestinationProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-salesforcedestinationproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct SalesforceDestinationProperties {
        /// Property [`DataTransferApi`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-salesforcedestinationproperties.html#cfn-appflow-flow-salesforcedestinationproperties-datatransferapi).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_transfer_api: Option<::Value<String>>,
        /// Property [`ErrorHandlingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-salesforcedestinationproperties.html#cfn-appflow-flow-salesforcedestinationproperties-errorhandlingconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub error_handling_config: Option<::Value<ErrorHandlingConfig>>,
        /// Property [`IdFieldNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-salesforcedestinationproperties.html#cfn-appflow-flow-salesforcedestinationproperties-idfieldnames).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id_field_names: Option<::ValueList<String>>,
        /// Property [`Object`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-salesforcedestinationproperties.html#cfn-appflow-flow-salesforcedestinationproperties-object).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object: ::Value<String>,
        /// Property [`WriteOperationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-salesforcedestinationproperties.html#cfn-appflow-flow-salesforcedestinationproperties-writeoperationtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub write_operation_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SalesforceDestinationProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref data_transfer_api) = self.data_transfer_api {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataTransferApi", data_transfer_api)?;
            }
            if let Some(ref error_handling_config) = self.error_handling_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ErrorHandlingConfig", error_handling_config)?;
            }
            if let Some(ref id_field_names) = self.id_field_names {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdFieldNames", id_field_names)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Object", &self.object)?;
            if let Some(ref write_operation_type) = self.write_operation_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WriteOperationType", write_operation_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SalesforceDestinationProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SalesforceDestinationProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SalesforceDestinationProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SalesforceDestinationProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_transfer_api: Option<::Value<String>> = None;
                    let mut error_handling_config: Option<::Value<ErrorHandlingConfig>> = None;
                    let mut id_field_names: Option<::ValueList<String>> = None;
                    let mut object: Option<::Value<String>> = None;
                    let mut write_operation_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataTransferApi" => {
                                data_transfer_api = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ErrorHandlingConfig" => {
                                error_handling_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IdFieldNames" => {
                                id_field_names = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Object" => {
                                object = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WriteOperationType" => {
                                write_operation_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SalesforceDestinationProperties {
                        data_transfer_api: data_transfer_api,
                        error_handling_config: error_handling_config,
                        id_field_names: id_field_names,
                        object: object.ok_or(::serde::de::Error::missing_field("Object"))?,
                        write_operation_type: write_operation_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.SalesforceSourceProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-salesforcesourceproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct SalesforceSourceProperties {
        /// Property [`DataTransferApi`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-salesforcesourceproperties.html#cfn-appflow-flow-salesforcesourceproperties-datatransferapi).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_transfer_api: Option<::Value<String>>,
        /// Property [`EnableDynamicFieldUpdate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-salesforcesourceproperties.html#cfn-appflow-flow-salesforcesourceproperties-enabledynamicfieldupdate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable_dynamic_field_update: Option<::Value<bool>>,
        /// Property [`IncludeDeletedRecords`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-salesforcesourceproperties.html#cfn-appflow-flow-salesforcesourceproperties-includedeletedrecords).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_deleted_records: Option<::Value<bool>>,
        /// Property [`Object`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-salesforcesourceproperties.html#cfn-appflow-flow-salesforcesourceproperties-object).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object: ::Value<String>,
    }

    impl ::codec::SerializeValue for SalesforceSourceProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref data_transfer_api) = self.data_transfer_api {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataTransferApi", data_transfer_api)?;
            }
            if let Some(ref enable_dynamic_field_update) = self.enable_dynamic_field_update {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableDynamicFieldUpdate", enable_dynamic_field_update)?;
            }
            if let Some(ref include_deleted_records) = self.include_deleted_records {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeDeletedRecords", include_deleted_records)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Object", &self.object)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SalesforceSourceProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SalesforceSourceProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SalesforceSourceProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SalesforceSourceProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_transfer_api: Option<::Value<String>> = None;
                    let mut enable_dynamic_field_update: Option<::Value<bool>> = None;
                    let mut include_deleted_records: Option<::Value<bool>> = None;
                    let mut object: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataTransferApi" => {
                                data_transfer_api = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnableDynamicFieldUpdate" => {
                                enable_dynamic_field_update = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeDeletedRecords" => {
                                include_deleted_records = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Object" => {
                                object = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SalesforceSourceProperties {
                        data_transfer_api: data_transfer_api,
                        enable_dynamic_field_update: enable_dynamic_field_update,
                        include_deleted_records: include_deleted_records,
                        object: object.ok_or(::serde::de::Error::missing_field("Object"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.ScheduledTriggerProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-scheduledtriggerproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct ScheduledTriggerProperties {
        /// Property [`DataPullMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-scheduledtriggerproperties.html#cfn-appflow-flow-scheduledtriggerproperties-datapullmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_pull_mode: Option<::Value<String>>,
        /// Property [`FirstExecutionFrom`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-scheduledtriggerproperties.html#cfn-appflow-flow-scheduledtriggerproperties-firstexecutionfrom).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub first_execution_from: Option<::Value<f64>>,
        /// Property [`FlowErrorDeactivationThreshold`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-scheduledtriggerproperties.html#cfn-appflow-flow-scheduledtriggerproperties-flowerrordeactivationthreshold).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub flow_error_deactivation_threshold: Option<::Value<u32>>,
        /// Property [`ScheduleEndTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-scheduledtriggerproperties.html#cfn-appflow-flow-scheduledtriggerproperties-scheduleendtime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schedule_end_time: Option<::Value<f64>>,
        /// Property [`ScheduleExpression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-scheduledtriggerproperties.html#cfn-appflow-flow-scheduledtriggerproperties-scheduleexpression).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schedule_expression: ::Value<String>,
        /// Property [`ScheduleOffset`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-scheduledtriggerproperties.html#cfn-appflow-flow-scheduledtriggerproperties-scheduleoffset).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schedule_offset: Option<::Value<f64>>,
        /// Property [`ScheduleStartTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-scheduledtriggerproperties.html#cfn-appflow-flow-scheduledtriggerproperties-schedulestarttime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schedule_start_time: Option<::Value<f64>>,
        /// Property [`TimeZone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-scheduledtriggerproperties.html#cfn-appflow-flow-scheduledtriggerproperties-timezone).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub time_zone: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ScheduledTriggerProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref data_pull_mode) = self.data_pull_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataPullMode", data_pull_mode)?;
            }
            if let Some(ref first_execution_from) = self.first_execution_from {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FirstExecutionFrom", first_execution_from)?;
            }
            if let Some(ref flow_error_deactivation_threshold) = self.flow_error_deactivation_threshold {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FlowErrorDeactivationThreshold", flow_error_deactivation_threshold)?;
            }
            if let Some(ref schedule_end_time) = self.schedule_end_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScheduleEndTime", schedule_end_time)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScheduleExpression", &self.schedule_expression)?;
            if let Some(ref schedule_offset) = self.schedule_offset {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScheduleOffset", schedule_offset)?;
            }
            if let Some(ref schedule_start_time) = self.schedule_start_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScheduleStartTime", schedule_start_time)?;
            }
            if let Some(ref time_zone) = self.time_zone {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeZone", time_zone)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ScheduledTriggerProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ScheduledTriggerProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ScheduledTriggerProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ScheduledTriggerProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_pull_mode: Option<::Value<String>> = None;
                    let mut first_execution_from: Option<::Value<f64>> = None;
                    let mut flow_error_deactivation_threshold: Option<::Value<u32>> = None;
                    let mut schedule_end_time: Option<::Value<f64>> = None;
                    let mut schedule_expression: Option<::Value<String>> = None;
                    let mut schedule_offset: Option<::Value<f64>> = None;
                    let mut schedule_start_time: Option<::Value<f64>> = None;
                    let mut time_zone: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataPullMode" => {
                                data_pull_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FirstExecutionFrom" => {
                                first_execution_from = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FlowErrorDeactivationThreshold" => {
                                flow_error_deactivation_threshold = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScheduleEndTime" => {
                                schedule_end_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScheduleExpression" => {
                                schedule_expression = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScheduleOffset" => {
                                schedule_offset = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScheduleStartTime" => {
                                schedule_start_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimeZone" => {
                                time_zone = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ScheduledTriggerProperties {
                        data_pull_mode: data_pull_mode,
                        first_execution_from: first_execution_from,
                        flow_error_deactivation_threshold: flow_error_deactivation_threshold,
                        schedule_end_time: schedule_end_time,
                        schedule_expression: schedule_expression.ok_or(::serde::de::Error::missing_field("ScheduleExpression"))?,
                        schedule_offset: schedule_offset,
                        schedule_start_time: schedule_start_time,
                        time_zone: time_zone,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.ServiceNowSourceProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-servicenowsourceproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct ServiceNowSourceProperties {
        /// Property [`Object`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-servicenowsourceproperties.html#cfn-appflow-flow-servicenowsourceproperties-object).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object: ::Value<String>,
    }

    impl ::codec::SerializeValue for ServiceNowSourceProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Object", &self.object)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ServiceNowSourceProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ServiceNowSourceProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ServiceNowSourceProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ServiceNowSourceProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut object: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Object" => {
                                object = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ServiceNowSourceProperties {
                        object: object.ok_or(::serde::de::Error::missing_field("Object"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.SingularSourceProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-singularsourceproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct SingularSourceProperties {
        /// Property [`Object`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-singularsourceproperties.html#cfn-appflow-flow-singularsourceproperties-object).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object: ::Value<String>,
    }

    impl ::codec::SerializeValue for SingularSourceProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Object", &self.object)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SingularSourceProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SingularSourceProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SingularSourceProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SingularSourceProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut object: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Object" => {
                                object = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SingularSourceProperties {
                        object: object.ok_or(::serde::de::Error::missing_field("Object"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.SlackSourceProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-slacksourceproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct SlackSourceProperties {
        /// Property [`Object`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-slacksourceproperties.html#cfn-appflow-flow-slacksourceproperties-object).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object: ::Value<String>,
    }

    impl ::codec::SerializeValue for SlackSourceProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Object", &self.object)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SlackSourceProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SlackSourceProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SlackSourceProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SlackSourceProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut object: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Object" => {
                                object = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SlackSourceProperties {
                        object: object.ok_or(::serde::de::Error::missing_field("Object"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.SnowflakeDestinationProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-snowflakedestinationproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct SnowflakeDestinationProperties {
        /// Property [`BucketPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-snowflakedestinationproperties.html#cfn-appflow-flow-snowflakedestinationproperties-bucketprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_prefix: Option<::Value<String>>,
        /// Property [`ErrorHandlingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-snowflakedestinationproperties.html#cfn-appflow-flow-snowflakedestinationproperties-errorhandlingconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub error_handling_config: Option<::Value<ErrorHandlingConfig>>,
        /// Property [`IntermediateBucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-snowflakedestinationproperties.html#cfn-appflow-flow-snowflakedestinationproperties-intermediatebucketname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub intermediate_bucket_name: ::Value<String>,
        /// Property [`Object`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-snowflakedestinationproperties.html#cfn-appflow-flow-snowflakedestinationproperties-object).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object: ::Value<String>,
    }

    impl ::codec::SerializeValue for SnowflakeDestinationProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref bucket_prefix) = self.bucket_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketPrefix", bucket_prefix)?;
            }
            if let Some(ref error_handling_config) = self.error_handling_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ErrorHandlingConfig", error_handling_config)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntermediateBucketName", &self.intermediate_bucket_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Object", &self.object)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SnowflakeDestinationProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SnowflakeDestinationProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SnowflakeDestinationProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SnowflakeDestinationProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket_prefix: Option<::Value<String>> = None;
                    let mut error_handling_config: Option<::Value<ErrorHandlingConfig>> = None;
                    let mut intermediate_bucket_name: Option<::Value<String>> = None;
                    let mut object: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketPrefix" => {
                                bucket_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ErrorHandlingConfig" => {
                                error_handling_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IntermediateBucketName" => {
                                intermediate_bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Object" => {
                                object = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SnowflakeDestinationProperties {
                        bucket_prefix: bucket_prefix,
                        error_handling_config: error_handling_config,
                        intermediate_bucket_name: intermediate_bucket_name.ok_or(::serde::de::Error::missing_field("IntermediateBucketName"))?,
                        object: object.ok_or(::serde::de::Error::missing_field("Object"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.SourceConnectorProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sourceconnectorproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct SourceConnectorProperties {
        /// Property [`Amplitude`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sourceconnectorproperties.html#cfn-appflow-flow-sourceconnectorproperties-amplitude).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub amplitude: Option<::Value<AmplitudeSourceProperties>>,
        /// Property [`CustomConnector`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sourceconnectorproperties.html#cfn-appflow-flow-sourceconnectorproperties-customconnector).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_connector: Option<::Value<CustomConnectorSourceProperties>>,
        /// Property [`Datadog`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sourceconnectorproperties.html#cfn-appflow-flow-sourceconnectorproperties-datadog).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub datadog: Option<::Value<DatadogSourceProperties>>,
        /// Property [`Dynatrace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sourceconnectorproperties.html#cfn-appflow-flow-sourceconnectorproperties-dynatrace).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dynatrace: Option<::Value<DynatraceSourceProperties>>,
        /// Property [`GoogleAnalytics`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sourceconnectorproperties.html#cfn-appflow-flow-sourceconnectorproperties-googleanalytics).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub google_analytics: Option<::Value<GoogleAnalyticsSourceProperties>>,
        /// Property [`InforNexus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sourceconnectorproperties.html#cfn-appflow-flow-sourceconnectorproperties-infornexus).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub infor_nexus: Option<::Value<InforNexusSourceProperties>>,
        /// Property [`Marketo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sourceconnectorproperties.html#cfn-appflow-flow-sourceconnectorproperties-marketo).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub marketo: Option<::Value<MarketoSourceProperties>>,
        /// Property [`Pardot`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sourceconnectorproperties.html#cfn-appflow-flow-sourceconnectorproperties-pardot).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pardot: Option<::Value<PardotSourceProperties>>,
        /// Property [`S3`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sourceconnectorproperties.html#cfn-appflow-flow-sourceconnectorproperties-s3).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3: Option<::Value<S3SourceProperties>>,
        /// Property [`SAPOData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sourceconnectorproperties.html#cfn-appflow-flow-sourceconnectorproperties-sapodata).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sapo_data: Option<::Value<SAPODataSourceProperties>>,
        /// Property [`Salesforce`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sourceconnectorproperties.html#cfn-appflow-flow-sourceconnectorproperties-salesforce).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub salesforce: Option<::Value<SalesforceSourceProperties>>,
        /// Property [`ServiceNow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sourceconnectorproperties.html#cfn-appflow-flow-sourceconnectorproperties-servicenow).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub service_now: Option<::Value<ServiceNowSourceProperties>>,
        /// Property [`Singular`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sourceconnectorproperties.html#cfn-appflow-flow-sourceconnectorproperties-singular).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub singular: Option<::Value<SingularSourceProperties>>,
        /// Property [`Slack`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sourceconnectorproperties.html#cfn-appflow-flow-sourceconnectorproperties-slack).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub slack: Option<::Value<SlackSourceProperties>>,
        /// Property [`Trendmicro`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sourceconnectorproperties.html#cfn-appflow-flow-sourceconnectorproperties-trendmicro).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub trendmicro: Option<::Value<TrendmicroSourceProperties>>,
        /// Property [`Veeva`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sourceconnectorproperties.html#cfn-appflow-flow-sourceconnectorproperties-veeva).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub veeva: Option<::Value<VeevaSourceProperties>>,
        /// Property [`Zendesk`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sourceconnectorproperties.html#cfn-appflow-flow-sourceconnectorproperties-zendesk).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub zendesk: Option<::Value<ZendeskSourceProperties>>,
    }

    impl ::codec::SerializeValue for SourceConnectorProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref amplitude) = self.amplitude {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Amplitude", amplitude)?;
            }
            if let Some(ref custom_connector) = self.custom_connector {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomConnector", custom_connector)?;
            }
            if let Some(ref datadog) = self.datadog {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Datadog", datadog)?;
            }
            if let Some(ref dynatrace) = self.dynatrace {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Dynatrace", dynatrace)?;
            }
            if let Some(ref google_analytics) = self.google_analytics {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GoogleAnalytics", google_analytics)?;
            }
            if let Some(ref infor_nexus) = self.infor_nexus {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InforNexus", infor_nexus)?;
            }
            if let Some(ref marketo) = self.marketo {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Marketo", marketo)?;
            }
            if let Some(ref pardot) = self.pardot {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Pardot", pardot)?;
            }
            if let Some(ref s3) = self.s3 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3", s3)?;
            }
            if let Some(ref sapo_data) = self.sapo_data {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SAPOData", sapo_data)?;
            }
            if let Some(ref salesforce) = self.salesforce {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Salesforce", salesforce)?;
            }
            if let Some(ref service_now) = self.service_now {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceNow", service_now)?;
            }
            if let Some(ref singular) = self.singular {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Singular", singular)?;
            }
            if let Some(ref slack) = self.slack {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Slack", slack)?;
            }
            if let Some(ref trendmicro) = self.trendmicro {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Trendmicro", trendmicro)?;
            }
            if let Some(ref veeva) = self.veeva {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Veeva", veeva)?;
            }
            if let Some(ref zendesk) = self.zendesk {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Zendesk", zendesk)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SourceConnectorProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SourceConnectorProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SourceConnectorProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SourceConnectorProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut amplitude: Option<::Value<AmplitudeSourceProperties>> = None;
                    let mut custom_connector: Option<::Value<CustomConnectorSourceProperties>> = None;
                    let mut datadog: Option<::Value<DatadogSourceProperties>> = None;
                    let mut dynatrace: Option<::Value<DynatraceSourceProperties>> = None;
                    let mut google_analytics: Option<::Value<GoogleAnalyticsSourceProperties>> = None;
                    let mut infor_nexus: Option<::Value<InforNexusSourceProperties>> = None;
                    let mut marketo: Option<::Value<MarketoSourceProperties>> = None;
                    let mut pardot: Option<::Value<PardotSourceProperties>> = None;
                    let mut s3: Option<::Value<S3SourceProperties>> = None;
                    let mut sapo_data: Option<::Value<SAPODataSourceProperties>> = None;
                    let mut salesforce: Option<::Value<SalesforceSourceProperties>> = None;
                    let mut service_now: Option<::Value<ServiceNowSourceProperties>> = None;
                    let mut singular: Option<::Value<SingularSourceProperties>> = None;
                    let mut slack: Option<::Value<SlackSourceProperties>> = None;
                    let mut trendmicro: Option<::Value<TrendmicroSourceProperties>> = None;
                    let mut veeva: Option<::Value<VeevaSourceProperties>> = None;
                    let mut zendesk: Option<::Value<ZendeskSourceProperties>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Amplitude" => {
                                amplitude = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CustomConnector" => {
                                custom_connector = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Datadog" => {
                                datadog = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Dynatrace" => {
                                dynatrace = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GoogleAnalytics" => {
                                google_analytics = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InforNexus" => {
                                infor_nexus = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Marketo" => {
                                marketo = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Pardot" => {
                                pardot = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3" => {
                                s3 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SAPOData" => {
                                sapo_data = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Salesforce" => {
                                salesforce = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServiceNow" => {
                                service_now = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Singular" => {
                                singular = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Slack" => {
                                slack = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Trendmicro" => {
                                trendmicro = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Veeva" => {
                                veeva = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Zendesk" => {
                                zendesk = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SourceConnectorProperties {
                        amplitude: amplitude,
                        custom_connector: custom_connector,
                        datadog: datadog,
                        dynatrace: dynatrace,
                        google_analytics: google_analytics,
                        infor_nexus: infor_nexus,
                        marketo: marketo,
                        pardot: pardot,
                        s3: s3,
                        sapo_data: sapo_data,
                        salesforce: salesforce,
                        service_now: service_now,
                        singular: singular,
                        slack: slack,
                        trendmicro: trendmicro,
                        veeva: veeva,
                        zendesk: zendesk,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.SourceFlowConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sourceflowconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct SourceFlowConfig {
        /// Property [`ApiVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sourceflowconfig.html#cfn-appflow-flow-sourceflowconfig-apiversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub api_version: Option<::Value<String>>,
        /// Property [`ConnectorProfileName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sourceflowconfig.html#cfn-appflow-flow-sourceflowconfig-connectorprofilename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connector_profile_name: Option<::Value<String>>,
        /// Property [`ConnectorType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sourceflowconfig.html#cfn-appflow-flow-sourceflowconfig-connectortype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connector_type: ::Value<String>,
        /// Property [`IncrementalPullConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sourceflowconfig.html#cfn-appflow-flow-sourceflowconfig-incrementalpullconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub incremental_pull_config: Option<::Value<IncrementalPullConfig>>,
        /// Property [`SourceConnectorProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-sourceflowconfig.html#cfn-appflow-flow-sourceflowconfig-sourceconnectorproperties).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_connector_properties: ::Value<SourceConnectorProperties>,
    }

    impl ::codec::SerializeValue for SourceFlowConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref api_version) = self.api_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApiVersion", api_version)?;
            }
            if let Some(ref connector_profile_name) = self.connector_profile_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectorProfileName", connector_profile_name)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectorType", &self.connector_type)?;
            if let Some(ref incremental_pull_config) = self.incremental_pull_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncrementalPullConfig", incremental_pull_config)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceConnectorProperties", &self.source_connector_properties)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SourceFlowConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SourceFlowConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SourceFlowConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SourceFlowConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut api_version: Option<::Value<String>> = None;
                    let mut connector_profile_name: Option<::Value<String>> = None;
                    let mut connector_type: Option<::Value<String>> = None;
                    let mut incremental_pull_config: Option<::Value<IncrementalPullConfig>> = None;
                    let mut source_connector_properties: Option<::Value<SourceConnectorProperties>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ApiVersion" => {
                                api_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConnectorProfileName" => {
                                connector_profile_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConnectorType" => {
                                connector_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncrementalPullConfig" => {
                                incremental_pull_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceConnectorProperties" => {
                                source_connector_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SourceFlowConfig {
                        api_version: api_version,
                        connector_profile_name: connector_profile_name,
                        connector_type: connector_type.ok_or(::serde::de::Error::missing_field("ConnectorType"))?,
                        incremental_pull_config: incremental_pull_config,
                        source_connector_properties: source_connector_properties.ok_or(::serde::de::Error::missing_field("SourceConnectorProperties"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.SuccessResponseHandlingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-successresponsehandlingconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct SuccessResponseHandlingConfig {
        /// Property [`BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-successresponsehandlingconfig.html#cfn-appflow-flow-successresponsehandlingconfig-bucketname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_name: Option<::Value<String>>,
        /// Property [`BucketPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-successresponsehandlingconfig.html#cfn-appflow-flow-successresponsehandlingconfig-bucketprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_prefix: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SuccessResponseHandlingConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref bucket_name) = self.bucket_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketName", bucket_name)?;
            }
            if let Some(ref bucket_prefix) = self.bucket_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketPrefix", bucket_prefix)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SuccessResponseHandlingConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SuccessResponseHandlingConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SuccessResponseHandlingConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SuccessResponseHandlingConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket_name: Option<::Value<String>> = None;
                    let mut bucket_prefix: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketName" => {
                                bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BucketPrefix" => {
                                bucket_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SuccessResponseHandlingConfig {
                        bucket_name: bucket_name,
                        bucket_prefix: bucket_prefix,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.Task`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-task.html) property type.
    #[derive(Debug, Default)]
    pub struct Task {
        /// Property [`ConnectorOperator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-task.html#cfn-appflow-flow-task-connectoroperator).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connector_operator: Option<::Value<ConnectorOperator>>,
        /// Property [`DestinationField`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-task.html#cfn-appflow-flow-task-destinationfield).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination_field: Option<::Value<String>>,
        /// Property [`SourceFields`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-task.html#cfn-appflow-flow-task-sourcefields).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_fields: ::ValueList<String>,
        /// Property [`TaskProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-task.html#cfn-appflow-flow-task-taskproperties).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub task_properties: Option<::ValueList<TaskPropertiesObject>>,
        /// Property [`TaskType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-task.html#cfn-appflow-flow-task-tasktype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub task_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for Task {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref connector_operator) = self.connector_operator {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectorOperator", connector_operator)?;
            }
            if let Some(ref destination_field) = self.destination_field {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationField", destination_field)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceFields", &self.source_fields)?;
            if let Some(ref task_properties) = self.task_properties {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TaskProperties", task_properties)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TaskType", &self.task_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Task {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Task, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Task;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Task")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut connector_operator: Option<::Value<ConnectorOperator>> = None;
                    let mut destination_field: Option<::Value<String>> = None;
                    let mut source_fields: Option<::ValueList<String>> = None;
                    let mut task_properties: Option<::ValueList<TaskPropertiesObject>> = None;
                    let mut task_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConnectorOperator" => {
                                connector_operator = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DestinationField" => {
                                destination_field = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceFields" => {
                                source_fields = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TaskProperties" => {
                                task_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TaskType" => {
                                task_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Task {
                        connector_operator: connector_operator,
                        destination_field: destination_field,
                        source_fields: source_fields.ok_or(::serde::de::Error::missing_field("SourceFields"))?,
                        task_properties: task_properties,
                        task_type: task_type.ok_or(::serde::de::Error::missing_field("TaskType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.TaskPropertiesObject`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-taskpropertiesobject.html) property type.
    #[derive(Debug, Default)]
    pub struct TaskPropertiesObject {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-taskpropertiesobject.html#cfn-appflow-flow-taskpropertiesobject-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-taskpropertiesobject.html#cfn-appflow-flow-taskpropertiesobject-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for TaskPropertiesObject {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TaskPropertiesObject {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TaskPropertiesObject, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TaskPropertiesObject;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TaskPropertiesObject")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TaskPropertiesObject {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.TrendmicroSourceProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-trendmicrosourceproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct TrendmicroSourceProperties {
        /// Property [`Object`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-trendmicrosourceproperties.html#cfn-appflow-flow-trendmicrosourceproperties-object).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object: ::Value<String>,
    }

    impl ::codec::SerializeValue for TrendmicroSourceProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Object", &self.object)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TrendmicroSourceProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TrendmicroSourceProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TrendmicroSourceProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TrendmicroSourceProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut object: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Object" => {
                                object = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TrendmicroSourceProperties {
                        object: object.ok_or(::serde::de::Error::missing_field("Object"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.TriggerConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-triggerconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct TriggerConfig {
        /// Property [`TriggerProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-triggerconfig.html#cfn-appflow-flow-triggerconfig-triggerproperties).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub trigger_properties: Option<::Value<ScheduledTriggerProperties>>,
        /// Property [`TriggerType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-triggerconfig.html#cfn-appflow-flow-triggerconfig-triggertype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub trigger_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for TriggerConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref trigger_properties) = self.trigger_properties {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TriggerProperties", trigger_properties)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TriggerType", &self.trigger_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TriggerConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TriggerConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TriggerConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TriggerConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut trigger_properties: Option<::Value<ScheduledTriggerProperties>> = None;
                    let mut trigger_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TriggerProperties" => {
                                trigger_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TriggerType" => {
                                trigger_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TriggerConfig {
                        trigger_properties: trigger_properties,
                        trigger_type: trigger_type.ok_or(::serde::de::Error::missing_field("TriggerType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.UpsolverDestinationProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-upsolverdestinationproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct UpsolverDestinationProperties {
        /// Property [`BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-upsolverdestinationproperties.html#cfn-appflow-flow-upsolverdestinationproperties-bucketname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_name: ::Value<String>,
        /// Property [`BucketPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-upsolverdestinationproperties.html#cfn-appflow-flow-upsolverdestinationproperties-bucketprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_prefix: Option<::Value<String>>,
        /// Property [`S3OutputFormatConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-upsolverdestinationproperties.html#cfn-appflow-flow-upsolverdestinationproperties-s3outputformatconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_output_format_config: ::Value<UpsolverS3OutputFormatConfig>,
    }

    impl ::codec::SerializeValue for UpsolverDestinationProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketName", &self.bucket_name)?;
            if let Some(ref bucket_prefix) = self.bucket_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketPrefix", bucket_prefix)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3OutputFormatConfig", &self.s3_output_format_config)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for UpsolverDestinationProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<UpsolverDestinationProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = UpsolverDestinationProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type UpsolverDestinationProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket_name: Option<::Value<String>> = None;
                    let mut bucket_prefix: Option<::Value<String>> = None;
                    let mut s3_output_format_config: Option<::Value<UpsolverS3OutputFormatConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketName" => {
                                bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BucketPrefix" => {
                                bucket_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3OutputFormatConfig" => {
                                s3_output_format_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(UpsolverDestinationProperties {
                        bucket_name: bucket_name.ok_or(::serde::de::Error::missing_field("BucketName"))?,
                        bucket_prefix: bucket_prefix,
                        s3_output_format_config: s3_output_format_config.ok_or(::serde::de::Error::missing_field("S3OutputFormatConfig"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.UpsolverS3OutputFormatConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-upsolvers3outputformatconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct UpsolverS3OutputFormatConfig {
        /// Property [`AggregationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-upsolvers3outputformatconfig.html#cfn-appflow-flow-upsolvers3outputformatconfig-aggregationconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aggregation_config: Option<::Value<AggregationConfig>>,
        /// Property [`FileType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-upsolvers3outputformatconfig.html#cfn-appflow-flow-upsolvers3outputformatconfig-filetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub file_type: Option<::Value<String>>,
        /// Property [`PrefixConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-upsolvers3outputformatconfig.html#cfn-appflow-flow-upsolvers3outputformatconfig-prefixconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub prefix_config: ::Value<PrefixConfig>,
    }

    impl ::codec::SerializeValue for UpsolverS3OutputFormatConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref aggregation_config) = self.aggregation_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AggregationConfig", aggregation_config)?;
            }
            if let Some(ref file_type) = self.file_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FileType", file_type)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrefixConfig", &self.prefix_config)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for UpsolverS3OutputFormatConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<UpsolverS3OutputFormatConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = UpsolverS3OutputFormatConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type UpsolverS3OutputFormatConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut aggregation_config: Option<::Value<AggregationConfig>> = None;
                    let mut file_type: Option<::Value<String>> = None;
                    let mut prefix_config: Option<::Value<PrefixConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AggregationConfig" => {
                                aggregation_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FileType" => {
                                file_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PrefixConfig" => {
                                prefix_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(UpsolverS3OutputFormatConfig {
                        aggregation_config: aggregation_config,
                        file_type: file_type,
                        prefix_config: prefix_config.ok_or(::serde::de::Error::missing_field("PrefixConfig"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.VeevaSourceProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-veevasourceproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct VeevaSourceProperties {
        /// Property [`DocumentType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-veevasourceproperties.html#cfn-appflow-flow-veevasourceproperties-documenttype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub document_type: Option<::Value<String>>,
        /// Property [`IncludeAllVersions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-veevasourceproperties.html#cfn-appflow-flow-veevasourceproperties-includeallversions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_all_versions: Option<::Value<bool>>,
        /// Property [`IncludeRenditions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-veevasourceproperties.html#cfn-appflow-flow-veevasourceproperties-includerenditions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_renditions: Option<::Value<bool>>,
        /// Property [`IncludeSourceFiles`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-veevasourceproperties.html#cfn-appflow-flow-veevasourceproperties-includesourcefiles).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub include_source_files: Option<::Value<bool>>,
        /// Property [`Object`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-veevasourceproperties.html#cfn-appflow-flow-veevasourceproperties-object).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object: ::Value<String>,
    }

    impl ::codec::SerializeValue for VeevaSourceProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref document_type) = self.document_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentType", document_type)?;
            }
            if let Some(ref include_all_versions) = self.include_all_versions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeAllVersions", include_all_versions)?;
            }
            if let Some(ref include_renditions) = self.include_renditions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeRenditions", include_renditions)?;
            }
            if let Some(ref include_source_files) = self.include_source_files {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncludeSourceFiles", include_source_files)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Object", &self.object)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VeevaSourceProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VeevaSourceProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VeevaSourceProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VeevaSourceProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut document_type: Option<::Value<String>> = None;
                    let mut include_all_versions: Option<::Value<bool>> = None;
                    let mut include_renditions: Option<::Value<bool>> = None;
                    let mut include_source_files: Option<::Value<bool>> = None;
                    let mut object: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DocumentType" => {
                                document_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeAllVersions" => {
                                include_all_versions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeRenditions" => {
                                include_renditions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncludeSourceFiles" => {
                                include_source_files = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Object" => {
                                object = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VeevaSourceProperties {
                        document_type: document_type,
                        include_all_versions: include_all_versions,
                        include_renditions: include_renditions,
                        include_source_files: include_source_files,
                        object: object.ok_or(::serde::de::Error::missing_field("Object"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.ZendeskDestinationProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-zendeskdestinationproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct ZendeskDestinationProperties {
        /// Property [`ErrorHandlingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-zendeskdestinationproperties.html#cfn-appflow-flow-zendeskdestinationproperties-errorhandlingconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub error_handling_config: Option<::Value<ErrorHandlingConfig>>,
        /// Property [`IdFieldNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-zendeskdestinationproperties.html#cfn-appflow-flow-zendeskdestinationproperties-idfieldnames).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id_field_names: Option<::ValueList<String>>,
        /// Property [`Object`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-zendeskdestinationproperties.html#cfn-appflow-flow-zendeskdestinationproperties-object).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object: ::Value<String>,
        /// Property [`WriteOperationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-zendeskdestinationproperties.html#cfn-appflow-flow-zendeskdestinationproperties-writeoperationtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub write_operation_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ZendeskDestinationProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref error_handling_config) = self.error_handling_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ErrorHandlingConfig", error_handling_config)?;
            }
            if let Some(ref id_field_names) = self.id_field_names {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdFieldNames", id_field_names)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Object", &self.object)?;
            if let Some(ref write_operation_type) = self.write_operation_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WriteOperationType", write_operation_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ZendeskDestinationProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ZendeskDestinationProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ZendeskDestinationProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ZendeskDestinationProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut error_handling_config: Option<::Value<ErrorHandlingConfig>> = None;
                    let mut id_field_names: Option<::ValueList<String>> = None;
                    let mut object: Option<::Value<String>> = None;
                    let mut write_operation_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ErrorHandlingConfig" => {
                                error_handling_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IdFieldNames" => {
                                id_field_names = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Object" => {
                                object = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WriteOperationType" => {
                                write_operation_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ZendeskDestinationProperties {
                        error_handling_config: error_handling_config,
                        id_field_names: id_field_names,
                        object: object.ok_or(::serde::de::Error::missing_field("Object"))?,
                        write_operation_type: write_operation_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::AppFlow::Flow.ZendeskSourceProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-zendesksourceproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct ZendeskSourceProperties {
        /// Property [`Object`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appflow-flow-zendesksourceproperties.html#cfn-appflow-flow-zendesksourceproperties-object).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object: ::Value<String>,
    }

    impl ::codec::SerializeValue for ZendeskSourceProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Object", &self.object)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ZendeskSourceProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ZendeskSourceProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ZendeskSourceProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ZendeskSourceProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut object: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Object" => {
                                object = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ZendeskSourceProperties {
                        object: object.ok_or(::serde::de::Error::missing_field("Object"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
