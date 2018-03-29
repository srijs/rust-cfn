/// The [`AWS::CodeCommit::Repository`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codecommit-repository.html) resource type.
pub struct Repository {
    properties: RepositoryProperties
}

/// Properties for the `Repository` resource.
#[derive(Serialize, Deserialize)]
pub struct RepositoryProperties {
    #[serde(rename="RepositoryDescription")]
    pub repository_description: String,
    #[serde(rename="RepositoryName")]
    pub repository_name: String,
    #[serde(rename="Triggers")]
    pub triggers: Vec<self::repository::RepositoryTrigger>,
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

impl ::private::Sealed for Repository {}

impl From<RepositoryProperties> for Repository {
    fn from(properties: RepositoryProperties) -> Repository {
        Repository { properties }
    }
}

pub mod repository {
    /// The [`AWS::CodeCommit::Repository.RepositoryTrigger`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codecommit-repository-repositorytrigger.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct RepositoryTrigger {
        #[serde(rename="Branches")]
        pub branches: Vec<String>,
        #[serde(rename="CustomData")]
        pub custom_data: String,
        #[serde(rename="DestinationArn")]
        pub destination_arn: String,
        #[serde(rename="Events")]
        pub events: Vec<String>,
        #[serde(rename="Name")]
        pub name: String,
    }

}

