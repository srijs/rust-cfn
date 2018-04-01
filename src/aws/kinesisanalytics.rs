//! Types for the `KinesisAnalytics` service.

/// The [`AWS::KinesisAnalytics::Application`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisanalytics-application.html) resource type.
#[derive(Debug)]
pub struct Application {
    properties: ApplicationProperties
}

/// Properties for the `Application` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationProperties {
    /// Property `ApplicationCode`.
    #[serde(rename = "ApplicationCode")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_code: Option<::Value<String>>,
    /// Property `ApplicationDescription`.
    #[serde(rename = "ApplicationDescription")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_description: Option<::Value<String>>,
    /// Property `ApplicationName`.
    #[serde(rename = "ApplicationName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_name: Option<::Value<String>>,
    /// Property `Inputs`.
    #[serde(rename = "Inputs")]
    pub inputs: ::ValueList<self::application::Input>,
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
#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationOutputProperties {
    /// Property `ApplicationName`.
    #[serde(rename = "ApplicationName")]
    pub application_name: ::Value<String>,
    /// Property `Output`.
    #[serde(rename = "Output")]
    pub output: ::Value<self::application_output::Output>,
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
#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationReferenceDataSourceProperties {
    /// Property `ApplicationName`.
    #[serde(rename = "ApplicationName")]
    pub application_name: ::Value<String>,
    /// Property `ReferenceDataSource`.
    #[serde(rename = "ReferenceDataSource")]
    pub reference_data_source: ::Value<self::application_reference_data_source::ReferenceDataSource>,
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
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CSVMappingParameters {
        /// Property `RecordColumnDelimiter`.
        #[serde(rename = "RecordColumnDelimiter")]
        pub record_column_delimiter: ::Value<String>,
        /// Property `RecordRowDelimiter`.
        #[serde(rename = "RecordRowDelimiter")]
        pub record_row_delimiter: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(CSVMappingParameters);

    /// The [`AWS::KinesisAnalytics::Application.Input`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-application-input.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Input {
        /// Property `InputParallelism`.
        #[serde(rename = "InputParallelism")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub input_parallelism: Option<::Value<InputParallelism>>,
        /// Property `InputProcessingConfiguration`.
        #[serde(rename = "InputProcessingConfiguration")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub input_processing_configuration: Option<::Value<InputProcessingConfiguration>>,
        /// Property `InputSchema`.
        #[serde(rename = "InputSchema")]
        pub input_schema: ::Value<InputSchema>,
        /// Property `KinesisFirehoseInput`.
        #[serde(rename = "KinesisFirehoseInput")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub kinesis_firehose_input: Option<::Value<KinesisFirehoseInput>>,
        /// Property `KinesisStreamsInput`.
        #[serde(rename = "KinesisStreamsInput")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub kinesis_streams_input: Option<::Value<KinesisStreamsInput>>,
        /// Property `NamePrefix`.
        #[serde(rename = "NamePrefix")]
        pub name_prefix: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(Input);

    /// The [`AWS::KinesisAnalytics::Application.InputLambdaProcessor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-application-inputlambdaprocessor.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct InputLambdaProcessor {
        /// Property `ResourceARN`.
        #[serde(rename = "ResourceARN")]
        pub resource_arn: ::Value<String>,
        /// Property `RoleARN`.
        #[serde(rename = "RoleARN")]
        pub role_arn: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(InputLambdaProcessor);

    /// The [`AWS::KinesisAnalytics::Application.InputParallelism`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-application-inputparallelism.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct InputParallelism {
        /// Property `Count`.
        #[serde(rename = "Count")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub count: Option<::Value<u32>>,
    }

    cfn_internal__inherit_codec_impls!(InputParallelism);

    /// The [`AWS::KinesisAnalytics::Application.InputProcessingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-application-inputprocessingconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct InputProcessingConfiguration {
        /// Property `InputLambdaProcessor`.
        #[serde(rename = "InputLambdaProcessor")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub input_lambda_processor: Option<::Value<InputLambdaProcessor>>,
    }

    cfn_internal__inherit_codec_impls!(InputProcessingConfiguration);

    /// The [`AWS::KinesisAnalytics::Application.InputSchema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-application-inputschema.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct InputSchema {
        /// Property `RecordColumns`.
        #[serde(rename = "RecordColumns")]
        pub record_columns: ::ValueList<RecordColumn>,
        /// Property `RecordEncoding`.
        #[serde(rename = "RecordEncoding")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub record_encoding: Option<::Value<String>>,
        /// Property `RecordFormat`.
        #[serde(rename = "RecordFormat")]
        pub record_format: ::Value<RecordFormat>,
    }

    cfn_internal__inherit_codec_impls!(InputSchema);

    /// The [`AWS::KinesisAnalytics::Application.JSONMappingParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-application-jsonmappingparameters.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct JSONMappingParameters {
        /// Property `RecordRowPath`.
        #[serde(rename = "RecordRowPath")]
        pub record_row_path: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(JSONMappingParameters);

    /// The [`AWS::KinesisAnalytics::Application.KinesisFirehoseInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-application-kinesisfirehoseinput.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct KinesisFirehoseInput {
        /// Property `ResourceARN`.
        #[serde(rename = "ResourceARN")]
        pub resource_arn: ::Value<String>,
        /// Property `RoleARN`.
        #[serde(rename = "RoleARN")]
        pub role_arn: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(KinesisFirehoseInput);

    /// The [`AWS::KinesisAnalytics::Application.KinesisStreamsInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-application-kinesisstreamsinput.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct KinesisStreamsInput {
        /// Property `ResourceARN`.
        #[serde(rename = "ResourceARN")]
        pub resource_arn: ::Value<String>,
        /// Property `RoleARN`.
        #[serde(rename = "RoleARN")]
        pub role_arn: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(KinesisStreamsInput);

    /// The [`AWS::KinesisAnalytics::Application.MappingParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-application-mappingparameters.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MappingParameters {
        /// Property `CSVMappingParameters`.
        #[serde(rename = "CSVMappingParameters")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub csv_mapping_parameters: Option<::Value<CSVMappingParameters>>,
        /// Property `JSONMappingParameters`.
        #[serde(rename = "JSONMappingParameters")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub json_mapping_parameters: Option<::Value<JSONMappingParameters>>,
    }

    cfn_internal__inherit_codec_impls!(MappingParameters);

    /// The [`AWS::KinesisAnalytics::Application.RecordColumn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-application-recordcolumn.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RecordColumn {
        /// Property `Mapping`.
        #[serde(rename = "Mapping")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub mapping: Option<::Value<String>>,
        /// Property `Name`.
        #[serde(rename = "Name")]
        pub name: ::Value<String>,
        /// Property `SqlType`.
        #[serde(rename = "SqlType")]
        pub sql_type: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(RecordColumn);

    /// The [`AWS::KinesisAnalytics::Application.RecordFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-application-recordformat.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RecordFormat {
        /// Property `MappingParameters`.
        #[serde(rename = "MappingParameters")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub mapping_parameters: Option<::Value<MappingParameters>>,
        /// Property `RecordFormatType`.
        #[serde(rename = "RecordFormatType")]
        pub record_format_type: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(RecordFormat);
}

pub mod application_output {
    //! Property types for the `ApplicationOutput` resource.

    /// The [`AWS::KinesisAnalytics::ApplicationOutput.DestinationSchema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-applicationoutput-destinationschema.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DestinationSchema {
        /// Property `RecordFormatType`.
        #[serde(rename = "RecordFormatType")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub record_format_type: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(DestinationSchema);

    /// The [`AWS::KinesisAnalytics::ApplicationOutput.KinesisFirehoseOutput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-applicationoutput-kinesisfirehoseoutput.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct KinesisFirehoseOutput {
        /// Property `ResourceARN`.
        #[serde(rename = "ResourceARN")]
        pub resource_arn: ::Value<String>,
        /// Property `RoleARN`.
        #[serde(rename = "RoleARN")]
        pub role_arn: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(KinesisFirehoseOutput);

    /// The [`AWS::KinesisAnalytics::ApplicationOutput.KinesisStreamsOutput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-applicationoutput-kinesisstreamsoutput.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct KinesisStreamsOutput {
        /// Property `ResourceARN`.
        #[serde(rename = "ResourceARN")]
        pub resource_arn: ::Value<String>,
        /// Property `RoleARN`.
        #[serde(rename = "RoleARN")]
        pub role_arn: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(KinesisStreamsOutput);

    /// The [`AWS::KinesisAnalytics::ApplicationOutput.LambdaOutput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-applicationoutput-lambdaoutput.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LambdaOutput {
        /// Property `ResourceARN`.
        #[serde(rename = "ResourceARN")]
        pub resource_arn: ::Value<String>,
        /// Property `RoleARN`.
        #[serde(rename = "RoleARN")]
        pub role_arn: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(LambdaOutput);

    /// The [`AWS::KinesisAnalytics::ApplicationOutput.Output`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-applicationoutput-output.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        /// Property `DestinationSchema`.
        #[serde(rename = "DestinationSchema")]
        pub destination_schema: ::Value<DestinationSchema>,
        /// Property `KinesisFirehoseOutput`.
        #[serde(rename = "KinesisFirehoseOutput")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub kinesis_firehose_output: Option<::Value<KinesisFirehoseOutput>>,
        /// Property `KinesisStreamsOutput`.
        #[serde(rename = "KinesisStreamsOutput")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub kinesis_streams_output: Option<::Value<KinesisStreamsOutput>>,
        /// Property `LambdaOutput`.
        #[serde(rename = "LambdaOutput")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub lambda_output: Option<::Value<LambdaOutput>>,
        /// Property `Name`.
        #[serde(rename = "Name")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(Output);
}

pub mod application_reference_data_source {
    //! Property types for the `ApplicationReferenceDataSource` resource.

    /// The [`AWS::KinesisAnalytics::ApplicationReferenceDataSource.CSVMappingParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-applicationreferencedatasource-csvmappingparameters.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CSVMappingParameters {
        /// Property `RecordColumnDelimiter`.
        #[serde(rename = "RecordColumnDelimiter")]
        pub record_column_delimiter: ::Value<String>,
        /// Property `RecordRowDelimiter`.
        #[serde(rename = "RecordRowDelimiter")]
        pub record_row_delimiter: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(CSVMappingParameters);

    /// The [`AWS::KinesisAnalytics::ApplicationReferenceDataSource.JSONMappingParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-applicationreferencedatasource-jsonmappingparameters.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct JSONMappingParameters {
        /// Property `RecordRowPath`.
        #[serde(rename = "RecordRowPath")]
        pub record_row_path: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(JSONMappingParameters);

    /// The [`AWS::KinesisAnalytics::ApplicationReferenceDataSource.MappingParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-applicationreferencedatasource-mappingparameters.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MappingParameters {
        /// Property `CSVMappingParameters`.
        #[serde(rename = "CSVMappingParameters")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub csv_mapping_parameters: Option<::Value<CSVMappingParameters>>,
        /// Property `JSONMappingParameters`.
        #[serde(rename = "JSONMappingParameters")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub json_mapping_parameters: Option<::Value<JSONMappingParameters>>,
    }

    cfn_internal__inherit_codec_impls!(MappingParameters);

    /// The [`AWS::KinesisAnalytics::ApplicationReferenceDataSource.RecordColumn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-applicationreferencedatasource-recordcolumn.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RecordColumn {
        /// Property `Mapping`.
        #[serde(rename = "Mapping")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub mapping: Option<::Value<String>>,
        /// Property `Name`.
        #[serde(rename = "Name")]
        pub name: ::Value<String>,
        /// Property `SqlType`.
        #[serde(rename = "SqlType")]
        pub sql_type: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(RecordColumn);

    /// The [`AWS::KinesisAnalytics::ApplicationReferenceDataSource.RecordFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-applicationreferencedatasource-recordformat.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RecordFormat {
        /// Property `MappingParameters`.
        #[serde(rename = "MappingParameters")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub mapping_parameters: Option<::Value<MappingParameters>>,
        /// Property `RecordFormatType`.
        #[serde(rename = "RecordFormatType")]
        pub record_format_type: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(RecordFormat);

    /// The [`AWS::KinesisAnalytics::ApplicationReferenceDataSource.ReferenceDataSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-applicationreferencedatasource-referencedatasource.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ReferenceDataSource {
        /// Property `ReferenceSchema`.
        #[serde(rename = "ReferenceSchema")]
        pub reference_schema: ::Value<ReferenceSchema>,
        /// Property `S3ReferenceDataSource`.
        #[serde(rename = "S3ReferenceDataSource")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub s3_reference_data_source: Option<::Value<S3ReferenceDataSource>>,
        /// Property `TableName`.
        #[serde(rename = "TableName")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub table_name: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(ReferenceDataSource);

    /// The [`AWS::KinesisAnalytics::ApplicationReferenceDataSource.ReferenceSchema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-applicationreferencedatasource-referenceschema.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ReferenceSchema {
        /// Property `RecordColumns`.
        #[serde(rename = "RecordColumns")]
        pub record_columns: ::ValueList<RecordColumn>,
        /// Property `RecordEncoding`.
        #[serde(rename = "RecordEncoding")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub record_encoding: Option<::Value<String>>,
        /// Property `RecordFormat`.
        #[serde(rename = "RecordFormat")]
        pub record_format: ::Value<RecordFormat>,
    }

    cfn_internal__inherit_codec_impls!(ReferenceSchema);

    /// The [`AWS::KinesisAnalytics::ApplicationReferenceDataSource.S3ReferenceDataSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-applicationreferencedatasource-s3referencedatasource.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct S3ReferenceDataSource {
        /// Property `BucketARN`.
        #[serde(rename = "BucketARN")]
        pub bucket_arn: ::Value<String>,
        /// Property `FileKey`.
        #[serde(rename = "FileKey")]
        pub file_key: ::Value<String>,
        /// Property `ReferenceRoleARN`.
        #[serde(rename = "ReferenceRoleARN")]
        pub reference_role_arn: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(S3ReferenceDataSource);
}
