//! Types for the `LookoutEquipment` service.

/// The [`AWS::LookoutEquipment::InferenceScheduler`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lookoutequipment-inferencescheduler.html) resource type.
#[derive(Debug, Default)]
pub struct InferenceScheduler {
    properties: InferenceSchedulerProperties
}

/// Properties for the `InferenceScheduler` resource.
#[derive(Debug, Default)]
pub struct InferenceSchedulerProperties {
    /// Property [`DataDelayOffsetInMinutes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lookoutequipment-inferencescheduler.html#cfn-lookoutequipment-inferencescheduler-datadelayoffsetinminutes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub data_delay_offset_in_minutes: Option<::Value<u32>>,
    /// Property [`DataInputConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lookoutequipment-inferencescheduler.html#cfn-lookoutequipment-inferencescheduler-datainputconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub data_input_configuration: ::Value<self::inference_scheduler::DataInputConfiguration>,
    /// Property [`DataOutputConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lookoutequipment-inferencescheduler.html#cfn-lookoutequipment-inferencescheduler-dataoutputconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub data_output_configuration: ::Value<self::inference_scheduler::DataOutputConfiguration>,
    /// Property [`DataUploadFrequency`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lookoutequipment-inferencescheduler.html#cfn-lookoutequipment-inferencescheduler-datauploadfrequency).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub data_upload_frequency: ::Value<String>,
    /// Property [`InferenceSchedulerName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lookoutequipment-inferencescheduler.html#cfn-lookoutequipment-inferencescheduler-inferenceschedulername).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub inference_scheduler_name: Option<::Value<String>>,
    /// Property [`ModelName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lookoutequipment-inferencescheduler.html#cfn-lookoutequipment-inferencescheduler-modelname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub model_name: ::Value<String>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lookoutequipment-inferencescheduler.html#cfn-lookoutequipment-inferencescheduler-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: ::Value<String>,
    /// Property [`ServerSideKmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lookoutequipment-inferencescheduler.html#cfn-lookoutequipment-inferencescheduler-serversidekmskeyid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub server_side_kms_key_id: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lookoutequipment-inferencescheduler.html#cfn-lookoutequipment-inferencescheduler-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for InferenceSchedulerProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref data_delay_offset_in_minutes) = self.data_delay_offset_in_minutes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataDelayOffsetInMinutes", data_delay_offset_in_minutes)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataInputConfiguration", &self.data_input_configuration)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataOutputConfiguration", &self.data_output_configuration)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataUploadFrequency", &self.data_upload_frequency)?;
        if let Some(ref inference_scheduler_name) = self.inference_scheduler_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InferenceSchedulerName", inference_scheduler_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelName", &self.model_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        if let Some(ref server_side_kms_key_id) = self.server_side_kms_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerSideKmsKeyId", server_side_kms_key_id)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for InferenceSchedulerProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<InferenceSchedulerProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = InferenceSchedulerProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type InferenceSchedulerProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut data_delay_offset_in_minutes: Option<::Value<u32>> = None;
                let mut data_input_configuration: Option<::Value<self::inference_scheduler::DataInputConfiguration>> = None;
                let mut data_output_configuration: Option<::Value<self::inference_scheduler::DataOutputConfiguration>> = None;
                let mut data_upload_frequency: Option<::Value<String>> = None;
                let mut inference_scheduler_name: Option<::Value<String>> = None;
                let mut model_name: Option<::Value<String>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut server_side_kms_key_id: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DataDelayOffsetInMinutes" => {
                            data_delay_offset_in_minutes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DataInputConfiguration" => {
                            data_input_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DataOutputConfiguration" => {
                            data_output_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DataUploadFrequency" => {
                            data_upload_frequency = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InferenceSchedulerName" => {
                            inference_scheduler_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ModelName" => {
                            model_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServerSideKmsKeyId" => {
                            server_side_kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(InferenceSchedulerProperties {
                    data_delay_offset_in_minutes: data_delay_offset_in_minutes,
                    data_input_configuration: data_input_configuration.ok_or(::serde::de::Error::missing_field("DataInputConfiguration"))?,
                    data_output_configuration: data_output_configuration.ok_or(::serde::de::Error::missing_field("DataOutputConfiguration"))?,
                    data_upload_frequency: data_upload_frequency.ok_or(::serde::de::Error::missing_field("DataUploadFrequency"))?,
                    inference_scheduler_name: inference_scheduler_name,
                    model_name: model_name.ok_or(::serde::de::Error::missing_field("ModelName"))?,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    server_side_kms_key_id: server_side_kms_key_id,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for InferenceScheduler {
    type Properties = InferenceSchedulerProperties;
    const TYPE: &'static str = "AWS::LookoutEquipment::InferenceScheduler";
    fn properties(&self) -> &InferenceSchedulerProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut InferenceSchedulerProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for InferenceScheduler {}

impl From<InferenceSchedulerProperties> for InferenceScheduler {
    fn from(properties: InferenceSchedulerProperties) -> InferenceScheduler {
        InferenceScheduler { properties }
    }
}

pub mod inference_scheduler {
    //! Property types for the `InferenceScheduler` resource.

    /// The [`AWS::LookoutEquipment::InferenceScheduler.DataInputConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutequipment-inferencescheduler-datainputconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct DataInputConfiguration {
        /// Property [`InferenceInputNameConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutequipment-inferencescheduler-datainputconfiguration.html#cfn-lookoutequipment-inferencescheduler-datainputconfiguration-inferenceinputnameconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub inference_input_name_configuration: Option<::Value<InputNameConfiguration>>,
        /// Property [`InputTimeZoneOffset`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutequipment-inferencescheduler-datainputconfiguration.html#cfn-lookoutequipment-inferencescheduler-datainputconfiguration-inputtimezoneoffset).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_time_zone_offset: Option<::Value<String>>,
        /// Property [`S3InputConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutequipment-inferencescheduler-datainputconfiguration.html#cfn-lookoutequipment-inferencescheduler-datainputconfiguration-s3inputconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_input_configuration: ::Value<S3InputConfiguration>,
    }

    impl ::codec::SerializeValue for DataInputConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref inference_input_name_configuration) = self.inference_input_name_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InferenceInputNameConfiguration", inference_input_name_configuration)?;
            }
            if let Some(ref input_time_zone_offset) = self.input_time_zone_offset {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputTimeZoneOffset", input_time_zone_offset)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3InputConfiguration", &self.s3_input_configuration)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataInputConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataInputConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataInputConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataInputConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut inference_input_name_configuration: Option<::Value<InputNameConfiguration>> = None;
                    let mut input_time_zone_offset: Option<::Value<String>> = None;
                    let mut s3_input_configuration: Option<::Value<S3InputConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InferenceInputNameConfiguration" => {
                                inference_input_name_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InputTimeZoneOffset" => {
                                input_time_zone_offset = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3InputConfiguration" => {
                                s3_input_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataInputConfiguration {
                        inference_input_name_configuration: inference_input_name_configuration,
                        input_time_zone_offset: input_time_zone_offset,
                        s3_input_configuration: s3_input_configuration.ok_or(::serde::de::Error::missing_field("S3InputConfiguration"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LookoutEquipment::InferenceScheduler.DataOutputConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutequipment-inferencescheduler-dataoutputconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct DataOutputConfiguration {
        /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutequipment-inferencescheduler-dataoutputconfiguration.html#cfn-lookoutequipment-inferencescheduler-dataoutputconfiguration-kmskeyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kms_key_id: Option<::Value<String>>,
        /// Property [`S3OutputConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutequipment-inferencescheduler-dataoutputconfiguration.html#cfn-lookoutequipment-inferencescheduler-dataoutputconfiguration-s3outputconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_output_configuration: ::Value<S3OutputConfiguration>,
    }

    impl ::codec::SerializeValue for DataOutputConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref kms_key_id) = self.kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3OutputConfiguration", &self.s3_output_configuration)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataOutputConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataOutputConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataOutputConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataOutputConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut kms_key_id: Option<::Value<String>> = None;
                    let mut s3_output_configuration: Option<::Value<S3OutputConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KmsKeyId" => {
                                kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3OutputConfiguration" => {
                                s3_output_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataOutputConfiguration {
                        kms_key_id: kms_key_id,
                        s3_output_configuration: s3_output_configuration.ok_or(::serde::de::Error::missing_field("S3OutputConfiguration"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LookoutEquipment::InferenceScheduler.InputNameConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutequipment-inferencescheduler-inputnameconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct InputNameConfiguration {
        /// Property [`ComponentTimestampDelimiter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutequipment-inferencescheduler-inputnameconfiguration.html#cfn-lookoutequipment-inferencescheduler-inputnameconfiguration-componenttimestampdelimiter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub component_timestamp_delimiter: Option<::Value<String>>,
        /// Property [`TimestampFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutequipment-inferencescheduler-inputnameconfiguration.html#cfn-lookoutequipment-inferencescheduler-inputnameconfiguration-timestampformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timestamp_format: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for InputNameConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref component_timestamp_delimiter) = self.component_timestamp_delimiter {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComponentTimestampDelimiter", component_timestamp_delimiter)?;
            }
            if let Some(ref timestamp_format) = self.timestamp_format {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimestampFormat", timestamp_format)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InputNameConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InputNameConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InputNameConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InputNameConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut component_timestamp_delimiter: Option<::Value<String>> = None;
                    let mut timestamp_format: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ComponentTimestampDelimiter" => {
                                component_timestamp_delimiter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimestampFormat" => {
                                timestamp_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InputNameConfiguration {
                        component_timestamp_delimiter: component_timestamp_delimiter,
                        timestamp_format: timestamp_format,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LookoutEquipment::InferenceScheduler.S3InputConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutequipment-inferencescheduler-s3inputconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct S3InputConfiguration {
        /// Property [`Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutequipment-inferencescheduler-s3inputconfiguration.html#cfn-lookoutequipment-inferencescheduler-s3inputconfiguration-bucket).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket: ::Value<String>,
        /// Property [`Prefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutequipment-inferencescheduler-s3inputconfiguration.html#cfn-lookoutequipment-inferencescheduler-s3inputconfiguration-prefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub prefix: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for S3InputConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bucket", &self.bucket)?;
            if let Some(ref prefix) = self.prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Prefix", prefix)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3InputConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3InputConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3InputConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3InputConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket: Option<::Value<String>> = None;
                    let mut prefix: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Bucket" => {
                                bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Prefix" => {
                                prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3InputConfiguration {
                        bucket: bucket.ok_or(::serde::de::Error::missing_field("Bucket"))?,
                        prefix: prefix,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::LookoutEquipment::InferenceScheduler.S3OutputConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutequipment-inferencescheduler-s3outputconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct S3OutputConfiguration {
        /// Property [`Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutequipment-inferencescheduler-s3outputconfiguration.html#cfn-lookoutequipment-inferencescheduler-s3outputconfiguration-bucket).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket: ::Value<String>,
        /// Property [`Prefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutequipment-inferencescheduler-s3outputconfiguration.html#cfn-lookoutequipment-inferencescheduler-s3outputconfiguration-prefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub prefix: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for S3OutputConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bucket", &self.bucket)?;
            if let Some(ref prefix) = self.prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Prefix", prefix)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3OutputConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3OutputConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3OutputConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3OutputConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket: Option<::Value<String>> = None;
                    let mut prefix: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Bucket" => {
                                bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Prefix" => {
                                prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3OutputConfiguration {
                        bucket: bucket.ok_or(::serde::de::Error::missing_field("Bucket"))?,
                        prefix: prefix,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
