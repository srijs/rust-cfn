//! Types for the `Personalize` service.

/// The [`AWS::Personalize::Dataset`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-personalize-dataset.html) resource type.
#[derive(Debug, Default)]
pub struct Dataset {
    properties: DatasetProperties
}

/// Properties for the `Dataset` resource.
#[derive(Debug, Default)]
pub struct DatasetProperties {
    /// Property [`DatasetGroupArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-personalize-dataset.html#cfn-personalize-dataset-datasetgrouparn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub dataset_group_arn: ::Value<String>,
    /// Property [`DatasetImportJob`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-personalize-dataset.html#cfn-personalize-dataset-datasetimportjob).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub dataset_import_job: Option<::Value<self::dataset::DatasetImportJob>>,
    /// Property [`DatasetType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-personalize-dataset.html#cfn-personalize-dataset-datasettype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub dataset_type: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-personalize-dataset.html#cfn-personalize-dataset-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`SchemaArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-personalize-dataset.html#cfn-personalize-dataset-schemaarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub schema_arn: ::Value<String>,
}

impl ::serde::Serialize for DatasetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatasetGroupArn", &self.dataset_group_arn)?;
        if let Some(ref dataset_import_job) = self.dataset_import_job {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatasetImportJob", dataset_import_job)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatasetType", &self.dataset_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SchemaArn", &self.schema_arn)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DatasetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DatasetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DatasetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DatasetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut dataset_group_arn: Option<::Value<String>> = None;
                let mut dataset_import_job: Option<::Value<self::dataset::DatasetImportJob>> = None;
                let mut dataset_type: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut schema_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DatasetGroupArn" => {
                            dataset_group_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DatasetImportJob" => {
                            dataset_import_job = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DatasetType" => {
                            dataset_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SchemaArn" => {
                            schema_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DatasetProperties {
                    dataset_group_arn: dataset_group_arn.ok_or(::serde::de::Error::missing_field("DatasetGroupArn"))?,
                    dataset_import_job: dataset_import_job,
                    dataset_type: dataset_type.ok_or(::serde::de::Error::missing_field("DatasetType"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    schema_arn: schema_arn.ok_or(::serde::de::Error::missing_field("SchemaArn"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Dataset {
    type Properties = DatasetProperties;
    const TYPE: &'static str = "AWS::Personalize::Dataset";
    fn properties(&self) -> &DatasetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DatasetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Dataset {}

impl From<DatasetProperties> for Dataset {
    fn from(properties: DatasetProperties) -> Dataset {
        Dataset { properties }
    }
}

/// The [`AWS::Personalize::DatasetGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-personalize-datasetgroup.html) resource type.
#[derive(Debug, Default)]
pub struct DatasetGroup {
    properties: DatasetGroupProperties
}

/// Properties for the `DatasetGroup` resource.
#[derive(Debug, Default)]
pub struct DatasetGroupProperties {
    /// Property [`Domain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-personalize-datasetgroup.html#cfn-personalize-datasetgroup-domain).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub domain: Option<::Value<String>>,
    /// Property [`KmsKeyArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-personalize-datasetgroup.html#cfn-personalize-datasetgroup-kmskeyarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kms_key_arn: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-personalize-datasetgroup.html#cfn-personalize-datasetgroup-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-personalize-datasetgroup.html#cfn-personalize-datasetgroup-rolearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub role_arn: Option<::Value<String>>,
}

impl ::serde::Serialize for DatasetGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref domain) = self.domain {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Domain", domain)?;
        }
        if let Some(ref kms_key_arn) = self.kms_key_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyArn", kms_key_arn)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref role_arn) = self.role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", role_arn)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DatasetGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DatasetGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DatasetGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DatasetGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut domain: Option<::Value<String>> = None;
                let mut kms_key_arn: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut role_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Domain" => {
                            domain = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsKeyArn" => {
                            kms_key_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DatasetGroupProperties {
                    domain: domain,
                    kms_key_arn: kms_key_arn,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    role_arn: role_arn,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DatasetGroup {
    type Properties = DatasetGroupProperties;
    const TYPE: &'static str = "AWS::Personalize::DatasetGroup";
    fn properties(&self) -> &DatasetGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DatasetGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DatasetGroup {}

impl From<DatasetGroupProperties> for DatasetGroup {
    fn from(properties: DatasetGroupProperties) -> DatasetGroup {
        DatasetGroup { properties }
    }
}

/// The [`AWS::Personalize::Schema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-personalize-schema.html) resource type.
#[derive(Debug, Default)]
pub struct Schema {
    properties: SchemaProperties
}

/// Properties for the `Schema` resource.
#[derive(Debug, Default)]
pub struct SchemaProperties {
    /// Property [`Domain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-personalize-schema.html#cfn-personalize-schema-domain).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub domain: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-personalize-schema.html#cfn-personalize-schema-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Schema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-personalize-schema.html#cfn-personalize-schema-schema).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub schema: ::Value<String>,
}

impl ::serde::Serialize for SchemaProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref domain) = self.domain {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Domain", domain)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Schema", &self.schema)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SchemaProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SchemaProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SchemaProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SchemaProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut domain: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut schema: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Domain" => {
                            domain = ::serde::de::MapAccess::next_value(&mut map)?;
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

                Ok(SchemaProperties {
                    domain: domain,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    schema: schema.ok_or(::serde::de::Error::missing_field("Schema"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Schema {
    type Properties = SchemaProperties;
    const TYPE: &'static str = "AWS::Personalize::Schema";
    fn properties(&self) -> &SchemaProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SchemaProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Schema {}

impl From<SchemaProperties> for Schema {
    fn from(properties: SchemaProperties) -> Schema {
        Schema { properties }
    }
}

/// The [`AWS::Personalize::Solution`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-personalize-solution.html) resource type.
#[derive(Debug, Default)]
pub struct Solution {
    properties: SolutionProperties
}

/// Properties for the `Solution` resource.
#[derive(Debug, Default)]
pub struct SolutionProperties {
    /// Property [`DatasetGroupArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-personalize-solution.html#cfn-personalize-solution-datasetgrouparn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub dataset_group_arn: ::Value<String>,
    /// Property [`EventType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-personalize-solution.html#cfn-personalize-solution-eventtype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub event_type: Option<::Value<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-personalize-solution.html#cfn-personalize-solution-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`PerformAutoML`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-personalize-solution.html#cfn-personalize-solution-performautoml).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub perform_auto_ml: Option<::Value<bool>>,
    /// Property [`PerformHPO`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-personalize-solution.html#cfn-personalize-solution-performhpo).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub perform_hpo: Option<::Value<bool>>,
    /// Property [`RecipeArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-personalize-solution.html#cfn-personalize-solution-recipearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub recipe_arn: Option<::Value<String>>,
    /// Property [`SolutionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-personalize-solution.html#cfn-personalize-solution-solutionconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub solution_config: Option<::Value<self::solution::SolutionConfig>>,
}

impl ::serde::Serialize for SolutionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatasetGroupArn", &self.dataset_group_arn)?;
        if let Some(ref event_type) = self.event_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventType", event_type)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref perform_auto_ml) = self.perform_auto_ml {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PerformAutoML", perform_auto_ml)?;
        }
        if let Some(ref perform_hpo) = self.perform_hpo {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PerformHPO", perform_hpo)?;
        }
        if let Some(ref recipe_arn) = self.recipe_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecipeArn", recipe_arn)?;
        }
        if let Some(ref solution_config) = self.solution_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SolutionConfig", solution_config)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SolutionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SolutionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SolutionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SolutionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut dataset_group_arn: Option<::Value<String>> = None;
                let mut event_type: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut perform_auto_ml: Option<::Value<bool>> = None;
                let mut perform_hpo: Option<::Value<bool>> = None;
                let mut recipe_arn: Option<::Value<String>> = None;
                let mut solution_config: Option<::Value<self::solution::SolutionConfig>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DatasetGroupArn" => {
                            dataset_group_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EventType" => {
                            event_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PerformAutoML" => {
                            perform_auto_ml = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PerformHPO" => {
                            perform_hpo = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RecipeArn" => {
                            recipe_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SolutionConfig" => {
                            solution_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SolutionProperties {
                    dataset_group_arn: dataset_group_arn.ok_or(::serde::de::Error::missing_field("DatasetGroupArn"))?,
                    event_type: event_type,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    perform_auto_ml: perform_auto_ml,
                    perform_hpo: perform_hpo,
                    recipe_arn: recipe_arn,
                    solution_config: solution_config,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Solution {
    type Properties = SolutionProperties;
    const TYPE: &'static str = "AWS::Personalize::Solution";
    fn properties(&self) -> &SolutionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SolutionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Solution {}

impl From<SolutionProperties> for Solution {
    fn from(properties: SolutionProperties) -> Solution {
        Solution { properties }
    }
}

pub mod dataset {
    //! Property types for the `Dataset` resource.

    /// The [`AWS::Personalize::Dataset.DatasetImportJob`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-dataset-datasetimportjob.html) property type.
    #[derive(Debug, Default)]
    pub struct DatasetImportJob {
        /// Property [`DataSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-dataset-datasetimportjob.html#cfn-personalize-dataset-datasetimportjob-datasource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_source: Option<::Value<::json::Value>>,
        /// Property [`DatasetArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-dataset-datasetimportjob.html#cfn-personalize-dataset-datasetimportjob-datasetarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dataset_arn: Option<::Value<String>>,
        /// Property [`DatasetImportJobArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-dataset-datasetimportjob.html#cfn-personalize-dataset-datasetimportjob-datasetimportjobarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dataset_import_job_arn: Option<::Value<String>>,
        /// Property [`JobName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-dataset-datasetimportjob.html#cfn-personalize-dataset-datasetimportjob-jobname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub job_name: Option<::Value<String>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-dataset-datasetimportjob.html#cfn-personalize-dataset-datasetimportjob-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DatasetImportJob {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref data_source) = self.data_source {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataSource", data_source)?;
            }
            if let Some(ref dataset_arn) = self.dataset_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatasetArn", dataset_arn)?;
            }
            if let Some(ref dataset_import_job_arn) = self.dataset_import_job_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatasetImportJobArn", dataset_import_job_arn)?;
            }
            if let Some(ref job_name) = self.job_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "JobName", job_name)?;
            }
            if let Some(ref role_arn) = self.role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", role_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DatasetImportJob {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DatasetImportJob, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DatasetImportJob;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DatasetImportJob")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_source: Option<::Value<::json::Value>> = None;
                    let mut dataset_arn: Option<::Value<String>> = None;
                    let mut dataset_import_job_arn: Option<::Value<String>> = None;
                    let mut job_name: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataSource" => {
                                data_source = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatasetArn" => {
                                dataset_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DatasetImportJobArn" => {
                                dataset_import_job_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "JobName" => {
                                job_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DatasetImportJob {
                        data_source: data_source,
                        dataset_arn: dataset_arn,
                        dataset_import_job_arn: dataset_import_job_arn,
                        job_name: job_name,
                        role_arn: role_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod solution {
    //! Property types for the `Solution` resource.

    /// The [`AWS::Personalize::Solution.SolutionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-solutionconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct SolutionConfig {
        /// Property [`AlgorithmHyperParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-solutionconfig.html#cfn-personalize-solution-solutionconfig-algorithmhyperparameters).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub algorithm_hyper_parameters: Option<::ValueMap<String>>,
        /// Property [`AutoMLConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-solutionconfig.html#cfn-personalize-solution-solutionconfig-automlconfig).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub auto_ml_config: Option<::Value<::json::Value>>,
        /// Property [`EventValueThreshold`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-solutionconfig.html#cfn-personalize-solution-solutionconfig-eventvaluethreshold).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub event_value_threshold: Option<::Value<String>>,
        /// Property [`FeatureTransformationParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-solutionconfig.html#cfn-personalize-solution-solutionconfig-featuretransformationparameters).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub feature_transformation_parameters: Option<::ValueMap<String>>,
        /// Property [`HpoConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-solutionconfig.html#cfn-personalize-solution-solutionconfig-hpoconfig).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub hpo_config: Option<::Value<::json::Value>>,
    }

    impl ::codec::SerializeValue for SolutionConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref algorithm_hyper_parameters) = self.algorithm_hyper_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AlgorithmHyperParameters", algorithm_hyper_parameters)?;
            }
            if let Some(ref auto_ml_config) = self.auto_ml_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoMLConfig", auto_ml_config)?;
            }
            if let Some(ref event_value_threshold) = self.event_value_threshold {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventValueThreshold", event_value_threshold)?;
            }
            if let Some(ref feature_transformation_parameters) = self.feature_transformation_parameters {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FeatureTransformationParameters", feature_transformation_parameters)?;
            }
            if let Some(ref hpo_config) = self.hpo_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HpoConfig", hpo_config)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SolutionConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SolutionConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SolutionConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SolutionConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut algorithm_hyper_parameters: Option<::ValueMap<String>> = None;
                    let mut auto_ml_config: Option<::Value<::json::Value>> = None;
                    let mut event_value_threshold: Option<::Value<String>> = None;
                    let mut feature_transformation_parameters: Option<::ValueMap<String>> = None;
                    let mut hpo_config: Option<::Value<::json::Value>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AlgorithmHyperParameters" => {
                                algorithm_hyper_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AutoMLConfig" => {
                                auto_ml_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EventValueThreshold" => {
                                event_value_threshold = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FeatureTransformationParameters" => {
                                feature_transformation_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HpoConfig" => {
                                hpo_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SolutionConfig {
                        algorithm_hyper_parameters: algorithm_hyper_parameters,
                        auto_ml_config: auto_ml_config,
                        event_value_threshold: event_value_threshold,
                        feature_transformation_parameters: feature_transformation_parameters,
                        hpo_config: hpo_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
