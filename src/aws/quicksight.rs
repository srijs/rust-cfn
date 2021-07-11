//! Types for the `QuickSight` service.

/// The [`AWS::QuickSight::Analysis`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-analysis.html) resource type.
#[derive(Debug, Default)]
pub struct Analysis {
    properties: AnalysisProperties
}

/// Properties for the `Analysis` resource.
#[derive(Debug, Default)]
pub struct AnalysisProperties {
    /// Property [`AnalysisId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-analysis.html#cfn-quicksight-analysis-analysisid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub analysis_id: ::Value<String>,
    /// Property [`AwsAccountId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-analysis.html#cfn-quicksight-analysis-awsaccountid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub aws_account_id: ::Value<String>,
    /// Property [`Errors`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-analysis.html#cfn-quicksight-analysis-errors).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub errors: Option<::ValueList<self::analysis::AnalysisError>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-analysis.html#cfn-quicksight-analysis-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-analysis.html#cfn-quicksight-analysis-parameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub parameters: Option<::Value<self::analysis::Parameters>>,
    /// Property [`Permissions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-analysis.html#cfn-quicksight-analysis-permissions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub permissions: Option<::ValueList<self::analysis::ResourcePermission>>,
    /// Property [`SourceEntity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-analysis.html#cfn-quicksight-analysis-sourceentity).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub source_entity: Option<::Value<self::analysis::AnalysisSourceEntity>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-analysis.html#cfn-quicksight-analysis-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`ThemeArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-analysis.html#cfn-quicksight-analysis-themearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub theme_arn: Option<::Value<String>>,
}

impl ::serde::Serialize for AnalysisProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AnalysisId", &self.analysis_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AwsAccountId", &self.aws_account_id)?;
        if let Some(ref errors) = self.errors {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Errors", errors)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref parameters) = self.parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", parameters)?;
        }
        if let Some(ref permissions) = self.permissions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Permissions", permissions)?;
        }
        if let Some(ref source_entity) = self.source_entity {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceEntity", source_entity)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref theme_arn) = self.theme_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThemeArn", theme_arn)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AnalysisProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AnalysisProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AnalysisProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AnalysisProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut analysis_id: Option<::Value<String>> = None;
                let mut aws_account_id: Option<::Value<String>> = None;
                let mut errors: Option<::ValueList<self::analysis::AnalysisError>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut parameters: Option<::Value<self::analysis::Parameters>> = None;
                let mut permissions: Option<::ValueList<self::analysis::ResourcePermission>> = None;
                let mut source_entity: Option<::Value<self::analysis::AnalysisSourceEntity>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut theme_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AnalysisId" => {
                            analysis_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AwsAccountId" => {
                            aws_account_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Errors" => {
                            errors = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Parameters" => {
                            parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Permissions" => {
                            permissions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceEntity" => {
                            source_entity = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ThemeArn" => {
                            theme_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AnalysisProperties {
                    analysis_id: analysis_id.ok_or(::serde::de::Error::missing_field("AnalysisId"))?,
                    aws_account_id: aws_account_id.ok_or(::serde::de::Error::missing_field("AwsAccountId"))?,
                    errors: errors,
                    name: name,
                    parameters: parameters,
                    permissions: permissions,
                    source_entity: source_entity,
                    tags: tags,
                    theme_arn: theme_arn,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Analysis {
    type Properties = AnalysisProperties;
    const TYPE: &'static str = "AWS::QuickSight::Analysis";
    fn properties(&self) -> &AnalysisProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AnalysisProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Analysis {}

impl From<AnalysisProperties> for Analysis {
    fn from(properties: AnalysisProperties) -> Analysis {
        Analysis { properties }
    }
}

/// The [`AWS::QuickSight::Dashboard`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-dashboard.html) resource type.
#[derive(Debug, Default)]
pub struct Dashboard {
    properties: DashboardProperties
}

/// Properties for the `Dashboard` resource.
#[derive(Debug, Default)]
pub struct DashboardProperties {
    /// Property [`AwsAccountId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-dashboard.html#cfn-quicksight-dashboard-awsaccountid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub aws_account_id: ::Value<String>,
    /// Property [`DashboardId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-dashboard.html#cfn-quicksight-dashboard-dashboardid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub dashboard_id: ::Value<String>,
    /// Property [`DashboardPublishOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-dashboard.html#cfn-quicksight-dashboard-dashboardpublishoptions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub dashboard_publish_options: Option<::Value<self::dashboard::DashboardPublishOptions>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-dashboard.html#cfn-quicksight-dashboard-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-dashboard.html#cfn-quicksight-dashboard-parameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub parameters: Option<::Value<self::dashboard::Parameters>>,
    /// Property [`Permissions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-dashboard.html#cfn-quicksight-dashboard-permissions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub permissions: Option<::ValueList<self::dashboard::ResourcePermission>>,
    /// Property [`SourceEntity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-dashboard.html#cfn-quicksight-dashboard-sourceentity).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub source_entity: Option<::Value<self::dashboard::DashboardSourceEntity>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-dashboard.html#cfn-quicksight-dashboard-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`ThemeArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-dashboard.html#cfn-quicksight-dashboard-themearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub theme_arn: Option<::Value<String>>,
    /// Property [`VersionDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-dashboard.html#cfn-quicksight-dashboard-versiondescription).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub version_description: Option<::Value<String>>,
}

impl ::serde::Serialize for DashboardProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AwsAccountId", &self.aws_account_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DashboardId", &self.dashboard_id)?;
        if let Some(ref dashboard_publish_options) = self.dashboard_publish_options {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DashboardPublishOptions", dashboard_publish_options)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref parameters) = self.parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", parameters)?;
        }
        if let Some(ref permissions) = self.permissions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Permissions", permissions)?;
        }
        if let Some(ref source_entity) = self.source_entity {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceEntity", source_entity)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref theme_arn) = self.theme_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThemeArn", theme_arn)?;
        }
        if let Some(ref version_description) = self.version_description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VersionDescription", version_description)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DashboardProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DashboardProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DashboardProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DashboardProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut aws_account_id: Option<::Value<String>> = None;
                let mut dashboard_id: Option<::Value<String>> = None;
                let mut dashboard_publish_options: Option<::Value<self::dashboard::DashboardPublishOptions>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut parameters: Option<::Value<self::dashboard::Parameters>> = None;
                let mut permissions: Option<::ValueList<self::dashboard::ResourcePermission>> = None;
                let mut source_entity: Option<::Value<self::dashboard::DashboardSourceEntity>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut theme_arn: Option<::Value<String>> = None;
                let mut version_description: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AwsAccountId" => {
                            aws_account_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DashboardId" => {
                            dashboard_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DashboardPublishOptions" => {
                            dashboard_publish_options = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Parameters" => {
                            parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Permissions" => {
                            permissions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceEntity" => {
                            source_entity = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ThemeArn" => {
                            theme_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VersionDescription" => {
                            version_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DashboardProperties {
                    aws_account_id: aws_account_id.ok_or(::serde::de::Error::missing_field("AwsAccountId"))?,
                    dashboard_id: dashboard_id.ok_or(::serde::de::Error::missing_field("DashboardId"))?,
                    dashboard_publish_options: dashboard_publish_options,
                    name: name,
                    parameters: parameters,
                    permissions: permissions,
                    source_entity: source_entity,
                    tags: tags,
                    theme_arn: theme_arn,
                    version_description: version_description,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Dashboard {
    type Properties = DashboardProperties;
    const TYPE: &'static str = "AWS::QuickSight::Dashboard";
    fn properties(&self) -> &DashboardProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DashboardProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Dashboard {}

impl From<DashboardProperties> for Dashboard {
    fn from(properties: DashboardProperties) -> Dashboard {
        Dashboard { properties }
    }
}

/// The [`AWS::QuickSight::DataSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-dataset.html) resource type.
#[derive(Debug, Default)]
pub struct DataSet {
    properties: DataSetProperties
}

/// Properties for the `DataSet` resource.
#[derive(Debug, Default)]
pub struct DataSetProperties {
    /// Property [`AwsAccountId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-dataset.html#cfn-quicksight-dataset-awsaccountid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub aws_account_id: Option<::Value<String>>,
    /// Property [`ColumnGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-dataset.html#cfn-quicksight-dataset-columngroups).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub column_groups: Option<::ValueList<self::data_set::ColumnGroup>>,
    /// Property [`ColumnLevelPermissionRules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-dataset.html#cfn-quicksight-dataset-columnlevelpermissionrules).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub column_level_permission_rules: Option<::ValueList<self::data_set::ColumnLevelPermissionRule>>,
    /// Property [`DataSetId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-dataset.html#cfn-quicksight-dataset-datasetid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub data_set_id: Option<::Value<String>>,
    /// Property [`FieldFolders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-dataset.html#cfn-quicksight-dataset-fieldfolders).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub field_folders: Option<::ValueMap<self::data_set::FieldFolder>>,
    /// Property [`ImportMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-dataset.html#cfn-quicksight-dataset-importmode).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub import_mode: Option<::Value<String>>,
    /// Property [`IngestionWaitPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-dataset.html#cfn-quicksight-dataset-ingestionwaitpolicy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ingestion_wait_policy: Option<::Value<self::data_set::IngestionWaitPolicy>>,
    /// Property [`LogicalTableMap`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-dataset.html#cfn-quicksight-dataset-logicaltablemap).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub logical_table_map: Option<::ValueMap<self::data_set::LogicalTable>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-dataset.html#cfn-quicksight-dataset-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Permissions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-dataset.html#cfn-quicksight-dataset-permissions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub permissions: Option<::ValueList<self::data_set::ResourcePermission>>,
    /// Property [`PhysicalTableMap`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-dataset.html#cfn-quicksight-dataset-physicaltablemap).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub physical_table_map: Option<::ValueMap<self::data_set::PhysicalTable>>,
    /// Property [`RowLevelPermissionDataSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-dataset.html#cfn-quicksight-dataset-rowlevelpermissiondataset).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub row_level_permission_data_set: Option<::Value<self::data_set::RowLevelPermissionDataSet>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-dataset.html#cfn-quicksight-dataset-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for DataSetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref aws_account_id) = self.aws_account_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AwsAccountId", aws_account_id)?;
        }
        if let Some(ref column_groups) = self.column_groups {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColumnGroups", column_groups)?;
        }
        if let Some(ref column_level_permission_rules) = self.column_level_permission_rules {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColumnLevelPermissionRules", column_level_permission_rules)?;
        }
        if let Some(ref data_set_id) = self.data_set_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataSetId", data_set_id)?;
        }
        if let Some(ref field_folders) = self.field_folders {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FieldFolders", field_folders)?;
        }
        if let Some(ref import_mode) = self.import_mode {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImportMode", import_mode)?;
        }
        if let Some(ref ingestion_wait_policy) = self.ingestion_wait_policy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IngestionWaitPolicy", ingestion_wait_policy)?;
        }
        if let Some(ref logical_table_map) = self.logical_table_map {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogicalTableMap", logical_table_map)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref permissions) = self.permissions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Permissions", permissions)?;
        }
        if let Some(ref physical_table_map) = self.physical_table_map {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PhysicalTableMap", physical_table_map)?;
        }
        if let Some(ref row_level_permission_data_set) = self.row_level_permission_data_set {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RowLevelPermissionDataSet", row_level_permission_data_set)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DataSetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DataSetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DataSetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DataSetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut aws_account_id: Option<::Value<String>> = None;
                let mut column_groups: Option<::ValueList<self::data_set::ColumnGroup>> = None;
                let mut column_level_permission_rules: Option<::ValueList<self::data_set::ColumnLevelPermissionRule>> = None;
                let mut data_set_id: Option<::Value<String>> = None;
                let mut field_folders: Option<::ValueMap<self::data_set::FieldFolder>> = None;
                let mut import_mode: Option<::Value<String>> = None;
                let mut ingestion_wait_policy: Option<::Value<self::data_set::IngestionWaitPolicy>> = None;
                let mut logical_table_map: Option<::ValueMap<self::data_set::LogicalTable>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut permissions: Option<::ValueList<self::data_set::ResourcePermission>> = None;
                let mut physical_table_map: Option<::ValueMap<self::data_set::PhysicalTable>> = None;
                let mut row_level_permission_data_set: Option<::Value<self::data_set::RowLevelPermissionDataSet>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AwsAccountId" => {
                            aws_account_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ColumnGroups" => {
                            column_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ColumnLevelPermissionRules" => {
                            column_level_permission_rules = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DataSetId" => {
                            data_set_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FieldFolders" => {
                            field_folders = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ImportMode" => {
                            import_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IngestionWaitPolicy" => {
                            ingestion_wait_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LogicalTableMap" => {
                            logical_table_map = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Permissions" => {
                            permissions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PhysicalTableMap" => {
                            physical_table_map = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RowLevelPermissionDataSet" => {
                            row_level_permission_data_set = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DataSetProperties {
                    aws_account_id: aws_account_id,
                    column_groups: column_groups,
                    column_level_permission_rules: column_level_permission_rules,
                    data_set_id: data_set_id,
                    field_folders: field_folders,
                    import_mode: import_mode,
                    ingestion_wait_policy: ingestion_wait_policy,
                    logical_table_map: logical_table_map,
                    name: name,
                    permissions: permissions,
                    physical_table_map: physical_table_map,
                    row_level_permission_data_set: row_level_permission_data_set,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DataSet {
    type Properties = DataSetProperties;
    const TYPE: &'static str = "AWS::QuickSight::DataSet";
    fn properties(&self) -> &DataSetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DataSetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DataSet {}

impl From<DataSetProperties> for DataSet {
    fn from(properties: DataSetProperties) -> DataSet {
        DataSet { properties }
    }
}

/// The [`AWS::QuickSight::DataSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-datasource.html) resource type.
#[derive(Debug, Default)]
pub struct DataSource {
    properties: DataSourceProperties
}

/// Properties for the `DataSource` resource.
#[derive(Debug, Default)]
pub struct DataSourceProperties {
    /// Property [`AlternateDataSourceParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-datasource.html#cfn-quicksight-datasource-alternatedatasourceparameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub alternate_data_source_parameters: Option<::ValueList<self::data_source::DataSourceParameters>>,
    /// Property [`AwsAccountId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-datasource.html#cfn-quicksight-datasource-awsaccountid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub aws_account_id: Option<::Value<String>>,
    /// Property [`Credentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-datasource.html#cfn-quicksight-datasource-credentials).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub credentials: Option<::Value<self::data_source::DataSourceCredentials>>,
    /// Property [`DataSourceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-datasource.html#cfn-quicksight-datasource-datasourceid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub data_source_id: Option<::Value<String>>,
    /// Property [`DataSourceParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-datasource.html#cfn-quicksight-datasource-datasourceparameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub data_source_parameters: Option<::Value<self::data_source::DataSourceParameters>>,
    /// Property [`ErrorInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-datasource.html#cfn-quicksight-datasource-errorinfo).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub error_info: Option<::Value<self::data_source::DataSourceErrorInfo>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-datasource.html#cfn-quicksight-datasource-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Permissions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-datasource.html#cfn-quicksight-datasource-permissions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub permissions: Option<::ValueList<self::data_source::ResourcePermission>>,
    /// Property [`SslProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-datasource.html#cfn-quicksight-datasource-sslproperties).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ssl_properties: Option<::Value<self::data_source::SslProperties>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-datasource.html#cfn-quicksight-datasource-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-datasource.html#cfn-quicksight-datasource-type).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub r#type: Option<::Value<String>>,
    /// Property [`VpcConnectionProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-datasource.html#cfn-quicksight-datasource-vpcconnectionproperties).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub vpc_connection_properties: Option<::Value<self::data_source::VpcConnectionProperties>>,
}

impl ::serde::Serialize for DataSourceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref alternate_data_source_parameters) = self.alternate_data_source_parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AlternateDataSourceParameters", alternate_data_source_parameters)?;
        }
        if let Some(ref aws_account_id) = self.aws_account_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AwsAccountId", aws_account_id)?;
        }
        if let Some(ref credentials) = self.credentials {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Credentials", credentials)?;
        }
        if let Some(ref data_source_id) = self.data_source_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataSourceId", data_source_id)?;
        }
        if let Some(ref data_source_parameters) = self.data_source_parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataSourceParameters", data_source_parameters)?;
        }
        if let Some(ref error_info) = self.error_info {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ErrorInfo", error_info)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref permissions) = self.permissions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Permissions", permissions)?;
        }
        if let Some(ref ssl_properties) = self.ssl_properties {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SslProperties", ssl_properties)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref r#type) = self.r#type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
        }
        if let Some(ref vpc_connection_properties) = self.vpc_connection_properties {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcConnectionProperties", vpc_connection_properties)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DataSourceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DataSourceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DataSourceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DataSourceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut alternate_data_source_parameters: Option<::ValueList<self::data_source::DataSourceParameters>> = None;
                let mut aws_account_id: Option<::Value<String>> = None;
                let mut credentials: Option<::Value<self::data_source::DataSourceCredentials>> = None;
                let mut data_source_id: Option<::Value<String>> = None;
                let mut data_source_parameters: Option<::Value<self::data_source::DataSourceParameters>> = None;
                let mut error_info: Option<::Value<self::data_source::DataSourceErrorInfo>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut permissions: Option<::ValueList<self::data_source::ResourcePermission>> = None;
                let mut ssl_properties: Option<::Value<self::data_source::SslProperties>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut r#type: Option<::Value<String>> = None;
                let mut vpc_connection_properties: Option<::Value<self::data_source::VpcConnectionProperties>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AlternateDataSourceParameters" => {
                            alternate_data_source_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AwsAccountId" => {
                            aws_account_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Credentials" => {
                            credentials = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DataSourceId" => {
                            data_source_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DataSourceParameters" => {
                            data_source_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ErrorInfo" => {
                            error_info = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Permissions" => {
                            permissions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SslProperties" => {
                            ssl_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Type" => {
                            r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcConnectionProperties" => {
                            vpc_connection_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DataSourceProperties {
                    alternate_data_source_parameters: alternate_data_source_parameters,
                    aws_account_id: aws_account_id,
                    credentials: credentials,
                    data_source_id: data_source_id,
                    data_source_parameters: data_source_parameters,
                    error_info: error_info,
                    name: name,
                    permissions: permissions,
                    ssl_properties: ssl_properties,
                    tags: tags,
                    r#type: r#type,
                    vpc_connection_properties: vpc_connection_properties,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DataSource {
    type Properties = DataSourceProperties;
    const TYPE: &'static str = "AWS::QuickSight::DataSource";
    fn properties(&self) -> &DataSourceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DataSourceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DataSource {}

impl From<DataSourceProperties> for DataSource {
    fn from(properties: DataSourceProperties) -> DataSource {
        DataSource { properties }
    }
}

/// The [`AWS::QuickSight::Template`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-template.html) resource type.
#[derive(Debug, Default)]
pub struct Template {
    properties: TemplateProperties
}

/// Properties for the `Template` resource.
#[derive(Debug, Default)]
pub struct TemplateProperties {
    /// Property [`AwsAccountId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-template.html#cfn-quicksight-template-awsaccountid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub aws_account_id: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-template.html#cfn-quicksight-template-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Permissions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-template.html#cfn-quicksight-template-permissions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub permissions: Option<::ValueList<self::template::ResourcePermission>>,
    /// Property [`SourceEntity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-template.html#cfn-quicksight-template-sourceentity).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub source_entity: Option<::Value<self::template::TemplateSourceEntity>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-template.html#cfn-quicksight-template-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TemplateId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-template.html#cfn-quicksight-template-templateid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub template_id: ::Value<String>,
    /// Property [`VersionDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-template.html#cfn-quicksight-template-versiondescription).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub version_description: Option<::Value<String>>,
}

impl ::serde::Serialize for TemplateProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AwsAccountId", &self.aws_account_id)?;
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref permissions) = self.permissions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Permissions", permissions)?;
        }
        if let Some(ref source_entity) = self.source_entity {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceEntity", source_entity)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemplateId", &self.template_id)?;
        if let Some(ref version_description) = self.version_description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VersionDescription", version_description)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for TemplateProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<TemplateProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TemplateProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type TemplateProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut aws_account_id: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut permissions: Option<::ValueList<self::template::ResourcePermission>> = None;
                let mut source_entity: Option<::Value<self::template::TemplateSourceEntity>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut template_id: Option<::Value<String>> = None;
                let mut version_description: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AwsAccountId" => {
                            aws_account_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Permissions" => {
                            permissions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceEntity" => {
                            source_entity = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TemplateId" => {
                            template_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VersionDescription" => {
                            version_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(TemplateProperties {
                    aws_account_id: aws_account_id.ok_or(::serde::de::Error::missing_field("AwsAccountId"))?,
                    name: name,
                    permissions: permissions,
                    source_entity: source_entity,
                    tags: tags,
                    template_id: template_id.ok_or(::serde::de::Error::missing_field("TemplateId"))?,
                    version_description: version_description,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Template {
    type Properties = TemplateProperties;
    const TYPE: &'static str = "AWS::QuickSight::Template";
    fn properties(&self) -> &TemplateProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TemplateProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Template {}

impl From<TemplateProperties> for Template {
    fn from(properties: TemplateProperties) -> Template {
        Template { properties }
    }
}

/// The [`AWS::QuickSight::Theme`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-theme.html) resource type.
#[derive(Debug, Default)]
pub struct Theme {
    properties: ThemeProperties
}

/// Properties for the `Theme` resource.
#[derive(Debug, Default)]
pub struct ThemeProperties {
    /// Property [`AwsAccountId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-theme.html#cfn-quicksight-theme-awsaccountid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub aws_account_id: ::Value<String>,
    /// Property [`BaseThemeId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-theme.html#cfn-quicksight-theme-basethemeid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub base_theme_id: Option<::Value<String>>,
    /// Property [`Configuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-theme.html#cfn-quicksight-theme-configuration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub configuration: Option<::Value<self::theme::ThemeConfiguration>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-theme.html#cfn-quicksight-theme-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Permissions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-theme.html#cfn-quicksight-theme-permissions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub permissions: Option<::ValueList<self::theme::ResourcePermission>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-theme.html#cfn-quicksight-theme-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`ThemeId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-theme.html#cfn-quicksight-theme-themeid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub theme_id: ::Value<String>,
    /// Property [`VersionDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-quicksight-theme.html#cfn-quicksight-theme-versiondescription).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub version_description: Option<::Value<String>>,
}

impl ::serde::Serialize for ThemeProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AwsAccountId", &self.aws_account_id)?;
        if let Some(ref base_theme_id) = self.base_theme_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BaseThemeId", base_theme_id)?;
        }
        if let Some(ref configuration) = self.configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Configuration", configuration)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref permissions) = self.permissions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Permissions", permissions)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThemeId", &self.theme_id)?;
        if let Some(ref version_description) = self.version_description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VersionDescription", version_description)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ThemeProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ThemeProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ThemeProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ThemeProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut aws_account_id: Option<::Value<String>> = None;
                let mut base_theme_id: Option<::Value<String>> = None;
                let mut configuration: Option<::Value<self::theme::ThemeConfiguration>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut permissions: Option<::ValueList<self::theme::ResourcePermission>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut theme_id: Option<::Value<String>> = None;
                let mut version_description: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AwsAccountId" => {
                            aws_account_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BaseThemeId" => {
                            base_theme_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Configuration" => {
                            configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Permissions" => {
                            permissions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ThemeId" => {
                            theme_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VersionDescription" => {
                            version_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ThemeProperties {
                    aws_account_id: aws_account_id.ok_or(::serde::de::Error::missing_field("AwsAccountId"))?,
                    base_theme_id: base_theme_id,
                    configuration: configuration,
                    name: name,
                    permissions: permissions,
                    tags: tags,
                    theme_id: theme_id.ok_or(::serde::de::Error::missing_field("ThemeId"))?,
                    version_description: version_description,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Theme {
    type Properties = ThemeProperties;
    const TYPE: &'static str = "AWS::QuickSight::Theme";
    fn properties(&self) -> &ThemeProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ThemeProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Theme {}

impl From<ThemeProperties> for Theme {
    fn from(properties: ThemeProperties) -> Theme {
        Theme { properties }
    }
}

pub mod analysis {
    //! Property types for the `Analysis` resource.

    /// The [`AWS::QuickSight::Analysis.AnalysisError`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-analysis-analysiserror.html) property type.
    #[derive(Debug, Default)]
    pub struct AnalysisError {
        /// Property [`Message`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-analysis-analysiserror.html#cfn-quicksight-analysis-analysiserror-message).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub message: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-analysis-analysiserror.html#cfn-quicksight-analysis-analysiserror-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AnalysisError {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref message) = self.message {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Message", message)?;
            }
            if let Some(ref r#type) = self.r#type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AnalysisError {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AnalysisError, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AnalysisError;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AnalysisError")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut message: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Message" => {
                                message = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AnalysisError {
                        message: message,
                        r#type: r#type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::Analysis.AnalysisSourceEntity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-analysis-analysissourceentity.html) property type.
    #[derive(Debug, Default)]
    pub struct AnalysisSourceEntity {
        /// Property [`SourceTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-analysis-analysissourceentity.html#cfn-quicksight-analysis-analysissourceentity-sourcetemplate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_template: Option<::Value<AnalysisSourceTemplate>>,
    }

    impl ::codec::SerializeValue for AnalysisSourceEntity {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref source_template) = self.source_template {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceTemplate", source_template)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AnalysisSourceEntity {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AnalysisSourceEntity, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AnalysisSourceEntity;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AnalysisSourceEntity")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut source_template: Option<::Value<AnalysisSourceTemplate>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SourceTemplate" => {
                                source_template = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AnalysisSourceEntity {
                        source_template: source_template,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::Analysis.AnalysisSourceTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-analysis-analysissourcetemplate.html) property type.
    #[derive(Debug, Default)]
    pub struct AnalysisSourceTemplate {
        /// Property [`Arn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-analysis-analysissourcetemplate.html#cfn-quicksight-analysis-analysissourcetemplate-arn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub arn: ::Value<String>,
        /// Property [`DataSetReferences`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-analysis-analysissourcetemplate.html#cfn-quicksight-analysis-analysissourcetemplate-datasetreferences).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_set_references: ::ValueList<DataSetReference>,
    }

    impl ::codec::SerializeValue for AnalysisSourceTemplate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arn", &self.arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataSetReferences", &self.data_set_references)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AnalysisSourceTemplate {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AnalysisSourceTemplate, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AnalysisSourceTemplate;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AnalysisSourceTemplate")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arn: Option<::Value<String>> = None;
                    let mut data_set_references: Option<::ValueList<DataSetReference>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Arn" => {
                                arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataSetReferences" => {
                                data_set_references = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AnalysisSourceTemplate {
                        arn: arn.ok_or(::serde::de::Error::missing_field("Arn"))?,
                        data_set_references: data_set_references.ok_or(::serde::de::Error::missing_field("DataSetReferences"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::Analysis.DataSetReference`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-analysis-datasetreference.html) property type.
    #[derive(Debug, Default)]
    pub struct DataSetReference {
        /// Property [`DataSetArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-analysis-datasetreference.html#cfn-quicksight-analysis-datasetreference-datasetarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_set_arn: ::Value<String>,
        /// Property [`DataSetPlaceholder`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-analysis-datasetreference.html#cfn-quicksight-analysis-datasetreference-datasetplaceholder).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_set_placeholder: ::Value<String>,
    }

    impl ::codec::SerializeValue for DataSetReference {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataSetArn", &self.data_set_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataSetPlaceholder", &self.data_set_placeholder)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataSetReference {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataSetReference, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataSetReference;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataSetReference")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_set_arn: Option<::Value<String>> = None;
                    let mut data_set_placeholder: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataSetArn" => {
                                data_set_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataSetPlaceholder" => {
                                data_set_placeholder = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataSetReference {
                        data_set_arn: data_set_arn.ok_or(::serde::de::Error::missing_field("DataSetArn"))?,
                        data_set_placeholder: data_set_placeholder.ok_or(::serde::de::Error::missing_field("DataSetPlaceholder"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::Analysis.DateTimeParameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-analysis-datetimeparameter.html) property type.
    #[derive(Debug, Default)]
    pub struct DateTimeParameter {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-analysis-datetimeparameter.html#cfn-quicksight-analysis-datetimeparameter-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Values`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-analysis-datetimeparameter.html#cfn-quicksight-analysis-datetimeparameter-values).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub values: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for DateTimeParameter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", &self.values)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DateTimeParameter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DateTimeParameter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DateTimeParameter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DateTimeParameter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut values: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Values" => {
                                values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DateTimeParameter {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        values: values.ok_or(::serde::de::Error::missing_field("Values"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::Analysis.DecimalParameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-analysis-decimalparameter.html) property type.
    #[derive(Debug, Default)]
    pub struct DecimalParameter {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-analysis-decimalparameter.html#cfn-quicksight-analysis-decimalparameter-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Values`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-analysis-decimalparameter.html#cfn-quicksight-analysis-decimalparameter-values).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub values: ::ValueList<f64>,
    }

    impl ::codec::SerializeValue for DecimalParameter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", &self.values)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DecimalParameter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DecimalParameter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DecimalParameter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DecimalParameter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut values: Option<::ValueList<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Values" => {
                                values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DecimalParameter {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        values: values.ok_or(::serde::de::Error::missing_field("Values"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::Analysis.IntegerParameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-analysis-integerparameter.html) property type.
    #[derive(Debug, Default)]
    pub struct IntegerParameter {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-analysis-integerparameter.html#cfn-quicksight-analysis-integerparameter-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Values`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-analysis-integerparameter.html#cfn-quicksight-analysis-integerparameter-values).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub values: ::ValueList<f64>,
    }

    impl ::codec::SerializeValue for IntegerParameter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", &self.values)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IntegerParameter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IntegerParameter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IntegerParameter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IntegerParameter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut values: Option<::ValueList<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Values" => {
                                values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IntegerParameter {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        values: values.ok_or(::serde::de::Error::missing_field("Values"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::Analysis.Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-analysis-parameters.html) property type.
    #[derive(Debug, Default)]
    pub struct Parameters {
        /// Property [`DateTimeParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-analysis-parameters.html#cfn-quicksight-analysis-parameters-datetimeparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub date_time_parameters: Option<::ValueList<DateTimeParameter>>,
        /// Property [`DecimalParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-analysis-parameters.html#cfn-quicksight-analysis-parameters-decimalparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub decimal_parameters: Option<::ValueList<DecimalParameter>>,
        /// Property [`IntegerParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-analysis-parameters.html#cfn-quicksight-analysis-parameters-integerparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub integer_parameters: Option<::ValueList<IntegerParameter>>,
        /// Property [`StringParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-analysis-parameters.html#cfn-quicksight-analysis-parameters-stringparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub string_parameters: Option<::ValueList<StringParameter>>,
    }

    impl ::codec::SerializeValue for Parameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref date_time_parameters) = self.date_time_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DateTimeParameters", date_time_parameters)?;
            }
            if let Some(ref decimal_parameters) = self.decimal_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DecimalParameters", decimal_parameters)?;
            }
            if let Some(ref integer_parameters) = self.integer_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntegerParameters", integer_parameters)?;
            }
            if let Some(ref string_parameters) = self.string_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StringParameters", string_parameters)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Parameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Parameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Parameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Parameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut date_time_parameters: Option<::ValueList<DateTimeParameter>> = None;
                    let mut decimal_parameters: Option<::ValueList<DecimalParameter>> = None;
                    let mut integer_parameters: Option<::ValueList<IntegerParameter>> = None;
                    let mut string_parameters: Option<::ValueList<StringParameter>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DateTimeParameters" => {
                                date_time_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DecimalParameters" => {
                                decimal_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IntegerParameters" => {
                                integer_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StringParameters" => {
                                string_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Parameters {
                        date_time_parameters: date_time_parameters,
                        decimal_parameters: decimal_parameters,
                        integer_parameters: integer_parameters,
                        string_parameters: string_parameters,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::Analysis.ResourcePermission`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-analysis-resourcepermission.html) property type.
    #[derive(Debug, Default)]
    pub struct ResourcePermission {
        /// Property [`Actions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-analysis-resourcepermission.html#cfn-quicksight-analysis-resourcepermission-actions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub actions: ::ValueList<String>,
        /// Property [`Principal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-analysis-resourcepermission.html#cfn-quicksight-analysis-resourcepermission-principal).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub principal: ::Value<String>,
    }

    impl ::codec::SerializeValue for ResourcePermission {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Actions", &self.actions)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Principal", &self.principal)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResourcePermission {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourcePermission, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResourcePermission;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResourcePermission")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut actions: Option<::ValueList<String>> = None;
                    let mut principal: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Actions" => {
                                actions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Principal" => {
                                principal = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResourcePermission {
                        actions: actions.ok_or(::serde::de::Error::missing_field("Actions"))?,
                        principal: principal.ok_or(::serde::de::Error::missing_field("Principal"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::Analysis.Sheet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-analysis-sheet.html) property type.
    #[derive(Debug, Default)]
    pub struct Sheet {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-analysis-sheet.html#cfn-quicksight-analysis-sheet-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`SheetId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-analysis-sheet.html#cfn-quicksight-analysis-sheet-sheetid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sheet_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Sheet {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref sheet_id) = self.sheet_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SheetId", sheet_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Sheet {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Sheet, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Sheet;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Sheet")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut sheet_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SheetId" => {
                                sheet_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Sheet {
                        name: name,
                        sheet_id: sheet_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::Analysis.StringParameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-analysis-stringparameter.html) property type.
    #[derive(Debug, Default)]
    pub struct StringParameter {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-analysis-stringparameter.html#cfn-quicksight-analysis-stringparameter-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Values`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-analysis-stringparameter.html#cfn-quicksight-analysis-stringparameter-values).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub values: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for StringParameter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", &self.values)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StringParameter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StringParameter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StringParameter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StringParameter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut values: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Values" => {
                                values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StringParameter {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        values: values.ok_or(::serde::de::Error::missing_field("Values"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod dashboard {
    //! Property types for the `Dashboard` resource.

    /// The [`AWS::QuickSight::Dashboard.AdHocFilteringOption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dashboard-adhocfilteringoption.html) property type.
    #[derive(Debug, Default)]
    pub struct AdHocFilteringOption {
        /// Property [`AvailabilityStatus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dashboard-adhocfilteringoption.html#cfn-quicksight-dashboard-adhocfilteringoption-availabilitystatus).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub availability_status: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AdHocFilteringOption {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref availability_status) = self.availability_status {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityStatus", availability_status)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AdHocFilteringOption {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AdHocFilteringOption, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AdHocFilteringOption;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AdHocFilteringOption")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut availability_status: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AvailabilityStatus" => {
                                availability_status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AdHocFilteringOption {
                        availability_status: availability_status,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::Dashboard.DashboardPublishOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dashboard-dashboardpublishoptions.html) property type.
    #[derive(Debug, Default)]
    pub struct DashboardPublishOptions {
        /// Property [`AdHocFilteringOption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dashboard-dashboardpublishoptions.html#cfn-quicksight-dashboard-dashboardpublishoptions-adhocfilteringoption).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ad_hoc_filtering_option: Option<::Value<AdHocFilteringOption>>,
        /// Property [`ExportToCSVOption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dashboard-dashboardpublishoptions.html#cfn-quicksight-dashboard-dashboardpublishoptions-exporttocsvoption).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub export_to_csv_option: Option<::Value<ExportToCSVOption>>,
        /// Property [`SheetControlsOption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dashboard-dashboardpublishoptions.html#cfn-quicksight-dashboard-dashboardpublishoptions-sheetcontrolsoption).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sheet_controls_option: Option<::Value<SheetControlsOption>>,
    }

    impl ::codec::SerializeValue for DashboardPublishOptions {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref ad_hoc_filtering_option) = self.ad_hoc_filtering_option {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdHocFilteringOption", ad_hoc_filtering_option)?;
            }
            if let Some(ref export_to_csv_option) = self.export_to_csv_option {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExportToCSVOption", export_to_csv_option)?;
            }
            if let Some(ref sheet_controls_option) = self.sheet_controls_option {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SheetControlsOption", sheet_controls_option)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DashboardPublishOptions {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DashboardPublishOptions, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DashboardPublishOptions;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DashboardPublishOptions")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ad_hoc_filtering_option: Option<::Value<AdHocFilteringOption>> = None;
                    let mut export_to_csv_option: Option<::Value<ExportToCSVOption>> = None;
                    let mut sheet_controls_option: Option<::Value<SheetControlsOption>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AdHocFilteringOption" => {
                                ad_hoc_filtering_option = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExportToCSVOption" => {
                                export_to_csv_option = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SheetControlsOption" => {
                                sheet_controls_option = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DashboardPublishOptions {
                        ad_hoc_filtering_option: ad_hoc_filtering_option,
                        export_to_csv_option: export_to_csv_option,
                        sheet_controls_option: sheet_controls_option,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::Dashboard.DashboardSourceEntity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dashboard-dashboardsourceentity.html) property type.
    #[derive(Debug, Default)]
    pub struct DashboardSourceEntity {
        /// Property [`SourceTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dashboard-dashboardsourceentity.html#cfn-quicksight-dashboard-dashboardsourceentity-sourcetemplate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_template: Option<::Value<DashboardSourceTemplate>>,
    }

    impl ::codec::SerializeValue for DashboardSourceEntity {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref source_template) = self.source_template {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceTemplate", source_template)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DashboardSourceEntity {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DashboardSourceEntity, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DashboardSourceEntity;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DashboardSourceEntity")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut source_template: Option<::Value<DashboardSourceTemplate>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SourceTemplate" => {
                                source_template = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DashboardSourceEntity {
                        source_template: source_template,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::Dashboard.DashboardSourceTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dashboard-dashboardsourcetemplate.html) property type.
    #[derive(Debug, Default)]
    pub struct DashboardSourceTemplate {
        /// Property [`Arn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dashboard-dashboardsourcetemplate.html#cfn-quicksight-dashboard-dashboardsourcetemplate-arn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub arn: ::Value<String>,
        /// Property [`DataSetReferences`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dashboard-dashboardsourcetemplate.html#cfn-quicksight-dashboard-dashboardsourcetemplate-datasetreferences).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_set_references: ::ValueList<DataSetReference>,
    }

    impl ::codec::SerializeValue for DashboardSourceTemplate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arn", &self.arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataSetReferences", &self.data_set_references)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DashboardSourceTemplate {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DashboardSourceTemplate, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DashboardSourceTemplate;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DashboardSourceTemplate")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arn: Option<::Value<String>> = None;
                    let mut data_set_references: Option<::ValueList<DataSetReference>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Arn" => {
                                arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataSetReferences" => {
                                data_set_references = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DashboardSourceTemplate {
                        arn: arn.ok_or(::serde::de::Error::missing_field("Arn"))?,
                        data_set_references: data_set_references.ok_or(::serde::de::Error::missing_field("DataSetReferences"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::Dashboard.DataSetReference`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dashboard-datasetreference.html) property type.
    #[derive(Debug, Default)]
    pub struct DataSetReference {
        /// Property [`DataSetArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dashboard-datasetreference.html#cfn-quicksight-dashboard-datasetreference-datasetarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_set_arn: ::Value<String>,
        /// Property [`DataSetPlaceholder`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dashboard-datasetreference.html#cfn-quicksight-dashboard-datasetreference-datasetplaceholder).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_set_placeholder: ::Value<String>,
    }

    impl ::codec::SerializeValue for DataSetReference {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataSetArn", &self.data_set_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataSetPlaceholder", &self.data_set_placeholder)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataSetReference {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataSetReference, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataSetReference;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataSetReference")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_set_arn: Option<::Value<String>> = None;
                    let mut data_set_placeholder: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataSetArn" => {
                                data_set_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataSetPlaceholder" => {
                                data_set_placeholder = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataSetReference {
                        data_set_arn: data_set_arn.ok_or(::serde::de::Error::missing_field("DataSetArn"))?,
                        data_set_placeholder: data_set_placeholder.ok_or(::serde::de::Error::missing_field("DataSetPlaceholder"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::Dashboard.DateTimeParameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dashboard-datetimeparameter.html) property type.
    #[derive(Debug, Default)]
    pub struct DateTimeParameter {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dashboard-datetimeparameter.html#cfn-quicksight-dashboard-datetimeparameter-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Values`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dashboard-datetimeparameter.html#cfn-quicksight-dashboard-datetimeparameter-values).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub values: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for DateTimeParameter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", &self.values)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DateTimeParameter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DateTimeParameter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DateTimeParameter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DateTimeParameter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut values: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Values" => {
                                values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DateTimeParameter {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        values: values.ok_or(::serde::de::Error::missing_field("Values"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::Dashboard.DecimalParameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dashboard-decimalparameter.html) property type.
    #[derive(Debug, Default)]
    pub struct DecimalParameter {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dashboard-decimalparameter.html#cfn-quicksight-dashboard-decimalparameter-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Values`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dashboard-decimalparameter.html#cfn-quicksight-dashboard-decimalparameter-values).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub values: ::ValueList<f64>,
    }

    impl ::codec::SerializeValue for DecimalParameter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", &self.values)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DecimalParameter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DecimalParameter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DecimalParameter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DecimalParameter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut values: Option<::ValueList<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Values" => {
                                values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DecimalParameter {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        values: values.ok_or(::serde::de::Error::missing_field("Values"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::Dashboard.ExportToCSVOption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dashboard-exporttocsvoption.html) property type.
    #[derive(Debug, Default)]
    pub struct ExportToCSVOption {
        /// Property [`AvailabilityStatus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dashboard-exporttocsvoption.html#cfn-quicksight-dashboard-exporttocsvoption-availabilitystatus).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub availability_status: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ExportToCSVOption {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref availability_status) = self.availability_status {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityStatus", availability_status)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ExportToCSVOption {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ExportToCSVOption, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ExportToCSVOption;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ExportToCSVOption")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut availability_status: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AvailabilityStatus" => {
                                availability_status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ExportToCSVOption {
                        availability_status: availability_status,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::Dashboard.IntegerParameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dashboard-integerparameter.html) property type.
    #[derive(Debug, Default)]
    pub struct IntegerParameter {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dashboard-integerparameter.html#cfn-quicksight-dashboard-integerparameter-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Values`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dashboard-integerparameter.html#cfn-quicksight-dashboard-integerparameter-values).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub values: ::ValueList<f64>,
    }

    impl ::codec::SerializeValue for IntegerParameter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", &self.values)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IntegerParameter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IntegerParameter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IntegerParameter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IntegerParameter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut values: Option<::ValueList<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Values" => {
                                values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IntegerParameter {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        values: values.ok_or(::serde::de::Error::missing_field("Values"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::Dashboard.Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dashboard-parameters.html) property type.
    #[derive(Debug, Default)]
    pub struct Parameters {
        /// Property [`DateTimeParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dashboard-parameters.html#cfn-quicksight-dashboard-parameters-datetimeparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub date_time_parameters: Option<::ValueList<DateTimeParameter>>,
        /// Property [`DecimalParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dashboard-parameters.html#cfn-quicksight-dashboard-parameters-decimalparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub decimal_parameters: Option<::ValueList<DecimalParameter>>,
        /// Property [`IntegerParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dashboard-parameters.html#cfn-quicksight-dashboard-parameters-integerparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub integer_parameters: Option<::ValueList<IntegerParameter>>,
        /// Property [`StringParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dashboard-parameters.html#cfn-quicksight-dashboard-parameters-stringparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub string_parameters: Option<::ValueList<StringParameter>>,
    }

    impl ::codec::SerializeValue for Parameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref date_time_parameters) = self.date_time_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DateTimeParameters", date_time_parameters)?;
            }
            if let Some(ref decimal_parameters) = self.decimal_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DecimalParameters", decimal_parameters)?;
            }
            if let Some(ref integer_parameters) = self.integer_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntegerParameters", integer_parameters)?;
            }
            if let Some(ref string_parameters) = self.string_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StringParameters", string_parameters)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Parameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Parameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Parameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Parameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut date_time_parameters: Option<::ValueList<DateTimeParameter>> = None;
                    let mut decimal_parameters: Option<::ValueList<DecimalParameter>> = None;
                    let mut integer_parameters: Option<::ValueList<IntegerParameter>> = None;
                    let mut string_parameters: Option<::ValueList<StringParameter>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DateTimeParameters" => {
                                date_time_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DecimalParameters" => {
                                decimal_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IntegerParameters" => {
                                integer_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StringParameters" => {
                                string_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Parameters {
                        date_time_parameters: date_time_parameters,
                        decimal_parameters: decimal_parameters,
                        integer_parameters: integer_parameters,
                        string_parameters: string_parameters,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::Dashboard.ResourcePermission`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dashboard-resourcepermission.html) property type.
    #[derive(Debug, Default)]
    pub struct ResourcePermission {
        /// Property [`Actions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dashboard-resourcepermission.html#cfn-quicksight-dashboard-resourcepermission-actions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub actions: ::ValueList<String>,
        /// Property [`Principal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dashboard-resourcepermission.html#cfn-quicksight-dashboard-resourcepermission-principal).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub principal: ::Value<String>,
    }

    impl ::codec::SerializeValue for ResourcePermission {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Actions", &self.actions)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Principal", &self.principal)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResourcePermission {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourcePermission, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResourcePermission;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResourcePermission")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut actions: Option<::ValueList<String>> = None;
                    let mut principal: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Actions" => {
                                actions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Principal" => {
                                principal = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResourcePermission {
                        actions: actions.ok_or(::serde::de::Error::missing_field("Actions"))?,
                        principal: principal.ok_or(::serde::de::Error::missing_field("Principal"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::Dashboard.SheetControlsOption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dashboard-sheetcontrolsoption.html) property type.
    #[derive(Debug, Default)]
    pub struct SheetControlsOption {
        /// Property [`VisibilityState`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dashboard-sheetcontrolsoption.html#cfn-quicksight-dashboard-sheetcontrolsoption-visibilitystate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub visibility_state: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SheetControlsOption {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref visibility_state) = self.visibility_state {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VisibilityState", visibility_state)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SheetControlsOption {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SheetControlsOption, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SheetControlsOption;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SheetControlsOption")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut visibility_state: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "VisibilityState" => {
                                visibility_state = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SheetControlsOption {
                        visibility_state: visibility_state,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::Dashboard.StringParameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dashboard-stringparameter.html) property type.
    #[derive(Debug, Default)]
    pub struct StringParameter {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dashboard-stringparameter.html#cfn-quicksight-dashboard-stringparameter-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Values`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dashboard-stringparameter.html#cfn-quicksight-dashboard-stringparameter-values).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub values: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for StringParameter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", &self.values)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StringParameter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StringParameter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StringParameter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StringParameter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut values: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Values" => {
                                values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StringParameter {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        values: values.ok_or(::serde::de::Error::missing_field("Values"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod data_set {
    //! Property types for the `DataSet` resource.

    /// The [`AWS::QuickSight::DataSet.CalculatedColumn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-calculatedcolumn.html) property type.
    #[derive(Debug, Default)]
    pub struct CalculatedColumn {
        /// Property [`ColumnId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-calculatedcolumn.html#cfn-quicksight-dataset-calculatedcolumn-columnid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub column_id: ::Value<String>,
        /// Property [`ColumnName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-calculatedcolumn.html#cfn-quicksight-dataset-calculatedcolumn-columnname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub column_name: ::Value<String>,
        /// Property [`Expression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-calculatedcolumn.html#cfn-quicksight-dataset-calculatedcolumn-expression).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub expression: ::Value<String>,
    }

    impl ::codec::SerializeValue for CalculatedColumn {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColumnId", &self.column_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColumnName", &self.column_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Expression", &self.expression)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CalculatedColumn {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CalculatedColumn, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CalculatedColumn;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CalculatedColumn")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut column_id: Option<::Value<String>> = None;
                    let mut column_name: Option<::Value<String>> = None;
                    let mut expression: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ColumnId" => {
                                column_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ColumnName" => {
                                column_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Expression" => {
                                expression = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CalculatedColumn {
                        column_id: column_id.ok_or(::serde::de::Error::missing_field("ColumnId"))?,
                        column_name: column_name.ok_or(::serde::de::Error::missing_field("ColumnName"))?,
                        expression: expression.ok_or(::serde::de::Error::missing_field("Expression"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSet.CastColumnTypeOperation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-castcolumntypeoperation.html) property type.
    #[derive(Debug, Default)]
    pub struct CastColumnTypeOperation {
        /// Property [`ColumnName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-castcolumntypeoperation.html#cfn-quicksight-dataset-castcolumntypeoperation-columnname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub column_name: ::Value<String>,
        /// Property [`Format`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-castcolumntypeoperation.html#cfn-quicksight-dataset-castcolumntypeoperation-format).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub format: Option<::Value<String>>,
        /// Property [`NewColumnType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-castcolumntypeoperation.html#cfn-quicksight-dataset-castcolumntypeoperation-newcolumntype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub new_column_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for CastColumnTypeOperation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColumnName", &self.column_name)?;
            if let Some(ref format) = self.format {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Format", format)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NewColumnType", &self.new_column_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CastColumnTypeOperation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CastColumnTypeOperation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CastColumnTypeOperation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CastColumnTypeOperation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut column_name: Option<::Value<String>> = None;
                    let mut format: Option<::Value<String>> = None;
                    let mut new_column_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ColumnName" => {
                                column_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Format" => {
                                format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NewColumnType" => {
                                new_column_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CastColumnTypeOperation {
                        column_name: column_name.ok_or(::serde::de::Error::missing_field("ColumnName"))?,
                        format: format,
                        new_column_type: new_column_type.ok_or(::serde::de::Error::missing_field("NewColumnType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSet.ColumnDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-columndescription.html) property type.
    #[derive(Debug, Default)]
    pub struct ColumnDescription {
        /// Property [`Text`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-columndescription.html#cfn-quicksight-dataset-columndescription-text).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ColumnDescription {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref text) = self.text {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Text", text)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ColumnDescription {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ColumnDescription, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ColumnDescription;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ColumnDescription")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut text: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Text" => {
                                text = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ColumnDescription {
                        text: text,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSet.ColumnGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-columngroup.html) property type.
    #[derive(Debug, Default)]
    pub struct ColumnGroup {
        /// Property [`GeoSpatialColumnGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-columngroup.html#cfn-quicksight-dataset-columngroup-geospatialcolumngroup).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub geo_spatial_column_group: Option<::Value<GeoSpatialColumnGroup>>,
    }

    impl ::codec::SerializeValue for ColumnGroup {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref geo_spatial_column_group) = self.geo_spatial_column_group {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GeoSpatialColumnGroup", geo_spatial_column_group)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ColumnGroup {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ColumnGroup, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ColumnGroup;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ColumnGroup")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut geo_spatial_column_group: Option<::Value<GeoSpatialColumnGroup>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "GeoSpatialColumnGroup" => {
                                geo_spatial_column_group = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ColumnGroup {
                        geo_spatial_column_group: geo_spatial_column_group,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSet.ColumnLevelPermissionRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-columnlevelpermissionrule.html) property type.
    #[derive(Debug, Default)]
    pub struct ColumnLevelPermissionRule {
        /// Property [`ColumnNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-columnlevelpermissionrule.html#cfn-quicksight-dataset-columnlevelpermissionrule-columnnames).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub column_names: Option<::ValueList<String>>,
        /// Property [`Principals`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-columnlevelpermissionrule.html#cfn-quicksight-dataset-columnlevelpermissionrule-principals).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub principals: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for ColumnLevelPermissionRule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref column_names) = self.column_names {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColumnNames", column_names)?;
            }
            if let Some(ref principals) = self.principals {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Principals", principals)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ColumnLevelPermissionRule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ColumnLevelPermissionRule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ColumnLevelPermissionRule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ColumnLevelPermissionRule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut column_names: Option<::ValueList<String>> = None;
                    let mut principals: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ColumnNames" => {
                                column_names = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Principals" => {
                                principals = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ColumnLevelPermissionRule {
                        column_names: column_names,
                        principals: principals,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSet.ColumnTag`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-columntag.html) property type.
    #[derive(Debug, Default)]
    pub struct ColumnTag {
        /// Property [`ColumnDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-columntag.html#cfn-quicksight-dataset-columntag-columndescription).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub column_description: Option<::Value<ColumnDescription>>,
        /// Property [`ColumnGeographicRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-columntag.html#cfn-quicksight-dataset-columntag-columngeographicrole).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub column_geographic_role: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ColumnTag {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref column_description) = self.column_description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColumnDescription", column_description)?;
            }
            if let Some(ref column_geographic_role) = self.column_geographic_role {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColumnGeographicRole", column_geographic_role)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ColumnTag {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ColumnTag, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ColumnTag;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ColumnTag")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut column_description: Option<::Value<ColumnDescription>> = None;
                    let mut column_geographic_role: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ColumnDescription" => {
                                column_description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ColumnGeographicRole" => {
                                column_geographic_role = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ColumnTag {
                        column_description: column_description,
                        column_geographic_role: column_geographic_role,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSet.CreateColumnsOperation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-createcolumnsoperation.html) property type.
    #[derive(Debug, Default)]
    pub struct CreateColumnsOperation {
        /// Property [`Columns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-createcolumnsoperation.html#cfn-quicksight-dataset-createcolumnsoperation-columns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub columns: ::ValueList<CalculatedColumn>,
    }

    impl ::codec::SerializeValue for CreateColumnsOperation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Columns", &self.columns)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CreateColumnsOperation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CreateColumnsOperation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CreateColumnsOperation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CreateColumnsOperation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut columns: Option<::ValueList<CalculatedColumn>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Columns" => {
                                columns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CreateColumnsOperation {
                        columns: columns.ok_or(::serde::de::Error::missing_field("Columns"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSet.CustomSql`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-customsql.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomSql {
        /// Property [`Columns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-customsql.html#cfn-quicksight-dataset-customsql-columns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub columns: ::ValueList<InputColumn>,
        /// Property [`DataSourceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-customsql.html#cfn-quicksight-dataset-customsql-datasourcearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_source_arn: ::Value<String>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-customsql.html#cfn-quicksight-dataset-customsql-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`SqlQuery`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-customsql.html#cfn-quicksight-dataset-customsql-sqlquery).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sql_query: ::Value<String>,
    }

    impl ::codec::SerializeValue for CustomSql {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Columns", &self.columns)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataSourceArn", &self.data_source_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SqlQuery", &self.sql_query)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomSql {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomSql, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomSql;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomSql")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut columns: Option<::ValueList<InputColumn>> = None;
                    let mut data_source_arn: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut sql_query: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Columns" => {
                                columns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataSourceArn" => {
                                data_source_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SqlQuery" => {
                                sql_query = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomSql {
                        columns: columns.ok_or(::serde::de::Error::missing_field("Columns"))?,
                        data_source_arn: data_source_arn.ok_or(::serde::de::Error::missing_field("DataSourceArn"))?,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        sql_query: sql_query.ok_or(::serde::de::Error::missing_field("SqlQuery"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSet.FieldFolder`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-fieldfolder.html) property type.
    #[derive(Debug, Default)]
    pub struct FieldFolder {
        /// Property [`Columns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-fieldfolder.html#cfn-quicksight-dataset-fieldfolder-columns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub columns: Option<::ValueList<String>>,
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-fieldfolder.html#cfn-quicksight-dataset-fieldfolder-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for FieldFolder {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref columns) = self.columns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Columns", columns)?;
            }
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FieldFolder {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FieldFolder, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FieldFolder;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FieldFolder")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut columns: Option<::ValueList<String>> = None;
                    let mut description: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Columns" => {
                                columns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FieldFolder {
                        columns: columns,
                        description: description,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSet.FilterOperation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-filteroperation.html) property type.
    #[derive(Debug, Default)]
    pub struct FilterOperation {
        /// Property [`ConditionExpression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-filteroperation.html#cfn-quicksight-dataset-filteroperation-conditionexpression).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub condition_expression: ::Value<String>,
    }

    impl ::codec::SerializeValue for FilterOperation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConditionExpression", &self.condition_expression)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FilterOperation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FilterOperation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FilterOperation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FilterOperation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut condition_expression: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConditionExpression" => {
                                condition_expression = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FilterOperation {
                        condition_expression: condition_expression.ok_or(::serde::de::Error::missing_field("ConditionExpression"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSet.GeoSpatialColumnGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-geospatialcolumngroup.html) property type.
    #[derive(Debug, Default)]
    pub struct GeoSpatialColumnGroup {
        /// Property [`Columns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-geospatialcolumngroup.html#cfn-quicksight-dataset-geospatialcolumngroup-columns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub columns: ::ValueList<String>,
        /// Property [`CountryCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-geospatialcolumngroup.html#cfn-quicksight-dataset-geospatialcolumngroup-countrycode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub country_code: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-geospatialcolumngroup.html#cfn-quicksight-dataset-geospatialcolumngroup-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for GeoSpatialColumnGroup {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Columns", &self.columns)?;
            if let Some(ref country_code) = self.country_code {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CountryCode", country_code)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GeoSpatialColumnGroup {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GeoSpatialColumnGroup, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GeoSpatialColumnGroup;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GeoSpatialColumnGroup")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut columns: Option<::ValueList<String>> = None;
                    let mut country_code: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Columns" => {
                                columns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CountryCode" => {
                                country_code = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GeoSpatialColumnGroup {
                        columns: columns.ok_or(::serde::de::Error::missing_field("Columns"))?,
                        country_code: country_code,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSet.IngestionWaitPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-ingestionwaitpolicy.html) property type.
    #[derive(Debug, Default)]
    pub struct IngestionWaitPolicy {
        /// Property [`IngestionWaitTimeInHours`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-ingestionwaitpolicy.html#cfn-quicksight-dataset-ingestionwaitpolicy-ingestionwaittimeinhours).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ingestion_wait_time_in_hours: Option<::Value<f64>>,
        /// Property [`WaitForSpiceIngestion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-ingestionwaitpolicy.html#cfn-quicksight-dataset-ingestionwaitpolicy-waitforspiceingestion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub wait_for_spice_ingestion: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for IngestionWaitPolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref ingestion_wait_time_in_hours) = self.ingestion_wait_time_in_hours {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IngestionWaitTimeInHours", ingestion_wait_time_in_hours)?;
            }
            if let Some(ref wait_for_spice_ingestion) = self.wait_for_spice_ingestion {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WaitForSpiceIngestion", wait_for_spice_ingestion)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IngestionWaitPolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IngestionWaitPolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IngestionWaitPolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IngestionWaitPolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ingestion_wait_time_in_hours: Option<::Value<f64>> = None;
                    let mut wait_for_spice_ingestion: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "IngestionWaitTimeInHours" => {
                                ingestion_wait_time_in_hours = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WaitForSpiceIngestion" => {
                                wait_for_spice_ingestion = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IngestionWaitPolicy {
                        ingestion_wait_time_in_hours: ingestion_wait_time_in_hours,
                        wait_for_spice_ingestion: wait_for_spice_ingestion,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSet.InputColumn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-inputcolumn.html) property type.
    #[derive(Debug, Default)]
    pub struct InputColumn {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-inputcolumn.html#cfn-quicksight-dataset-inputcolumn-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-inputcolumn.html#cfn-quicksight-dataset-inputcolumn-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for InputColumn {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InputColumn {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InputColumn, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InputColumn;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InputColumn")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InputColumn {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSet.JoinInstruction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-joininstruction.html) property type.
    #[derive(Debug, Default)]
    pub struct JoinInstruction {
        /// Property [`LeftJoinKeyProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-joininstruction.html#cfn-quicksight-dataset-joininstruction-leftjoinkeyproperties).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub left_join_key_properties: Option<::Value<JoinKeyProperties>>,
        /// Property [`LeftOperand`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-joininstruction.html#cfn-quicksight-dataset-joininstruction-leftoperand).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub left_operand: ::Value<String>,
        /// Property [`OnClause`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-joininstruction.html#cfn-quicksight-dataset-joininstruction-onclause).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub on_clause: ::Value<String>,
        /// Property [`RightJoinKeyProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-joininstruction.html#cfn-quicksight-dataset-joininstruction-rightjoinkeyproperties).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub right_join_key_properties: Option<::Value<JoinKeyProperties>>,
        /// Property [`RightOperand`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-joininstruction.html#cfn-quicksight-dataset-joininstruction-rightoperand).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub right_operand: ::Value<String>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-joininstruction.html#cfn-quicksight-dataset-joininstruction-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for JoinInstruction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref left_join_key_properties) = self.left_join_key_properties {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LeftJoinKeyProperties", left_join_key_properties)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LeftOperand", &self.left_operand)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OnClause", &self.on_clause)?;
            if let Some(ref right_join_key_properties) = self.right_join_key_properties {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RightJoinKeyProperties", right_join_key_properties)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RightOperand", &self.right_operand)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for JoinInstruction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<JoinInstruction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = JoinInstruction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type JoinInstruction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut left_join_key_properties: Option<::Value<JoinKeyProperties>> = None;
                    let mut left_operand: Option<::Value<String>> = None;
                    let mut on_clause: Option<::Value<String>> = None;
                    let mut right_join_key_properties: Option<::Value<JoinKeyProperties>> = None;
                    let mut right_operand: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LeftJoinKeyProperties" => {
                                left_join_key_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LeftOperand" => {
                                left_operand = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OnClause" => {
                                on_clause = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RightJoinKeyProperties" => {
                                right_join_key_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RightOperand" => {
                                right_operand = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(JoinInstruction {
                        left_join_key_properties: left_join_key_properties,
                        left_operand: left_operand.ok_or(::serde::de::Error::missing_field("LeftOperand"))?,
                        on_clause: on_clause.ok_or(::serde::de::Error::missing_field("OnClause"))?,
                        right_join_key_properties: right_join_key_properties,
                        right_operand: right_operand.ok_or(::serde::de::Error::missing_field("RightOperand"))?,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSet.JoinKeyProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-joinkeyproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct JoinKeyProperties {
        /// Property [`UniqueKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-joinkeyproperties.html#cfn-quicksight-dataset-joinkeyproperties-uniquekey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unique_key: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for JoinKeyProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref unique_key) = self.unique_key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UniqueKey", unique_key)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for JoinKeyProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<JoinKeyProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = JoinKeyProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type JoinKeyProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut unique_key: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "UniqueKey" => {
                                unique_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(JoinKeyProperties {
                        unique_key: unique_key,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSet.LogicalTable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-logicaltable.html) property type.
    #[derive(Debug, Default)]
    pub struct LogicalTable {
        /// Property [`Alias`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-logicaltable.html#cfn-quicksight-dataset-logicaltable-alias).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub alias: ::Value<String>,
        /// Property [`DataTransforms`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-logicaltable.html#cfn-quicksight-dataset-logicaltable-datatransforms).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_transforms: Option<::ValueList<TransformOperation>>,
        /// Property [`Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-logicaltable.html#cfn-quicksight-dataset-logicaltable-source).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source: ::Value<LogicalTableSource>,
    }

    impl ::codec::SerializeValue for LogicalTable {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Alias", &self.alias)?;
            if let Some(ref data_transforms) = self.data_transforms {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataTransforms", data_transforms)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Source", &self.source)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LogicalTable {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LogicalTable, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LogicalTable;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LogicalTable")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut alias: Option<::Value<String>> = None;
                    let mut data_transforms: Option<::ValueList<TransformOperation>> = None;
                    let mut source: Option<::Value<LogicalTableSource>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Alias" => {
                                alias = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataTransforms" => {
                                data_transforms = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Source" => {
                                source = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LogicalTable {
                        alias: alias.ok_or(::serde::de::Error::missing_field("Alias"))?,
                        data_transforms: data_transforms,
                        source: source.ok_or(::serde::de::Error::missing_field("Source"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSet.LogicalTableSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-logicaltablesource.html) property type.
    #[derive(Debug, Default)]
    pub struct LogicalTableSource {
        /// Property [`JoinInstruction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-logicaltablesource.html#cfn-quicksight-dataset-logicaltablesource-joininstruction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub join_instruction: Option<::Value<JoinInstruction>>,
        /// Property [`PhysicalTableId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-logicaltablesource.html#cfn-quicksight-dataset-logicaltablesource-physicaltableid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub physical_table_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LogicalTableSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref join_instruction) = self.join_instruction {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "JoinInstruction", join_instruction)?;
            }
            if let Some(ref physical_table_id) = self.physical_table_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PhysicalTableId", physical_table_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LogicalTableSource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LogicalTableSource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LogicalTableSource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LogicalTableSource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut join_instruction: Option<::Value<JoinInstruction>> = None;
                    let mut physical_table_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "JoinInstruction" => {
                                join_instruction = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PhysicalTableId" => {
                                physical_table_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LogicalTableSource {
                        join_instruction: join_instruction,
                        physical_table_id: physical_table_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSet.OutputColumn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-outputcolumn.html) property type.
    #[derive(Debug, Default)]
    pub struct OutputColumn {
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-outputcolumn.html#cfn-quicksight-dataset-outputcolumn-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-outputcolumn.html#cfn-quicksight-dataset-outputcolumn-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-outputcolumn.html#cfn-quicksight-dataset-outputcolumn-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for OutputColumn {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref r#type) = self.r#type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OutputColumn {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OutputColumn, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OutputColumn;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OutputColumn")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut description: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OutputColumn {
                        description: description,
                        name: name,
                        r#type: r#type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSet.PhysicalTable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-physicaltable.html) property type.
    #[derive(Debug, Default)]
    pub struct PhysicalTable {
        /// Property [`CustomSql`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-physicaltable.html#cfn-quicksight-dataset-physicaltable-customsql).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_sql: Option<::Value<CustomSql>>,
        /// Property [`RelationalTable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-physicaltable.html#cfn-quicksight-dataset-physicaltable-relationaltable).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub relational_table: Option<::Value<RelationalTable>>,
        /// Property [`S3Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-physicaltable.html#cfn-quicksight-dataset-physicaltable-s3source).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_source: Option<::Value<S3Source>>,
    }

    impl ::codec::SerializeValue for PhysicalTable {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref custom_sql) = self.custom_sql {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomSql", custom_sql)?;
            }
            if let Some(ref relational_table) = self.relational_table {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RelationalTable", relational_table)?;
            }
            if let Some(ref s3_source) = self.s3_source {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Source", s3_source)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PhysicalTable {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PhysicalTable, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PhysicalTable;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PhysicalTable")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut custom_sql: Option<::Value<CustomSql>> = None;
                    let mut relational_table: Option<::Value<RelationalTable>> = None;
                    let mut s3_source: Option<::Value<S3Source>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CustomSql" => {
                                custom_sql = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RelationalTable" => {
                                relational_table = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Source" => {
                                s3_source = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PhysicalTable {
                        custom_sql: custom_sql,
                        relational_table: relational_table,
                        s3_source: s3_source,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSet.ProjectOperation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-projectoperation.html) property type.
    #[derive(Debug, Default)]
    pub struct ProjectOperation {
        /// Property [`ProjectedColumns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-projectoperation.html#cfn-quicksight-dataset-projectoperation-projectedcolumns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub projected_columns: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for ProjectOperation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProjectedColumns", &self.projected_columns)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ProjectOperation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ProjectOperation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ProjectOperation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ProjectOperation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut projected_columns: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ProjectedColumns" => {
                                projected_columns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ProjectOperation {
                        projected_columns: projected_columns.ok_or(::serde::de::Error::missing_field("ProjectedColumns"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSet.RelationalTable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-relationaltable.html) property type.
    #[derive(Debug, Default)]
    pub struct RelationalTable {
        /// Property [`Catalog`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-relationaltable.html#cfn-quicksight-dataset-relationaltable-catalog).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub catalog: Option<::Value<String>>,
        /// Property [`DataSourceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-relationaltable.html#cfn-quicksight-dataset-relationaltable-datasourcearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_source_arn: ::Value<String>,
        /// Property [`InputColumns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-relationaltable.html#cfn-quicksight-dataset-relationaltable-inputcolumns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_columns: ::ValueList<InputColumn>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-relationaltable.html#cfn-quicksight-dataset-relationaltable-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Schema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-relationaltable.html#cfn-quicksight-dataset-relationaltable-schema).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schema: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for RelationalTable {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref catalog) = self.catalog {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Catalog", catalog)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataSourceArn", &self.data_source_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputColumns", &self.input_columns)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            if let Some(ref schema) = self.schema {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Schema", schema)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RelationalTable {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RelationalTable, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RelationalTable;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RelationalTable")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut catalog: Option<::Value<String>> = None;
                    let mut data_source_arn: Option<::Value<String>> = None;
                    let mut input_columns: Option<::ValueList<InputColumn>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut schema: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Catalog" => {
                                catalog = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataSourceArn" => {
                                data_source_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InputColumns" => {
                                input_columns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Schema" => {
                                schema = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RelationalTable {
                        catalog: catalog,
                        data_source_arn: data_source_arn.ok_or(::serde::de::Error::missing_field("DataSourceArn"))?,
                        input_columns: input_columns.ok_or(::serde::de::Error::missing_field("InputColumns"))?,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        schema: schema,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSet.RenameColumnOperation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-renamecolumnoperation.html) property type.
    #[derive(Debug, Default)]
    pub struct RenameColumnOperation {
        /// Property [`ColumnName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-renamecolumnoperation.html#cfn-quicksight-dataset-renamecolumnoperation-columnname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub column_name: ::Value<String>,
        /// Property [`NewColumnName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-renamecolumnoperation.html#cfn-quicksight-dataset-renamecolumnoperation-newcolumnname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub new_column_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for RenameColumnOperation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColumnName", &self.column_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NewColumnName", &self.new_column_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RenameColumnOperation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RenameColumnOperation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RenameColumnOperation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RenameColumnOperation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut column_name: Option<::Value<String>> = None;
                    let mut new_column_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ColumnName" => {
                                column_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NewColumnName" => {
                                new_column_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RenameColumnOperation {
                        column_name: column_name.ok_or(::serde::de::Error::missing_field("ColumnName"))?,
                        new_column_name: new_column_name.ok_or(::serde::de::Error::missing_field("NewColumnName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSet.ResourcePermission`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-resourcepermission.html) property type.
    #[derive(Debug, Default)]
    pub struct ResourcePermission {
        /// Property [`Actions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-resourcepermission.html#cfn-quicksight-dataset-resourcepermission-actions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub actions: ::ValueList<String>,
        /// Property [`Principal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-resourcepermission.html#cfn-quicksight-dataset-resourcepermission-principal).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub principal: ::Value<String>,
    }

    impl ::codec::SerializeValue for ResourcePermission {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Actions", &self.actions)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Principal", &self.principal)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResourcePermission {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourcePermission, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResourcePermission;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResourcePermission")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut actions: Option<::ValueList<String>> = None;
                    let mut principal: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Actions" => {
                                actions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Principal" => {
                                principal = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResourcePermission {
                        actions: actions.ok_or(::serde::de::Error::missing_field("Actions"))?,
                        principal: principal.ok_or(::serde::de::Error::missing_field("Principal"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSet.RowLevelPermissionDataSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-rowlevelpermissiondataset.html) property type.
    #[derive(Debug, Default)]
    pub struct RowLevelPermissionDataSet {
        /// Property [`Arn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-rowlevelpermissiondataset.html#cfn-quicksight-dataset-rowlevelpermissiondataset-arn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub arn: ::Value<String>,
        /// Property [`Namespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-rowlevelpermissiondataset.html#cfn-quicksight-dataset-rowlevelpermissiondataset-namespace).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub namespace: Option<::Value<String>>,
        /// Property [`PermissionPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-rowlevelpermissiondataset.html#cfn-quicksight-dataset-rowlevelpermissiondataset-permissionpolicy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub permission_policy: ::Value<String>,
    }

    impl ::codec::SerializeValue for RowLevelPermissionDataSet {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arn", &self.arn)?;
            if let Some(ref namespace) = self.namespace {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Namespace", namespace)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PermissionPolicy", &self.permission_policy)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RowLevelPermissionDataSet {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RowLevelPermissionDataSet, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RowLevelPermissionDataSet;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RowLevelPermissionDataSet")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arn: Option<::Value<String>> = None;
                    let mut namespace: Option<::Value<String>> = None;
                    let mut permission_policy: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Arn" => {
                                arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Namespace" => {
                                namespace = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PermissionPolicy" => {
                                permission_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RowLevelPermissionDataSet {
                        arn: arn.ok_or(::serde::de::Error::missing_field("Arn"))?,
                        namespace: namespace,
                        permission_policy: permission_policy.ok_or(::serde::de::Error::missing_field("PermissionPolicy"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSet.S3Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-s3source.html) property type.
    #[derive(Debug, Default)]
    pub struct S3Source {
        /// Property [`DataSourceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-s3source.html#cfn-quicksight-dataset-s3source-datasourcearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_source_arn: ::Value<String>,
        /// Property [`InputColumns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-s3source.html#cfn-quicksight-dataset-s3source-inputcolumns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_columns: ::ValueList<InputColumn>,
        /// Property [`UploadSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-s3source.html#cfn-quicksight-dataset-s3source-uploadsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub upload_settings: Option<::Value<UploadSettings>>,
    }

    impl ::codec::SerializeValue for S3Source {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataSourceArn", &self.data_source_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputColumns", &self.input_columns)?;
            if let Some(ref upload_settings) = self.upload_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UploadSettings", upload_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3Source {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3Source, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3Source;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3Source")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_source_arn: Option<::Value<String>> = None;
                    let mut input_columns: Option<::ValueList<InputColumn>> = None;
                    let mut upload_settings: Option<::Value<UploadSettings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataSourceArn" => {
                                data_source_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InputColumns" => {
                                input_columns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UploadSettings" => {
                                upload_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Source {
                        data_source_arn: data_source_arn.ok_or(::serde::de::Error::missing_field("DataSourceArn"))?,
                        input_columns: input_columns.ok_or(::serde::de::Error::missing_field("InputColumns"))?,
                        upload_settings: upload_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSet.TagColumnOperation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-tagcolumnoperation.html) property type.
    #[derive(Debug, Default)]
    pub struct TagColumnOperation {
        /// Property [`ColumnName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-tagcolumnoperation.html#cfn-quicksight-dataset-tagcolumnoperation-columnname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub column_name: ::Value<String>,
        /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-tagcolumnoperation.html#cfn-quicksight-dataset-tagcolumnoperation-tags).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tags: ::ValueList<ColumnTag>,
    }

    impl ::codec::SerializeValue for TagColumnOperation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ColumnName", &self.column_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TagColumnOperation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TagColumnOperation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TagColumnOperation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TagColumnOperation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut column_name: Option<::Value<String>> = None;
                    let mut tags: Option<::ValueList<ColumnTag>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ColumnName" => {
                                column_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Tags" => {
                                tags = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TagColumnOperation {
                        column_name: column_name.ok_or(::serde::de::Error::missing_field("ColumnName"))?,
                        tags: tags.ok_or(::serde::de::Error::missing_field("Tags"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSet.TransformOperation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-transformoperation.html) property type.
    #[derive(Debug, Default)]
    pub struct TransformOperation {
        /// Property [`CastColumnTypeOperation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-transformoperation.html#cfn-quicksight-dataset-transformoperation-castcolumntypeoperation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cast_column_type_operation: Option<::Value<CastColumnTypeOperation>>,
        /// Property [`CreateColumnsOperation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-transformoperation.html#cfn-quicksight-dataset-transformoperation-createcolumnsoperation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub create_columns_operation: Option<::Value<CreateColumnsOperation>>,
        /// Property [`FilterOperation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-transformoperation.html#cfn-quicksight-dataset-transformoperation-filteroperation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub filter_operation: Option<::Value<FilterOperation>>,
        /// Property [`ProjectOperation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-transformoperation.html#cfn-quicksight-dataset-transformoperation-projectoperation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub project_operation: Option<::Value<ProjectOperation>>,
        /// Property [`RenameColumnOperation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-transformoperation.html#cfn-quicksight-dataset-transformoperation-renamecolumnoperation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rename_column_operation: Option<::Value<RenameColumnOperation>>,
        /// Property [`TagColumnOperation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-transformoperation.html#cfn-quicksight-dataset-transformoperation-tagcolumnoperation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tag_column_operation: Option<::Value<TagColumnOperation>>,
    }

    impl ::codec::SerializeValue for TransformOperation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cast_column_type_operation) = self.cast_column_type_operation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CastColumnTypeOperation", cast_column_type_operation)?;
            }
            if let Some(ref create_columns_operation) = self.create_columns_operation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CreateColumnsOperation", create_columns_operation)?;
            }
            if let Some(ref filter_operation) = self.filter_operation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FilterOperation", filter_operation)?;
            }
            if let Some(ref project_operation) = self.project_operation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProjectOperation", project_operation)?;
            }
            if let Some(ref rename_column_operation) = self.rename_column_operation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RenameColumnOperation", rename_column_operation)?;
            }
            if let Some(ref tag_column_operation) = self.tag_column_operation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TagColumnOperation", tag_column_operation)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TransformOperation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TransformOperation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TransformOperation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TransformOperation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cast_column_type_operation: Option<::Value<CastColumnTypeOperation>> = None;
                    let mut create_columns_operation: Option<::Value<CreateColumnsOperation>> = None;
                    let mut filter_operation: Option<::Value<FilterOperation>> = None;
                    let mut project_operation: Option<::Value<ProjectOperation>> = None;
                    let mut rename_column_operation: Option<::Value<RenameColumnOperation>> = None;
                    let mut tag_column_operation: Option<::Value<TagColumnOperation>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CastColumnTypeOperation" => {
                                cast_column_type_operation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CreateColumnsOperation" => {
                                create_columns_operation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FilterOperation" => {
                                filter_operation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProjectOperation" => {
                                project_operation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RenameColumnOperation" => {
                                rename_column_operation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TagColumnOperation" => {
                                tag_column_operation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TransformOperation {
                        cast_column_type_operation: cast_column_type_operation,
                        create_columns_operation: create_columns_operation,
                        filter_operation: filter_operation,
                        project_operation: project_operation,
                        rename_column_operation: rename_column_operation,
                        tag_column_operation: tag_column_operation,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSet.UploadSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-uploadsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct UploadSettings {
        /// Property [`ContainsHeader`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-uploadsettings.html#cfn-quicksight-dataset-uploadsettings-containsheader).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub contains_header: Option<::Value<bool>>,
        /// Property [`Delimiter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-uploadsettings.html#cfn-quicksight-dataset-uploadsettings-delimiter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub delimiter: Option<::Value<String>>,
        /// Property [`Format`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-uploadsettings.html#cfn-quicksight-dataset-uploadsettings-format).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub format: Option<::Value<String>>,
        /// Property [`StartFromRow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-uploadsettings.html#cfn-quicksight-dataset-uploadsettings-startfromrow).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub start_from_row: Option<::Value<f64>>,
        /// Property [`TextQualifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-dataset-uploadsettings.html#cfn-quicksight-dataset-uploadsettings-textqualifier).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text_qualifier: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for UploadSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref contains_header) = self.contains_header {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainsHeader", contains_header)?;
            }
            if let Some(ref delimiter) = self.delimiter {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Delimiter", delimiter)?;
            }
            if let Some(ref format) = self.format {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Format", format)?;
            }
            if let Some(ref start_from_row) = self.start_from_row {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartFromRow", start_from_row)?;
            }
            if let Some(ref text_qualifier) = self.text_qualifier {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextQualifier", text_qualifier)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for UploadSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<UploadSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = UploadSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type UploadSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut contains_header: Option<::Value<bool>> = None;
                    let mut delimiter: Option<::Value<String>> = None;
                    let mut format: Option<::Value<String>> = None;
                    let mut start_from_row: Option<::Value<f64>> = None;
                    let mut text_qualifier: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContainsHeader" => {
                                contains_header = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Delimiter" => {
                                delimiter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Format" => {
                                format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StartFromRow" => {
                                start_from_row = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TextQualifier" => {
                                text_qualifier = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(UploadSettings {
                        contains_header: contains_header,
                        delimiter: delimiter,
                        format: format,
                        start_from_row: start_from_row,
                        text_qualifier: text_qualifier,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod data_source {
    //! Property types for the `DataSource` resource.

    /// The [`AWS::QuickSight::DataSource.AmazonElasticsearchParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-amazonelasticsearchparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct AmazonElasticsearchParameters {
        /// Property [`Domain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-amazonelasticsearchparameters.html#cfn-quicksight-datasource-amazonelasticsearchparameters-domain).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub domain: ::Value<String>,
    }

    impl ::codec::SerializeValue for AmazonElasticsearchParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Domain", &self.domain)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AmazonElasticsearchParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AmazonElasticsearchParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AmazonElasticsearchParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AmazonElasticsearchParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut domain: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Domain" => {
                                domain = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AmazonElasticsearchParameters {
                        domain: domain.ok_or(::serde::de::Error::missing_field("Domain"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSource.AthenaParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-athenaparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct AthenaParameters {
        /// Property [`WorkGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-athenaparameters.html#cfn-quicksight-datasource-athenaparameters-workgroup).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub work_group: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AthenaParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref work_group) = self.work_group {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkGroup", work_group)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AthenaParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AthenaParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AthenaParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AthenaParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut work_group: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "WorkGroup" => {
                                work_group = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AthenaParameters {
                        work_group: work_group,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSource.AuroraParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-auroraparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct AuroraParameters {
        /// Property [`Database`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-auroraparameters.html#cfn-quicksight-datasource-auroraparameters-database).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database: ::Value<String>,
        /// Property [`Host`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-auroraparameters.html#cfn-quicksight-datasource-auroraparameters-host).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub host: ::Value<String>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-auroraparameters.html#cfn-quicksight-datasource-auroraparameters-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: ::Value<f64>,
    }

    impl ::codec::SerializeValue for AuroraParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Database", &self.database)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Host", &self.host)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", &self.port)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AuroraParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AuroraParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AuroraParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AuroraParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut database: Option<::Value<String>> = None;
                    let mut host: Option<::Value<String>> = None;
                    let mut port: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Database" => {
                                database = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Host" => {
                                host = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AuroraParameters {
                        database: database.ok_or(::serde::de::Error::missing_field("Database"))?,
                        host: host.ok_or(::serde::de::Error::missing_field("Host"))?,
                        port: port.ok_or(::serde::de::Error::missing_field("Port"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSource.AuroraPostgreSqlParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-aurorapostgresqlparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct AuroraPostgreSqlParameters {
        /// Property [`Database`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-aurorapostgresqlparameters.html#cfn-quicksight-datasource-aurorapostgresqlparameters-database).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database: ::Value<String>,
        /// Property [`Host`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-aurorapostgresqlparameters.html#cfn-quicksight-datasource-aurorapostgresqlparameters-host).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub host: ::Value<String>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-aurorapostgresqlparameters.html#cfn-quicksight-datasource-aurorapostgresqlparameters-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: ::Value<f64>,
    }

    impl ::codec::SerializeValue for AuroraPostgreSqlParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Database", &self.database)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Host", &self.host)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", &self.port)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AuroraPostgreSqlParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AuroraPostgreSqlParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AuroraPostgreSqlParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AuroraPostgreSqlParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut database: Option<::Value<String>> = None;
                    let mut host: Option<::Value<String>> = None;
                    let mut port: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Database" => {
                                database = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Host" => {
                                host = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AuroraPostgreSqlParameters {
                        database: database.ok_or(::serde::de::Error::missing_field("Database"))?,
                        host: host.ok_or(::serde::de::Error::missing_field("Host"))?,
                        port: port.ok_or(::serde::de::Error::missing_field("Port"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSource.CredentialPair`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-credentialpair.html) property type.
    #[derive(Debug, Default)]
    pub struct CredentialPair {
        /// Property [`AlternateDataSourceParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-credentialpair.html#cfn-quicksight-datasource-credentialpair-alternatedatasourceparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub alternate_data_source_parameters: Option<::ValueList<DataSourceParameters>>,
        /// Property [`Password`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-credentialpair.html#cfn-quicksight-datasource-credentialpair-password).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub password: ::Value<String>,
        /// Property [`Username`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-credentialpair.html#cfn-quicksight-datasource-credentialpair-username).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub username: ::Value<String>,
    }

    impl ::codec::SerializeValue for CredentialPair {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref alternate_data_source_parameters) = self.alternate_data_source_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AlternateDataSourceParameters", alternate_data_source_parameters)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Password", &self.password)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Username", &self.username)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CredentialPair {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CredentialPair, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CredentialPair;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CredentialPair")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut alternate_data_source_parameters: Option<::ValueList<DataSourceParameters>> = None;
                    let mut password: Option<::Value<String>> = None;
                    let mut username: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AlternateDataSourceParameters" => {
                                alternate_data_source_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Password" => {
                                password = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Username" => {
                                username = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CredentialPair {
                        alternate_data_source_parameters: alternate_data_source_parameters,
                        password: password.ok_or(::serde::de::Error::missing_field("Password"))?,
                        username: username.ok_or(::serde::de::Error::missing_field("Username"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSource.DataSourceCredentials`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-datasourcecredentials.html) property type.
    #[derive(Debug, Default)]
    pub struct DataSourceCredentials {
        /// Property [`CopySourceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-datasourcecredentials.html#cfn-quicksight-datasource-datasourcecredentials-copysourcearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub copy_source_arn: Option<::Value<String>>,
        /// Property [`CredentialPair`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-datasourcecredentials.html#cfn-quicksight-datasource-datasourcecredentials-credentialpair).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub credential_pair: Option<::Value<CredentialPair>>,
    }

    impl ::codec::SerializeValue for DataSourceCredentials {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref copy_source_arn) = self.copy_source_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CopySourceArn", copy_source_arn)?;
            }
            if let Some(ref credential_pair) = self.credential_pair {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CredentialPair", credential_pair)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataSourceCredentials {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataSourceCredentials, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataSourceCredentials;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataSourceCredentials")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut copy_source_arn: Option<::Value<String>> = None;
                    let mut credential_pair: Option<::Value<CredentialPair>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CopySourceArn" => {
                                copy_source_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CredentialPair" => {
                                credential_pair = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataSourceCredentials {
                        copy_source_arn: copy_source_arn,
                        credential_pair: credential_pair,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSource.DataSourceErrorInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-datasourceerrorinfo.html) property type.
    #[derive(Debug, Default)]
    pub struct DataSourceErrorInfo {
        /// Property [`Message`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-datasourceerrorinfo.html#cfn-quicksight-datasource-datasourceerrorinfo-message).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub message: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-datasourceerrorinfo.html#cfn-quicksight-datasource-datasourceerrorinfo-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DataSourceErrorInfo {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref message) = self.message {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Message", message)?;
            }
            if let Some(ref r#type) = self.r#type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataSourceErrorInfo {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataSourceErrorInfo, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataSourceErrorInfo;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataSourceErrorInfo")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut message: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Message" => {
                                message = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataSourceErrorInfo {
                        message: message,
                        r#type: r#type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSource.DataSourceParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-datasourceparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct DataSourceParameters {
        /// Property [`AmazonElasticsearchParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-datasourceparameters.html#cfn-quicksight-datasource-datasourceparameters-amazonelasticsearchparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub amazon_elasticsearch_parameters: Option<::Value<AmazonElasticsearchParameters>>,
        /// Property [`AthenaParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-datasourceparameters.html#cfn-quicksight-datasource-datasourceparameters-athenaparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub athena_parameters: Option<::Value<AthenaParameters>>,
        /// Property [`AuroraParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-datasourceparameters.html#cfn-quicksight-datasource-datasourceparameters-auroraparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aurora_parameters: Option<::Value<AuroraParameters>>,
        /// Property [`AuroraPostgreSqlParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-datasourceparameters.html#cfn-quicksight-datasource-datasourceparameters-aurorapostgresqlparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aurora_postgre_sql_parameters: Option<::Value<AuroraPostgreSqlParameters>>,
        /// Property [`MariaDbParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-datasourceparameters.html#cfn-quicksight-datasource-datasourceparameters-mariadbparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub maria_db_parameters: Option<::Value<MariaDbParameters>>,
        /// Property [`MySqlParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-datasourceparameters.html#cfn-quicksight-datasource-datasourceparameters-mysqlparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub my_sql_parameters: Option<::Value<MySqlParameters>>,
        /// Property [`OracleParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-datasourceparameters.html#cfn-quicksight-datasource-datasourceparameters-oracleparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub oracle_parameters: Option<::Value<OracleParameters>>,
        /// Property [`PostgreSqlParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-datasourceparameters.html#cfn-quicksight-datasource-datasourceparameters-postgresqlparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub postgre_sql_parameters: Option<::Value<PostgreSqlParameters>>,
        /// Property [`PrestoParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-datasourceparameters.html#cfn-quicksight-datasource-datasourceparameters-prestoparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub presto_parameters: Option<::Value<PrestoParameters>>,
        /// Property [`RdsParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-datasourceparameters.html#cfn-quicksight-datasource-datasourceparameters-rdsparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rds_parameters: Option<::Value<RdsParameters>>,
        /// Property [`RedshiftParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-datasourceparameters.html#cfn-quicksight-datasource-datasourceparameters-redshiftparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub redshift_parameters: Option<::Value<RedshiftParameters>>,
        /// Property [`S3Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-datasourceparameters.html#cfn-quicksight-datasource-datasourceparameters-s3parameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_parameters: Option<::Value<S3Parameters>>,
        /// Property [`SnowflakeParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-datasourceparameters.html#cfn-quicksight-datasource-datasourceparameters-snowflakeparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub snowflake_parameters: Option<::Value<SnowflakeParameters>>,
        /// Property [`SparkParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-datasourceparameters.html#cfn-quicksight-datasource-datasourceparameters-sparkparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub spark_parameters: Option<::Value<SparkParameters>>,
        /// Property [`SqlServerParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-datasourceparameters.html#cfn-quicksight-datasource-datasourceparameters-sqlserverparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sql_server_parameters: Option<::Value<SqlServerParameters>>,
        /// Property [`TeradataParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-datasourceparameters.html#cfn-quicksight-datasource-datasourceparameters-teradataparameters).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub teradata_parameters: Option<::Value<TeradataParameters>>,
    }

    impl ::codec::SerializeValue for DataSourceParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref amazon_elasticsearch_parameters) = self.amazon_elasticsearch_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AmazonElasticsearchParameters", amazon_elasticsearch_parameters)?;
            }
            if let Some(ref athena_parameters) = self.athena_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AthenaParameters", athena_parameters)?;
            }
            if let Some(ref aurora_parameters) = self.aurora_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuroraParameters", aurora_parameters)?;
            }
            if let Some(ref aurora_postgre_sql_parameters) = self.aurora_postgre_sql_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuroraPostgreSqlParameters", aurora_postgre_sql_parameters)?;
            }
            if let Some(ref maria_db_parameters) = self.maria_db_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MariaDbParameters", maria_db_parameters)?;
            }
            if let Some(ref my_sql_parameters) = self.my_sql_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MySqlParameters", my_sql_parameters)?;
            }
            if let Some(ref oracle_parameters) = self.oracle_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OracleParameters", oracle_parameters)?;
            }
            if let Some(ref postgre_sql_parameters) = self.postgre_sql_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PostgreSqlParameters", postgre_sql_parameters)?;
            }
            if let Some(ref presto_parameters) = self.presto_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrestoParameters", presto_parameters)?;
            }
            if let Some(ref rds_parameters) = self.rds_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RdsParameters", rds_parameters)?;
            }
            if let Some(ref redshift_parameters) = self.redshift_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RedshiftParameters", redshift_parameters)?;
            }
            if let Some(ref s3_parameters) = self.s3_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Parameters", s3_parameters)?;
            }
            if let Some(ref snowflake_parameters) = self.snowflake_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnowflakeParameters", snowflake_parameters)?;
            }
            if let Some(ref spark_parameters) = self.spark_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SparkParameters", spark_parameters)?;
            }
            if let Some(ref sql_server_parameters) = self.sql_server_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SqlServerParameters", sql_server_parameters)?;
            }
            if let Some(ref teradata_parameters) = self.teradata_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TeradataParameters", teradata_parameters)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataSourceParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataSourceParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataSourceParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataSourceParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut amazon_elasticsearch_parameters: Option<::Value<AmazonElasticsearchParameters>> = None;
                    let mut athena_parameters: Option<::Value<AthenaParameters>> = None;
                    let mut aurora_parameters: Option<::Value<AuroraParameters>> = None;
                    let mut aurora_postgre_sql_parameters: Option<::Value<AuroraPostgreSqlParameters>> = None;
                    let mut maria_db_parameters: Option<::Value<MariaDbParameters>> = None;
                    let mut my_sql_parameters: Option<::Value<MySqlParameters>> = None;
                    let mut oracle_parameters: Option<::Value<OracleParameters>> = None;
                    let mut postgre_sql_parameters: Option<::Value<PostgreSqlParameters>> = None;
                    let mut presto_parameters: Option<::Value<PrestoParameters>> = None;
                    let mut rds_parameters: Option<::Value<RdsParameters>> = None;
                    let mut redshift_parameters: Option<::Value<RedshiftParameters>> = None;
                    let mut s3_parameters: Option<::Value<S3Parameters>> = None;
                    let mut snowflake_parameters: Option<::Value<SnowflakeParameters>> = None;
                    let mut spark_parameters: Option<::Value<SparkParameters>> = None;
                    let mut sql_server_parameters: Option<::Value<SqlServerParameters>> = None;
                    let mut teradata_parameters: Option<::Value<TeradataParameters>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AmazonElasticsearchParameters" => {
                                amazon_elasticsearch_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AthenaParameters" => {
                                athena_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AuroraParameters" => {
                                aurora_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AuroraPostgreSqlParameters" => {
                                aurora_postgre_sql_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MariaDbParameters" => {
                                maria_db_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MySqlParameters" => {
                                my_sql_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OracleParameters" => {
                                oracle_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PostgreSqlParameters" => {
                                postgre_sql_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PrestoParameters" => {
                                presto_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RdsParameters" => {
                                rds_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RedshiftParameters" => {
                                redshift_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Parameters" => {
                                s3_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SnowflakeParameters" => {
                                snowflake_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SparkParameters" => {
                                spark_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SqlServerParameters" => {
                                sql_server_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TeradataParameters" => {
                                teradata_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataSourceParameters {
                        amazon_elasticsearch_parameters: amazon_elasticsearch_parameters,
                        athena_parameters: athena_parameters,
                        aurora_parameters: aurora_parameters,
                        aurora_postgre_sql_parameters: aurora_postgre_sql_parameters,
                        maria_db_parameters: maria_db_parameters,
                        my_sql_parameters: my_sql_parameters,
                        oracle_parameters: oracle_parameters,
                        postgre_sql_parameters: postgre_sql_parameters,
                        presto_parameters: presto_parameters,
                        rds_parameters: rds_parameters,
                        redshift_parameters: redshift_parameters,
                        s3_parameters: s3_parameters,
                        snowflake_parameters: snowflake_parameters,
                        spark_parameters: spark_parameters,
                        sql_server_parameters: sql_server_parameters,
                        teradata_parameters: teradata_parameters,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSource.ManifestFileLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-manifestfilelocation.html) property type.
    #[derive(Debug, Default)]
    pub struct ManifestFileLocation {
        /// Property [`Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-manifestfilelocation.html#cfn-quicksight-datasource-manifestfilelocation-bucket).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket: ::Value<String>,
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-manifestfilelocation.html#cfn-quicksight-datasource-manifestfilelocation-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
    }

    impl ::codec::SerializeValue for ManifestFileLocation {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bucket", &self.bucket)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ManifestFileLocation {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ManifestFileLocation, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ManifestFileLocation;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ManifestFileLocation")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket: Option<::Value<String>> = None;
                    let mut key: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Bucket" => {
                                bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ManifestFileLocation {
                        bucket: bucket.ok_or(::serde::de::Error::missing_field("Bucket"))?,
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSource.MariaDbParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-mariadbparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct MariaDbParameters {
        /// Property [`Database`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-mariadbparameters.html#cfn-quicksight-datasource-mariadbparameters-database).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database: ::Value<String>,
        /// Property [`Host`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-mariadbparameters.html#cfn-quicksight-datasource-mariadbparameters-host).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub host: ::Value<String>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-mariadbparameters.html#cfn-quicksight-datasource-mariadbparameters-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: ::Value<f64>,
    }

    impl ::codec::SerializeValue for MariaDbParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Database", &self.database)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Host", &self.host)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", &self.port)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MariaDbParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MariaDbParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MariaDbParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MariaDbParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut database: Option<::Value<String>> = None;
                    let mut host: Option<::Value<String>> = None;
                    let mut port: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Database" => {
                                database = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Host" => {
                                host = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MariaDbParameters {
                        database: database.ok_or(::serde::de::Error::missing_field("Database"))?,
                        host: host.ok_or(::serde::de::Error::missing_field("Host"))?,
                        port: port.ok_or(::serde::de::Error::missing_field("Port"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSource.MySqlParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-mysqlparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct MySqlParameters {
        /// Property [`Database`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-mysqlparameters.html#cfn-quicksight-datasource-mysqlparameters-database).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database: ::Value<String>,
        /// Property [`Host`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-mysqlparameters.html#cfn-quicksight-datasource-mysqlparameters-host).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub host: ::Value<String>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-mysqlparameters.html#cfn-quicksight-datasource-mysqlparameters-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: ::Value<f64>,
    }

    impl ::codec::SerializeValue for MySqlParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Database", &self.database)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Host", &self.host)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", &self.port)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MySqlParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MySqlParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MySqlParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MySqlParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut database: Option<::Value<String>> = None;
                    let mut host: Option<::Value<String>> = None;
                    let mut port: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Database" => {
                                database = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Host" => {
                                host = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MySqlParameters {
                        database: database.ok_or(::serde::de::Error::missing_field("Database"))?,
                        host: host.ok_or(::serde::de::Error::missing_field("Host"))?,
                        port: port.ok_or(::serde::de::Error::missing_field("Port"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSource.OracleParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-oracleparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct OracleParameters {
        /// Property [`Database`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-oracleparameters.html#cfn-quicksight-datasource-oracleparameters-database).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database: ::Value<String>,
        /// Property [`Host`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-oracleparameters.html#cfn-quicksight-datasource-oracleparameters-host).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub host: ::Value<String>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-oracleparameters.html#cfn-quicksight-datasource-oracleparameters-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: ::Value<f64>,
    }

    impl ::codec::SerializeValue for OracleParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Database", &self.database)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Host", &self.host)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", &self.port)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OracleParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OracleParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OracleParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OracleParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut database: Option<::Value<String>> = None;
                    let mut host: Option<::Value<String>> = None;
                    let mut port: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Database" => {
                                database = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Host" => {
                                host = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OracleParameters {
                        database: database.ok_or(::serde::de::Error::missing_field("Database"))?,
                        host: host.ok_or(::serde::de::Error::missing_field("Host"))?,
                        port: port.ok_or(::serde::de::Error::missing_field("Port"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSource.PostgreSqlParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-postgresqlparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct PostgreSqlParameters {
        /// Property [`Database`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-postgresqlparameters.html#cfn-quicksight-datasource-postgresqlparameters-database).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database: ::Value<String>,
        /// Property [`Host`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-postgresqlparameters.html#cfn-quicksight-datasource-postgresqlparameters-host).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub host: ::Value<String>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-postgresqlparameters.html#cfn-quicksight-datasource-postgresqlparameters-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: ::Value<f64>,
    }

    impl ::codec::SerializeValue for PostgreSqlParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Database", &self.database)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Host", &self.host)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", &self.port)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PostgreSqlParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PostgreSqlParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PostgreSqlParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PostgreSqlParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut database: Option<::Value<String>> = None;
                    let mut host: Option<::Value<String>> = None;
                    let mut port: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Database" => {
                                database = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Host" => {
                                host = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PostgreSqlParameters {
                        database: database.ok_or(::serde::de::Error::missing_field("Database"))?,
                        host: host.ok_or(::serde::de::Error::missing_field("Host"))?,
                        port: port.ok_or(::serde::de::Error::missing_field("Port"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSource.PrestoParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-prestoparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct PrestoParameters {
        /// Property [`Catalog`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-prestoparameters.html#cfn-quicksight-datasource-prestoparameters-catalog).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub catalog: ::Value<String>,
        /// Property [`Host`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-prestoparameters.html#cfn-quicksight-datasource-prestoparameters-host).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub host: ::Value<String>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-prestoparameters.html#cfn-quicksight-datasource-prestoparameters-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: ::Value<f64>,
    }

    impl ::codec::SerializeValue for PrestoParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Catalog", &self.catalog)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Host", &self.host)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", &self.port)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PrestoParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PrestoParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PrestoParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PrestoParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut catalog: Option<::Value<String>> = None;
                    let mut host: Option<::Value<String>> = None;
                    let mut port: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Catalog" => {
                                catalog = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Host" => {
                                host = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PrestoParameters {
                        catalog: catalog.ok_or(::serde::de::Error::missing_field("Catalog"))?,
                        host: host.ok_or(::serde::de::Error::missing_field("Host"))?,
                        port: port.ok_or(::serde::de::Error::missing_field("Port"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSource.RdsParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-rdsparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct RdsParameters {
        /// Property [`Database`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-rdsparameters.html#cfn-quicksight-datasource-rdsparameters-database).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database: ::Value<String>,
        /// Property [`InstanceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-rdsparameters.html#cfn-quicksight-datasource-rdsparameters-instanceid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub instance_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for RdsParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Database", &self.database)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceId", &self.instance_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RdsParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RdsParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RdsParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RdsParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut database: Option<::Value<String>> = None;
                    let mut instance_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Database" => {
                                database = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InstanceId" => {
                                instance_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RdsParameters {
                        database: database.ok_or(::serde::de::Error::missing_field("Database"))?,
                        instance_id: instance_id.ok_or(::serde::de::Error::missing_field("InstanceId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSource.RedshiftParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-redshiftparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct RedshiftParameters {
        /// Property [`ClusterId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-redshiftparameters.html#cfn-quicksight-datasource-redshiftparameters-clusterid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cluster_id: Option<::Value<String>>,
        /// Property [`Database`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-redshiftparameters.html#cfn-quicksight-datasource-redshiftparameters-database).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database: ::Value<String>,
        /// Property [`Host`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-redshiftparameters.html#cfn-quicksight-datasource-redshiftparameters-host).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub host: Option<::Value<String>>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-redshiftparameters.html#cfn-quicksight-datasource-redshiftparameters-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: Option<::Value<f64>>,
    }

    impl ::codec::SerializeValue for RedshiftParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cluster_id) = self.cluster_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterId", cluster_id)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Database", &self.database)?;
            if let Some(ref host) = self.host {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Host", host)?;
            }
            if let Some(ref port) = self.port {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", port)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RedshiftParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RedshiftParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RedshiftParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RedshiftParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cluster_id: Option<::Value<String>> = None;
                    let mut database: Option<::Value<String>> = None;
                    let mut host: Option<::Value<String>> = None;
                    let mut port: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ClusterId" => {
                                cluster_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Database" => {
                                database = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Host" => {
                                host = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RedshiftParameters {
                        cluster_id: cluster_id,
                        database: database.ok_or(::serde::de::Error::missing_field("Database"))?,
                        host: host,
                        port: port,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSource.ResourcePermission`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-resourcepermission.html) property type.
    #[derive(Debug, Default)]
    pub struct ResourcePermission {
        /// Property [`Actions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-resourcepermission.html#cfn-quicksight-datasource-resourcepermission-actions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub actions: ::ValueList<String>,
        /// Property [`Principal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-resourcepermission.html#cfn-quicksight-datasource-resourcepermission-principal).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub principal: ::Value<String>,
    }

    impl ::codec::SerializeValue for ResourcePermission {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Actions", &self.actions)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Principal", &self.principal)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResourcePermission {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourcePermission, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResourcePermission;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResourcePermission")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut actions: Option<::ValueList<String>> = None;
                    let mut principal: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Actions" => {
                                actions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Principal" => {
                                principal = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResourcePermission {
                        actions: actions.ok_or(::serde::de::Error::missing_field("Actions"))?,
                        principal: principal.ok_or(::serde::de::Error::missing_field("Principal"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSource.S3Parameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-s3parameters.html) property type.
    #[derive(Debug, Default)]
    pub struct S3Parameters {
        /// Property [`ManifestFileLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-s3parameters.html#cfn-quicksight-datasource-s3parameters-manifestfilelocation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub manifest_file_location: ::Value<ManifestFileLocation>,
    }

    impl ::codec::SerializeValue for S3Parameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManifestFileLocation", &self.manifest_file_location)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3Parameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3Parameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3Parameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3Parameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut manifest_file_location: Option<::Value<ManifestFileLocation>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ManifestFileLocation" => {
                                manifest_file_location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Parameters {
                        manifest_file_location: manifest_file_location.ok_or(::serde::de::Error::missing_field("ManifestFileLocation"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSource.SnowflakeParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-snowflakeparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct SnowflakeParameters {
        /// Property [`Database`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-snowflakeparameters.html#cfn-quicksight-datasource-snowflakeparameters-database).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database: ::Value<String>,
        /// Property [`Host`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-snowflakeparameters.html#cfn-quicksight-datasource-snowflakeparameters-host).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub host: ::Value<String>,
        /// Property [`Warehouse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-snowflakeparameters.html#cfn-quicksight-datasource-snowflakeparameters-warehouse).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub warehouse: ::Value<String>,
    }

    impl ::codec::SerializeValue for SnowflakeParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Database", &self.database)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Host", &self.host)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Warehouse", &self.warehouse)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SnowflakeParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SnowflakeParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SnowflakeParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SnowflakeParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut database: Option<::Value<String>> = None;
                    let mut host: Option<::Value<String>> = None;
                    let mut warehouse: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Database" => {
                                database = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Host" => {
                                host = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Warehouse" => {
                                warehouse = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SnowflakeParameters {
                        database: database.ok_or(::serde::de::Error::missing_field("Database"))?,
                        host: host.ok_or(::serde::de::Error::missing_field("Host"))?,
                        warehouse: warehouse.ok_or(::serde::de::Error::missing_field("Warehouse"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSource.SparkParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-sparkparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct SparkParameters {
        /// Property [`Host`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-sparkparameters.html#cfn-quicksight-datasource-sparkparameters-host).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub host: ::Value<String>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-sparkparameters.html#cfn-quicksight-datasource-sparkparameters-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: ::Value<f64>,
    }

    impl ::codec::SerializeValue for SparkParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Host", &self.host)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", &self.port)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SparkParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SparkParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SparkParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SparkParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut host: Option<::Value<String>> = None;
                    let mut port: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Host" => {
                                host = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SparkParameters {
                        host: host.ok_or(::serde::de::Error::missing_field("Host"))?,
                        port: port.ok_or(::serde::de::Error::missing_field("Port"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSource.SqlServerParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-sqlserverparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct SqlServerParameters {
        /// Property [`Database`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-sqlserverparameters.html#cfn-quicksight-datasource-sqlserverparameters-database).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database: ::Value<String>,
        /// Property [`Host`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-sqlserverparameters.html#cfn-quicksight-datasource-sqlserverparameters-host).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub host: ::Value<String>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-sqlserverparameters.html#cfn-quicksight-datasource-sqlserverparameters-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: ::Value<f64>,
    }

    impl ::codec::SerializeValue for SqlServerParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Database", &self.database)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Host", &self.host)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", &self.port)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SqlServerParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SqlServerParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SqlServerParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SqlServerParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut database: Option<::Value<String>> = None;
                    let mut host: Option<::Value<String>> = None;
                    let mut port: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Database" => {
                                database = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Host" => {
                                host = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SqlServerParameters {
                        database: database.ok_or(::serde::de::Error::missing_field("Database"))?,
                        host: host.ok_or(::serde::de::Error::missing_field("Host"))?,
                        port: port.ok_or(::serde::de::Error::missing_field("Port"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSource.SslProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-sslproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct SslProperties {
        /// Property [`DisableSsl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-sslproperties.html#cfn-quicksight-datasource-sslproperties-disablessl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub disable_ssl: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for SslProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref disable_ssl) = self.disable_ssl {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisableSsl", disable_ssl)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SslProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SslProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SslProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SslProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut disable_ssl: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DisableSsl" => {
                                disable_ssl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SslProperties {
                        disable_ssl: disable_ssl,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSource.TeradataParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-teradataparameters.html) property type.
    #[derive(Debug, Default)]
    pub struct TeradataParameters {
        /// Property [`Database`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-teradataparameters.html#cfn-quicksight-datasource-teradataparameters-database).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database: ::Value<String>,
        /// Property [`Host`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-teradataparameters.html#cfn-quicksight-datasource-teradataparameters-host).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub host: ::Value<String>,
        /// Property [`Port`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-teradataparameters.html#cfn-quicksight-datasource-teradataparameters-port).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub port: ::Value<f64>,
    }

    impl ::codec::SerializeValue for TeradataParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Database", &self.database)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Host", &self.host)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", &self.port)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TeradataParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TeradataParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TeradataParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TeradataParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut database: Option<::Value<String>> = None;
                    let mut host: Option<::Value<String>> = None;
                    let mut port: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Database" => {
                                database = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Host" => {
                                host = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Port" => {
                                port = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TeradataParameters {
                        database: database.ok_or(::serde::de::Error::missing_field("Database"))?,
                        host: host.ok_or(::serde::de::Error::missing_field("Host"))?,
                        port: port.ok_or(::serde::de::Error::missing_field("Port"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::DataSource.VpcConnectionProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-vpcconnectionproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct VpcConnectionProperties {
        /// Property [`VpcConnectionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-datasource-vpcconnectionproperties.html#cfn-quicksight-datasource-vpcconnectionproperties-vpcconnectionarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vpc_connection_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for VpcConnectionProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcConnectionArn", &self.vpc_connection_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VpcConnectionProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VpcConnectionProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VpcConnectionProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VpcConnectionProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut vpc_connection_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "VpcConnectionArn" => {
                                vpc_connection_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VpcConnectionProperties {
                        vpc_connection_arn: vpc_connection_arn.ok_or(::serde::de::Error::missing_field("VpcConnectionArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod template {
    //! Property types for the `Template` resource.

    /// The [`AWS::QuickSight::Template.DataSetReference`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-template-datasetreference.html) property type.
    #[derive(Debug, Default)]
    pub struct DataSetReference {
        /// Property [`DataSetArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-template-datasetreference.html#cfn-quicksight-template-datasetreference-datasetarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_set_arn: ::Value<String>,
        /// Property [`DataSetPlaceholder`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-template-datasetreference.html#cfn-quicksight-template-datasetreference-datasetplaceholder).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_set_placeholder: ::Value<String>,
    }

    impl ::codec::SerializeValue for DataSetReference {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataSetArn", &self.data_set_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataSetPlaceholder", &self.data_set_placeholder)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataSetReference {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataSetReference, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataSetReference;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataSetReference")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_set_arn: Option<::Value<String>> = None;
                    let mut data_set_placeholder: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataSetArn" => {
                                data_set_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataSetPlaceholder" => {
                                data_set_placeholder = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataSetReference {
                        data_set_arn: data_set_arn.ok_or(::serde::de::Error::missing_field("DataSetArn"))?,
                        data_set_placeholder: data_set_placeholder.ok_or(::serde::de::Error::missing_field("DataSetPlaceholder"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::Template.ResourcePermission`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-template-resourcepermission.html) property type.
    #[derive(Debug, Default)]
    pub struct ResourcePermission {
        /// Property [`Actions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-template-resourcepermission.html#cfn-quicksight-template-resourcepermission-actions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub actions: ::ValueList<String>,
        /// Property [`Principal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-template-resourcepermission.html#cfn-quicksight-template-resourcepermission-principal).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub principal: ::Value<String>,
    }

    impl ::codec::SerializeValue for ResourcePermission {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Actions", &self.actions)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Principal", &self.principal)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResourcePermission {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourcePermission, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResourcePermission;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResourcePermission")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut actions: Option<::ValueList<String>> = None;
                    let mut principal: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Actions" => {
                                actions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Principal" => {
                                principal = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResourcePermission {
                        actions: actions.ok_or(::serde::de::Error::missing_field("Actions"))?,
                        principal: principal.ok_or(::serde::de::Error::missing_field("Principal"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::Template.TemplateSourceAnalysis`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-template-templatesourceanalysis.html) property type.
    #[derive(Debug, Default)]
    pub struct TemplateSourceAnalysis {
        /// Property [`Arn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-template-templatesourceanalysis.html#cfn-quicksight-template-templatesourceanalysis-arn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub arn: ::Value<String>,
        /// Property [`DataSetReferences`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-template-templatesourceanalysis.html#cfn-quicksight-template-templatesourceanalysis-datasetreferences).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_set_references: ::ValueList<DataSetReference>,
    }

    impl ::codec::SerializeValue for TemplateSourceAnalysis {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arn", &self.arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataSetReferences", &self.data_set_references)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TemplateSourceAnalysis {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TemplateSourceAnalysis, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TemplateSourceAnalysis;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TemplateSourceAnalysis")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arn: Option<::Value<String>> = None;
                    let mut data_set_references: Option<::ValueList<DataSetReference>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Arn" => {
                                arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataSetReferences" => {
                                data_set_references = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TemplateSourceAnalysis {
                        arn: arn.ok_or(::serde::de::Error::missing_field("Arn"))?,
                        data_set_references: data_set_references.ok_or(::serde::de::Error::missing_field("DataSetReferences"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::Template.TemplateSourceEntity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-template-templatesourceentity.html) property type.
    #[derive(Debug, Default)]
    pub struct TemplateSourceEntity {
        /// Property [`SourceAnalysis`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-template-templatesourceentity.html#cfn-quicksight-template-templatesourceentity-sourceanalysis).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_analysis: Option<::Value<TemplateSourceAnalysis>>,
        /// Property [`SourceTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-template-templatesourceentity.html#cfn-quicksight-template-templatesourceentity-sourcetemplate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_template: Option<::Value<TemplateSourceTemplate>>,
    }

    impl ::codec::SerializeValue for TemplateSourceEntity {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref source_analysis) = self.source_analysis {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceAnalysis", source_analysis)?;
            }
            if let Some(ref source_template) = self.source_template {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceTemplate", source_template)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TemplateSourceEntity {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TemplateSourceEntity, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TemplateSourceEntity;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TemplateSourceEntity")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut source_analysis: Option<::Value<TemplateSourceAnalysis>> = None;
                    let mut source_template: Option<::Value<TemplateSourceTemplate>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SourceAnalysis" => {
                                source_analysis = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceTemplate" => {
                                source_template = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TemplateSourceEntity {
                        source_analysis: source_analysis,
                        source_template: source_template,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::Template.TemplateSourceTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-template-templatesourcetemplate.html) property type.
    #[derive(Debug, Default)]
    pub struct TemplateSourceTemplate {
        /// Property [`Arn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-template-templatesourcetemplate.html#cfn-quicksight-template-templatesourcetemplate-arn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for TemplateSourceTemplate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Arn", &self.arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TemplateSourceTemplate {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TemplateSourceTemplate, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TemplateSourceTemplate;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TemplateSourceTemplate")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Arn" => {
                                arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TemplateSourceTemplate {
                        arn: arn.ok_or(::serde::de::Error::missing_field("Arn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod theme {
    //! Property types for the `Theme` resource.

    /// The [`AWS::QuickSight::Theme.BorderStyle`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-borderstyle.html) property type.
    #[derive(Debug, Default)]
    pub struct BorderStyle {
        /// Property [`Show`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-borderstyle.html#cfn-quicksight-theme-borderstyle-show).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub show: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for BorderStyle {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref show) = self.show {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Show", show)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BorderStyle {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BorderStyle, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BorderStyle;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BorderStyle")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut show: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Show" => {
                                show = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BorderStyle {
                        show: show,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::Theme.DataColorPalette`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-datacolorpalette.html) property type.
    #[derive(Debug, Default)]
    pub struct DataColorPalette {
        /// Property [`Colors`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-datacolorpalette.html#cfn-quicksight-theme-datacolorpalette-colors).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub colors: Option<::ValueList<String>>,
        /// Property [`EmptyFillColor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-datacolorpalette.html#cfn-quicksight-theme-datacolorpalette-emptyfillcolor).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub empty_fill_color: Option<::Value<String>>,
        /// Property [`MinMaxGradient`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-datacolorpalette.html#cfn-quicksight-theme-datacolorpalette-minmaxgradient).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min_max_gradient: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for DataColorPalette {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref colors) = self.colors {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Colors", colors)?;
            }
            if let Some(ref empty_fill_color) = self.empty_fill_color {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EmptyFillColor", empty_fill_color)?;
            }
            if let Some(ref min_max_gradient) = self.min_max_gradient {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinMaxGradient", min_max_gradient)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataColorPalette {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataColorPalette, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataColorPalette;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataColorPalette")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut colors: Option<::ValueList<String>> = None;
                    let mut empty_fill_color: Option<::Value<String>> = None;
                    let mut min_max_gradient: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Colors" => {
                                colors = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EmptyFillColor" => {
                                empty_fill_color = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinMaxGradient" => {
                                min_max_gradient = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataColorPalette {
                        colors: colors,
                        empty_fill_color: empty_fill_color,
                        min_max_gradient: min_max_gradient,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::Theme.Font`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-font.html) property type.
    #[derive(Debug, Default)]
    pub struct Font {
        /// Property [`FontFamily`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-font.html#cfn-quicksight-theme-font-fontfamily).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub font_family: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Font {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref font_family) = self.font_family {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FontFamily", font_family)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Font {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Font, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Font;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Font")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut font_family: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FontFamily" => {
                                font_family = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Font {
                        font_family: font_family,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::Theme.GutterStyle`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-gutterstyle.html) property type.
    #[derive(Debug, Default)]
    pub struct GutterStyle {
        /// Property [`Show`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-gutterstyle.html#cfn-quicksight-theme-gutterstyle-show).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub show: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for GutterStyle {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref show) = self.show {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Show", show)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GutterStyle {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GutterStyle, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GutterStyle;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GutterStyle")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut show: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Show" => {
                                show = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GutterStyle {
                        show: show,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::Theme.MarginStyle`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-marginstyle.html) property type.
    #[derive(Debug, Default)]
    pub struct MarginStyle {
        /// Property [`Show`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-marginstyle.html#cfn-quicksight-theme-marginstyle-show).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub show: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for MarginStyle {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref show) = self.show {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Show", show)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MarginStyle {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MarginStyle, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MarginStyle;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MarginStyle")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut show: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Show" => {
                                show = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MarginStyle {
                        show: show,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::Theme.ResourcePermission`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-resourcepermission.html) property type.
    #[derive(Debug, Default)]
    pub struct ResourcePermission {
        /// Property [`Actions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-resourcepermission.html#cfn-quicksight-theme-resourcepermission-actions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub actions: ::ValueList<String>,
        /// Property [`Principal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-resourcepermission.html#cfn-quicksight-theme-resourcepermission-principal).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub principal: ::Value<String>,
    }

    impl ::codec::SerializeValue for ResourcePermission {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Actions", &self.actions)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Principal", &self.principal)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResourcePermission {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourcePermission, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResourcePermission;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResourcePermission")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut actions: Option<::ValueList<String>> = None;
                    let mut principal: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Actions" => {
                                actions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Principal" => {
                                principal = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResourcePermission {
                        actions: actions.ok_or(::serde::de::Error::missing_field("Actions"))?,
                        principal: principal.ok_or(::serde::de::Error::missing_field("Principal"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::Theme.SheetStyle`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-sheetstyle.html) property type.
    #[derive(Debug, Default)]
    pub struct SheetStyle {
        /// Property [`Tile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-sheetstyle.html#cfn-quicksight-theme-sheetstyle-tile).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tile: Option<::Value<TileStyle>>,
        /// Property [`TileLayout`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-sheetstyle.html#cfn-quicksight-theme-sheetstyle-tilelayout).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tile_layout: Option<::Value<TileLayoutStyle>>,
    }

    impl ::codec::SerializeValue for SheetStyle {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref tile) = self.tile {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tile", tile)?;
            }
            if let Some(ref tile_layout) = self.tile_layout {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TileLayout", tile_layout)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SheetStyle {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SheetStyle, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SheetStyle;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SheetStyle")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut tile: Option<::Value<TileStyle>> = None;
                    let mut tile_layout: Option<::Value<TileLayoutStyle>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Tile" => {
                                tile = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TileLayout" => {
                                tile_layout = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SheetStyle {
                        tile: tile,
                        tile_layout: tile_layout,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::Theme.ThemeConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-themeconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct ThemeConfiguration {
        /// Property [`DataColorPalette`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-themeconfiguration.html#cfn-quicksight-theme-themeconfiguration-datacolorpalette).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_color_palette: Option<::Value<DataColorPalette>>,
        /// Property [`Sheet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-themeconfiguration.html#cfn-quicksight-theme-themeconfiguration-sheet).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sheet: Option<::Value<SheetStyle>>,
        /// Property [`Typography`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-themeconfiguration.html#cfn-quicksight-theme-themeconfiguration-typography).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub typography: Option<::Value<Typography>>,
        /// Property [`UIColorPalette`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-themeconfiguration.html#cfn-quicksight-theme-themeconfiguration-uicolorpalette).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ui_color_palette: Option<::Value<UIColorPalette>>,
    }

    impl ::codec::SerializeValue for ThemeConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref data_color_palette) = self.data_color_palette {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataColorPalette", data_color_palette)?;
            }
            if let Some(ref sheet) = self.sheet {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Sheet", sheet)?;
            }
            if let Some(ref typography) = self.typography {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Typography", typography)?;
            }
            if let Some(ref ui_color_palette) = self.ui_color_palette {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UIColorPalette", ui_color_palette)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ThemeConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ThemeConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ThemeConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ThemeConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_color_palette: Option<::Value<DataColorPalette>> = None;
                    let mut sheet: Option<::Value<SheetStyle>> = None;
                    let mut typography: Option<::Value<Typography>> = None;
                    let mut ui_color_palette: Option<::Value<UIColorPalette>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataColorPalette" => {
                                data_color_palette = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Sheet" => {
                                sheet = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Typography" => {
                                typography = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UIColorPalette" => {
                                ui_color_palette = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ThemeConfiguration {
                        data_color_palette: data_color_palette,
                        sheet: sheet,
                        typography: typography,
                        ui_color_palette: ui_color_palette,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::Theme.TileLayoutStyle`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-tilelayoutstyle.html) property type.
    #[derive(Debug, Default)]
    pub struct TileLayoutStyle {
        /// Property [`Gutter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-tilelayoutstyle.html#cfn-quicksight-theme-tilelayoutstyle-gutter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub gutter: Option<::Value<GutterStyle>>,
        /// Property [`Margin`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-tilelayoutstyle.html#cfn-quicksight-theme-tilelayoutstyle-margin).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub margin: Option<::Value<MarginStyle>>,
    }

    impl ::codec::SerializeValue for TileLayoutStyle {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref gutter) = self.gutter {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Gutter", gutter)?;
            }
            if let Some(ref margin) = self.margin {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Margin", margin)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TileLayoutStyle {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TileLayoutStyle, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TileLayoutStyle;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TileLayoutStyle")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut gutter: Option<::Value<GutterStyle>> = None;
                    let mut margin: Option<::Value<MarginStyle>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Gutter" => {
                                gutter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Margin" => {
                                margin = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TileLayoutStyle {
                        gutter: gutter,
                        margin: margin,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::Theme.TileStyle`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-tilestyle.html) property type.
    #[derive(Debug, Default)]
    pub struct TileStyle {
        /// Property [`Border`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-tilestyle.html#cfn-quicksight-theme-tilestyle-border).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub border: Option<::Value<BorderStyle>>,
    }

    impl ::codec::SerializeValue for TileStyle {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref border) = self.border {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Border", border)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TileStyle {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TileStyle, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TileStyle;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TileStyle")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut border: Option<::Value<BorderStyle>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Border" => {
                                border = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TileStyle {
                        border: border,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::Theme.Typography`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-typography.html) property type.
    #[derive(Debug, Default)]
    pub struct Typography {
        /// Property [`FontFamilies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-typography.html#cfn-quicksight-theme-typography-fontfamilies).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub font_families: Option<::ValueList<Font>>,
    }

    impl ::codec::SerializeValue for Typography {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref font_families) = self.font_families {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FontFamilies", font_families)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Typography {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Typography, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Typography;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Typography")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut font_families: Option<::ValueList<Font>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FontFamilies" => {
                                font_families = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Typography {
                        font_families: font_families,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::QuickSight::Theme.UIColorPalette`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-uicolorpalette.html) property type.
    #[derive(Debug, Default)]
    pub struct UIColorPalette {
        /// Property [`Accent`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-uicolorpalette.html#cfn-quicksight-theme-uicolorpalette-accent).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub accent: Option<::Value<String>>,
        /// Property [`AccentForeground`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-uicolorpalette.html#cfn-quicksight-theme-uicolorpalette-accentforeground).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub accent_foreground: Option<::Value<String>>,
        /// Property [`Danger`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-uicolorpalette.html#cfn-quicksight-theme-uicolorpalette-danger).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub danger: Option<::Value<String>>,
        /// Property [`DangerForeground`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-uicolorpalette.html#cfn-quicksight-theme-uicolorpalette-dangerforeground).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub danger_foreground: Option<::Value<String>>,
        /// Property [`Dimension`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-uicolorpalette.html#cfn-quicksight-theme-uicolorpalette-dimension).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dimension: Option<::Value<String>>,
        /// Property [`DimensionForeground`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-uicolorpalette.html#cfn-quicksight-theme-uicolorpalette-dimensionforeground).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dimension_foreground: Option<::Value<String>>,
        /// Property [`Measure`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-uicolorpalette.html#cfn-quicksight-theme-uicolorpalette-measure).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub measure: Option<::Value<String>>,
        /// Property [`MeasureForeground`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-uicolorpalette.html#cfn-quicksight-theme-uicolorpalette-measureforeground).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub measure_foreground: Option<::Value<String>>,
        /// Property [`PrimaryBackground`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-uicolorpalette.html#cfn-quicksight-theme-uicolorpalette-primarybackground).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub primary_background: Option<::Value<String>>,
        /// Property [`PrimaryForeground`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-uicolorpalette.html#cfn-quicksight-theme-uicolorpalette-primaryforeground).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub primary_foreground: Option<::Value<String>>,
        /// Property [`SecondaryBackground`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-uicolorpalette.html#cfn-quicksight-theme-uicolorpalette-secondarybackground).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secondary_background: Option<::Value<String>>,
        /// Property [`SecondaryForeground`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-uicolorpalette.html#cfn-quicksight-theme-uicolorpalette-secondaryforeground).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secondary_foreground: Option<::Value<String>>,
        /// Property [`Success`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-uicolorpalette.html#cfn-quicksight-theme-uicolorpalette-success).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub success: Option<::Value<String>>,
        /// Property [`SuccessForeground`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-uicolorpalette.html#cfn-quicksight-theme-uicolorpalette-successforeground).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub success_foreground: Option<::Value<String>>,
        /// Property [`Warning`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-uicolorpalette.html#cfn-quicksight-theme-uicolorpalette-warning).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub warning: Option<::Value<String>>,
        /// Property [`WarningForeground`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-quicksight-theme-uicolorpalette.html#cfn-quicksight-theme-uicolorpalette-warningforeground).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub warning_foreground: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for UIColorPalette {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref accent) = self.accent {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Accent", accent)?;
            }
            if let Some(ref accent_foreground) = self.accent_foreground {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccentForeground", accent_foreground)?;
            }
            if let Some(ref danger) = self.danger {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Danger", danger)?;
            }
            if let Some(ref danger_foreground) = self.danger_foreground {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DangerForeground", danger_foreground)?;
            }
            if let Some(ref dimension) = self.dimension {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Dimension", dimension)?;
            }
            if let Some(ref dimension_foreground) = self.dimension_foreground {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DimensionForeground", dimension_foreground)?;
            }
            if let Some(ref measure) = self.measure {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Measure", measure)?;
            }
            if let Some(ref measure_foreground) = self.measure_foreground {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MeasureForeground", measure_foreground)?;
            }
            if let Some(ref primary_background) = self.primary_background {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrimaryBackground", primary_background)?;
            }
            if let Some(ref primary_foreground) = self.primary_foreground {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrimaryForeground", primary_foreground)?;
            }
            if let Some(ref secondary_background) = self.secondary_background {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecondaryBackground", secondary_background)?;
            }
            if let Some(ref secondary_foreground) = self.secondary_foreground {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecondaryForeground", secondary_foreground)?;
            }
            if let Some(ref success) = self.success {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Success", success)?;
            }
            if let Some(ref success_foreground) = self.success_foreground {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SuccessForeground", success_foreground)?;
            }
            if let Some(ref warning) = self.warning {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Warning", warning)?;
            }
            if let Some(ref warning_foreground) = self.warning_foreground {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WarningForeground", warning_foreground)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for UIColorPalette {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<UIColorPalette, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = UIColorPalette;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type UIColorPalette")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut accent: Option<::Value<String>> = None;
                    let mut accent_foreground: Option<::Value<String>> = None;
                    let mut danger: Option<::Value<String>> = None;
                    let mut danger_foreground: Option<::Value<String>> = None;
                    let mut dimension: Option<::Value<String>> = None;
                    let mut dimension_foreground: Option<::Value<String>> = None;
                    let mut measure: Option<::Value<String>> = None;
                    let mut measure_foreground: Option<::Value<String>> = None;
                    let mut primary_background: Option<::Value<String>> = None;
                    let mut primary_foreground: Option<::Value<String>> = None;
                    let mut secondary_background: Option<::Value<String>> = None;
                    let mut secondary_foreground: Option<::Value<String>> = None;
                    let mut success: Option<::Value<String>> = None;
                    let mut success_foreground: Option<::Value<String>> = None;
                    let mut warning: Option<::Value<String>> = None;
                    let mut warning_foreground: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Accent" => {
                                accent = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AccentForeground" => {
                                accent_foreground = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Danger" => {
                                danger = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DangerForeground" => {
                                danger_foreground = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Dimension" => {
                                dimension = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DimensionForeground" => {
                                dimension_foreground = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Measure" => {
                                measure = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MeasureForeground" => {
                                measure_foreground = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PrimaryBackground" => {
                                primary_background = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PrimaryForeground" => {
                                primary_foreground = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecondaryBackground" => {
                                secondary_background = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecondaryForeground" => {
                                secondary_foreground = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Success" => {
                                success = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SuccessForeground" => {
                                success_foreground = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Warning" => {
                                warning = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WarningForeground" => {
                                warning_foreground = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(UIColorPalette {
                        accent: accent,
                        accent_foreground: accent_foreground,
                        danger: danger,
                        danger_foreground: danger_foreground,
                        dimension: dimension,
                        dimension_foreground: dimension_foreground,
                        measure: measure,
                        measure_foreground: measure_foreground,
                        primary_background: primary_background,
                        primary_foreground: primary_foreground,
                        secondary_background: secondary_background,
                        secondary_foreground: secondary_foreground,
                        success: success,
                        success_foreground: success_foreground,
                        warning: warning,
                        warning_foreground: warning_foreground,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
