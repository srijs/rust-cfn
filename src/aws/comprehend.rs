//! Types for the `Comprehend` service.

/// The [`AWS::Comprehend::DocumentClassifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-comprehend-documentclassifier.html) resource type.
#[derive(Debug, Default)]
pub struct DocumentClassifier {
    properties: DocumentClassifierProperties
}

/// Properties for the `DocumentClassifier` resource.
#[derive(Debug, Default)]
pub struct DocumentClassifierProperties {
    /// Property [`DataAccessRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-comprehend-documentclassifier.html#cfn-comprehend-documentclassifier-dataaccessrolearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub data_access_role_arn: ::Value<String>,
    /// Property [`DocumentClassifierName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-comprehend-documentclassifier.html#cfn-comprehend-documentclassifier-documentclassifiername).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub document_classifier_name: ::Value<String>,
    /// Property [`InputDataConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-comprehend-documentclassifier.html#cfn-comprehend-documentclassifier-inputdataconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub input_data_config: ::Value<self::document_classifier::DocumentClassifierInputDataConfig>,
    /// Property [`LanguageCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-comprehend-documentclassifier.html#cfn-comprehend-documentclassifier-languagecode).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub language_code: ::Value<String>,
    /// Property [`Mode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-comprehend-documentclassifier.html#cfn-comprehend-documentclassifier-mode).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub mode: Option<::Value<String>>,
    /// Property [`ModelKmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-comprehend-documentclassifier.html#cfn-comprehend-documentclassifier-modelkmskeyid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub model_kms_key_id: Option<::Value<String>>,
    /// Property [`ModelPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-comprehend-documentclassifier.html#cfn-comprehend-documentclassifier-modelpolicy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub model_policy: Option<::Value<String>>,
    /// Property [`OutputDataConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-comprehend-documentclassifier.html#cfn-comprehend-documentclassifier-outputdataconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub output_data_config: Option<::Value<self::document_classifier::DocumentClassifierOutputDataConfig>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-comprehend-documentclassifier.html#cfn-comprehend-documentclassifier-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`VersionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-comprehend-documentclassifier.html#cfn-comprehend-documentclassifier-versionname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub version_name: Option<::Value<String>>,
    /// Property [`VolumeKmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-comprehend-documentclassifier.html#cfn-comprehend-documentclassifier-volumekmskeyid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub volume_kms_key_id: Option<::Value<String>>,
    /// Property [`VpcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-comprehend-documentclassifier.html#cfn-comprehend-documentclassifier-vpcconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub vpc_config: Option<::Value<self::document_classifier::VpcConfig>>,
}

impl ::serde::Serialize for DocumentClassifierProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataAccessRoleArn", &self.data_access_role_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentClassifierName", &self.document_classifier_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputDataConfig", &self.input_data_config)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LanguageCode", &self.language_code)?;
        if let Some(ref mode) = self.mode {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Mode", mode)?;
        }
        if let Some(ref model_kms_key_id) = self.model_kms_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelKmsKeyId", model_kms_key_id)?;
        }
        if let Some(ref model_policy) = self.model_policy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelPolicy", model_policy)?;
        }
        if let Some(ref output_data_config) = self.output_data_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutputDataConfig", output_data_config)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref version_name) = self.version_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VersionName", version_name)?;
        }
        if let Some(ref volume_kms_key_id) = self.volume_kms_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeKmsKeyId", volume_kms_key_id)?;
        }
        if let Some(ref vpc_config) = self.vpc_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcConfig", vpc_config)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DocumentClassifierProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DocumentClassifierProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DocumentClassifierProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DocumentClassifierProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut data_access_role_arn: Option<::Value<String>> = None;
                let mut document_classifier_name: Option<::Value<String>> = None;
                let mut input_data_config: Option<::Value<self::document_classifier::DocumentClassifierInputDataConfig>> = None;
                let mut language_code: Option<::Value<String>> = None;
                let mut mode: Option<::Value<String>> = None;
                let mut model_kms_key_id: Option<::Value<String>> = None;
                let mut model_policy: Option<::Value<String>> = None;
                let mut output_data_config: Option<::Value<self::document_classifier::DocumentClassifierOutputDataConfig>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut version_name: Option<::Value<String>> = None;
                let mut volume_kms_key_id: Option<::Value<String>> = None;
                let mut vpc_config: Option<::Value<self::document_classifier::VpcConfig>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DataAccessRoleArn" => {
                            data_access_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DocumentClassifierName" => {
                            document_classifier_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InputDataConfig" => {
                            input_data_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LanguageCode" => {
                            language_code = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Mode" => {
                            mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ModelKmsKeyId" => {
                            model_kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ModelPolicy" => {
                            model_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OutputDataConfig" => {
                            output_data_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VersionName" => {
                            version_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VolumeKmsKeyId" => {
                            volume_kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcConfig" => {
                            vpc_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DocumentClassifierProperties {
                    data_access_role_arn: data_access_role_arn.ok_or(::serde::de::Error::missing_field("DataAccessRoleArn"))?,
                    document_classifier_name: document_classifier_name.ok_or(::serde::de::Error::missing_field("DocumentClassifierName"))?,
                    input_data_config: input_data_config.ok_or(::serde::de::Error::missing_field("InputDataConfig"))?,
                    language_code: language_code.ok_or(::serde::de::Error::missing_field("LanguageCode"))?,
                    mode: mode,
                    model_kms_key_id: model_kms_key_id,
                    model_policy: model_policy,
                    output_data_config: output_data_config,
                    tags: tags,
                    version_name: version_name,
                    volume_kms_key_id: volume_kms_key_id,
                    vpc_config: vpc_config,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DocumentClassifier {
    type Properties = DocumentClassifierProperties;
    const TYPE: &'static str = "AWS::Comprehend::DocumentClassifier";
    fn properties(&self) -> &DocumentClassifierProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DocumentClassifierProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DocumentClassifier {}

impl From<DocumentClassifierProperties> for DocumentClassifier {
    fn from(properties: DocumentClassifierProperties) -> DocumentClassifier {
        DocumentClassifier { properties }
    }
}

/// The [`AWS::Comprehend::Flywheel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-comprehend-flywheel.html) resource type.
#[derive(Debug, Default)]
pub struct Flywheel {
    properties: FlywheelProperties
}

/// Properties for the `Flywheel` resource.
#[derive(Debug, Default)]
pub struct FlywheelProperties {
    /// Property [`ActiveModelArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-comprehend-flywheel.html#cfn-comprehend-flywheel-activemodelarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub active_model_arn: Option<::Value<String>>,
    /// Property [`DataAccessRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-comprehend-flywheel.html#cfn-comprehend-flywheel-dataaccessrolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub data_access_role_arn: ::Value<String>,
    /// Property [`DataLakeS3Uri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-comprehend-flywheel.html#cfn-comprehend-flywheel-datalakes3uri).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub data_lake_s3_uri: ::Value<String>,
    /// Property [`DataSecurityConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-comprehend-flywheel.html#cfn-comprehend-flywheel-datasecurityconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub data_security_config: Option<::Value<self::flywheel::DataSecurityConfig>>,
    /// Property [`FlywheelName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-comprehend-flywheel.html#cfn-comprehend-flywheel-flywheelname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub flywheel_name: ::Value<String>,
    /// Property [`ModelType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-comprehend-flywheel.html#cfn-comprehend-flywheel-modeltype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub model_type: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-comprehend-flywheel.html#cfn-comprehend-flywheel-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TaskConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-comprehend-flywheel.html#cfn-comprehend-flywheel-taskconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub task_config: Option<::Value<self::flywheel::TaskConfig>>,
}

impl ::serde::Serialize for FlywheelProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref active_model_arn) = self.active_model_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ActiveModelArn", active_model_arn)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataAccessRoleArn", &self.data_access_role_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataLakeS3Uri", &self.data_lake_s3_uri)?;
        if let Some(ref data_security_config) = self.data_security_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataSecurityConfig", data_security_config)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FlywheelName", &self.flywheel_name)?;
        if let Some(ref model_type) = self.model_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelType", model_type)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref task_config) = self.task_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TaskConfig", task_config)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for FlywheelProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<FlywheelProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = FlywheelProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type FlywheelProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut active_model_arn: Option<::Value<String>> = None;
                let mut data_access_role_arn: Option<::Value<String>> = None;
                let mut data_lake_s3_uri: Option<::Value<String>> = None;
                let mut data_security_config: Option<::Value<self::flywheel::DataSecurityConfig>> = None;
                let mut flywheel_name: Option<::Value<String>> = None;
                let mut model_type: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut task_config: Option<::Value<self::flywheel::TaskConfig>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ActiveModelArn" => {
                            active_model_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DataAccessRoleArn" => {
                            data_access_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DataLakeS3Uri" => {
                            data_lake_s3_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DataSecurityConfig" => {
                            data_security_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FlywheelName" => {
                            flywheel_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ModelType" => {
                            model_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TaskConfig" => {
                            task_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(FlywheelProperties {
                    active_model_arn: active_model_arn,
                    data_access_role_arn: data_access_role_arn.ok_or(::serde::de::Error::missing_field("DataAccessRoleArn"))?,
                    data_lake_s3_uri: data_lake_s3_uri.ok_or(::serde::de::Error::missing_field("DataLakeS3Uri"))?,
                    data_security_config: data_security_config,
                    flywheel_name: flywheel_name.ok_or(::serde::de::Error::missing_field("FlywheelName"))?,
                    model_type: model_type,
                    tags: tags,
                    task_config: task_config,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Flywheel {
    type Properties = FlywheelProperties;
    const TYPE: &'static str = "AWS::Comprehend::Flywheel";
    fn properties(&self) -> &FlywheelProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut FlywheelProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Flywheel {}

impl From<FlywheelProperties> for Flywheel {
    fn from(properties: FlywheelProperties) -> Flywheel {
        Flywheel { properties }
    }
}

pub mod document_classifier {
    //! Property types for the `DocumentClassifier` resource.

    /// The [`AWS::Comprehend::DocumentClassifier.AugmentedManifestsListItem`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-documentclassifier-augmentedmanifestslistitem.html) property type.
    #[derive(Debug, Default)]
    pub struct AugmentedManifestsListItem {
        /// Property [`AttributeNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-documentclassifier-augmentedmanifestslistitem.html#cfn-comprehend-documentclassifier-augmentedmanifestslistitem-attributenames).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub attribute_names: ::ValueList<String>,
        /// Property [`S3Uri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-documentclassifier-augmentedmanifestslistitem.html#cfn-comprehend-documentclassifier-augmentedmanifestslistitem-s3uri).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_uri: ::Value<String>,
        /// Property [`Split`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-documentclassifier-augmentedmanifestslistitem.html#cfn-comprehend-documentclassifier-augmentedmanifestslistitem-split).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub split: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AugmentedManifestsListItem {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttributeNames", &self.attribute_names)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Uri", &self.s3_uri)?;
            if let Some(ref split) = self.split {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Split", split)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AugmentedManifestsListItem {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AugmentedManifestsListItem, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AugmentedManifestsListItem;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AugmentedManifestsListItem")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attribute_names: Option<::ValueList<String>> = None;
                    let mut s3_uri: Option<::Value<String>> = None;
                    let mut split: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AttributeNames" => {
                                attribute_names = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Uri" => {
                                s3_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Split" => {
                                split = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AugmentedManifestsListItem {
                        attribute_names: attribute_names.ok_or(::serde::de::Error::missing_field("AttributeNames"))?,
                        s3_uri: s3_uri.ok_or(::serde::de::Error::missing_field("S3Uri"))?,
                        split: split,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Comprehend::DocumentClassifier.DocumentClassifierDocuments`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-documentclassifier-documentclassifierdocuments.html) property type.
    #[derive(Debug, Default)]
    pub struct DocumentClassifierDocuments {
        /// Property [`S3Uri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-documentclassifier-documentclassifierdocuments.html#cfn-comprehend-documentclassifier-documentclassifierdocuments-s3uri).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_uri: ::Value<String>,
        /// Property [`TestS3Uri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-documentclassifier-documentclassifierdocuments.html#cfn-comprehend-documentclassifier-documentclassifierdocuments-tests3uri).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub test_s3_uri: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DocumentClassifierDocuments {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Uri", &self.s3_uri)?;
            if let Some(ref test_s3_uri) = self.test_s3_uri {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TestS3Uri", test_s3_uri)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DocumentClassifierDocuments {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DocumentClassifierDocuments, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DocumentClassifierDocuments;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DocumentClassifierDocuments")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut s3_uri: Option<::Value<String>> = None;
                    let mut test_s3_uri: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3Uri" => {
                                s3_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TestS3Uri" => {
                                test_s3_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DocumentClassifierDocuments {
                        s3_uri: s3_uri.ok_or(::serde::de::Error::missing_field("S3Uri"))?,
                        test_s3_uri: test_s3_uri,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Comprehend::DocumentClassifier.DocumentClassifierInputDataConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-documentclassifier-documentclassifierinputdataconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct DocumentClassifierInputDataConfig {
        /// Property [`AugmentedManifests`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-documentclassifier-documentclassifierinputdataconfig.html#cfn-comprehend-documentclassifier-documentclassifierinputdataconfig-augmentedmanifests).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub augmented_manifests: Option<::ValueList<AugmentedManifestsListItem>>,
        /// Property [`DataFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-documentclassifier-documentclassifierinputdataconfig.html#cfn-comprehend-documentclassifier-documentclassifierinputdataconfig-dataformat).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub data_format: Option<::Value<String>>,
        /// Property [`DocumentReaderConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-documentclassifier-documentclassifierinputdataconfig.html#cfn-comprehend-documentclassifier-documentclassifierinputdataconfig-documentreaderconfig).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub document_reader_config: Option<::Value<DocumentReaderConfig>>,
        /// Property [`DocumentType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-documentclassifier-documentclassifierinputdataconfig.html#cfn-comprehend-documentclassifier-documentclassifierinputdataconfig-documenttype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub document_type: Option<::Value<String>>,
        /// Property [`Documents`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-documentclassifier-documentclassifierinputdataconfig.html#cfn-comprehend-documentclassifier-documentclassifierinputdataconfig-documents).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub documents: Option<::Value<DocumentClassifierDocuments>>,
        /// Property [`LabelDelimiter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-documentclassifier-documentclassifierinputdataconfig.html#cfn-comprehend-documentclassifier-documentclassifierinputdataconfig-labeldelimiter).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub label_delimiter: Option<::Value<String>>,
        /// Property [`S3Uri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-documentclassifier-documentclassifierinputdataconfig.html#cfn-comprehend-documentclassifier-documentclassifierinputdataconfig-s3uri).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_uri: Option<::Value<String>>,
        /// Property [`TestS3Uri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-documentclassifier-documentclassifierinputdataconfig.html#cfn-comprehend-documentclassifier-documentclassifierinputdataconfig-tests3uri).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub test_s3_uri: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DocumentClassifierInputDataConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref augmented_manifests) = self.augmented_manifests {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AugmentedManifests", augmented_manifests)?;
            }
            if let Some(ref data_format) = self.data_format {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataFormat", data_format)?;
            }
            if let Some(ref document_reader_config) = self.document_reader_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentReaderConfig", document_reader_config)?;
            }
            if let Some(ref document_type) = self.document_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentType", document_type)?;
            }
            if let Some(ref documents) = self.documents {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Documents", documents)?;
            }
            if let Some(ref label_delimiter) = self.label_delimiter {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LabelDelimiter", label_delimiter)?;
            }
            if let Some(ref s3_uri) = self.s3_uri {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Uri", s3_uri)?;
            }
            if let Some(ref test_s3_uri) = self.test_s3_uri {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TestS3Uri", test_s3_uri)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DocumentClassifierInputDataConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DocumentClassifierInputDataConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DocumentClassifierInputDataConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DocumentClassifierInputDataConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut augmented_manifests: Option<::ValueList<AugmentedManifestsListItem>> = None;
                    let mut data_format: Option<::Value<String>> = None;
                    let mut document_reader_config: Option<::Value<DocumentReaderConfig>> = None;
                    let mut document_type: Option<::Value<String>> = None;
                    let mut documents: Option<::Value<DocumentClassifierDocuments>> = None;
                    let mut label_delimiter: Option<::Value<String>> = None;
                    let mut s3_uri: Option<::Value<String>> = None;
                    let mut test_s3_uri: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AugmentedManifests" => {
                                augmented_manifests = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataFormat" => {
                                data_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DocumentReaderConfig" => {
                                document_reader_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DocumentType" => {
                                document_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Documents" => {
                                documents = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LabelDelimiter" => {
                                label_delimiter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Uri" => {
                                s3_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TestS3Uri" => {
                                test_s3_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DocumentClassifierInputDataConfig {
                        augmented_manifests: augmented_manifests,
                        data_format: data_format,
                        document_reader_config: document_reader_config,
                        document_type: document_type,
                        documents: documents,
                        label_delimiter: label_delimiter,
                        s3_uri: s3_uri,
                        test_s3_uri: test_s3_uri,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Comprehend::DocumentClassifier.DocumentClassifierOutputDataConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-documentclassifier-documentclassifieroutputdataconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct DocumentClassifierOutputDataConfig {
        /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-documentclassifier-documentclassifieroutputdataconfig.html#cfn-comprehend-documentclassifier-documentclassifieroutputdataconfig-kmskeyid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub kms_key_id: Option<::Value<String>>,
        /// Property [`S3Uri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-documentclassifier-documentclassifieroutputdataconfig.html#cfn-comprehend-documentclassifier-documentclassifieroutputdataconfig-s3uri).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_uri: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DocumentClassifierOutputDataConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref kms_key_id) = self.kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
            }
            if let Some(ref s3_uri) = self.s3_uri {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Uri", s3_uri)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DocumentClassifierOutputDataConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DocumentClassifierOutputDataConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DocumentClassifierOutputDataConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DocumentClassifierOutputDataConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut kms_key_id: Option<::Value<String>> = None;
                    let mut s3_uri: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KmsKeyId" => {
                                kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Uri" => {
                                s3_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DocumentClassifierOutputDataConfig {
                        kms_key_id: kms_key_id,
                        s3_uri: s3_uri,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Comprehend::DocumentClassifier.DocumentReaderConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-documentclassifier-documentreaderconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct DocumentReaderConfig {
        /// Property [`DocumentReadAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-documentclassifier-documentreaderconfig.html#cfn-comprehend-documentclassifier-documentreaderconfig-documentreadaction).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub document_read_action: ::Value<String>,
        /// Property [`DocumentReadMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-documentclassifier-documentreaderconfig.html#cfn-comprehend-documentclassifier-documentreaderconfig-documentreadmode).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub document_read_mode: Option<::Value<String>>,
        /// Property [`FeatureTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-documentclassifier-documentreaderconfig.html#cfn-comprehend-documentclassifier-documentreaderconfig-featuretypes).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub feature_types: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for DocumentReaderConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentReadAction", &self.document_read_action)?;
            if let Some(ref document_read_mode) = self.document_read_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentReadMode", document_read_mode)?;
            }
            if let Some(ref feature_types) = self.feature_types {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FeatureTypes", feature_types)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DocumentReaderConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DocumentReaderConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DocumentReaderConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DocumentReaderConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut document_read_action: Option<::Value<String>> = None;
                    let mut document_read_mode: Option<::Value<String>> = None;
                    let mut feature_types: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DocumentReadAction" => {
                                document_read_action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DocumentReadMode" => {
                                document_read_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FeatureTypes" => {
                                feature_types = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DocumentReaderConfig {
                        document_read_action: document_read_action.ok_or(::serde::de::Error::missing_field("DocumentReadAction"))?,
                        document_read_mode: document_read_mode,
                        feature_types: feature_types,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Comprehend::DocumentClassifier.VpcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-documentclassifier-vpcconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct VpcConfig {
        /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-documentclassifier-vpcconfig.html#cfn-comprehend-documentclassifier-vpcconfig-securitygroupids).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub security_group_ids: ::ValueList<String>,
        /// Property [`Subnets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-documentclassifier-vpcconfig.html#cfn-comprehend-documentclassifier-vpcconfig-subnets).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub subnets: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for VpcConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", &self.security_group_ids)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subnets", &self.subnets)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VpcConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VpcConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VpcConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VpcConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut security_group_ids: Option<::ValueList<String>> = None;
                    let mut subnets: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SecurityGroupIds" => {
                                security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Subnets" => {
                                subnets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VpcConfig {
                        security_group_ids: security_group_ids.ok_or(::serde::de::Error::missing_field("SecurityGroupIds"))?,
                        subnets: subnets.ok_or(::serde::de::Error::missing_field("Subnets"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod flywheel {
    //! Property types for the `Flywheel` resource.

    /// The [`AWS::Comprehend::Flywheel.DataSecurityConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-flywheel-datasecurityconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct DataSecurityConfig {
        /// Property [`DataLakeKmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-flywheel-datasecurityconfig.html#cfn-comprehend-flywheel-datasecurityconfig-datalakekmskeyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub data_lake_kms_key_id: Option<::Value<String>>,
        /// Property [`ModelKmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-flywheel-datasecurityconfig.html#cfn-comprehend-flywheel-datasecurityconfig-modelkmskeyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub model_kms_key_id: Option<::Value<String>>,
        /// Property [`VolumeKmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-flywheel-datasecurityconfig.html#cfn-comprehend-flywheel-datasecurityconfig-volumekmskeyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub volume_kms_key_id: Option<::Value<String>>,
        /// Property [`VpcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-flywheel-datasecurityconfig.html#cfn-comprehend-flywheel-datasecurityconfig-vpcconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vpc_config: Option<::Value<VpcConfig>>,
    }

    impl ::codec::SerializeValue for DataSecurityConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref data_lake_kms_key_id) = self.data_lake_kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataLakeKmsKeyId", data_lake_kms_key_id)?;
            }
            if let Some(ref model_kms_key_id) = self.model_kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelKmsKeyId", model_kms_key_id)?;
            }
            if let Some(ref volume_kms_key_id) = self.volume_kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeKmsKeyId", volume_kms_key_id)?;
            }
            if let Some(ref vpc_config) = self.vpc_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcConfig", vpc_config)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataSecurityConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataSecurityConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataSecurityConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataSecurityConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut data_lake_kms_key_id: Option<::Value<String>> = None;
                    let mut model_kms_key_id: Option<::Value<String>> = None;
                    let mut volume_kms_key_id: Option<::Value<String>> = None;
                    let mut vpc_config: Option<::Value<VpcConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DataLakeKmsKeyId" => {
                                data_lake_kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ModelKmsKeyId" => {
                                model_kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VolumeKmsKeyId" => {
                                volume_kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpcConfig" => {
                                vpc_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataSecurityConfig {
                        data_lake_kms_key_id: data_lake_kms_key_id,
                        model_kms_key_id: model_kms_key_id,
                        volume_kms_key_id: volume_kms_key_id,
                        vpc_config: vpc_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Comprehend::Flywheel.DocumentClassificationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-flywheel-documentclassificationconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct DocumentClassificationConfig {
        /// Property [`Labels`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-flywheel-documentclassificationconfig.html#cfn-comprehend-flywheel-documentclassificationconfig-labels).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub labels: Option<::ValueList<String>>,
        /// Property [`Mode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-flywheel-documentclassificationconfig.html#cfn-comprehend-flywheel-documentclassificationconfig-mode).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub mode: ::Value<String>,
    }

    impl ::codec::SerializeValue for DocumentClassificationConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref labels) = self.labels {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Labels", labels)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Mode", &self.mode)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DocumentClassificationConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DocumentClassificationConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DocumentClassificationConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DocumentClassificationConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut labels: Option<::ValueList<String>> = None;
                    let mut mode: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Labels" => {
                                labels = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Mode" => {
                                mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DocumentClassificationConfig {
                        labels: labels,
                        mode: mode.ok_or(::serde::de::Error::missing_field("Mode"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Comprehend::Flywheel.EntityRecognitionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-flywheel-entityrecognitionconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct EntityRecognitionConfig {
        /// Property [`EntityTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-flywheel-entityrecognitionconfig.html#cfn-comprehend-flywheel-entityrecognitionconfig-entitytypes).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub entity_types: Option<::ValueList<EntityTypesListItem>>,
    }

    impl ::codec::SerializeValue for EntityRecognitionConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref entity_types) = self.entity_types {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EntityTypes", entity_types)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EntityRecognitionConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EntityRecognitionConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EntityRecognitionConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EntityRecognitionConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut entity_types: Option<::ValueList<EntityTypesListItem>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EntityTypes" => {
                                entity_types = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EntityRecognitionConfig {
                        entity_types: entity_types,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Comprehend::Flywheel.EntityTypesListItem`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-flywheel-entitytypeslistitem.html) property type.
    #[derive(Debug, Default)]
    pub struct EntityTypesListItem {
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-flywheel-entitytypeslistitem.html#cfn-comprehend-flywheel-entitytypeslistitem-type).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for EntityTypesListItem {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EntityTypesListItem {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EntityTypesListItem, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EntityTypesListItem;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EntityTypesListItem")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EntityTypesListItem {
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Comprehend::Flywheel.TaskConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-flywheel-taskconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct TaskConfig {
        /// Property [`DocumentClassificationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-flywheel-taskconfig.html#cfn-comprehend-flywheel-taskconfig-documentclassificationconfig).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub document_classification_config: Option<::Value<DocumentClassificationConfig>>,
        /// Property [`EntityRecognitionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-flywheel-taskconfig.html#cfn-comprehend-flywheel-taskconfig-entityrecognitionconfig).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub entity_recognition_config: Option<::Value<EntityRecognitionConfig>>,
        /// Property [`LanguageCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-flywheel-taskconfig.html#cfn-comprehend-flywheel-taskconfig-languagecode).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub language_code: ::Value<String>,
    }

    impl ::codec::SerializeValue for TaskConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref document_classification_config) = self.document_classification_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentClassificationConfig", document_classification_config)?;
            }
            if let Some(ref entity_recognition_config) = self.entity_recognition_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EntityRecognitionConfig", entity_recognition_config)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LanguageCode", &self.language_code)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TaskConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TaskConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TaskConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TaskConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut document_classification_config: Option<::Value<DocumentClassificationConfig>> = None;
                    let mut entity_recognition_config: Option<::Value<EntityRecognitionConfig>> = None;
                    let mut language_code: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DocumentClassificationConfig" => {
                                document_classification_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EntityRecognitionConfig" => {
                                entity_recognition_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LanguageCode" => {
                                language_code = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TaskConfig {
                        document_classification_config: document_classification_config,
                        entity_recognition_config: entity_recognition_config,
                        language_code: language_code.ok_or(::serde::de::Error::missing_field("LanguageCode"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Comprehend::Flywheel.VpcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-flywheel-vpcconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct VpcConfig {
        /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-flywheel-vpcconfig.html#cfn-comprehend-flywheel-vpcconfig-securitygroupids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_group_ids: ::ValueList<String>,
        /// Property [`Subnets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-comprehend-flywheel-vpcconfig.html#cfn-comprehend-flywheel-vpcconfig-subnets).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subnets: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for VpcConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", &self.security_group_ids)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subnets", &self.subnets)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VpcConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VpcConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VpcConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VpcConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut security_group_ids: Option<::ValueList<String>> = None;
                    let mut subnets: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SecurityGroupIds" => {
                                security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Subnets" => {
                                subnets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VpcConfig {
                        security_group_ids: security_group_ids.ok_or(::serde::de::Error::missing_field("SecurityGroupIds"))?,
                        subnets: subnets.ok_or(::serde::de::Error::missing_field("Subnets"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
