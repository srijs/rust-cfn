/// The [`AWS::CodeCommit::Repository`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codecommit-repository.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Repository {
    properties: RepositoryProperties
}

/// Properties for the `Repository` resource.
#[derive(Serialize, Deserialize)]
pub struct RepositoryProperties {
    #[serde(rename="RepositoryDescription")]
    pub repository_description: (),
    #[serde(rename="RepositoryName")]
    pub repository_name: (),
    #[serde(rename="Triggers")]
    pub triggers: (),
}

impl<'a> ::Resource<'a> for Repository {
    type Properties = RepositoryProperties;
    const TYPE: &'static str = "AWS::CodeCommit::Repository";
    fn properties(&self) -> &RepositoryProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RepositoryProperties {
        &mut self.properties
    }
}

impl From<RepositoryProperties> for Repository {
    fn from(properties: RepositoryProperties) -> Repository {
        Repository { properties }
    }
}

