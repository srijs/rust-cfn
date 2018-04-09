//! Types for the `KinesisAnalytics` service.

/// The [`AWS::KinesisAnalytics::Application`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisanalytics-application.html) resource type.
#[derive(Debug)]
pub struct Application {
    properties: ApplicationProperties
}

/// Properties for the `Application` resource.
#[derive(Debug)]
pub struct ApplicationProperties {
    /// Property `ApplicationCode`.
    pub application_code: Option<::Value<String>>,
    /// Property `ApplicationDescription`.
    pub application_description: Option<::Value<String>>,
    /// Property `ApplicationName`.
    pub application_name: Option<::Value<String>>,
    /// Property `Inputs`.
    pub inputs: ::ValueList<self::application::Input>,
}

impl ::serde::Serialize for ApplicationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationCode", &self.application_code)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationDescription", &self.application_description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationName", &self.application_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Inputs", &self.inputs)?;
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
                let mut application_code = None;
                let mut application_description = None;
                let mut application_name = None;
                let mut inputs = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApplicationCode" => {
                            application_code = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ApplicationDescription" => {
                            application_description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ApplicationName" => {
                            application_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Inputs" => {
                            inputs = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(ApplicationProperties {
                    application_code: application_code,
                    application_description: application_description,
                    application_name: application_name,
                    inputs: inputs.ok_or(::serde::de::Error::missing_field("Inputs"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for Application {
    type Properties = ApplicationProperties;
    const TYPE: &'static str = "AWS::KinesisAnalytics::Application";
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

/// The [`AWS::KinesisAnalytics::ApplicationOutput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisanalytics-applicationoutput.html) resource type.
#[derive(Debug)]
pub struct ApplicationOutput {
    properties: ApplicationOutputProperties
}

/// Properties for the `ApplicationOutput` resource.
#[derive(Debug)]
pub struct ApplicationOutputProperties {
    /// Property `ApplicationName`.
    pub application_name: ::Value<String>,
    /// Property `Output`.
    pub output: ::Value<self::application_output::Output>,
}

impl ::serde::Serialize for ApplicationOutputProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
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
                let mut application_name = None;
                let mut output = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApplicationName" => {
                            application_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Output" => {
                            output = Some(::serde::de::MapAccess::next_value(&mut map)?);
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

impl<'a> ::Resource<'a> for ApplicationOutput {
    type Properties = ApplicationOutputProperties;
    const TYPE: &'static str = "AWS::KinesisAnalytics::ApplicationOutput";
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

/// The [`AWS::KinesisAnalytics::ApplicationReferenceDataSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisanalytics-applicationreferencedatasource.html) resource type.
#[derive(Debug)]
pub struct ApplicationReferenceDataSource {
    properties: ApplicationReferenceDataSourceProperties
}

/// Properties for the `ApplicationReferenceDataSource` resource.
#[derive(Debug)]
pub struct ApplicationReferenceDataSourceProperties {
    /// Property `ApplicationName`.
    pub application_name: ::Value<String>,
    /// Property `ReferenceDataSource`.
    pub reference_data_source: ::Value<self::application_reference_data_source::ReferenceDataSource>,
}

impl ::serde::Serialize for ApplicationReferenceDataSourceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
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
                let mut application_name = None;
                let mut reference_data_source = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApplicationName" => {
                            application_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ReferenceDataSource" => {
                            reference_data_source = Some(::serde::de::MapAccess::next_value(&mut map)?);
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

impl<'a> ::Resource<'a> for ApplicationReferenceDataSource {
    type Properties = ApplicationReferenceDataSourceProperties;
    const TYPE: &'static str = "AWS::KinesisAnalytics::ApplicationReferenceDataSource";
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

    /// The [`AWS::KinesisAnalytics::Application.CSVMappingParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-application-csvmappingparameters.html) property type.
    #[derive(Debug)]
    pub struct CSVMappingParameters {
        /// Property `RecordColumnDelimiter`.
        pub record_column_delimiter: ::Value<String>,
        /// Property `RecordRowDelimiter`.
        pub record_row_delimiter: ::Value<String>,
    }

    impl ::codec::SerializeValue for CSVMappingParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
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
                    let mut record_column_delimiter = None;
                    let mut record_row_delimiter = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RecordColumnDelimiter" => {
                                record_column_delimiter = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RecordRowDelimiter" => {
                                record_row_delimiter = Some(::serde::de::MapAccess::next_value(&mut map)?);
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

    /// The [`AWS::KinesisAnalytics::Application.Input`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-application-input.html) property type.
    #[derive(Debug)]
    pub struct Input {
        /// Property `InputParallelism`.
        pub input_parallelism: Option<::Value<InputParallelism>>,
        /// Property `InputProcessingConfiguration`.
        pub input_processing_configuration: Option<::Value<InputProcessingConfiguration>>,
        /// Property `InputSchema`.
        pub input_schema: ::Value<InputSchema>,
        /// Property `KinesisFirehoseInput`.
        pub kinesis_firehose_input: Option<::Value<KinesisFirehoseInput>>,
        /// Property `KinesisStreamsInput`.
        pub kinesis_streams_input: Option<::Value<KinesisStreamsInput>>,
        /// Property `NamePrefix`.
        pub name_prefix: ::Value<String>,
    }

    impl ::codec::SerializeValue for Input {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputParallelism", &self.input_parallelism)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputProcessingConfiguration", &self.input_processing_configuration)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputSchema", &self.input_schema)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KinesisFirehoseInput", &self.kinesis_firehose_input)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KinesisStreamsInput", &self.kinesis_streams_input)?;
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
                    let mut input_parallelism = None;
                    let mut input_processing_configuration = None;
                    let mut input_schema = None;
                    let mut kinesis_firehose_input = None;
                    let mut kinesis_streams_input = None;
                    let mut name_prefix = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InputParallelism" => {
                                input_parallelism = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "InputProcessingConfiguration" => {
                                input_processing_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "InputSchema" => {
                                input_schema = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "KinesisFirehoseInput" => {
                                kinesis_firehose_input = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "KinesisStreamsInput" => {
                                kinesis_streams_input = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "NamePrefix" => {
                                name_prefix = Some(::serde::de::MapAccess::next_value(&mut map)?);
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

    /// The [`AWS::KinesisAnalytics::Application.InputLambdaProcessor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-application-inputlambdaprocessor.html) property type.
    #[derive(Debug)]
    pub struct InputLambdaProcessor {
        /// Property `ResourceARN`.
        pub resource_arn: ::Value<String>,
        /// Property `RoleARN`.
        pub role_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for InputLambdaProcessor {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceARN", &self.resource_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleARN", &self.role_arn)?;
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
                    let mut resource_arn = None;
                    let mut role_arn = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ResourceARN" => {
                                resource_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RoleARN" => {
                                role_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(InputLambdaProcessor {
                        resource_arn: resource_arn.ok_or(::serde::de::Error::missing_field("ResourceARN"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleARN"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalytics::Application.InputParallelism`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-application-inputparallelism.html) property type.
    #[derive(Debug)]
    pub struct InputParallelism {
        /// Property `Count`.
        pub count: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for InputParallelism {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Count", &self.count)?;
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
                    let mut count = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Count" => {
                                count = Some(::serde::de::MapAccess::next_value(&mut map)?);
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

    /// The [`AWS::KinesisAnalytics::Application.InputProcessingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-application-inputprocessingconfiguration.html) property type.
    #[derive(Debug)]
    pub struct InputProcessingConfiguration {
        /// Property `InputLambdaProcessor`.
        pub input_lambda_processor: Option<::Value<InputLambdaProcessor>>,
    }

    impl ::codec::SerializeValue for InputProcessingConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputLambdaProcessor", &self.input_lambda_processor)?;
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
                    let mut input_lambda_processor = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InputLambdaProcessor" => {
                                input_lambda_processor = Some(::serde::de::MapAccess::next_value(&mut map)?);
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

    /// The [`AWS::KinesisAnalytics::Application.InputSchema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-application-inputschema.html) property type.
    #[derive(Debug)]
    pub struct InputSchema {
        /// Property `RecordColumns`.
        pub record_columns: ::ValueList<RecordColumn>,
        /// Property `RecordEncoding`.
        pub record_encoding: Option<::Value<String>>,
        /// Property `RecordFormat`.
        pub record_format: ::Value<RecordFormat>,
    }

    impl ::codec::SerializeValue for InputSchema {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecordColumns", &self.record_columns)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecordEncoding", &self.record_encoding)?;
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
                    let mut record_columns = None;
                    let mut record_encoding = None;
                    let mut record_format = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RecordColumns" => {
                                record_columns = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RecordEncoding" => {
                                record_encoding = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RecordFormat" => {
                                record_format = Some(::serde::de::MapAccess::next_value(&mut map)?);
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

    /// The [`AWS::KinesisAnalytics::Application.JSONMappingParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-application-jsonmappingparameters.html) property type.
    #[derive(Debug)]
    pub struct JSONMappingParameters {
        /// Property `RecordRowPath`.
        pub record_row_path: ::Value<String>,
    }

    impl ::codec::SerializeValue for JSONMappingParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
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
                    let mut record_row_path = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RecordRowPath" => {
                                record_row_path = Some(::serde::de::MapAccess::next_value(&mut map)?);
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

    /// The [`AWS::KinesisAnalytics::Application.KinesisFirehoseInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-application-kinesisfirehoseinput.html) property type.
    #[derive(Debug)]
    pub struct KinesisFirehoseInput {
        /// Property `ResourceARN`.
        pub resource_arn: ::Value<String>,
        /// Property `RoleARN`.
        pub role_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for KinesisFirehoseInput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceARN", &self.resource_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleARN", &self.role_arn)?;
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
                    let mut resource_arn = None;
                    let mut role_arn = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ResourceARN" => {
                                resource_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RoleARN" => {
                                role_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(KinesisFirehoseInput {
                        resource_arn: resource_arn.ok_or(::serde::de::Error::missing_field("ResourceARN"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleARN"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalytics::Application.KinesisStreamsInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-application-kinesisstreamsinput.html) property type.
    #[derive(Debug)]
    pub struct KinesisStreamsInput {
        /// Property `ResourceARN`.
        pub resource_arn: ::Value<String>,
        /// Property `RoleARN`.
        pub role_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for KinesisStreamsInput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceARN", &self.resource_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleARN", &self.role_arn)?;
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
                    let mut resource_arn = None;
                    let mut role_arn = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ResourceARN" => {
                                resource_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RoleARN" => {
                                role_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(KinesisStreamsInput {
                        resource_arn: resource_arn.ok_or(::serde::de::Error::missing_field("ResourceARN"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleARN"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalytics::Application.MappingParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-application-mappingparameters.html) property type.
    #[derive(Debug)]
    pub struct MappingParameters {
        /// Property `CSVMappingParameters`.
        pub csv_mapping_parameters: Option<::Value<CSVMappingParameters>>,
        /// Property `JSONMappingParameters`.
        pub json_mapping_parameters: Option<::Value<JSONMappingParameters>>,
    }

    impl ::codec::SerializeValue for MappingParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CSVMappingParameters", &self.csv_mapping_parameters)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "JSONMappingParameters", &self.json_mapping_parameters)?;
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
                    let mut csv_mapping_parameters = None;
                    let mut json_mapping_parameters = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CSVMappingParameters" => {
                                csv_mapping_parameters = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "JSONMappingParameters" => {
                                json_mapping_parameters = Some(::serde::de::MapAccess::next_value(&mut map)?);
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

    /// The [`AWS::KinesisAnalytics::Application.RecordColumn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-application-recordcolumn.html) property type.
    #[derive(Debug)]
    pub struct RecordColumn {
        /// Property `Mapping`.
        pub mapping: Option<::Value<String>>,
        /// Property `Name`.
        pub name: ::Value<String>,
        /// Property `SqlType`.
        pub sql_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for RecordColumn {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Mapping", &self.mapping)?;
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
                    let mut mapping = None;
                    let mut name = None;
                    let mut sql_type = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Mapping" => {
                                mapping = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Name" => {
                                name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SqlType" => {
                                sql_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
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

    /// The [`AWS::KinesisAnalytics::Application.RecordFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-application-recordformat.html) property type.
    #[derive(Debug)]
    pub struct RecordFormat {
        /// Property `MappingParameters`.
        pub mapping_parameters: Option<::Value<MappingParameters>>,
        /// Property `RecordFormatType`.
        pub record_format_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for RecordFormat {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MappingParameters", &self.mapping_parameters)?;
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
                    let mut mapping_parameters = None;
                    let mut record_format_type = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MappingParameters" => {
                                mapping_parameters = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RecordFormatType" => {
                                record_format_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
}

pub mod application_output {
    //! Property types for the `ApplicationOutput` resource.

    /// The [`AWS::KinesisAnalytics::ApplicationOutput.DestinationSchema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-applicationoutput-destinationschema.html) property type.
    #[derive(Debug)]
    pub struct DestinationSchema {
        /// Property `RecordFormatType`.
        pub record_format_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DestinationSchema {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecordFormatType", &self.record_format_type)?;
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
                    let mut record_format_type = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RecordFormatType" => {
                                record_format_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
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

    /// The [`AWS::KinesisAnalytics::ApplicationOutput.KinesisFirehoseOutput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-applicationoutput-kinesisfirehoseoutput.html) property type.
    #[derive(Debug)]
    pub struct KinesisFirehoseOutput {
        /// Property `ResourceARN`.
        pub resource_arn: ::Value<String>,
        /// Property `RoleARN`.
        pub role_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for KinesisFirehoseOutput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceARN", &self.resource_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleARN", &self.role_arn)?;
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
                    let mut resource_arn = None;
                    let mut role_arn = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ResourceARN" => {
                                resource_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RoleARN" => {
                                role_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(KinesisFirehoseOutput {
                        resource_arn: resource_arn.ok_or(::serde::de::Error::missing_field("ResourceARN"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleARN"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalytics::ApplicationOutput.KinesisStreamsOutput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-applicationoutput-kinesisstreamsoutput.html) property type.
    #[derive(Debug)]
    pub struct KinesisStreamsOutput {
        /// Property `ResourceARN`.
        pub resource_arn: ::Value<String>,
        /// Property `RoleARN`.
        pub role_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for KinesisStreamsOutput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceARN", &self.resource_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleARN", &self.role_arn)?;
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
                    let mut resource_arn = None;
                    let mut role_arn = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ResourceARN" => {
                                resource_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RoleARN" => {
                                role_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(KinesisStreamsOutput {
                        resource_arn: resource_arn.ok_or(::serde::de::Error::missing_field("ResourceARN"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleARN"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalytics::ApplicationOutput.LambdaOutput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-applicationoutput-lambdaoutput.html) property type.
    #[derive(Debug)]
    pub struct LambdaOutput {
        /// Property `ResourceARN`.
        pub resource_arn: ::Value<String>,
        /// Property `RoleARN`.
        pub role_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for LambdaOutput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceARN", &self.resource_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleARN", &self.role_arn)?;
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
                    let mut resource_arn = None;
                    let mut role_arn = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ResourceARN" => {
                                resource_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RoleARN" => {
                                role_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(LambdaOutput {
                        resource_arn: resource_arn.ok_or(::serde::de::Error::missing_field("ResourceARN"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleARN"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::KinesisAnalytics::ApplicationOutput.Output`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-applicationoutput-output.html) property type.
    #[derive(Debug)]
    pub struct Output {
        /// Property `DestinationSchema`.
        pub destination_schema: ::Value<DestinationSchema>,
        /// Property `KinesisFirehoseOutput`.
        pub kinesis_firehose_output: Option<::Value<KinesisFirehoseOutput>>,
        /// Property `KinesisStreamsOutput`.
        pub kinesis_streams_output: Option<::Value<KinesisStreamsOutput>>,
        /// Property `LambdaOutput`.
        pub lambda_output: Option<::Value<LambdaOutput>>,
        /// Property `Name`.
        pub name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Output {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationSchema", &self.destination_schema)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KinesisFirehoseOutput", &self.kinesis_firehose_output)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KinesisStreamsOutput", &self.kinesis_streams_output)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaOutput", &self.lambda_output)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
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
                    let mut destination_schema = None;
                    let mut kinesis_firehose_output = None;
                    let mut kinesis_streams_output = None;
                    let mut lambda_output = None;
                    let mut name = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DestinationSchema" => {
                                destination_schema = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "KinesisFirehoseOutput" => {
                                kinesis_firehose_output = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "KinesisStreamsOutput" => {
                                kinesis_streams_output = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "LambdaOutput" => {
                                lambda_output = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Name" => {
                                name = Some(::serde::de::MapAccess::next_value(&mut map)?);
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

    /// The [`AWS::KinesisAnalytics::ApplicationReferenceDataSource.CSVMappingParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-applicationreferencedatasource-csvmappingparameters.html) property type.
    #[derive(Debug)]
    pub struct CSVMappingParameters {
        /// Property `RecordColumnDelimiter`.
        pub record_column_delimiter: ::Value<String>,
        /// Property `RecordRowDelimiter`.
        pub record_row_delimiter: ::Value<String>,
    }

    impl ::codec::SerializeValue for CSVMappingParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
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
                    let mut record_column_delimiter = None;
                    let mut record_row_delimiter = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RecordColumnDelimiter" => {
                                record_column_delimiter = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RecordRowDelimiter" => {
                                record_row_delimiter = Some(::serde::de::MapAccess::next_value(&mut map)?);
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

    /// The [`AWS::KinesisAnalytics::ApplicationReferenceDataSource.JSONMappingParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-applicationreferencedatasource-jsonmappingparameters.html) property type.
    #[derive(Debug)]
    pub struct JSONMappingParameters {
        /// Property `RecordRowPath`.
        pub record_row_path: ::Value<String>,
    }

    impl ::codec::SerializeValue for JSONMappingParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
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
                    let mut record_row_path = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RecordRowPath" => {
                                record_row_path = Some(::serde::de::MapAccess::next_value(&mut map)?);
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

    /// The [`AWS::KinesisAnalytics::ApplicationReferenceDataSource.MappingParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-applicationreferencedatasource-mappingparameters.html) property type.
    #[derive(Debug)]
    pub struct MappingParameters {
        /// Property `CSVMappingParameters`.
        pub csv_mapping_parameters: Option<::Value<CSVMappingParameters>>,
        /// Property `JSONMappingParameters`.
        pub json_mapping_parameters: Option<::Value<JSONMappingParameters>>,
    }

    impl ::codec::SerializeValue for MappingParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CSVMappingParameters", &self.csv_mapping_parameters)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "JSONMappingParameters", &self.json_mapping_parameters)?;
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
                    let mut csv_mapping_parameters = None;
                    let mut json_mapping_parameters = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CSVMappingParameters" => {
                                csv_mapping_parameters = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "JSONMappingParameters" => {
                                json_mapping_parameters = Some(::serde::de::MapAccess::next_value(&mut map)?);
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

    /// The [`AWS::KinesisAnalytics::ApplicationReferenceDataSource.RecordColumn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-applicationreferencedatasource-recordcolumn.html) property type.
    #[derive(Debug)]
    pub struct RecordColumn {
        /// Property `Mapping`.
        pub mapping: Option<::Value<String>>,
        /// Property `Name`.
        pub name: ::Value<String>,
        /// Property `SqlType`.
        pub sql_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for RecordColumn {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Mapping", &self.mapping)?;
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
                    let mut mapping = None;
                    let mut name = None;
                    let mut sql_type = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Mapping" => {
                                mapping = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Name" => {
                                name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SqlType" => {
                                sql_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
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

    /// The [`AWS::KinesisAnalytics::ApplicationReferenceDataSource.RecordFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-applicationreferencedatasource-recordformat.html) property type.
    #[derive(Debug)]
    pub struct RecordFormat {
        /// Property `MappingParameters`.
        pub mapping_parameters: Option<::Value<MappingParameters>>,
        /// Property `RecordFormatType`.
        pub record_format_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for RecordFormat {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MappingParameters", &self.mapping_parameters)?;
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
                    let mut mapping_parameters = None;
                    let mut record_format_type = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MappingParameters" => {
                                mapping_parameters = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RecordFormatType" => {
                                record_format_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
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

    /// The [`AWS::KinesisAnalytics::ApplicationReferenceDataSource.ReferenceDataSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-applicationreferencedatasource-referencedatasource.html) property type.
    #[derive(Debug)]
    pub struct ReferenceDataSource {
        /// Property `ReferenceSchema`.
        pub reference_schema: ::Value<ReferenceSchema>,
        /// Property `S3ReferenceDataSource`.
        pub s3_reference_data_source: Option<::Value<S3ReferenceDataSource>>,
        /// Property `TableName`.
        pub table_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ReferenceDataSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReferenceSchema", &self.reference_schema)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3ReferenceDataSource", &self.s3_reference_data_source)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableName", &self.table_name)?;
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
                    let mut reference_schema = None;
                    let mut s3_reference_data_source = None;
                    let mut table_name = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ReferenceSchema" => {
                                reference_schema = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "S3ReferenceDataSource" => {
                                s3_reference_data_source = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "TableName" => {
                                table_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
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

    /// The [`AWS::KinesisAnalytics::ApplicationReferenceDataSource.ReferenceSchema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-applicationreferencedatasource-referenceschema.html) property type.
    #[derive(Debug)]
    pub struct ReferenceSchema {
        /// Property `RecordColumns`.
        pub record_columns: ::ValueList<RecordColumn>,
        /// Property `RecordEncoding`.
        pub record_encoding: Option<::Value<String>>,
        /// Property `RecordFormat`.
        pub record_format: ::Value<RecordFormat>,
    }

    impl ::codec::SerializeValue for ReferenceSchema {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecordColumns", &self.record_columns)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecordEncoding", &self.record_encoding)?;
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
                    let mut record_columns = None;
                    let mut record_encoding = None;
                    let mut record_format = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RecordColumns" => {
                                record_columns = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RecordEncoding" => {
                                record_encoding = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RecordFormat" => {
                                record_format = Some(::serde::de::MapAccess::next_value(&mut map)?);
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

    /// The [`AWS::KinesisAnalytics::ApplicationReferenceDataSource.S3ReferenceDataSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-applicationreferencedatasource-s3referencedatasource.html) property type.
    #[derive(Debug)]
    pub struct S3ReferenceDataSource {
        /// Property `BucketARN`.
        pub bucket_arn: ::Value<String>,
        /// Property `FileKey`.
        pub file_key: ::Value<String>,
        /// Property `ReferenceRoleARN`.
        pub reference_role_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for S3ReferenceDataSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            #[allow(unused_mut)]
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketARN", &self.bucket_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FileKey", &self.file_key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReferenceRoleARN", &self.reference_role_arn)?;
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
                    let mut bucket_arn = None;
                    let mut file_key = None;
                    let mut reference_role_arn = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketARN" => {
                                bucket_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "FileKey" => {
                                file_key = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ReferenceRoleARN" => {
                                reference_role_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(S3ReferenceDataSource {
                        bucket_arn: bucket_arn.ok_or(::serde::de::Error::missing_field("BucketARN"))?,
                        file_key: file_key.ok_or(::serde::de::Error::missing_field("FileKey"))?,
                        reference_role_arn: reference_role_arn.ok_or(::serde::de::Error::missing_field("ReferenceRoleARN"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
