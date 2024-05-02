//! Types for the `LakeFormation` service.

/// The [`AWS::LakeFormation::DataCellsFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-datacellsfilter.html) resource type.
#[derive(Debug, Default)]
pub struct DataCellsFilter {
    properties: DataCellsFilterProperties
}

/// Properties for the `DataCellsFilter` resource.
#[derive(Debug, Default)]
pub struct DataCellsFilterProperties {
    /// Property [`ColumnNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-datacellsfilter.html#cfn-lakeformation-datacellsfilter-columnnames).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub column_names: Option<::ValueList<String>>,
    /// Property [`ColumnWildcard`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-datacellsfilter.html#cfn-lakeformation-datacellsfilter-columnwildcard).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub column_wildcard: Option<::Value<self::data_cells_filter::ColumnWildcard>>,
    /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-datacellsfilter.html#cfn-lakeformation-datacellsfilter-databasename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub database_name: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-datacellsfilter.html#cfn-lakeformation-datacellsfilter-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`RowFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-datacellsfilter.html#cfn-lakeformation-datacellsfilter-rowfilter).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub row_filter: Option<::Value<self::data_cells_filter::RowFilter>>,
    /// Property [`TableCatalogId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-datacellsfilter.html#cfn-lakeformation-datacellsfilter-tablecatalogid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub table_catalog_id: ::Value<String>,
    /// Property [`TableName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-datacellsfilter.html#cfn-lakeformation-datacellsfilter-tablename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub table_name: ::Value<String>,
}

impl ::serde::Serialize for DataCellsFilterProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref column_names) = self.column_names {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColumnNames", column_names)?;
        }
        if let Some(ref column_wildcard) = self.column_wildcard {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColumnWildcard", column_wildcard)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", &self.database_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref row_filter) = self.row_filter {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RowFilter", row_filter)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableCatalogId", &self.table_catalog_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableName", &self.table_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DataCellsFilterProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DataCellsFilterProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DataCellsFilterProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DataCellsFilterProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut column_names: Option<::ValueList<String>> = None;
                let mut column_wildcard: Option<::Value<self::data_cells_filter::ColumnWildcard>> = None;
                let mut database_name: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut row_filter: Option<::Value<self::data_cells_filter::RowFilter>> = None;
                let mut table_catalog_id: Option<::Value<String>> = None;
                let mut table_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ColumnNames" => {
                            column_names = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ColumnWildcard" => {
                            column_wildcard = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DatabaseName" => {
                            database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RowFilter" => {
                            row_filter = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TableCatalogId" => {
                            table_catalog_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TableName" => {
                            table_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DataCellsFilterProperties {
                    column_names: column_names,
                    column_wildcard: column_wildcard,
                    database_name: database_name.ok_or(::serde::de::Error::missing_field("DatabaseName"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    row_filter: row_filter,
                    table_catalog_id: table_catalog_id.ok_or(::serde::de::Error::missing_field("TableCatalogId"))?,
                    table_name: table_name.ok_or(::serde::de::Error::missing_field("TableName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DataCellsFilter {
    type Properties = DataCellsFilterProperties;
    const TYPE: &'static str = "AWS::LakeFormation::DataCellsFilter";
    fn properties(&self) -> &DataCellsFilterProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DataCellsFilterProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DataCellsFilter {}

impl From<DataCellsFilterProperties> for DataCellsFilter {
    fn from(properties: DataCellsFilterProperties) -> DataCellsFilter {
        DataCellsFilter { properties }
    }
}

/// The [`AWS::LakeFormation::DataLakeSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-datalakesettings.html) resource type.
#[derive(Debug, Default)]
pub struct DataLakeSettings {
    properties: DataLakeSettingsProperties
}

/// Properties for the `DataLakeSettings` resource.
#[derive(Debug, Default)]
pub struct DataLakeSettingsProperties {
    /// Property [`Admins`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-datalakesettings.html#cfn-lakeformation-datalakesettings-admins).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub admins: Option<::Value<self::data_lake_settings::Admins>>,
    /// Property [`AllowExternalDataFiltering`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-datalakesettings.html#cfn-lakeformation-datalakesettings-allowexternaldatafiltering).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub allow_external_data_filtering: Option<::Value<bool>>,
    /// Property [`AllowFullTableExternalDataAccess`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-datalakesettings.html#cfn-lakeformation-datalakesettings-allowfulltableexternaldataaccess).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub allow_full_table_external_data_access: Option<::Value<bool>>,
    /// Property [`AuthorizedSessionTagValueList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-datalakesettings.html#cfn-lakeformation-datalakesettings-authorizedsessiontagvaluelist).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub authorized_session_tag_value_list: Option<::ValueList<String>>,
    /// Property [`CreateDatabaseDefaultPermissions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-datalakesettings.html#cfn-lakeformation-datalakesettings-createdatabasedefaultpermissions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub create_database_default_permissions: Option<::Value<self::data_lake_settings::CreateDatabaseDefaultPermissions>>,
    /// Property [`CreateTableDefaultPermissions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-datalakesettings.html#cfn-lakeformation-datalakesettings-createtabledefaultpermissions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub create_table_default_permissions: Option<::Value<self::data_lake_settings::CreateTableDefaultPermissions>>,
    /// Property [`ExternalDataFilteringAllowList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-datalakesettings.html#cfn-lakeformation-datalakesettings-externaldatafilteringallowlist).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub external_data_filtering_allow_list: Option<::Value<self::data_lake_settings::ExternalDataFilteringAllowList>>,
    /// Property [`MutationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-datalakesettings.html#cfn-lakeformation-datalakesettings-mutationtype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub mutation_type: Option<::Value<String>>,
    /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-datalakesettings.html#cfn-lakeformation-datalakesettings-parameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub parameters: Option<::Value<::json::Value>>,
    /// Property [`TrustedResourceOwners`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-datalakesettings.html#cfn-lakeformation-datalakesettings-trustedresourceowners).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub trusted_resource_owners: Option<::ValueList<String>>,
}

impl ::serde::Serialize for DataLakeSettingsProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref admins) = self.admins {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Admins", admins)?;
        }
        if let Some(ref allow_external_data_filtering) = self.allow_external_data_filtering {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowExternalDataFiltering", allow_external_data_filtering)?;
        }
        if let Some(ref allow_full_table_external_data_access) = self.allow_full_table_external_data_access {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowFullTableExternalDataAccess", allow_full_table_external_data_access)?;
        }
        if let Some(ref authorized_session_tag_value_list) = self.authorized_session_tag_value_list {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthorizedSessionTagValueList", authorized_session_tag_value_list)?;
        }
        if let Some(ref create_database_default_permissions) = self.create_database_default_permissions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CreateDatabaseDefaultPermissions", create_database_default_permissions)?;
        }
        if let Some(ref create_table_default_permissions) = self.create_table_default_permissions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CreateTableDefaultPermissions", create_table_default_permissions)?;
        }
        if let Some(ref external_data_filtering_allow_list) = self.external_data_filtering_allow_list {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExternalDataFilteringAllowList", external_data_filtering_allow_list)?;
        }
        if let Some(ref mutation_type) = self.mutation_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MutationType", mutation_type)?;
        }
        if let Some(ref parameters) = self.parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", parameters)?;
        }
        if let Some(ref trusted_resource_owners) = self.trusted_resource_owners {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TrustedResourceOwners", trusted_resource_owners)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DataLakeSettingsProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DataLakeSettingsProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DataLakeSettingsProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DataLakeSettingsProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut admins: Option<::Value<self::data_lake_settings::Admins>> = None;
                let mut allow_external_data_filtering: Option<::Value<bool>> = None;
                let mut allow_full_table_external_data_access: Option<::Value<bool>> = None;
                let mut authorized_session_tag_value_list: Option<::ValueList<String>> = None;
                let mut create_database_default_permissions: Option<::Value<self::data_lake_settings::CreateDatabaseDefaultPermissions>> = None;
                let mut create_table_default_permissions: Option<::Value<self::data_lake_settings::CreateTableDefaultPermissions>> = None;
                let mut external_data_filtering_allow_list: Option<::Value<self::data_lake_settings::ExternalDataFilteringAllowList>> = None;
                let mut mutation_type: Option<::Value<String>> = None;
                let mut parameters: Option<::Value<::json::Value>> = None;
                let mut trusted_resource_owners: Option<::ValueList<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Admins" => {
                            admins = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AllowExternalDataFiltering" => {
                            allow_external_data_filtering = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AllowFullTableExternalDataAccess" => {
                            allow_full_table_external_data_access = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AuthorizedSessionTagValueList" => {
                            authorized_session_tag_value_list = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CreateDatabaseDefaultPermissions" => {
                            create_database_default_permissions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CreateTableDefaultPermissions" => {
                            create_table_default_permissions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ExternalDataFilteringAllowList" => {
                            external_data_filtering_allow_list = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MutationType" => {
                            mutation_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Parameters" => {
                            parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TrustedResourceOwners" => {
                            trusted_resource_owners = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DataLakeSettingsProperties {
                    admins: admins,
                    allow_external_data_filtering: allow_external_data_filtering,
                    allow_full_table_external_data_access: allow_full_table_external_data_access,
                    authorized_session_tag_value_list: authorized_session_tag_value_list,
                    create_database_default_permissions: create_database_default_permissions,
                    create_table_default_permissions: create_table_default_permissions,
                    external_data_filtering_allow_list: external_data_filtering_allow_list,
                    mutation_type: mutation_type,
                    parameters: parameters,
                    trusted_resource_owners: trusted_resource_owners,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DataLakeSettings {
    type Properties = DataLakeSettingsProperties;
    const TYPE: &'static str = "AWS::LakeFormation::DataLakeSettings";
    fn properties(&self) -> &DataLakeSettingsProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DataLakeSettingsProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DataLakeSettings {}

impl From<DataLakeSettingsProperties> for DataLakeSettings {
    fn from(properties: DataLakeSettingsProperties) -> DataLakeSettings {
        DataLakeSettings { properties }
    }
}

/// The [`AWS::LakeFormation::Permissions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-permissions.html) resource type.
#[derive(Debug, Default)]
pub struct Permissions {
    properties: PermissionsProperties
}

/// Properties for the `Permissions` resource.
#[derive(Debug, Default)]
pub struct PermissionsProperties {
    /// Property [`DataLakePrincipal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-permissions.html#cfn-lakeformation-permissions-datalakeprincipal).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub data_lake_principal: ::Value<self::permissions::DataLakePrincipal>,
    /// Property [`Permissions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-permissions.html#cfn-lakeformation-permissions-permissions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub permissions: Option<::ValueList<String>>,
    /// Property [`PermissionsWithGrantOption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-permissions.html#cfn-lakeformation-permissions-permissionswithgrantoption).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub permissions_with_grant_option: Option<::ValueList<String>>,
    /// Property [`Resource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-permissions.html#cfn-lakeformation-permissions-resource).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub resource: ::Value<self::permissions::Resource>,
}

impl ::serde::Serialize for PermissionsProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataLakePrincipal", &self.data_lake_principal)?;
        if let Some(ref permissions) = self.permissions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Permissions", permissions)?;
        }
        if let Some(ref permissions_with_grant_option) = self.permissions_with_grant_option {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PermissionsWithGrantOption", permissions_with_grant_option)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Resource", &self.resource)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PermissionsProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PermissionsProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PermissionsProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PermissionsProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut data_lake_principal: Option<::Value<self::permissions::DataLakePrincipal>> = None;
                let mut permissions: Option<::ValueList<String>> = None;
                let mut permissions_with_grant_option: Option<::ValueList<String>> = None;
                let mut resource: Option<::Value<self::permissions::Resource>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DataLakePrincipal" => {
                            data_lake_principal = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Permissions" => {
                            permissions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PermissionsWithGrantOption" => {
                            permissions_with_grant_option = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Resource" => {
                            resource = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PermissionsProperties {
                    data_lake_principal: data_lake_principal.ok_or(::serde::de::Error::missing_field("DataLakePrincipal"))?,
                    permissions: permissions,
                    permissions_with_grant_option: permissions_with_grant_option,
                    resource: resource.ok_or(::serde::de::Error::missing_field("Resource"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Permissions {
    type Properties = PermissionsProperties;
    const TYPE: &'static str = "AWS::LakeFormation::Permissions";
    fn properties(&self) -> &PermissionsProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PermissionsProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Permissions {}

impl From<PermissionsProperties> for Permissions {
    fn from(properties: PermissionsProperties) -> Permissions {
        Permissions { properties }
    }
}

/// The [`AWS::LakeFormation::PrincipalPermissions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-principalpermissions.html) resource type.
#[derive(Debug, Default)]
pub struct PrincipalPermissions {
    properties: PrincipalPermissionsProperties
}

/// Properties for the `PrincipalPermissions` resource.
#[derive(Debug, Default)]
pub struct PrincipalPermissionsProperties {
    /// Property [`Catalog`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-principalpermissions.html#cfn-lakeformation-principalpermissions-catalog).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub catalog: Option<::Value<String>>,
    /// Property [`Permissions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-principalpermissions.html#cfn-lakeformation-principalpermissions-permissions).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub permissions: ::ValueList<String>,
    /// Property [`PermissionsWithGrantOption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-principalpermissions.html#cfn-lakeformation-principalpermissions-permissionswithgrantoption).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub permissions_with_grant_option: ::ValueList<String>,
    /// Property [`Principal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-principalpermissions.html#cfn-lakeformation-principalpermissions-principal).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub principal: ::Value<self::principal_permissions::DataLakePrincipal>,
    /// Property [`Resource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-principalpermissions.html#cfn-lakeformation-principalpermissions-resource).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub resource: ::Value<self::principal_permissions::Resource>,
}

impl ::serde::Serialize for PrincipalPermissionsProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref catalog) = self.catalog {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Catalog", catalog)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Permissions", &self.permissions)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PermissionsWithGrantOption", &self.permissions_with_grant_option)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Principal", &self.principal)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Resource", &self.resource)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PrincipalPermissionsProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PrincipalPermissionsProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PrincipalPermissionsProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PrincipalPermissionsProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut catalog: Option<::Value<String>> = None;
                let mut permissions: Option<::ValueList<String>> = None;
                let mut permissions_with_grant_option: Option<::ValueList<String>> = None;
                let mut principal: Option<::Value<self::principal_permissions::DataLakePrincipal>> = None;
                let mut resource: Option<::Value<self::principal_permissions::Resource>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Catalog" => {
                            catalog = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Permissions" => {
                            permissions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PermissionsWithGrantOption" => {
                            permissions_with_grant_option = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Principal" => {
                            principal = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Resource" => {
                            resource = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PrincipalPermissionsProperties {
                    catalog: catalog,
                    permissions: permissions.ok_or(::serde::de::Error::missing_field("Permissions"))?,
                    permissions_with_grant_option: permissions_with_grant_option.ok_or(::serde::de::Error::missing_field("PermissionsWithGrantOption"))?,
                    principal: principal.ok_or(::serde::de::Error::missing_field("Principal"))?,
                    resource: resource.ok_or(::serde::de::Error::missing_field("Resource"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for PrincipalPermissions {
    type Properties = PrincipalPermissionsProperties;
    const TYPE: &'static str = "AWS::LakeFormation::PrincipalPermissions";
    fn properties(&self) -> &PrincipalPermissionsProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PrincipalPermissionsProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for PrincipalPermissions {}

impl From<PrincipalPermissionsProperties> for PrincipalPermissions {
    fn from(properties: PrincipalPermissionsProperties) -> PrincipalPermissions {
        PrincipalPermissions { properties }
    }
}

/// The [`AWS::LakeFormation::Resource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-resource.html) resource type.
#[derive(Debug, Default)]
pub struct Resource {
    properties: ResourceProperties
}

/// Properties for the `Resource` resource.
#[derive(Debug, Default)]
pub struct ResourceProperties {
    /// Property [`HybridAccessEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-resource.html#cfn-lakeformation-resource-hybridaccessenabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub hybrid_access_enabled: Option<::Value<bool>>,
    /// Property [`ResourceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-resource.html#cfn-lakeformation-resource-resourcearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub resource_arn: ::Value<String>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-resource.html#cfn-lakeformation-resource-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: Option<::Value<String>>,
    /// Property [`UseServiceLinkedRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-resource.html#cfn-lakeformation-resource-useservicelinkedrole).
    ///
    /// Update type: _Conditional_.
    /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
    /// For more information, see the relevant resource type documentation.
    pub use_service_linked_role: ::Value<bool>,
    /// Property [`WithFederation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-resource.html#cfn-lakeformation-resource-withfederation).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub with_federation: Option<::Value<bool>>,
}

impl ::serde::Serialize for ResourceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref hybrid_access_enabled) = self.hybrid_access_enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HybridAccessEnabled", hybrid_access_enabled)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceArn", &self.resource_arn)?;
        if let Some(ref role_arn) = self.role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", role_arn)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "UseServiceLinkedRole", &self.use_service_linked_role)?;
        if let Some(ref with_federation) = self.with_federation {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WithFederation", with_federation)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ResourceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ResourceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut hybrid_access_enabled: Option<::Value<bool>> = None;
                let mut resource_arn: Option<::Value<String>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut use_service_linked_role: Option<::Value<bool>> = None;
                let mut with_federation: Option<::Value<bool>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "HybridAccessEnabled" => {
                            hybrid_access_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourceArn" => {
                            resource_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UseServiceLinkedRole" => {
                            use_service_linked_role = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "WithFederation" => {
                            with_federation = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ResourceProperties {
                    hybrid_access_enabled: hybrid_access_enabled,
                    resource_arn: resource_arn.ok_or(::serde::de::Error::missing_field("ResourceArn"))?,
                    role_arn: role_arn,
                    use_service_linked_role: use_service_linked_role.ok_or(::serde::de::Error::missing_field("UseServiceLinkedRole"))?,
                    with_federation: with_federation,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Resource {
    type Properties = ResourceProperties;
    const TYPE: &'static str = "AWS::LakeFormation::Resource";
    fn properties(&self) -> &ResourceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ResourceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Resource {}

impl From<ResourceProperties> for Resource {
    fn from(properties: ResourceProperties) -> Resource {
        Resource { properties }
    }
}

/// The [`AWS::LakeFormation::Tag`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-tag.html) resource type.
#[derive(Debug, Default)]
pub struct Tag {
    properties: TagProperties
}

/// Properties for the `Tag` resource.
#[derive(Debug, Default)]
pub struct TagProperties {
    /// Property [`CatalogId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-tag.html#cfn-lakeformation-tag-catalogid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub catalog_id: Option<::Value<String>>,
    /// Property [`TagKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-tag.html#cfn-lakeformation-tag-tagkey).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tag_key: ::Value<String>,
    /// Property [`TagValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-tag.html#cfn-lakeformation-tag-tagvalues).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tag_values: ::ValueList<String>,
}

impl ::serde::Serialize for TagProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref catalog_id) = self.catalog_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CatalogId", catalog_id)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TagKey", &self.tag_key)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TagValues", &self.tag_values)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for TagProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<TagProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TagProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type TagProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut catalog_id: Option<::Value<String>> = None;
                let mut tag_key: Option<::Value<String>> = None;
                let mut tag_values: Option<::ValueList<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CatalogId" => {
                            catalog_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TagKey" => {
                            tag_key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TagValues" => {
                            tag_values = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(TagProperties {
                    catalog_id: catalog_id,
                    tag_key: tag_key.ok_or(::serde::de::Error::missing_field("TagKey"))?,
                    tag_values: tag_values.ok_or(::serde::de::Error::missing_field("TagValues"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Tag {
    type Properties = TagProperties;
    const TYPE: &'static str = "AWS::LakeFormation::Tag";
    fn properties(&self) -> &TagProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TagProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Tag {}

impl From<TagProperties> for Tag {
    fn from(properties: TagProperties) -> Tag {
        Tag { properties }
    }
}

/// The [`AWS::LakeFormation::TagAssociation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-tagassociation.html) resource type.
#[derive(Debug, Default)]
pub struct TagAssociation {
    properties: TagAssociationProperties
}

/// Properties for the `TagAssociation` resource.
#[derive(Debug, Default)]
pub struct TagAssociationProperties {
    /// Property [`LFTags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-tagassociation.html#cfn-lakeformation-tagassociation-lftags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub lf_tags: ::ValueList<self::tag_association::LFTagPair>,
    /// Property [`Resource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lakeformation-tagassociation.html#cfn-lakeformation-tagassociation-resource).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub resource: ::Value<self::tag_association::Resource>,
}

impl ::serde::Serialize for TagAssociationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LFTags", &self.lf_tags)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Resource", &self.resource)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for TagAssociationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<TagAssociationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TagAssociationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type TagAssociationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut lf_tags: Option<::ValueList<self::tag_association::LFTagPair>> = None;
                let mut resource: Option<::Value<self::tag_association::Resource>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "LFTags" => {
                            lf_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Resource" => {
                            resource = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(TagAssociationProperties {
                    lf_tags: lf_tags.ok_or(::serde::de::Error::missing_field("LFTags"))?,
                    resource: resource.ok_or(::serde::de::Error::missing_field("Resource"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for TagAssociation {
    type Properties = TagAssociationProperties;
    const TYPE: &'static str = "AWS::LakeFormation::TagAssociation";
    fn properties(&self) -> &TagAssociationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TagAssociationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for TagAssociation {}

impl From<TagAssociationProperties> for TagAssociation {
    fn from(properties: TagAssociationProperties) -> TagAssociation {
        TagAssociation { properties }
    }
}

pub mod data_cells_filter {
    //! Property types for the `DataCellsFilter` resource.

    /// The [`AWS::LakeFormation::DataCellsFilter.ColumnWildcard`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-datacellsfilter-columnwildcard.html) property type.
    #[derive(Debug, Default)]
    pub struct ColumnWildcard {
        /// Property [`ExcludedColumnNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-datacellsfilter-columnwildcard.html#cfn-lakeformation-datacellsfilter-columnwildcard-excludedcolumnnames).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub excluded_column_names: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for ColumnWildcard {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref excluded_column_names) = self.excluded_column_names {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExcludedColumnNames", excluded_column_names)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ColumnWildcard {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ColumnWildcard, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ColumnWildcard;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ColumnWildcard")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut excluded_column_names: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ExcludedColumnNames" => {
                                excluded_column_names = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ColumnWildcard {
                        excluded_column_names: excluded_column_names,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LakeFormation::DataCellsFilter.RowFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-datacellsfilter-rowfilter.html) property type.
    #[derive(Debug, Default)]
    pub struct RowFilter {
        /// Property [`AllRowsWildcard`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-datacellsfilter-rowfilter.html#cfn-lakeformation-datacellsfilter-rowfilter-allrowswildcard).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub all_rows_wildcard: Option<::Value<::json::Value>>,
        /// Property [`FilterExpression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-datacellsfilter-rowfilter.html#cfn-lakeformation-datacellsfilter-rowfilter-filterexpression).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub filter_expression: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for RowFilter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref all_rows_wildcard) = self.all_rows_wildcard {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllRowsWildcard", all_rows_wildcard)?;
            }
            if let Some(ref filter_expression) = self.filter_expression {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FilterExpression", filter_expression)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RowFilter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RowFilter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RowFilter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RowFilter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut all_rows_wildcard: Option<::Value<::json::Value>> = None;
                    let mut filter_expression: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllRowsWildcard" => {
                                all_rows_wildcard = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FilterExpression" => {
                                filter_expression = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RowFilter {
                        all_rows_wildcard: all_rows_wildcard,
                        filter_expression: filter_expression,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod data_lake_settings {
    //! Property types for the `DataLakeSettings` resource.

    /// The [`AWS::LakeFormation::DataLakeSettings.Admins`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-datalakesettings-admins.html) property type.
    #[derive(Debug, Default)]
    pub struct Admins {
    }

    impl ::codec::SerializeValue for Admins {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Admins {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Admins, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Admins;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Admins")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(Admins {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LakeFormation::DataLakeSettings.CreateDatabaseDefaultPermissions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-datalakesettings-createdatabasedefaultpermissions.html) property type.
    #[derive(Debug, Default)]
    pub struct CreateDatabaseDefaultPermissions {
    }

    impl ::codec::SerializeValue for CreateDatabaseDefaultPermissions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CreateDatabaseDefaultPermissions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CreateDatabaseDefaultPermissions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CreateDatabaseDefaultPermissions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CreateDatabaseDefaultPermissions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(CreateDatabaseDefaultPermissions {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LakeFormation::DataLakeSettings.CreateTableDefaultPermissions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-datalakesettings-createtabledefaultpermissions.html) property type.
    #[derive(Debug, Default)]
    pub struct CreateTableDefaultPermissions {
    }

    impl ::codec::SerializeValue for CreateTableDefaultPermissions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CreateTableDefaultPermissions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CreateTableDefaultPermissions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CreateTableDefaultPermissions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CreateTableDefaultPermissions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(CreateTableDefaultPermissions {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LakeFormation::DataLakeSettings.DataLakePrincipal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-datalakesettings-datalakeprincipal.html) property type.
    #[derive(Debug, Default)]
    pub struct DataLakePrincipal {
        /// Property [`DataLakePrincipalIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-datalakesettings-datalakeprincipal.html#cfn-lakeformation-datalakesettings-datalakeprincipal-datalakeprincipalidentifier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_lake_principal_identifier: ::Value<String>,
    }

    impl ::codec::SerializeValue for DataLakePrincipal {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataLakePrincipalIdentifier", &self.data_lake_principal_identifier)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataLakePrincipal {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataLakePrincipal, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataLakePrincipal;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataLakePrincipal")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_lake_principal_identifier: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataLakePrincipalIdentifier" => {
                                data_lake_principal_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataLakePrincipal {
                        data_lake_principal_identifier: data_lake_principal_identifier.ok_or(::serde::de::Error::missing_field("DataLakePrincipalIdentifier"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LakeFormation::DataLakeSettings.ExternalDataFilteringAllowList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-datalakesettings-externaldatafilteringallowlist.html) property type.
    #[derive(Debug, Default)]
    pub struct ExternalDataFilteringAllowList {
    }

    impl ::codec::SerializeValue for ExternalDataFilteringAllowList {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ExternalDataFilteringAllowList {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ExternalDataFilteringAllowList, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ExternalDataFilteringAllowList;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ExternalDataFilteringAllowList")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(ExternalDataFilteringAllowList {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LakeFormation::DataLakeSettings.PrincipalPermissions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-datalakesettings-principalpermissions.html) property type.
    #[derive(Debug, Default)]
    pub struct PrincipalPermissions {
        /// Property [`Permissions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-datalakesettings-principalpermissions.html#cfn-lakeformation-datalakesettings-principalpermissions-permissions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub permissions: ::ValueList<String>,
        /// Property [`Principal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-datalakesettings-principalpermissions.html#cfn-lakeformation-datalakesettings-principalpermissions-principal).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub principal: ::Value<DataLakePrincipal>,
    }

    impl ::codec::SerializeValue for PrincipalPermissions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Permissions", &self.permissions)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Principal", &self.principal)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PrincipalPermissions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PrincipalPermissions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PrincipalPermissions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PrincipalPermissions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut permissions: Option<::ValueList<String>> = None;
                    let mut principal: Option<::Value<DataLakePrincipal>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Permissions" => {
                                permissions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Principal" => {
                                principal = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PrincipalPermissions {
                        permissions: permissions.ok_or(::serde::de::Error::missing_field("Permissions"))?,
                        principal: principal.ok_or(::serde::de::Error::missing_field("Principal"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod permissions {
    //! Property types for the `Permissions` resource.

    /// The [`AWS::LakeFormation::Permissions.ColumnWildcard`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-columnwildcard.html) property type.
    #[derive(Debug, Default)]
    pub struct ColumnWildcard {
        /// Property [`ExcludedColumnNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-columnwildcard.html#cfn-lakeformation-permissions-columnwildcard-excludedcolumnnames).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub excluded_column_names: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for ColumnWildcard {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref excluded_column_names) = self.excluded_column_names {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExcludedColumnNames", excluded_column_names)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ColumnWildcard {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ColumnWildcard, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ColumnWildcard;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ColumnWildcard")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut excluded_column_names: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ExcludedColumnNames" => {
                                excluded_column_names = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ColumnWildcard {
                        excluded_column_names: excluded_column_names,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LakeFormation::Permissions.DataLakePrincipal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-datalakeprincipal.html) property type.
    #[derive(Debug, Default)]
    pub struct DataLakePrincipal {
        /// Property [`DataLakePrincipalIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-datalakeprincipal.html#cfn-lakeformation-permissions-datalakeprincipal-datalakeprincipalidentifier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_lake_principal_identifier: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DataLakePrincipal {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref data_lake_principal_identifier) = self.data_lake_principal_identifier {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataLakePrincipalIdentifier", data_lake_principal_identifier)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataLakePrincipal {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataLakePrincipal, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataLakePrincipal;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataLakePrincipal")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_lake_principal_identifier: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataLakePrincipalIdentifier" => {
                                data_lake_principal_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataLakePrincipal {
                        data_lake_principal_identifier: data_lake_principal_identifier,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LakeFormation::Permissions.DataLocationResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-datalocationresource.html) property type.
    #[derive(Debug, Default)]
    pub struct DataLocationResource {
        /// Property [`CatalogId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-datalocationresource.html#cfn-lakeformation-permissions-datalocationresource-catalogid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub catalog_id: Option<::Value<String>>,
        /// Property [`S3Resource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-datalocationresource.html#cfn-lakeformation-permissions-datalocationresource-s3resource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_resource: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DataLocationResource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref catalog_id) = self.catalog_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CatalogId", catalog_id)?;
            }
            if let Some(ref s3_resource) = self.s3_resource {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Resource", s3_resource)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataLocationResource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataLocationResource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataLocationResource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataLocationResource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut catalog_id: Option<::Value<String>> = None;
                    let mut s3_resource: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CatalogId" => {
                                catalog_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Resource" => {
                                s3_resource = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataLocationResource {
                        catalog_id: catalog_id,
                        s3_resource: s3_resource,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LakeFormation::Permissions.DatabaseResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-databaseresource.html) property type.
    #[derive(Debug, Default)]
    pub struct DatabaseResource {
        /// Property [`CatalogId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-databaseresource.html#cfn-lakeformation-permissions-databaseresource-catalogid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub catalog_id: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-databaseresource.html#cfn-lakeformation-permissions-databaseresource-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DatabaseResource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref catalog_id) = self.catalog_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CatalogId", catalog_id)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DatabaseResource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DatabaseResource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DatabaseResource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DatabaseResource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut catalog_id: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CatalogId" => {
                                catalog_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DatabaseResource {
                        catalog_id: catalog_id,
                        name: name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LakeFormation::Permissions.Resource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-resource.html) property type.
    #[derive(Debug, Default)]
    pub struct Resource {
        /// Property [`DataLocationResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-resource.html#cfn-lakeformation-permissions-resource-datalocationresource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_location_resource: Option<::Value<DataLocationResource>>,
        /// Property [`DatabaseResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-resource.html#cfn-lakeformation-permissions-resource-databaseresource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_resource: Option<::Value<DatabaseResource>>,
        /// Property [`TableResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-resource.html#cfn-lakeformation-permissions-resource-tableresource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub table_resource: Option<::Value<TableResource>>,
        /// Property [`TableWithColumnsResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-resource.html#cfn-lakeformation-permissions-resource-tablewithcolumnsresource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub table_with_columns_resource: Option<::Value<TableWithColumnsResource>>,
    }

    impl ::codec::SerializeValue for Resource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref data_location_resource) = self.data_location_resource {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataLocationResource", data_location_resource)?;
            }
            if let Some(ref database_resource) = self.database_resource {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseResource", database_resource)?;
            }
            if let Some(ref table_resource) = self.table_resource {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableResource", table_resource)?;
            }
            if let Some(ref table_with_columns_resource) = self.table_with_columns_resource {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableWithColumnsResource", table_with_columns_resource)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Resource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Resource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Resource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Resource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_location_resource: Option<::Value<DataLocationResource>> = None;
                    let mut database_resource: Option<::Value<DatabaseResource>> = None;
                    let mut table_resource: Option<::Value<TableResource>> = None;
                    let mut table_with_columns_resource: Option<::Value<TableWithColumnsResource>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataLocationResource" => {
                                data_location_resource = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabaseResource" => {
                                database_resource = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TableResource" => {
                                table_resource = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TableWithColumnsResource" => {
                                table_with_columns_resource = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Resource {
                        data_location_resource: data_location_resource,
                        database_resource: database_resource,
                        table_resource: table_resource,
                        table_with_columns_resource: table_with_columns_resource,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LakeFormation::Permissions.TableResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-tableresource.html) property type.
    #[derive(Debug, Default)]
    pub struct TableResource {
        /// Property [`CatalogId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-tableresource.html#cfn-lakeformation-permissions-tableresource-catalogid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub catalog_id: Option<::Value<String>>,
        /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-tableresource.html#cfn-lakeformation-permissions-tableresource-databasename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_name: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-tableresource.html#cfn-lakeformation-permissions-tableresource-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`TableWildcard`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-tableresource.html#cfn-lakeformation-permissions-tableresource-tablewildcard).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub table_wildcard: Option<::Value<TableWildcard>>,
    }

    impl ::codec::SerializeValue for TableResource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref catalog_id) = self.catalog_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CatalogId", catalog_id)?;
            }
            if let Some(ref database_name) = self.database_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", database_name)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref table_wildcard) = self.table_wildcard {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableWildcard", table_wildcard)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TableResource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TableResource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TableResource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TableResource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut catalog_id: Option<::Value<String>> = None;
                    let mut database_name: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut table_wildcard: Option<::Value<TableWildcard>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CatalogId" => {
                                catalog_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabaseName" => {
                                database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TableWildcard" => {
                                table_wildcard = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TableResource {
                        catalog_id: catalog_id,
                        database_name: database_name,
                        name: name,
                        table_wildcard: table_wildcard,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LakeFormation::Permissions.TableWildcard`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-tablewildcard.html) property type.
    #[derive(Debug, Default)]
    pub struct TableWildcard {
    }

    impl ::codec::SerializeValue for TableWildcard {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TableWildcard {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TableWildcard, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TableWildcard;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TableWildcard")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(TableWildcard {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LakeFormation::Permissions.TableWithColumnsResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-tablewithcolumnsresource.html) property type.
    #[derive(Debug, Default)]
    pub struct TableWithColumnsResource {
        /// Property [`CatalogId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-tablewithcolumnsresource.html#cfn-lakeformation-permissions-tablewithcolumnsresource-catalogid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub catalog_id: Option<::Value<String>>,
        /// Property [`ColumnNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-tablewithcolumnsresource.html#cfn-lakeformation-permissions-tablewithcolumnsresource-columnnames).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub column_names: Option<::ValueList<String>>,
        /// Property [`ColumnWildcard`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-tablewithcolumnsresource.html#cfn-lakeformation-permissions-tablewithcolumnsresource-columnwildcard).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub column_wildcard: Option<::Value<ColumnWildcard>>,
        /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-tablewithcolumnsresource.html#cfn-lakeformation-permissions-tablewithcolumnsresource-databasename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_name: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-permissions-tablewithcolumnsresource.html#cfn-lakeformation-permissions-tablewithcolumnsresource-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for TableWithColumnsResource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref catalog_id) = self.catalog_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CatalogId", catalog_id)?;
            }
            if let Some(ref column_names) = self.column_names {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColumnNames", column_names)?;
            }
            if let Some(ref column_wildcard) = self.column_wildcard {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColumnWildcard", column_wildcard)?;
            }
            if let Some(ref database_name) = self.database_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", database_name)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TableWithColumnsResource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TableWithColumnsResource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TableWithColumnsResource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TableWithColumnsResource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut catalog_id: Option<::Value<String>> = None;
                    let mut column_names: Option<::ValueList<String>> = None;
                    let mut column_wildcard: Option<::Value<ColumnWildcard>> = None;
                    let mut database_name: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CatalogId" => {
                                catalog_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ColumnNames" => {
                                column_names = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ColumnWildcard" => {
                                column_wildcard = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabaseName" => {
                                database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TableWithColumnsResource {
                        catalog_id: catalog_id,
                        column_names: column_names,
                        column_wildcard: column_wildcard,
                        database_name: database_name,
                        name: name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod principal_permissions {
    //! Property types for the `PrincipalPermissions` resource.

    /// The [`AWS::LakeFormation::PrincipalPermissions.ColumnWildcard`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-columnwildcard.html) property type.
    #[derive(Debug, Default)]
    pub struct ColumnWildcard {
        /// Property [`ExcludedColumnNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-columnwildcard.html#cfn-lakeformation-principalpermissions-columnwildcard-excludedcolumnnames).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub excluded_column_names: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for ColumnWildcard {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref excluded_column_names) = self.excluded_column_names {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExcludedColumnNames", excluded_column_names)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ColumnWildcard {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ColumnWildcard, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ColumnWildcard;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ColumnWildcard")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut excluded_column_names: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ExcludedColumnNames" => {
                                excluded_column_names = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ColumnWildcard {
                        excluded_column_names: excluded_column_names,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LakeFormation::PrincipalPermissions.DataCellsFilterResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-datacellsfilterresource.html) property type.
    #[derive(Debug, Default)]
    pub struct DataCellsFilterResource {
        /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-datacellsfilterresource.html#cfn-lakeformation-principalpermissions-datacellsfilterresource-databasename).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub database_name: ::Value<String>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-datacellsfilterresource.html#cfn-lakeformation-principalpermissions-datacellsfilterresource-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`TableCatalogId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-datacellsfilterresource.html#cfn-lakeformation-principalpermissions-datacellsfilterresource-tablecatalogid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub table_catalog_id: ::Value<String>,
        /// Property [`TableName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-datacellsfilterresource.html#cfn-lakeformation-principalpermissions-datacellsfilterresource-tablename).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub table_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for DataCellsFilterResource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", &self.database_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableCatalogId", &self.table_catalog_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableName", &self.table_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataCellsFilterResource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataCellsFilterResource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataCellsFilterResource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataCellsFilterResource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut database_name: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut table_catalog_id: Option<::Value<String>> = None;
                    let mut table_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DatabaseName" => {
                                database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TableCatalogId" => {
                                table_catalog_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TableName" => {
                                table_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataCellsFilterResource {
                        database_name: database_name.ok_or(::serde::de::Error::missing_field("DatabaseName"))?,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        table_catalog_id: table_catalog_id.ok_or(::serde::de::Error::missing_field("TableCatalogId"))?,
                        table_name: table_name.ok_or(::serde::de::Error::missing_field("TableName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LakeFormation::PrincipalPermissions.DataLakePrincipal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-datalakeprincipal.html) property type.
    #[derive(Debug, Default)]
    pub struct DataLakePrincipal {
        /// Property [`DataLakePrincipalIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-datalakeprincipal.html#cfn-lakeformation-principalpermissions-datalakeprincipal-datalakeprincipalidentifier).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub data_lake_principal_identifier: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DataLakePrincipal {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref data_lake_principal_identifier) = self.data_lake_principal_identifier {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataLakePrincipalIdentifier", data_lake_principal_identifier)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataLakePrincipal {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataLakePrincipal, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataLakePrincipal;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataLakePrincipal")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_lake_principal_identifier: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataLakePrincipalIdentifier" => {
                                data_lake_principal_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataLakePrincipal {
                        data_lake_principal_identifier: data_lake_principal_identifier,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LakeFormation::PrincipalPermissions.DataLocationResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-datalocationresource.html) property type.
    #[derive(Debug, Default)]
    pub struct DataLocationResource {
        /// Property [`CatalogId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-datalocationresource.html#cfn-lakeformation-principalpermissions-datalocationresource-catalogid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub catalog_id: ::Value<String>,
        /// Property [`ResourceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-datalocationresource.html#cfn-lakeformation-principalpermissions-datalocationresource-resourcearn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub resource_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for DataLocationResource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CatalogId", &self.catalog_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceArn", &self.resource_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataLocationResource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataLocationResource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataLocationResource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataLocationResource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut catalog_id: Option<::Value<String>> = None;
                    let mut resource_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CatalogId" => {
                                catalog_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceArn" => {
                                resource_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataLocationResource {
                        catalog_id: catalog_id.ok_or(::serde::de::Error::missing_field("CatalogId"))?,
                        resource_arn: resource_arn.ok_or(::serde::de::Error::missing_field("ResourceArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LakeFormation::PrincipalPermissions.DatabaseResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-databaseresource.html) property type.
    #[derive(Debug, Default)]
    pub struct DatabaseResource {
        /// Property [`CatalogId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-databaseresource.html#cfn-lakeformation-principalpermissions-databaseresource-catalogid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub catalog_id: ::Value<String>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-databaseresource.html#cfn-lakeformation-principalpermissions-databaseresource-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for DatabaseResource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CatalogId", &self.catalog_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DatabaseResource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DatabaseResource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DatabaseResource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DatabaseResource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut catalog_id: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CatalogId" => {
                                catalog_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DatabaseResource {
                        catalog_id: catalog_id.ok_or(::serde::de::Error::missing_field("CatalogId"))?,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LakeFormation::PrincipalPermissions.LFTag`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-lftag.html) property type.
    #[derive(Debug, Default)]
    pub struct LFTag {
        /// Property [`TagKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-lftag.html#cfn-lakeformation-principalpermissions-lftag-tagkey).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub tag_key: Option<::Value<String>>,
        /// Property [`TagValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-lftag.html#cfn-lakeformation-principalpermissions-lftag-tagvalues).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub tag_values: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for LFTag {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref tag_key) = self.tag_key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TagKey", tag_key)?;
            }
            if let Some(ref tag_values) = self.tag_values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TagValues", tag_values)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LFTag {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LFTag, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LFTag;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LFTag")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut tag_key: Option<::Value<String>> = None;
                    let mut tag_values: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TagKey" => {
                                tag_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TagValues" => {
                                tag_values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LFTag {
                        tag_key: tag_key,
                        tag_values: tag_values,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LakeFormation::PrincipalPermissions.LFTagKeyResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-lftagkeyresource.html) property type.
    #[derive(Debug, Default)]
    pub struct LFTagKeyResource {
        /// Property [`CatalogId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-lftagkeyresource.html#cfn-lakeformation-principalpermissions-lftagkeyresource-catalogid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub catalog_id: ::Value<String>,
        /// Property [`TagKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-lftagkeyresource.html#cfn-lakeformation-principalpermissions-lftagkeyresource-tagkey).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub tag_key: ::Value<String>,
        /// Property [`TagValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-lftagkeyresource.html#cfn-lakeformation-principalpermissions-lftagkeyresource-tagvalues).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub tag_values: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for LFTagKeyResource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CatalogId", &self.catalog_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TagKey", &self.tag_key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TagValues", &self.tag_values)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LFTagKeyResource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LFTagKeyResource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LFTagKeyResource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LFTagKeyResource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut catalog_id: Option<::Value<String>> = None;
                    let mut tag_key: Option<::Value<String>> = None;
                    let mut tag_values: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CatalogId" => {
                                catalog_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TagKey" => {
                                tag_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TagValues" => {
                                tag_values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LFTagKeyResource {
                        catalog_id: catalog_id.ok_or(::serde::de::Error::missing_field("CatalogId"))?,
                        tag_key: tag_key.ok_or(::serde::de::Error::missing_field("TagKey"))?,
                        tag_values: tag_values.ok_or(::serde::de::Error::missing_field("TagValues"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LakeFormation::PrincipalPermissions.LFTagPolicyResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-lftagpolicyresource.html) property type.
    #[derive(Debug, Default)]
    pub struct LFTagPolicyResource {
        /// Property [`CatalogId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-lftagpolicyresource.html#cfn-lakeformation-principalpermissions-lftagpolicyresource-catalogid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub catalog_id: ::Value<String>,
        /// Property [`Expression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-lftagpolicyresource.html#cfn-lakeformation-principalpermissions-lftagpolicyresource-expression).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub expression: ::ValueList<LFTag>,
        /// Property [`ResourceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-lftagpolicyresource.html#cfn-lakeformation-principalpermissions-lftagpolicyresource-resourcetype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub resource_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for LFTagPolicyResource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CatalogId", &self.catalog_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Expression", &self.expression)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceType", &self.resource_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LFTagPolicyResource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LFTagPolicyResource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LFTagPolicyResource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LFTagPolicyResource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut catalog_id: Option<::Value<String>> = None;
                    let mut expression: Option<::ValueList<LFTag>> = None;
                    let mut resource_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CatalogId" => {
                                catalog_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Expression" => {
                                expression = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResourceType" => {
                                resource_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LFTagPolicyResource {
                        catalog_id: catalog_id.ok_or(::serde::de::Error::missing_field("CatalogId"))?,
                        expression: expression.ok_or(::serde::de::Error::missing_field("Expression"))?,
                        resource_type: resource_type.ok_or(::serde::de::Error::missing_field("ResourceType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LakeFormation::PrincipalPermissions.Resource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-resource.html) property type.
    #[derive(Debug, Default)]
    pub struct Resource {
        /// Property [`Catalog`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-resource.html#cfn-lakeformation-principalpermissions-resource-catalog).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub catalog: Option<::Value<::json::Value>>,
        /// Property [`DataCellsFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-resource.html#cfn-lakeformation-principalpermissions-resource-datacellsfilter).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub data_cells_filter: Option<::Value<DataCellsFilterResource>>,
        /// Property [`DataLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-resource.html#cfn-lakeformation-principalpermissions-resource-datalocation).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub data_location: Option<::Value<DataLocationResource>>,
        /// Property [`Database`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-resource.html#cfn-lakeformation-principalpermissions-resource-database).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub database: Option<::Value<DatabaseResource>>,
        /// Property [`LFTag`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-resource.html#cfn-lakeformation-principalpermissions-resource-lftag).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub lf_tag: Option<::Value<LFTagKeyResource>>,
        /// Property [`LFTagPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-resource.html#cfn-lakeformation-principalpermissions-resource-lftagpolicy).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub lf_tag_policy: Option<::Value<LFTagPolicyResource>>,
        /// Property [`Table`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-resource.html#cfn-lakeformation-principalpermissions-resource-table).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub table: Option<::Value<TableResource>>,
        /// Property [`TableWithColumns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-resource.html#cfn-lakeformation-principalpermissions-resource-tablewithcolumns).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub table_with_columns: Option<::Value<TableWithColumnsResource>>,
    }

    impl ::codec::SerializeValue for Resource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref catalog) = self.catalog {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Catalog", catalog)?;
            }
            if let Some(ref data_cells_filter) = self.data_cells_filter {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataCellsFilter", data_cells_filter)?;
            }
            if let Some(ref data_location) = self.data_location {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataLocation", data_location)?;
            }
            if let Some(ref database) = self.database {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Database", database)?;
            }
            if let Some(ref lf_tag) = self.lf_tag {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LFTag", lf_tag)?;
            }
            if let Some(ref lf_tag_policy) = self.lf_tag_policy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LFTagPolicy", lf_tag_policy)?;
            }
            if let Some(ref table) = self.table {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Table", table)?;
            }
            if let Some(ref table_with_columns) = self.table_with_columns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableWithColumns", table_with_columns)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Resource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Resource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Resource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Resource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut catalog: Option<::Value<::json::Value>> = None;
                    let mut data_cells_filter: Option<::Value<DataCellsFilterResource>> = None;
                    let mut data_location: Option<::Value<DataLocationResource>> = None;
                    let mut database: Option<::Value<DatabaseResource>> = None;
                    let mut lf_tag: Option<::Value<LFTagKeyResource>> = None;
                    let mut lf_tag_policy: Option<::Value<LFTagPolicyResource>> = None;
                    let mut table: Option<::Value<TableResource>> = None;
                    let mut table_with_columns: Option<::Value<TableWithColumnsResource>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Catalog" => {
                                catalog = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataCellsFilter" => {
                                data_cells_filter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataLocation" => {
                                data_location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Database" => {
                                database = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LFTag" => {
                                lf_tag = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LFTagPolicy" => {
                                lf_tag_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Table" => {
                                table = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TableWithColumns" => {
                                table_with_columns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Resource {
                        catalog: catalog,
                        data_cells_filter: data_cells_filter,
                        data_location: data_location,
                        database: database,
                        lf_tag: lf_tag,
                        lf_tag_policy: lf_tag_policy,
                        table: table,
                        table_with_columns: table_with_columns,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LakeFormation::PrincipalPermissions.TableResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-tableresource.html) property type.
    #[derive(Debug, Default)]
    pub struct TableResource {
        /// Property [`CatalogId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-tableresource.html#cfn-lakeformation-principalpermissions-tableresource-catalogid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub catalog_id: ::Value<String>,
        /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-tableresource.html#cfn-lakeformation-principalpermissions-tableresource-databasename).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub database_name: ::Value<String>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-tableresource.html#cfn-lakeformation-principalpermissions-tableresource-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`TableWildcard`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-tableresource.html#cfn-lakeformation-principalpermissions-tableresource-tablewildcard).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub table_wildcard: Option<::Value<::json::Value>>,
    }

    impl ::codec::SerializeValue for TableResource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CatalogId", &self.catalog_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", &self.database_name)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref table_wildcard) = self.table_wildcard {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableWildcard", table_wildcard)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TableResource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TableResource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TableResource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TableResource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut catalog_id: Option<::Value<String>> = None;
                    let mut database_name: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut table_wildcard: Option<::Value<::json::Value>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CatalogId" => {
                                catalog_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabaseName" => {
                                database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TableWildcard" => {
                                table_wildcard = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TableResource {
                        catalog_id: catalog_id.ok_or(::serde::de::Error::missing_field("CatalogId"))?,
                        database_name: database_name.ok_or(::serde::de::Error::missing_field("DatabaseName"))?,
                        name: name,
                        table_wildcard: table_wildcard,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LakeFormation::PrincipalPermissions.TableWithColumnsResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-tablewithcolumnsresource.html) property type.
    #[derive(Debug, Default)]
    pub struct TableWithColumnsResource {
        /// Property [`CatalogId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-tablewithcolumnsresource.html#cfn-lakeformation-principalpermissions-tablewithcolumnsresource-catalogid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub catalog_id: ::Value<String>,
        /// Property [`ColumnNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-tablewithcolumnsresource.html#cfn-lakeformation-principalpermissions-tablewithcolumnsresource-columnnames).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub column_names: Option<::ValueList<String>>,
        /// Property [`ColumnWildcard`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-tablewithcolumnsresource.html#cfn-lakeformation-principalpermissions-tablewithcolumnsresource-columnwildcard).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub column_wildcard: Option<::Value<ColumnWildcard>>,
        /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-tablewithcolumnsresource.html#cfn-lakeformation-principalpermissions-tablewithcolumnsresource-databasename).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub database_name: ::Value<String>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-principalpermissions-tablewithcolumnsresource.html#cfn-lakeformation-principalpermissions-tablewithcolumnsresource-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for TableWithColumnsResource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CatalogId", &self.catalog_id)?;
            if let Some(ref column_names) = self.column_names {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColumnNames", column_names)?;
            }
            if let Some(ref column_wildcard) = self.column_wildcard {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColumnWildcard", column_wildcard)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", &self.database_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TableWithColumnsResource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TableWithColumnsResource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TableWithColumnsResource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TableWithColumnsResource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut catalog_id: Option<::Value<String>> = None;
                    let mut column_names: Option<::ValueList<String>> = None;
                    let mut column_wildcard: Option<::Value<ColumnWildcard>> = None;
                    let mut database_name: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CatalogId" => {
                                catalog_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ColumnNames" => {
                                column_names = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ColumnWildcard" => {
                                column_wildcard = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabaseName" => {
                                database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TableWithColumnsResource {
                        catalog_id: catalog_id.ok_or(::serde::de::Error::missing_field("CatalogId"))?,
                        column_names: column_names,
                        column_wildcard: column_wildcard,
                        database_name: database_name.ok_or(::serde::de::Error::missing_field("DatabaseName"))?,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod tag_association {
    //! Property types for the `TagAssociation` resource.

    /// The [`AWS::LakeFormation::TagAssociation.DatabaseResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-tagassociation-databaseresource.html) property type.
    #[derive(Debug, Default)]
    pub struct DatabaseResource {
        /// Property [`CatalogId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-tagassociation-databaseresource.html#cfn-lakeformation-tagassociation-databaseresource-catalogid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub catalog_id: ::Value<String>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-tagassociation-databaseresource.html#cfn-lakeformation-tagassociation-databaseresource-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for DatabaseResource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CatalogId", &self.catalog_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DatabaseResource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DatabaseResource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DatabaseResource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DatabaseResource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut catalog_id: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CatalogId" => {
                                catalog_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DatabaseResource {
                        catalog_id: catalog_id.ok_or(::serde::de::Error::missing_field("CatalogId"))?,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LakeFormation::TagAssociation.LFTagPair`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-tagassociation-lftagpair.html) property type.
    #[derive(Debug, Default)]
    pub struct LFTagPair {
        /// Property [`CatalogId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-tagassociation-lftagpair.html#cfn-lakeformation-tagassociation-lftagpair-catalogid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub catalog_id: ::Value<String>,
        /// Property [`TagKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-tagassociation-lftagpair.html#cfn-lakeformation-tagassociation-lftagpair-tagkey).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub tag_key: ::Value<String>,
        /// Property [`TagValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-tagassociation-lftagpair.html#cfn-lakeformation-tagassociation-lftagpair-tagvalues).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub tag_values: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for LFTagPair {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CatalogId", &self.catalog_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TagKey", &self.tag_key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TagValues", &self.tag_values)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LFTagPair {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LFTagPair, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LFTagPair;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LFTagPair")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut catalog_id: Option<::Value<String>> = None;
                    let mut tag_key: Option<::Value<String>> = None;
                    let mut tag_values: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CatalogId" => {
                                catalog_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TagKey" => {
                                tag_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TagValues" => {
                                tag_values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LFTagPair {
                        catalog_id: catalog_id.ok_or(::serde::de::Error::missing_field("CatalogId"))?,
                        tag_key: tag_key.ok_or(::serde::de::Error::missing_field("TagKey"))?,
                        tag_values: tag_values.ok_or(::serde::de::Error::missing_field("TagValues"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LakeFormation::TagAssociation.Resource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-tagassociation-resource.html) property type.
    #[derive(Debug, Default)]
    pub struct Resource {
        /// Property [`Catalog`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-tagassociation-resource.html#cfn-lakeformation-tagassociation-resource-catalog).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub catalog: Option<::Value<::json::Value>>,
        /// Property [`Database`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-tagassociation-resource.html#cfn-lakeformation-tagassociation-resource-database).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub database: Option<::Value<DatabaseResource>>,
        /// Property [`Table`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-tagassociation-resource.html#cfn-lakeformation-tagassociation-resource-table).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub table: Option<::Value<TableResource>>,
        /// Property [`TableWithColumns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-tagassociation-resource.html#cfn-lakeformation-tagassociation-resource-tablewithcolumns).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub table_with_columns: Option<::Value<TableWithColumnsResource>>,
    }

    impl ::codec::SerializeValue for Resource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref catalog) = self.catalog {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Catalog", catalog)?;
            }
            if let Some(ref database) = self.database {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Database", database)?;
            }
            if let Some(ref table) = self.table {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Table", table)?;
            }
            if let Some(ref table_with_columns) = self.table_with_columns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableWithColumns", table_with_columns)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Resource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Resource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Resource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Resource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut catalog: Option<::Value<::json::Value>> = None;
                    let mut database: Option<::Value<DatabaseResource>> = None;
                    let mut table: Option<::Value<TableResource>> = None;
                    let mut table_with_columns: Option<::Value<TableWithColumnsResource>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Catalog" => {
                                catalog = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Database" => {
                                database = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Table" => {
                                table = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TableWithColumns" => {
                                table_with_columns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Resource {
                        catalog: catalog,
                        database: database,
                        table: table,
                        table_with_columns: table_with_columns,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LakeFormation::TagAssociation.TableResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-tagassociation-tableresource.html) property type.
    #[derive(Debug, Default)]
    pub struct TableResource {
        /// Property [`CatalogId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-tagassociation-tableresource.html#cfn-lakeformation-tagassociation-tableresource-catalogid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub catalog_id: ::Value<String>,
        /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-tagassociation-tableresource.html#cfn-lakeformation-tagassociation-tableresource-databasename).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub database_name: ::Value<String>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-tagassociation-tableresource.html#cfn-lakeformation-tagassociation-tableresource-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`TableWildcard`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-tagassociation-tableresource.html#cfn-lakeformation-tagassociation-tableresource-tablewildcard).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub table_wildcard: Option<::Value<::json::Value>>,
    }

    impl ::codec::SerializeValue for TableResource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CatalogId", &self.catalog_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", &self.database_name)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref table_wildcard) = self.table_wildcard {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableWildcard", table_wildcard)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TableResource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TableResource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TableResource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TableResource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut catalog_id: Option<::Value<String>> = None;
                    let mut database_name: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut table_wildcard: Option<::Value<::json::Value>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CatalogId" => {
                                catalog_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabaseName" => {
                                database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TableWildcard" => {
                                table_wildcard = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TableResource {
                        catalog_id: catalog_id.ok_or(::serde::de::Error::missing_field("CatalogId"))?,
                        database_name: database_name.ok_or(::serde::de::Error::missing_field("DatabaseName"))?,
                        name: name,
                        table_wildcard: table_wildcard,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LakeFormation::TagAssociation.TableWithColumnsResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-tagassociation-tablewithcolumnsresource.html) property type.
    #[derive(Debug, Default)]
    pub struct TableWithColumnsResource {
        /// Property [`CatalogId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-tagassociation-tablewithcolumnsresource.html#cfn-lakeformation-tagassociation-tablewithcolumnsresource-catalogid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub catalog_id: ::Value<String>,
        /// Property [`ColumnNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-tagassociation-tablewithcolumnsresource.html#cfn-lakeformation-tagassociation-tablewithcolumnsresource-columnnames).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub column_names: ::ValueList<String>,
        /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-tagassociation-tablewithcolumnsresource.html#cfn-lakeformation-tagassociation-tablewithcolumnsresource-databasename).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub database_name: ::Value<String>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lakeformation-tagassociation-tablewithcolumnsresource.html#cfn-lakeformation-tagassociation-tablewithcolumnsresource-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for TableWithColumnsResource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CatalogId", &self.catalog_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColumnNames", &self.column_names)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", &self.database_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TableWithColumnsResource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TableWithColumnsResource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TableWithColumnsResource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TableWithColumnsResource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut catalog_id: Option<::Value<String>> = None;
                    let mut column_names: Option<::ValueList<String>> = None;
                    let mut database_name: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CatalogId" => {
                                catalog_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ColumnNames" => {
                                column_names = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatabaseName" => {
                                database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TableWithColumnsResource {
                        catalog_id: catalog_id.ok_or(::serde::de::Error::missing_field("CatalogId"))?,
                        column_names: column_names.ok_or(::serde::de::Error::missing_field("ColumnNames"))?,
                        database_name: database_name.ok_or(::serde::de::Error::missing_field("DatabaseName"))?,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
