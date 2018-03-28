/// The [`AWS::CodePipeline::CustomActionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codepipeline-customactiontype.html) resource.
#[derive(Serialize, Deserialize)]
pub struct CustomActionType {
    properties: CustomActionTypeProperties
}

/// Properties for the `CustomActionType` resource.
#[derive(Serialize, Deserialize)]
pub struct CustomActionTypeProperties {
    #[serde(rename="Category")]
    pub category: (),
    #[serde(rename="ConfigurationProperties")]
    pub configuration_properties: (),
    #[serde(rename="InputArtifactDetails")]
    pub input_artifact_details: (),
    #[serde(rename="OutputArtifactDetails")]
    pub output_artifact_details: (),
    #[serde(rename="Provider")]
    pub provider: (),
    #[serde(rename="Settings")]
    pub settings: (),
    #[serde(rename="Version")]
    pub version: (),
}

impl<'a> ::Resource<'a> for CustomActionType {
    type Properties = CustomActionTypeProperties;
    const TYPE: &'static str = "AWS::CodePipeline::CustomActionType";
    fn properties(&self) -> &CustomActionTypeProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CustomActionTypeProperties {
        &mut self.properties
    }
}

impl From<CustomActionTypeProperties> for CustomActionType {
    fn from(properties: CustomActionTypeProperties) -> CustomActionType {
        CustomActionType { properties }
    }
}

/// The [`AWS::CodePipeline::Pipeline`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codepipeline-pipeline.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Pipeline {
    properties: PipelineProperties
}

/// Properties for the `Pipeline` resource.
#[derive(Serialize, Deserialize)]
pub struct PipelineProperties {
    #[serde(rename="ArtifactStore")]
    pub artifact_store: (),
    #[serde(rename="DisableInboundStageTransitions")]
    pub disable_inbound_stage_transitions: (),
    #[serde(rename="Name")]
    pub name: (),
    #[serde(rename="RestartExecutionOnUpdate")]
    pub restart_execution_on_update: (),
    #[serde(rename="RoleArn")]
    pub role_arn: (),
    #[serde(rename="Stages")]
    pub stages: (),
}

impl<'a> ::Resource<'a> for Pipeline {
    type Properties = PipelineProperties;
    const TYPE: &'static str = "AWS::CodePipeline::Pipeline";
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

