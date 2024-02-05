//! Types for the `Cassandra` service.

/// The [`AWS::Cassandra::Keyspace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cassandra-keyspace.html) resource type.
#[derive(Debug, Default)]
pub struct Keyspace {
    properties: KeyspaceProperties
}

/// Properties for the `Keyspace` resource.
#[derive(Debug, Default)]
pub struct KeyspaceProperties {
    /// Property [`KeyspaceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cassandra-keyspace.html#cfn-cassandra-keyspace-keyspacename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub keyspace_name: Option<::Value<String>>,
    /// Property [`ReplicationSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cassandra-keyspace.html#cfn-cassandra-keyspace-replicationspecification).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub replication_specification: Option<::Value<self::keyspace::ReplicationSpecification>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cassandra-keyspace.html#cfn-cassandra-keyspace-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for KeyspaceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref keyspace_name) = self.keyspace_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyspaceName", keyspace_name)?;
        }
        if let Some(ref replication_specification) = self.replication_specification {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicationSpecification", replication_specification)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for KeyspaceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<KeyspaceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = KeyspaceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type KeyspaceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut keyspace_name: Option<::Value<String>> = None;
                let mut replication_specification: Option<::Value<self::keyspace::ReplicationSpecification>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "KeyspaceName" => {
                            keyspace_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReplicationSpecification" => {
                            replication_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(KeyspaceProperties {
                    keyspace_name: keyspace_name,
                    replication_specification: replication_specification,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Keyspace {
    type Properties = KeyspaceProperties;
    const TYPE: &'static str = "AWS::Cassandra::Keyspace";
    fn properties(&self) -> &KeyspaceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut KeyspaceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Keyspace {}

impl From<KeyspaceProperties> for Keyspace {
    fn from(properties: KeyspaceProperties) -> Keyspace {
        Keyspace { properties }
    }
}

/// The [`AWS::Cassandra::Table`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cassandra-table.html) resource type.
#[derive(Debug, Default)]
pub struct Table {
    properties: TableProperties
}

/// Properties for the `Table` resource.
#[derive(Debug, Default)]
pub struct TableProperties {
    /// Property [`BillingMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cassandra-table.html#cfn-cassandra-table-billingmode).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub billing_mode: Option<::Value<self::table::BillingMode>>,
    /// Property [`ClientSideTimestampsEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cassandra-table.html#cfn-cassandra-table-clientsidetimestampsenabled).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub client_side_timestamps_enabled: Option<::Value<bool>>,
    /// Property [`ClusteringKeyColumns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cassandra-table.html#cfn-cassandra-table-clusteringkeycolumns).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub clustering_key_columns: Option<::ValueList<self::table::ClusteringKeyColumn>>,
    /// Property [`DefaultTimeToLive`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cassandra-table.html#cfn-cassandra-table-defaulttimetolive).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub default_time_to_live: Option<::Value<u32>>,
    /// Property [`EncryptionSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cassandra-table.html#cfn-cassandra-table-encryptionspecification).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub encryption_specification: Option<::Value<self::table::EncryptionSpecification>>,
    /// Property [`KeyspaceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cassandra-table.html#cfn-cassandra-table-keyspacename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub keyspace_name: ::Value<String>,
    /// Property [`PartitionKeyColumns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cassandra-table.html#cfn-cassandra-table-partitionkeycolumns).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub partition_key_columns: ::ValueList<self::table::Column>,
    /// Property [`PointInTimeRecoveryEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cassandra-table.html#cfn-cassandra-table-pointintimerecoveryenabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub point_in_time_recovery_enabled: Option<::Value<bool>>,
    /// Property [`RegularColumns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cassandra-table.html#cfn-cassandra-table-regularcolumns).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub regular_columns: Option<::ValueList<self::table::Column>>,
    /// Property [`TableName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cassandra-table.html#cfn-cassandra-table-tablename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub table_name: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cassandra-table.html#cfn-cassandra-table-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for TableProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref billing_mode) = self.billing_mode {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BillingMode", billing_mode)?;
        }
        if let Some(ref client_side_timestamps_enabled) = self.client_side_timestamps_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientSideTimestampsEnabled", client_side_timestamps_enabled)?;
        }
        if let Some(ref clustering_key_columns) = self.clustering_key_columns {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusteringKeyColumns", clustering_key_columns)?;
        }
        if let Some(ref default_time_to_live) = self.default_time_to_live {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultTimeToLive", default_time_to_live)?;
        }
        if let Some(ref encryption_specification) = self.encryption_specification {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionSpecification", encryption_specification)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyspaceName", &self.keyspace_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PartitionKeyColumns", &self.partition_key_columns)?;
        if let Some(ref point_in_time_recovery_enabled) = self.point_in_time_recovery_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PointInTimeRecoveryEnabled", point_in_time_recovery_enabled)?;
        }
        if let Some(ref regular_columns) = self.regular_columns {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RegularColumns", regular_columns)?;
        }
        if let Some(ref table_name) = self.table_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableName", table_name)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
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
                let mut billing_mode: Option<::Value<self::table::BillingMode>> = None;
                let mut client_side_timestamps_enabled: Option<::Value<bool>> = None;
                let mut clustering_key_columns: Option<::ValueList<self::table::ClusteringKeyColumn>> = None;
                let mut default_time_to_live: Option<::Value<u32>> = None;
                let mut encryption_specification: Option<::Value<self::table::EncryptionSpecification>> = None;
                let mut keyspace_name: Option<::Value<String>> = None;
                let mut partition_key_columns: Option<::ValueList<self::table::Column>> = None;
                let mut point_in_time_recovery_enabled: Option<::Value<bool>> = None;
                let mut regular_columns: Option<::ValueList<self::table::Column>> = None;
                let mut table_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "BillingMode" => {
                            billing_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClientSideTimestampsEnabled" => {
                            client_side_timestamps_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClusteringKeyColumns" => {
                            clustering_key_columns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DefaultTimeToLive" => {
                            default_time_to_live = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EncryptionSpecification" => {
                            encryption_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KeyspaceName" => {
                            keyspace_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PartitionKeyColumns" => {
                            partition_key_columns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PointInTimeRecoveryEnabled" => {
                            point_in_time_recovery_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RegularColumns" => {
                            regular_columns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TableName" => {
                            table_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(TableProperties {
                    billing_mode: billing_mode,
                    client_side_timestamps_enabled: client_side_timestamps_enabled,
                    clustering_key_columns: clustering_key_columns,
                    default_time_to_live: default_time_to_live,
                    encryption_specification: encryption_specification,
                    keyspace_name: keyspace_name.ok_or(::serde::de::Error::missing_field("KeyspaceName"))?,
                    partition_key_columns: partition_key_columns.ok_or(::serde::de::Error::missing_field("PartitionKeyColumns"))?,
                    point_in_time_recovery_enabled: point_in_time_recovery_enabled,
                    regular_columns: regular_columns,
                    table_name: table_name,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Table {
    type Properties = TableProperties;
    const TYPE: &'static str = "AWS::Cassandra::Table";
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

pub mod keyspace {
    //! Property types for the `Keyspace` resource.

    /// The [`AWS::Cassandra::Keyspace.ReplicationSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cassandra-keyspace-replicationspecification.html) property type.
    #[derive(Debug, Default)]
    pub struct ReplicationSpecification {
        /// Property [`RegionList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cassandra-keyspace-replicationspecification.html#cfn-cassandra-keyspace-replicationspecification-regionlist).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub region_list: Option<::ValueList<String>>,
        /// Property [`ReplicationStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cassandra-keyspace-replicationspecification.html#cfn-cassandra-keyspace-replicationspecification-replicationstrategy).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub replication_strategy: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ReplicationSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref region_list) = self.region_list {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RegionList", region_list)?;
            }
            if let Some(ref replication_strategy) = self.replication_strategy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicationStrategy", replication_strategy)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ReplicationSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ReplicationSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ReplicationSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ReplicationSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut region_list: Option<::ValueList<String>> = None;
                    let mut replication_strategy: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RegionList" => {
                                region_list = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReplicationStrategy" => {
                                replication_strategy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ReplicationSpecification {
                        region_list: region_list,
                        replication_strategy: replication_strategy,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod table {
    //! Property types for the `Table` resource.

    /// The [`AWS::Cassandra::Table.BillingMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cassandra-table-billingmode.html) property type.
    #[derive(Debug, Default)]
    pub struct BillingMode {
        /// Property [`Mode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cassandra-table-billingmode.html#cfn-cassandra-table-billingmode-mode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mode: ::Value<String>,
        /// Property [`ProvisionedThroughput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cassandra-table-billingmode.html#cfn-cassandra-table-billingmode-provisionedthroughput).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub provisioned_throughput: Option<::Value<ProvisionedThroughput>>,
    }

    impl ::codec::SerializeValue for BillingMode {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Mode", &self.mode)?;
            if let Some(ref provisioned_throughput) = self.provisioned_throughput {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProvisionedThroughput", provisioned_throughput)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BillingMode {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BillingMode, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BillingMode;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BillingMode")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut mode: Option<::Value<String>> = None;
                    let mut provisioned_throughput: Option<::Value<ProvisionedThroughput>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Mode" => {
                                mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProvisionedThroughput" => {
                                provisioned_throughput = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BillingMode {
                        mode: mode.ok_or(::serde::de::Error::missing_field("Mode"))?,
                        provisioned_throughput: provisioned_throughput,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Cassandra::Table.ClusteringKeyColumn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cassandra-table-clusteringkeycolumn.html) property type.
    #[derive(Debug, Default)]
    pub struct ClusteringKeyColumn {
        /// Property [`Column`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cassandra-table-clusteringkeycolumn.html#cfn-cassandra-table-clusteringkeycolumn-column).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub column: ::Value<Column>,
        /// Property [`OrderBy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cassandra-table-clusteringkeycolumn.html#cfn-cassandra-table-clusteringkeycolumn-orderby).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub order_by: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ClusteringKeyColumn {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Column", &self.column)?;
            if let Some(ref order_by) = self.order_by {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OrderBy", order_by)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ClusteringKeyColumn {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ClusteringKeyColumn, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ClusteringKeyColumn;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ClusteringKeyColumn")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut column: Option<::Value<Column>> = None;
                    let mut order_by: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Column" => {
                                column = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OrderBy" => {
                                order_by = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ClusteringKeyColumn {
                        column: column.ok_or(::serde::de::Error::missing_field("Column"))?,
                        order_by: order_by,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Cassandra::Table.Column`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cassandra-table-column.html) property type.
    #[derive(Debug, Default)]
    pub struct Column {
        /// Property [`ColumnName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cassandra-table-column.html#cfn-cassandra-table-column-columnname).
        ///
        /// Update type: _Conditional_.
        /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
        /// For more information, see the relevant resource type documentation.
        pub column_name: ::Value<String>,
        /// Property [`ColumnType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cassandra-table-column.html#cfn-cassandra-table-column-columntype).
        ///
        /// Update type: _Conditional_.
        /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
        /// For more information, see the relevant resource type documentation.
        pub column_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for Column {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColumnName", &self.column_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColumnType", &self.column_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Column {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Column, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Column;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Column")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut column_name: Option<::Value<String>> = None;
                    let mut column_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ColumnName" => {
                                column_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ColumnType" => {
                                column_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Column {
                        column_name: column_name.ok_or(::serde::de::Error::missing_field("ColumnName"))?,
                        column_type: column_type.ok_or(::serde::de::Error::missing_field("ColumnType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Cassandra::Table.EncryptionSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cassandra-table-encryptionspecification.html) property type.
    #[derive(Debug, Default)]
    pub struct EncryptionSpecification {
        /// Property [`EncryptionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cassandra-table-encryptionspecification.html#cfn-cassandra-table-encryptionspecification-encryptiontype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encryption_type: ::Value<String>,
        /// Property [`KmsKeyIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cassandra-table-encryptionspecification.html#cfn-cassandra-table-encryptionspecification-kmskeyidentifier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kms_key_identifier: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EncryptionSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionType", &self.encryption_type)?;
            if let Some(ref kms_key_identifier) = self.kms_key_identifier {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyIdentifier", kms_key_identifier)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EncryptionSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EncryptionSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EncryptionSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EncryptionSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut encryption_type: Option<::Value<String>> = None;
                    let mut kms_key_identifier: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EncryptionType" => {
                                encryption_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KmsKeyIdentifier" => {
                                kms_key_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EncryptionSpecification {
                        encryption_type: encryption_type.ok_or(::serde::de::Error::missing_field("EncryptionType"))?,
                        kms_key_identifier: kms_key_identifier,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Cassandra::Table.ProvisionedThroughput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cassandra-table-provisionedthroughput.html) property type.
    #[derive(Debug, Default)]
    pub struct ProvisionedThroughput {
        /// Property [`ReadCapacityUnits`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cassandra-table-provisionedthroughput.html#cfn-cassandra-table-provisionedthroughput-readcapacityunits).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub read_capacity_units: ::Value<u32>,
        /// Property [`WriteCapacityUnits`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cassandra-table-provisionedthroughput.html#cfn-cassandra-table-provisionedthroughput-writecapacityunits).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub write_capacity_units: ::Value<u32>,
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
                    let mut read_capacity_units: Option<::Value<u32>> = None;
                    let mut write_capacity_units: Option<::Value<u32>> = None;

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
}
