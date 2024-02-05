//! Types for the `Athena` service.

/// The [`AWS::Athena::CapacityReservation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-athena-capacityreservation.html) resource type.
#[derive(Debug, Default)]
pub struct CapacityReservation {
    properties: CapacityReservationProperties
}

/// Properties for the `CapacityReservation` resource.
#[derive(Debug, Default)]
pub struct CapacityReservationProperties {
    /// Property [`CapacityAssignmentConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-athena-capacityreservation.html#cfn-athena-capacityreservation-capacityassignmentconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub capacity_assignment_configuration: Option<::Value<self::capacity_reservation::CapacityAssignmentConfiguration>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-athena-capacityreservation.html#cfn-athena-capacityreservation-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-athena-capacityreservation.html#cfn-athena-capacityreservation-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TargetDpus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-athena-capacityreservation.html#cfn-athena-capacityreservation-targetdpus).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub target_dpus: ::Value<u32>,
}

impl ::serde::Serialize for CapacityReservationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref capacity_assignment_configuration) = self.capacity_assignment_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CapacityAssignmentConfiguration", capacity_assignment_configuration)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetDpus", &self.target_dpus)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CapacityReservationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CapacityReservationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CapacityReservationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CapacityReservationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut capacity_assignment_configuration: Option<::Value<self::capacity_reservation::CapacityAssignmentConfiguration>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut target_dpus: Option<::Value<u32>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CapacityAssignmentConfiguration" => {
                            capacity_assignment_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetDpus" => {
                            target_dpus = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CapacityReservationProperties {
                    capacity_assignment_configuration: capacity_assignment_configuration,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    tags: tags,
                    target_dpus: target_dpus.ok_or(::serde::de::Error::missing_field("TargetDpus"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for CapacityReservation {
    type Properties = CapacityReservationProperties;
    const TYPE: &'static str = "AWS::Athena::CapacityReservation";
    fn properties(&self) -> &CapacityReservationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CapacityReservationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for CapacityReservation {}

impl From<CapacityReservationProperties> for CapacityReservation {
    fn from(properties: CapacityReservationProperties) -> CapacityReservation {
        CapacityReservation { properties }
    }
}

/// The [`AWS::Athena::DataCatalog`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-athena-datacatalog.html) resource type.
#[derive(Debug, Default)]
pub struct DataCatalog {
    properties: DataCatalogProperties
}

/// Properties for the `DataCatalog` resource.
#[derive(Debug, Default)]
pub struct DataCatalogProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-athena-datacatalog.html#cfn-athena-datacatalog-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-athena-datacatalog.html#cfn-athena-datacatalog-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-athena-datacatalog.html#cfn-athena-datacatalog-parameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub parameters: Option<::ValueMap<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-athena-datacatalog.html#cfn-athena-datacatalog-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-athena-datacatalog.html#cfn-athena-datacatalog-type).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub r#type: ::Value<String>,
}

impl ::serde::Serialize for DataCatalogProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref parameters) = self.parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", parameters)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DataCatalogProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DataCatalogProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DataCatalogProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DataCatalogProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut parameters: Option<::ValueMap<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut r#type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Parameters" => {
                            parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Type" => {
                            r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DataCatalogProperties {
                    description: description,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    parameters: parameters,
                    tags: tags,
                    r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DataCatalog {
    type Properties = DataCatalogProperties;
    const TYPE: &'static str = "AWS::Athena::DataCatalog";
    fn properties(&self) -> &DataCatalogProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DataCatalogProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DataCatalog {}

impl From<DataCatalogProperties> for DataCatalog {
    fn from(properties: DataCatalogProperties) -> DataCatalog {
        DataCatalog { properties }
    }
}

/// The [`AWS::Athena::NamedQuery`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-athena-namedquery.html) resource type.
#[derive(Debug, Default)]
pub struct NamedQuery {
    properties: NamedQueryProperties
}

/// Properties for the `NamedQuery` resource.
#[derive(Debug, Default)]
pub struct NamedQueryProperties {
    /// Property [`Database`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-athena-namedquery.html#cfn-athena-namedquery-database).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub database: ::Value<String>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-athena-namedquery.html#cfn-athena-namedquery-description).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-athena-namedquery.html#cfn-athena-namedquery-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`QueryString`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-athena-namedquery.html#cfn-athena-namedquery-querystring).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub query_string: ::Value<String>,
    /// Property [`WorkGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-athena-namedquery.html#cfn-athena-namedquery-workgroup).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub work_group: Option<::Value<String>>,
}

impl ::serde::Serialize for NamedQueryProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Database", &self.database)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryString", &self.query_string)?;
        if let Some(ref work_group) = self.work_group {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkGroup", work_group)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for NamedQueryProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<NamedQueryProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = NamedQueryProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type NamedQueryProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut database: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut query_string: Option<::Value<String>> = None;
                let mut work_group: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Database" => {
                            database = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "QueryString" => {
                            query_string = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "WorkGroup" => {
                            work_group = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(NamedQueryProperties {
                    database: database.ok_or(::serde::de::Error::missing_field("Database"))?,
                    description: description,
                    name: name,
                    query_string: query_string.ok_or(::serde::de::Error::missing_field("QueryString"))?,
                    work_group: work_group,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for NamedQuery {
    type Properties = NamedQueryProperties;
    const TYPE: &'static str = "AWS::Athena::NamedQuery";
    fn properties(&self) -> &NamedQueryProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut NamedQueryProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for NamedQuery {}

impl From<NamedQueryProperties> for NamedQuery {
    fn from(properties: NamedQueryProperties) -> NamedQuery {
        NamedQuery { properties }
    }
}

/// The [`AWS::Athena::PreparedStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-athena-preparedstatement.html) resource type.
#[derive(Debug, Default)]
pub struct PreparedStatement {
    properties: PreparedStatementProperties
}

/// Properties for the `PreparedStatement` resource.
#[derive(Debug, Default)]
pub struct PreparedStatementProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-athena-preparedstatement.html#cfn-athena-preparedstatement-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`QueryStatement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-athena-preparedstatement.html#cfn-athena-preparedstatement-querystatement).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub query_statement: ::Value<String>,
    /// Property [`StatementName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-athena-preparedstatement.html#cfn-athena-preparedstatement-statementname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub statement_name: ::Value<String>,
    /// Property [`WorkGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-athena-preparedstatement.html#cfn-athena-preparedstatement-workgroup).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub work_group: ::Value<String>,
}

impl ::serde::Serialize for PreparedStatementProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryStatement", &self.query_statement)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StatementName", &self.statement_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkGroup", &self.work_group)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PreparedStatementProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PreparedStatementProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PreparedStatementProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PreparedStatementProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut query_statement: Option<::Value<String>> = None;
                let mut statement_name: Option<::Value<String>> = None;
                let mut work_group: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "QueryStatement" => {
                            query_statement = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StatementName" => {
                            statement_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "WorkGroup" => {
                            work_group = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PreparedStatementProperties {
                    description: description,
                    query_statement: query_statement.ok_or(::serde::de::Error::missing_field("QueryStatement"))?,
                    statement_name: statement_name.ok_or(::serde::de::Error::missing_field("StatementName"))?,
                    work_group: work_group.ok_or(::serde::de::Error::missing_field("WorkGroup"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for PreparedStatement {
    type Properties = PreparedStatementProperties;
    const TYPE: &'static str = "AWS::Athena::PreparedStatement";
    fn properties(&self) -> &PreparedStatementProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PreparedStatementProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for PreparedStatement {}

impl From<PreparedStatementProperties> for PreparedStatement {
    fn from(properties: PreparedStatementProperties) -> PreparedStatement {
        PreparedStatement { properties }
    }
}

/// The [`AWS::Athena::WorkGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-athena-workgroup.html) resource type.
#[derive(Debug, Default)]
pub struct WorkGroup {
    properties: WorkGroupProperties
}

/// Properties for the `WorkGroup` resource.
#[derive(Debug, Default)]
pub struct WorkGroupProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-athena-workgroup.html#cfn-athena-workgroup-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-athena-workgroup.html#cfn-athena-workgroup-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`RecursiveDeleteOption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-athena-workgroup.html#cfn-athena-workgroup-recursivedeleteoption).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub recursive_delete_option: Option<::Value<bool>>,
    /// Property [`State`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-athena-workgroup.html#cfn-athena-workgroup-state).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub state: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-athena-workgroup.html#cfn-athena-workgroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`WorkGroupConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-athena-workgroup.html#cfn-athena-workgroup-workgroupconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub work_group_configuration: Option<::Value<self::work_group::WorkGroupConfiguration>>,
}

impl ::serde::Serialize for WorkGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref recursive_delete_option) = self.recursive_delete_option {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecursiveDeleteOption", recursive_delete_option)?;
        }
        if let Some(ref state) = self.state {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "State", state)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref work_group_configuration) = self.work_group_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkGroupConfiguration", work_group_configuration)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for WorkGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<WorkGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = WorkGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type WorkGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut recursive_delete_option: Option<::Value<bool>> = None;
                let mut state: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut work_group_configuration: Option<::Value<self::work_group::WorkGroupConfiguration>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RecursiveDeleteOption" => {
                            recursive_delete_option = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "State" => {
                            state = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "WorkGroupConfiguration" => {
                            work_group_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(WorkGroupProperties {
                    description: description,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    recursive_delete_option: recursive_delete_option,
                    state: state,
                    tags: tags,
                    work_group_configuration: work_group_configuration,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for WorkGroup {
    type Properties = WorkGroupProperties;
    const TYPE: &'static str = "AWS::Athena::WorkGroup";
    fn properties(&self) -> &WorkGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut WorkGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for WorkGroup {}

impl From<WorkGroupProperties> for WorkGroup {
    fn from(properties: WorkGroupProperties) -> WorkGroup {
        WorkGroup { properties }
    }
}

pub mod capacity_reservation {
    //! Property types for the `CapacityReservation` resource.

    /// The [`AWS::Athena::CapacityReservation.CapacityAssignment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-capacityreservation-capacityassignment.html) property type.
    #[derive(Debug, Default)]
    pub struct CapacityAssignment {
        /// Property [`WorkgroupNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-capacityreservation-capacityassignment.html#cfn-athena-capacityreservation-capacityassignment-workgroupnames).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub workgroup_names: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for CapacityAssignment {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkgroupNames", &self.workgroup_names)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CapacityAssignment {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CapacityAssignment, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CapacityAssignment;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CapacityAssignment")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut workgroup_names: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "WorkgroupNames" => {
                                workgroup_names = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CapacityAssignment {
                        workgroup_names: workgroup_names.ok_or(::serde::de::Error::missing_field("WorkgroupNames"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Athena::CapacityReservation.CapacityAssignmentConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-capacityreservation-capacityassignmentconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct CapacityAssignmentConfiguration {
        /// Property [`CapacityAssignments`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-capacityreservation-capacityassignmentconfiguration.html#cfn-athena-capacityreservation-capacityassignmentconfiguration-capacityassignments).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub capacity_assignments: ::ValueList<CapacityAssignment>,
    }

    impl ::codec::SerializeValue for CapacityAssignmentConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CapacityAssignments", &self.capacity_assignments)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CapacityAssignmentConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CapacityAssignmentConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CapacityAssignmentConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CapacityAssignmentConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut capacity_assignments: Option<::ValueList<CapacityAssignment>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CapacityAssignments" => {
                                capacity_assignments = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CapacityAssignmentConfiguration {
                        capacity_assignments: capacity_assignments.ok_or(::serde::de::Error::missing_field("CapacityAssignments"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod work_group {
    //! Property types for the `WorkGroup` resource.

    /// The [`AWS::Athena::WorkGroup.AclConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-workgroup-aclconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct AclConfiguration {
        /// Property [`S3AclOption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-workgroup-aclconfiguration.html#cfn-athena-workgroup-aclconfiguration-s3acloption).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_acl_option: ::Value<String>,
    }

    impl ::codec::SerializeValue for AclConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3AclOption", &self.s3_acl_option)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AclConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AclConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AclConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AclConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut s3_acl_option: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3AclOption" => {
                                s3_acl_option = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AclConfiguration {
                        s3_acl_option: s3_acl_option.ok_or(::serde::de::Error::missing_field("S3AclOption"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Athena::WorkGroup.CustomerContentEncryptionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-workgroup-customercontentencryptionconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomerContentEncryptionConfiguration {
        /// Property [`KmsKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-workgroup-customercontentencryptionconfiguration.html#cfn-athena-workgroup-customercontentencryptionconfiguration-kmskey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kms_key: ::Value<String>,
    }

    impl ::codec::SerializeValue for CustomerContentEncryptionConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKey", &self.kms_key)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomerContentEncryptionConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomerContentEncryptionConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomerContentEncryptionConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomerContentEncryptionConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut kms_key: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KmsKey" => {
                                kms_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomerContentEncryptionConfiguration {
                        kms_key: kms_key.ok_or(::serde::de::Error::missing_field("KmsKey"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Athena::WorkGroup.EncryptionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-workgroup-encryptionconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct EncryptionConfiguration {
        /// Property [`EncryptionOption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-workgroup-encryptionconfiguration.html#cfn-athena-workgroup-encryptionconfiguration-encryptionoption).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encryption_option: ::Value<String>,
        /// Property [`KmsKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-workgroup-encryptionconfiguration.html#cfn-athena-workgroup-encryptionconfiguration-kmskey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kms_key: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EncryptionConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionOption", &self.encryption_option)?;
            if let Some(ref kms_key) = self.kms_key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKey", kms_key)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EncryptionConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EncryptionConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EncryptionConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EncryptionConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut encryption_option: Option<::Value<String>> = None;
                    let mut kms_key: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EncryptionOption" => {
                                encryption_option = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KmsKey" => {
                                kms_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EncryptionConfiguration {
                        encryption_option: encryption_option.ok_or(::serde::de::Error::missing_field("EncryptionOption"))?,
                        kms_key: kms_key,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Athena::WorkGroup.EngineVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-workgroup-engineversion.html) property type.
    #[derive(Debug, Default)]
    pub struct EngineVersion {
        /// Property [`EffectiveEngineVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-workgroup-engineversion.html#cfn-athena-workgroup-engineversion-effectiveengineversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub effective_engine_version: Option<::Value<String>>,
        /// Property [`SelectedEngineVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-workgroup-engineversion.html#cfn-athena-workgroup-engineversion-selectedengineversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub selected_engine_version: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EngineVersion {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref effective_engine_version) = self.effective_engine_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EffectiveEngineVersion", effective_engine_version)?;
            }
            if let Some(ref selected_engine_version) = self.selected_engine_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SelectedEngineVersion", selected_engine_version)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EngineVersion {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EngineVersion, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EngineVersion;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EngineVersion")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut effective_engine_version: Option<::Value<String>> = None;
                    let mut selected_engine_version: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EffectiveEngineVersion" => {
                                effective_engine_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SelectedEngineVersion" => {
                                selected_engine_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EngineVersion {
                        effective_engine_version: effective_engine_version,
                        selected_engine_version: selected_engine_version,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Athena::WorkGroup.ResultConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-workgroup-resultconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ResultConfiguration {
        /// Property [`AclConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-workgroup-resultconfiguration.html#cfn-athena-workgroup-resultconfiguration-aclconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub acl_configuration: Option<::Value<AclConfiguration>>,
        /// Property [`EncryptionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-workgroup-resultconfiguration.html#cfn-athena-workgroup-resultconfiguration-encryptionconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encryption_configuration: Option<::Value<EncryptionConfiguration>>,
        /// Property [`ExpectedBucketOwner`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-workgroup-resultconfiguration.html#cfn-athena-workgroup-resultconfiguration-expectedbucketowner).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub expected_bucket_owner: Option<::Value<String>>,
        /// Property [`OutputLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-workgroup-resultconfiguration.html#cfn-athena-workgroup-resultconfiguration-outputlocation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub output_location: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ResultConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref acl_configuration) = self.acl_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AclConfiguration", acl_configuration)?;
            }
            if let Some(ref encryption_configuration) = self.encryption_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionConfiguration", encryption_configuration)?;
            }
            if let Some(ref expected_bucket_owner) = self.expected_bucket_owner {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExpectedBucketOwner", expected_bucket_owner)?;
            }
            if let Some(ref output_location) = self.output_location {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutputLocation", output_location)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResultConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResultConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResultConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResultConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut acl_configuration: Option<::Value<AclConfiguration>> = None;
                    let mut encryption_configuration: Option<::Value<EncryptionConfiguration>> = None;
                    let mut expected_bucket_owner: Option<::Value<String>> = None;
                    let mut output_location: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AclConfiguration" => {
                                acl_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EncryptionConfiguration" => {
                                encryption_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExpectedBucketOwner" => {
                                expected_bucket_owner = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OutputLocation" => {
                                output_location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResultConfiguration {
                        acl_configuration: acl_configuration,
                        encryption_configuration: encryption_configuration,
                        expected_bucket_owner: expected_bucket_owner,
                        output_location: output_location,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Athena::WorkGroup.WorkGroupConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-workgroup-workgroupconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct WorkGroupConfiguration {
        /// Property [`AdditionalConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-workgroup-workgroupconfiguration.html#cfn-athena-workgroup-workgroupconfiguration-additionalconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub additional_configuration: Option<::Value<String>>,
        /// Property [`BytesScannedCutoffPerQuery`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-workgroup-workgroupconfiguration.html#cfn-athena-workgroup-workgroupconfiguration-bytesscannedcutoffperquery).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bytes_scanned_cutoff_per_query: Option<::Value<u32>>,
        /// Property [`CustomerContentEncryptionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-workgroup-workgroupconfiguration.html#cfn-athena-workgroup-workgroupconfiguration-customercontentencryptionconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub customer_content_encryption_configuration: Option<::Value<CustomerContentEncryptionConfiguration>>,
        /// Property [`EnforceWorkGroupConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-workgroup-workgroupconfiguration.html#cfn-athena-workgroup-workgroupconfiguration-enforceworkgroupconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enforce_work_group_configuration: Option<::Value<bool>>,
        /// Property [`EngineVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-workgroup-workgroupconfiguration.html#cfn-athena-workgroup-workgroupconfiguration-engineversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub engine_version: Option<::Value<EngineVersion>>,
        /// Property [`ExecutionRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-workgroup-workgroupconfiguration.html#cfn-athena-workgroup-workgroupconfiguration-executionrole).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub execution_role: Option<::Value<String>>,
        /// Property [`PublishCloudWatchMetricsEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-workgroup-workgroupconfiguration.html#cfn-athena-workgroup-workgroupconfiguration-publishcloudwatchmetricsenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub publish_cloud_watch_metrics_enabled: Option<::Value<bool>>,
        /// Property [`RequesterPaysEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-workgroup-workgroupconfiguration.html#cfn-athena-workgroup-workgroupconfiguration-requesterpaysenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub requester_pays_enabled: Option<::Value<bool>>,
        /// Property [`ResultConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-workgroup-workgroupconfiguration.html#cfn-athena-workgroup-workgroupconfiguration-resultconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub result_configuration: Option<::Value<ResultConfiguration>>,
    }

    impl ::codec::SerializeValue for WorkGroupConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref additional_configuration) = self.additional_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdditionalConfiguration", additional_configuration)?;
            }
            if let Some(ref bytes_scanned_cutoff_per_query) = self.bytes_scanned_cutoff_per_query {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BytesScannedCutoffPerQuery", bytes_scanned_cutoff_per_query)?;
            }
            if let Some(ref customer_content_encryption_configuration) = self.customer_content_encryption_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomerContentEncryptionConfiguration", customer_content_encryption_configuration)?;
            }
            if let Some(ref enforce_work_group_configuration) = self.enforce_work_group_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnforceWorkGroupConfiguration", enforce_work_group_configuration)?;
            }
            if let Some(ref engine_version) = self.engine_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EngineVersion", engine_version)?;
            }
            if let Some(ref execution_role) = self.execution_role {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExecutionRole", execution_role)?;
            }
            if let Some(ref publish_cloud_watch_metrics_enabled) = self.publish_cloud_watch_metrics_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PublishCloudWatchMetricsEnabled", publish_cloud_watch_metrics_enabled)?;
            }
            if let Some(ref requester_pays_enabled) = self.requester_pays_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequesterPaysEnabled", requester_pays_enabled)?;
            }
            if let Some(ref result_configuration) = self.result_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResultConfiguration", result_configuration)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for WorkGroupConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<WorkGroupConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = WorkGroupConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type WorkGroupConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut additional_configuration: Option<::Value<String>> = None;
                    let mut bytes_scanned_cutoff_per_query: Option<::Value<u32>> = None;
                    let mut customer_content_encryption_configuration: Option<::Value<CustomerContentEncryptionConfiguration>> = None;
                    let mut enforce_work_group_configuration: Option<::Value<bool>> = None;
                    let mut engine_version: Option<::Value<EngineVersion>> = None;
                    let mut execution_role: Option<::Value<String>> = None;
                    let mut publish_cloud_watch_metrics_enabled: Option<::Value<bool>> = None;
                    let mut requester_pays_enabled: Option<::Value<bool>> = None;
                    let mut result_configuration: Option<::Value<ResultConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AdditionalConfiguration" => {
                                additional_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BytesScannedCutoffPerQuery" => {
                                bytes_scanned_cutoff_per_query = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CustomerContentEncryptionConfiguration" => {
                                customer_content_encryption_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnforceWorkGroupConfiguration" => {
                                enforce_work_group_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EngineVersion" => {
                                engine_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExecutionRole" => {
                                execution_role = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PublishCloudWatchMetricsEnabled" => {
                                publish_cloud_watch_metrics_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RequesterPaysEnabled" => {
                                requester_pays_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResultConfiguration" => {
                                result_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(WorkGroupConfiguration {
                        additional_configuration: additional_configuration,
                        bytes_scanned_cutoff_per_query: bytes_scanned_cutoff_per_query,
                        customer_content_encryption_configuration: customer_content_encryption_configuration,
                        enforce_work_group_configuration: enforce_work_group_configuration,
                        engine_version: engine_version,
                        execution_role: execution_role,
                        publish_cloud_watch_metrics_enabled: publish_cloud_watch_metrics_enabled,
                        requester_pays_enabled: requester_pays_enabled,
                        result_configuration: result_configuration,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
