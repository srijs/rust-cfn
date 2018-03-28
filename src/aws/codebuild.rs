/// The [`AWS::CodeBuild::Project`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codebuild-project.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Project {
    properties: ProjectProperties
}

/// Properties for the `Project` resource.
#[derive(Serialize, Deserialize)]
pub struct ProjectProperties {
    #[serde(rename="Artifacts")]
    pub artifacts: (),
    #[serde(rename="BadgeEnabled")]
    pub badge_enabled: bool,
    #[serde(rename="Cache")]
    pub cache: (),
    #[serde(rename="Description")]
    pub description: String,
    #[serde(rename="EncryptionKey")]
    pub encryption_key: String,
    #[serde(rename="Environment")]
    pub environment: (),
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="ServiceRole")]
    pub service_role: String,
    #[serde(rename="Source")]
    pub source: (),
    #[serde(rename="Tags")]
    pub tags: Vec<()>,
    #[serde(rename="TimeoutInMinutes")]
    pub timeout_in_minutes: u32,
    #[serde(rename="Triggers")]
    pub triggers: (),
    #[serde(rename="VpcConfig")]
    pub vpc_config: (),
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

