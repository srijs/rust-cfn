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
    #[serde(rename = "Artifacts")]
    pub artifacts: ::Value<self::project::Artifacts>,
    /// Property `BadgeEnabled`.
    #[serde(rename = "BadgeEnabled")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub badge_enabled: Option<::Value<bool>>,
    /// Property `Cache`.
    #[serde(rename = "Cache")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache: Option<::Value<self::project::ProjectCache>>,
    /// Property `Description`.
    #[serde(rename = "Description")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<::Value<String>>,
    /// Property `EncryptionKey`.
    #[serde(rename = "EncryptionKey")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<::Value<String>>,
    /// Property `Environment`.
    #[serde(rename = "Environment")]
    pub environment: ::Value<self::project::Environment>,
    /// Property `Name`.
    #[serde(rename = "Name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<::Value<String>>,
    /// Property `ServiceRole`.
    #[serde(rename = "ServiceRole")]
    pub service_role: ::Value<String>,
    /// Property `Source`.
    #[serde(rename = "Source")]
    pub source: ::Value<self::project::Source>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `TimeoutInMinutes`.
    #[serde(rename = "TimeoutInMinutes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_in_minutes: Option<::Value<u32>>,
    /// Property `Triggers`.
    #[serde(rename = "Triggers")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub triggers: Option<::Value<self::project::ProjectTriggers>>,
    /// Property `VpcConfig`.
    #[serde(rename = "VpcConfig")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<::Value<self::project::VpcConfig>>,
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
        #[serde(rename = "Location")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub location: Option<::Value<String>>,
        /// Property `Name`.
        #[serde(rename = "Name")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<::Value<String>>,
        /// Property `NamespaceType`.
        #[serde(rename = "NamespaceType")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub namespace_type: Option<::Value<String>>,
        /// Property `Packaging`.
        #[serde(rename = "Packaging")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub packaging: Option<::Value<String>>,
        /// Property `Path`.
        #[serde(rename = "Path")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub path: Option<::Value<String>>,
        /// Property `Type`.
        #[serde(rename = "Type")]
        pub type_: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(Artifacts);

    /// The [`AWS::CodeBuild::Project.Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-environment.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Environment {
        /// Property `ComputeType`.
        #[serde(rename = "ComputeType")]
        pub compute_type: ::Value<String>,
        /// Property `EnvironmentVariables`.
        #[serde(rename = "EnvironmentVariables")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub environment_variables: Option<::ValueList<EnvironmentVariable>>,
        /// Property `Image`.
        #[serde(rename = "Image")]
        pub image: ::Value<String>,
        /// Property `PrivilegedMode`.
        #[serde(rename = "PrivilegedMode")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub privileged_mode: Option<::Value<bool>>,
        /// Property `Type`.
        #[serde(rename = "Type")]
        pub type_: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(Environment);

    /// The [`AWS::CodeBuild::Project.EnvironmentVariable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-environmentvariable.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EnvironmentVariable {
        /// Property `Name`.
        #[serde(rename = "Name")]
        pub name: ::Value<String>,
        /// Property `Type`.
        #[serde(rename = "Type")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<::Value<String>>,
        /// Property `Value`.
        #[serde(rename = "Value")]
        pub value: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(EnvironmentVariable);

    /// The [`AWS::CodeBuild::Project.ProjectCache`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-projectcache.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ProjectCache {
        /// Property `Location`.
        #[serde(rename = "Location")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub location: Option<::Value<String>>,
        /// Property `Type`.
        #[serde(rename = "Type")]
        pub type_: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(ProjectCache);

    /// The [`AWS::CodeBuild::Project.ProjectTriggers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-projecttriggers.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ProjectTriggers {
        /// Property `Webhook`.
        #[serde(rename = "Webhook")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub webhook: Option<::Value<bool>>,
    }

    cfn_internal__inherit_codec_impls!(ProjectTriggers);

    /// The [`AWS::CodeBuild::Project.Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-source.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Source {
        /// Property `Auth`.
        #[serde(rename = "Auth")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub auth: Option<::Value<SourceAuth>>,
        /// Property `BuildSpec`.
        #[serde(rename = "BuildSpec")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub build_spec: Option<::Value<String>>,
        /// Property `GitCloneDepth`.
        #[serde(rename = "GitCloneDepth")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub git_clone_depth: Option<::Value<u32>>,
        /// Property `InsecureSsl`.
        #[serde(rename = "InsecureSsl")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub insecure_ssl: Option<::Value<bool>>,
        /// Property `Location`.
        #[serde(rename = "Location")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub location: Option<::Value<String>>,
        /// Property `Type`.
        #[serde(rename = "Type")]
        pub type_: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(Source);

    /// The [`AWS::CodeBuild::Project.SourceAuth`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-sourceauth.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SourceAuth {
        /// Property `Resource`.
        #[serde(rename = "Resource")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<::Value<String>>,
        /// Property `Type`.
        #[serde(rename = "Type")]
        pub type_: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(SourceAuth);

    /// The [`AWS::CodeBuild::Project.VpcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codebuild-project-vpcconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct VpcConfig {
        /// Property `SecurityGroupIds`.
        #[serde(rename = "SecurityGroupIds")]
        pub security_group_ids: ::ValueList<String>,
        /// Property `Subnets`.
        #[serde(rename = "Subnets")]
        pub subnets: ::ValueList<String>,
        /// Property `VpcId`.
        #[serde(rename = "VpcId")]
        pub vpc_id: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(VpcConfig);
}
