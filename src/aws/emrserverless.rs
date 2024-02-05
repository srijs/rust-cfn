//! Types for the `EMRServerless` service.

/// The [`AWS::EMRServerless::Application`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emrserverless-application.html) resource type.
#[derive(Debug, Default)]
pub struct Application {
    properties: ApplicationProperties
}

/// Properties for the `Application` resource.
#[derive(Debug, Default)]
pub struct ApplicationProperties {
    /// Property [`Architecture`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emrserverless-application.html#cfn-emrserverless-application-architecture).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub architecture: Option<::Value<String>>,
    /// Property [`AutoStartConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emrserverless-application.html#cfn-emrserverless-application-autostartconfiguration).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub auto_start_configuration: Option<::Value<self::application::AutoStartConfiguration>>,
    /// Property [`AutoStopConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emrserverless-application.html#cfn-emrserverless-application-autostopconfiguration).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub auto_stop_configuration: Option<::Value<self::application::AutoStopConfiguration>>,
    /// Property [`ImageConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emrserverless-application.html#cfn-emrserverless-application-imageconfiguration).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub image_configuration: Option<::Value<self::application::ImageConfigurationInput>>,
    /// Property [`InitialCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emrserverless-application.html#cfn-emrserverless-application-initialcapacity).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub initial_capacity: Option<::ValueList<self::application::InitialCapacityConfigKeyValuePair>>,
    /// Property [`MaximumCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emrserverless-application.html#cfn-emrserverless-application-maximumcapacity).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub maximum_capacity: Option<::Value<self::application::MaximumAllowedResources>>,
    /// Property [`MonitoringConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emrserverless-application.html#cfn-emrserverless-application-monitoringconfiguration).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub monitoring_configuration: Option<::Value<self::application::MonitoringConfiguration>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emrserverless-application.html#cfn-emrserverless-application-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`NetworkConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emrserverless-application.html#cfn-emrserverless-application-networkconfiguration).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub network_configuration: Option<::Value<self::application::NetworkConfiguration>>,
    /// Property [`ReleaseLabel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emrserverless-application.html#cfn-emrserverless-application-releaselabel).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub release_label: ::Value<String>,
    /// Property [`RuntimeConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emrserverless-application.html#cfn-emrserverless-application-runtimeconfiguration).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub runtime_configuration: Option<::ValueList<self::application::ConfigurationObject>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emrserverless-application.html#cfn-emrserverless-application-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emrserverless-application.html#cfn-emrserverless-application-type).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub r#type: ::Value<String>,
    /// Property [`WorkerTypeSpecifications`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emrserverless-application.html#cfn-emrserverless-application-workertypespecifications).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub worker_type_specifications: Option<::ValueMap<self::application::WorkerTypeSpecificationInput>>,
}

impl ::serde::Serialize for ApplicationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref architecture) = self.architecture {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Architecture", architecture)?;
        }
        if let Some(ref auto_start_configuration) = self.auto_start_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoStartConfiguration", auto_start_configuration)?;
        }
        if let Some(ref auto_stop_configuration) = self.auto_stop_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoStopConfiguration", auto_stop_configuration)?;
        }
        if let Some(ref image_configuration) = self.image_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageConfiguration", image_configuration)?;
        }
        if let Some(ref initial_capacity) = self.initial_capacity {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InitialCapacity", initial_capacity)?;
        }
        if let Some(ref maximum_capacity) = self.maximum_capacity {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumCapacity", maximum_capacity)?;
        }
        if let Some(ref monitoring_configuration) = self.monitoring_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MonitoringConfiguration", monitoring_configuration)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref network_configuration) = self.network_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkConfiguration", network_configuration)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReleaseLabel", &self.release_label)?;
        if let Some(ref runtime_configuration) = self.runtime_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuntimeConfiguration", runtime_configuration)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
        if let Some(ref worker_type_specifications) = self.worker_type_specifications {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkerTypeSpecifications", worker_type_specifications)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ApplicationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ApplicationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ApplicationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ApplicationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut architecture: Option<::Value<String>> = None;
                let mut auto_start_configuration: Option<::Value<self::application::AutoStartConfiguration>> = None;
                let mut auto_stop_configuration: Option<::Value<self::application::AutoStopConfiguration>> = None;
                let mut image_configuration: Option<::Value<self::application::ImageConfigurationInput>> = None;
                let mut initial_capacity: Option<::ValueList<self::application::InitialCapacityConfigKeyValuePair>> = None;
                let mut maximum_capacity: Option<::Value<self::application::MaximumAllowedResources>> = None;
                let mut monitoring_configuration: Option<::Value<self::application::MonitoringConfiguration>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut network_configuration: Option<::Value<self::application::NetworkConfiguration>> = None;
                let mut release_label: Option<::Value<String>> = None;
                let mut runtime_configuration: Option<::ValueList<self::application::ConfigurationObject>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut r#type: Option<::Value<String>> = None;
                let mut worker_type_specifications: Option<::ValueMap<self::application::WorkerTypeSpecificationInput>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Architecture" => {
                            architecture = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AutoStartConfiguration" => {
                            auto_start_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AutoStopConfiguration" => {
                            auto_stop_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ImageConfiguration" => {
                            image_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InitialCapacity" => {
                            initial_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaximumCapacity" => {
                            maximum_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MonitoringConfiguration" => {
                            monitoring_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NetworkConfiguration" => {
                            network_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReleaseLabel" => {
                            release_label = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RuntimeConfiguration" => {
                            runtime_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Type" => {
                            r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "WorkerTypeSpecifications" => {
                            worker_type_specifications = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ApplicationProperties {
                    architecture: architecture,
                    auto_start_configuration: auto_start_configuration,
                    auto_stop_configuration: auto_stop_configuration,
                    image_configuration: image_configuration,
                    initial_capacity: initial_capacity,
                    maximum_capacity: maximum_capacity,
                    monitoring_configuration: monitoring_configuration,
                    name: name,
                    network_configuration: network_configuration,
                    release_label: release_label.ok_or(::serde::de::Error::missing_field("ReleaseLabel"))?,
                    runtime_configuration: runtime_configuration,
                    tags: tags,
                    r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    worker_type_specifications: worker_type_specifications,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Application {
    type Properties = ApplicationProperties;
    const TYPE: &'static str = "AWS::EMRServerless::Application";
    fn properties(&self) -> &ApplicationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ApplicationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Application {}

impl From<ApplicationProperties> for Application {
    fn from(properties: ApplicationProperties) -> Application {
        Application { properties }
    }
}

pub mod application {
    //! Property types for the `Application` resource.

    /// The [`AWS::EMRServerless::Application.AutoStartConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-autostartconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct AutoStartConfiguration {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-autostartconfiguration.html#cfn-emrserverless-application-autostartconfiguration-enabled).
        ///
        /// Update type: _Conditional_.
        /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
        /// For more information, see the relevant resource type documentation.
        pub enabled: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for AutoStartConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enabled) = self.enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AutoStartConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AutoStartConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AutoStartConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AutoStartConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AutoStartConfiguration {
                        enabled: enabled,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMRServerless::Application.AutoStopConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-autostopconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct AutoStopConfiguration {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-autostopconfiguration.html#cfn-emrserverless-application-autostopconfiguration-enabled).
        ///
        /// Update type: _Conditional_.
        /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
        /// For more information, see the relevant resource type documentation.
        pub enabled: Option<::Value<bool>>,
        /// Property [`IdleTimeoutMinutes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-autostopconfiguration.html#cfn-emrserverless-application-autostopconfiguration-idletimeoutminutes).
        ///
        /// Update type: _Conditional_.
        /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
        /// For more information, see the relevant resource type documentation.
        pub idle_timeout_minutes: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for AutoStopConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enabled) = self.enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
            }
            if let Some(ref idle_timeout_minutes) = self.idle_timeout_minutes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdleTimeoutMinutes", idle_timeout_minutes)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AutoStopConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AutoStopConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AutoStopConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AutoStopConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut idle_timeout_minutes: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IdleTimeoutMinutes" => {
                                idle_timeout_minutes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AutoStopConfiguration {
                        enabled: enabled,
                        idle_timeout_minutes: idle_timeout_minutes,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMRServerless::Application.CloudWatchLoggingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-cloudwatchloggingconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct CloudWatchLoggingConfiguration {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-cloudwatchloggingconfiguration.html#cfn-emrserverless-application-cloudwatchloggingconfiguration-enabled).
        ///
        /// Update type: _Conditional_.
        /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
        /// For more information, see the relevant resource type documentation.
        pub enabled: Option<::Value<bool>>,
        /// Property [`EncryptionKeyArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-cloudwatchloggingconfiguration.html#cfn-emrserverless-application-cloudwatchloggingconfiguration-encryptionkeyarn).
        ///
        /// Update type: _Conditional_.
        /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
        /// For more information, see the relevant resource type documentation.
        pub encryption_key_arn: Option<::Value<String>>,
        /// Property [`LogGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-cloudwatchloggingconfiguration.html#cfn-emrserverless-application-cloudwatchloggingconfiguration-loggroupname).
        ///
        /// Update type: _Conditional_.
        /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
        /// For more information, see the relevant resource type documentation.
        pub log_group_name: Option<::Value<String>>,
        /// Property [`LogStreamNamePrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-cloudwatchloggingconfiguration.html#cfn-emrserverless-application-cloudwatchloggingconfiguration-logstreamnameprefix).
        ///
        /// Update type: _Conditional_.
        /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
        /// For more information, see the relevant resource type documentation.
        pub log_stream_name_prefix: Option<::Value<String>>,
        /// Property [`LogTypeMap`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-cloudwatchloggingconfiguration.html#cfn-emrserverless-application-cloudwatchloggingconfiguration-logtypemap).
        ///
        /// Update type: _Conditional_.
        /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
        /// For more information, see the relevant resource type documentation.
        pub log_type_map: Option<::ValueList<LogTypeMapKeyValuePair>>,
    }

    impl ::codec::SerializeValue for CloudWatchLoggingConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enabled) = self.enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
            }
            if let Some(ref encryption_key_arn) = self.encryption_key_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionKeyArn", encryption_key_arn)?;
            }
            if let Some(ref log_group_name) = self.log_group_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogGroupName", log_group_name)?;
            }
            if let Some(ref log_stream_name_prefix) = self.log_stream_name_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogStreamNamePrefix", log_stream_name_prefix)?;
            }
            if let Some(ref log_type_map) = self.log_type_map {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogTypeMap", log_type_map)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CloudWatchLoggingConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CloudWatchLoggingConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CloudWatchLoggingConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CloudWatchLoggingConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut encryption_key_arn: Option<::Value<String>> = None;
                    let mut log_group_name: Option<::Value<String>> = None;
                    let mut log_stream_name_prefix: Option<::Value<String>> = None;
                    let mut log_type_map: Option<::ValueList<LogTypeMapKeyValuePair>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EncryptionKeyArn" => {
                                encryption_key_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogGroupName" => {
                                log_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogStreamNamePrefix" => {
                                log_stream_name_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogTypeMap" => {
                                log_type_map = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CloudWatchLoggingConfiguration {
                        enabled: enabled,
                        encryption_key_arn: encryption_key_arn,
                        log_group_name: log_group_name,
                        log_stream_name_prefix: log_stream_name_prefix,
                        log_type_map: log_type_map,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMRServerless::Application.ConfigurationObject`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-configurationobject.html) property type.
    #[derive(Debug, Default)]
    pub struct ConfigurationObject {
        /// Property [`Classification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-configurationobject.html#cfn-emrserverless-application-configurationobject-classification).
        ///
        /// Update type: _Conditional_.
        /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
        /// For more information, see the relevant resource type documentation.
        pub classification: ::Value<String>,
        /// Property [`Configurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-configurationobject.html#cfn-emrserverless-application-configurationobject-configurations).
        ///
        /// Update type: _Conditional_.
        /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
        /// For more information, see the relevant resource type documentation.
        pub configurations: Option<::ValueList<ConfigurationObject>>,
        /// Property [`Properties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-configurationobject.html#cfn-emrserverless-application-configurationobject-properties).
        ///
        /// Update type: _Conditional_.
        /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
        /// For more information, see the relevant resource type documentation.
        pub properties: Option<::ValueMap<String>>,
    }

    impl ::codec::SerializeValue for ConfigurationObject {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Classification", &self.classification)?;
            if let Some(ref configurations) = self.configurations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Configurations", configurations)?;
            }
            if let Some(ref properties) = self.properties {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Properties", properties)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConfigurationObject {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConfigurationObject, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConfigurationObject;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConfigurationObject")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut classification: Option<::Value<String>> = None;
                    let mut configurations: Option<::ValueList<ConfigurationObject>> = None;
                    let mut properties: Option<::ValueMap<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Classification" => {
                                classification = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Configurations" => {
                                configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Properties" => {
                                properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConfigurationObject {
                        classification: classification.ok_or(::serde::de::Error::missing_field("Classification"))?,
                        configurations: configurations,
                        properties: properties,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMRServerless::Application.ImageConfigurationInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-imageconfigurationinput.html) property type.
    #[derive(Debug, Default)]
    pub struct ImageConfigurationInput {
        /// Property [`ImageUri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-imageconfigurationinput.html#cfn-emrserverless-application-imageconfigurationinput-imageuri).
        ///
        /// Update type: _Conditional_.
        /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
        /// For more information, see the relevant resource type documentation.
        pub image_uri: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ImageConfigurationInput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref image_uri) = self.image_uri {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageUri", image_uri)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ImageConfigurationInput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ImageConfigurationInput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ImageConfigurationInput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ImageConfigurationInput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut image_uri: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ImageUri" => {
                                image_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ImageConfigurationInput {
                        image_uri: image_uri,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMRServerless::Application.InitialCapacityConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-initialcapacityconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct InitialCapacityConfig {
        /// Property [`WorkerConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-initialcapacityconfig.html#cfn-emrserverless-application-initialcapacityconfig-workerconfiguration).
        ///
        /// Update type: _Conditional_.
        /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
        /// For more information, see the relevant resource type documentation.
        pub worker_configuration: ::Value<WorkerConfiguration>,
        /// Property [`WorkerCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-initialcapacityconfig.html#cfn-emrserverless-application-initialcapacityconfig-workercount).
        ///
        /// Update type: _Conditional_.
        /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
        /// For more information, see the relevant resource type documentation.
        pub worker_count: ::Value<u32>,
    }

    impl ::codec::SerializeValue for InitialCapacityConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkerConfiguration", &self.worker_configuration)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkerCount", &self.worker_count)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InitialCapacityConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InitialCapacityConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InitialCapacityConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InitialCapacityConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut worker_configuration: Option<::Value<WorkerConfiguration>> = None;
                    let mut worker_count: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "WorkerConfiguration" => {
                                worker_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WorkerCount" => {
                                worker_count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InitialCapacityConfig {
                        worker_configuration: worker_configuration.ok_or(::serde::de::Error::missing_field("WorkerConfiguration"))?,
                        worker_count: worker_count.ok_or(::serde::de::Error::missing_field("WorkerCount"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMRServerless::Application.InitialCapacityConfigKeyValuePair`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-initialcapacityconfigkeyvaluepair.html) property type.
    #[derive(Debug, Default)]
    pub struct InitialCapacityConfigKeyValuePair {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-initialcapacityconfigkeyvaluepair.html#cfn-emrserverless-application-initialcapacityconfigkeyvaluepair-key).
        ///
        /// Update type: _Conditional_.
        /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
        /// For more information, see the relevant resource type documentation.
        pub key: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-initialcapacityconfigkeyvaluepair.html#cfn-emrserverless-application-initialcapacityconfigkeyvaluepair-value).
        ///
        /// Update type: _Conditional_.
        /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
        /// For more information, see the relevant resource type documentation.
        pub value: ::Value<InitialCapacityConfig>,
    }

    impl ::codec::SerializeValue for InitialCapacityConfigKeyValuePair {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InitialCapacityConfigKeyValuePair {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InitialCapacityConfigKeyValuePair, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InitialCapacityConfigKeyValuePair;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InitialCapacityConfigKeyValuePair")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut value: Option<::Value<InitialCapacityConfig>> = None;

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

                    Ok(InitialCapacityConfigKeyValuePair {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMRServerless::Application.LogTypeMapKeyValuePair`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-logtypemapkeyvaluepair.html) property type.
    #[derive(Debug, Default)]
    pub struct LogTypeMapKeyValuePair {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-logtypemapkeyvaluepair.html#cfn-emrserverless-application-logtypemapkeyvaluepair-key).
        ///
        /// Update type: _Conditional_.
        /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
        /// For more information, see the relevant resource type documentation.
        pub key: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-logtypemapkeyvaluepair.html#cfn-emrserverless-application-logtypemapkeyvaluepair-value).
        ///
        /// Update type: _Conditional_.
        /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
        /// For more information, see the relevant resource type documentation.
        pub value: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for LogTypeMapKeyValuePair {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LogTypeMapKeyValuePair {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LogTypeMapKeyValuePair, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LogTypeMapKeyValuePair;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LogTypeMapKeyValuePair")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut value: Option<::ValueList<String>> = None;

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

                    Ok(LogTypeMapKeyValuePair {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMRServerless::Application.ManagedPersistenceMonitoringConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-managedpersistencemonitoringconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ManagedPersistenceMonitoringConfiguration {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-managedpersistencemonitoringconfiguration.html#cfn-emrserverless-application-managedpersistencemonitoringconfiguration-enabled).
        ///
        /// Update type: _Conditional_.
        /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
        /// For more information, see the relevant resource type documentation.
        pub enabled: Option<::Value<bool>>,
        /// Property [`EncryptionKeyArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-managedpersistencemonitoringconfiguration.html#cfn-emrserverless-application-managedpersistencemonitoringconfiguration-encryptionkeyarn).
        ///
        /// Update type: _Conditional_.
        /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
        /// For more information, see the relevant resource type documentation.
        pub encryption_key_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ManagedPersistenceMonitoringConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enabled) = self.enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
            }
            if let Some(ref encryption_key_arn) = self.encryption_key_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionKeyArn", encryption_key_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ManagedPersistenceMonitoringConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ManagedPersistenceMonitoringConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ManagedPersistenceMonitoringConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ManagedPersistenceMonitoringConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut encryption_key_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EncryptionKeyArn" => {
                                encryption_key_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ManagedPersistenceMonitoringConfiguration {
                        enabled: enabled,
                        encryption_key_arn: encryption_key_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMRServerless::Application.MaximumAllowedResources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-maximumallowedresources.html) property type.
    #[derive(Debug, Default)]
    pub struct MaximumAllowedResources {
        /// Property [`Cpu`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-maximumallowedresources.html#cfn-emrserverless-application-maximumallowedresources-cpu).
        ///
        /// Update type: _Conditional_.
        /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
        /// For more information, see the relevant resource type documentation.
        pub cpu: ::Value<String>,
        /// Property [`Disk`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-maximumallowedresources.html#cfn-emrserverless-application-maximumallowedresources-disk).
        ///
        /// Update type: _Conditional_.
        /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
        /// For more information, see the relevant resource type documentation.
        pub disk: Option<::Value<String>>,
        /// Property [`Memory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-maximumallowedresources.html#cfn-emrserverless-application-maximumallowedresources-memory).
        ///
        /// Update type: _Conditional_.
        /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
        /// For more information, see the relevant resource type documentation.
        pub memory: ::Value<String>,
    }

    impl ::codec::SerializeValue for MaximumAllowedResources {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cpu", &self.cpu)?;
            if let Some(ref disk) = self.disk {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Disk", disk)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Memory", &self.memory)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MaximumAllowedResources {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MaximumAllowedResources, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MaximumAllowedResources;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MaximumAllowedResources")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cpu: Option<::Value<String>> = None;
                    let mut disk: Option<::Value<String>> = None;
                    let mut memory: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Cpu" => {
                                cpu = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Disk" => {
                                disk = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Memory" => {
                                memory = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MaximumAllowedResources {
                        cpu: cpu.ok_or(::serde::de::Error::missing_field("Cpu"))?,
                        disk: disk,
                        memory: memory.ok_or(::serde::de::Error::missing_field("Memory"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMRServerless::Application.MonitoringConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-monitoringconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct MonitoringConfiguration {
        /// Property [`CloudWatchLoggingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-monitoringconfiguration.html#cfn-emrserverless-application-monitoringconfiguration-cloudwatchloggingconfiguration).
        ///
        /// Update type: _Conditional_.
        /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
        /// For more information, see the relevant resource type documentation.
        pub cloud_watch_logging_configuration: Option<::Value<CloudWatchLoggingConfiguration>>,
        /// Property [`ManagedPersistenceMonitoringConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-monitoringconfiguration.html#cfn-emrserverless-application-monitoringconfiguration-managedpersistencemonitoringconfiguration).
        ///
        /// Update type: _Conditional_.
        /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
        /// For more information, see the relevant resource type documentation.
        pub managed_persistence_monitoring_configuration: Option<::Value<ManagedPersistenceMonitoringConfiguration>>,
        /// Property [`S3MonitoringConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-monitoringconfiguration.html#cfn-emrserverless-application-monitoringconfiguration-s3monitoringconfiguration).
        ///
        /// Update type: _Conditional_.
        /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
        /// For more information, see the relevant resource type documentation.
        pub s3_monitoring_configuration: Option<::Value<S3MonitoringConfiguration>>,
    }

    impl ::codec::SerializeValue for MonitoringConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cloud_watch_logging_configuration) = self.cloud_watch_logging_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLoggingConfiguration", cloud_watch_logging_configuration)?;
            }
            if let Some(ref managed_persistence_monitoring_configuration) = self.managed_persistence_monitoring_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManagedPersistenceMonitoringConfiguration", managed_persistence_monitoring_configuration)?;
            }
            if let Some(ref s3_monitoring_configuration) = self.s3_monitoring_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3MonitoringConfiguration", s3_monitoring_configuration)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MonitoringConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MonitoringConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MonitoringConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MonitoringConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cloud_watch_logging_configuration: Option<::Value<CloudWatchLoggingConfiguration>> = None;
                    let mut managed_persistence_monitoring_configuration: Option<::Value<ManagedPersistenceMonitoringConfiguration>> = None;
                    let mut s3_monitoring_configuration: Option<::Value<S3MonitoringConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CloudWatchLoggingConfiguration" => {
                                cloud_watch_logging_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ManagedPersistenceMonitoringConfiguration" => {
                                managed_persistence_monitoring_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3MonitoringConfiguration" => {
                                s3_monitoring_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MonitoringConfiguration {
                        cloud_watch_logging_configuration: cloud_watch_logging_configuration,
                        managed_persistence_monitoring_configuration: managed_persistence_monitoring_configuration,
                        s3_monitoring_configuration: s3_monitoring_configuration,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMRServerless::Application.NetworkConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-networkconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct NetworkConfiguration {
        /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-networkconfiguration.html#cfn-emrserverless-application-networkconfiguration-securitygroupids).
        ///
        /// Update type: _Conditional_.
        /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
        /// For more information, see the relevant resource type documentation.
        pub security_group_ids: Option<::ValueList<String>>,
        /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-networkconfiguration.html#cfn-emrserverless-application-networkconfiguration-subnetids).
        ///
        /// Update type: _Conditional_.
        /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
        /// For more information, see the relevant resource type documentation.
        pub subnet_ids: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for NetworkConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref security_group_ids) = self.security_group_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", security_group_ids)?;
            }
            if let Some(ref subnet_ids) = self.subnet_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", subnet_ids)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NetworkConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NetworkConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NetworkConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NetworkConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut security_group_ids: Option<::ValueList<String>> = None;
                    let mut subnet_ids: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SecurityGroupIds" => {
                                security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubnetIds" => {
                                subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NetworkConfiguration {
                        security_group_ids: security_group_ids,
                        subnet_ids: subnet_ids,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMRServerless::Application.S3MonitoringConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-s3monitoringconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct S3MonitoringConfiguration {
        /// Property [`EncryptionKeyArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-s3monitoringconfiguration.html#cfn-emrserverless-application-s3monitoringconfiguration-encryptionkeyarn).
        ///
        /// Update type: _Conditional_.
        /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
        /// For more information, see the relevant resource type documentation.
        pub encryption_key_arn: Option<::Value<String>>,
        /// Property [`LogUri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-s3monitoringconfiguration.html#cfn-emrserverless-application-s3monitoringconfiguration-loguri).
        ///
        /// Update type: _Conditional_.
        /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
        /// For more information, see the relevant resource type documentation.
        pub log_uri: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for S3MonitoringConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref encryption_key_arn) = self.encryption_key_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionKeyArn", encryption_key_arn)?;
            }
            if let Some(ref log_uri) = self.log_uri {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogUri", log_uri)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3MonitoringConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3MonitoringConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3MonitoringConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3MonitoringConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut encryption_key_arn: Option<::Value<String>> = None;
                    let mut log_uri: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EncryptionKeyArn" => {
                                encryption_key_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogUri" => {
                                log_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3MonitoringConfiguration {
                        encryption_key_arn: encryption_key_arn,
                        log_uri: log_uri,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMRServerless::Application.WorkerConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-workerconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct WorkerConfiguration {
        /// Property [`Cpu`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-workerconfiguration.html#cfn-emrserverless-application-workerconfiguration-cpu).
        ///
        /// Update type: _Conditional_.
        /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
        /// For more information, see the relevant resource type documentation.
        pub cpu: ::Value<String>,
        /// Property [`Disk`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-workerconfiguration.html#cfn-emrserverless-application-workerconfiguration-disk).
        ///
        /// Update type: _Conditional_.
        /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
        /// For more information, see the relevant resource type documentation.
        pub disk: Option<::Value<String>>,
        /// Property [`Memory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-workerconfiguration.html#cfn-emrserverless-application-workerconfiguration-memory).
        ///
        /// Update type: _Conditional_.
        /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
        /// For more information, see the relevant resource type documentation.
        pub memory: ::Value<String>,
    }

    impl ::codec::SerializeValue for WorkerConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cpu", &self.cpu)?;
            if let Some(ref disk) = self.disk {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Disk", disk)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Memory", &self.memory)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for WorkerConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<WorkerConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = WorkerConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type WorkerConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cpu: Option<::Value<String>> = None;
                    let mut disk: Option<::Value<String>> = None;
                    let mut memory: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Cpu" => {
                                cpu = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Disk" => {
                                disk = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Memory" => {
                                memory = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(WorkerConfiguration {
                        cpu: cpu.ok_or(::serde::de::Error::missing_field("Cpu"))?,
                        disk: disk,
                        memory: memory.ok_or(::serde::de::Error::missing_field("Memory"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMRServerless::Application.WorkerTypeSpecificationInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-workertypespecificationinput.html) property type.
    #[derive(Debug, Default)]
    pub struct WorkerTypeSpecificationInput {
        /// Property [`ImageConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-workertypespecificationinput.html#cfn-emrserverless-application-workertypespecificationinput-imageconfiguration).
        ///
        /// Update type: _Conditional_.
        /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
        /// For more information, see the relevant resource type documentation.
        pub image_configuration: Option<::Value<ImageConfigurationInput>>,
    }

    impl ::codec::SerializeValue for WorkerTypeSpecificationInput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref image_configuration) = self.image_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageConfiguration", image_configuration)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for WorkerTypeSpecificationInput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<WorkerTypeSpecificationInput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = WorkerTypeSpecificationInput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type WorkerTypeSpecificationInput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut image_configuration: Option<::Value<ImageConfigurationInput>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ImageConfiguration" => {
                                image_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(WorkerTypeSpecificationInput {
                        image_configuration: image_configuration,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
