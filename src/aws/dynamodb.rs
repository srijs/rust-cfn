//! Types for the `DynamoDB` service.

/// The [`AWS::DynamoDB::GlobalTable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dynamodb-globaltable.html) resource type.
#[derive(Debug, Default)]
pub struct GlobalTable {
    properties: GlobalTableProperties
}

/// Properties for the `GlobalTable` resource.
#[derive(Debug, Default)]
pub struct GlobalTableProperties {
    /// Property [`AttributeDefinitions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dynamodb-globaltable.html#cfn-dynamodb-globaltable-attributedefinitions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub attribute_definitions: ::ValueList<self::global_table::AttributeDefinition>,
    /// Property [`BillingMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dynamodb-globaltable.html#cfn-dynamodb-globaltable-billingmode).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub billing_mode: Option<::Value<String>>,
    /// Property [`GlobalSecondaryIndexes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dynamodb-globaltable.html#cfn-dynamodb-globaltable-globalsecondaryindexes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub global_secondary_indexes: Option<::ValueList<self::global_table::GlobalSecondaryIndex>>,
    /// Property [`KeySchema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dynamodb-globaltable.html#cfn-dynamodb-globaltable-keyschema).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub key_schema: ::ValueList<self::global_table::KeySchema>,
    /// Property [`LocalSecondaryIndexes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dynamodb-globaltable.html#cfn-dynamodb-globaltable-localsecondaryindexes).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub local_secondary_indexes: Option<::ValueList<self::global_table::LocalSecondaryIndex>>,
    /// Property [`Replicas`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dynamodb-globaltable.html#cfn-dynamodb-globaltable-replicas).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub replicas: ::ValueList<self::global_table::ReplicaSpecification>,
    /// Property [`SSESpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dynamodb-globaltable.html#cfn-dynamodb-globaltable-ssespecification).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sse_specification: Option<::Value<self::global_table::SSESpecification>>,
    /// Property [`StreamSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dynamodb-globaltable.html#cfn-dynamodb-globaltable-streamspecification).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub stream_specification: Option<::Value<self::global_table::StreamSpecification>>,
    /// Property [`TableName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dynamodb-globaltable.html#cfn-dynamodb-globaltable-tablename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub table_name: Option<::Value<String>>,
    /// Property [`TimeToLiveSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dynamodb-globaltable.html#cfn-dynamodb-globaltable-timetolivespecification).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub time_to_live_specification: Option<::Value<self::global_table::TimeToLiveSpecification>>,
    /// Property [`WriteProvisionedThroughputSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dynamodb-globaltable.html#cfn-dynamodb-globaltable-writeprovisionedthroughputsettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub write_provisioned_throughput_settings: Option<::Value<self::global_table::WriteProvisionedThroughputSettings>>,
}

impl ::serde::Serialize for GlobalTableProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttributeDefinitions", &self.attribute_definitions)?;
        if let Some(ref billing_mode) = self.billing_mode {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BillingMode", billing_mode)?;
        }
        if let Some(ref global_secondary_indexes) = self.global_secondary_indexes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GlobalSecondaryIndexes", global_secondary_indexes)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeySchema", &self.key_schema)?;
        if let Some(ref local_secondary_indexes) = self.local_secondary_indexes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LocalSecondaryIndexes", local_secondary_indexes)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Replicas", &self.replicas)?;
        if let Some(ref sse_specification) = self.sse_specification {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SSESpecification", sse_specification)?;
        }
        if let Some(ref stream_specification) = self.stream_specification {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamSpecification", stream_specification)?;
        }
        if let Some(ref table_name) = self.table_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableName", table_name)?;
        }
        if let Some(ref time_to_live_specification) = self.time_to_live_specification {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeToLiveSpecification", time_to_live_specification)?;
        }
        if let Some(ref write_provisioned_throughput_settings) = self.write_provisioned_throughput_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WriteProvisionedThroughputSettings", write_provisioned_throughput_settings)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for GlobalTableProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<GlobalTableProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = GlobalTableProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type GlobalTableProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut attribute_definitions: Option<::ValueList<self::global_table::AttributeDefinition>> = None;
                let mut billing_mode: Option<::Value<String>> = None;
                let mut global_secondary_indexes: Option<::ValueList<self::global_table::GlobalSecondaryIndex>> = None;
                let mut key_schema: Option<::ValueList<self::global_table::KeySchema>> = None;
                let mut local_secondary_indexes: Option<::ValueList<self::global_table::LocalSecondaryIndex>> = None;
                let mut replicas: Option<::ValueList<self::global_table::ReplicaSpecification>> = None;
                let mut sse_specification: Option<::Value<self::global_table::SSESpecification>> = None;
                let mut stream_specification: Option<::Value<self::global_table::StreamSpecification>> = None;
                let mut table_name: Option<::Value<String>> = None;
                let mut time_to_live_specification: Option<::Value<self::global_table::TimeToLiveSpecification>> = None;
                let mut write_provisioned_throughput_settings: Option<::Value<self::global_table::WriteProvisionedThroughputSettings>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AttributeDefinitions" => {
                            attribute_definitions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BillingMode" => {
                            billing_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GlobalSecondaryIndexes" => {
                            global_secondary_indexes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KeySchema" => {
                            key_schema = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LocalSecondaryIndexes" => {
                            local_secondary_indexes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Replicas" => {
                            replicas = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SSESpecification" => {
                            sse_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StreamSpecification" => {
                            stream_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TableName" => {
                            table_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TimeToLiveSpecification" => {
                            time_to_live_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "WriteProvisionedThroughputSettings" => {
                            write_provisioned_throughput_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(GlobalTableProperties {
                    attribute_definitions: attribute_definitions.ok_or(::serde::de::Error::missing_field("AttributeDefinitions"))?,
                    billing_mode: billing_mode,
                    global_secondary_indexes: global_secondary_indexes,
                    key_schema: key_schema.ok_or(::serde::de::Error::missing_field("KeySchema"))?,
                    local_secondary_indexes: local_secondary_indexes,
                    replicas: replicas.ok_or(::serde::de::Error::missing_field("Replicas"))?,
                    sse_specification: sse_specification,
                    stream_specification: stream_specification,
                    table_name: table_name,
                    time_to_live_specification: time_to_live_specification,
                    write_provisioned_throughput_settings: write_provisioned_throughput_settings,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for GlobalTable {
    type Properties = GlobalTableProperties;
    const TYPE: &'static str = "AWS::DynamoDB::GlobalTable";
    fn properties(&self) -> &GlobalTableProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut GlobalTableProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for GlobalTable {}

impl From<GlobalTableProperties> for GlobalTable {
    fn from(properties: GlobalTableProperties) -> GlobalTable {
        GlobalTable { properties }
    }
}

/// The [`AWS::DynamoDB::Table`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dynamodb-table.html) resource type.
#[derive(Debug, Default)]
pub struct Table {
    properties: TableProperties
}

/// Properties for the `Table` resource.
#[derive(Debug, Default)]
pub struct TableProperties {
    /// Property [`AttributeDefinitions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dynamodb-table.html#cfn-dynamodb-table-attributedef).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub attribute_definitions: Option<::ValueList<self::table::AttributeDefinition>>,
    /// Property [`BillingMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dynamodb-table.html#cfn-dynamodb-table-billingmode).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub billing_mode: Option<::Value<String>>,
    /// Property [`ContributorInsightsSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dynamodb-table.html#cfn-dynamodb-contributorinsightsspecification-enabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub contributor_insights_specification: Option<::Value<self::table::ContributorInsightsSpecification>>,
    /// Property [`GlobalSecondaryIndexes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dynamodb-table.html#cfn-dynamodb-table-gsi).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub global_secondary_indexes: Option<::ValueList<self::table::GlobalSecondaryIndex>>,
    /// Property [`KeySchema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dynamodb-table.html#cfn-dynamodb-table-keyschema).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub key_schema: ::ValueList<self::table::KeySchema>,
    /// Property [`KinesisStreamSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dynamodb-table.html#cfn-dynamodb-table-kinesisstreamspecification).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub kinesis_stream_specification: Option<::Value<self::table::KinesisStreamSpecification>>,
    /// Property [`LocalSecondaryIndexes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dynamodb-table.html#cfn-dynamodb-table-lsi).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub local_secondary_indexes: Option<::ValueList<self::table::LocalSecondaryIndex>>,
    /// Property [`PointInTimeRecoverySpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dynamodb-table.html#cfn-dynamodb-table-pointintimerecoveryspecification).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub point_in_time_recovery_specification: Option<::Value<self::table::PointInTimeRecoverySpecification>>,
    /// Property [`ProvisionedThroughput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dynamodb-table.html#cfn-dynamodb-table-provisionedthroughput).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub provisioned_throughput: Option<::Value<self::table::ProvisionedThroughput>>,
    /// Property [`SSESpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dynamodb-table.html#cfn-dynamodb-table-ssespecification).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sse_specification: Option<::Value<self::table::SSESpecification>>,
    /// Property [`StreamSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dynamodb-table.html#cfn-dynamodb-table-streamspecification).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub stream_specification: Option<::Value<self::table::StreamSpecification>>,
    /// Property [`TableName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dynamodb-table.html#cfn-dynamodb-table-tablename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub table_name: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dynamodb-table.html#cfn-dynamodb-table-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TimeToLiveSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dynamodb-table.html#cfn-dynamodb-table-timetolivespecification).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub time_to_live_specification: Option<::Value<self::table::TimeToLiveSpecification>>,
}

impl ::serde::Serialize for TableProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref attribute_definitions) = self.attribute_definitions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttributeDefinitions", attribute_definitions)?;
        }
        if let Some(ref billing_mode) = self.billing_mode {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BillingMode", billing_mode)?;
        }
        if let Some(ref contributor_insights_specification) = self.contributor_insights_specification {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContributorInsightsSpecification", contributor_insights_specification)?;
        }
        if let Some(ref global_secondary_indexes) = self.global_secondary_indexes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GlobalSecondaryIndexes", global_secondary_indexes)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeySchema", &self.key_schema)?;
        if let Some(ref kinesis_stream_specification) = self.kinesis_stream_specification {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KinesisStreamSpecification", kinesis_stream_specification)?;
        }
        if let Some(ref local_secondary_indexes) = self.local_secondary_indexes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LocalSecondaryIndexes", local_secondary_indexes)?;
        }
        if let Some(ref point_in_time_recovery_specification) = self.point_in_time_recovery_specification {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PointInTimeRecoverySpecification", point_in_time_recovery_specification)?;
        }
        if let Some(ref provisioned_throughput) = self.provisioned_throughput {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProvisionedThroughput", provisioned_throughput)?;
        }
        if let Some(ref sse_specification) = self.sse_specification {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SSESpecification", sse_specification)?;
        }
        if let Some(ref stream_specification) = self.stream_specification {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamSpecification", stream_specification)?;
        }
        if let Some(ref table_name) = self.table_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableName", table_name)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref time_to_live_specification) = self.time_to_live_specification {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeToLiveSpecification", time_to_live_specification)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for TableProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<TableProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TableProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type TableProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut attribute_definitions: Option<::ValueList<self::table::AttributeDefinition>> = None;
                let mut billing_mode: Option<::Value<String>> = None;
                let mut contributor_insights_specification: Option<::Value<self::table::ContributorInsightsSpecification>> = None;
                let mut global_secondary_indexes: Option<::ValueList<self::table::GlobalSecondaryIndex>> = None;
                let mut key_schema: Option<::ValueList<self::table::KeySchema>> = None;
                let mut kinesis_stream_specification: Option<::Value<self::table::KinesisStreamSpecification>> = None;
                let mut local_secondary_indexes: Option<::ValueList<self::table::LocalSecondaryIndex>> = None;
                let mut point_in_time_recovery_specification: Option<::Value<self::table::PointInTimeRecoverySpecification>> = None;
                let mut provisioned_throughput: Option<::Value<self::table::ProvisionedThroughput>> = None;
                let mut sse_specification: Option<::Value<self::table::SSESpecification>> = None;
                let mut stream_specification: Option<::Value<self::table::StreamSpecification>> = None;
                let mut table_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut time_to_live_specification: Option<::Value<self::table::TimeToLiveSpecification>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AttributeDefinitions" => {
                            attribute_definitions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BillingMode" => {
                            billing_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ContributorInsightsSpecification" => {
                            contributor_insights_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GlobalSecondaryIndexes" => {
                            global_secondary_indexes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KeySchema" => {
                            key_schema = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KinesisStreamSpecification" => {
                            kinesis_stream_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LocalSecondaryIndexes" => {
                            local_secondary_indexes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PointInTimeRecoverySpecification" => {
                            point_in_time_recovery_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProvisionedThroughput" => {
                            provisioned_throughput = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SSESpecification" => {
                            sse_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StreamSpecification" => {
                            stream_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TableName" => {
                            table_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TimeToLiveSpecification" => {
                            time_to_live_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(TableProperties {
                    attribute_definitions: attribute_definitions,
                    billing_mode: billing_mode,
                    contributor_insights_specification: contributor_insights_specification,
                    global_secondary_indexes: global_secondary_indexes,
                    key_schema: key_schema.ok_or(::serde::de::Error::missing_field("KeySchema"))?,
                    kinesis_stream_specification: kinesis_stream_specification,
                    local_secondary_indexes: local_secondary_indexes,
                    point_in_time_recovery_specification: point_in_time_recovery_specification,
                    provisioned_throughput: provisioned_throughput,
                    sse_specification: sse_specification,
                    stream_specification: stream_specification,
                    table_name: table_name,
                    tags: tags,
                    time_to_live_specification: time_to_live_specification,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Table {
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

pub mod global_table {
    //! Property types for the `GlobalTable` resource.

    /// The [`AWS::DynamoDB::GlobalTable.AttributeDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-attributedefinition.html) property type.
    #[derive(Debug, Default)]
    pub struct AttributeDefinition {
        /// Property [`AttributeName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-attributedefinition.html#cfn-dynamodb-globaltable-attributedefinition-attributename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attribute_name: ::Value<String>,
        /// Property [`AttributeType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-attributedefinition.html#cfn-dynamodb-globaltable-attributedefinition-attributetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attribute_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for AttributeDefinition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttributeName", &self.attribute_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttributeType", &self.attribute_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AttributeDefinition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AttributeDefinition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AttributeDefinition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AttributeDefinition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attribute_name: Option<::Value<String>> = None;
                    let mut attribute_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AttributeName" => {
                                attribute_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AttributeType" => {
                                attribute_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AttributeDefinition {
                        attribute_name: attribute_name.ok_or(::serde::de::Error::missing_field("AttributeName"))?,
                        attribute_type: attribute_type.ok_or(::serde::de::Error::missing_field("AttributeType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DynamoDB::GlobalTable.CapacityAutoScalingSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-capacityautoscalingsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct CapacityAutoScalingSettings {
        /// Property [`MaxCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-capacityautoscalingsettings.html#cfn-dynamodb-globaltable-capacityautoscalingsettings-maxcapacity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_capacity: ::Value<u32>,
        /// Property [`MinCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-capacityautoscalingsettings.html#cfn-dynamodb-globaltable-capacityautoscalingsettings-mincapacity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min_capacity: ::Value<u32>,
        /// Property [`SeedCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-capacityautoscalingsettings.html#cfn-dynamodb-globaltable-capacityautoscalingsettings-seedcapacity).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub seed_capacity: Option<::Value<u32>>,
        /// Property [`TargetTrackingScalingPolicyConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-capacityautoscalingsettings.html#cfn-dynamodb-globaltable-capacityautoscalingsettings-targettrackingscalingpolicyconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_tracking_scaling_policy_configuration: ::Value<TargetTrackingScalingPolicyConfiguration>,
    }

    impl ::codec::SerializeValue for CapacityAutoScalingSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxCapacity", &self.max_capacity)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinCapacity", &self.min_capacity)?;
            if let Some(ref seed_capacity) = self.seed_capacity {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SeedCapacity", seed_capacity)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetTrackingScalingPolicyConfiguration", &self.target_tracking_scaling_policy_configuration)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CapacityAutoScalingSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CapacityAutoScalingSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CapacityAutoScalingSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CapacityAutoScalingSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut max_capacity: Option<::Value<u32>> = None;
                    let mut min_capacity: Option<::Value<u32>> = None;
                    let mut seed_capacity: Option<::Value<u32>> = None;
                    let mut target_tracking_scaling_policy_configuration: Option<::Value<TargetTrackingScalingPolicyConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxCapacity" => {
                                max_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinCapacity" => {
                                min_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SeedCapacity" => {
                                seed_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetTrackingScalingPolicyConfiguration" => {
                                target_tracking_scaling_policy_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CapacityAutoScalingSettings {
                        max_capacity: max_capacity.ok_or(::serde::de::Error::missing_field("MaxCapacity"))?,
                        min_capacity: min_capacity.ok_or(::serde::de::Error::missing_field("MinCapacity"))?,
                        seed_capacity: seed_capacity,
                        target_tracking_scaling_policy_configuration: target_tracking_scaling_policy_configuration.ok_or(::serde::de::Error::missing_field("TargetTrackingScalingPolicyConfiguration"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DynamoDB::GlobalTable.ContributorInsightsSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-contributorinsightsspecification.html) property type.
    #[derive(Debug, Default)]
    pub struct ContributorInsightsSpecification {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-contributorinsightsspecification.html#cfn-dynamodb-globaltable-contributorinsightsspecification-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: ::Value<bool>,
    }

    impl ::codec::SerializeValue for ContributorInsightsSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ContributorInsightsSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ContributorInsightsSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ContributorInsightsSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ContributorInsightsSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ContributorInsightsSpecification {
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DynamoDB::GlobalTable.GlobalSecondaryIndex`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-globalsecondaryindex.html) property type.
    #[derive(Debug, Default)]
    pub struct GlobalSecondaryIndex {
        /// Property [`IndexName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-globalsecondaryindex.html#cfn-dynamodb-globaltable-globalsecondaryindex-indexname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub index_name: ::Value<String>,
        /// Property [`KeySchema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-globalsecondaryindex.html#cfn-dynamodb-globaltable-globalsecondaryindex-keyschema).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key_schema: ::ValueList<KeySchema>,
        /// Property [`Projection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-globalsecondaryindex.html#cfn-dynamodb-globaltable-globalsecondaryindex-projection).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub projection: ::Value<Projection>,
        /// Property [`WriteProvisionedThroughputSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-globalsecondaryindex.html#cfn-dynamodb-globaltable-globalsecondaryindex-writeprovisionedthroughputsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub write_provisioned_throughput_settings: Option<::Value<WriteProvisionedThroughputSettings>>,
    }

    impl ::codec::SerializeValue for GlobalSecondaryIndex {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IndexName", &self.index_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeySchema", &self.key_schema)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Projection", &self.projection)?;
            if let Some(ref write_provisioned_throughput_settings) = self.write_provisioned_throughput_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WriteProvisionedThroughputSettings", write_provisioned_throughput_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GlobalSecondaryIndex {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GlobalSecondaryIndex, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GlobalSecondaryIndex;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GlobalSecondaryIndex")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut index_name: Option<::Value<String>> = None;
                    let mut key_schema: Option<::ValueList<KeySchema>> = None;
                    let mut projection: Option<::Value<Projection>> = None;
                    let mut write_provisioned_throughput_settings: Option<::Value<WriteProvisionedThroughputSettings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "IndexName" => {
                                index_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeySchema" => {
                                key_schema = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Projection" => {
                                projection = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WriteProvisionedThroughputSettings" => {
                                write_provisioned_throughput_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GlobalSecondaryIndex {
                        index_name: index_name.ok_or(::serde::de::Error::missing_field("IndexName"))?,
                        key_schema: key_schema.ok_or(::serde::de::Error::missing_field("KeySchema"))?,
                        projection: projection.ok_or(::serde::de::Error::missing_field("Projection"))?,
                        write_provisioned_throughput_settings: write_provisioned_throughput_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DynamoDB::GlobalTable.KeySchema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-keyschema.html) property type.
    #[derive(Debug, Default)]
    pub struct KeySchema {
        /// Property [`AttributeName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-keyschema.html#cfn-dynamodb-globaltable-keyschema-attributename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attribute_name: ::Value<String>,
        /// Property [`KeyType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-keyschema.html#cfn-dynamodb-globaltable-keyschema-keytype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for KeySchema {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttributeName", &self.attribute_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyType", &self.key_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KeySchema {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KeySchema, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KeySchema;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KeySchema")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attribute_name: Option<::Value<String>> = None;
                    let mut key_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AttributeName" => {
                                attribute_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeyType" => {
                                key_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KeySchema {
                        attribute_name: attribute_name.ok_or(::serde::de::Error::missing_field("AttributeName"))?,
                        key_type: key_type.ok_or(::serde::de::Error::missing_field("KeyType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DynamoDB::GlobalTable.LocalSecondaryIndex`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-localsecondaryindex.html) property type.
    #[derive(Debug, Default)]
    pub struct LocalSecondaryIndex {
        /// Property [`IndexName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-localsecondaryindex.html#cfn-dynamodb-globaltable-localsecondaryindex-indexname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub index_name: ::Value<String>,
        /// Property [`KeySchema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-localsecondaryindex.html#cfn-dynamodb-globaltable-localsecondaryindex-keyschema).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub key_schema: ::ValueList<KeySchema>,
        /// Property [`Projection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-localsecondaryindex.html#cfn-dynamodb-globaltable-localsecondaryindex-projection).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub projection: ::Value<Projection>,
    }

    impl ::codec::SerializeValue for LocalSecondaryIndex {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IndexName", &self.index_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeySchema", &self.key_schema)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Projection", &self.projection)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LocalSecondaryIndex {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LocalSecondaryIndex, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LocalSecondaryIndex;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LocalSecondaryIndex")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut index_name: Option<::Value<String>> = None;
                    let mut key_schema: Option<::ValueList<KeySchema>> = None;
                    let mut projection: Option<::Value<Projection>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "IndexName" => {
                                index_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeySchema" => {
                                key_schema = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Projection" => {
                                projection = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LocalSecondaryIndex {
                        index_name: index_name.ok_or(::serde::de::Error::missing_field("IndexName"))?,
                        key_schema: key_schema.ok_or(::serde::de::Error::missing_field("KeySchema"))?,
                        projection: projection.ok_or(::serde::de::Error::missing_field("Projection"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DynamoDB::GlobalTable.PointInTimeRecoverySpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-pointintimerecoveryspecification.html) property type.
    #[derive(Debug, Default)]
    pub struct PointInTimeRecoverySpecification {
        /// Property [`PointInTimeRecoveryEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-pointintimerecoveryspecification.html#cfn-dynamodb-globaltable-pointintimerecoveryspecification-pointintimerecoveryenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub point_in_time_recovery_enabled: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for PointInTimeRecoverySpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref point_in_time_recovery_enabled) = self.point_in_time_recovery_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PointInTimeRecoveryEnabled", point_in_time_recovery_enabled)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PointInTimeRecoverySpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PointInTimeRecoverySpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PointInTimeRecoverySpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PointInTimeRecoverySpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut point_in_time_recovery_enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PointInTimeRecoveryEnabled" => {
                                point_in_time_recovery_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PointInTimeRecoverySpecification {
                        point_in_time_recovery_enabled: point_in_time_recovery_enabled,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DynamoDB::GlobalTable.Projection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-projection.html) property type.
    #[derive(Debug, Default)]
    pub struct Projection {
        /// Property [`NonKeyAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-projection.html#cfn-dynamodb-globaltable-projection-nonkeyattributes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub non_key_attributes: Option<::ValueList<String>>,
        /// Property [`ProjectionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-projection.html#cfn-dynamodb-globaltable-projection-projectiontype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub projection_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Projection {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref non_key_attributes) = self.non_key_attributes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NonKeyAttributes", non_key_attributes)?;
            }
            if let Some(ref projection_type) = self.projection_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProjectionType", projection_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Projection {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Projection, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Projection;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Projection")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut non_key_attributes: Option<::ValueList<String>> = None;
                    let mut projection_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "NonKeyAttributes" => {
                                non_key_attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProjectionType" => {
                                projection_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Projection {
                        non_key_attributes: non_key_attributes,
                        projection_type: projection_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DynamoDB::GlobalTable.ReadProvisionedThroughputSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-readprovisionedthroughputsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct ReadProvisionedThroughputSettings {
        /// Property [`ReadCapacityAutoScalingSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-readprovisionedthroughputsettings.html#cfn-dynamodb-globaltable-readprovisionedthroughputsettings-readcapacityautoscalingsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub read_capacity_auto_scaling_settings: Option<::Value<CapacityAutoScalingSettings>>,
        /// Property [`ReadCapacityUnits`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-readprovisionedthroughputsettings.html#cfn-dynamodb-globaltable-readprovisionedthroughputsettings-readcapacityunits).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub read_capacity_units: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for ReadProvisionedThroughputSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref read_capacity_auto_scaling_settings) = self.read_capacity_auto_scaling_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReadCapacityAutoScalingSettings", read_capacity_auto_scaling_settings)?;
            }
            if let Some(ref read_capacity_units) = self.read_capacity_units {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReadCapacityUnits", read_capacity_units)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ReadProvisionedThroughputSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ReadProvisionedThroughputSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ReadProvisionedThroughputSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ReadProvisionedThroughputSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut read_capacity_auto_scaling_settings: Option<::Value<CapacityAutoScalingSettings>> = None;
                    let mut read_capacity_units: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ReadCapacityAutoScalingSettings" => {
                                read_capacity_auto_scaling_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReadCapacityUnits" => {
                                read_capacity_units = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ReadProvisionedThroughputSettings {
                        read_capacity_auto_scaling_settings: read_capacity_auto_scaling_settings,
                        read_capacity_units: read_capacity_units,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DynamoDB::GlobalTable.ReplicaGlobalSecondaryIndexSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-replicaglobalsecondaryindexspecification.html) property type.
    #[derive(Debug, Default)]
    pub struct ReplicaGlobalSecondaryIndexSpecification {
        /// Property [`ContributorInsightsSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-replicaglobalsecondaryindexspecification.html#cfn-dynamodb-globaltable-replicaglobalsecondaryindexspecification-contributorinsightsspecification).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub contributor_insights_specification: Option<::Value<ContributorInsightsSpecification>>,
        /// Property [`IndexName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-replicaglobalsecondaryindexspecification.html#cfn-dynamodb-globaltable-replicaglobalsecondaryindexspecification-indexname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub index_name: ::Value<String>,
        /// Property [`ReadProvisionedThroughputSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-replicaglobalsecondaryindexspecification.html#cfn-dynamodb-globaltable-replicaglobalsecondaryindexspecification-readprovisionedthroughputsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub read_provisioned_throughput_settings: Option<::Value<ReadProvisionedThroughputSettings>>,
    }

    impl ::codec::SerializeValue for ReplicaGlobalSecondaryIndexSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref contributor_insights_specification) = self.contributor_insights_specification {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContributorInsightsSpecification", contributor_insights_specification)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IndexName", &self.index_name)?;
            if let Some(ref read_provisioned_throughput_settings) = self.read_provisioned_throughput_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReadProvisionedThroughputSettings", read_provisioned_throughput_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ReplicaGlobalSecondaryIndexSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ReplicaGlobalSecondaryIndexSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ReplicaGlobalSecondaryIndexSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ReplicaGlobalSecondaryIndexSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut contributor_insights_specification: Option<::Value<ContributorInsightsSpecification>> = None;
                    let mut index_name: Option<::Value<String>> = None;
                    let mut read_provisioned_throughput_settings: Option<::Value<ReadProvisionedThroughputSettings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContributorInsightsSpecification" => {
                                contributor_insights_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IndexName" => {
                                index_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReadProvisionedThroughputSettings" => {
                                read_provisioned_throughput_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ReplicaGlobalSecondaryIndexSpecification {
                        contributor_insights_specification: contributor_insights_specification,
                        index_name: index_name.ok_or(::serde::de::Error::missing_field("IndexName"))?,
                        read_provisioned_throughput_settings: read_provisioned_throughput_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DynamoDB::GlobalTable.ReplicaSSESpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-replicassespecification.html) property type.
    #[derive(Debug, Default)]
    pub struct ReplicaSSESpecification {
        /// Property [`KMSMasterKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-replicassespecification.html#cfn-dynamodb-globaltable-replicassespecification-kmsmasterkeyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kms_master_key_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for ReplicaSSESpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KMSMasterKeyId", &self.kms_master_key_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ReplicaSSESpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ReplicaSSESpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ReplicaSSESpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ReplicaSSESpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut kms_master_key_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KMSMasterKeyId" => {
                                kms_master_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ReplicaSSESpecification {
                        kms_master_key_id: kms_master_key_id.ok_or(::serde::de::Error::missing_field("KMSMasterKeyId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DynamoDB::GlobalTable.ReplicaSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-replicaspecification.html) property type.
    #[derive(Debug, Default)]
    pub struct ReplicaSpecification {
        /// Property [`ContributorInsightsSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-replicaspecification.html#cfn-dynamodb-globaltable-replicaspecification-contributorinsightsspecification).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub contributor_insights_specification: Option<::Value<ContributorInsightsSpecification>>,
        /// Property [`GlobalSecondaryIndexes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-replicaspecification.html#cfn-dynamodb-globaltable-replicaspecification-globalsecondaryindexes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub global_secondary_indexes: Option<::ValueList<ReplicaGlobalSecondaryIndexSpecification>>,
        /// Property [`PointInTimeRecoverySpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-replicaspecification.html#cfn-dynamodb-globaltable-replicaspecification-pointintimerecoveryspecification).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub point_in_time_recovery_specification: Option<::Value<PointInTimeRecoverySpecification>>,
        /// Property [`ReadProvisionedThroughputSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-replicaspecification.html#cfn-dynamodb-globaltable-replicaspecification-readprovisionedthroughputsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub read_provisioned_throughput_settings: Option<::Value<ReadProvisionedThroughputSettings>>,
        /// Property [`Region`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-replicaspecification.html#cfn-dynamodb-globaltable-replicaspecification-region).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub region: ::Value<String>,
        /// Property [`SSESpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-replicaspecification.html#cfn-dynamodb-globaltable-replicaspecification-ssespecification).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sse_specification: Option<::Value<ReplicaSSESpecification>>,
        /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-replicaspecification.html#cfn-dynamodb-globaltable-replicaspecification-tags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tags: Option<::ValueList<::Tag>>,
    }

    impl ::codec::SerializeValue for ReplicaSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref contributor_insights_specification) = self.contributor_insights_specification {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContributorInsightsSpecification", contributor_insights_specification)?;
            }
            if let Some(ref global_secondary_indexes) = self.global_secondary_indexes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GlobalSecondaryIndexes", global_secondary_indexes)?;
            }
            if let Some(ref point_in_time_recovery_specification) = self.point_in_time_recovery_specification {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PointInTimeRecoverySpecification", point_in_time_recovery_specification)?;
            }
            if let Some(ref read_provisioned_throughput_settings) = self.read_provisioned_throughput_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReadProvisionedThroughputSettings", read_provisioned_throughput_settings)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Region", &self.region)?;
            if let Some(ref sse_specification) = self.sse_specification {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SSESpecification", sse_specification)?;
            }
            if let Some(ref tags) = self.tags {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ReplicaSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ReplicaSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ReplicaSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ReplicaSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut contributor_insights_specification: Option<::Value<ContributorInsightsSpecification>> = None;
                    let mut global_secondary_indexes: Option<::ValueList<ReplicaGlobalSecondaryIndexSpecification>> = None;
                    let mut point_in_time_recovery_specification: Option<::Value<PointInTimeRecoverySpecification>> = None;
                    let mut read_provisioned_throughput_settings: Option<::Value<ReadProvisionedThroughputSettings>> = None;
                    let mut region: Option<::Value<String>> = None;
                    let mut sse_specification: Option<::Value<ReplicaSSESpecification>> = None;
                    let mut tags: Option<::ValueList<::Tag>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContributorInsightsSpecification" => {
                                contributor_insights_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GlobalSecondaryIndexes" => {
                                global_secondary_indexes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PointInTimeRecoverySpecification" => {
                                point_in_time_recovery_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReadProvisionedThroughputSettings" => {
                                read_provisioned_throughput_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Region" => {
                                region = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SSESpecification" => {
                                sse_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Tags" => {
                                tags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ReplicaSpecification {
                        contributor_insights_specification: contributor_insights_specification,
                        global_secondary_indexes: global_secondary_indexes,
                        point_in_time_recovery_specification: point_in_time_recovery_specification,
                        read_provisioned_throughput_settings: read_provisioned_throughput_settings,
                        region: region.ok_or(::serde::de::Error::missing_field("Region"))?,
                        sse_specification: sse_specification,
                        tags: tags,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DynamoDB::GlobalTable.SSESpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-ssespecification.html) property type.
    #[derive(Debug, Default)]
    pub struct SSESpecification {
        /// Property [`SSEEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-ssespecification.html#cfn-dynamodb-globaltable-ssespecification-sseenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sse_enabled: ::Value<bool>,
        /// Property [`SSEType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-ssespecification.html#cfn-dynamodb-globaltable-ssespecification-ssetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sse_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SSESpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SSEEnabled", &self.sse_enabled)?;
            if let Some(ref sse_type) = self.sse_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SSEType", sse_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SSESpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SSESpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SSESpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SSESpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut sse_enabled: Option<::Value<bool>> = None;
                    let mut sse_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SSEEnabled" => {
                                sse_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SSEType" => {
                                sse_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SSESpecification {
                        sse_enabled: sse_enabled.ok_or(::serde::de::Error::missing_field("SSEEnabled"))?,
                        sse_type: sse_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DynamoDB::GlobalTable.StreamSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-streamspecification.html) property type.
    #[derive(Debug, Default)]
    pub struct StreamSpecification {
        /// Property [`StreamViewType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-streamspecification.html#cfn-dynamodb-globaltable-streamspecification-streamviewtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stream_view_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for StreamSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamViewType", &self.stream_view_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StreamSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StreamSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StreamSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StreamSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut stream_view_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "StreamViewType" => {
                                stream_view_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StreamSpecification {
                        stream_view_type: stream_view_type.ok_or(::serde::de::Error::missing_field("StreamViewType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DynamoDB::GlobalTable.TargetTrackingScalingPolicyConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-targettrackingscalingpolicyconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct TargetTrackingScalingPolicyConfiguration {
        /// Property [`DisableScaleIn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-targettrackingscalingpolicyconfiguration.html#cfn-dynamodb-globaltable-targettrackingscalingpolicyconfiguration-disablescalein).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub disable_scale_in: Option<::Value<bool>>,
        /// Property [`ScaleInCooldown`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-targettrackingscalingpolicyconfiguration.html#cfn-dynamodb-globaltable-targettrackingscalingpolicyconfiguration-scaleincooldown).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scale_in_cooldown: Option<::Value<u32>>,
        /// Property [`ScaleOutCooldown`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-targettrackingscalingpolicyconfiguration.html#cfn-dynamodb-globaltable-targettrackingscalingpolicyconfiguration-scaleoutcooldown).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scale_out_cooldown: Option<::Value<u32>>,
        /// Property [`TargetValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-targettrackingscalingpolicyconfiguration.html#cfn-dynamodb-globaltable-targettrackingscalingpolicyconfiguration-targetvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_value: ::Value<f64>,
    }

    impl ::codec::SerializeValue for TargetTrackingScalingPolicyConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref disable_scale_in) = self.disable_scale_in {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisableScaleIn", disable_scale_in)?;
            }
            if let Some(ref scale_in_cooldown) = self.scale_in_cooldown {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScaleInCooldown", scale_in_cooldown)?;
            }
            if let Some(ref scale_out_cooldown) = self.scale_out_cooldown {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScaleOutCooldown", scale_out_cooldown)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetValue", &self.target_value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TargetTrackingScalingPolicyConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TargetTrackingScalingPolicyConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TargetTrackingScalingPolicyConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TargetTrackingScalingPolicyConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut disable_scale_in: Option<::Value<bool>> = None;
                    let mut scale_in_cooldown: Option<::Value<u32>> = None;
                    let mut scale_out_cooldown: Option<::Value<u32>> = None;
                    let mut target_value: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DisableScaleIn" => {
                                disable_scale_in = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScaleInCooldown" => {
                                scale_in_cooldown = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScaleOutCooldown" => {
                                scale_out_cooldown = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetValue" => {
                                target_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TargetTrackingScalingPolicyConfiguration {
                        disable_scale_in: disable_scale_in,
                        scale_in_cooldown: scale_in_cooldown,
                        scale_out_cooldown: scale_out_cooldown,
                        target_value: target_value.ok_or(::serde::de::Error::missing_field("TargetValue"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DynamoDB::GlobalTable.TimeToLiveSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-timetolivespecification.html) property type.
    #[derive(Debug, Default)]
    pub struct TimeToLiveSpecification {
        /// Property [`AttributeName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-timetolivespecification.html#cfn-dynamodb-globaltable-timetolivespecification-attributename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attribute_name: Option<::Value<String>>,
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-timetolivespecification.html#cfn-dynamodb-globaltable-timetolivespecification-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: ::Value<bool>,
    }

    impl ::codec::SerializeValue for TimeToLiveSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref attribute_name) = self.attribute_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttributeName", attribute_name)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TimeToLiveSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TimeToLiveSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TimeToLiveSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TimeToLiveSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attribute_name: Option<::Value<String>> = None;
                    let mut enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AttributeName" => {
                                attribute_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TimeToLiveSpecification {
                        attribute_name: attribute_name,
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DynamoDB::GlobalTable.WriteProvisionedThroughputSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-writeprovisionedthroughputsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct WriteProvisionedThroughputSettings {
        /// Property [`WriteCapacityAutoScalingSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-writeprovisionedthroughputsettings.html#cfn-dynamodb-globaltable-writeprovisionedthroughputsettings-writecapacityautoscalingsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub write_capacity_auto_scaling_settings: Option<::Value<CapacityAutoScalingSettings>>,
    }

    impl ::codec::SerializeValue for WriteProvisionedThroughputSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref write_capacity_auto_scaling_settings) = self.write_capacity_auto_scaling_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WriteCapacityAutoScalingSettings", write_capacity_auto_scaling_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for WriteProvisionedThroughputSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<WriteProvisionedThroughputSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = WriteProvisionedThroughputSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type WriteProvisionedThroughputSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut write_capacity_auto_scaling_settings: Option<::Value<CapacityAutoScalingSettings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "WriteCapacityAutoScalingSettings" => {
                                write_capacity_auto_scaling_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(WriteProvisionedThroughputSettings {
                        write_capacity_auto_scaling_settings: write_capacity_auto_scaling_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod table {
    //! Property types for the `Table` resource.

    /// The [`AWS::DynamoDB::Table.AttributeDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-attributedef.html) property type.
    #[derive(Debug, Default)]
    pub struct AttributeDefinition {
        /// Property [`AttributeName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-attributedef.html#cfn-dynamodb-attributedef-attributename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attribute_name: ::Value<String>,
        /// Property [`AttributeType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-attributedef.html#cfn-dynamodb-attributedef-attributename-attributetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attribute_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for AttributeDefinition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttributeName", &self.attribute_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttributeType", &self.attribute_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AttributeDefinition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AttributeDefinition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AttributeDefinition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AttributeDefinition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attribute_name: Option<::Value<String>> = None;
                    let mut attribute_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AttributeName" => {
                                attribute_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AttributeType" => {
                                attribute_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AttributeDefinition {
                        attribute_name: attribute_name.ok_or(::serde::de::Error::missing_field("AttributeName"))?,
                        attribute_type: attribute_type.ok_or(::serde::de::Error::missing_field("AttributeType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DynamoDB::Table.ContributorInsightsSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-contributorinsightsspecification.html) property type.
    #[derive(Debug, Default)]
    pub struct ContributorInsightsSpecification {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-contributorinsightsspecification.html#cfn-dynamodb-contributorinsightsspecification-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: ::Value<bool>,
    }

    impl ::codec::SerializeValue for ContributorInsightsSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ContributorInsightsSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ContributorInsightsSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ContributorInsightsSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ContributorInsightsSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ContributorInsightsSpecification {
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DynamoDB::Table.GlobalSecondaryIndex`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-gsi.html) property type.
    #[derive(Debug, Default)]
    pub struct GlobalSecondaryIndex {
        /// Property [`ContributorInsightsSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-gsi.html#cfn-dynamodb-contributorinsightsspecification-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub contributor_insights_specification: Option<::Value<ContributorInsightsSpecification>>,
        /// Property [`IndexName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-gsi.html#cfn-dynamodb-gsi-indexname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub index_name: ::Value<String>,
        /// Property [`KeySchema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-gsi.html#cfn-dynamodb-gsi-keyschema).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key_schema: ::ValueList<KeySchema>,
        /// Property [`Projection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-gsi.html#cfn-dynamodb-gsi-projection).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub projection: ::Value<Projection>,
        /// Property [`ProvisionedThroughput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-gsi.html#cfn-dynamodb-gsi-provisionedthroughput).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub provisioned_throughput: Option<::Value<ProvisionedThroughput>>,
    }

    impl ::codec::SerializeValue for GlobalSecondaryIndex {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref contributor_insights_specification) = self.contributor_insights_specification {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContributorInsightsSpecification", contributor_insights_specification)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IndexName", &self.index_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeySchema", &self.key_schema)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Projection", &self.projection)?;
            if let Some(ref provisioned_throughput) = self.provisioned_throughput {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProvisionedThroughput", provisioned_throughput)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GlobalSecondaryIndex {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GlobalSecondaryIndex, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GlobalSecondaryIndex;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GlobalSecondaryIndex")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut contributor_insights_specification: Option<::Value<ContributorInsightsSpecification>> = None;
                    let mut index_name: Option<::Value<String>> = None;
                    let mut key_schema: Option<::ValueList<KeySchema>> = None;
                    let mut projection: Option<::Value<Projection>> = None;
                    let mut provisioned_throughput: Option<::Value<ProvisionedThroughput>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContributorInsightsSpecification" => {
                                contributor_insights_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IndexName" => {
                                index_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeySchema" => {
                                key_schema = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Projection" => {
                                projection = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProvisionedThroughput" => {
                                provisioned_throughput = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GlobalSecondaryIndex {
                        contributor_insights_specification: contributor_insights_specification,
                        index_name: index_name.ok_or(::serde::de::Error::missing_field("IndexName"))?,
                        key_schema: key_schema.ok_or(::serde::de::Error::missing_field("KeySchema"))?,
                        projection: projection.ok_or(::serde::de::Error::missing_field("Projection"))?,
                        provisioned_throughput: provisioned_throughput,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DynamoDB::Table.KeySchema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-keyschema.html) property type.
    #[derive(Debug, Default)]
    pub struct KeySchema {
        /// Property [`AttributeName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-keyschema.html#aws-properties-dynamodb-keyschema-attributename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attribute_name: ::Value<String>,
        /// Property [`KeyType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-keyschema.html#aws-properties-dynamodb-keyschema-keytype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for KeySchema {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttributeName", &self.attribute_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyType", &self.key_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KeySchema {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KeySchema, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KeySchema;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KeySchema")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attribute_name: Option<::Value<String>> = None;
                    let mut key_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AttributeName" => {
                                attribute_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeyType" => {
                                key_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KeySchema {
                        attribute_name: attribute_name.ok_or(::serde::de::Error::missing_field("AttributeName"))?,
                        key_type: key_type.ok_or(::serde::de::Error::missing_field("KeyType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DynamoDB::Table.KinesisStreamSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-kinesisstreamspecification.html) property type.
    #[derive(Debug, Default)]
    pub struct KinesisStreamSpecification {
        /// Property [`StreamArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-kinesisstreamspecification.html#cfn-dynamodb-kinesisstreamspecification-streamarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stream_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for KinesisStreamSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamArn", &self.stream_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KinesisStreamSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KinesisStreamSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KinesisStreamSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KinesisStreamSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut stream_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "StreamArn" => {
                                stream_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KinesisStreamSpecification {
                        stream_arn: stream_arn.ok_or(::serde::de::Error::missing_field("StreamArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DynamoDB::Table.LocalSecondaryIndex`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-lsi.html) property type.
    #[derive(Debug, Default)]
    pub struct LocalSecondaryIndex {
        /// Property [`IndexName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-lsi.html#cfn-dynamodb-lsi-indexname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub index_name: ::Value<String>,
        /// Property [`KeySchema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-lsi.html#cfn-dynamodb-lsi-keyschema).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key_schema: ::ValueList<KeySchema>,
        /// Property [`Projection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-lsi.html#cfn-dynamodb-lsi-projection).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub projection: ::Value<Projection>,
    }

    impl ::codec::SerializeValue for LocalSecondaryIndex {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IndexName", &self.index_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeySchema", &self.key_schema)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Projection", &self.projection)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LocalSecondaryIndex {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LocalSecondaryIndex, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LocalSecondaryIndex;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LocalSecondaryIndex")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut index_name: Option<::Value<String>> = None;
                    let mut key_schema: Option<::ValueList<KeySchema>> = None;
                    let mut projection: Option<::Value<Projection>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "IndexName" => {
                                index_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeySchema" => {
                                key_schema = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Projection" => {
                                projection = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LocalSecondaryIndex {
                        index_name: index_name.ok_or(::serde::de::Error::missing_field("IndexName"))?,
                        key_schema: key_schema.ok_or(::serde::de::Error::missing_field("KeySchema"))?,
                        projection: projection.ok_or(::serde::de::Error::missing_field("Projection"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DynamoDB::Table.PointInTimeRecoverySpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-table-pointintimerecoveryspecification.html) property type.
    #[derive(Debug, Default)]
    pub struct PointInTimeRecoverySpecification {
        /// Property [`PointInTimeRecoveryEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-table-pointintimerecoveryspecification.html#cfn-dynamodb-table-pointintimerecoveryspecification-pointintimerecoveryenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub point_in_time_recovery_enabled: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for PointInTimeRecoverySpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref point_in_time_recovery_enabled) = self.point_in_time_recovery_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PointInTimeRecoveryEnabled", point_in_time_recovery_enabled)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PointInTimeRecoverySpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PointInTimeRecoverySpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PointInTimeRecoverySpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PointInTimeRecoverySpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut point_in_time_recovery_enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PointInTimeRecoveryEnabled" => {
                                point_in_time_recovery_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PointInTimeRecoverySpecification {
                        point_in_time_recovery_enabled: point_in_time_recovery_enabled,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DynamoDB::Table.Projection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-projectionobject.html) property type.
    #[derive(Debug, Default)]
    pub struct Projection {
        /// Property [`NonKeyAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-projectionobject.html#cfn-dynamodb-projectionobj-nonkeyatt).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub non_key_attributes: Option<::ValueList<String>>,
        /// Property [`ProjectionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-projectionobject.html#cfn-dynamodb-projectionobj-projtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub projection_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Projection {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref non_key_attributes) = self.non_key_attributes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NonKeyAttributes", non_key_attributes)?;
            }
            if let Some(ref projection_type) = self.projection_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProjectionType", projection_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Projection {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Projection, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Projection;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Projection")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut non_key_attributes: Option<::ValueList<String>> = None;
                    let mut projection_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "NonKeyAttributes" => {
                                non_key_attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProjectionType" => {
                                projection_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Projection {
                        non_key_attributes: non_key_attributes,
                        projection_type: projection_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DynamoDB::Table.ProvisionedThroughput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-provisionedthroughput.html) property type.
    #[derive(Debug, Default)]
    pub struct ProvisionedThroughput {
        /// Property [`ReadCapacityUnits`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-provisionedthroughput.html#cfn-dynamodb-provisionedthroughput-readcapacityunits).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub read_capacity_units: ::Value<u64>,
        /// Property [`WriteCapacityUnits`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-provisionedthroughput.html#cfn-dynamodb-provisionedthroughput-writecapacityunits).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub write_capacity_units: ::Value<u64>,
    }

    impl ::codec::SerializeValue for ProvisionedThroughput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReadCapacityUnits", &self.read_capacity_units)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WriteCapacityUnits", &self.write_capacity_units)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ProvisionedThroughput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ProvisionedThroughput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ProvisionedThroughput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ProvisionedThroughput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut read_capacity_units: Option<::Value<u64>> = None;
                    let mut write_capacity_units: Option<::Value<u64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ReadCapacityUnits" => {
                                read_capacity_units = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WriteCapacityUnits" => {
                                write_capacity_units = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ProvisionedThroughput {
                        read_capacity_units: read_capacity_units.ok_or(::serde::de::Error::missing_field("ReadCapacityUnits"))?,
                        write_capacity_units: write_capacity_units.ok_or(::serde::de::Error::missing_field("WriteCapacityUnits"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DynamoDB::Table.SSESpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-table-ssespecification.html) property type.
    #[derive(Debug, Default)]
    pub struct SSESpecification {
        /// Property [`KMSMasterKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-table-ssespecification.html#cfn-dynamodb-table-ssespecification-kmsmasterkeyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kms_master_key_id: Option<::Value<String>>,
        /// Property [`SSEEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-table-ssespecification.html#cfn-dynamodb-table-ssespecification-sseenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sse_enabled: ::Value<bool>,
        /// Property [`SSEType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-table-ssespecification.html#cfn-dynamodb-table-ssespecification-ssetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sse_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SSESpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref kms_master_key_id) = self.kms_master_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KMSMasterKeyId", kms_master_key_id)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SSEEnabled", &self.sse_enabled)?;
            if let Some(ref sse_type) = self.sse_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SSEType", sse_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SSESpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SSESpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SSESpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SSESpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut kms_master_key_id: Option<::Value<String>> = None;
                    let mut sse_enabled: Option<::Value<bool>> = None;
                    let mut sse_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KMSMasterKeyId" => {
                                kms_master_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SSEEnabled" => {
                                sse_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SSEType" => {
                                sse_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SSESpecification {
                        kms_master_key_id: kms_master_key_id,
                        sse_enabled: sse_enabled.ok_or(::serde::de::Error::missing_field("SSEEnabled"))?,
                        sse_type: sse_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DynamoDB::Table.StreamSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-streamspecification.html) property type.
    #[derive(Debug, Default)]
    pub struct StreamSpecification {
        /// Property [`StreamViewType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-streamspecification.html#cfn-dynamodb-streamspecification-streamviewtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stream_view_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for StreamSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamViewType", &self.stream_view_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StreamSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StreamSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StreamSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StreamSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut stream_view_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "StreamViewType" => {
                                stream_view_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StreamSpecification {
                        stream_view_type: stream_view_type.ok_or(::serde::de::Error::missing_field("StreamViewType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DynamoDB::Table.TimeToLiveSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-timetolivespecification.html) property type.
    #[derive(Debug, Default)]
    pub struct TimeToLiveSpecification {
        /// Property [`AttributeName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-timetolivespecification.html#cfn-dynamodb-timetolivespecification-attributename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attribute_name: ::Value<String>,
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-timetolivespecification.html#cfn-dynamodb-timetolivespecification-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: ::Value<bool>,
    }

    impl ::codec::SerializeValue for TimeToLiveSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttributeName", &self.attribute_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TimeToLiveSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TimeToLiveSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TimeToLiveSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TimeToLiveSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attribute_name: Option<::Value<String>> = None;
                    let mut enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AttributeName" => {
                                attribute_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TimeToLiveSpecification {
                        attribute_name: attribute_name.ok_or(::serde::de::Error::missing_field("AttributeName"))?,
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
