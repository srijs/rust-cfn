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
    #[serde(rename = "GrokClassifier")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grok_classifier: Option<::Value<self::classifier::GrokClassifier>>,
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
    #[serde(rename = "CatalogId")]
    pub catalog_id: ::Value<String>,
    /// Property `ConnectionInput`.
    #[serde(rename = "ConnectionInput")]
    pub connection_input: ::Value<self::connection::ConnectionInput>,
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
    #[serde(rename = "Classifiers")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classifiers: Option<::ValueList<String>>,
    /// Property `DatabaseName`.
    #[serde(rename = "DatabaseName")]
    pub database_name: ::Value<String>,
    /// Property `Description`.
    #[serde(rename = "Description")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<::Value<String>>,
    /// Property `Name`.
    #[serde(rename = "Name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<::Value<String>>,
    /// Property `Role`.
    #[serde(rename = "Role")]
    pub role: ::Value<String>,
    /// Property `Schedule`.
    #[serde(rename = "Schedule")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<::Value<self::crawler::Schedule>>,
    /// Property `SchemaChangePolicy`.
    #[serde(rename = "SchemaChangePolicy")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_change_policy: Option<::Value<self::crawler::SchemaChangePolicy>>,
    /// Property `TablePrefix`.
    #[serde(rename = "TablePrefix")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table_prefix: Option<::Value<String>>,
    /// Property `Targets`.
    #[serde(rename = "Targets")]
    pub targets: ::Value<self::crawler::Targets>,
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
    #[serde(rename = "CatalogId")]
    pub catalog_id: ::Value<String>,
    /// Property `DatabaseInput`.
    #[serde(rename = "DatabaseInput")]
    pub database_input: ::Value<self::database::DatabaseInput>,
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
    #[serde(rename = "EndpointName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint_name: Option<::Value<String>>,
    /// Property `ExtraJarsS3Path`.
    #[serde(rename = "ExtraJarsS3Path")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_jars_s3_path: Option<::Value<String>>,
    /// Property `ExtraPythonLibsS3Path`.
    #[serde(rename = "ExtraPythonLibsS3Path")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_python_libs_s3_path: Option<::Value<String>>,
    /// Property `NumberOfNodes`.
    #[serde(rename = "NumberOfNodes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_of_nodes: Option<::Value<u32>>,
    /// Property `PublicKey`.
    #[serde(rename = "PublicKey")]
    pub public_key: ::Value<String>,
    /// Property `RoleArn`.
    #[serde(rename = "RoleArn")]
    pub role_arn: ::Value<String>,
    /// Property `SecurityGroupIds`.
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<::ValueList<String>>,
    /// Property `SubnetId`.
    #[serde(rename = "SubnetId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<::Value<String>>,
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
    #[serde(rename = "AllocatedCapacity")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocated_capacity: Option<::Value<f64>>,
    /// Property `Command`.
    #[serde(rename = "Command")]
    pub command: ::Value<self::job::JobCommand>,
    /// Property `Connections`.
    #[serde(rename = "Connections")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connections: Option<::Value<self::job::ConnectionsList>>,
    /// Property `DefaultArguments`.
    #[serde(rename = "DefaultArguments")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_arguments: Option<::Value<::json::Value>>,
    /// Property `Description`.
    #[serde(rename = "Description")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<::Value<String>>,
    /// Property `ExecutionProperty`.
    #[serde(rename = "ExecutionProperty")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_property: Option<::Value<self::job::ExecutionProperty>>,
    /// Property `LogUri`.
    #[serde(rename = "LogUri")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub log_uri: Option<::Value<String>>,
    /// Property `MaxRetries`.
    #[serde(rename = "MaxRetries")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<::Value<f64>>,
    /// Property `Name`.
    #[serde(rename = "Name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<::Value<String>>,
    /// Property `Role`.
    #[serde(rename = "Role")]
    pub role: ::Value<String>,
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
    #[serde(rename = "CatalogId")]
    pub catalog_id: ::Value<String>,
    /// Property `DatabaseName`.
    #[serde(rename = "DatabaseName")]
    pub database_name: ::Value<String>,
    /// Property `PartitionInput`.
    #[serde(rename = "PartitionInput")]
    pub partition_input: ::Value<self::partition::PartitionInput>,
    /// Property `TableName`.
    #[serde(rename = "TableName")]
    pub table_name: ::Value<String>,
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
    #[serde(rename = "CatalogId")]
    pub catalog_id: ::Value<String>,
    /// Property `DatabaseName`.
    #[serde(rename = "DatabaseName")]
    pub database_name: ::Value<String>,
    /// Property `TableInput`.
    #[serde(rename = "TableInput")]
    pub table_input: ::Value<self::table::TableInput>,
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
    #[serde(rename = "Actions")]
    pub actions: ::ValueList<self::trigger::Action>,
    /// Property `Description`.
    #[serde(rename = "Description")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<::Value<String>>,
    /// Property `Name`.
    #[serde(rename = "Name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<::Value<String>>,
    /// Property `Predicate`.
    #[serde(rename = "Predicate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub predicate: Option<::Value<self::trigger::Predicate>>,
    /// Property `Schedule`.
    #[serde(rename = "Schedule")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<::Value<String>>,
    /// Property `Type`.
    #[serde(rename = "Type")]
    pub type_: ::Value<String>,
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
        #[serde(rename = "Classification")]
        pub classification: ::Value<String>,
        /// Property `CustomPatterns`.
        #[serde(rename = "CustomPatterns")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub custom_patterns: Option<::Value<String>>,
        /// Property `GrokPattern`.
        #[serde(rename = "GrokPattern")]
        pub grok_pattern: ::Value<String>,
        /// Property `Name`.
        #[serde(rename = "Name")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(GrokClassifier);
}

pub mod connection {
    //! Property types for the `Connection` resource.

    /// The [`AWS::Glue::Connection.ConnectionInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-connection-connectioninput.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ConnectionInput {
        /// Property `ConnectionProperties`.
        #[serde(rename = "ConnectionProperties")]
        pub connection_properties: ::Value<::json::Value>,
        /// Property `ConnectionType`.
        #[serde(rename = "ConnectionType")]
        pub connection_type: ::Value<String>,
        /// Property `Description`.
        #[serde(rename = "Description")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<::Value<String>>,
        /// Property `MatchCriteria`.
        #[serde(rename = "MatchCriteria")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub match_criteria: Option<::ValueList<String>>,
        /// Property `Name`.
        #[serde(rename = "Name")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<::Value<String>>,
        /// Property `PhysicalConnectionRequirements`.
        #[serde(rename = "PhysicalConnectionRequirements")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub physical_connection_requirements: Option<::Value<PhysicalConnectionRequirements>>,
    }

    cfn_internal__inherit_codec_impls!(ConnectionInput);

    /// The [`AWS::Glue::Connection.PhysicalConnectionRequirements`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-connection-physicalconnectionrequirements.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PhysicalConnectionRequirements {
        /// Property `AvailabilityZone`.
        #[serde(rename = "AvailabilityZone")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub availability_zone: Option<::Value<String>>,
        /// Property `SecurityGroupIdList`.
        #[serde(rename = "SecurityGroupIdList")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub security_group_id_list: Option<::ValueList<String>>,
        /// Property `SubnetId`.
        #[serde(rename = "SubnetId")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub subnet_id: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(PhysicalConnectionRequirements);
}

pub mod crawler {
    //! Property types for the `Crawler` resource.

    /// The [`AWS::Glue::Crawler.JdbcTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-jdbctarget.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct JdbcTarget {
        /// Property `ConnectionName`.
        #[serde(rename = "ConnectionName")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub connection_name: Option<::Value<String>>,
        /// Property `Exclusions`.
        #[serde(rename = "Exclusions")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub exclusions: Option<::ValueList<String>>,
        /// Property `Path`.
        #[serde(rename = "Path")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub path: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(JdbcTarget);

    /// The [`AWS::Glue::Crawler.S3Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-s3target.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct S3Target {
        /// Property `Exclusions`.
        #[serde(rename = "Exclusions")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub exclusions: Option<::ValueList<String>>,
        /// Property `Path`.
        #[serde(rename = "Path")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub path: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(S3Target);

    /// The [`AWS::Glue::Crawler.Schedule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-schedule.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Schedule {
        /// Property `ScheduleExpression`.
        #[serde(rename = "ScheduleExpression")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub schedule_expression: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(Schedule);

    /// The [`AWS::Glue::Crawler.SchemaChangePolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-schemachangepolicy.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SchemaChangePolicy {
        /// Property `DeleteBehavior`.
        #[serde(rename = "DeleteBehavior")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub delete_behavior: Option<::Value<String>>,
        /// Property `UpdateBehavior`.
        #[serde(rename = "UpdateBehavior")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub update_behavior: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(SchemaChangePolicy);

    /// The [`AWS::Glue::Crawler.Targets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-crawler-targets.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Targets {
        /// Property `JdbcTargets`.
        #[serde(rename = "JdbcTargets")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub jdbc_targets: Option<::ValueList<JdbcTarget>>,
        /// Property `S3Targets`.
        #[serde(rename = "S3Targets")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub s3_targets: Option<::ValueList<S3Target>>,
    }

    cfn_internal__inherit_codec_impls!(Targets);
}

pub mod database {
    //! Property types for the `Database` resource.

    /// The [`AWS::Glue::Database.DatabaseInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-database-databaseinput.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DatabaseInput {
        /// Property `Description`.
        #[serde(rename = "Description")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<::Value<String>>,
        /// Property `LocationUri`.
        #[serde(rename = "LocationUri")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub location_uri: Option<::Value<String>>,
        /// Property `Name`.
        #[serde(rename = "Name")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<::Value<String>>,
        /// Property `Parameters`.
        #[serde(rename = "Parameters")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub parameters: Option<::Value<::json::Value>>,
    }

    cfn_internal__inherit_codec_impls!(DatabaseInput);
}

pub mod job {
    //! Property types for the `Job` resource.

    /// The [`AWS::Glue::Job.ConnectionsList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-job-connectionslist.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ConnectionsList {
        /// Property `Connections`.
        #[serde(rename = "Connections")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub connections: Option<::ValueList<String>>,
    }

    cfn_internal__inherit_codec_impls!(ConnectionsList);

    /// The [`AWS::Glue::Job.ExecutionProperty`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-job-executionproperty.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ExecutionProperty {
        /// Property `MaxConcurrentRuns`.
        #[serde(rename = "MaxConcurrentRuns")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub max_concurrent_runs: Option<::Value<f64>>,
    }

    cfn_internal__inherit_codec_impls!(ExecutionProperty);

    /// The [`AWS::Glue::Job.JobCommand`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-job-jobcommand.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct JobCommand {
        /// Property `Name`.
        #[serde(rename = "Name")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<::Value<String>>,
        /// Property `ScriptLocation`.
        #[serde(rename = "ScriptLocation")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub script_location: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(JobCommand);
}

pub mod partition {
    //! Property types for the `Partition` resource.

    /// The [`AWS::Glue::Partition.Column`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-column.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Column {
        /// Property `Comment`.
        #[serde(rename = "Comment")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub comment: Option<::Value<String>>,
        /// Property `Name`.
        #[serde(rename = "Name")]
        pub name: ::Value<String>,
        /// Property `Type`.
        #[serde(rename = "Type")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(Column);

    /// The [`AWS::Glue::Partition.Order`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-order.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Order {
        /// Property `Column`.
        #[serde(rename = "Column")]
        pub column: ::Value<String>,
        /// Property `SortOrder`.
        #[serde(rename = "SortOrder")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sort_order: Option<::Value<u32>>,
    }

    cfn_internal__inherit_codec_impls!(Order);

    /// The [`AWS::Glue::Partition.PartitionInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-partitioninput.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PartitionInput {
        /// Property `Parameters`.
        #[serde(rename = "Parameters")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub parameters: Option<::Value<::json::Value>>,
        /// Property `StorageDescriptor`.
        #[serde(rename = "StorageDescriptor")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub storage_descriptor: Option<::Value<StorageDescriptor>>,
        /// Property `Values`.
        #[serde(rename = "Values")]
        pub values: ::ValueList<String>,
    }

    cfn_internal__inherit_codec_impls!(PartitionInput);

    /// The [`AWS::Glue::Partition.SerdeInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-serdeinfo.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SerdeInfo {
        /// Property `Name`.
        #[serde(rename = "Name")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<::Value<String>>,
        /// Property `Parameters`.
        #[serde(rename = "Parameters")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub parameters: Option<::Value<::json::Value>>,
        /// Property `SerializationLibrary`.
        #[serde(rename = "SerializationLibrary")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub serialization_library: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(SerdeInfo);

    /// The [`AWS::Glue::Partition.SkewedInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-skewedinfo.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SkewedInfo {
        /// Property `SkewedColumnNames`.
        #[serde(rename = "SkewedColumnNames")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub skewed_column_names: Option<::ValueList<String>>,
        /// Property `SkewedColumnValueLocationMaps`.
        #[serde(rename = "SkewedColumnValueLocationMaps")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub skewed_column_value_location_maps: Option<::Value<::json::Value>>,
        /// Property `SkewedColumnValues`.
        #[serde(rename = "SkewedColumnValues")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub skewed_column_values: Option<::ValueList<String>>,
    }

    cfn_internal__inherit_codec_impls!(SkewedInfo);

    /// The [`AWS::Glue::Partition.StorageDescriptor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-partition-storagedescriptor.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StorageDescriptor {
        /// Property `BucketColumns`.
        #[serde(rename = "BucketColumns")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub bucket_columns: Option<::ValueList<String>>,
        /// Property `Columns`.
        #[serde(rename = "Columns")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub columns: Option<::ValueList<Column>>,
        /// Property `Compressed`.
        #[serde(rename = "Compressed")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub compressed: Option<::Value<bool>>,
        /// Property `InputFormat`.
        #[serde(rename = "InputFormat")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub input_format: Option<::Value<String>>,
        /// Property `Location`.
        #[serde(rename = "Location")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub location: Option<::Value<String>>,
        /// Property `NumberOfBuckets`.
        #[serde(rename = "NumberOfBuckets")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub number_of_buckets: Option<::Value<u32>>,
        /// Property `OutputFormat`.
        #[serde(rename = "OutputFormat")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub output_format: Option<::Value<String>>,
        /// Property `Parameters`.
        #[serde(rename = "Parameters")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub parameters: Option<::Value<::json::Value>>,
        /// Property `SerdeInfo`.
        #[serde(rename = "SerdeInfo")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub serde_info: Option<::Value<SerdeInfo>>,
        /// Property `SkewedInfo`.
        #[serde(rename = "SkewedInfo")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub skewed_info: Option<::Value<SkewedInfo>>,
        /// Property `SortColumns`.
        #[serde(rename = "SortColumns")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sort_columns: Option<::ValueList<Order>>,
        /// Property `StoredAsSubDirectories`.
        #[serde(rename = "StoredAsSubDirectories")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub stored_as_sub_directories: Option<::Value<bool>>,
    }

    cfn_internal__inherit_codec_impls!(StorageDescriptor);
}

pub mod table {
    //! Property types for the `Table` resource.

    /// The [`AWS::Glue::Table.Column`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-column.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Column {
        /// Property `Comment`.
        #[serde(rename = "Comment")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub comment: Option<::Value<String>>,
        /// Property `Name`.
        #[serde(rename = "Name")]
        pub name: ::Value<String>,
        /// Property `Type`.
        #[serde(rename = "Type")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(Column);

    /// The [`AWS::Glue::Table.Order`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-order.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Order {
        /// Property `Column`.
        #[serde(rename = "Column")]
        pub column: ::Value<String>,
        /// Property `SortOrder`.
        #[serde(rename = "SortOrder")]
        pub sort_order: ::Value<u32>,
    }

    cfn_internal__inherit_codec_impls!(Order);

    /// The [`AWS::Glue::Table.SerdeInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-serdeinfo.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SerdeInfo {
        /// Property `Name`.
        #[serde(rename = "Name")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<::Value<String>>,
        /// Property `Parameters`.
        #[serde(rename = "Parameters")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub parameters: Option<::Value<::json::Value>>,
        /// Property `SerializationLibrary`.
        #[serde(rename = "SerializationLibrary")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub serialization_library: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(SerdeInfo);

    /// The [`AWS::Glue::Table.SkewedInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-skewedinfo.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SkewedInfo {
        /// Property `SkewedColumnNames`.
        #[serde(rename = "SkewedColumnNames")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub skewed_column_names: Option<::ValueList<String>>,
        /// Property `SkewedColumnValueLocationMaps`.
        #[serde(rename = "SkewedColumnValueLocationMaps")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub skewed_column_value_location_maps: Option<::Value<::json::Value>>,
        /// Property `SkewedColumnValues`.
        #[serde(rename = "SkewedColumnValues")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub skewed_column_values: Option<::ValueList<String>>,
    }

    cfn_internal__inherit_codec_impls!(SkewedInfo);

    /// The [`AWS::Glue::Table.StorageDescriptor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-storagedescriptor.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StorageDescriptor {
        /// Property `BucketColumns`.
        #[serde(rename = "BucketColumns")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub bucket_columns: Option<::ValueList<String>>,
        /// Property `Columns`.
        #[serde(rename = "Columns")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub columns: Option<::ValueList<Column>>,
        /// Property `Compressed`.
        #[serde(rename = "Compressed")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub compressed: Option<::Value<bool>>,
        /// Property `InputFormat`.
        #[serde(rename = "InputFormat")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub input_format: Option<::Value<String>>,
        /// Property `Location`.
        #[serde(rename = "Location")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub location: Option<::Value<String>>,
        /// Property `NumberOfBuckets`.
        #[serde(rename = "NumberOfBuckets")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub number_of_buckets: Option<::Value<u32>>,
        /// Property `OutputFormat`.
        #[serde(rename = "OutputFormat")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub output_format: Option<::Value<String>>,
        /// Property `Parameters`.
        #[serde(rename = "Parameters")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub parameters: Option<::Value<::json::Value>>,
        /// Property `SerdeInfo`.
        #[serde(rename = "SerdeInfo")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub serde_info: Option<::Value<SerdeInfo>>,
        /// Property `SkewedInfo`.
        #[serde(rename = "SkewedInfo")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub skewed_info: Option<::Value<SkewedInfo>>,
        /// Property `SortColumns`.
        #[serde(rename = "SortColumns")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sort_columns: Option<::ValueList<Order>>,
        /// Property `StoredAsSubDirectories`.
        #[serde(rename = "StoredAsSubDirectories")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub stored_as_sub_directories: Option<::Value<bool>>,
    }

    cfn_internal__inherit_codec_impls!(StorageDescriptor);

    /// The [`AWS::Glue::Table.TableInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-table-tableinput.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TableInput {
        /// Property `Description`.
        #[serde(rename = "Description")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<::Value<String>>,
        /// Property `Name`.
        #[serde(rename = "Name")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<::Value<String>>,
        /// Property `Owner`.
        #[serde(rename = "Owner")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub owner: Option<::Value<String>>,
        /// Property `Parameters`.
        #[serde(rename = "Parameters")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub parameters: Option<::Value<::json::Value>>,
        /// Property `PartitionKeys`.
        #[serde(rename = "PartitionKeys")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub partition_keys: Option<::ValueList<Column>>,
        /// Property `Retention`.
        #[serde(rename = "Retention")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub retention: Option<::Value<u32>>,
        /// Property `StorageDescriptor`.
        #[serde(rename = "StorageDescriptor")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub storage_descriptor: Option<::Value<StorageDescriptor>>,
        /// Property `TableType`.
        #[serde(rename = "TableType")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub table_type: Option<::Value<String>>,
        /// Property `ViewExpandedText`.
        #[serde(rename = "ViewExpandedText")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub view_expanded_text: Option<::Value<String>>,
        /// Property `ViewOriginalText`.
        #[serde(rename = "ViewOriginalText")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub view_original_text: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(TableInput);
}

pub mod trigger {
    //! Property types for the `Trigger` resource.

    /// The [`AWS::Glue::Trigger.Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-action.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Action {
        /// Property `Arguments`.
        #[serde(rename = "Arguments")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub arguments: Option<::Value<::json::Value>>,
        /// Property `JobName`.
        #[serde(rename = "JobName")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub job_name: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(Action);

    /// The [`AWS::Glue::Trigger.Condition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-condition.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Condition {
        /// Property `JobName`.
        #[serde(rename = "JobName")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub job_name: Option<::Value<String>>,
        /// Property `LogicalOperator`.
        #[serde(rename = "LogicalOperator")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub logical_operator: Option<::Value<String>>,
        /// Property `State`.
        #[serde(rename = "State")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub state: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(Condition);

    /// The [`AWS::Glue::Trigger.Predicate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-glue-trigger-predicate.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Predicate {
        /// Property `Conditions`.
        #[serde(rename = "Conditions")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub conditions: Option<::ValueList<Condition>>,
        /// Property `Logical`.
        #[serde(rename = "Logical")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub logical: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(Predicate);
}
