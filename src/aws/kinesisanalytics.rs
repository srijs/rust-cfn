/// The [`AWS::KinesisAnalytics::Application`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisanalytics-application.html) resource.
pub struct Application {
    properties: ApplicationProperties
}

/// Properties for the `Application` resource.
#[derive(Serialize, Deserialize)]
pub struct ApplicationProperties {
    #[serde(rename="ApplicationCode")]
    pub application_code: String,
    #[serde(rename="ApplicationDescription")]
    pub application_description: String,
    #[serde(rename="ApplicationName")]
    pub application_name: String,
    #[serde(rename="Inputs")]
    pub inputs: Vec<self::application::Input>,
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

impl From<ApplicationProperties> for Application {
    fn from(properties: ApplicationProperties) -> Application {
        Application { properties }
    }
}

/// The [`AWS::KinesisAnalytics::ApplicationOutput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisanalytics-applicationoutput.html) resource.
pub struct ApplicationOutput {
    properties: ApplicationOutputProperties
}

/// Properties for the `ApplicationOutput` resource.
#[derive(Serialize, Deserialize)]
pub struct ApplicationOutputProperties {
    #[serde(rename="ApplicationName")]
    pub application_name: String,
    #[serde(rename="Output")]
    pub output: self::application_output::Output,
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

impl From<ApplicationOutputProperties> for ApplicationOutput {
    fn from(properties: ApplicationOutputProperties) -> ApplicationOutput {
        ApplicationOutput { properties }
    }
}

/// The [`AWS::KinesisAnalytics::ApplicationReferenceDataSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisanalytics-applicationreferencedatasource.html) resource.
pub struct ApplicationReferenceDataSource {
    properties: ApplicationReferenceDataSourceProperties
}

/// Properties for the `ApplicationReferenceDataSource` resource.
#[derive(Serialize, Deserialize)]
pub struct ApplicationReferenceDataSourceProperties {
    #[serde(rename="ApplicationName")]
    pub application_name: String,
    #[serde(rename="ReferenceDataSource")]
    pub reference_data_source: self::application_reference_data_source::ReferenceDataSource,
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

impl From<ApplicationReferenceDataSourceProperties> for ApplicationReferenceDataSource {
    fn from(properties: ApplicationReferenceDataSourceProperties) -> ApplicationReferenceDataSource {
        ApplicationReferenceDataSource { properties }
    }
}

pub mod application {
    #[derive(Serialize, Deserialize)]
    pub struct CSVMappingParameters {
        #[serde(rename="RecordColumnDelimiter")]
        pub record_column_delimiter: String,
        #[serde(rename="RecordRowDelimiter")]
        pub record_row_delimiter: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Input {
        #[serde(rename="InputParallelism")]
        pub input_parallelism: InputParallelism,
        #[serde(rename="InputProcessingConfiguration")]
        pub input_processing_configuration: InputProcessingConfiguration,
        #[serde(rename="InputSchema")]
        pub input_schema: InputSchema,
        #[serde(rename="KinesisFirehoseInput")]
        pub kinesis_firehose_input: KinesisFirehoseInput,
        #[serde(rename="KinesisStreamsInput")]
        pub kinesis_streams_input: KinesisStreamsInput,
        #[serde(rename="NamePrefix")]
        pub name_prefix: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct InputLambdaProcessor {
        #[serde(rename="ResourceARN")]
        pub resource_arn: String,
        #[serde(rename="RoleARN")]
        pub role_arn: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct InputParallelism {
        #[serde(rename="Count")]
        pub count: u32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct InputProcessingConfiguration {
        #[serde(rename="InputLambdaProcessor")]
        pub input_lambda_processor: InputLambdaProcessor,
    }

    #[derive(Serialize, Deserialize)]
    pub struct InputSchema {
        #[serde(rename="RecordColumns")]
        pub record_columns: Vec<RecordColumn>,
        #[serde(rename="RecordEncoding")]
        pub record_encoding: String,
        #[serde(rename="RecordFormat")]
        pub record_format: RecordFormat,
    }

    #[derive(Serialize, Deserialize)]
    pub struct JSONMappingParameters {
        #[serde(rename="RecordRowPath")]
        pub record_row_path: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct KinesisFirehoseInput {
        #[serde(rename="ResourceARN")]
        pub resource_arn: String,
        #[serde(rename="RoleARN")]
        pub role_arn: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct KinesisStreamsInput {
        #[serde(rename="ResourceARN")]
        pub resource_arn: String,
        #[serde(rename="RoleARN")]
        pub role_arn: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct MappingParameters {
        #[serde(rename="CSVMappingParameters")]
        pub csv_mapping_parameters: CSVMappingParameters,
        #[serde(rename="JSONMappingParameters")]
        pub json_mapping_parameters: JSONMappingParameters,
    }

    #[derive(Serialize, Deserialize)]
    pub struct RecordColumn {
        #[serde(rename="Mapping")]
        pub mapping: String,
        #[serde(rename="Name")]
        pub name: String,
        #[serde(rename="SqlType")]
        pub sql_type: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct RecordFormat {
        #[serde(rename="MappingParameters")]
        pub mapping_parameters: MappingParameters,
        #[serde(rename="RecordFormatType")]
        pub record_format_type: String,
    }

}

pub mod application_output {
    #[derive(Serialize, Deserialize)]
    pub struct DestinationSchema {
        #[serde(rename="RecordFormatType")]
        pub record_format_type: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct KinesisFirehoseOutput {
        #[serde(rename="ResourceARN")]
        pub resource_arn: String,
        #[serde(rename="RoleARN")]
        pub role_arn: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct KinesisStreamsOutput {
        #[serde(rename="ResourceARN")]
        pub resource_arn: String,
        #[serde(rename="RoleARN")]
        pub role_arn: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct LambdaOutput {
        #[serde(rename="ResourceARN")]
        pub resource_arn: String,
        #[serde(rename="RoleARN")]
        pub role_arn: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Output {
        #[serde(rename="DestinationSchema")]
        pub destination_schema: DestinationSchema,
        #[serde(rename="KinesisFirehoseOutput")]
        pub kinesis_firehose_output: KinesisFirehoseOutput,
        #[serde(rename="KinesisStreamsOutput")]
        pub kinesis_streams_output: KinesisStreamsOutput,
        #[serde(rename="LambdaOutput")]
        pub lambda_output: LambdaOutput,
        #[serde(rename="Name")]
        pub name: String,
    }

}

pub mod application_reference_data_source {
    #[derive(Serialize, Deserialize)]
    pub struct CSVMappingParameters {
        #[serde(rename="RecordColumnDelimiter")]
        pub record_column_delimiter: String,
        #[serde(rename="RecordRowDelimiter")]
        pub record_row_delimiter: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct JSONMappingParameters {
        #[serde(rename="RecordRowPath")]
        pub record_row_path: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct MappingParameters {
        #[serde(rename="CSVMappingParameters")]
        pub csv_mapping_parameters: CSVMappingParameters,
        #[serde(rename="JSONMappingParameters")]
        pub json_mapping_parameters: JSONMappingParameters,
    }

    #[derive(Serialize, Deserialize)]
    pub struct RecordColumn {
        #[serde(rename="Mapping")]
        pub mapping: String,
        #[serde(rename="Name")]
        pub name: String,
        #[serde(rename="SqlType")]
        pub sql_type: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct RecordFormat {
        #[serde(rename="MappingParameters")]
        pub mapping_parameters: MappingParameters,
        #[serde(rename="RecordFormatType")]
        pub record_format_type: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ReferenceDataSource {
        #[serde(rename="ReferenceSchema")]
        pub reference_schema: ReferenceSchema,
        #[serde(rename="S3ReferenceDataSource")]
        pub s3_reference_data_source: S3ReferenceDataSource,
        #[serde(rename="TableName")]
        pub table_name: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ReferenceSchema {
        #[serde(rename="RecordColumns")]
        pub record_columns: Vec<RecordColumn>,
        #[serde(rename="RecordEncoding")]
        pub record_encoding: String,
        #[serde(rename="RecordFormat")]
        pub record_format: RecordFormat,
    }

    #[derive(Serialize, Deserialize)]
    pub struct S3ReferenceDataSource {
        #[serde(rename="BucketARN")]
        pub bucket_arn: String,
        #[serde(rename="FileKey")]
        pub file_key: String,
        #[serde(rename="ReferenceRoleARN")]
        pub reference_role_arn: String,
    }

}

