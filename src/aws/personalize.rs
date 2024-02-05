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

    /// The [`AWS::Personalize::Dataset.DataSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-dataset-datasource.html) property type.
    #[derive(Debug, Default)]
    pub struct DataSource {
        /// Property [`DataLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-dataset-datasource.html#cfn-personalize-dataset-datasource-datalocation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_location: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DataSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref data_location) = self.data_location {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataLocation", data_location)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataSource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataSource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataSource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataSource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_location: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataLocation" => {
                                data_location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataSource {
                        data_location: data_location,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Personalize::Dataset.DatasetImportJob`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-dataset-datasetimportjob.html) property type.
    #[derive(Debug, Default)]
    pub struct DatasetImportJob {
        /// Property [`DataSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-dataset-datasetimportjob.html#cfn-personalize-dataset-datasetimportjob-datasource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_source: Option<::Value<DataSource>>,
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
                    let mut data_source: Option<::Value<DataSource>> = None;
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

    /// The [`AWS::Personalize::Solution.AlgorithmHyperParameterRanges`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-algorithmhyperparameterranges.html) property type.
    #[derive(Debug, Default)]
    pub struct AlgorithmHyperParameterRanges {
        /// Property [`CategoricalHyperParameterRanges`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-algorithmhyperparameterranges.html#cfn-personalize-solution-algorithmhyperparameterranges-categoricalhyperparameterranges).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub categorical_hyper_parameter_ranges: Option<::ValueList<CategoricalHyperParameterRange>>,
        /// Property [`ContinuousHyperParameterRanges`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-algorithmhyperparameterranges.html#cfn-personalize-solution-algorithmhyperparameterranges-continuoushyperparameterranges).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub continuous_hyper_parameter_ranges: Option<::ValueList<ContinuousHyperParameterRange>>,
        /// Property [`IntegerHyperParameterRanges`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-algorithmhyperparameterranges.html#cfn-personalize-solution-algorithmhyperparameterranges-integerhyperparameterranges).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub integer_hyper_parameter_ranges: Option<::ValueList<IntegerHyperParameterRange>>,
    }

    impl ::codec::SerializeValue for AlgorithmHyperParameterRanges {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref categorical_hyper_parameter_ranges) = self.categorical_hyper_parameter_ranges {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CategoricalHyperParameterRanges", categorical_hyper_parameter_ranges)?;
            }
            if let Some(ref continuous_hyper_parameter_ranges) = self.continuous_hyper_parameter_ranges {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContinuousHyperParameterRanges", continuous_hyper_parameter_ranges)?;
            }
            if let Some(ref integer_hyper_parameter_ranges) = self.integer_hyper_parameter_ranges {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntegerHyperParameterRanges", integer_hyper_parameter_ranges)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AlgorithmHyperParameterRanges {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AlgorithmHyperParameterRanges, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AlgorithmHyperParameterRanges;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AlgorithmHyperParameterRanges")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut categorical_hyper_parameter_ranges: Option<::ValueList<CategoricalHyperParameterRange>> = None;
                    let mut continuous_hyper_parameter_ranges: Option<::ValueList<ContinuousHyperParameterRange>> = None;
                    let mut integer_hyper_parameter_ranges: Option<::ValueList<IntegerHyperParameterRange>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CategoricalHyperParameterRanges" => {
                                categorical_hyper_parameter_ranges = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ContinuousHyperParameterRanges" => {
                                continuous_hyper_parameter_ranges = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IntegerHyperParameterRanges" => {
                                integer_hyper_parameter_ranges = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AlgorithmHyperParameterRanges {
                        categorical_hyper_parameter_ranges: categorical_hyper_parameter_ranges,
                        continuous_hyper_parameter_ranges: continuous_hyper_parameter_ranges,
                        integer_hyper_parameter_ranges: integer_hyper_parameter_ranges,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Personalize::Solution.AutoMLConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-automlconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct AutoMLConfig {
        /// Property [`MetricName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-automlconfig.html#cfn-personalize-solution-automlconfig-metricname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub metric_name: Option<::Value<String>>,
        /// Property [`RecipeList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-automlconfig.html#cfn-personalize-solution-automlconfig-recipelist).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub recipe_list: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for AutoMLConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref metric_name) = self.metric_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricName", metric_name)?;
            }
            if let Some(ref recipe_list) = self.recipe_list {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecipeList", recipe_list)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AutoMLConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AutoMLConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AutoMLConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AutoMLConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut metric_name: Option<::Value<String>> = None;
                    let mut recipe_list: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MetricName" => {
                                metric_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RecipeList" => {
                                recipe_list = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AutoMLConfig {
                        metric_name: metric_name,
                        recipe_list: recipe_list,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Personalize::Solution.CategoricalHyperParameterRange`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-categoricalhyperparameterrange.html) property type.
    #[derive(Debug, Default)]
    pub struct CategoricalHyperParameterRange {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-categoricalhyperparameterrange.html#cfn-personalize-solution-categoricalhyperparameterrange-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Values`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-categoricalhyperparameterrange.html#cfn-personalize-solution-categoricalhyperparameterrange-values).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub values: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for CategoricalHyperParameterRange {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref values) = self.values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", values)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CategoricalHyperParameterRange {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CategoricalHyperParameterRange, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CategoricalHyperParameterRange;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CategoricalHyperParameterRange")
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

                    Ok(CategoricalHyperParameterRange {
                        name: name,
                        values: values,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Personalize::Solution.ContinuousHyperParameterRange`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-continuoushyperparameterrange.html) property type.
    #[derive(Debug, Default)]
    pub struct ContinuousHyperParameterRange {
        /// Property [`MaxValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-continuoushyperparameterrange.html#cfn-personalize-solution-continuoushyperparameterrange-maxvalue).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub max_value: Option<::Value<f64>>,
        /// Property [`MinValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-continuoushyperparameterrange.html#cfn-personalize-solution-continuoushyperparameterrange-minvalue).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub min_value: Option<::Value<f64>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-continuoushyperparameterrange.html#cfn-personalize-solution-continuoushyperparameterrange-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ContinuousHyperParameterRange {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref max_value) = self.max_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxValue", max_value)?;
            }
            if let Some(ref min_value) = self.min_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinValue", min_value)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ContinuousHyperParameterRange {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ContinuousHyperParameterRange, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ContinuousHyperParameterRange;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ContinuousHyperParameterRange")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut max_value: Option<::Value<f64>> = None;
                    let mut min_value: Option<::Value<f64>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxValue" => {
                                max_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinValue" => {
                                min_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ContinuousHyperParameterRange {
                        max_value: max_value,
                        min_value: min_value,
                        name: name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Personalize::Solution.HpoConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-hpoconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct HpoConfig {
        /// Property [`AlgorithmHyperParameterRanges`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-hpoconfig.html#cfn-personalize-solution-hpoconfig-algorithmhyperparameterranges).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub algorithm_hyper_parameter_ranges: Option<::Value<AlgorithmHyperParameterRanges>>,
        /// Property [`HpoObjective`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-hpoconfig.html#cfn-personalize-solution-hpoconfig-hpoobjective).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub hpo_objective: Option<::Value<HpoObjective>>,
        /// Property [`HpoResourceConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-hpoconfig.html#cfn-personalize-solution-hpoconfig-hporesourceconfig).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub hpo_resource_config: Option<::Value<HpoResourceConfig>>,
    }

    impl ::codec::SerializeValue for HpoConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref algorithm_hyper_parameter_ranges) = self.algorithm_hyper_parameter_ranges {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AlgorithmHyperParameterRanges", algorithm_hyper_parameter_ranges)?;
            }
            if let Some(ref hpo_objective) = self.hpo_objective {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HpoObjective", hpo_objective)?;
            }
            if let Some(ref hpo_resource_config) = self.hpo_resource_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HpoResourceConfig", hpo_resource_config)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HpoConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HpoConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HpoConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HpoConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut algorithm_hyper_parameter_ranges: Option<::Value<AlgorithmHyperParameterRanges>> = None;
                    let mut hpo_objective: Option<::Value<HpoObjective>> = None;
                    let mut hpo_resource_config: Option<::Value<HpoResourceConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AlgorithmHyperParameterRanges" => {
                                algorithm_hyper_parameter_ranges = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HpoObjective" => {
                                hpo_objective = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HpoResourceConfig" => {
                                hpo_resource_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HpoConfig {
                        algorithm_hyper_parameter_ranges: algorithm_hyper_parameter_ranges,
                        hpo_objective: hpo_objective,
                        hpo_resource_config: hpo_resource_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Personalize::Solution.HpoObjective`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-hpoobjective.html) property type.
    #[derive(Debug, Default)]
    pub struct HpoObjective {
        /// Property [`MetricName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-hpoobjective.html#cfn-personalize-solution-hpoobjective-metricname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub metric_name: Option<::Value<String>>,
        /// Property [`MetricRegex`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-hpoobjective.html#cfn-personalize-solution-hpoobjective-metricregex).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub metric_regex: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-hpoobjective.html#cfn-personalize-solution-hpoobjective-type).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub r#type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for HpoObjective {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref metric_name) = self.metric_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricName", metric_name)?;
            }
            if let Some(ref metric_regex) = self.metric_regex {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricRegex", metric_regex)?;
            }
            if let Some(ref r#type) = self.r#type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HpoObjective {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HpoObjective, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HpoObjective;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HpoObjective")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut metric_name: Option<::Value<String>> = None;
                    let mut metric_regex: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MetricName" => {
                                metric_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetricRegex" => {
                                metric_regex = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HpoObjective {
                        metric_name: metric_name,
                        metric_regex: metric_regex,
                        r#type: r#type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Personalize::Solution.HpoResourceConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-hporesourceconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct HpoResourceConfig {
        /// Property [`MaxNumberOfTrainingJobs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-hporesourceconfig.html#cfn-personalize-solution-hporesourceconfig-maxnumberoftrainingjobs).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub max_number_of_training_jobs: Option<::Value<String>>,
        /// Property [`MaxParallelTrainingJobs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-hporesourceconfig.html#cfn-personalize-solution-hporesourceconfig-maxparalleltrainingjobs).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub max_parallel_training_jobs: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for HpoResourceConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref max_number_of_training_jobs) = self.max_number_of_training_jobs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxNumberOfTrainingJobs", max_number_of_training_jobs)?;
            }
            if let Some(ref max_parallel_training_jobs) = self.max_parallel_training_jobs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxParallelTrainingJobs", max_parallel_training_jobs)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HpoResourceConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HpoResourceConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HpoResourceConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HpoResourceConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut max_number_of_training_jobs: Option<::Value<String>> = None;
                    let mut max_parallel_training_jobs: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxNumberOfTrainingJobs" => {
                                max_number_of_training_jobs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxParallelTrainingJobs" => {
                                max_parallel_training_jobs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HpoResourceConfig {
                        max_number_of_training_jobs: max_number_of_training_jobs,
                        max_parallel_training_jobs: max_parallel_training_jobs,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Personalize::Solution.IntegerHyperParameterRange`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-integerhyperparameterrange.html) property type.
    #[derive(Debug, Default)]
    pub struct IntegerHyperParameterRange {
        /// Property [`MaxValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-integerhyperparameterrange.html#cfn-personalize-solution-integerhyperparameterrange-maxvalue).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub max_value: Option<::Value<u32>>,
        /// Property [`MinValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-integerhyperparameterrange.html#cfn-personalize-solution-integerhyperparameterrange-minvalue).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub min_value: Option<::Value<u32>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-personalize-solution-integerhyperparameterrange.html#cfn-personalize-solution-integerhyperparameterrange-name).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for IntegerHyperParameterRange {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref max_value) = self.max_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxValue", max_value)?;
            }
            if let Some(ref min_value) = self.min_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinValue", min_value)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IntegerHyperParameterRange {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IntegerHyperParameterRange, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IntegerHyperParameterRange;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IntegerHyperParameterRange")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut max_value: Option<::Value<u32>> = None;
                    let mut min_value: Option<::Value<u32>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxValue" => {
                                max_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinValue" => {
                                min_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IntegerHyperParameterRange {
                        max_value: max_value,
                        min_value: min_value,
                        name: name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

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
        pub auto_ml_config: Option<::Value<AutoMLConfig>>,
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
        pub hpo_config: Option<::Value<HpoConfig>>,
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
                    let mut auto_ml_config: Option<::Value<AutoMLConfig>> = None;
                    let mut event_value_threshold: Option<::Value<String>> = None;
                    let mut feature_transformation_parameters: Option<::ValueMap<String>> = None;
                    let mut hpo_config: Option<::Value<HpoConfig>> = None;

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
