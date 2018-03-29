/// The [`AWS::DataPipeline::Pipeline`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datapipeline-pipeline.html) resource.
pub struct Pipeline {
    properties: PipelineProperties
}

/// Properties for the `Pipeline` resource.
#[derive(Serialize, Deserialize)]
pub struct PipelineProperties {
    #[serde(rename="Activate")]
    pub activate: bool,
    #[serde(rename="Description")]
    pub description: String,
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="ParameterObjects")]
    pub parameter_objects: Vec<self::pipeline::ParameterObject>,
    #[serde(rename="ParameterValues")]
    pub parameter_values: Vec<self::pipeline::ParameterValue>,
    #[serde(rename="PipelineObjects")]
    pub pipeline_objects: Vec<self::pipeline::PipelineObject>,
    #[serde(rename="PipelineTags")]
    pub pipeline_tags: Vec<self::pipeline::PipelineTag>,
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

impl From<PipelineProperties> for Pipeline {
    fn from(properties: PipelineProperties) -> Pipeline {
        Pipeline { properties }
    }
}

pub mod pipeline {
    #[derive(Serialize, Deserialize)]
    pub struct Field {
        #[serde(rename="Key")]
        pub key: String,
        #[serde(rename="RefValue")]
        pub ref_value: String,
        #[serde(rename="StringValue")]
        pub string_value: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ParameterAttribute {
        #[serde(rename="Key")]
        pub key: String,
        #[serde(rename="StringValue")]
        pub string_value: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ParameterObject {
        #[serde(rename="Attributes")]
        pub attributes: Vec<ParameterAttribute>,
        #[serde(rename="Id")]
        pub id: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ParameterValue {
        #[serde(rename="Id")]
        pub id: String,
        #[serde(rename="StringValue")]
        pub string_value: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct PipelineObject {
        #[serde(rename="Fields")]
        pub fields: Vec<Field>,
        #[serde(rename="Id")]
        pub id: String,
        #[serde(rename="Name")]
        pub name: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct PipelineTag {
        #[serde(rename="Key")]
        pub key: String,
        #[serde(rename="Value")]
        pub value: String,
    }

}

