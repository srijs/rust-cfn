//! Types for the `CodeBuild` service.

/// The [`AWS::CodeBuild::Project`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-project.html) resource type.
#[derive(Debug)]
pub struct Project {
    properties: ProjectProperties
}

/// Properties for the `Project` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectProperties {
    /// Property `Artifacts`.
    #[serde(rename="Artifacts")]
    pub artifacts: self::project::Artifacts,
    /// Property `BadgeEnabled`.
    #[serde(rename="BadgeEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge_enabled: Option<bool>,
    /// Property `Cache`.
    #[serde(rename="Cache")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<self::project::ProjectCache>,
    /// Property `Description`.
    #[serde(rename="Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Property `EncryptionKey`.
    #[serde(rename="EncryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,
    /// Property `Environment`.
    #[serde(rename="Environment")]
    pub environment: self::project::Environment,
    /// Property `Name`.
    #[serde(rename="Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Property `ServiceRole`.
    #[serde(rename="ServiceRole")]
    pub service_role: String,
    /// Property `Source`.
    #[serde(rename="Source")]
    pub source: self::project::Source,
    /// Property `Tags`.
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::Tags>,
    /// Property `TimeoutInMinutes`.
    #[serde(rename="TimeoutInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_minutes: Option<u32>,
    /// Property `Triggers`.
    #[serde(rename="Triggers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggers: Option<self::project::ProjectTriggers>,
    /// Property `VpcConfig`.
    #[serde(rename="VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<self::project::VpcConfig>,
}

impl<'a> ::Resource<'a> for Project {
    type Properties = ProjectProperties;
    const TYPE: &'static str = "AWS::CodeBuild::Project";
    fn properties(&self) -> &ProjectProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ProjectProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Project {}

impl From<ProjectProperties> for Project {
    fn from(properties: ProjectProperties) -> Project {
        Project { properties }
    }
}

pub mod project {
    //! Property types for the `Project` resource.

    /// The [`AWS::CodeBuild::Project.Artifacts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-artifacts.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Artifacts {
        /// Property `Location`.
        #[serde(rename="Location")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub location: Option<String>,
        /// Property `Name`.
        #[serde(rename="Name")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        /// Property `NamespaceType`.
        #[serde(rename="NamespaceType")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub namespace_type: Option<String>,
        /// Property `Packaging`.
        #[serde(rename="Packaging")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub packaging: Option<String>,
        /// Property `Path`.
        #[serde(rename="Path")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub path: Option<String>,
        /// Property `Type`.
        #[serde(rename="Type")]
        pub type_: String,
    }

    /// The [`AWS::CodeBuild::Project.Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-environment.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Environment {
        /// Property `ComputeType`.
        #[serde(rename="ComputeType")]
        pub compute_type: String,
        /// Property `EnvironmentVariables`.
        #[serde(rename="EnvironmentVariables")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub environment_variables: Option<Vec<EnvironmentVariable>>,
        /// Property `Image`.
        #[serde(rename="Image")]
        pub image: String,
        /// Property `PrivilegedMode`.
        #[serde(rename="PrivilegedMode")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub privileged_mode: Option<bool>,
        /// Property `Type`.
        #[serde(rename="Type")]
        pub type_: String,
    }

    /// The [`AWS::CodeBuild::Project.EnvironmentVariable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-environmentvariable.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EnvironmentVariable {
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
        /// Property `Type`.
        #[serde(rename="Type")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
        /// Property `Value`.
        #[serde(rename="Value")]
        pub value: String,
    }

    /// The [`AWS::CodeBuild::Project.ProjectCache`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-projectcache.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ProjectCache {
        /// Property `Location`.
        #[serde(rename="Location")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub location: Option<String>,
        /// Property `Type`.
        #[serde(rename="Type")]
        pub type_: String,
    }

    /// The [`AWS::CodeBuild::Project.ProjectTriggers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-projecttriggers.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ProjectTriggers {
        /// Property `Webhook`.
        #[serde(rename="Webhook")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub webhook: Option<bool>,
    }

    /// The [`AWS::CodeBuild::Project.Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-source.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Source {
        /// Property `Auth`.
        #[serde(rename="Auth")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub auth: Option<SourceAuth>,
        /// Property `BuildSpec`.
        #[serde(rename="BuildSpec")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub build_spec: Option<String>,
        /// Property `GitCloneDepth`.
        #[serde(rename="GitCloneDepth")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub git_clone_depth: Option<u32>,
        /// Property `InsecureSsl`.
        #[serde(rename="InsecureSsl")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub insecure_ssl: Option<bool>,
        /// Property `Location`.
        #[serde(rename="Location")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub location: Option<String>,
        /// Property `Type`.
        #[serde(rename="Type")]
        pub type_: String,
    }

    /// The [`AWS::CodeBuild::Project.SourceAuth`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-sourceauth.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SourceAuth {
        /// Property `Resource`.
        #[serde(rename="Resource")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        /// Property `Type`.
        #[serde(rename="Type")]
        pub type_: String,
    }

    /// The [`AWS::CodeBuild::Project.VpcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-vpcconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VpcConfig {
        /// Property `SecurityGroupIds`.
        #[serde(rename="SecurityGroupIds")]
        pub security_group_ids: Vec<String>,
        /// Property `Subnets`.
        #[serde(rename="Subnets")]
        pub subnets: Vec<String>,
        /// Property `VpcId`.
        #[serde(rename="VpcId")]
        pub vpc_id: String,
    }
}
