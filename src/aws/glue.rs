/// The [`AWS::Glue::Classifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-classifier.html) resource type.
pub struct Classifier {
    properties: ClassifierProperties
}

/// Properties for the `Classifier` resource.
#[derive(Serialize, Deserialize)]
pub struct ClassifierProperties {
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
pub struct Connection {
    properties: ConnectionProperties
}

/// Properties for the `Connection` resource.
#[derive(Serialize, Deserialize)]
pub struct ConnectionProperties {
    #[serde(rename="CatalogId")]
    pub catalog_id: String,
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
pub struct Crawler {
    properties: CrawlerProperties
}

/// Properties for the `Crawler` resource.
#[derive(Serialize, Deserialize)]
pub struct CrawlerProperties {
    #[serde(rename="Classifiers")]
    pub classifiers: Vec<String>,
    #[serde(rename="DatabaseName")]
    pub database_name: String,
    #[serde(rename="Description")]
    pub description: String,
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="Role")]
    pub role: String,
    #[serde(rename="Schedule")]
    pub schedule: self::crawler::Schedule,
    #[serde(rename="SchemaChangePolicy")]
    pub schema_change_policy: self::crawler::SchemaChangePolicy,
    #[serde(rename="TablePrefix")]
    pub table_prefix: String,
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
pub struct Database {
    properties: DatabaseProperties
}

/// Properties for the `Database` resource.
#[derive(Serialize, Deserialize)]
pub struct DatabaseProperties {
    #[serde(rename="CatalogId")]
    pub catalog_id: String,
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
pub struct DevEndpoint {
    properties: DevEndpointProperties
}

/// Properties for the `DevEndpoint` resource.
#[derive(Serialize, Deserialize)]
pub struct DevEndpointProperties {
    #[serde(rename="EndpointName")]
    pub endpoint_name: String,
    #[serde(rename="ExtraJarsS3Path")]
    pub extra_jars_s3_path: String,
    #[serde(rename="ExtraPythonLibsS3Path")]
    pub extra_python_libs_s3_path: String,
    #[serde(rename="NumberOfNodes")]
    pub number_of_nodes: u32,
    #[serde(rename="PublicKey")]
    pub public_key: String,
    #[serde(rename="RoleArn")]
    pub role_arn: String,
    #[serde(rename="SecurityGroupIds")]
    pub security_group_ids: Vec<String>,
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
pub struct Job {
    properties: JobProperties
}

/// Properties for the `Job` resource.
#[derive(Serialize, Deserialize)]
pub struct JobProperties {
    #[serde(rename="AllocatedCapacity")]
    pub allocated_capacity: f64,
    #[serde(rename="Command")]
    pub command: self::job::JobCommand,
    #[serde(rename="Connections")]
    pub connections: self::job::ConnectionsList,
    #[serde(rename="DefaultArguments")]
    pub default_arguments: ::json::Value,
    #[serde(rename="Description")]
    pub description: String,
    #[serde(rename="ExecutionProperty")]
    pub execution_property: self::job::ExecutionProperty,
    #[serde(rename="LogUri")]
    pub log_uri: String,
    #[serde(rename="MaxRetries")]
    pub max_retries: f64,
    #[serde(rename="Name")]
    pub name: String,
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
pub struct Partition {
    properties: PartitionProperties
}

/// Properties for the `Partition` resource.
#[derive(Serialize, Deserialize)]
pub struct PartitionProperties {
    #[serde(rename="CatalogId")]
    pub catalog_id: String,
    #[serde(rename="DatabaseName")]
    pub database_name: String,
    #[serde(rename="PartitionInput")]
    pub partition_input: self::partition::PartitionInput,
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
pub struct Table {
    properties: TableProperties
}

/// Properties for the `Table` resource.
#[derive(Serialize, Deserialize)]
pub struct TableProperties {
    #[serde(rename="CatalogId")]
    pub catalog_id: String,
    #[serde(rename="DatabaseName")]
    pub database_name: String,
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
pub struct Trigger {
    properties: TriggerProperties
}

/// Properties for the `Trigger` resource.
#[derive(Serialize, Deserialize)]
pub struct TriggerProperties {
    #[serde(rename="Actions")]
    pub actions: Vec<self::trigger::Action>,
    #[serde(rename="Description")]
    pub description: String,
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="Predicate")]
    pub predicate: self::trigger::Predicate,
    #[serde(rename="Schedule")]
    pub schedule: String,
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
    /// The [`AWS::Glue::Classifier.GrokClassifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-classifier-grokclassifier.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct GrokClassifier {
        #[serde(rename="Classification")]
        pub classification: String,
        #[serde(rename="CustomPatterns")]
        pub custom_patterns: String,
        #[serde(rename="GrokPattern")]
        pub grok_pattern: String,
        #[serde(rename="Name")]
        pub name: String,
    }

}

pub mod connection {
    /// The [`AWS::Glue::Connection.ConnectionInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-connection-connectioninput.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct ConnectionInput {
        #[serde(rename="ConnectionProperties")]
        pub connection_properties: ::json::Value,
        #[serde(rename="ConnectionType")]
        pub connection_type: String,
        #[serde(rename="Description")]
        pub description: String,
        #[serde(rename="MatchCriteria")]
        pub match_criteria: Vec<String>,
        #[serde(rename="Name")]
        pub name: String,
        #[serde(rename="PhysicalConnectionRequirements")]
        pub physical_connection_requirements: PhysicalConnectionRequirements,
    }

    /// The [`AWS::Glue::Connection.PhysicalConnectionRequirements`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-connection-physicalconnectionrequirements.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct PhysicalConnectionRequirements {
        #[serde(rename="AvailabilityZone")]
        pub availability_zone: String,
        #[serde(rename="SecurityGroupIdList")]
        pub security_group_id_list: Vec<String>,
        #[serde(rename="SubnetId")]
        pub subnet_id: String,
    }

}

pub mod crawler {
    /// The [`AWS::Glue::Crawler.JdbcTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-jdbctarget.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct JdbcTarget {
        #[serde(rename="ConnectionName")]
        pub connection_name: String,
        #[serde(rename="Exclusions")]
        pub exclusions: Vec<String>,
        #[serde(rename="Path")]
        pub path: String,
    }

    /// The [`AWS::Glue::Crawler.S3Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-s3target.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct S3Target {
        #[serde(rename="Exclusions")]
        pub exclusions: Vec<String>,
        #[serde(rename="Path")]
        pub path: String,
    }

    /// The [`AWS::Glue::Crawler.Schedule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-schedule.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Schedule {
        #[serde(rename="ScheduleExpression")]
        pub schedule_expression: String,
    }

    /// The [`AWS::Glue::Crawler.SchemaChangePolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-schemachangepolicy.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct SchemaChangePolicy {
        #[serde(rename="DeleteBehavior")]
        pub delete_behavior: String,
        #[serde(rename="UpdateBehavior")]
        pub update_behavior: String,
    }

    /// The [`AWS::Glue::Crawler.Targets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-targets.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Targets {
        #[serde(rename="JdbcTargets")]
        pub jdbc_targets: Vec<JdbcTarget>,
        #[serde(rename="S3Targets")]
        pub s3_targets: Vec<S3Target>,
    }

}

pub mod database {
    /// The [`AWS::Glue::Database.DatabaseInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-database-databaseinput.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct DatabaseInput {
        #[serde(rename="Description")]
        pub description: String,
        #[serde(rename="LocationUri")]
        pub location_uri: String,
        #[serde(rename="Name")]
        pub name: String,
        #[serde(rename="Parameters")]
        pub parameters: ::json::Value,
    }

}

pub mod job {
    /// The [`AWS::Glue::Job.ConnectionsList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-job-connectionslist.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct ConnectionsList {
        #[serde(rename="Connections")]
        pub connections: Vec<String>,
    }

    /// The [`AWS::Glue::Job.ExecutionProperty`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-job-executionproperty.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct ExecutionProperty {
        #[serde(rename="MaxConcurrentRuns")]
        pub max_concurrent_runs: f64,
    }

    /// The [`AWS::Glue::Job.JobCommand`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-job-jobcommand.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct JobCommand {
        #[serde(rename="Name")]
        pub name: String,
        #[serde(rename="ScriptLocation")]
        pub script_location: String,
    }

}

pub mod partition {
    /// The [`AWS::Glue::Partition.Column`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-column.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Column {
        #[serde(rename="Comment")]
        pub comment: String,
        #[serde(rename="Name")]
        pub name: String,
        #[serde(rename="Type")]
        pub type_: String,
    }

    /// The [`AWS::Glue::Partition.Order`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-order.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Order {
        #[serde(rename="Column")]
        pub column: String,
        #[serde(rename="SortOrder")]
        pub sort_order: u32,
    }

    /// The [`AWS::Glue::Partition.PartitionInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-partitioninput.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct PartitionInput {
        #[serde(rename="Parameters")]
        pub parameters: ::json::Value,
        #[serde(rename="StorageDescriptor")]
        pub storage_descriptor: StorageDescriptor,
        #[serde(rename="Values")]
        pub values: Vec<String>,
    }

    /// The [`AWS::Glue::Partition.SerdeInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-serdeinfo.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct SerdeInfo {
        #[serde(rename="Name")]
        pub name: String,
        #[serde(rename="Parameters")]
        pub parameters: ::json::Value,
        #[serde(rename="SerializationLibrary")]
        pub serialization_library: String,
    }

    /// The [`AWS::Glue::Partition.SkewedInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-skewedinfo.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct SkewedInfo {
        #[serde(rename="SkewedColumnNames")]
        pub skewed_column_names: Vec<String>,
        #[serde(rename="SkewedColumnValueLocationMaps")]
        pub skewed_column_value_location_maps: ::json::Value,
        #[serde(rename="SkewedColumnValues")]
        pub skewed_column_values: Vec<String>,
    }

    /// The [`AWS::Glue::Partition.StorageDescriptor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-storagedescriptor.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct StorageDescriptor {
        #[serde(rename="BucketColumns")]
        pub bucket_columns: Vec<String>,
        #[serde(rename="Columns")]
        pub columns: Vec<Column>,
        #[serde(rename="Compressed")]
        pub compressed: bool,
        #[serde(rename="InputFormat")]
        pub input_format: String,
        #[serde(rename="Location")]
        pub location: String,
        #[serde(rename="NumberOfBuckets")]
        pub number_of_buckets: u32,
        #[serde(rename="OutputFormat")]
        pub output_format: String,
        #[serde(rename="Parameters")]
        pub parameters: ::json::Value,
        #[serde(rename="SerdeInfo")]
        pub serde_info: SerdeInfo,
        #[serde(rename="SkewedInfo")]
        pub skewed_info: SkewedInfo,
        #[serde(rename="SortColumns")]
        pub sort_columns: Vec<Order>,
        #[serde(rename="StoredAsSubDirectories")]
        pub stored_as_sub_directories: bool,
    }

}

pub mod table {
    /// The [`AWS::Glue::Table.Column`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-column.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Column {
        #[serde(rename="Comment")]
        pub comment: String,
        #[serde(rename="Name")]
        pub name: String,
        #[serde(rename="Type")]
        pub type_: String,
    }

    /// The [`AWS::Glue::Table.Order`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-order.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Order {
        #[serde(rename="Column")]
        pub column: String,
        #[serde(rename="SortOrder")]
        pub sort_order: u32,
    }

    /// The [`AWS::Glue::Table.SerdeInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-serdeinfo.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct SerdeInfo {
        #[serde(rename="Name")]
        pub name: String,
        #[serde(rename="Parameters")]
        pub parameters: ::json::Value,
        #[serde(rename="SerializationLibrary")]
        pub serialization_library: String,
    }

    /// The [`AWS::Glue::Table.SkewedInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-skewedinfo.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct SkewedInfo {
        #[serde(rename="SkewedColumnNames")]
        pub skewed_column_names: Vec<String>,
        #[serde(rename="SkewedColumnValueLocationMaps")]
        pub skewed_column_value_location_maps: ::json::Value,
        #[serde(rename="SkewedColumnValues")]
        pub skewed_column_values: Vec<String>,
    }

    /// The [`AWS::Glue::Table.StorageDescriptor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-storagedescriptor.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct StorageDescriptor {
        #[serde(rename="BucketColumns")]
        pub bucket_columns: Vec<String>,
        #[serde(rename="Columns")]
        pub columns: Vec<Column>,
        #[serde(rename="Compressed")]
        pub compressed: bool,
        #[serde(rename="InputFormat")]
        pub input_format: String,
        #[serde(rename="Location")]
        pub location: String,
        #[serde(rename="NumberOfBuckets")]
        pub number_of_buckets: u32,
        #[serde(rename="OutputFormat")]
        pub output_format: String,
        #[serde(rename="Parameters")]
        pub parameters: ::json::Value,
        #[serde(rename="SerdeInfo")]
        pub serde_info: SerdeInfo,
        #[serde(rename="SkewedInfo")]
        pub skewed_info: SkewedInfo,
        #[serde(rename="SortColumns")]
        pub sort_columns: Vec<Order>,
        #[serde(rename="StoredAsSubDirectories")]
        pub stored_as_sub_directories: bool,
    }

    /// The [`AWS::Glue::Table.TableInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-tableinput.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct TableInput {
        #[serde(rename="Description")]
        pub description: String,
        #[serde(rename="Name")]
        pub name: String,
        #[serde(rename="Owner")]
        pub owner: String,
        #[serde(rename="Parameters")]
        pub parameters: ::json::Value,
        #[serde(rename="PartitionKeys")]
        pub partition_keys: Vec<Column>,
        #[serde(rename="Retention")]
        pub retention: u32,
        #[serde(rename="StorageDescriptor")]
        pub storage_descriptor: StorageDescriptor,
        #[serde(rename="TableType")]
        pub table_type: String,
        #[serde(rename="ViewExpandedText")]
        pub view_expanded_text: String,
        #[serde(rename="ViewOriginalText")]
        pub view_original_text: String,
    }

}

pub mod trigger {
    /// The [`AWS::Glue::Trigger.Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-action.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Action {
        #[serde(rename="Arguments")]
        pub arguments: ::json::Value,
        #[serde(rename="JobName")]
        pub job_name: String,
    }

    /// The [`AWS::Glue::Trigger.Condition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-condition.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Condition {
        #[serde(rename="JobName")]
        pub job_name: String,
        #[serde(rename="LogicalOperator")]
        pub logical_operator: String,
        #[serde(rename="State")]
        pub state: String,
    }

    /// The [`AWS::Glue::Trigger.Predicate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-predicate.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Predicate {
        #[serde(rename="Conditions")]
        pub conditions: Vec<Condition>,
        #[serde(rename="Logical")]
        pub logical: String,
    }

}

