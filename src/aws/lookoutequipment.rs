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
    pub data_input_configuration: ::Value<::json::Value>,
    /// Property [`DataOutputConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lookoutequipment-inferencescheduler.html#cfn-lookoutequipment-inferencescheduler-dataoutputconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub data_output_configuration: ::Value<::json::Value>,
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
                let mut data_input_configuration: Option<::Value<::json::Value>> = None;
                let mut data_output_configuration: Option<::Value<::json::Value>> = None;
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
