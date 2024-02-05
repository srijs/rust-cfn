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
    pub encryption_config: Option<::Value<self::dataset::EncryptionConfig>>,
    /// Property [`Schema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-forecast-dataset.html#cfn-forecast-dataset-schema).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub schema: ::Value<self::dataset::Schema>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-forecast-dataset.html#cfn-forecast-dataset-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<self::dataset::TagsItems>>,
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
                let mut encryption_config: Option<::Value<self::dataset::EncryptionConfig>> = None;
                let mut schema: Option<::Value<self::dataset::Schema>> = None;
                let mut tags: Option<::ValueList<self::dataset::TagsItems>> = None;

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

pub mod dataset {
    //! Property types for the `Dataset` resource.

    /// The [`AWS::Forecast::Dataset.AttributesItems`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-forecast-dataset-attributesitems.html) property type.
    #[derive(Debug, Default)]
    pub struct AttributesItems {
        /// Property [`AttributeName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-forecast-dataset-attributesitems.html#cfn-forecast-dataset-attributesitems-attributename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attribute_name: Option<::Value<String>>,
        /// Property [`AttributeType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-forecast-dataset-attributesitems.html#cfn-forecast-dataset-attributesitems-attributetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attribute_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AttributesItems {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref attribute_name) = self.attribute_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttributeName", attribute_name)?;
            }
            if let Some(ref attribute_type) = self.attribute_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttributeType", attribute_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AttributesItems {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AttributesItems, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AttributesItems;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AttributesItems")
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

                    Ok(AttributesItems {
                        attribute_name: attribute_name,
                        attribute_type: attribute_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Forecast::Dataset.EncryptionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-forecast-dataset-encryptionconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct EncryptionConfig {
        /// Property [`KmsKeyArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-forecast-dataset-encryptionconfig.html#cfn-forecast-dataset-encryptionconfig-kmskeyarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kms_key_arn: Option<::Value<String>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-forecast-dataset-encryptionconfig.html#cfn-forecast-dataset-encryptionconfig-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EncryptionConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref kms_key_arn) = self.kms_key_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyArn", kms_key_arn)?;
            }
            if let Some(ref role_arn) = self.role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", role_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EncryptionConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EncryptionConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EncryptionConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EncryptionConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut kms_key_arn: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KmsKeyArn" => {
                                kms_key_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EncryptionConfig {
                        kms_key_arn: kms_key_arn,
                        role_arn: role_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Forecast::Dataset.Schema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-forecast-dataset-schema.html) property type.
    #[derive(Debug, Default)]
    pub struct Schema {
        /// Property [`Attributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-forecast-dataset-schema.html#cfn-forecast-dataset-schema-attributes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attributes: Option<::ValueList<AttributesItems>>,
    }

    impl ::codec::SerializeValue for Schema {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref attributes) = self.attributes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Attributes", attributes)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Schema {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Schema, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Schema;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Schema")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attributes: Option<::ValueList<AttributesItems>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Attributes" => {
                                attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Schema {
                        attributes: attributes,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Forecast::Dataset.TagsItems`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-forecast-dataset-tagsitems.html) property type.
    #[derive(Debug, Default)]
    pub struct TagsItems {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-forecast-dataset-tagsitems.html#cfn-forecast-dataset-tagsitems-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-forecast-dataset-tagsitems.html#cfn-forecast-dataset-tagsitems-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for TagsItems {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TagsItems {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TagsItems, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TagsItems;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TagsItems")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TagsItems {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
