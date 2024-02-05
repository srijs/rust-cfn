//! Types for the `KinesisAnalyticsV2` service.

/// The [`AWS::KinesisAnalyticsV2::Application`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisanalyticsv2-application.html) resource type.
#[derive(Debug, Default)]
pub struct Application {
    properties: ApplicationProperties
}

/// Properties for the `Application` resource.
#[derive(Debug, Default)]
pub struct ApplicationProperties {
    /// Property [`ApplicationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisanalyticsv2-application.html#cfn-kinesisanalyticsv2-application-applicationconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub application_configuration: Option<::Value<self::application::ApplicationConfiguration>>,
    /// Property [`ApplicationDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisanalyticsv2-application.html#cfn-kinesisanalyticsv2-application-applicationdescription).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub application_description: Option<::Value<String>>,
    /// Property [`ApplicationMaintenanceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisanalyticsv2-application.html#cfn-kinesisanalyticsv2-application-applicationmaintenanceconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub application_maintenance_configuration: Option<::Value<self::application::ApplicationMaintenanceConfiguration>>,
    /// Property [`ApplicationMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisanalyticsv2-application.html#cfn-kinesisanalyticsv2-application-applicationmode).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub application_mode: Option<::Value<String>>,
    /// Property [`ApplicationName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisanalyticsv2-application.html#cfn-kinesisanalyticsv2-application-applicationname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub application_name: Option<::Value<String>>,
    /// Property [`RunConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisanalyticsv2-application.html#cfn-kinesisanalyticsv2-application-runconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub run_configuration: Option<::Value<self::application::RunConfiguration>>,
    /// Property [`RuntimeEnvironment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisanalyticsv2-application.html#cfn-kinesisanalyticsv2-application-runtimeenvironment).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub runtime_environment: ::Value<String>,
    /// Property [`ServiceExecutionRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisanalyticsv2-application.html#cfn-kinesisanalyticsv2-application-serviceexecutionrole).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub service_execution_role: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisanalyticsv2-application.html#cfn-kinesisanalyticsv2-application-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ApplicationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref application_configuration) = self.application_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationConfiguration", application_configuration)?;
        }
        if let Some(ref application_description) = self.application_description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationDescription", application_description)?;
        }
        if let Some(ref application_maintenance_configuration) = self.application_maintenance_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationMaintenanceConfiguration", application_maintenance_configuration)?;
        }
        if let Some(ref application_mode) = self.application_mode {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationMode", application_mode)?;
        }
        if let Some(ref application_name) = self.application_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationName", application_name)?;
        }
        if let Some(ref run_configuration) = self.run_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RunConfiguration", run_configuration)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuntimeEnvironment", &self.runtime_environment)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceExecutionRole", &self.service_execution_role)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
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
                let mut application_configuration: Option<::Value<self::application::ApplicationConfiguration>> = None;
                let mut application_description: Option<::Value<String>> = None;
                let mut application_maintenance_configuration: Option<::Value<self::application::ApplicationMaintenanceConfiguration>> = None;
                let mut application_mode: Option<::Value<String>> = None;
                let mut application_name: Option<::Value<String>> = None;
                let mut run_configuration: Option<::Value<self::application::RunConfiguration>> = None;
                let mut runtime_environment: Option<::Value<String>> = None;
                let mut service_execution_role: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApplicationConfiguration" => {
                            application_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ApplicationDescription" => {
                            application_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ApplicationMaintenanceConfiguration" => {
                            application_maintenance_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ApplicationMode" => {
                            application_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ApplicationName" => {
                            application_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RunConfiguration" => {
                            run_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RuntimeEnvironment" => {
                            runtime_environment = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServiceExecutionRole" => {
                            service_execution_role = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ApplicationProperties {
                    application_configuration: application_configuration,
                    application_description: application_description,
                    application_maintenance_configuration: application_maintenance_configuration,
                    application_mode: application_mode,
                    application_name: application_name,
                    run_configuration: run_configuration,
                    runtime_environment: runtime_environment.ok_or(::serde::de::Error::missing_field("RuntimeEnvironment"))?,
                    service_execution_role: service_execution_role.ok_or(::serde::de::Error::missing_field("ServiceExecutionRole"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Application {
    type Properties = ApplicationProperties;
    const TYPE: &'static str = "AWS::KinesisAnalyticsV2::Application";
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

/// The [`AWS::KinesisAnalyticsV2::ApplicationCloudWatchLoggingOption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisanalyticsv2-applicationcloudwatchloggingoption.html) resource type.
#[derive(Debug, Default)]
pub struct ApplicationCloudWatchLoggingOption {
    properties: ApplicationCloudWatchLoggingOptionProperties
}

/// Properties for the `ApplicationCloudWatchLoggingOption` resource.
#[derive(Debug, Default)]
pub struct ApplicationCloudWatchLoggingOptionProperties {
    /// Property [`ApplicationName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisanalyticsv2-applicationcloudwatchloggingoption.html#cfn-kinesisanalyticsv2-applicationcloudwatchloggingoption-applicationname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub application_name: ::Value<String>,
    /// Property [`CloudWatchLoggingOption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisanalyticsv2-applicationcloudwatchloggingoption.html#cfn-kinesisanalyticsv2-applicationcloudwatchloggingoption-cloudwatchloggingoption).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cloud_watch_logging_option: ::Value<self::application_cloud_watch_logging_option::CloudWatchLoggingOption>,
}

impl ::serde::Serialize for ApplicationCloudWatchLoggingOptionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationName", &self.application_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLoggingOption", &self.cloud_watch_logging_option)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ApplicationCloudWatchLoggingOptionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ApplicationCloudWatchLoggingOptionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ApplicationCloudWatchLoggingOptionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ApplicationCloudWatchLoggingOptionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut application_name: Option<::Value<String>> = None;
                let mut cloud_watch_logging_option: Option<::Value<self::application_cloud_watch_logging_option::CloudWatchLoggingOption>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApplicationName" => {
                            application_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CloudWatchLoggingOption" => {
                            cloud_watch_logging_option = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ApplicationCloudWatchLoggingOptionProperties {
                    application_name: application_name.ok_or(::serde::de::Error::missing_field("ApplicationName"))?,
                    cloud_watch_logging_option: cloud_watch_logging_option.ok_or(::serde::de::Error::missing_field("CloudWatchLoggingOption"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ApplicationCloudWatchLoggingOption {
    type Properties = ApplicationCloudWatchLoggingOptionProperties;
    const TYPE: &'static str = "AWS::KinesisAnalyticsV2::ApplicationCloudWatchLoggingOption";
    fn properties(&self) -> &ApplicationCloudWatchLoggingOptionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ApplicationCloudWatchLoggingOptionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ApplicationCloudWatchLoggingOption {}

impl From<ApplicationCloudWatchLoggingOptionProperties> for ApplicationCloudWatchLoggingOption {
    fn from(properties: ApplicationCloudWatchLoggingOptionProperties) -> ApplicationCloudWatchLoggingOption {
        ApplicationCloudWatchLoggingOption { properties }
    }
}

/// The [`AWS::KinesisAnalyticsV2::ApplicationOutput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisanalyticsv2-applicationoutput.html) resource type.
#[derive(Debug, Default)]
pub struct ApplicationOutput {
    properties: ApplicationOutputProperties
}

/// Properties for the `ApplicationOutput` resource.
#[derive(Debug, Default)]
pub struct ApplicationOutputProperties {
    /// Property [`ApplicationName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisanalyticsv2-applicationoutput.html#cfn-kinesisanalyticsv2-applicationoutput-applicationname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub application_name: ::Value<String>,
    /// Property [`Output`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisanalyticsv2-applicationoutput.html#cfn-kinesisanalyticsv2-applicationoutput-output).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub output: ::Value<self::application_output::Output>,
}

impl ::serde::Serialize for ApplicationOutputProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationName", &self.application_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Output", &self.output)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ApplicationOutputProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ApplicationOutputProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ApplicationOutputProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ApplicationOutputProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut application_name: Option<::Value<String>> = None;
                let mut output: Option<::Value<self::application_output::Output>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApplicationName" => {
                            application_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Output" => {
                            output = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ApplicationOutputProperties {
                    application_name: application_name.ok_or(::serde::de::Error::missing_field("ApplicationName"))?,
                    output: output.ok_or(::serde::de::Error::missing_field("Output"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ApplicationOutput {
    type Properties = ApplicationOutputProperties;
    const TYPE: &'static str = "AWS::KinesisAnalyticsV2::ApplicationOutput";
    fn properties(&self) -> &ApplicationOutputProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ApplicationOutputProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ApplicationOutput {}

impl From<ApplicationOutputProperties> for ApplicationOutput {
    fn from(properties: ApplicationOutputProperties) -> ApplicationOutput {
        ApplicationOutput { properties }
    }
}

/// The [`AWS::KinesisAnalyticsV2::ApplicationReferenceDataSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisanalyticsv2-applicationreferencedatasource.html) resource type.
#[derive(Debug, Default)]
pub struct ApplicationReferenceDataSource {
    properties: ApplicationReferenceDataSourceProperties
}

/// Properties for the `ApplicationReferenceDataSource` resource.
#[derive(Debug, Default)]
pub struct ApplicationReferenceDataSourceProperties {
    /// Property [`ApplicationName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisanalyticsv2-applicationreferencedatasource.html#cfn-kinesisanalyticsv2-applicationreferencedatasource-applicationname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub application_name: ::Value<String>,
    /// Property [`ReferenceDataSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisanalyticsv2-applicationreferencedatasource.html#cfn-kinesisanalyticsv2-applicationreferencedatasource-referencedatasource).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub reference_data_source: ::Value<self::application_reference_data_source::ReferenceDataSource>,
}

impl ::serde::Serialize for ApplicationReferenceDataSourceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationName", &self.application_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReferenceDataSource", &self.reference_data_source)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ApplicationReferenceDataSourceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ApplicationReferenceDataSourceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ApplicationReferenceDataSourceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ApplicationReferenceDataSourceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut application_name: Option<::Value<String>> = None;
                let mut reference_data_source: Option<::Value<self::application_reference_data_source::ReferenceDataSource>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApplicationName" => {
                            application_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReferenceDataSource" => {
                            reference_data_source = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ApplicationReferenceDataSourceProperties {
                    application_name: application_name.ok_or(::serde::de::Error::missing_field("ApplicationName"))?,
                    reference_data_source: reference_data_source.ok_or(::serde::de::Error::missing_field("ReferenceDataSource"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ApplicationReferenceDataSource {
    type Properties = ApplicationReferenceDataSourceProperties;
    const TYPE: &'static str = "AWS::KinesisAnalyticsV2::ApplicationReferenceDataSource";
    fn properties(&self) -> &ApplicationReferenceDataSourceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ApplicationReferenceDataSourceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ApplicationReferenceDataSource {}

impl From<ApplicationReferenceDataSourceProperties> for ApplicationReferenceDataSource {
    fn from(properties: ApplicationReferenceDataSourceProperties) -> ApplicationReferenceDataSource {
        ApplicationReferenceDataSource { properties }
    }
}

pub mod application {
    //! Property types for the `Application` resource.

    /// The [`AWS::KinesisAnalyticsV2::Application.ApplicationCodeConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-applicationcodeconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ApplicationCodeConfiguration {
        /// Property [`CodeContent`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-applicationcodeconfiguration.html#cfn-kinesisanalyticsv2-application-applicationcodeconfiguration-codecontent).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub code_content: ::Value<CodeContent>,
        /// Property [`CodeContentType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-applicationcodeconfiguration.html#cfn-kinesisanalyticsv2-application-applicationcodeconfiguration-codecontenttype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub code_content_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for ApplicationCodeConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CodeContent", &self.code_content)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CodeContentType", &self.code_content_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ApplicationCodeConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ApplicationCodeConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ApplicationCodeConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ApplicationCodeConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut code_content: Option<::Value<CodeContent>> = None;
                    let mut code_content_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CodeContent" => {
                                code_content = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CodeContentType" => {
                                code_content_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ApplicationCodeConfiguration {
                        code_content: code_content.ok_or(::serde::de::Error::missing_field("CodeContent"))?,
                        code_content_type: code_content_type.ok_or(::serde::de::Error::missing_field("CodeContentType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::Application.ApplicationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-applicationconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ApplicationConfiguration {
        /// Property [`ApplicationCodeConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-applicationconfiguration.html#cfn-kinesisanalyticsv2-application-applicationconfiguration-applicationcodeconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub application_code_configuration: Option<::Value<ApplicationCodeConfiguration>>,
        /// Property [`ApplicationSnapshotConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-applicationconfiguration.html#cfn-kinesisanalyticsv2-application-applicationconfiguration-applicationsnapshotconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub application_snapshot_configuration: Option<::Value<ApplicationSnapshotConfiguration>>,
        /// Property [`EnvironmentProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-applicationconfiguration.html#cfn-kinesisanalyticsv2-application-applicationconfiguration-environmentproperties).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub environment_properties: Option<::Value<EnvironmentProperties>>,
        /// Property [`FlinkApplicationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-applicationconfiguration.html#cfn-kinesisanalyticsv2-application-applicationconfiguration-flinkapplicationconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub flink_application_configuration: Option<::Value<FlinkApplicationConfiguration>>,
        /// Property [`SqlApplicationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-applicationconfiguration.html#cfn-kinesisanalyticsv2-application-applicationconfiguration-sqlapplicationconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sql_application_configuration: Option<::Value<SqlApplicationConfiguration>>,
        /// Property [`VpcConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-applicationconfiguration.html#cfn-kinesisanalyticsv2-application-applicationconfiguration-vpcconfigurations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vpc_configurations: Option<::ValueList<VpcConfiguration>>,
        /// Property [`ZeppelinApplicationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-applicationconfiguration.html#cfn-kinesisanalyticsv2-application-applicationconfiguration-zeppelinapplicationconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub zeppelin_application_configuration: Option<::Value<ZeppelinApplicationConfiguration>>,
    }

    impl ::codec::SerializeValue for ApplicationConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref application_code_configuration) = self.application_code_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationCodeConfiguration", application_code_configuration)?;
            }
            if let Some(ref application_snapshot_configuration) = self.application_snapshot_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationSnapshotConfiguration", application_snapshot_configuration)?;
            }
            if let Some(ref environment_properties) = self.environment_properties {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnvironmentProperties", environment_properties)?;
            }
            if let Some(ref flink_application_configuration) = self.flink_application_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FlinkApplicationConfiguration", flink_application_configuration)?;
            }
            if let Some(ref sql_application_configuration) = self.sql_application_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SqlApplicationConfiguration", sql_application_configuration)?;
            }
            if let Some(ref vpc_configurations) = self.vpc_configurations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcConfigurations", vpc_configurations)?;
            }
            if let Some(ref zeppelin_application_configuration) = self.zeppelin_application_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ZeppelinApplicationConfiguration", zeppelin_application_configuration)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ApplicationConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ApplicationConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ApplicationConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ApplicationConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut application_code_configuration: Option<::Value<ApplicationCodeConfiguration>> = None;
                    let mut application_snapshot_configuration: Option<::Value<ApplicationSnapshotConfiguration>> = None;
                    let mut environment_properties: Option<::Value<EnvironmentProperties>> = None;
                    let mut flink_application_configuration: Option<::Value<FlinkApplicationConfiguration>> = None;
                    let mut sql_application_configuration: Option<::Value<SqlApplicationConfiguration>> = None;
                    let mut vpc_configurations: Option<::ValueList<VpcConfiguration>> = None;
                    let mut zeppelin_application_configuration: Option<::Value<ZeppelinApplicationConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ApplicationCodeConfiguration" => {
                                application_code_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ApplicationSnapshotConfiguration" => {
                                application_snapshot_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnvironmentProperties" => {
                                environment_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FlinkApplicationConfiguration" => {
                                flink_application_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SqlApplicationConfiguration" => {
                                sql_application_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpcConfigurations" => {
                                vpc_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ZeppelinApplicationConfiguration" => {
                                zeppelin_application_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ApplicationConfiguration {
                        application_code_configuration: application_code_configuration,
                        application_snapshot_configuration: application_snapshot_configuration,
                        environment_properties: environment_properties,
                        flink_application_configuration: flink_application_configuration,
                        sql_application_configuration: sql_application_configuration,
                        vpc_configurations: vpc_configurations,
                        zeppelin_application_configuration: zeppelin_application_configuration,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::Application.ApplicationMaintenanceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-applicationmaintenanceconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ApplicationMaintenanceConfiguration {
        /// Property [`ApplicationMaintenanceWindowStartTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-applicationmaintenanceconfiguration.html#cfn-kinesisanalyticsv2-application-applicationmaintenanceconfiguration-applicationmaintenancewindowstarttime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub application_maintenance_window_start_time: ::Value<String>,
    }

    impl ::codec::SerializeValue for ApplicationMaintenanceConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationMaintenanceWindowStartTime", &self.application_maintenance_window_start_time)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ApplicationMaintenanceConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ApplicationMaintenanceConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ApplicationMaintenanceConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ApplicationMaintenanceConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut application_maintenance_window_start_time: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ApplicationMaintenanceWindowStartTime" => {
                                application_maintenance_window_start_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ApplicationMaintenanceConfiguration {
                        application_maintenance_window_start_time: application_maintenance_window_start_time.ok_or(::serde::de::Error::missing_field("ApplicationMaintenanceWindowStartTime"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::Application.ApplicationRestoreConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-applicationrestoreconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ApplicationRestoreConfiguration {
        /// Property [`ApplicationRestoreType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-applicationrestoreconfiguration.html#cfn-kinesisanalyticsv2-application-applicationrestoreconfiguration-applicationrestoretype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub application_restore_type: ::Value<String>,
        /// Property [`SnapshotName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-applicationrestoreconfiguration.html#cfn-kinesisanalyticsv2-application-applicationrestoreconfiguration-snapshotname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub snapshot_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ApplicationRestoreConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationRestoreType", &self.application_restore_type)?;
            if let Some(ref snapshot_name) = self.snapshot_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotName", snapshot_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ApplicationRestoreConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ApplicationRestoreConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ApplicationRestoreConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ApplicationRestoreConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut application_restore_type: Option<::Value<String>> = None;
                    let mut snapshot_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ApplicationRestoreType" => {
                                application_restore_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SnapshotName" => {
                                snapshot_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ApplicationRestoreConfiguration {
                        application_restore_type: application_restore_type.ok_or(::serde::de::Error::missing_field("ApplicationRestoreType"))?,
                        snapshot_name: snapshot_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::Application.ApplicationSnapshotConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-applicationsnapshotconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ApplicationSnapshotConfiguration {
        /// Property [`SnapshotsEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-applicationsnapshotconfiguration.html#cfn-kinesisanalyticsv2-application-applicationsnapshotconfiguration-snapshotsenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub snapshots_enabled: ::Value<bool>,
    }

    impl ::codec::SerializeValue for ApplicationSnapshotConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotsEnabled", &self.snapshots_enabled)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ApplicationSnapshotConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ApplicationSnapshotConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ApplicationSnapshotConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ApplicationSnapshotConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut snapshots_enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SnapshotsEnabled" => {
                                snapshots_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ApplicationSnapshotConfiguration {
                        snapshots_enabled: snapshots_enabled.ok_or(::serde::de::Error::missing_field("SnapshotsEnabled"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::Application.CSVMappingParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-csvmappingparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct CSVMappingParameters {
        /// Property [`RecordColumnDelimiter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-csvmappingparameters.html#cfn-kinesisanalyticsv2-application-csvmappingparameters-recordcolumndelimiter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub record_column_delimiter: ::Value<String>,
        /// Property [`RecordRowDelimiter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-csvmappingparameters.html#cfn-kinesisanalyticsv2-application-csvmappingparameters-recordrowdelimiter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub record_row_delimiter: ::Value<String>,
    }

    impl ::codec::SerializeValue for CSVMappingParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecordColumnDelimiter", &self.record_column_delimiter)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecordRowDelimiter", &self.record_row_delimiter)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CSVMappingParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CSVMappingParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CSVMappingParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CSVMappingParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut record_column_delimiter: Option<::Value<String>> = None;
                    let mut record_row_delimiter: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RecordColumnDelimiter" => {
                                record_column_delimiter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RecordRowDelimiter" => {
                                record_row_delimiter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CSVMappingParameters {
                        record_column_delimiter: record_column_delimiter.ok_or(::serde::de::Error::missing_field("RecordColumnDelimiter"))?,
                        record_row_delimiter: record_row_delimiter.ok_or(::serde::de::Error::missing_field("RecordRowDelimiter"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::Application.CatalogConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-catalogconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct CatalogConfiguration {
        /// Property [`GlueDataCatalogConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-catalogconfiguration.html#cfn-kinesisanalyticsv2-application-catalogconfiguration-gluedatacatalogconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub glue_data_catalog_configuration: Option<::Value<GlueDataCatalogConfiguration>>,
    }

    impl ::codec::SerializeValue for CatalogConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref glue_data_catalog_configuration) = self.glue_data_catalog_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GlueDataCatalogConfiguration", glue_data_catalog_configuration)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CatalogConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CatalogConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CatalogConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CatalogConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut glue_data_catalog_configuration: Option<::Value<GlueDataCatalogConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "GlueDataCatalogConfiguration" => {
                                glue_data_catalog_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CatalogConfiguration {
                        glue_data_catalog_configuration: glue_data_catalog_configuration,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::Application.CheckpointConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-checkpointconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct CheckpointConfiguration {
        /// Property [`CheckpointInterval`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-checkpointconfiguration.html#cfn-kinesisanalyticsv2-application-checkpointconfiguration-checkpointinterval).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub checkpoint_interval: Option<::Value<u32>>,
        /// Property [`CheckpointingEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-checkpointconfiguration.html#cfn-kinesisanalyticsv2-application-checkpointconfiguration-checkpointingenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub checkpointing_enabled: Option<::Value<bool>>,
        /// Property [`ConfigurationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-checkpointconfiguration.html#cfn-kinesisanalyticsv2-application-checkpointconfiguration-configurationtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub configuration_type: ::Value<String>,
        /// Property [`MinPauseBetweenCheckpoints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-checkpointconfiguration.html#cfn-kinesisanalyticsv2-application-checkpointconfiguration-minpausebetweencheckpoints).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min_pause_between_checkpoints: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for CheckpointConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref checkpoint_interval) = self.checkpoint_interval {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CheckpointInterval", checkpoint_interval)?;
            }
            if let Some(ref checkpointing_enabled) = self.checkpointing_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CheckpointingEnabled", checkpointing_enabled)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfigurationType", &self.configuration_type)?;
            if let Some(ref min_pause_between_checkpoints) = self.min_pause_between_checkpoints {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinPauseBetweenCheckpoints", min_pause_between_checkpoints)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CheckpointConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CheckpointConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CheckpointConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CheckpointConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut checkpoint_interval: Option<::Value<u32>> = None;
                    let mut checkpointing_enabled: Option<::Value<bool>> = None;
                    let mut configuration_type: Option<::Value<String>> = None;
                    let mut min_pause_between_checkpoints: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CheckpointInterval" => {
                                checkpoint_interval = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CheckpointingEnabled" => {
                                checkpointing_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConfigurationType" => {
                                configuration_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinPauseBetweenCheckpoints" => {
                                min_pause_between_checkpoints = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CheckpointConfiguration {
                        checkpoint_interval: checkpoint_interval,
                        checkpointing_enabled: checkpointing_enabled,
                        configuration_type: configuration_type.ok_or(::serde::de::Error::missing_field("ConfigurationType"))?,
                        min_pause_between_checkpoints: min_pause_between_checkpoints,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::Application.CodeContent`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-codecontent.html) property type.
    #[derive(Debug, Default)]
    pub struct CodeContent {
        /// Property [`S3ContentLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-codecontent.html#cfn-kinesisanalyticsv2-application-codecontent-s3contentlocation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_content_location: Option<::Value<S3ContentLocation>>,
        /// Property [`TextContent`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-codecontent.html#cfn-kinesisanalyticsv2-application-codecontent-textcontent).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text_content: Option<::Value<String>>,
        /// Property [`ZipFileContent`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-codecontent.html#cfn-kinesisanalyticsv2-application-codecontent-zipfilecontent).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub zip_file_content: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CodeContent {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref s3_content_location) = self.s3_content_location {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3ContentLocation", s3_content_location)?;
            }
            if let Some(ref text_content) = self.text_content {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextContent", text_content)?;
            }
            if let Some(ref zip_file_content) = self.zip_file_content {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ZipFileContent", zip_file_content)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CodeContent {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CodeContent, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CodeContent;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CodeContent")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut s3_content_location: Option<::Value<S3ContentLocation>> = None;
                    let mut text_content: Option<::Value<String>> = None;
                    let mut zip_file_content: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3ContentLocation" => {
                                s3_content_location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TextContent" => {
                                text_content = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ZipFileContent" => {
                                zip_file_content = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CodeContent {
                        s3_content_location: s3_content_location,
                        text_content: text_content,
                        zip_file_content: zip_file_content,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::Application.CustomArtifactConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-customartifactconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomArtifactConfiguration {
        /// Property [`ArtifactType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-customartifactconfiguration.html#cfn-kinesisanalyticsv2-application-customartifactconfiguration-artifacttype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub artifact_type: ::Value<String>,
        /// Property [`MavenReference`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-customartifactconfiguration.html#cfn-kinesisanalyticsv2-application-customartifactconfiguration-mavenreference).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub maven_reference: Option<::Value<MavenReference>>,
        /// Property [`S3ContentLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-customartifactconfiguration.html#cfn-kinesisanalyticsv2-application-customartifactconfiguration-s3contentlocation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_content_location: Option<::Value<S3ContentLocation>>,
    }

    impl ::codec::SerializeValue for CustomArtifactConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ArtifactType", &self.artifact_type)?;
            if let Some(ref maven_reference) = self.maven_reference {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MavenReference", maven_reference)?;
            }
            if let Some(ref s3_content_location) = self.s3_content_location {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3ContentLocation", s3_content_location)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomArtifactConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomArtifactConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomArtifactConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomArtifactConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut artifact_type: Option<::Value<String>> = None;
                    let mut maven_reference: Option<::Value<MavenReference>> = None;
                    let mut s3_content_location: Option<::Value<S3ContentLocation>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ArtifactType" => {
                                artifact_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MavenReference" => {
                                maven_reference = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3ContentLocation" => {
                                s3_content_location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomArtifactConfiguration {
                        artifact_type: artifact_type.ok_or(::serde::de::Error::missing_field("ArtifactType"))?,
                        maven_reference: maven_reference,
                        s3_content_location: s3_content_location,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::Application.DeployAsApplicationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-deployasapplicationconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct DeployAsApplicationConfiguration {
        /// Property [`S3ContentLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-deployasapplicationconfiguration.html#cfn-kinesisanalyticsv2-application-deployasapplicationconfiguration-s3contentlocation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_content_location: ::Value<S3ContentBaseLocation>,
    }

    impl ::codec::SerializeValue for DeployAsApplicationConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3ContentLocation", &self.s3_content_location)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DeployAsApplicationConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DeployAsApplicationConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DeployAsApplicationConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DeployAsApplicationConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut s3_content_location: Option<::Value<S3ContentBaseLocation>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3ContentLocation" => {
                                s3_content_location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DeployAsApplicationConfiguration {
                        s3_content_location: s3_content_location.ok_or(::serde::de::Error::missing_field("S3ContentLocation"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::Application.EnvironmentProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-environmentproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct EnvironmentProperties {
        /// Property [`PropertyGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-environmentproperties.html#cfn-kinesisanalyticsv2-application-environmentproperties-propertygroups).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub property_groups: Option<::ValueList<PropertyGroup>>,
    }

    impl ::codec::SerializeValue for EnvironmentProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref property_groups) = self.property_groups {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PropertyGroups", property_groups)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EnvironmentProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EnvironmentProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EnvironmentProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EnvironmentProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut property_groups: Option<::ValueList<PropertyGroup>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PropertyGroups" => {
                                property_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EnvironmentProperties {
                        property_groups: property_groups,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::Application.FlinkApplicationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-flinkapplicationconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct FlinkApplicationConfiguration {
        /// Property [`CheckpointConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-flinkapplicationconfiguration.html#cfn-kinesisanalyticsv2-application-flinkapplicationconfiguration-checkpointconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub checkpoint_configuration: Option<::Value<CheckpointConfiguration>>,
        /// Property [`MonitoringConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-flinkapplicationconfiguration.html#cfn-kinesisanalyticsv2-application-flinkapplicationconfiguration-monitoringconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub monitoring_configuration: Option<::Value<MonitoringConfiguration>>,
        /// Property [`ParallelismConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-flinkapplicationconfiguration.html#cfn-kinesisanalyticsv2-application-flinkapplicationconfiguration-parallelismconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parallelism_configuration: Option<::Value<ParallelismConfiguration>>,
    }

    impl ::codec::SerializeValue for FlinkApplicationConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref checkpoint_configuration) = self.checkpoint_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CheckpointConfiguration", checkpoint_configuration)?;
            }
            if let Some(ref monitoring_configuration) = self.monitoring_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MonitoringConfiguration", monitoring_configuration)?;
            }
            if let Some(ref parallelism_configuration) = self.parallelism_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParallelismConfiguration", parallelism_configuration)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FlinkApplicationConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FlinkApplicationConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FlinkApplicationConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FlinkApplicationConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut checkpoint_configuration: Option<::Value<CheckpointConfiguration>> = None;
                    let mut monitoring_configuration: Option<::Value<MonitoringConfiguration>> = None;
                    let mut parallelism_configuration: Option<::Value<ParallelismConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CheckpointConfiguration" => {
                                checkpoint_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MonitoringConfiguration" => {
                                monitoring_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ParallelismConfiguration" => {
                                parallelism_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FlinkApplicationConfiguration {
                        checkpoint_configuration: checkpoint_configuration,
                        monitoring_configuration: monitoring_configuration,
                        parallelism_configuration: parallelism_configuration,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::Application.FlinkRunConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-flinkrunconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct FlinkRunConfiguration {
        /// Property [`AllowNonRestoredState`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-flinkrunconfiguration.html#cfn-kinesisanalyticsv2-application-flinkrunconfiguration-allownonrestoredstate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allow_non_restored_state: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for FlinkRunConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref allow_non_restored_state) = self.allow_non_restored_state {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowNonRestoredState", allow_non_restored_state)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FlinkRunConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FlinkRunConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FlinkRunConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FlinkRunConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allow_non_restored_state: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllowNonRestoredState" => {
                                allow_non_restored_state = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FlinkRunConfiguration {
                        allow_non_restored_state: allow_non_restored_state,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::Application.GlueDataCatalogConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-gluedatacatalogconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct GlueDataCatalogConfiguration {
        /// Property [`DatabaseARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-gluedatacatalogconfiguration.html#cfn-kinesisanalyticsv2-application-gluedatacatalogconfiguration-databasearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for GlueDataCatalogConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref database_arn) = self.database_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseARN", database_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GlueDataCatalogConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GlueDataCatalogConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GlueDataCatalogConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GlueDataCatalogConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut database_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DatabaseARN" => {
                                database_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GlueDataCatalogConfiguration {
                        database_arn: database_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::Application.Input`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-input.html) property type.
    #[derive(Debug, Default)]
    pub struct Input {
        /// Property [`InputParallelism`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-input.html#cfn-kinesisanalyticsv2-application-input-inputparallelism).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_parallelism: Option<::Value<InputParallelism>>,
        /// Property [`InputProcessingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-input.html#cfn-kinesisanalyticsv2-application-input-inputprocessingconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_processing_configuration: Option<::Value<InputProcessingConfiguration>>,
        /// Property [`InputSchema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-input.html#cfn-kinesisanalyticsv2-application-input-inputschema).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_schema: ::Value<InputSchema>,
        /// Property [`KinesisFirehoseInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-input.html#cfn-kinesisanalyticsv2-application-input-kinesisfirehoseinput).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kinesis_firehose_input: Option<::Value<KinesisFirehoseInput>>,
        /// Property [`KinesisStreamsInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-input.html#cfn-kinesisanalyticsv2-application-input-kinesisstreamsinput).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kinesis_streams_input: Option<::Value<KinesisStreamsInput>>,
        /// Property [`NamePrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-input.html#cfn-kinesisanalyticsv2-application-input-nameprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name_prefix: ::Value<String>,
    }

    impl ::codec::SerializeValue for Input {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref input_parallelism) = self.input_parallelism {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputParallelism", input_parallelism)?;
            }
            if let Some(ref input_processing_configuration) = self.input_processing_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputProcessingConfiguration", input_processing_configuration)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputSchema", &self.input_schema)?;
            if let Some(ref kinesis_firehose_input) = self.kinesis_firehose_input {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KinesisFirehoseInput", kinesis_firehose_input)?;
            }
            if let Some(ref kinesis_streams_input) = self.kinesis_streams_input {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KinesisStreamsInput", kinesis_streams_input)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NamePrefix", &self.name_prefix)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Input {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Input, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Input;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Input")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut input_parallelism: Option<::Value<InputParallelism>> = None;
                    let mut input_processing_configuration: Option<::Value<InputProcessingConfiguration>> = None;
                    let mut input_schema: Option<::Value<InputSchema>> = None;
                    let mut kinesis_firehose_input: Option<::Value<KinesisFirehoseInput>> = None;
                    let mut kinesis_streams_input: Option<::Value<KinesisStreamsInput>> = None;
                    let mut name_prefix: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InputParallelism" => {
                                input_parallelism = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InputProcessingConfiguration" => {
                                input_processing_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InputSchema" => {
                                input_schema = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KinesisFirehoseInput" => {
                                kinesis_firehose_input = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KinesisStreamsInput" => {
                                kinesis_streams_input = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NamePrefix" => {
                                name_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Input {
                        input_parallelism: input_parallelism,
                        input_processing_configuration: input_processing_configuration,
                        input_schema: input_schema.ok_or(::serde::de::Error::missing_field("InputSchema"))?,
                        kinesis_firehose_input: kinesis_firehose_input,
                        kinesis_streams_input: kinesis_streams_input,
                        name_prefix: name_prefix.ok_or(::serde::de::Error::missing_field("NamePrefix"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::Application.InputLambdaProcessor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-inputlambdaprocessor.html) property type.
    #[derive(Debug, Default)]
    pub struct InputLambdaProcessor {
        /// Property [`ResourceARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-inputlambdaprocessor.html#cfn-kinesisanalyticsv2-application-inputlambdaprocessor-resourcearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for InputLambdaProcessor {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceARN", &self.resource_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InputLambdaProcessor {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InputLambdaProcessor, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InputLambdaProcessor;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InputLambdaProcessor")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut resource_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ResourceARN" => {
                                resource_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InputLambdaProcessor {
                        resource_arn: resource_arn.ok_or(::serde::de::Error::missing_field("ResourceARN"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::Application.InputParallelism`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-inputparallelism.html) property type.
    #[derive(Debug, Default)]
    pub struct InputParallelism {
        /// Property [`Count`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-inputparallelism.html#cfn-kinesisanalyticsv2-application-inputparallelism-count).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub count: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for InputParallelism {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref count) = self.count {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Count", count)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InputParallelism {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InputParallelism, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InputParallelism;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InputParallelism")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut count: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Count" => {
                                count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InputParallelism {
                        count: count,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::Application.InputProcessingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-inputprocessingconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct InputProcessingConfiguration {
        /// Property [`InputLambdaProcessor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-inputprocessingconfiguration.html#cfn-kinesisanalyticsv2-application-inputprocessingconfiguration-inputlambdaprocessor).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_lambda_processor: Option<::Value<InputLambdaProcessor>>,
    }

    impl ::codec::SerializeValue for InputProcessingConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref input_lambda_processor) = self.input_lambda_processor {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputLambdaProcessor", input_lambda_processor)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InputProcessingConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InputProcessingConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InputProcessingConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InputProcessingConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut input_lambda_processor: Option<::Value<InputLambdaProcessor>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InputLambdaProcessor" => {
                                input_lambda_processor = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InputProcessingConfiguration {
                        input_lambda_processor: input_lambda_processor,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::Application.InputSchema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-inputschema.html) property type.
    #[derive(Debug, Default)]
    pub struct InputSchema {
        /// Property [`RecordColumns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-inputschema.html#cfn-kinesisanalyticsv2-application-inputschema-recordcolumns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub record_columns: ::ValueList<RecordColumn>,
        /// Property [`RecordEncoding`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-inputschema.html#cfn-kinesisanalyticsv2-application-inputschema-recordencoding).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub record_encoding: Option<::Value<String>>,
        /// Property [`RecordFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-inputschema.html#cfn-kinesisanalyticsv2-application-inputschema-recordformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub record_format: ::Value<RecordFormat>,
    }

    impl ::codec::SerializeValue for InputSchema {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecordColumns", &self.record_columns)?;
            if let Some(ref record_encoding) = self.record_encoding {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecordEncoding", record_encoding)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecordFormat", &self.record_format)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InputSchema {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InputSchema, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InputSchema;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InputSchema")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut record_columns: Option<::ValueList<RecordColumn>> = None;
                    let mut record_encoding: Option<::Value<String>> = None;
                    let mut record_format: Option<::Value<RecordFormat>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RecordColumns" => {
                                record_columns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RecordEncoding" => {
                                record_encoding = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RecordFormat" => {
                                record_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InputSchema {
                        record_columns: record_columns.ok_or(::serde::de::Error::missing_field("RecordColumns"))?,
                        record_encoding: record_encoding,
                        record_format: record_format.ok_or(::serde::de::Error::missing_field("RecordFormat"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::Application.JSONMappingParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-jsonmappingparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct JSONMappingParameters {
        /// Property [`RecordRowPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-jsonmappingparameters.html#cfn-kinesisanalyticsv2-application-jsonmappingparameters-recordrowpath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub record_row_path: ::Value<String>,
    }

    impl ::codec::SerializeValue for JSONMappingParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecordRowPath", &self.record_row_path)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for JSONMappingParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<JSONMappingParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = JSONMappingParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type JSONMappingParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut record_row_path: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RecordRowPath" => {
                                record_row_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(JSONMappingParameters {
                        record_row_path: record_row_path.ok_or(::serde::de::Error::missing_field("RecordRowPath"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::Application.KinesisFirehoseInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-kinesisfirehoseinput.html) property type.
    #[derive(Debug, Default)]
    pub struct KinesisFirehoseInput {
        /// Property [`ResourceARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-kinesisfirehoseinput.html#cfn-kinesisanalyticsv2-application-kinesisfirehoseinput-resourcearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for KinesisFirehoseInput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceARN", &self.resource_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KinesisFirehoseInput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KinesisFirehoseInput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KinesisFirehoseInput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KinesisFirehoseInput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut resource_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ResourceARN" => {
                                resource_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KinesisFirehoseInput {
                        resource_arn: resource_arn.ok_or(::serde::de::Error::missing_field("ResourceARN"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::Application.KinesisStreamsInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-kinesisstreamsinput.html) property type.
    #[derive(Debug, Default)]
    pub struct KinesisStreamsInput {
        /// Property [`ResourceARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-kinesisstreamsinput.html#cfn-kinesisanalyticsv2-application-kinesisstreamsinput-resourcearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for KinesisStreamsInput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceARN", &self.resource_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KinesisStreamsInput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KinesisStreamsInput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KinesisStreamsInput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KinesisStreamsInput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut resource_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ResourceARN" => {
                                resource_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KinesisStreamsInput {
                        resource_arn: resource_arn.ok_or(::serde::de::Error::missing_field("ResourceARN"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::Application.MappingParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-mappingparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct MappingParameters {
        /// Property [`CSVMappingParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-mappingparameters.html#cfn-kinesisanalyticsv2-application-mappingparameters-csvmappingparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub csv_mapping_parameters: Option<::Value<CSVMappingParameters>>,
        /// Property [`JSONMappingParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-mappingparameters.html#cfn-kinesisanalyticsv2-application-mappingparameters-jsonmappingparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub json_mapping_parameters: Option<::Value<JSONMappingParameters>>,
    }

    impl ::codec::SerializeValue for MappingParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref csv_mapping_parameters) = self.csv_mapping_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CSVMappingParameters", csv_mapping_parameters)?;
            }
            if let Some(ref json_mapping_parameters) = self.json_mapping_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "JSONMappingParameters", json_mapping_parameters)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MappingParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MappingParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MappingParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MappingParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut csv_mapping_parameters: Option<::Value<CSVMappingParameters>> = None;
                    let mut json_mapping_parameters: Option<::Value<JSONMappingParameters>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CSVMappingParameters" => {
                                csv_mapping_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "JSONMappingParameters" => {
                                json_mapping_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MappingParameters {
                        csv_mapping_parameters: csv_mapping_parameters,
                        json_mapping_parameters: json_mapping_parameters,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::Application.MavenReference`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-mavenreference.html) property type.
    #[derive(Debug, Default)]
    pub struct MavenReference {
        /// Property [`ArtifactId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-mavenreference.html#cfn-kinesisanalyticsv2-application-mavenreference-artifactid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub artifact_id: ::Value<String>,
        /// Property [`GroupId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-mavenreference.html#cfn-kinesisanalyticsv2-application-mavenreference-groupid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub group_id: ::Value<String>,
        /// Property [`Version`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-mavenreference.html#cfn-kinesisanalyticsv2-application-mavenreference-version).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub version: ::Value<String>,
    }

    impl ::codec::SerializeValue for MavenReference {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ArtifactId", &self.artifact_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupId", &self.group_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Version", &self.version)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MavenReference {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MavenReference, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MavenReference;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MavenReference")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut artifact_id: Option<::Value<String>> = None;
                    let mut group_id: Option<::Value<String>> = None;
                    let mut version: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ArtifactId" => {
                                artifact_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GroupId" => {
                                group_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Version" => {
                                version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MavenReference {
                        artifact_id: artifact_id.ok_or(::serde::de::Error::missing_field("ArtifactId"))?,
                        group_id: group_id.ok_or(::serde::de::Error::missing_field("GroupId"))?,
                        version: version.ok_or(::serde::de::Error::missing_field("Version"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::Application.MonitoringConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-monitoringconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct MonitoringConfiguration {
        /// Property [`ConfigurationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-monitoringconfiguration.html#cfn-kinesisanalyticsv2-application-monitoringconfiguration-configurationtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub configuration_type: ::Value<String>,
        /// Property [`LogLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-monitoringconfiguration.html#cfn-kinesisanalyticsv2-application-monitoringconfiguration-loglevel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_level: Option<::Value<String>>,
        /// Property [`MetricsLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-monitoringconfiguration.html#cfn-kinesisanalyticsv2-application-monitoringconfiguration-metricslevel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metrics_level: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for MonitoringConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfigurationType", &self.configuration_type)?;
            if let Some(ref log_level) = self.log_level {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogLevel", log_level)?;
            }
            if let Some(ref metrics_level) = self.metrics_level {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricsLevel", metrics_level)?;
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
                    let mut configuration_type: Option<::Value<String>> = None;
                    let mut log_level: Option<::Value<String>> = None;
                    let mut metrics_level: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConfigurationType" => {
                                configuration_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogLevel" => {
                                log_level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetricsLevel" => {
                                metrics_level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MonitoringConfiguration {
                        configuration_type: configuration_type.ok_or(::serde::de::Error::missing_field("ConfigurationType"))?,
                        log_level: log_level,
                        metrics_level: metrics_level,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::Application.ParallelismConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-parallelismconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ParallelismConfiguration {
        /// Property [`AutoScalingEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-parallelismconfiguration.html#cfn-kinesisanalyticsv2-application-parallelismconfiguration-autoscalingenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub auto_scaling_enabled: Option<::Value<bool>>,
        /// Property [`ConfigurationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-parallelismconfiguration.html#cfn-kinesisanalyticsv2-application-parallelismconfiguration-configurationtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub configuration_type: ::Value<String>,
        /// Property [`Parallelism`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-parallelismconfiguration.html#cfn-kinesisanalyticsv2-application-parallelismconfiguration-parallelism).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parallelism: Option<::Value<u32>>,
        /// Property [`ParallelismPerKPU`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-parallelismconfiguration.html#cfn-kinesisanalyticsv2-application-parallelismconfiguration-parallelismperkpu).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parallelism_per_kpu: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for ParallelismConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref auto_scaling_enabled) = self.auto_scaling_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoScalingEnabled", auto_scaling_enabled)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfigurationType", &self.configuration_type)?;
            if let Some(ref parallelism) = self.parallelism {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parallelism", parallelism)?;
            }
            if let Some(ref parallelism_per_kpu) = self.parallelism_per_kpu {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParallelismPerKPU", parallelism_per_kpu)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ParallelismConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ParallelismConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ParallelismConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ParallelismConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut auto_scaling_enabled: Option<::Value<bool>> = None;
                    let mut configuration_type: Option<::Value<String>> = None;
                    let mut parallelism: Option<::Value<u32>> = None;
                    let mut parallelism_per_kpu: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AutoScalingEnabled" => {
                                auto_scaling_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConfigurationType" => {
                                configuration_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Parallelism" => {
                                parallelism = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ParallelismPerKPU" => {
                                parallelism_per_kpu = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ParallelismConfiguration {
                        auto_scaling_enabled: auto_scaling_enabled,
                        configuration_type: configuration_type.ok_or(::serde::de::Error::missing_field("ConfigurationType"))?,
                        parallelism: parallelism,
                        parallelism_per_kpu: parallelism_per_kpu,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::Application.PropertyGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-propertygroup.html) property type.
    #[derive(Debug, Default)]
    pub struct PropertyGroup {
        /// Property [`PropertyGroupId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-propertygroup.html#cfn-kinesisanalyticsv2-application-propertygroup-propertygroupid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub property_group_id: Option<::Value<String>>,
        /// Property [`PropertyMap`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-propertygroup.html#cfn-kinesisanalyticsv2-application-propertygroup-propertymap).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub property_map: Option<::ValueMap<String>>,
    }

    impl ::codec::SerializeValue for PropertyGroup {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref property_group_id) = self.property_group_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PropertyGroupId", property_group_id)?;
            }
            if let Some(ref property_map) = self.property_map {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PropertyMap", property_map)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PropertyGroup {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PropertyGroup, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PropertyGroup;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PropertyGroup")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut property_group_id: Option<::Value<String>> = None;
                    let mut property_map: Option<::ValueMap<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PropertyGroupId" => {
                                property_group_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PropertyMap" => {
                                property_map = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PropertyGroup {
                        property_group_id: property_group_id,
                        property_map: property_map,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::Application.RecordColumn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-recordcolumn.html) property type.
    #[derive(Debug, Default)]
    pub struct RecordColumn {
        /// Property [`Mapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-recordcolumn.html#cfn-kinesisanalyticsv2-application-recordcolumn-mapping).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mapping: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-recordcolumn.html#cfn-kinesisanalyticsv2-application-recordcolumn-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`SqlType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-recordcolumn.html#cfn-kinesisanalyticsv2-application-recordcolumn-sqltype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sql_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for RecordColumn {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref mapping) = self.mapping {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Mapping", mapping)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SqlType", &self.sql_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RecordColumn {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RecordColumn, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RecordColumn;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RecordColumn")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut mapping: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut sql_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Mapping" => {
                                mapping = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SqlType" => {
                                sql_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RecordColumn {
                        mapping: mapping,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        sql_type: sql_type.ok_or(::serde::de::Error::missing_field("SqlType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::Application.RecordFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-recordformat.html) property type.
    #[derive(Debug, Default)]
    pub struct RecordFormat {
        /// Property [`MappingParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-recordformat.html#cfn-kinesisanalyticsv2-application-recordformat-mappingparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mapping_parameters: Option<::Value<MappingParameters>>,
        /// Property [`RecordFormatType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-recordformat.html#cfn-kinesisanalyticsv2-application-recordformat-recordformattype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub record_format_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for RecordFormat {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref mapping_parameters) = self.mapping_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MappingParameters", mapping_parameters)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecordFormatType", &self.record_format_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RecordFormat {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RecordFormat, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RecordFormat;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RecordFormat")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut mapping_parameters: Option<::Value<MappingParameters>> = None;
                    let mut record_format_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MappingParameters" => {
                                mapping_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RecordFormatType" => {
                                record_format_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RecordFormat {
                        mapping_parameters: mapping_parameters,
                        record_format_type: record_format_type.ok_or(::serde::de::Error::missing_field("RecordFormatType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::Application.RunConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-runconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct RunConfiguration {
        /// Property [`ApplicationRestoreConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-runconfiguration.html#cfn-kinesisanalyticsv2-application-runconfiguration-applicationrestoreconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub application_restore_configuration: Option<::Value<ApplicationRestoreConfiguration>>,
        /// Property [`FlinkRunConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-runconfiguration.html#cfn-kinesisanalyticsv2-application-runconfiguration-flinkrunconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub flink_run_configuration: Option<::Value<FlinkRunConfiguration>>,
    }

    impl ::codec::SerializeValue for RunConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref application_restore_configuration) = self.application_restore_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationRestoreConfiguration", application_restore_configuration)?;
            }
            if let Some(ref flink_run_configuration) = self.flink_run_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FlinkRunConfiguration", flink_run_configuration)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RunConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RunConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RunConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RunConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut application_restore_configuration: Option<::Value<ApplicationRestoreConfiguration>> = None;
                    let mut flink_run_configuration: Option<::Value<FlinkRunConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ApplicationRestoreConfiguration" => {
                                application_restore_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FlinkRunConfiguration" => {
                                flink_run_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RunConfiguration {
                        application_restore_configuration: application_restore_configuration,
                        flink_run_configuration: flink_run_configuration,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::Application.S3ContentBaseLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-s3contentbaselocation.html) property type.
    #[derive(Debug, Default)]
    pub struct S3ContentBaseLocation {
        /// Property [`BasePath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-s3contentbaselocation.html#cfn-kinesisanalyticsv2-application-s3contentbaselocation-basepath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub base_path: Option<::Value<String>>,
        /// Property [`BucketARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-s3contentbaselocation.html#cfn-kinesisanalyticsv2-application-s3contentbaselocation-bucketarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for S3ContentBaseLocation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref base_path) = self.base_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BasePath", base_path)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketARN", &self.bucket_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3ContentBaseLocation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3ContentBaseLocation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3ContentBaseLocation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3ContentBaseLocation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut base_path: Option<::Value<String>> = None;
                    let mut bucket_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BasePath" => {
                                base_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BucketARN" => {
                                bucket_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3ContentBaseLocation {
                        base_path: base_path,
                        bucket_arn: bucket_arn.ok_or(::serde::de::Error::missing_field("BucketARN"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::Application.S3ContentLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-s3contentlocation.html) property type.
    #[derive(Debug, Default)]
    pub struct S3ContentLocation {
        /// Property [`BucketARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-s3contentlocation.html#cfn-kinesisanalyticsv2-application-s3contentlocation-bucketarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_arn: ::Value<String>,
        /// Property [`FileKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-s3contentlocation.html#cfn-kinesisanalyticsv2-application-s3contentlocation-filekey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub file_key: ::Value<String>,
        /// Property [`ObjectVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-s3contentlocation.html#cfn-kinesisanalyticsv2-application-s3contentlocation-objectversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub object_version: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for S3ContentLocation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketARN", &self.bucket_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FileKey", &self.file_key)?;
            if let Some(ref object_version) = self.object_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ObjectVersion", object_version)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3ContentLocation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3ContentLocation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3ContentLocation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3ContentLocation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket_arn: Option<::Value<String>> = None;
                    let mut file_key: Option<::Value<String>> = None;
                    let mut object_version: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketARN" => {
                                bucket_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FileKey" => {
                                file_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ObjectVersion" => {
                                object_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3ContentLocation {
                        bucket_arn: bucket_arn.ok_or(::serde::de::Error::missing_field("BucketARN"))?,
                        file_key: file_key.ok_or(::serde::de::Error::missing_field("FileKey"))?,
                        object_version: object_version,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::Application.SqlApplicationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-sqlapplicationconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct SqlApplicationConfiguration {
        /// Property [`Inputs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-sqlapplicationconfiguration.html#cfn-kinesisanalyticsv2-application-sqlapplicationconfiguration-inputs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub inputs: Option<::ValueList<Input>>,
    }

    impl ::codec::SerializeValue for SqlApplicationConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref inputs) = self.inputs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Inputs", inputs)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SqlApplicationConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SqlApplicationConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SqlApplicationConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SqlApplicationConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut inputs: Option<::ValueList<Input>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Inputs" => {
                                inputs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SqlApplicationConfiguration {
                        inputs: inputs,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::Application.VpcConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-vpcconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct VpcConfiguration {
        /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-vpcconfiguration.html#cfn-kinesisanalyticsv2-application-vpcconfiguration-securitygroupids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_group_ids: ::ValueList<String>,
        /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-vpcconfiguration.html#cfn-kinesisanalyticsv2-application-vpcconfiguration-subnetids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subnet_ids: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for VpcConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", &self.security_group_ids)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", &self.subnet_ids)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VpcConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VpcConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VpcConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VpcConfiguration")
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

                    Ok(VpcConfiguration {
                        security_group_ids: security_group_ids.ok_or(::serde::de::Error::missing_field("SecurityGroupIds"))?,
                        subnet_ids: subnet_ids.ok_or(::serde::de::Error::missing_field("SubnetIds"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::Application.ZeppelinApplicationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-zeppelinapplicationconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ZeppelinApplicationConfiguration {
        /// Property [`CatalogConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-zeppelinapplicationconfiguration.html#cfn-kinesisanalyticsv2-application-zeppelinapplicationconfiguration-catalogconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub catalog_configuration: Option<::Value<CatalogConfiguration>>,
        /// Property [`CustomArtifactsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-zeppelinapplicationconfiguration.html#cfn-kinesisanalyticsv2-application-zeppelinapplicationconfiguration-customartifactsconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_artifacts_configuration: Option<::ValueList<CustomArtifactConfiguration>>,
        /// Property [`DeployAsApplicationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-zeppelinapplicationconfiguration.html#cfn-kinesisanalyticsv2-application-zeppelinapplicationconfiguration-deployasapplicationconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub deploy_as_application_configuration: Option<::Value<DeployAsApplicationConfiguration>>,
        /// Property [`MonitoringConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-zeppelinapplicationconfiguration.html#cfn-kinesisanalyticsv2-application-zeppelinapplicationconfiguration-monitoringconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub monitoring_configuration: Option<::Value<ZeppelinMonitoringConfiguration>>,
    }

    impl ::codec::SerializeValue for ZeppelinApplicationConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref catalog_configuration) = self.catalog_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CatalogConfiguration", catalog_configuration)?;
            }
            if let Some(ref custom_artifacts_configuration) = self.custom_artifacts_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomArtifactsConfiguration", custom_artifacts_configuration)?;
            }
            if let Some(ref deploy_as_application_configuration) = self.deploy_as_application_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeployAsApplicationConfiguration", deploy_as_application_configuration)?;
            }
            if let Some(ref monitoring_configuration) = self.monitoring_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MonitoringConfiguration", monitoring_configuration)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ZeppelinApplicationConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ZeppelinApplicationConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ZeppelinApplicationConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ZeppelinApplicationConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut catalog_configuration: Option<::Value<CatalogConfiguration>> = None;
                    let mut custom_artifacts_configuration: Option<::ValueList<CustomArtifactConfiguration>> = None;
                    let mut deploy_as_application_configuration: Option<::Value<DeployAsApplicationConfiguration>> = None;
                    let mut monitoring_configuration: Option<::Value<ZeppelinMonitoringConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CatalogConfiguration" => {
                                catalog_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CustomArtifactsConfiguration" => {
                                custom_artifacts_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeployAsApplicationConfiguration" => {
                                deploy_as_application_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MonitoringConfiguration" => {
                                monitoring_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ZeppelinApplicationConfiguration {
                        catalog_configuration: catalog_configuration,
                        custom_artifacts_configuration: custom_artifacts_configuration,
                        deploy_as_application_configuration: deploy_as_application_configuration,
                        monitoring_configuration: monitoring_configuration,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::Application.ZeppelinMonitoringConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-zeppelinmonitoringconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ZeppelinMonitoringConfiguration {
        /// Property [`LogLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-zeppelinmonitoringconfiguration.html#cfn-kinesisanalyticsv2-application-zeppelinmonitoringconfiguration-loglevel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_level: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ZeppelinMonitoringConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref log_level) = self.log_level {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogLevel", log_level)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ZeppelinMonitoringConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ZeppelinMonitoringConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ZeppelinMonitoringConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ZeppelinMonitoringConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut log_level: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LogLevel" => {
                                log_level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ZeppelinMonitoringConfiguration {
                        log_level: log_level,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod application_cloud_watch_logging_option {
    //! Property types for the `ApplicationCloudWatchLoggingOption` resource.

    /// The [`AWS::KinesisAnalyticsV2::ApplicationCloudWatchLoggingOption.CloudWatchLoggingOption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationcloudwatchloggingoption-cloudwatchloggingoption.html) property type.
    #[derive(Debug, Default)]
    pub struct CloudWatchLoggingOption {
        /// Property [`LogStreamARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationcloudwatchloggingoption-cloudwatchloggingoption.html#cfn-kinesisanalyticsv2-applicationcloudwatchloggingoption-cloudwatchloggingoption-logstreamarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_stream_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for CloudWatchLoggingOption {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogStreamARN", &self.log_stream_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CloudWatchLoggingOption {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CloudWatchLoggingOption, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CloudWatchLoggingOption;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CloudWatchLoggingOption")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut log_stream_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LogStreamARN" => {
                                log_stream_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CloudWatchLoggingOption {
                        log_stream_arn: log_stream_arn.ok_or(::serde::de::Error::missing_field("LogStreamARN"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod application_output {
    //! Property types for the `ApplicationOutput` resource.

    /// The [`AWS::KinesisAnalyticsV2::ApplicationOutput.DestinationSchema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationoutput-destinationschema.html) property type.
    #[derive(Debug, Default)]
    pub struct DestinationSchema {
        /// Property [`RecordFormatType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationoutput-destinationschema.html#cfn-kinesisanalyticsv2-applicationoutput-destinationschema-recordformattype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub record_format_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DestinationSchema {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref record_format_type) = self.record_format_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecordFormatType", record_format_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DestinationSchema {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DestinationSchema, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DestinationSchema;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DestinationSchema")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut record_format_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RecordFormatType" => {
                                record_format_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DestinationSchema {
                        record_format_type: record_format_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::ApplicationOutput.KinesisFirehoseOutput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationoutput-kinesisfirehoseoutput.html) property type.
    #[derive(Debug, Default)]
    pub struct KinesisFirehoseOutput {
        /// Property [`ResourceARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationoutput-kinesisfirehoseoutput.html#cfn-kinesisanalyticsv2-applicationoutput-kinesisfirehoseoutput-resourcearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for KinesisFirehoseOutput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceARN", &self.resource_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KinesisFirehoseOutput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KinesisFirehoseOutput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KinesisFirehoseOutput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KinesisFirehoseOutput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut resource_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ResourceARN" => {
                                resource_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KinesisFirehoseOutput {
                        resource_arn: resource_arn.ok_or(::serde::de::Error::missing_field("ResourceARN"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::ApplicationOutput.KinesisStreamsOutput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationoutput-kinesisstreamsoutput.html) property type.
    #[derive(Debug, Default)]
    pub struct KinesisStreamsOutput {
        /// Property [`ResourceARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationoutput-kinesisstreamsoutput.html#cfn-kinesisanalyticsv2-applicationoutput-kinesisstreamsoutput-resourcearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for KinesisStreamsOutput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceARN", &self.resource_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KinesisStreamsOutput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KinesisStreamsOutput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KinesisStreamsOutput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KinesisStreamsOutput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut resource_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ResourceARN" => {
                                resource_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KinesisStreamsOutput {
                        resource_arn: resource_arn.ok_or(::serde::de::Error::missing_field("ResourceARN"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::ApplicationOutput.LambdaOutput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationoutput-lambdaoutput.html) property type.
    #[derive(Debug, Default)]
    pub struct LambdaOutput {
        /// Property [`ResourceARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationoutput-lambdaoutput.html#cfn-kinesisanalyticsv2-applicationoutput-lambdaoutput-resourcearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for LambdaOutput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceARN", &self.resource_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LambdaOutput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LambdaOutput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LambdaOutput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LambdaOutput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut resource_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ResourceARN" => {
                                resource_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LambdaOutput {
                        resource_arn: resource_arn.ok_or(::serde::de::Error::missing_field("ResourceARN"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::ApplicationOutput.Output`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationoutput-output.html) property type.
    #[derive(Debug, Default)]
    pub struct Output {
        /// Property [`DestinationSchema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationoutput-output.html#cfn-kinesisanalyticsv2-applicationoutput-output-destinationschema).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination_schema: ::Value<DestinationSchema>,
        /// Property [`KinesisFirehoseOutput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationoutput-output.html#cfn-kinesisanalyticsv2-applicationoutput-output-kinesisfirehoseoutput).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kinesis_firehose_output: Option<::Value<KinesisFirehoseOutput>>,
        /// Property [`KinesisStreamsOutput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationoutput-output.html#cfn-kinesisanalyticsv2-applicationoutput-output-kinesisstreamsoutput).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kinesis_streams_output: Option<::Value<KinesisStreamsOutput>>,
        /// Property [`LambdaOutput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationoutput-output.html#cfn-kinesisanalyticsv2-applicationoutput-output-lambdaoutput).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lambda_output: Option<::Value<LambdaOutput>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationoutput-output.html#cfn-kinesisanalyticsv2-applicationoutput-output-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Output {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationSchema", &self.destination_schema)?;
            if let Some(ref kinesis_firehose_output) = self.kinesis_firehose_output {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KinesisFirehoseOutput", kinesis_firehose_output)?;
            }
            if let Some(ref kinesis_streams_output) = self.kinesis_streams_output {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KinesisStreamsOutput", kinesis_streams_output)?;
            }
            if let Some(ref lambda_output) = self.lambda_output {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaOutput", lambda_output)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Output {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Output, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Output;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Output")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destination_schema: Option<::Value<DestinationSchema>> = None;
                    let mut kinesis_firehose_output: Option<::Value<KinesisFirehoseOutput>> = None;
                    let mut kinesis_streams_output: Option<::Value<KinesisStreamsOutput>> = None;
                    let mut lambda_output: Option<::Value<LambdaOutput>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DestinationSchema" => {
                                destination_schema = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KinesisFirehoseOutput" => {
                                kinesis_firehose_output = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KinesisStreamsOutput" => {
                                kinesis_streams_output = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LambdaOutput" => {
                                lambda_output = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Output {
                        destination_schema: destination_schema.ok_or(::serde::de::Error::missing_field("DestinationSchema"))?,
                        kinesis_firehose_output: kinesis_firehose_output,
                        kinesis_streams_output: kinesis_streams_output,
                        lambda_output: lambda_output,
                        name: name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod application_reference_data_source {
    //! Property types for the `ApplicationReferenceDataSource` resource.

    /// The [`AWS::KinesisAnalyticsV2::ApplicationReferenceDataSource.CSVMappingParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationreferencedatasource-csvmappingparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct CSVMappingParameters {
        /// Property [`RecordColumnDelimiter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationreferencedatasource-csvmappingparameters.html#cfn-kinesisanalyticsv2-applicationreferencedatasource-csvmappingparameters-recordcolumndelimiter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub record_column_delimiter: ::Value<String>,
        /// Property [`RecordRowDelimiter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationreferencedatasource-csvmappingparameters.html#cfn-kinesisanalyticsv2-applicationreferencedatasource-csvmappingparameters-recordrowdelimiter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub record_row_delimiter: ::Value<String>,
    }

    impl ::codec::SerializeValue for CSVMappingParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecordColumnDelimiter", &self.record_column_delimiter)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecordRowDelimiter", &self.record_row_delimiter)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CSVMappingParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CSVMappingParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CSVMappingParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CSVMappingParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut record_column_delimiter: Option<::Value<String>> = None;
                    let mut record_row_delimiter: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RecordColumnDelimiter" => {
                                record_column_delimiter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RecordRowDelimiter" => {
                                record_row_delimiter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CSVMappingParameters {
                        record_column_delimiter: record_column_delimiter.ok_or(::serde::de::Error::missing_field("RecordColumnDelimiter"))?,
                        record_row_delimiter: record_row_delimiter.ok_or(::serde::de::Error::missing_field("RecordRowDelimiter"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::ApplicationReferenceDataSource.JSONMappingParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationreferencedatasource-jsonmappingparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct JSONMappingParameters {
        /// Property [`RecordRowPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationreferencedatasource-jsonmappingparameters.html#cfn-kinesisanalyticsv2-applicationreferencedatasource-jsonmappingparameters-recordrowpath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub record_row_path: ::Value<String>,
    }

    impl ::codec::SerializeValue for JSONMappingParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecordRowPath", &self.record_row_path)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for JSONMappingParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<JSONMappingParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = JSONMappingParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type JSONMappingParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut record_row_path: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RecordRowPath" => {
                                record_row_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(JSONMappingParameters {
                        record_row_path: record_row_path.ok_or(::serde::de::Error::missing_field("RecordRowPath"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::ApplicationReferenceDataSource.MappingParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationreferencedatasource-mappingparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct MappingParameters {
        /// Property [`CSVMappingParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationreferencedatasource-mappingparameters.html#cfn-kinesisanalyticsv2-applicationreferencedatasource-mappingparameters-csvmappingparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub csv_mapping_parameters: Option<::Value<CSVMappingParameters>>,
        /// Property [`JSONMappingParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationreferencedatasource-mappingparameters.html#cfn-kinesisanalyticsv2-applicationreferencedatasource-mappingparameters-jsonmappingparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub json_mapping_parameters: Option<::Value<JSONMappingParameters>>,
    }

    impl ::codec::SerializeValue for MappingParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref csv_mapping_parameters) = self.csv_mapping_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CSVMappingParameters", csv_mapping_parameters)?;
            }
            if let Some(ref json_mapping_parameters) = self.json_mapping_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "JSONMappingParameters", json_mapping_parameters)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MappingParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MappingParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MappingParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MappingParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut csv_mapping_parameters: Option<::Value<CSVMappingParameters>> = None;
                    let mut json_mapping_parameters: Option<::Value<JSONMappingParameters>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CSVMappingParameters" => {
                                csv_mapping_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "JSONMappingParameters" => {
                                json_mapping_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MappingParameters {
                        csv_mapping_parameters: csv_mapping_parameters,
                        json_mapping_parameters: json_mapping_parameters,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::ApplicationReferenceDataSource.RecordColumn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationreferencedatasource-recordcolumn.html) property type.
    #[derive(Debug, Default)]
    pub struct RecordColumn {
        /// Property [`Mapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationreferencedatasource-recordcolumn.html#cfn-kinesisanalyticsv2-applicationreferencedatasource-recordcolumn-mapping).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mapping: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationreferencedatasource-recordcolumn.html#cfn-kinesisanalyticsv2-applicationreferencedatasource-recordcolumn-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`SqlType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationreferencedatasource-recordcolumn.html#cfn-kinesisanalyticsv2-applicationreferencedatasource-recordcolumn-sqltype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sql_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for RecordColumn {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref mapping) = self.mapping {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Mapping", mapping)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SqlType", &self.sql_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RecordColumn {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RecordColumn, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RecordColumn;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RecordColumn")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut mapping: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut sql_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Mapping" => {
                                mapping = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SqlType" => {
                                sql_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RecordColumn {
                        mapping: mapping,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        sql_type: sql_type.ok_or(::serde::de::Error::missing_field("SqlType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::ApplicationReferenceDataSource.RecordFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationreferencedatasource-recordformat.html) property type.
    #[derive(Debug, Default)]
    pub struct RecordFormat {
        /// Property [`MappingParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationreferencedatasource-recordformat.html#cfn-kinesisanalyticsv2-applicationreferencedatasource-recordformat-mappingparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mapping_parameters: Option<::Value<MappingParameters>>,
        /// Property [`RecordFormatType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationreferencedatasource-recordformat.html#cfn-kinesisanalyticsv2-applicationreferencedatasource-recordformat-recordformattype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub record_format_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for RecordFormat {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref mapping_parameters) = self.mapping_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MappingParameters", mapping_parameters)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecordFormatType", &self.record_format_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RecordFormat {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RecordFormat, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RecordFormat;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RecordFormat")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut mapping_parameters: Option<::Value<MappingParameters>> = None;
                    let mut record_format_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MappingParameters" => {
                                mapping_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RecordFormatType" => {
                                record_format_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RecordFormat {
                        mapping_parameters: mapping_parameters,
                        record_format_type: record_format_type.ok_or(::serde::de::Error::missing_field("RecordFormatType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::ApplicationReferenceDataSource.ReferenceDataSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationreferencedatasource-referencedatasource.html) property type.
    #[derive(Debug, Default)]
    pub struct ReferenceDataSource {
        /// Property [`ReferenceSchema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationreferencedatasource-referencedatasource.html#cfn-kinesisanalyticsv2-applicationreferencedatasource-referencedatasource-referenceschema).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub reference_schema: ::Value<ReferenceSchema>,
        /// Property [`S3ReferenceDataSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationreferencedatasource-referencedatasource.html#cfn-kinesisanalyticsv2-applicationreferencedatasource-referencedatasource-s3referencedatasource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_reference_data_source: Option<::Value<S3ReferenceDataSource>>,
        /// Property [`TableName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationreferencedatasource-referencedatasource.html#cfn-kinesisanalyticsv2-applicationreferencedatasource-referencedatasource-tablename).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub table_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ReferenceDataSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReferenceSchema", &self.reference_schema)?;
            if let Some(ref s3_reference_data_source) = self.s3_reference_data_source {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3ReferenceDataSource", s3_reference_data_source)?;
            }
            if let Some(ref table_name) = self.table_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableName", table_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ReferenceDataSource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ReferenceDataSource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ReferenceDataSource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ReferenceDataSource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut reference_schema: Option<::Value<ReferenceSchema>> = None;
                    let mut s3_reference_data_source: Option<::Value<S3ReferenceDataSource>> = None;
                    let mut table_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ReferenceSchema" => {
                                reference_schema = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3ReferenceDataSource" => {
                                s3_reference_data_source = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TableName" => {
                                table_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ReferenceDataSource {
                        reference_schema: reference_schema.ok_or(::serde::de::Error::missing_field("ReferenceSchema"))?,
                        s3_reference_data_source: s3_reference_data_source,
                        table_name: table_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::ApplicationReferenceDataSource.ReferenceSchema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationreferencedatasource-referenceschema.html) property type.
    #[derive(Debug, Default)]
    pub struct ReferenceSchema {
        /// Property [`RecordColumns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationreferencedatasource-referenceschema.html#cfn-kinesisanalyticsv2-applicationreferencedatasource-referenceschema-recordcolumns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub record_columns: ::ValueList<RecordColumn>,
        /// Property [`RecordEncoding`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationreferencedatasource-referenceschema.html#cfn-kinesisanalyticsv2-applicationreferencedatasource-referenceschema-recordencoding).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub record_encoding: Option<::Value<String>>,
        /// Property [`RecordFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationreferencedatasource-referenceschema.html#cfn-kinesisanalyticsv2-applicationreferencedatasource-referenceschema-recordformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub record_format: ::Value<RecordFormat>,
    }

    impl ::codec::SerializeValue for ReferenceSchema {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecordColumns", &self.record_columns)?;
            if let Some(ref record_encoding) = self.record_encoding {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecordEncoding", record_encoding)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecordFormat", &self.record_format)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ReferenceSchema {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ReferenceSchema, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ReferenceSchema;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ReferenceSchema")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut record_columns: Option<::ValueList<RecordColumn>> = None;
                    let mut record_encoding: Option<::Value<String>> = None;
                    let mut record_format: Option<::Value<RecordFormat>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RecordColumns" => {
                                record_columns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RecordEncoding" => {
                                record_encoding = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RecordFormat" => {
                                record_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ReferenceSchema {
                        record_columns: record_columns.ok_or(::serde::de::Error::missing_field("RecordColumns"))?,
                        record_encoding: record_encoding,
                        record_format: record_format.ok_or(::serde::de::Error::missing_field("RecordFormat"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalyticsV2::ApplicationReferenceDataSource.S3ReferenceDataSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationreferencedatasource-s3referencedatasource.html) property type.
    #[derive(Debug, Default)]
    pub struct S3ReferenceDataSource {
        /// Property [`BucketARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationreferencedatasource-s3referencedatasource.html#cfn-kinesisanalyticsv2-applicationreferencedatasource-s3referencedatasource-bucketarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_arn: ::Value<String>,
        /// Property [`FileKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationreferencedatasource-s3referencedatasource.html#cfn-kinesisanalyticsv2-applicationreferencedatasource-s3referencedatasource-filekey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub file_key: ::Value<String>,
    }

    impl ::codec::SerializeValue for S3ReferenceDataSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketARN", &self.bucket_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FileKey", &self.file_key)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3ReferenceDataSource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3ReferenceDataSource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3ReferenceDataSource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3ReferenceDataSource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket_arn: Option<::Value<String>> = None;
                    let mut file_key: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketARN" => {
                                bucket_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FileKey" => {
                                file_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3ReferenceDataSource {
                        bucket_arn: bucket_arn.ok_or(::serde::de::Error::missing_field("BucketARN"))?,
                        file_key: file_key.ok_or(::serde::de::Error::missing_field("FileKey"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
