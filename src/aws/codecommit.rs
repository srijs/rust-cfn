//! Types for the `CodeCommit` service.

/// The [`AWS::CodeCommit::Repository`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codecommit-repository.html) resource type.
#[derive(Debug)]
pub struct Repository {
    properties: RepositoryProperties
}

/// Properties for the `Repository` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct RepositoryProperties {
    /// Property `RepositoryDescription`.
    #[serde(rename="RepositoryDescription")]
    pub repository_description: String,
    /// Property `RepositoryName`.
    #[serde(rename="RepositoryName")]
    pub repository_name: String,
    /// Property `Triggers`.
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
    //! Property types for the `Repository` resource.

    /// The [`AWS::CodeCommit::Repository.RepositoryTrigger`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codecommit-repository-repositorytrigger.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RepositoryTrigger {
        /// Property `Branches`.
        #[serde(rename="Branches")]
        pub branches: Vec<String>,
        /// Property `CustomData`.
        #[serde(rename="CustomData")]
        pub custom_data: String,
        /// Property `DestinationArn`.
        #[serde(rename="DestinationArn")]
        pub destination_arn: String,
        /// Property `Events`.
        #[serde(rename="Events")]
        pub events: Vec<String>,
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
    }
}
