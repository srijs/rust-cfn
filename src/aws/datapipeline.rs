//! Types for the `DataPipeline` service.

/// The [`AWS::DataPipeline::Pipeline`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datapipeline-pipeline.html) resource type.
#[derive(Debug)]
pub struct Pipeline {
    properties: PipelineProperties
}

/// Properties for the `Pipeline` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct PipelineProperties {
    /// Property `Activate`.
    #[serde(rename = "Activate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activate: Option<::Value<bool>>,
    /// Property `Description`.
    #[serde(rename = "Description")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<::Value<String>>,
    /// Property `Name`.
    #[serde(rename = "Name")]
    pub name: ::Value<String>,
    /// Property `ParameterObjects`.
    #[serde(rename = "ParameterObjects")]
    pub parameter_objects: ::ValueList<self::pipeline::ParameterObject>,
    /// Property `ParameterValues`.
    #[serde(rename = "ParameterValues")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameter_values: Option<::ValueList<self::pipeline::ParameterValue>>,
    /// Property `PipelineObjects`.
    #[serde(rename = "PipelineObjects")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pipeline_objects: Option<::ValueList<self::pipeline::PipelineObject>>,
    /// Property `PipelineTags`.
    #[serde(rename = "PipelineTags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pipeline_tags: Option<::ValueList<self::pipeline::PipelineTag>>,
}

impl<'a> ::Resource<'a> for Pipeline {
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
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Field {
        /// Property `Key`.
        #[serde(rename = "Key")]
        pub key: ::Value<String>,
        /// Property `RefValue`.
        #[serde(rename = "RefValue")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ref_value: Option<::Value<String>>,
        /// Property `StringValue`.
        #[serde(rename = "StringValue")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub string_value: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(Field);

    /// The [`AWS::DataPipeline::Pipeline.ParameterAttribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datapipeline-pipeline-parameterobjects-attributes.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ParameterAttribute {
        /// Property `Key`.
        #[serde(rename = "Key")]
        pub key: ::Value<String>,
        /// Property `StringValue`.
        #[serde(rename = "StringValue")]
        pub string_value: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(ParameterAttribute);

    /// The [`AWS::DataPipeline::Pipeline.ParameterObject`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datapipeline-pipeline-parameterobjects.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ParameterObject {
        /// Property `Attributes`.
        #[serde(rename = "Attributes")]
        pub attributes: ::ValueList<ParameterAttribute>,
        /// Property `Id`.
        #[serde(rename = "Id")]
        pub id: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(ParameterObject);

    /// The [`AWS::DataPipeline::Pipeline.ParameterValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datapipeline-pipeline-parametervalues.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ParameterValue {
        /// Property `Id`.
        #[serde(rename = "Id")]
        pub id: ::Value<String>,
        /// Property `StringValue`.
        #[serde(rename = "StringValue")]
        pub string_value: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(ParameterValue);

    /// The [`AWS::DataPipeline::Pipeline.PipelineObject`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datapipeline-pipeline-pipelineobjects.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PipelineObject {
        /// Property `Fields`.
        #[serde(rename = "Fields")]
        pub fields: ::ValueList<Field>,
        /// Property `Id`.
        #[serde(rename = "Id")]
        pub id: ::Value<String>,
        /// Property `Name`.
        #[serde(rename = "Name")]
        pub name: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(PipelineObject);

    /// The [`AWS::DataPipeline::Pipeline.PipelineTag`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datapipeline-pipeline-pipelinetags.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PipelineTag {
        /// Property `Key`.
        #[serde(rename = "Key")]
        pub key: ::Value<String>,
        /// Property `Value`.
        #[serde(rename = "Value")]
        pub value: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(PipelineTag);
}
