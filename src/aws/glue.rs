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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grok_classifier: Option<self::classifier::GrokClassifier>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifiers: Option<Vec<String>>,
    /// Property `DatabaseName`.
    #[serde(rename="DatabaseName")]
    pub database_name: String,
    /// Property `Description`.
    #[serde(rename="Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Property `Name`.
    #[serde(rename="Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Property `Role`.
    #[serde(rename="Role")]
    pub role: String,
    /// Property `Schedule`.
    #[serde(rename="Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<self::crawler::Schedule>,
    /// Property `SchemaChangePolicy`.
    #[serde(rename="SchemaChangePolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_change_policy: Option<self::crawler::SchemaChangePolicy>,
    /// Property `TablePrefix`.
    #[serde(rename="TablePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_prefix: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_name: Option<String>,
    /// Property `ExtraJarsS3Path`.
    #[serde(rename="ExtraJarsS3Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_jars_s3_path: Option<String>,
    /// Property `ExtraPythonLibsS3Path`.
    #[serde(rename="ExtraPythonLibsS3Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_python_libs_s3_path: Option<String>,
    /// Property `NumberOfNodes`.
    #[serde(rename="NumberOfNodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_nodes: Option<u32>,
    /// Property `PublicKey`.
    #[serde(rename="PublicKey")]
    pub public_key: String,
    /// Property `RoleArn`.
    #[serde(rename="RoleArn")]
    pub role_arn: String,
    /// Property `SecurityGroupIds`.
    #[serde(rename="SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// Property `SubnetId`.
    #[serde(rename="SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_capacity: Option<f64>,
    /// Property `Command`.
    #[serde(rename="Command")]
    pub command: self::job::JobCommand,
    /// Property `Connections`.
    #[serde(rename="Connections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<self::job::ConnectionsList>,
    /// Property `DefaultArguments`.
    #[serde(rename="DefaultArguments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_arguments: Option<::json::Value>,
    /// Property `Description`.
    #[serde(rename="Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Property `ExecutionProperty`.
    #[serde(rename="ExecutionProperty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_property: Option<self::job::ExecutionProperty>,
    /// Property `LogUri`.
    #[serde(rename="LogUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_uri: Option<String>,
    /// Property `MaxRetries`.
    #[serde(rename="MaxRetries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<f64>,
    /// Property `Name`.
    #[serde(rename="Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Property `Name`.
    #[serde(rename="Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Property `Predicate`.
    #[serde(rename="Predicate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicate: Option<self::trigger::Predicate>,
    /// Property `Schedule`.
    #[serde(rename="Schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none")]
        pub custom_patterns: Option<String>,
        /// Property `GrokPattern`.
        #[serde(rename="GrokPattern")]
        pub grok_pattern: String,
        /// Property `Name`.
        #[serde(rename="Name")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        /// Property `MatchCriteria`.
        #[serde(rename="MatchCriteria")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub match_criteria: Option<Vec<String>>,
        /// Property `Name`.
        #[serde(rename="Name")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        /// Property `PhysicalConnectionRequirements`.
        #[serde(rename="PhysicalConnectionRequirements")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub physical_connection_requirements: Option<PhysicalConnectionRequirements>,
    }

    /// The [`AWS::Glue::Connection.PhysicalConnectionRequirements`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-connection-physicalconnectionrequirements.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PhysicalConnectionRequirements {
        /// Property `AvailabilityZone`.
        #[serde(rename="AvailabilityZone")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub availability_zone: Option<String>,
        /// Property `SecurityGroupIdList`.
        #[serde(rename="SecurityGroupIdList")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub security_group_id_list: Option<Vec<String>>,
        /// Property `SubnetId`.
        #[serde(rename="SubnetId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub subnet_id: Option<String>,
    }
}

pub mod crawler {
    //! Property types for the `Crawler` resource.

    /// The [`AWS::Glue::Crawler.JdbcTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-jdbctarget.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct JdbcTarget {
        /// Property `ConnectionName`.
        #[serde(rename="ConnectionName")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub connection_name: Option<String>,
        /// Property `Exclusions`.
        #[serde(rename="Exclusions")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub exclusions: Option<Vec<String>>,
        /// Property `Path`.
        #[serde(rename="Path")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub path: Option<String>,
    }

    /// The [`AWS::Glue::Crawler.S3Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-s3target.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct S3Target {
        /// Property `Exclusions`.
        #[serde(rename="Exclusions")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub exclusions: Option<Vec<String>>,
        /// Property `Path`.
        #[serde(rename="Path")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub path: Option<String>,
    }

    /// The [`AWS::Glue::Crawler.Schedule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-schedule.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Schedule {
        /// Property `ScheduleExpression`.
        #[serde(rename="ScheduleExpression")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub schedule_expression: Option<String>,
    }

    /// The [`AWS::Glue::Crawler.SchemaChangePolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-schemachangepolicy.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SchemaChangePolicy {
        /// Property `DeleteBehavior`.
        #[serde(rename="DeleteBehavior")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub delete_behavior: Option<String>,
        /// Property `UpdateBehavior`.
        #[serde(rename="UpdateBehavior")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub update_behavior: Option<String>,
    }

    /// The [`AWS::Glue::Crawler.Targets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-targets.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Targets {
        /// Property `JdbcTargets`.
        #[serde(rename="JdbcTargets")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub jdbc_targets: Option<Vec<JdbcTarget>>,
        /// Property `S3Targets`.
        #[serde(rename="S3Targets")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub s3_targets: Option<Vec<S3Target>>,
    }
}

pub mod database {
    //! Property types for the `Database` resource.

    /// The [`AWS::Glue::Database.DatabaseInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-database-databaseinput.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DatabaseInput {
        /// Property `Description`.
        #[serde(rename="Description")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        /// Property `LocationUri`.
        #[serde(rename="LocationUri")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub location_uri: Option<String>,
        /// Property `Name`.
        #[serde(rename="Name")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        /// Property `Parameters`.
        #[serde(rename="Parameters")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub parameters: Option<::json::Value>,
    }
}

pub mod job {
    //! Property types for the `Job` resource.

    /// The [`AWS::Glue::Job.ConnectionsList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-job-connectionslist.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ConnectionsList {
        /// Property `Connections`.
        #[serde(rename="Connections")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub connections: Option<Vec<String>>,
    }

    /// The [`AWS::Glue::Job.ExecutionProperty`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-job-executionproperty.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ExecutionProperty {
        /// Property `MaxConcurrentRuns`.
        #[serde(rename="MaxConcurrentRuns")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub max_concurrent_runs: Option<f64>,
    }

    /// The [`AWS::Glue::Job.JobCommand`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-job-jobcommand.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct JobCommand {
        /// Property `Name`.
        #[serde(rename="Name")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        /// Property `ScriptLocation`.
        #[serde(rename="ScriptLocation")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub script_location: Option<String>,
    }
}

pub mod partition {
    //! Property types for the `Partition` resource.

    /// The [`AWS::Glue::Partition.Column`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-column.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Column {
        /// Property `Comment`.
        #[serde(rename="Comment")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub comment: Option<String>,
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
        /// Property `Type`.
        #[serde(rename="Type")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    /// The [`AWS::Glue::Partition.Order`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-order.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Order {
        /// Property `Column`.
        #[serde(rename="Column")]
        pub column: String,
        /// Property `SortOrder`.
        #[serde(rename="SortOrder")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sort_order: Option<u32>,
    }

    /// The [`AWS::Glue::Partition.PartitionInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-partitioninput.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PartitionInput {
        /// Property `Parameters`.
        #[serde(rename="Parameters")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub parameters: Option<::json::Value>,
        /// Property `StorageDescriptor`.
        #[serde(rename="StorageDescriptor")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub storage_descriptor: Option<StorageDescriptor>,
        /// Property `Values`.
        #[serde(rename="Values")]
        pub values: Vec<String>,
    }

    /// The [`AWS::Glue::Partition.SerdeInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-serdeinfo.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SerdeInfo {
        /// Property `Name`.
        #[serde(rename="Name")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        /// Property `Parameters`.
        #[serde(rename="Parameters")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub parameters: Option<::json::Value>,
        /// Property `SerializationLibrary`.
        #[serde(rename="SerializationLibrary")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub serialization_library: Option<String>,
    }

    /// The [`AWS::Glue::Partition.SkewedInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-skewedinfo.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SkewedInfo {
        /// Property `SkewedColumnNames`.
        #[serde(rename="SkewedColumnNames")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub skewed_column_names: Option<Vec<String>>,
        /// Property `SkewedColumnValueLocationMaps`.
        #[serde(rename="SkewedColumnValueLocationMaps")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub skewed_column_value_location_maps: Option<::json::Value>,
        /// Property `SkewedColumnValues`.
        #[serde(rename="SkewedColumnValues")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub skewed_column_values: Option<Vec<String>>,
    }

    /// The [`AWS::Glue::Partition.StorageDescriptor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-storagedescriptor.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StorageDescriptor {
        /// Property `BucketColumns`.
        #[serde(rename="BucketColumns")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub bucket_columns: Option<Vec<String>>,
        /// Property `Columns`.
        #[serde(rename="Columns")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub columns: Option<Vec<Column>>,
        /// Property `Compressed`.
        #[serde(rename="Compressed")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub compressed: Option<bool>,
        /// Property `InputFormat`.
        #[serde(rename="InputFormat")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub input_format: Option<String>,
        /// Property `Location`.
        #[serde(rename="Location")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub location: Option<String>,
        /// Property `NumberOfBuckets`.
        #[serde(rename="NumberOfBuckets")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub number_of_buckets: Option<u32>,
        /// Property `OutputFormat`.
        #[serde(rename="OutputFormat")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub output_format: Option<String>,
        /// Property `Parameters`.
        #[serde(rename="Parameters")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub parameters: Option<::json::Value>,
        /// Property `SerdeInfo`.
        #[serde(rename="SerdeInfo")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub serde_info: Option<SerdeInfo>,
        /// Property `SkewedInfo`.
        #[serde(rename="SkewedInfo")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub skewed_info: Option<SkewedInfo>,
        /// Property `SortColumns`.
        #[serde(rename="SortColumns")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sort_columns: Option<Vec<Order>>,
        /// Property `StoredAsSubDirectories`.
        #[serde(rename="StoredAsSubDirectories")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub stored_as_sub_directories: Option<bool>,
    }
}

pub mod table {
    //! Property types for the `Table` resource.

    /// The [`AWS::Glue::Table.Column`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-column.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Column {
        /// Property `Comment`.
        #[serde(rename="Comment")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub comment: Option<String>,
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
        /// Property `Type`.
        #[serde(rename="Type")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
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
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        /// Property `Parameters`.
        #[serde(rename="Parameters")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub parameters: Option<::json::Value>,
        /// Property `SerializationLibrary`.
        #[serde(rename="SerializationLibrary")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub serialization_library: Option<String>,
    }

    /// The [`AWS::Glue::Table.SkewedInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-skewedinfo.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SkewedInfo {
        /// Property `SkewedColumnNames`.
        #[serde(rename="SkewedColumnNames")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub skewed_column_names: Option<Vec<String>>,
        /// Property `SkewedColumnValueLocationMaps`.
        #[serde(rename="SkewedColumnValueLocationMaps")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub skewed_column_value_location_maps: Option<::json::Value>,
        /// Property `SkewedColumnValues`.
        #[serde(rename="SkewedColumnValues")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub skewed_column_values: Option<Vec<String>>,
    }

    /// The [`AWS::Glue::Table.StorageDescriptor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-storagedescriptor.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StorageDescriptor {
        /// Property `BucketColumns`.
        #[serde(rename="BucketColumns")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub bucket_columns: Option<Vec<String>>,
        /// Property `Columns`.
        #[serde(rename="Columns")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub columns: Option<Vec<Column>>,
        /// Property `Compressed`.
        #[serde(rename="Compressed")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub compressed: Option<bool>,
        /// Property `InputFormat`.
        #[serde(rename="InputFormat")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub input_format: Option<String>,
        /// Property `Location`.
        #[serde(rename="Location")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub location: Option<String>,
        /// Property `NumberOfBuckets`.
        #[serde(rename="NumberOfBuckets")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub number_of_buckets: Option<u32>,
        /// Property `OutputFormat`.
        #[serde(rename="OutputFormat")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub output_format: Option<String>,
        /// Property `Parameters`.
        #[serde(rename="Parameters")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub parameters: Option<::json::Value>,
        /// Property `SerdeInfo`.
        #[serde(rename="SerdeInfo")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub serde_info: Option<SerdeInfo>,
        /// Property `SkewedInfo`.
        #[serde(rename="SkewedInfo")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub skewed_info: Option<SkewedInfo>,
        /// Property `SortColumns`.
        #[serde(rename="SortColumns")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sort_columns: Option<Vec<Order>>,
        /// Property `StoredAsSubDirectories`.
        #[serde(rename="StoredAsSubDirectories")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub stored_as_sub_directories: Option<bool>,
    }

    /// The [`AWS::Glue::Table.TableInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-tableinput.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TableInput {
        /// Property `Description`.
        #[serde(rename="Description")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        /// Property `Name`.
        #[serde(rename="Name")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        /// Property `Owner`.
        #[serde(rename="Owner")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub owner: Option<String>,
        /// Property `Parameters`.
        #[serde(rename="Parameters")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub parameters: Option<::json::Value>,
        /// Property `PartitionKeys`.
        #[serde(rename="PartitionKeys")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub partition_keys: Option<Vec<Column>>,
        /// Property `Retention`.
        #[serde(rename="Retention")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub retention: Option<u32>,
        /// Property `StorageDescriptor`.
        #[serde(rename="StorageDescriptor")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub storage_descriptor: Option<StorageDescriptor>,
        /// Property `TableType`.
        #[serde(rename="TableType")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub table_type: Option<String>,
        /// Property `ViewExpandedText`.
        #[serde(rename="ViewExpandedText")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub view_expanded_text: Option<String>,
        /// Property `ViewOriginalText`.
        #[serde(rename="ViewOriginalText")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub view_original_text: Option<String>,
    }
}

pub mod trigger {
    //! Property types for the `Trigger` resource.

    /// The [`AWS::Glue::Trigger.Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-action.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Action {
        /// Property `Arguments`.
        #[serde(rename="Arguments")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub arguments: Option<::json::Value>,
        /// Property `JobName`.
        #[serde(rename="JobName")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub job_name: Option<String>,
    }

    /// The [`AWS::Glue::Trigger.Condition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-condition.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Condition {
        /// Property `JobName`.
        #[serde(rename="JobName")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub job_name: Option<String>,
        /// Property `LogicalOperator`.
        #[serde(rename="LogicalOperator")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub logical_operator: Option<String>,
        /// Property `State`.
        #[serde(rename="State")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub state: Option<String>,
    }

    /// The [`AWS::Glue::Trigger.Predicate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-predicate.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Predicate {
        /// Property `Conditions`.
        #[serde(rename="Conditions")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub conditions: Option<Vec<Condition>>,
        /// Property `Logical`.
        #[serde(rename="Logical")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub logical: Option<String>,
    }
}
