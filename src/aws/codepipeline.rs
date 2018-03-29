/// The [`AWS::CodePipeline::CustomActionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codepipeline-customactiontype.html) resource.
#[derive(Serialize, Deserialize)]
pub struct CustomActionType {
    properties: CustomActionTypeProperties
}

/// Properties for the `CustomActionType` resource.
#[derive(Serialize, Deserialize)]
pub struct CustomActionTypeProperties {
    #[serde(rename="Category")]
    pub category: String,
    #[serde(rename="ConfigurationProperties")]
    pub configuration_properties: Vec<self::custom_action_type::ConfigurationProperties>,
    #[serde(rename="InputArtifactDetails")]
    pub input_artifact_details: self::custom_action_type::ArtifactDetails,
    #[serde(rename="OutputArtifactDetails")]
    pub output_artifact_details: self::custom_action_type::ArtifactDetails,
    #[serde(rename="Provider")]
    pub provider: String,
    #[serde(rename="Settings")]
    pub settings: self::custom_action_type::Settings,
    #[serde(rename="Version")]
    pub version: String,
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
    pub artifact_store: self::pipeline::ArtifactStore,
    #[serde(rename="DisableInboundStageTransitions")]
    pub disable_inbound_stage_transitions: Vec<self::pipeline::StageTransition>,
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="RestartExecutionOnUpdate")]
    pub restart_execution_on_update: bool,
    #[serde(rename="RoleArn")]
    pub role_arn: String,
    #[serde(rename="Stages")]
    pub stages: Vec<self::pipeline::StageDeclaration>,
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

pub mod custom_action_type {
    #[derive(Serialize, Deserialize)]
    pub struct ArtifactDetails {
        #[serde(rename="MaximumCount")]
        pub maximum_count: u32,
        #[serde(rename="MinimumCount")]
        pub minimum_count: u32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ConfigurationProperties {
        #[serde(rename="Description")]
        pub description: String,
        #[serde(rename="Key")]
        pub key: bool,
        #[serde(rename="Name")]
        pub name: String,
        #[serde(rename="Queryable")]
        pub queryable: bool,
        #[serde(rename="Required")]
        pub required: bool,
        #[serde(rename="Secret")]
        pub secret: bool,
        #[serde(rename="Type")]
        pub type_: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Settings {
        #[serde(rename="EntityUrlTemplate")]
        pub entity_url_template: String,
        #[serde(rename="ExecutionUrlTemplate")]
        pub execution_url_template: String,
        #[serde(rename="RevisionUrlTemplate")]
        pub revision_url_template: String,
        #[serde(rename="ThirdPartyConfigurationUrl")]
        pub third_party_configuration_url: String,
    }

}

pub mod pipeline {
    #[derive(Serialize, Deserialize)]
    pub struct ActionDeclaration {
        #[serde(rename="ActionTypeId")]
        pub action_type_id: ActionTypeId,
        #[serde(rename="Configuration")]
        pub configuration: ::json::Value,
        #[serde(rename="InputArtifacts")]
        pub input_artifacts: Vec<InputArtifact>,
        #[serde(rename="Name")]
        pub name: String,
        #[serde(rename="OutputArtifacts")]
        pub output_artifacts: Vec<OutputArtifact>,
        #[serde(rename="RoleArn")]
        pub role_arn: String,
        #[serde(rename="RunOrder")]
        pub run_order: u32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ActionTypeId {
        #[serde(rename="Category")]
        pub category: String,
        #[serde(rename="Owner")]
        pub owner: String,
        #[serde(rename="Provider")]
        pub provider: String,
        #[serde(rename="Version")]
        pub version: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ArtifactStore {
        #[serde(rename="EncryptionKey")]
        pub encryption_key: EncryptionKey,
        #[serde(rename="Location")]
        pub location: String,
        #[serde(rename="Type")]
        pub type_: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct BlockerDeclaration {
        #[serde(rename="Name")]
        pub name: String,
        #[serde(rename="Type")]
        pub type_: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct EncryptionKey {
        #[serde(rename="Id")]
        pub id: String,
        #[serde(rename="Type")]
        pub type_: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct InputArtifact {
        #[serde(rename="Name")]
        pub name: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct OutputArtifact {
        #[serde(rename="Name")]
        pub name: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct StageDeclaration {
        #[serde(rename="Actions")]
        pub actions: Vec<ActionDeclaration>,
        #[serde(rename="Blockers")]
        pub blockers: Vec<BlockerDeclaration>,
        #[serde(rename="Name")]
        pub name: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct StageTransition {
        #[serde(rename="Reason")]
        pub reason: String,
        #[serde(rename="StageName")]
        pub stage_name: String,
    }

}

