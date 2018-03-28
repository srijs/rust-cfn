/// The [`AWS::DataPipeline::Pipeline`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datapipeline-pipeline.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Pipeline {
    properties: PipelineProperties
}

/// Properties for the `Pipeline` resource.
#[derive(Serialize, Deserialize)]
pub struct PipelineProperties {
    #[serde(rename="Activate")]
    pub activate: (),
    #[serde(rename="Description")]
    pub description: (),
    #[serde(rename="Name")]
    pub name: (),
    #[serde(rename="ParameterObjects")]
    pub parameter_objects: (),
    #[serde(rename="ParameterValues")]
    pub parameter_values: (),
    #[serde(rename="PipelineObjects")]
    pub pipeline_objects: (),
    #[serde(rename="PipelineTags")]
    pub pipeline_tags: (),
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

