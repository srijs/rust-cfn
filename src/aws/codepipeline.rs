//! Types for the `CodePipeline` service.

/// The [`AWS::CodePipeline::CustomActionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codepipeline-customactiontype.html) resource type.
#[derive(Debug)]
pub struct CustomActionType {
    properties: CustomActionTypeProperties
}

/// Properties for the `CustomActionType` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomActionTypeProperties {
    /// Property `Category`.
    #[serde(rename = "Category")]
    pub category: ::Value<String>,
    /// Property `ConfigurationProperties`.
    #[serde(rename = "ConfigurationProperties")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration_properties: Option<::ValueList<self::custom_action_type::ConfigurationProperties>>,
    /// Property `InputArtifactDetails`.
    #[serde(rename = "InputArtifactDetails")]
    pub input_artifact_details: ::Value<self::custom_action_type::ArtifactDetails>,
    /// Property `OutputArtifactDetails`.
    #[serde(rename = "OutputArtifactDetails")]
    pub output_artifact_details: ::Value<self::custom_action_type::ArtifactDetails>,
    /// Property `Provider`.
    #[serde(rename = "Provider")]
    pub provider: ::Value<String>,
    /// Property `Settings`.
    #[serde(rename = "Settings")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<::Value<self::custom_action_type::Settings>>,
    /// Property `Version`.
    #[serde(rename = "Version")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<::Value<String>>,
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
#[derive(Debug)]
pub struct Pipeline {
    properties: PipelineProperties
}

/// Properties for the `Pipeline` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct PipelineProperties {
    /// Property `ArtifactStore`.
    #[serde(rename = "ArtifactStore")]
    pub artifact_store: ::Value<self::pipeline::ArtifactStore>,
    /// Property `DisableInboundStageTransitions`.
    #[serde(rename = "DisableInboundStageTransitions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable_inbound_stage_transitions: Option<::ValueList<self::pipeline::StageTransition>>,
    /// Property `Name`.
    #[serde(rename = "Name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<::Value<String>>,
    /// Property `RestartExecutionOnUpdate`.
    #[serde(rename = "RestartExecutionOnUpdate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restart_execution_on_update: Option<::Value<bool>>,
    /// Property `RoleArn`.
    #[serde(rename = "RoleArn")]
    pub role_arn: ::Value<String>,
    /// Property `Stages`.
    #[serde(rename = "Stages")]
    pub stages: ::ValueList<self::pipeline::StageDeclaration>,
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
    //! Property types for the `CustomActionType` resource.

    /// The [`AWS::CodePipeline::CustomActionType.ArtifactDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-customactiontype-artifactdetails.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ArtifactDetails {
        /// Property `MaximumCount`.
        #[serde(rename = "MaximumCount")]
        pub maximum_count: ::Value<u32>,
        /// Property `MinimumCount`.
        #[serde(rename = "MinimumCount")]
        pub minimum_count: ::Value<u32>,
    }

    cfn_internal__inherit_codec_impls!(ArtifactDetails);

    /// The [`AWS::CodePipeline::CustomActionType.ConfigurationProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-customactiontype-configurationproperties.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ConfigurationProperties {
        /// Property `Description`.
        #[serde(rename = "Description")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<::Value<String>>,
        /// Property `Key`.
        #[serde(rename = "Key")]
        pub key: ::Value<bool>,
        /// Property `Name`.
        #[serde(rename = "Name")]
        pub name: ::Value<String>,
        /// Property `Queryable`.
        #[serde(rename = "Queryable")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub queryable: Option<::Value<bool>>,
        /// Property `Required`.
        #[serde(rename = "Required")]
        pub required: ::Value<bool>,
        /// Property `Secret`.
        #[serde(rename = "Secret")]
        pub secret: ::Value<bool>,
        /// Property `Type`.
        #[serde(rename = "Type")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(ConfigurationProperties);

    /// The [`AWS::CodePipeline::CustomActionType.Settings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-customactiontype-settings.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Settings {
        /// Property `EntityUrlTemplate`.
        #[serde(rename = "EntityUrlTemplate")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub entity_url_template: Option<::Value<String>>,
        /// Property `ExecutionUrlTemplate`.
        #[serde(rename = "ExecutionUrlTemplate")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub execution_url_template: Option<::Value<String>>,
        /// Property `RevisionUrlTemplate`.
        #[serde(rename = "RevisionUrlTemplate")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub revision_url_template: Option<::Value<String>>,
        /// Property `ThirdPartyConfigurationUrl`.
        #[serde(rename = "ThirdPartyConfigurationUrl")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub third_party_configuration_url: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(Settings);
}

pub mod pipeline {
    //! Property types for the `Pipeline` resource.

    /// The [`AWS::CodePipeline::Pipeline.ActionDeclaration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-stages-actions.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ActionDeclaration {
        /// Property `ActionTypeId`.
        #[serde(rename = "ActionTypeId")]
        pub action_type_id: ::Value<ActionTypeId>,
        /// Property `Configuration`.
        #[serde(rename = "Configuration")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configuration: Option<::Value<::json::Value>>,
        /// Property `InputArtifacts`.
        #[serde(rename = "InputArtifacts")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub input_artifacts: Option<::ValueList<InputArtifact>>,
        /// Property `Name`.
        #[serde(rename = "Name")]
        pub name: ::Value<String>,
        /// Property `OutputArtifacts`.
        #[serde(rename = "OutputArtifacts")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub output_artifacts: Option<::ValueList<OutputArtifact>>,
        /// Property `RoleArn`.
        #[serde(rename = "RoleArn")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub role_arn: Option<::Value<String>>,
        /// Property `RunOrder`.
        #[serde(rename = "RunOrder")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub run_order: Option<::Value<u32>>,
    }

    cfn_internal__inherit_codec_impls!(ActionDeclaration);

    /// The [`AWS::CodePipeline::Pipeline.ActionTypeId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-stages-actions-actiontypeid.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ActionTypeId {
        /// Property `Category`.
        #[serde(rename = "Category")]
        pub category: ::Value<String>,
        /// Property `Owner`.
        #[serde(rename = "Owner")]
        pub owner: ::Value<String>,
        /// Property `Provider`.
        #[serde(rename = "Provider")]
        pub provider: ::Value<String>,
        /// Property `Version`.
        #[serde(rename = "Version")]
        pub version: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(ActionTypeId);

    /// The [`AWS::CodePipeline::Pipeline.ArtifactStore`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-artifactstore.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ArtifactStore {
        /// Property `EncryptionKey`.
        #[serde(rename = "EncryptionKey")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub encryption_key: Option<::Value<EncryptionKey>>,
        /// Property `Location`.
        #[serde(rename = "Location")]
        pub location: ::Value<String>,
        /// Property `Type`.
        #[serde(rename = "Type")]
        pub type_: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(ArtifactStore);

    /// The [`AWS::CodePipeline::Pipeline.BlockerDeclaration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-stages-blockers.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct BlockerDeclaration {
        /// Property `Name`.
        #[serde(rename = "Name")]
        pub name: ::Value<String>,
        /// Property `Type`.
        #[serde(rename = "Type")]
        pub type_: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(BlockerDeclaration);

    /// The [`AWS::CodePipeline::Pipeline.EncryptionKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-artifactstore-encryptionkey.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EncryptionKey {
        /// Property `Id`.
        #[serde(rename = "Id")]
        pub id: ::Value<String>,
        /// Property `Type`.
        #[serde(rename = "Type")]
        pub type_: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(EncryptionKey);

    /// The [`AWS::CodePipeline::Pipeline.InputArtifact`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-stages-actions-inputartifacts.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct InputArtifact {
        /// Property `Name`.
        #[serde(rename = "Name")]
        pub name: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(InputArtifact);

    /// The [`AWS::CodePipeline::Pipeline.OutputArtifact`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-stages-actions-outputartifacts.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct OutputArtifact {
        /// Property `Name`.
        #[serde(rename = "Name")]
        pub name: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(OutputArtifact);

    /// The [`AWS::CodePipeline::Pipeline.StageDeclaration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-stages.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StageDeclaration {
        /// Property `Actions`.
        #[serde(rename = "Actions")]
        pub actions: ::ValueList<ActionDeclaration>,
        /// Property `Blockers`.
        #[serde(rename = "Blockers")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub blockers: Option<::ValueList<BlockerDeclaration>>,
        /// Property `Name`.
        #[serde(rename = "Name")]
        pub name: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(StageDeclaration);

    /// The [`AWS::CodePipeline::Pipeline.StageTransition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codepipeline-pipeline-disableinboundstagetransitions.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StageTransition {
        /// Property `Reason`.
        #[serde(rename = "Reason")]
        pub reason: ::Value<String>,
        /// Property `StageName`.
        #[serde(rename = "StageName")]
        pub stage_name: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(StageTransition);
}
