//! Types for the `EntityResolution` service.

/// The [`AWS::EntityResolution::IdMappingWorkflow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-entityresolution-idmappingworkflow.html) resource type.
#[derive(Debug, Default)]
pub struct IdMappingWorkflow {
    properties: IdMappingWorkflowProperties
}

/// Properties for the `IdMappingWorkflow` resource.
#[derive(Debug, Default)]
pub struct IdMappingWorkflowProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-entityresolution-idmappingworkflow.html#cfn-entityresolution-idmappingworkflow-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`IdMappingTechniques`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-entityresolution-idmappingworkflow.html#cfn-entityresolution-idmappingworkflow-idmappingtechniques).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub id_mapping_techniques: ::Value<self::id_mapping_workflow::IdMappingTechniques>,
    /// Property [`InputSourceConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-entityresolution-idmappingworkflow.html#cfn-entityresolution-idmappingworkflow-inputsourceconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub input_source_config: ::ValueList<self::id_mapping_workflow::IdMappingWorkflowInputSource>,
    /// Property [`OutputSourceConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-entityresolution-idmappingworkflow.html#cfn-entityresolution-idmappingworkflow-outputsourceconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub output_source_config: ::ValueList<self::id_mapping_workflow::IdMappingWorkflowOutputSource>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-entityresolution-idmappingworkflow.html#cfn-entityresolution-idmappingworkflow-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-entityresolution-idmappingworkflow.html#cfn-entityresolution-idmappingworkflow-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`WorkflowName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-entityresolution-idmappingworkflow.html#cfn-entityresolution-idmappingworkflow-workflowname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub workflow_name: ::Value<String>,
}

impl ::serde::Serialize for IdMappingWorkflowProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdMappingTechniques", &self.id_mapping_techniques)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputSourceConfig", &self.input_source_config)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutputSourceConfig", &self.output_source_config)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkflowName", &self.workflow_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for IdMappingWorkflowProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<IdMappingWorkflowProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = IdMappingWorkflowProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type IdMappingWorkflowProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut id_mapping_techniques: Option<::Value<self::id_mapping_workflow::IdMappingTechniques>> = None;
                let mut input_source_config: Option<::ValueList<self::id_mapping_workflow::IdMappingWorkflowInputSource>> = None;
                let mut output_source_config: Option<::ValueList<self::id_mapping_workflow::IdMappingWorkflowOutputSource>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut workflow_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IdMappingTechniques" => {
                            id_mapping_techniques = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InputSourceConfig" => {
                            input_source_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OutputSourceConfig" => {
                            output_source_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "WorkflowName" => {
                            workflow_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(IdMappingWorkflowProperties {
                    description: description,
                    id_mapping_techniques: id_mapping_techniques.ok_or(::serde::de::Error::missing_field("IdMappingTechniques"))?,
                    input_source_config: input_source_config.ok_or(::serde::de::Error::missing_field("InputSourceConfig"))?,
                    output_source_config: output_source_config.ok_or(::serde::de::Error::missing_field("OutputSourceConfig"))?,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    tags: tags,
                    workflow_name: workflow_name.ok_or(::serde::de::Error::missing_field("WorkflowName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for IdMappingWorkflow {
    type Properties = IdMappingWorkflowProperties;
    const TYPE: &'static str = "AWS::EntityResolution::IdMappingWorkflow";
    fn properties(&self) -> &IdMappingWorkflowProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut IdMappingWorkflowProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for IdMappingWorkflow {}

impl From<IdMappingWorkflowProperties> for IdMappingWorkflow {
    fn from(properties: IdMappingWorkflowProperties) -> IdMappingWorkflow {
        IdMappingWorkflow { properties }
    }
}

/// The [`AWS::EntityResolution::MatchingWorkflow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-entityresolution-matchingworkflow.html) resource type.
#[derive(Debug, Default)]
pub struct MatchingWorkflow {
    properties: MatchingWorkflowProperties
}

/// Properties for the `MatchingWorkflow` resource.
#[derive(Debug, Default)]
pub struct MatchingWorkflowProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-entityresolution-matchingworkflow.html#cfn-entityresolution-matchingworkflow-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`InputSourceConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-entityresolution-matchingworkflow.html#cfn-entityresolution-matchingworkflow-inputsourceconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub input_source_config: ::ValueList<self::matching_workflow::InputSource>,
    /// Property [`OutputSourceConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-entityresolution-matchingworkflow.html#cfn-entityresolution-matchingworkflow-outputsourceconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub output_source_config: ::ValueList<self::matching_workflow::OutputSource>,
    /// Property [`ResolutionTechniques`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-entityresolution-matchingworkflow.html#cfn-entityresolution-matchingworkflow-resolutiontechniques).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub resolution_techniques: ::Value<self::matching_workflow::ResolutionTechniques>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-entityresolution-matchingworkflow.html#cfn-entityresolution-matchingworkflow-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-entityresolution-matchingworkflow.html#cfn-entityresolution-matchingworkflow-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`WorkflowName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-entityresolution-matchingworkflow.html#cfn-entityresolution-matchingworkflow-workflowname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub workflow_name: ::Value<String>,
}

impl ::serde::Serialize for MatchingWorkflowProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputSourceConfig", &self.input_source_config)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutputSourceConfig", &self.output_source_config)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResolutionTechniques", &self.resolution_techniques)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkflowName", &self.workflow_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for MatchingWorkflowProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<MatchingWorkflowProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = MatchingWorkflowProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type MatchingWorkflowProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut input_source_config: Option<::ValueList<self::matching_workflow::InputSource>> = None;
                let mut output_source_config: Option<::ValueList<self::matching_workflow::OutputSource>> = None;
                let mut resolution_techniques: Option<::Value<self::matching_workflow::ResolutionTechniques>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut workflow_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InputSourceConfig" => {
                            input_source_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OutputSourceConfig" => {
                            output_source_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResolutionTechniques" => {
                            resolution_techniques = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "WorkflowName" => {
                            workflow_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(MatchingWorkflowProperties {
                    description: description,
                    input_source_config: input_source_config.ok_or(::serde::de::Error::missing_field("InputSourceConfig"))?,
                    output_source_config: output_source_config.ok_or(::serde::de::Error::missing_field("OutputSourceConfig"))?,
                    resolution_techniques: resolution_techniques.ok_or(::serde::de::Error::missing_field("ResolutionTechniques"))?,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    tags: tags,
                    workflow_name: workflow_name.ok_or(::serde::de::Error::missing_field("WorkflowName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for MatchingWorkflow {
    type Properties = MatchingWorkflowProperties;
    const TYPE: &'static str = "AWS::EntityResolution::MatchingWorkflow";
    fn properties(&self) -> &MatchingWorkflowProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut MatchingWorkflowProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for MatchingWorkflow {}

impl From<MatchingWorkflowProperties> for MatchingWorkflow {
    fn from(properties: MatchingWorkflowProperties) -> MatchingWorkflow {
        MatchingWorkflow { properties }
    }
}

/// The [`AWS::EntityResolution::SchemaMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-entityresolution-schemamapping.html) resource type.
#[derive(Debug, Default)]
pub struct SchemaMapping {
    properties: SchemaMappingProperties
}

/// Properties for the `SchemaMapping` resource.
#[derive(Debug, Default)]
pub struct SchemaMappingProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-entityresolution-schemamapping.html#cfn-entityresolution-schemamapping-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`MappedInputFields`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-entityresolution-schemamapping.html#cfn-entityresolution-schemamapping-mappedinputfields).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub mapped_input_fields: ::ValueList<self::schema_mapping::SchemaInputAttribute>,
    /// Property [`SchemaName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-entityresolution-schemamapping.html#cfn-entityresolution-schemamapping-schemaname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub schema_name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-entityresolution-schemamapping.html#cfn-entityresolution-schemamapping-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for SchemaMappingProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MappedInputFields", &self.mapped_input_fields)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SchemaName", &self.schema_name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SchemaMappingProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SchemaMappingProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SchemaMappingProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SchemaMappingProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut mapped_input_fields: Option<::ValueList<self::schema_mapping::SchemaInputAttribute>> = None;
                let mut schema_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MappedInputFields" => {
                            mapped_input_fields = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SchemaName" => {
                            schema_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SchemaMappingProperties {
                    description: description,
                    mapped_input_fields: mapped_input_fields.ok_or(::serde::de::Error::missing_field("MappedInputFields"))?,
                    schema_name: schema_name.ok_or(::serde::de::Error::missing_field("SchemaName"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for SchemaMapping {
    type Properties = SchemaMappingProperties;
    const TYPE: &'static str = "AWS::EntityResolution::SchemaMapping";
    fn properties(&self) -> &SchemaMappingProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SchemaMappingProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SchemaMapping {}

impl From<SchemaMappingProperties> for SchemaMapping {
    fn from(properties: SchemaMappingProperties) -> SchemaMapping {
        SchemaMapping { properties }
    }
}

pub mod id_mapping_workflow {
    //! Property types for the `IdMappingWorkflow` resource.

    /// The [`AWS::EntityResolution::IdMappingWorkflow.IdMappingTechniques`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-idmappingworkflow-idmappingtechniques.html) property type.
    #[derive(Debug, Default)]
    pub struct IdMappingTechniques {
        /// Property [`IdMappingType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-idmappingworkflow-idmappingtechniques.html#cfn-entityresolution-idmappingworkflow-idmappingtechniques-idmappingtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id_mapping_type: Option<::Value<String>>,
        /// Property [`ProviderProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-idmappingworkflow-idmappingtechniques.html#cfn-entityresolution-idmappingworkflow-idmappingtechniques-providerproperties).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub provider_properties: Option<::Value<ProviderProperties>>,
    }

    impl ::codec::SerializeValue for IdMappingTechniques {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref id_mapping_type) = self.id_mapping_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdMappingType", id_mapping_type)?;
            }
            if let Some(ref provider_properties) = self.provider_properties {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProviderProperties", provider_properties)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IdMappingTechniques {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IdMappingTechniques, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IdMappingTechniques;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IdMappingTechniques")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut id_mapping_type: Option<::Value<String>> = None;
                    let mut provider_properties: Option<::Value<ProviderProperties>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "IdMappingType" => {
                                id_mapping_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProviderProperties" => {
                                provider_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IdMappingTechniques {
                        id_mapping_type: id_mapping_type,
                        provider_properties: provider_properties,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EntityResolution::IdMappingWorkflow.IdMappingWorkflowInputSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-idmappingworkflow-idmappingworkflowinputsource.html) property type.
    #[derive(Debug, Default)]
    pub struct IdMappingWorkflowInputSource {
        /// Property [`InputSourceARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-idmappingworkflow-idmappingworkflowinputsource.html#cfn-entityresolution-idmappingworkflow-idmappingworkflowinputsource-inputsourcearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_source_arn: ::Value<String>,
        /// Property [`SchemaArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-idmappingworkflow-idmappingworkflowinputsource.html#cfn-entityresolution-idmappingworkflow-idmappingworkflowinputsource-schemaarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schema_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for IdMappingWorkflowInputSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputSourceARN", &self.input_source_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SchemaArn", &self.schema_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IdMappingWorkflowInputSource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IdMappingWorkflowInputSource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IdMappingWorkflowInputSource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IdMappingWorkflowInputSource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut input_source_arn: Option<::Value<String>> = None;
                    let mut schema_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InputSourceARN" => {
                                input_source_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SchemaArn" => {
                                schema_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IdMappingWorkflowInputSource {
                        input_source_arn: input_source_arn.ok_or(::serde::de::Error::missing_field("InputSourceARN"))?,
                        schema_arn: schema_arn.ok_or(::serde::de::Error::missing_field("SchemaArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EntityResolution::IdMappingWorkflow.IdMappingWorkflowOutputSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-idmappingworkflow-idmappingworkflowoutputsource.html) property type.
    #[derive(Debug, Default)]
    pub struct IdMappingWorkflowOutputSource {
        /// Property [`KMSArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-idmappingworkflow-idmappingworkflowoutputsource.html#cfn-entityresolution-idmappingworkflow-idmappingworkflowoutputsource-kmsarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kms_arn: Option<::Value<String>>,
        /// Property [`OutputS3Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-idmappingworkflow-idmappingworkflowoutputsource.html#cfn-entityresolution-idmappingworkflow-idmappingworkflowoutputsource-outputs3path).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub output_s3_path: ::Value<String>,
    }

    impl ::codec::SerializeValue for IdMappingWorkflowOutputSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref kms_arn) = self.kms_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KMSArn", kms_arn)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutputS3Path", &self.output_s3_path)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IdMappingWorkflowOutputSource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IdMappingWorkflowOutputSource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IdMappingWorkflowOutputSource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IdMappingWorkflowOutputSource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut kms_arn: Option<::Value<String>> = None;
                    let mut output_s3_path: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KMSArn" => {
                                kms_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OutputS3Path" => {
                                output_s3_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IdMappingWorkflowOutputSource {
                        kms_arn: kms_arn,
                        output_s3_path: output_s3_path.ok_or(::serde::de::Error::missing_field("OutputS3Path"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EntityResolution::IdMappingWorkflow.IntermediateSourceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-idmappingworkflow-intermediatesourceconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct IntermediateSourceConfiguration {
        /// Property [`IntermediateS3Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-idmappingworkflow-intermediatesourceconfiguration.html#cfn-entityresolution-idmappingworkflow-intermediatesourceconfiguration-intermediates3path).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub intermediate_s3_path: ::Value<String>,
    }

    impl ::codec::SerializeValue for IntermediateSourceConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntermediateS3Path", &self.intermediate_s3_path)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IntermediateSourceConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IntermediateSourceConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IntermediateSourceConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IntermediateSourceConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut intermediate_s3_path: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "IntermediateS3Path" => {
                                intermediate_s3_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IntermediateSourceConfiguration {
                        intermediate_s3_path: intermediate_s3_path.ok_or(::serde::de::Error::missing_field("IntermediateS3Path"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EntityResolution::IdMappingWorkflow.ProviderProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-idmappingworkflow-providerproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct ProviderProperties {
        /// Property [`IntermediateSourceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-idmappingworkflow-providerproperties.html#cfn-entityresolution-idmappingworkflow-providerproperties-intermediatesourceconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub intermediate_source_configuration: Option<::Value<IntermediateSourceConfiguration>>,
        /// Property [`ProviderConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-idmappingworkflow-providerproperties.html#cfn-entityresolution-idmappingworkflow-providerproperties-providerconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub provider_configuration: Option<::ValueMap<String>>,
        /// Property [`ProviderServiceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-idmappingworkflow-providerproperties.html#cfn-entityresolution-idmappingworkflow-providerproperties-providerservicearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub provider_service_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for ProviderProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref intermediate_source_configuration) = self.intermediate_source_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntermediateSourceConfiguration", intermediate_source_configuration)?;
            }
            if let Some(ref provider_configuration) = self.provider_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProviderConfiguration", provider_configuration)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProviderServiceArn", &self.provider_service_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ProviderProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ProviderProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ProviderProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ProviderProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut intermediate_source_configuration: Option<::Value<IntermediateSourceConfiguration>> = None;
                    let mut provider_configuration: Option<::ValueMap<String>> = None;
                    let mut provider_service_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "IntermediateSourceConfiguration" => {
                                intermediate_source_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProviderConfiguration" => {
                                provider_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProviderServiceArn" => {
                                provider_service_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ProviderProperties {
                        intermediate_source_configuration: intermediate_source_configuration,
                        provider_configuration: provider_configuration,
                        provider_service_arn: provider_service_arn.ok_or(::serde::de::Error::missing_field("ProviderServiceArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod matching_workflow {
    //! Property types for the `MatchingWorkflow` resource.

    /// The [`AWS::EntityResolution::MatchingWorkflow.InputSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-matchingworkflow-inputsource.html) property type.
    #[derive(Debug, Default)]
    pub struct InputSource {
        /// Property [`ApplyNormalization`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-matchingworkflow-inputsource.html#cfn-entityresolution-matchingworkflow-inputsource-applynormalization).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub apply_normalization: Option<::Value<bool>>,
        /// Property [`InputSourceARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-matchingworkflow-inputsource.html#cfn-entityresolution-matchingworkflow-inputsource-inputsourcearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_source_arn: ::Value<String>,
        /// Property [`SchemaArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-matchingworkflow-inputsource.html#cfn-entityresolution-matchingworkflow-inputsource-schemaarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schema_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for InputSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref apply_normalization) = self.apply_normalization {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplyNormalization", apply_normalization)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputSourceARN", &self.input_source_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SchemaArn", &self.schema_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InputSource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InputSource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InputSource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InputSource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut apply_normalization: Option<::Value<bool>> = None;
                    let mut input_source_arn: Option<::Value<String>> = None;
                    let mut schema_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ApplyNormalization" => {
                                apply_normalization = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InputSourceARN" => {
                                input_source_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SchemaArn" => {
                                schema_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InputSource {
                        apply_normalization: apply_normalization,
                        input_source_arn: input_source_arn.ok_or(::serde::de::Error::missing_field("InputSourceARN"))?,
                        schema_arn: schema_arn.ok_or(::serde::de::Error::missing_field("SchemaArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EntityResolution::MatchingWorkflow.IntermediateSourceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-matchingworkflow-intermediatesourceconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct IntermediateSourceConfiguration {
        /// Property [`IntermediateS3Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-matchingworkflow-intermediatesourceconfiguration.html#cfn-entityresolution-matchingworkflow-intermediatesourceconfiguration-intermediates3path).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub intermediate_s3_path: ::Value<String>,
    }

    impl ::codec::SerializeValue for IntermediateSourceConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntermediateS3Path", &self.intermediate_s3_path)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IntermediateSourceConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IntermediateSourceConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IntermediateSourceConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IntermediateSourceConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut intermediate_s3_path: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "IntermediateS3Path" => {
                                intermediate_s3_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IntermediateSourceConfiguration {
                        intermediate_s3_path: intermediate_s3_path.ok_or(::serde::de::Error::missing_field("IntermediateS3Path"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EntityResolution::MatchingWorkflow.OutputAttribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-matchingworkflow-outputattribute.html) property type.
    #[derive(Debug, Default)]
    pub struct OutputAttribute {
        /// Property [`Hashed`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-matchingworkflow-outputattribute.html#cfn-entityresolution-matchingworkflow-outputattribute-hashed).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hashed: Option<::Value<bool>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-matchingworkflow-outputattribute.html#cfn-entityresolution-matchingworkflow-outputattribute-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for OutputAttribute {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref hashed) = self.hashed {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Hashed", hashed)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OutputAttribute {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OutputAttribute, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OutputAttribute;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OutputAttribute")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut hashed: Option<::Value<bool>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Hashed" => {
                                hashed = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OutputAttribute {
                        hashed: hashed,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EntityResolution::MatchingWorkflow.OutputSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-matchingworkflow-outputsource.html) property type.
    #[derive(Debug, Default)]
    pub struct OutputSource {
        /// Property [`ApplyNormalization`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-matchingworkflow-outputsource.html#cfn-entityresolution-matchingworkflow-outputsource-applynormalization).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub apply_normalization: Option<::Value<bool>>,
        /// Property [`KMSArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-matchingworkflow-outputsource.html#cfn-entityresolution-matchingworkflow-outputsource-kmsarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kms_arn: Option<::Value<String>>,
        /// Property [`Output`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-matchingworkflow-outputsource.html#cfn-entityresolution-matchingworkflow-outputsource-output).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub output: ::ValueList<OutputAttribute>,
        /// Property [`OutputS3Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-matchingworkflow-outputsource.html#cfn-entityresolution-matchingworkflow-outputsource-outputs3path).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub output_s3_path: ::Value<String>,
    }

    impl ::codec::SerializeValue for OutputSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref apply_normalization) = self.apply_normalization {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplyNormalization", apply_normalization)?;
            }
            if let Some(ref kms_arn) = self.kms_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KMSArn", kms_arn)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Output", &self.output)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutputS3Path", &self.output_s3_path)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OutputSource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OutputSource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OutputSource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OutputSource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut apply_normalization: Option<::Value<bool>> = None;
                    let mut kms_arn: Option<::Value<String>> = None;
                    let mut output: Option<::ValueList<OutputAttribute>> = None;
                    let mut output_s3_path: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ApplyNormalization" => {
                                apply_normalization = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KMSArn" => {
                                kms_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Output" => {
                                output = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OutputS3Path" => {
                                output_s3_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OutputSource {
                        apply_normalization: apply_normalization,
                        kms_arn: kms_arn,
                        output: output.ok_or(::serde::de::Error::missing_field("Output"))?,
                        output_s3_path: output_s3_path.ok_or(::serde::de::Error::missing_field("OutputS3Path"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EntityResolution::MatchingWorkflow.ProviderProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-matchingworkflow-providerproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct ProviderProperties {
        /// Property [`IntermediateSourceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-matchingworkflow-providerproperties.html#cfn-entityresolution-matchingworkflow-providerproperties-intermediatesourceconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub intermediate_source_configuration: Option<::Value<IntermediateSourceConfiguration>>,
        /// Property [`ProviderConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-matchingworkflow-providerproperties.html#cfn-entityresolution-matchingworkflow-providerproperties-providerconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub provider_configuration: Option<::ValueMap<String>>,
        /// Property [`ProviderServiceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-matchingworkflow-providerproperties.html#cfn-entityresolution-matchingworkflow-providerproperties-providerservicearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub provider_service_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for ProviderProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref intermediate_source_configuration) = self.intermediate_source_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntermediateSourceConfiguration", intermediate_source_configuration)?;
            }
            if let Some(ref provider_configuration) = self.provider_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProviderConfiguration", provider_configuration)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProviderServiceArn", &self.provider_service_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ProviderProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ProviderProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ProviderProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ProviderProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut intermediate_source_configuration: Option<::Value<IntermediateSourceConfiguration>> = None;
                    let mut provider_configuration: Option<::ValueMap<String>> = None;
                    let mut provider_service_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "IntermediateSourceConfiguration" => {
                                intermediate_source_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProviderConfiguration" => {
                                provider_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProviderServiceArn" => {
                                provider_service_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ProviderProperties {
                        intermediate_source_configuration: intermediate_source_configuration,
                        provider_configuration: provider_configuration,
                        provider_service_arn: provider_service_arn.ok_or(::serde::de::Error::missing_field("ProviderServiceArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EntityResolution::MatchingWorkflow.ResolutionTechniques`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-matchingworkflow-resolutiontechniques.html) property type.
    #[derive(Debug, Default)]
    pub struct ResolutionTechniques {
        /// Property [`ProviderProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-matchingworkflow-resolutiontechniques.html#cfn-entityresolution-matchingworkflow-resolutiontechniques-providerproperties).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub provider_properties: Option<::Value<ProviderProperties>>,
        /// Property [`ResolutionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-matchingworkflow-resolutiontechniques.html#cfn-entityresolution-matchingworkflow-resolutiontechniques-resolutiontype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resolution_type: Option<::Value<String>>,
        /// Property [`RuleBasedProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-matchingworkflow-resolutiontechniques.html#cfn-entityresolution-matchingworkflow-resolutiontechniques-rulebasedproperties).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rule_based_properties: Option<::Value<RuleBasedProperties>>,
    }

    impl ::codec::SerializeValue for ResolutionTechniques {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref provider_properties) = self.provider_properties {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProviderProperties", provider_properties)?;
            }
            if let Some(ref resolution_type) = self.resolution_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResolutionType", resolution_type)?;
            }
            if let Some(ref rule_based_properties) = self.rule_based_properties {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleBasedProperties", rule_based_properties)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResolutionTechniques {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResolutionTechniques, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResolutionTechniques;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResolutionTechniques")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut provider_properties: Option<::Value<ProviderProperties>> = None;
                    let mut resolution_type: Option<::Value<String>> = None;
                    let mut rule_based_properties: Option<::Value<RuleBasedProperties>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ProviderProperties" => {
                                provider_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResolutionType" => {
                                resolution_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RuleBasedProperties" => {
                                rule_based_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResolutionTechniques {
                        provider_properties: provider_properties,
                        resolution_type: resolution_type,
                        rule_based_properties: rule_based_properties,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EntityResolution::MatchingWorkflow.Rule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-matchingworkflow-rule.html) property type.
    #[derive(Debug, Default)]
    pub struct Rule {
        /// Property [`MatchingKeys`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-matchingworkflow-rule.html#cfn-entityresolution-matchingworkflow-rule-matchingkeys).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub matching_keys: ::ValueList<String>,
        /// Property [`RuleName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-matchingworkflow-rule.html#cfn-entityresolution-matchingworkflow-rule-rulename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rule_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for Rule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MatchingKeys", &self.matching_keys)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleName", &self.rule_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Rule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Rule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Rule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Rule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut matching_keys: Option<::ValueList<String>> = None;
                    let mut rule_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MatchingKeys" => {
                                matching_keys = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RuleName" => {
                                rule_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Rule {
                        matching_keys: matching_keys.ok_or(::serde::de::Error::missing_field("MatchingKeys"))?,
                        rule_name: rule_name.ok_or(::serde::de::Error::missing_field("RuleName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EntityResolution::MatchingWorkflow.RuleBasedProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-matchingworkflow-rulebasedproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct RuleBasedProperties {
        /// Property [`AttributeMatchingModel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-matchingworkflow-rulebasedproperties.html#cfn-entityresolution-matchingworkflow-rulebasedproperties-attributematchingmodel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attribute_matching_model: ::Value<String>,
        /// Property [`Rules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-matchingworkflow-rulebasedproperties.html#cfn-entityresolution-matchingworkflow-rulebasedproperties-rules).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rules: ::ValueList<Rule>,
    }

    impl ::codec::SerializeValue for RuleBasedProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttributeMatchingModel", &self.attribute_matching_model)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Rules", &self.rules)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RuleBasedProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RuleBasedProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RuleBasedProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RuleBasedProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attribute_matching_model: Option<::Value<String>> = None;
                    let mut rules: Option<::ValueList<Rule>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AttributeMatchingModel" => {
                                attribute_matching_model = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Rules" => {
                                rules = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RuleBasedProperties {
                        attribute_matching_model: attribute_matching_model.ok_or(::serde::de::Error::missing_field("AttributeMatchingModel"))?,
                        rules: rules.ok_or(::serde::de::Error::missing_field("Rules"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod schema_mapping {
    //! Property types for the `SchemaMapping` resource.

    /// The [`AWS::EntityResolution::SchemaMapping.SchemaInputAttribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-schemamapping-schemainputattribute.html) property type.
    #[derive(Debug, Default)]
    pub struct SchemaInputAttribute {
        /// Property [`FieldName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-schemamapping-schemainputattribute.html#cfn-entityresolution-schemamapping-schemainputattribute-fieldname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub field_name: ::Value<String>,
        /// Property [`GroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-schemamapping-schemainputattribute.html#cfn-entityresolution-schemamapping-schemainputattribute-groupname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub group_name: Option<::Value<String>>,
        /// Property [`MatchKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-schemamapping-schemainputattribute.html#cfn-entityresolution-schemamapping-schemainputattribute-matchkey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub match_key: Option<::Value<String>>,
        /// Property [`SubType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-schemamapping-schemainputattribute.html#cfn-entityresolution-schemamapping-schemainputattribute-subtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sub_type: Option<::Value<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-schemamapping-schemainputattribute.html#cfn-entityresolution-schemamapping-schemainputattribute-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for SchemaInputAttribute {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FieldName", &self.field_name)?;
            if let Some(ref group_name) = self.group_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupName", group_name)?;
            }
            if let Some(ref match_key) = self.match_key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MatchKey", match_key)?;
            }
            if let Some(ref sub_type) = self.sub_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubType", sub_type)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SchemaInputAttribute {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SchemaInputAttribute, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SchemaInputAttribute;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SchemaInputAttribute")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut field_name: Option<::Value<String>> = None;
                    let mut group_name: Option<::Value<String>> = None;
                    let mut match_key: Option<::Value<String>> = None;
                    let mut sub_type: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FieldName" => {
                                field_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GroupName" => {
                                group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MatchKey" => {
                                match_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubType" => {
                                sub_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SchemaInputAttribute {
                        field_name: field_name.ok_or(::serde::de::Error::missing_field("FieldName"))?,
                        group_name: group_name,
                        match_key: match_key,
                        sub_type: sub_type,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
