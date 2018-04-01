//! Types for the `DynamoDB` service.

/// The [`AWS::DynamoDB::Table`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dynamodb-table.html) resource type.
#[derive(Debug)]
pub struct Table {
    properties: TableProperties
}

/// Properties for the `Table` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct TableProperties {
    /// Property `AttributeDefinitions`.
    #[serde(rename = "AttributeDefinitions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attribute_definitions: Option<::ValueList<self::table::AttributeDefinition>>,
    /// Property `GlobalSecondaryIndexes`.
    #[serde(rename = "GlobalSecondaryIndexes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub global_secondary_indexes: Option<::ValueList<self::table::GlobalSecondaryIndex>>,
    /// Property `KeySchema`.
    #[serde(rename = "KeySchema")]
    pub key_schema: ::ValueList<self::table::KeySchema>,
    /// Property `LocalSecondaryIndexes`.
    #[serde(rename = "LocalSecondaryIndexes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_secondary_indexes: Option<::ValueList<self::table::LocalSecondaryIndex>>,
    /// Property `ProvisionedThroughput`.
    #[serde(rename = "ProvisionedThroughput")]
    pub provisioned_throughput: ::Value<self::table::ProvisionedThroughput>,
    /// Property `SSESpecification`.
    #[serde(rename = "SSESpecification")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sse_specification: Option<::Value<self::table::SSESpecification>>,
    /// Property `StreamSpecification`.
    #[serde(rename = "StreamSpecification")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream_specification: Option<::Value<self::table::StreamSpecification>>,
    /// Property `TableName`.
    #[serde(rename = "TableName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table_name: Option<::Value<String>>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `TimeToLiveSpecification`.
    #[serde(rename = "TimeToLiveSpecification")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_to_live_specification: Option<::Value<self::table::TimeToLiveSpecification>>,
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

impl ::private::Sealed for Table {}

impl From<TableProperties> for Table {
    fn from(properties: TableProperties) -> Table {
        Table { properties }
    }
}

pub mod table {
    //! Property types for the `Table` resource.

    /// The [`AWS::DynamoDB::Table.AttributeDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-attributedef.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AttributeDefinition {
        /// Property `AttributeName`.
        #[serde(rename = "AttributeName")]
        pub attribute_name: ::Value<String>,
        /// Property `AttributeType`.
        #[serde(rename = "AttributeType")]
        pub attribute_type: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(AttributeDefinition);

    /// The [`AWS::DynamoDB::Table.GlobalSecondaryIndex`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-gsi.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct GlobalSecondaryIndex {
        /// Property `IndexName`.
        #[serde(rename = "IndexName")]
        pub index_name: ::Value<String>,
        /// Property `KeySchema`.
        #[serde(rename = "KeySchema")]
        pub key_schema: ::ValueList<KeySchema>,
        /// Property `Projection`.
        #[serde(rename = "Projection")]
        pub projection: ::Value<Projection>,
        /// Property `ProvisionedThroughput`.
        #[serde(rename = "ProvisionedThroughput")]
        pub provisioned_throughput: ::Value<ProvisionedThroughput>,
    }

    cfn_internal__inherit_codec_impls!(GlobalSecondaryIndex);

    /// The [`AWS::DynamoDB::Table.KeySchema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-keyschema.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct KeySchema {
        /// Property `AttributeName`.
        #[serde(rename = "AttributeName")]
        pub attribute_name: ::Value<String>,
        /// Property `KeyType`.
        #[serde(rename = "KeyType")]
        pub key_type: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(KeySchema);

    /// The [`AWS::DynamoDB::Table.LocalSecondaryIndex`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-lsi.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LocalSecondaryIndex {
        /// Property `IndexName`.
        #[serde(rename = "IndexName")]
        pub index_name: ::Value<String>,
        /// Property `KeySchema`.
        #[serde(rename = "KeySchema")]
        pub key_schema: ::ValueList<KeySchema>,
        /// Property `Projection`.
        #[serde(rename = "Projection")]
        pub projection: ::Value<Projection>,
    }

    cfn_internal__inherit_codec_impls!(LocalSecondaryIndex);

    /// The [`AWS::DynamoDB::Table.Projection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-projectionobject.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Projection {
        /// Property `NonKeyAttributes`.
        #[serde(rename = "NonKeyAttributes")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub non_key_attributes: Option<::ValueList<String>>,
        /// Property `ProjectionType`.
        #[serde(rename = "ProjectionType")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub projection_type: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(Projection);

    /// The [`AWS::DynamoDB::Table.ProvisionedThroughput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-provisionedthroughput.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ProvisionedThroughput {
        /// Property `ReadCapacityUnits`.
        #[serde(rename = "ReadCapacityUnits")]
        pub read_capacity_units: ::Value<u64>,
        /// Property `WriteCapacityUnits`.
        #[serde(rename = "WriteCapacityUnits")]
        pub write_capacity_units: ::Value<u64>,
    }

    cfn_internal__inherit_codec_impls!(ProvisionedThroughput);

    /// The [`AWS::DynamoDB::Table.SSESpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-table-ssespecification.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SSESpecification {
        /// Property `SSEEnabled`.
        #[serde(rename = "SSEEnabled")]
        pub sse_enabled: ::Value<bool>,
    }

    cfn_internal__inherit_codec_impls!(SSESpecification);

    /// The [`AWS::DynamoDB::Table.StreamSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-streamspecification.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StreamSpecification {
        /// Property `StreamViewType`.
        #[serde(rename = "StreamViewType")]
        pub stream_view_type: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(StreamSpecification);

    /// The [`AWS::DynamoDB::Table.TimeToLiveSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-timetolivespecification.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct TimeToLiveSpecification {
        /// Property `AttributeName`.
        #[serde(rename = "AttributeName")]
        pub attribute_name: ::Value<String>,
        /// Property `Enabled`.
        #[serde(rename = "Enabled")]
        pub enabled: ::Value<bool>,
    }

    cfn_internal__inherit_codec_impls!(TimeToLiveSpecification);
}
