/// The [`AWS::CodePipeline::CustomActionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codepipeline-customactiontype.html) resource type.
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

impl ::private::Sealed for CustomActionType {}

impl From<CustomActionTypeProperties> for CustomActionType {
    fn from(properties: CustomActionTypeProperties) -> CustomActionType {
        CustomActionType { properties }
    }
}

/// The [`AWS::CodePipeline::Pipeline`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codepipeline-pipeline.html) resource type.
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

impl ::private::Sealed for Pipeline {}

impl From<PipelineProperties> for Pipeline {
    fn from(properties: PipelineProperties) -> Pipeline {
        Pipeline { properties }
    }
}

pub mod custom_action_type {
    /// The [`AWS::CodePipeline::CustomActionType.ArtifactDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-customactiontype-artifactdetails.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct ArtifactDetails {
        #[serde(rename="MaximumCount")]
        pub maximum_count: u32,
        #[serde(rename="MinimumCount")]
        pub minimum_count: u32,
    }

    /// The [`AWS::CodePipeline::CustomActionType.ConfigurationProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-customactiontype-configurationproperties.html) property type.
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

    /// The [`AWS::CodePipeline::CustomActionType.Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-customactiontype-settings.html) property type.
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
    /// The [`AWS::CodePipeline::Pipeline.ActionDeclaration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-stages-actions.html) property type.
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

    /// The [`AWS::CodePipeline::Pipeline.ActionTypeId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-stages-actions-actiontypeid.html) property type.
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

    /// The [`AWS::CodePipeline::Pipeline.ArtifactStore`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-artifactstore.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct ArtifactStore {
        #[serde(rename="EncryptionKey")]
        pub encryption_key: EncryptionKey,
        #[serde(rename="Location")]
        pub location: String,
        #[serde(rename="Type")]
        pub type_: String,
    }

    /// The [`AWS::CodePipeline::Pipeline.BlockerDeclaration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-stages-blockers.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct BlockerDeclaration {
        #[serde(rename="Name")]
        pub name: String,
        #[serde(rename="Type")]
        pub type_: String,
    }

    /// The [`AWS::CodePipeline::Pipeline.EncryptionKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-artifactstore-encryptionkey.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct EncryptionKey {
        #[serde(rename="Id")]
        pub id: String,
        #[serde(rename="Type")]
        pub type_: String,
    }

    /// The [`AWS::CodePipeline::Pipeline.InputArtifact`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-stages-actions-inputartifacts.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct InputArtifact {
        #[serde(rename="Name")]
        pub name: String,
    }

    /// The [`AWS::CodePipeline::Pipeline.OutputArtifact`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-stages-actions-outputartifacts.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct OutputArtifact {
        #[serde(rename="Name")]
        pub name: String,
    }

    /// The [`AWS::CodePipeline::Pipeline.StageDeclaration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-stages.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct StageDeclaration {
        #[serde(rename="Actions")]
        pub actions: Vec<ActionDeclaration>,
        #[serde(rename="Blockers")]
        pub blockers: Vec<BlockerDeclaration>,
        #[serde(rename="Name")]
        pub name: String,
    }

    /// The [`AWS::CodePipeline::Pipeline.StageTransition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-disableinboundstagetransitions.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct StageTransition {
        #[serde(rename="Reason")]
        pub reason: String,
        #[serde(rename="StageName")]
        pub stage_name: String,
    }

}

