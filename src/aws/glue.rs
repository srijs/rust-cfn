//! Types for the `Glue` service.

/// The [`AWS::Glue::Classifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-classifier.html) resource type.
#[derive(Debug)]
pub struct Classifier {
    properties: ClassifierProperties
}

/// Properties for the `Classifier` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ClassifierProperties {
    /// Property `GrokClassifier`.
    #[serde(rename="GrokClassifier")]
    pub grok_classifier: self::classifier::GrokClassifier,
}

impl<'a> ::Resource<'a> for Classifier {
    type Properties = ClassifierProperties;
    const TYPE: &'static str = "AWS::Glue::Classifier";
    fn properties(&self) -> &ClassifierProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ClassifierProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Classifier {}

impl From<ClassifierProperties> for Classifier {
    fn from(properties: ClassifierProperties) -> Classifier {
        Classifier { properties }
    }
}

/// The [`AWS::Glue::Connection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-connection.html) resource type.
#[derive(Debug)]
pub struct Connection {
    properties: ConnectionProperties
}

/// Properties for the `Connection` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionProperties {
    /// Property `CatalogId`.
    #[serde(rename="CatalogId")]
    pub catalog_id: String,
    /// Property `ConnectionInput`.
    #[serde(rename="ConnectionInput")]
    pub connection_input: self::connection::ConnectionInput,
}

impl<'a> ::Resource<'a> for Connection {
    type Properties = ConnectionProperties;
    const TYPE: &'static str = "AWS::Glue::Connection";
    fn properties(&self) -> &ConnectionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ConnectionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Connection {}

impl From<ConnectionProperties> for Connection {
    fn from(properties: ConnectionProperties) -> Connection {
        Connection { properties }
    }
}

/// The [`AWS::Glue::Crawler`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-crawler.html) resource type.
#[derive(Debug)]
pub struct Crawler {
    properties: CrawlerProperties
}

/// Properties for the `Crawler` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct CrawlerProperties {
    /// Property `Classifiers`.
    #[serde(rename="Classifiers")]
    pub classifiers: Vec<String>,
    /// Property `DatabaseName`.
    #[serde(rename="DatabaseName")]
    pub database_name: String,
    /// Property `Description`.
    #[serde(rename="Description")]
    pub description: String,
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
    /// Property `Role`.
    #[serde(rename="Role")]
    pub role: String,
    /// Property `Schedule`.
    #[serde(rename="Schedule")]
    pub schedule: self::crawler::Schedule,
    /// Property `SchemaChangePolicy`.
    #[serde(rename="SchemaChangePolicy")]
    pub schema_change_policy: self::crawler::SchemaChangePolicy,
    /// Property `TablePrefix`.
    #[serde(rename="TablePrefix")]
    pub table_prefix: String,
    /// Property `Targets`.
    #[serde(rename="Targets")]
    pub targets: self::crawler::Targets,
}

impl<'a> ::Resource<'a> for Crawler {
    type Properties = CrawlerProperties;
    const TYPE: &'static str = "AWS::Glue::Crawler";
    fn properties(&self) -> &CrawlerProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CrawlerProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Crawler {}

impl From<CrawlerProperties> for Crawler {
    fn from(properties: CrawlerProperties) -> Crawler {
        Crawler { properties }
    }
}

/// The [`AWS::Glue::Database`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-database.html) resource type.
#[derive(Debug)]
pub struct Database {
    properties: DatabaseProperties
}

/// Properties for the `Database` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseProperties {
    /// Property `CatalogId`.
    #[serde(rename="CatalogId")]
    pub catalog_id: String,
    /// Property `DatabaseInput`.
    #[serde(rename="DatabaseInput")]
    pub database_input: self::database::DatabaseInput,
}

impl<'a> ::Resource<'a> for Database {
    type Properties = DatabaseProperties;
    const TYPE: &'static str = "AWS::Glue::Database";
    fn properties(&self) -> &DatabaseProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DatabaseProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Database {}

impl From<DatabaseProperties> for Database {
    fn from(properties: DatabaseProperties) -> Database {
        Database { properties }
    }
}

/// The [`AWS::Glue::DevEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-devendpoint.html) resource type.
#[derive(Debug)]
pub struct DevEndpoint {
    properties: DevEndpointProperties
}

/// Properties for the `DevEndpoint` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct DevEndpointProperties {
    /// Property `EndpointName`.
    #[serde(rename="EndpointName")]
    pub endpoint_name: String,
    /// Property `ExtraJarsS3Path`.
    #[serde(rename="ExtraJarsS3Path")]
    pub extra_jars_s3_path: String,
    /// Property `ExtraPythonLibsS3Path`.
    #[serde(rename="ExtraPythonLibsS3Path")]
    pub extra_python_libs_s3_path: String,
    /// Property `NumberOfNodes`.
    #[serde(rename="NumberOfNodes")]
    pub number_of_nodes: u32,
    /// Property `PublicKey`.
    #[serde(rename="PublicKey")]
    pub public_key: String,
    /// Property `RoleArn`.
    #[serde(rename="RoleArn")]
    pub role_arn: String,
    /// Property `SecurityGroupIds`.
    #[serde(rename="SecurityGroupIds")]
    pub security_group_ids: Vec<String>,
    /// Property `SubnetId`.
    #[serde(rename="SubnetId")]
    pub subnet_id: String,
}

impl<'a> ::Resource<'a> for DevEndpoint {
    type Properties = DevEndpointProperties;
    const TYPE: &'static str = "AWS::Glue::DevEndpoint";
    fn properties(&self) -> &DevEndpointProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DevEndpointProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DevEndpoint {}

impl From<DevEndpointProperties> for DevEndpoint {
    fn from(properties: DevEndpointProperties) -> DevEndpoint {
        DevEndpoint { properties }
    }
}

/// The [`AWS::Glue::Job`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-job.html) resource type.
#[derive(Debug)]
pub struct Job {
    properties: JobProperties
}

/// Properties for the `Job` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct JobProperties {
    /// Property `AllocatedCapacity`.
    #[serde(rename="AllocatedCapacity")]
    pub allocated_capacity: f64,
    /// Property `Command`.
    #[serde(rename="Command")]
    pub command: self::job::JobCommand,
    /// Property `Connections`.
    #[serde(rename="Connections")]
    pub connections: self::job::ConnectionsList,
    /// Property `DefaultArguments`.
    #[serde(rename="DefaultArguments")]
    pub default_arguments: ::json::Value,
    /// Property `Description`.
    #[serde(rename="Description")]
    pub description: String,
    /// Property `ExecutionProperty`.
    #[serde(rename="ExecutionProperty")]
    pub execution_property: self::job::ExecutionProperty,
    /// Property `LogUri`.
    #[serde(rename="LogUri")]
    pub log_uri: String,
    /// Property `MaxRetries`.
    #[serde(rename="MaxRetries")]
    pub max_retries: f64,
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
    /// Property `Role`.
    #[serde(rename="Role")]
    pub role: String,
}

impl<'a> ::Resource<'a> for Job {
    type Properties = JobProperties;
    const TYPE: &'static str = "AWS::Glue::Job";
    fn properties(&self) -> &JobProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut JobProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Job {}

impl From<JobProperties> for Job {
    fn from(properties: JobProperties) -> Job {
        Job { properties }
    }
}

/// The [`AWS::Glue::Partition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-partition.html) resource type.
#[derive(Debug)]
pub struct Partition {
    properties: PartitionProperties
}

/// Properties for the `Partition` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct PartitionProperties {
    /// Property `CatalogId`.
    #[serde(rename="CatalogId")]
    pub catalog_id: String,
    /// Property `DatabaseName`.
    #[serde(rename="DatabaseName")]
    pub database_name: String,
    /// Property `PartitionInput`.
    #[serde(rename="PartitionInput")]
    pub partition_input: self::partition::PartitionInput,
    /// Property `TableName`.
    #[serde(rename="TableName")]
    pub table_name: String,
}

impl<'a> ::Resource<'a> for Partition {
    type Properties = PartitionProperties;
    const TYPE: &'static str = "AWS::Glue::Partition";
    fn properties(&self) -> &PartitionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PartitionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Partition {}

impl From<PartitionProperties> for Partition {
    fn from(properties: PartitionProperties) -> Partition {
        Partition { properties }
    }
}

/// The [`AWS::Glue::Table`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-table.html) resource type.
#[derive(Debug)]
pub struct Table {
    properties: TableProperties
}

/// Properties for the `Table` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct TableProperties {
    /// Property `CatalogId`.
    #[serde(rename="CatalogId")]
    pub catalog_id: String,
    /// Property `DatabaseName`.
    #[serde(rename="DatabaseName")]
    pub database_name: String,
    /// Property `TableInput`.
    #[serde(rename="TableInput")]
    pub table_input: self::table::TableInput,
}

impl<'a> ::Resource<'a> for Table {
    type Properties = TableProperties;
    const TYPE: &'static str = "AWS::Glue::Table";
    fn properties(&self) -> &TableProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TableProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Table {}

impl From<TableProperties> for Table {
    fn from(properties: TableProperties) -> Table {
        Table { properties }
    }
}

/// The [`AWS::Glue::Trigger`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-trigger.html) resource type.
#[derive(Debug)]
pub struct Trigger {
    properties: TriggerProperties
}

/// Properties for the `Trigger` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct TriggerProperties {
    /// Property `Actions`.
    #[serde(rename="Actions")]
    pub actions: Vec<self::trigger::Action>,
    /// Property `Description`.
    #[serde(rename="Description")]
    pub description: String,
    /// Property `Name`.
    #[serde(rename="Name")]
    pub name: String,
    /// Property `Predicate`.
    #[serde(rename="Predicate")]
    pub predicate: self::trigger::Predicate,
    /// Property `Schedule`.
    #[serde(rename="Schedule")]
    pub schedule: String,
    /// Property `Type`.
    #[serde(rename="Type")]
    pub type_: String,
}

impl<'a> ::Resource<'a> for Trigger {
    type Properties = TriggerProperties;
    const TYPE: &'static str = "AWS::Glue::Trigger";
    fn properties(&self) -> &TriggerProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TriggerProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Trigger {}

impl From<TriggerProperties> for Trigger {
    fn from(properties: TriggerProperties) -> Trigger {
        Trigger { properties }
    }
}

pub mod classifier {
    //! Property types for the `Classifier` resource.

    /// The [`AWS::Glue::Classifier.GrokClassifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-classifier-grokclassifier.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GrokClassifier {
        /// Property `Classification`.
        #[serde(rename="Classification")]
        pub classification: String,
        /// Property `CustomPatterns`.
        #[serde(rename="CustomPatterns")]
        pub custom_patterns: String,
        /// Property `GrokPattern`.
        #[serde(rename="GrokPattern")]
        pub grok_pattern: String,
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
    }
}

pub mod connection {
    //! Property types for the `Connection` resource.

    /// The [`AWS::Glue::Connection.ConnectionInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-connection-connectioninput.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ConnectionInput {
        /// Property `ConnectionProperties`.
        #[serde(rename="ConnectionProperties")]
        pub connection_properties: ::json::Value,
        /// Property `ConnectionType`.
        #[serde(rename="ConnectionType")]
        pub connection_type: String,
        /// Property `Description`.
        #[serde(rename="Description")]
        pub description: String,
        /// Property `MatchCriteria`.
        #[serde(rename="MatchCriteria")]
        pub match_criteria: Vec<String>,
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
        /// Property `PhysicalConnectionRequirements`.
        #[serde(rename="PhysicalConnectionRequirements")]
        pub physical_connection_requirements: PhysicalConnectionRequirements,
    }

    /// The [`AWS::Glue::Connection.PhysicalConnectionRequirements`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-connection-physicalconnectionrequirements.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PhysicalConnectionRequirements {
        /// Property `AvailabilityZone`.
        #[serde(rename="AvailabilityZone")]
        pub availability_zone: String,
        /// Property `SecurityGroupIdList`.
        #[serde(rename="SecurityGroupIdList")]
        pub security_group_id_list: Vec<String>,
        /// Property `SubnetId`.
        #[serde(rename="SubnetId")]
        pub subnet_id: String,
    }
}

pub mod crawler {
    //! Property types for the `Crawler` resource.

    /// The [`AWS::Glue::Crawler.JdbcTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-jdbctarget.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct JdbcTarget {
        /// Property `ConnectionName`.
        #[serde(rename="ConnectionName")]
        pub connection_name: String,
        /// Property `Exclusions`.
        #[serde(rename="Exclusions")]
        pub exclusions: Vec<String>,
        /// Property `Path`.
        #[serde(rename="Path")]
        pub path: String,
    }

    /// The [`AWS::Glue::Crawler.S3Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-s3target.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct S3Target {
        /// Property `Exclusions`.
        #[serde(rename="Exclusions")]
        pub exclusions: Vec<String>,
        /// Property `Path`.
        #[serde(rename="Path")]
        pub path: String,
    }

    /// The [`AWS::Glue::Crawler.Schedule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-schedule.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Schedule {
        /// Property `ScheduleExpression`.
        #[serde(rename="ScheduleExpression")]
        pub schedule_expression: String,
    }

    /// The [`AWS::Glue::Crawler.SchemaChangePolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-schemachangepolicy.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SchemaChangePolicy {
        /// Property `DeleteBehavior`.
        #[serde(rename="DeleteBehavior")]
        pub delete_behavior: String,
        /// Property `UpdateBehavior`.
        #[serde(rename="UpdateBehavior")]
        pub update_behavior: String,
    }

    /// The [`AWS::Glue::Crawler.Targets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-targets.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Targets {
        /// Property `JdbcTargets`.
        #[serde(rename="JdbcTargets")]
        pub jdbc_targets: Vec<JdbcTarget>,
        /// Property `S3Targets`.
        #[serde(rename="S3Targets")]
        pub s3_targets: Vec<S3Target>,
    }
}

pub mod database {
    //! Property types for the `Database` resource.

    /// The [`AWS::Glue::Database.DatabaseInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-database-databaseinput.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DatabaseInput {
        /// Property `Description`.
        #[serde(rename="Description")]
        pub description: String,
        /// Property `LocationUri`.
        #[serde(rename="LocationUri")]
        pub location_uri: String,
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
        /// Property `Parameters`.
        #[serde(rename="Parameters")]
        pub parameters: ::json::Value,
    }
}

pub mod job {
    //! Property types for the `Job` resource.

    /// The [`AWS::Glue::Job.ConnectionsList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-job-connectionslist.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ConnectionsList {
        /// Property `Connections`.
        #[serde(rename="Connections")]
        pub connections: Vec<String>,
    }

    /// The [`AWS::Glue::Job.ExecutionProperty`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-job-executionproperty.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ExecutionProperty {
        /// Property `MaxConcurrentRuns`.
        #[serde(rename="MaxConcurrentRuns")]
        pub max_concurrent_runs: f64,
    }

    /// The [`AWS::Glue::Job.JobCommand`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-job-jobcommand.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct JobCommand {
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
        /// Property `ScriptLocation`.
        #[serde(rename="ScriptLocation")]
        pub script_location: String,
    }
}

pub mod partition {
    //! Property types for the `Partition` resource.

    /// The [`AWS::Glue::Partition.Column`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-column.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Column {
        /// Property `Comment`.
        #[serde(rename="Comment")]
        pub comment: String,
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
        /// Property `Type`.
        #[serde(rename="Type")]
        pub type_: String,
    }

    /// The [`AWS::Glue::Partition.Order`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-order.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Order {
        /// Property `Column`.
        #[serde(rename="Column")]
        pub column: String,
        /// Property `SortOrder`.
        #[serde(rename="SortOrder")]
        pub sort_order: u32,
    }

    /// The [`AWS::Glue::Partition.PartitionInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-partitioninput.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PartitionInput {
        /// Property `Parameters`.
        #[serde(rename="Parameters")]
        pub parameters: ::json::Value,
        /// Property `StorageDescriptor`.
        #[serde(rename="StorageDescriptor")]
        pub storage_descriptor: StorageDescriptor,
        /// Property `Values`.
        #[serde(rename="Values")]
        pub values: Vec<String>,
    }

    /// The [`AWS::Glue::Partition.SerdeInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-serdeinfo.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SerdeInfo {
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
        /// Property `Parameters`.
        #[serde(rename="Parameters")]
        pub parameters: ::json::Value,
        /// Property `SerializationLibrary`.
        #[serde(rename="SerializationLibrary")]
        pub serialization_library: String,
    }

    /// The [`AWS::Glue::Partition.SkewedInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-skewedinfo.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SkewedInfo {
        /// Property `SkewedColumnNames`.
        #[serde(rename="SkewedColumnNames")]
        pub skewed_column_names: Vec<String>,
        /// Property `SkewedColumnValueLocationMaps`.
        #[serde(rename="SkewedColumnValueLocationMaps")]
        pub skewed_column_value_location_maps: ::json::Value,
        /// Property `SkewedColumnValues`.
        #[serde(rename="SkewedColumnValues")]
        pub skewed_column_values: Vec<String>,
    }

    /// The [`AWS::Glue::Partition.StorageDescriptor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-storagedescriptor.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StorageDescriptor {
        /// Property `BucketColumns`.
        #[serde(rename="BucketColumns")]
        pub bucket_columns: Vec<String>,
        /// Property `Columns`.
        #[serde(rename="Columns")]
        pub columns: Vec<Column>,
        /// Property `Compressed`.
        #[serde(rename="Compressed")]
        pub compressed: bool,
        /// Property `InputFormat`.
        #[serde(rename="InputFormat")]
        pub input_format: String,
        /// Property `Location`.
        #[serde(rename="Location")]
        pub location: String,
        /// Property `NumberOfBuckets`.
        #[serde(rename="NumberOfBuckets")]
        pub number_of_buckets: u32,
        /// Property `OutputFormat`.
        #[serde(rename="OutputFormat")]
        pub output_format: String,
        /// Property `Parameters`.
        #[serde(rename="Parameters")]
        pub parameters: ::json::Value,
        /// Property `SerdeInfo`.
        #[serde(rename="SerdeInfo")]
        pub serde_info: SerdeInfo,
        /// Property `SkewedInfo`.
        #[serde(rename="SkewedInfo")]
        pub skewed_info: SkewedInfo,
        /// Property `SortColumns`.
        #[serde(rename="SortColumns")]
        pub sort_columns: Vec<Order>,
        /// Property `StoredAsSubDirectories`.
        #[serde(rename="StoredAsSubDirectories")]
        pub stored_as_sub_directories: bool,
    }
}

pub mod table {
    //! Property types for the `Table` resource.

    /// The [`AWS::Glue::Table.Column`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-column.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Column {
        /// Property `Comment`.
        #[serde(rename="Comment")]
        pub comment: String,
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
        /// Property `Type`.
        #[serde(rename="Type")]
        pub type_: String,
    }

    /// The [`AWS::Glue::Table.Order`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-order.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Order {
        /// Property `Column`.
        #[serde(rename="Column")]
        pub column: String,
        /// Property `SortOrder`.
        #[serde(rename="SortOrder")]
        pub sort_order: u32,
    }

    /// The [`AWS::Glue::Table.SerdeInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-serdeinfo.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SerdeInfo {
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
        /// Property `Parameters`.
        #[serde(rename="Parameters")]
        pub parameters: ::json::Value,
        /// Property `SerializationLibrary`.
        #[serde(rename="SerializationLibrary")]
        pub serialization_library: String,
    }

    /// The [`AWS::Glue::Table.SkewedInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-skewedinfo.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SkewedInfo {
        /// Property `SkewedColumnNames`.
        #[serde(rename="SkewedColumnNames")]
        pub skewed_column_names: Vec<String>,
        /// Property `SkewedColumnValueLocationMaps`.
        #[serde(rename="SkewedColumnValueLocationMaps")]
        pub skewed_column_value_location_maps: ::json::Value,
        /// Property `SkewedColumnValues`.
        #[serde(rename="SkewedColumnValues")]
        pub skewed_column_values: Vec<String>,
    }

    /// The [`AWS::Glue::Table.StorageDescriptor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-storagedescriptor.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StorageDescriptor {
        /// Property `BucketColumns`.
        #[serde(rename="BucketColumns")]
        pub bucket_columns: Vec<String>,
        /// Property `Columns`.
        #[serde(rename="Columns")]
        pub columns: Vec<Column>,
        /// Property `Compressed`.
        #[serde(rename="Compressed")]
        pub compressed: bool,
        /// Property `InputFormat`.
        #[serde(rename="InputFormat")]
        pub input_format: String,
        /// Property `Location`.
        #[serde(rename="Location")]
        pub location: String,
        /// Property `NumberOfBuckets`.
        #[serde(rename="NumberOfBuckets")]
        pub number_of_buckets: u32,
        /// Property `OutputFormat`.
        #[serde(rename="OutputFormat")]
        pub output_format: String,
        /// Property `Parameters`.
        #[serde(rename="Parameters")]
        pub parameters: ::json::Value,
        /// Property `SerdeInfo`.
        #[serde(rename="SerdeInfo")]
        pub serde_info: SerdeInfo,
        /// Property `SkewedInfo`.
        #[serde(rename="SkewedInfo")]
        pub skewed_info: SkewedInfo,
        /// Property `SortColumns`.
        #[serde(rename="SortColumns")]
        pub sort_columns: Vec<Order>,
        /// Property `StoredAsSubDirectories`.
        #[serde(rename="StoredAsSubDirectories")]
        pub stored_as_sub_directories: bool,
    }

    /// The [`AWS::Glue::Table.TableInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-tableinput.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TableInput {
        /// Property `Description`.
        #[serde(rename="Description")]
        pub description: String,
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
        /// Property `Owner`.
        #[serde(rename="Owner")]
        pub owner: String,
        /// Property `Parameters`.
        #[serde(rename="Parameters")]
        pub parameters: ::json::Value,
        /// Property `PartitionKeys`.
        #[serde(rename="PartitionKeys")]
        pub partition_keys: Vec<Column>,
        /// Property `Retention`.
        #[serde(rename="Retention")]
        pub retention: u32,
        /// Property `StorageDescriptor`.
        #[serde(rename="StorageDescriptor")]
        pub storage_descriptor: StorageDescriptor,
        /// Property `TableType`.
        #[serde(rename="TableType")]
        pub table_type: String,
        /// Property `ViewExpandedText`.
        #[serde(rename="ViewExpandedText")]
        pub view_expanded_text: String,
        /// Property `ViewOriginalText`.
        #[serde(rename="ViewOriginalText")]
        pub view_original_text: String,
    }
}

pub mod trigger {
    //! Property types for the `Trigger` resource.

    /// The [`AWS::Glue::Trigger.Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-action.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Action {
        /// Property `Arguments`.
        #[serde(rename="Arguments")]
        pub arguments: ::json::Value,
        /// Property `JobName`.
        #[serde(rename="JobName")]
        pub job_name: String,
    }

    /// The [`AWS::Glue::Trigger.Condition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-condition.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Condition {
        /// Property `JobName`.
        #[serde(rename="JobName")]
        pub job_name: String,
        /// Property `LogicalOperator`.
        #[serde(rename="LogicalOperator")]
        pub logical_operator: String,
        /// Property `State`.
        #[serde(rename="State")]
        pub state: String,
    }

    /// The [`AWS::Glue::Trigger.Predicate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-predicate.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Predicate {
        /// Property `Conditions`.
        #[serde(rename="Conditions")]
        pub conditions: Vec<Condition>,
        /// Property `Logical`.
        #[serde(rename="Logical")]
        pub logical: String,
    }
}
