/// The [`AWS::DynamoDB::Table`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dynamodb-table.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Table {
    properties: TableProperties
}

/// Properties for the `Table` resource.
#[derive(Serialize, Deserialize)]
pub struct TableProperties {
    #[serde(rename="AttributeDefinitions")]
    pub attribute_definitions: Vec<()>,
    #[serde(rename="GlobalSecondaryIndexes")]
    pub global_secondary_indexes: Vec<()>,
    #[serde(rename="KeySchema")]
    pub key_schema: Vec<()>,
    #[serde(rename="LocalSecondaryIndexes")]
    pub local_secondary_indexes: Vec<()>,
    #[serde(rename="ProvisionedThroughput")]
    pub provisioned_throughput: (),
    #[serde(rename="SSESpecification")]
    pub sse_specification: (),
    #[serde(rename="StreamSpecification")]
    pub stream_specification: (),
    #[serde(rename="TableName")]
    pub table_name: String,
    #[serde(rename="Tags")]
    pub tags: Vec<()>,
    #[serde(rename="TimeToLiveSpecification")]
    pub time_to_live_specification: (),
}

impl<'a> ::Resource<'a> for Table {
    type Properties = TableProperties;
    const TYPE: &'static str = "AWS::DynamoDB::Table";
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

