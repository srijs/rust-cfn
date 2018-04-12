//! Types for the `DataPipeline` service.

/// The [`AWS::DataPipeline::Pipeline`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datapipeline-pipeline.html) resource type.
#[derive(Debug)]
pub struct Pipeline {
    properties: PipelineProperties
}

/// Properties for the `Pipeline` resource.
#[derive(Debug)]
pub struct PipelineProperties {
    /// Property `Activate`.
    pub activate: Option<::Value<bool>>,
    /// Property `Description`.
    pub description: Option<::Value<String>>,
    /// Property `Name`.
    pub name: ::Value<String>,
    /// Property `ParameterObjects`.
    pub parameter_objects: ::ValueList<self::pipeline::ParameterObject>,
    /// Property `ParameterValues`.
    pub parameter_values: Option<::ValueList<self::pipeline::ParameterValue>>,
    /// Property `PipelineObjects`.
    pub pipeline_objects: Option<::ValueList<self::pipeline::PipelineObject>>,
    /// Property `PipelineTags`.
    pub pipeline_tags: Option<::ValueList<self::pipeline::PipelineTag>>,
}

impl ::serde::Serialize for PipelineProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref activate) = self.activate {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Activate", activate)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParameterObjects", &self.parameter_objects)?;
        if let Some(ref parameter_values) = self.parameter_values {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParameterValues", parameter_values)?;
        }
        if let Some(ref pipeline_objects) = self.pipeline_objects {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PipelineObjects", pipeline_objects)?;
        }
        if let Some(ref pipeline_tags) = self.pipeline_tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PipelineTags", pipeline_tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PipelineProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PipelineProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PipelineProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PipelineProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut activate: Option<::Value<bool>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut parameter_objects: Option<::ValueList<self::pipeline::ParameterObject>> = None;
                let mut parameter_values: Option<::ValueList<self::pipeline::ParameterValue>> = None;
                let mut pipeline_objects: Option<::ValueList<self::pipeline::PipelineObject>> = None;
                let mut pipeline_tags: Option<::ValueList<self::pipeline::PipelineTag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Activate" => {
                            activate = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ParameterObjects" => {
                            parameter_objects = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ParameterValues" => {
                            parameter_values = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PipelineObjects" => {
                            pipeline_objects = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PipelineTags" => {
                            pipeline_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PipelineProperties {
                    activate: activate,
                    description: description,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    parameter_objects: parameter_objects.ok_or(::serde::de::Error::missing_field("ParameterObjects"))?,
                    parameter_values: parameter_values,
                    pipeline_objects: pipeline_objects,
                    pipeline_tags: pipeline_tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Pipeline {
    type Properties = PipelineProperties;
    const TYPE: &'static str = "AWS::DataPipeline::Pipeline";
    fn properties(&self) -> &PipelineProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PipelineProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Pipeline {}

impl From<PipelineProperties> for Pipeline {
    fn from(properties: PipelineProperties) -> Pipeline {
        Pipeline { properties }
    }
}

pub mod pipeline {
    //! Property types for the `Pipeline` resource.

    /// The [`AWS::DataPipeline::Pipeline.Field`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datapipeline-pipeline-pipelineobjects-fields.html) property type.
    #[derive(Debug)]
    pub struct Field {
        /// Property `Key`.
        pub key: ::Value<String>,
        /// Property `RefValue`.
        pub ref_value: Option<::Value<String>>,
        /// Property `StringValue`.
        pub string_value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Field {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            if let Some(ref ref_value) = self.ref_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RefValue", ref_value)?;
            }
            if let Some(ref string_value) = self.string_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StringValue", string_value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Field {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Field, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Field;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Field")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut ref_value: Option<::Value<String>> = None;
                    let mut string_value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RefValue" => {
                                ref_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StringValue" => {
                                string_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Field {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        ref_value: ref_value,
                        string_value: string_value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataPipeline::Pipeline.ParameterAttribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datapipeline-pipeline-parameterobjects-attributes.html) property type.
    #[derive(Debug)]
    pub struct ParameterAttribute {
        /// Property `Key`.
        pub key: ::Value<String>,
        /// Property `StringValue`.
        pub string_value: ::Value<String>,
    }

    impl ::codec::SerializeValue for ParameterAttribute {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StringValue", &self.string_value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ParameterAttribute {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ParameterAttribute, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ParameterAttribute;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ParameterAttribute")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut string_value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StringValue" => {
                                string_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ParameterAttribute {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        string_value: string_value.ok_or(::serde::de::Error::missing_field("StringValue"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataPipeline::Pipeline.ParameterObject`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datapipeline-pipeline-parameterobjects.html) property type.
    #[derive(Debug)]
    pub struct ParameterObject {
        /// Property `Attributes`.
        pub attributes: ::ValueList<ParameterAttribute>,
        /// Property `Id`.
        pub id: ::Value<String>,
    }

    impl ::codec::SerializeValue for ParameterObject {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Attributes", &self.attributes)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ParameterObject {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ParameterObject, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ParameterObject;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ParameterObject")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attributes: Option<::ValueList<ParameterAttribute>> = None;
                    let mut id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Attributes" => {
                                attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ParameterObject {
                        attributes: attributes.ok_or(::serde::de::Error::missing_field("Attributes"))?,
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataPipeline::Pipeline.ParameterValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datapipeline-pipeline-parametervalues.html) property type.
    #[derive(Debug)]
    pub struct ParameterValue {
        /// Property `Id`.
        pub id: ::Value<String>,
        /// Property `StringValue`.
        pub string_value: ::Value<String>,
    }

    impl ::codec::SerializeValue for ParameterValue {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StringValue", &self.string_value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ParameterValue {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ParameterValue, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ParameterValue;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ParameterValue")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut id: Option<::Value<String>> = None;
                    let mut string_value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StringValue" => {
                                string_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ParameterValue {
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                        string_value: string_value.ok_or(::serde::de::Error::missing_field("StringValue"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataPipeline::Pipeline.PipelineObject`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datapipeline-pipeline-pipelineobjects.html) property type.
    #[derive(Debug)]
    pub struct PipelineObject {
        /// Property `Fields`.
        pub fields: ::ValueList<Field>,
        /// Property `Id`.
        pub id: ::Value<String>,
        /// Property `Name`.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for PipelineObject {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Fields", &self.fields)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PipelineObject {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PipelineObject, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PipelineObject;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PipelineObject")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut fields: Option<::ValueList<Field>> = None;
                    let mut id: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Fields" => {
                                fields = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PipelineObject {
                        fields: fields.ok_or(::serde::de::Error::missing_field("Fields"))?,
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::DataPipeline::Pipeline.PipelineTag`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datapipeline-pipeline-pipelinetags.html) property type.
    #[derive(Debug)]
    pub struct PipelineTag {
        /// Property `Key`.
        pub key: ::Value<String>,
        /// Property `Value`.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for PipelineTag {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PipelineTag {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PipelineTag, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PipelineTag;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PipelineTag")
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

                    Ok(PipelineTag {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
