//! Types for the `Forecast` service.

/// The [`AWS::Forecast::Dataset`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-forecast-dataset.html) resource type.
#[derive(Debug, Default)]
pub struct Dataset {
    properties: DatasetProperties
}

/// Properties for the `Dataset` resource.
#[derive(Debug, Default)]
pub struct DatasetProperties {
    /// Property [`DataFrequency`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-forecast-dataset.html#cfn-forecast-dataset-datafrequency).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub data_frequency: Option<::Value<String>>,
    /// Property [`DatasetName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-forecast-dataset.html#cfn-forecast-dataset-datasetname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub dataset_name: ::Value<String>,
    /// Property [`DatasetType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-forecast-dataset.html#cfn-forecast-dataset-datasettype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub dataset_type: ::Value<String>,
    /// Property [`Domain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-forecast-dataset.html#cfn-forecast-dataset-domain).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub domain: ::Value<String>,
    /// Property [`EncryptionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-forecast-dataset.html#cfn-forecast-dataset-encryptionconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub encryption_config: Option<::Value<::json::Value>>,
    /// Property [`Schema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-forecast-dataset.html#cfn-forecast-dataset-schema).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub schema: ::Value<::json::Value>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-forecast-dataset.html#cfn-forecast-dataset-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::json::Value>>,
}

impl ::serde::Serialize for DatasetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref data_frequency) = self.data_frequency {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataFrequency", data_frequency)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatasetName", &self.dataset_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatasetType", &self.dataset_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Domain", &self.domain)?;
        if let Some(ref encryption_config) = self.encryption_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionConfig", encryption_config)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Schema", &self.schema)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
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
                let mut data_frequency: Option<::Value<String>> = None;
                let mut dataset_name: Option<::Value<String>> = None;
                let mut dataset_type: Option<::Value<String>> = None;
                let mut domain: Option<::Value<String>> = None;
                let mut encryption_config: Option<::Value<::json::Value>> = None;
                let mut schema: Option<::Value<::json::Value>> = None;
                let mut tags: Option<::ValueList<::json::Value>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DataFrequency" => {
                            data_frequency = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DatasetName" => {
                            dataset_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DatasetType" => {
                            dataset_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Domain" => {
                            domain = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EncryptionConfig" => {
                            encryption_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Schema" => {
                            schema = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DatasetProperties {
                    data_frequency: data_frequency,
                    dataset_name: dataset_name.ok_or(::serde::de::Error::missing_field("DatasetName"))?,
                    dataset_type: dataset_type.ok_or(::serde::de::Error::missing_field("DatasetType"))?,
                    domain: domain.ok_or(::serde::de::Error::missing_field("Domain"))?,
                    encryption_config: encryption_config,
                    schema: schema.ok_or(::serde::de::Error::missing_field("Schema"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Dataset {
    type Properties = DatasetProperties;
    const TYPE: &'static str = "AWS::Forecast::Dataset";
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

/// The [`AWS::Forecast::DatasetGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-forecast-datasetgroup.html) resource type.
#[derive(Debug, Default)]
pub struct DatasetGroup {
    properties: DatasetGroupProperties
}

/// Properties for the `DatasetGroup` resource.
#[derive(Debug, Default)]
pub struct DatasetGroupProperties {
    /// Property [`DatasetArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-forecast-datasetgroup.html#cfn-forecast-datasetgroup-datasetarns).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub dataset_arns: Option<::ValueList<String>>,
    /// Property [`DatasetGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-forecast-datasetgroup.html#cfn-forecast-datasetgroup-datasetgroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub dataset_group_name: ::Value<String>,
    /// Property [`Domain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-forecast-datasetgroup.html#cfn-forecast-datasetgroup-domain).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub domain: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-forecast-datasetgroup.html#cfn-forecast-datasetgroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for DatasetGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref dataset_arns) = self.dataset_arns {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatasetArns", dataset_arns)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatasetGroupName", &self.dataset_group_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Domain", &self.domain)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
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
                let mut dataset_arns: Option<::ValueList<String>> = None;
                let mut dataset_group_name: Option<::Value<String>> = None;
                let mut domain: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DatasetArns" => {
                            dataset_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DatasetGroupName" => {
                            dataset_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Domain" => {
                            domain = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DatasetGroupProperties {
                    dataset_arns: dataset_arns,
                    dataset_group_name: dataset_group_name.ok_or(::serde::de::Error::missing_field("DatasetGroupName"))?,
                    domain: domain.ok_or(::serde::de::Error::missing_field("Domain"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DatasetGroup {
    type Properties = DatasetGroupProperties;
    const TYPE: &'static str = "AWS::Forecast::DatasetGroup";
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
