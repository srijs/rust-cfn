/// The [`AWS::Glue::Classifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-classifier.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Classifier {
    properties: ClassifierProperties
}

/// Properties for the `Classifier` resource.
#[derive(Serialize, Deserialize)]
pub struct ClassifierProperties {
    #[serde(rename="GrokClassifier")]
    pub grok_classifier: (),
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

impl From<ClassifierProperties> for Classifier {
    fn from(properties: ClassifierProperties) -> Classifier {
        Classifier { properties }
    }
}

/// The [`AWS::Glue::Connection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-connection.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Connection {
    properties: ConnectionProperties
}

/// Properties for the `Connection` resource.
#[derive(Serialize, Deserialize)]
pub struct ConnectionProperties {
    #[serde(rename="CatalogId")]
    pub catalog_id: String,
    #[serde(rename="ConnectionInput")]
    pub connection_input: (),
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

impl From<ConnectionProperties> for Connection {
    fn from(properties: ConnectionProperties) -> Connection {
        Connection { properties }
    }
}

/// The [`AWS::Glue::Crawler`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-crawler.html) resource.
#[derive(Serialize, Deserialize)]
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
    pub schedule: (),
    #[serde(rename="SchemaChangePolicy")]
    pub schema_change_policy: (),
    #[serde(rename="TablePrefix")]
    pub table_prefix: String,
    #[serde(rename="Targets")]
    pub targets: (),
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

impl From<CrawlerProperties> for Crawler {
    fn from(properties: CrawlerProperties) -> Crawler {
        Crawler { properties }
    }
}

/// The [`AWS::Glue::Database`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-database.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Database {
    properties: DatabaseProperties
}

/// Properties for the `Database` resource.
#[derive(Serialize, Deserialize)]
pub struct DatabaseProperties {
    #[serde(rename="CatalogId")]
    pub catalog_id: String,
    #[serde(rename="DatabaseInput")]
    pub database_input: (),
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

impl From<DatabaseProperties> for Database {
    fn from(properties: DatabaseProperties) -> Database {
        Database { properties }
    }
}

/// The [`AWS::Glue::DevEndpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-devendpoint.html) resource.
#[derive(Serialize, Deserialize)]
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

impl From<DevEndpointProperties> for DevEndpoint {
    fn from(properties: DevEndpointProperties) -> DevEndpoint {
        DevEndpoint { properties }
    }
}

/// The [`AWS::Glue::Job`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-job.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Job {
    properties: JobProperties
}

/// Properties for the `Job` resource.
#[derive(Serialize, Deserialize)]
pub struct JobProperties {
    #[serde(rename="AllocatedCapacity")]
    pub allocated_capacity: f64,
    #[serde(rename="Command")]
    pub command: (),
    #[serde(rename="Connections")]
    pub connections: (),
    #[serde(rename="DefaultArguments")]
    pub default_arguments: String,
    #[serde(rename="Description")]
    pub description: String,
    #[serde(rename="ExecutionProperty")]
    pub execution_property: (),
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

impl From<JobProperties> for Job {
    fn from(properties: JobProperties) -> Job {
        Job { properties }
    }
}

/// The [`AWS::Glue::Partition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-partition.html) resource.
#[derive(Serialize, Deserialize)]
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
    pub partition_input: (),
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

impl From<PartitionProperties> for Partition {
    fn from(properties: PartitionProperties) -> Partition {
        Partition { properties }
    }
}

/// The [`AWS::Glue::Table`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-table.html) resource.
#[derive(Serialize, Deserialize)]
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
    pub table_input: (),
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

impl From<TableProperties> for Table {
    fn from(properties: TableProperties) -> Table {
        Table { properties }
    }
}

/// The [`AWS::Glue::Trigger`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-glue-trigger.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Trigger {
    properties: TriggerProperties
}

/// Properties for the `Trigger` resource.
#[derive(Serialize, Deserialize)]
pub struct TriggerProperties {
    #[serde(rename="Actions")]
    pub actions: Vec<()>,
    #[serde(rename="Description")]
    pub description: String,
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="Predicate")]
    pub predicate: (),
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

impl From<TriggerProperties> for Trigger {
    fn from(properties: TriggerProperties) -> Trigger {
        Trigger { properties }
    }
}

