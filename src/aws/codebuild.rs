/// The [`AWS::CodeBuild::Project`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-project.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Project {
    properties: ProjectProperties
}

/// Properties for the `Project` resource.
#[derive(Serialize, Deserialize)]
pub struct ProjectProperties {
    #[serde(rename="Artifacts")]
    pub artifacts: self::project::Artifacts,
    #[serde(rename="BadgeEnabled")]
    pub badge_enabled: bool,
    #[serde(rename="Cache")]
    pub cache: self::project::ProjectCache,
    #[serde(rename="Description")]
    pub description: String,
    #[serde(rename="EncryptionKey")]
    pub encryption_key: String,
    #[serde(rename="Environment")]
    pub environment: self::project::Environment,
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="ServiceRole")]
    pub service_role: String,
    #[serde(rename="Source")]
    pub source: self::project::Source,
    #[serde(rename="Tags")]
    pub tags: ::Tags,
    #[serde(rename="TimeoutInMinutes")]
    pub timeout_in_minutes: u32,
    #[serde(rename="Triggers")]
    pub triggers: self::project::ProjectTriggers,
    #[serde(rename="VpcConfig")]
    pub vpc_config: self::project::VpcConfig,
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

impl From<ProjectProperties> for Project {
    fn from(properties: ProjectProperties) -> Project {
        Project { properties }
    }
}

pub mod project {
    #[derive(Serialize, Deserialize)]
    pub struct Artifacts {
        #[serde(rename="Location")]
        pub location: String,
        #[serde(rename="Name")]
        pub name: String,
        #[serde(rename="NamespaceType")]
        pub namespace_type: String,
        #[serde(rename="Packaging")]
        pub packaging: String,
        #[serde(rename="Path")]
        pub path: String,
        #[serde(rename="Type")]
        pub type_: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Environment {
        #[serde(rename="ComputeType")]
        pub compute_type: String,
        #[serde(rename="EnvironmentVariables")]
        pub environment_variables: Vec<EnvironmentVariable>,
        #[serde(rename="Image")]
        pub image: String,
        #[serde(rename="PrivilegedMode")]
        pub privileged_mode: bool,
        #[serde(rename="Type")]
        pub type_: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct EnvironmentVariable {
        #[serde(rename="Name")]
        pub name: String,
        #[serde(rename="Type")]
        pub type_: String,
        #[serde(rename="Value")]
        pub value: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ProjectCache {
        #[serde(rename="Location")]
        pub location: String,
        #[serde(rename="Type")]
        pub type_: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ProjectTriggers {
        #[serde(rename="Webhook")]
        pub webhook: bool,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Source {
        #[serde(rename="Auth")]
        pub auth: SourceAuth,
        #[serde(rename="BuildSpec")]
        pub build_spec: String,
        #[serde(rename="GitCloneDepth")]
        pub git_clone_depth: u32,
        #[serde(rename="InsecureSsl")]
        pub insecure_ssl: bool,
        #[serde(rename="Location")]
        pub location: String,
        #[serde(rename="Type")]
        pub type_: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct SourceAuth {
        #[serde(rename="Resource")]
        pub resource: String,
        #[serde(rename="Type")]
        pub type_: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct VpcConfig {
        #[serde(rename="SecurityGroupIds")]
        pub security_group_ids: Vec<String>,
        #[serde(rename="Subnets")]
        pub subnets: Vec<String>,
        #[serde(rename="VpcId")]
        pub vpc_id: String,
    }

}

