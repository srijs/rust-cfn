//! Types for the `DynamoDB` service.

/// The [`AWS::DynamoDB::Table`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dynamodb-table.html) resource type.
#[derive(Debug)]
pub struct Table {
    properties: TableProperties
}

/// Properties for the `Table` resource.
#[derive(Debug)]
pub struct TableProperties {
    /// Property `AttributeDefinitions`.
    pub attribute_definitions: Option<::ValueList<self::table::AttributeDefinition>>,
    /// Property `GlobalSecondaryIndexes`.
    pub global_secondary_indexes: Option<::ValueList<self::table::GlobalSecondaryIndex>>,
    /// Property `KeySchema`.
    pub key_schema: ::ValueList<self::table::KeySchema>,
    /// Property `LocalSecondaryIndexes`.
    pub local_secondary_indexes: Option<::ValueList<self::table::LocalSecondaryIndex>>,
    /// Property `ProvisionedThroughput`.
    pub provisioned_throughput: ::Value<self::table::ProvisionedThroughput>,
    /// Property `SSESpecification`.
    pub sse_specification: Option<::Value<self::table::SSESpecification>>,
    /// Property `StreamSpecification`.
    pub stream_specification: Option<::Value<self::table::StreamSpecification>>,
    /// Property `TableName`.
    pub table_name: Option<::Value<String>>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `TimeToLiveSpecification`.
    pub time_to_live_specification: Option<::Value<self::table::TimeToLiveSpecification>>,
}

impl ::serde::Serialize for TableProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref attribute_definitions) = self.attribute_definitions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttributeDefinitions", attribute_definitions)?;
        }
        if let Some(ref global_secondary_indexes) = self.global_secondary_indexes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GlobalSecondaryIndexes", global_secondary_indexes)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeySchema", &self.key_schema)?;
        if let Some(ref local_secondary_indexes) = self.local_secondary_indexes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LocalSecondaryIndexes", local_secondary_indexes)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProvisionedThroughput", &self.provisioned_throughput)?;
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
                let mut attribute_definitions = None;
                let mut global_secondary_indexes = None;
                let mut key_schema = None;
                let mut local_secondary_indexes = None;
                let mut provisioned_throughput = None;
                let mut sse_specification = None;
                let mut stream_specification = None;
                let mut table_name = None;
                let mut tags = None;
                let mut time_to_live_specification = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AttributeDefinitions" => {
                            attribute_definitions = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "GlobalSecondaryIndexes" => {
                            global_secondary_indexes = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "KeySchema" => {
                            key_schema = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "LocalSecondaryIndexes" => {
                            local_secondary_indexes = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ProvisionedThroughput" => {
                            provisioned_throughput = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SSESpecification" => {
                            sse_specification = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "StreamSpecification" => {
                            stream_specification = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "TableName" => {
                            table_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "TimeToLiveSpecification" => {
                            time_to_live_specification = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(TableProperties {
                    attribute_definitions: attribute_definitions,
                    global_secondary_indexes: global_secondary_indexes,
                    key_schema: key_schema.ok_or(::serde::de::Error::missing_field("KeySchema"))?,
                    local_secondary_indexes: local_secondary_indexes,
                    provisioned_throughput: provisioned_throughput.ok_or(::serde::de::Error::missing_field("ProvisionedThroughput"))?,
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

pub mod table {
    //! Property types for the `Table` resource.

    /// The [`AWS::DynamoDB::Table.AttributeDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-attributedef.html) property type.
    #[derive(Debug)]
    pub struct AttributeDefinition {
        /// Property `AttributeName`.
        pub attribute_name: ::Value<String>,
        /// Property `AttributeType`.
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
                    let mut attribute_name = None;
                    let mut attribute_type = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AttributeName" => {
                                attribute_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "AttributeType" => {
                                attribute_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
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

    /// The [`AWS::DynamoDB::Table.GlobalSecondaryIndex`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-gsi.html) property type.
    #[derive(Debug)]
    pub struct GlobalSecondaryIndex {
        /// Property `IndexName`.
        pub index_name: ::Value<String>,
        /// Property `KeySchema`.
        pub key_schema: ::ValueList<KeySchema>,
        /// Property `Projection`.
        pub projection: ::Value<Projection>,
        /// Property `ProvisionedThroughput`.
        pub provisioned_throughput: ::Value<ProvisionedThroughput>,
    }

    impl ::codec::SerializeValue for GlobalSecondaryIndex {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IndexName", &self.index_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeySchema", &self.key_schema)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Projection", &self.projection)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProvisionedThroughput", &self.provisioned_throughput)?;
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
                    let mut index_name = None;
                    let mut key_schema = None;
                    let mut projection = None;
                    let mut provisioned_throughput = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "IndexName" => {
                                index_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "KeySchema" => {
                                key_schema = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Projection" => {
                                projection = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ProvisionedThroughput" => {
                                provisioned_throughput = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(GlobalSecondaryIndex {
                        index_name: index_name.ok_or(::serde::de::Error::missing_field("IndexName"))?,
                        key_schema: key_schema.ok_or(::serde::de::Error::missing_field("KeySchema"))?,
                        projection: projection.ok_or(::serde::de::Error::missing_field("Projection"))?,
                        provisioned_throughput: provisioned_throughput.ok_or(::serde::de::Error::missing_field("ProvisionedThroughput"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DynamoDB::Table.KeySchema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-keyschema.html) property type.
    #[derive(Debug)]
    pub struct KeySchema {
        /// Property `AttributeName`.
        pub attribute_name: ::Value<String>,
        /// Property `KeyType`.
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
                    let mut attribute_name = None;
                    let mut key_type = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AttributeName" => {
                                attribute_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "KeyType" => {
                                key_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
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

    /// The [`AWS::DynamoDB::Table.LocalSecondaryIndex`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-lsi.html) property type.
    #[derive(Debug)]
    pub struct LocalSecondaryIndex {
        /// Property `IndexName`.
        pub index_name: ::Value<String>,
        /// Property `KeySchema`.
        pub key_schema: ::ValueList<KeySchema>,
        /// Property `Projection`.
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
                    let mut index_name = None;
                    let mut key_schema = None;
                    let mut projection = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "IndexName" => {
                                index_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "KeySchema" => {
                                key_schema = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Projection" => {
                                projection = Some(::serde::de::MapAccess::next_value(&mut map)?);
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

    /// The [`AWS::DynamoDB::Table.Projection`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-projectionobject.html) property type.
    #[derive(Debug)]
    pub struct Projection {
        /// Property `NonKeyAttributes`.
        pub non_key_attributes: Option<::ValueList<String>>,
        /// Property `ProjectionType`.
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
                    let mut non_key_attributes = None;
                    let mut projection_type = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "NonKeyAttributes" => {
                                non_key_attributes = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ProjectionType" => {
                                projection_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
    #[derive(Debug)]
    pub struct ProvisionedThroughput {
        /// Property `ReadCapacityUnits`.
        pub read_capacity_units: ::Value<u64>,
        /// Property `WriteCapacityUnits`.
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
                    let mut read_capacity_units = None;
                    let mut write_capacity_units = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ReadCapacityUnits" => {
                                read_capacity_units = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "WriteCapacityUnits" => {
                                write_capacity_units = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
    #[derive(Debug)]
    pub struct SSESpecification {
        /// Property `SSEEnabled`.
        pub sse_enabled: ::Value<bool>,
    }

    impl ::codec::SerializeValue for SSESpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SSEEnabled", &self.sse_enabled)?;
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
                    let mut sse_enabled = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SSEEnabled" => {
                                sse_enabled = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(SSESpecification {
                        sse_enabled: sse_enabled.ok_or(::serde::de::Error::missing_field("SSEEnabled"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DynamoDB::Table.StreamSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-streamspecification.html) property type.
    #[derive(Debug)]
    pub struct StreamSpecification {
        /// Property `StreamViewType`.
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
                    let mut stream_view_type = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "StreamViewType" => {
                                stream_view_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
    #[derive(Debug)]
    pub struct TimeToLiveSpecification {
        /// Property `AttributeName`.
        pub attribute_name: ::Value<String>,
        /// Property `Enabled`.
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
                    let mut attribute_name = None;
                    let mut enabled = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AttributeName" => {
                                attribute_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Enabled" => {
                                enabled = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
