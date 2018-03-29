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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_description: Option<String>,
    /// Property `RepositoryName`.
    #[serde(rename="RepositoryName")]
    pub repository_name: String,
    /// Property `Triggers`.
    #[serde(rename="Triggers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Vec<self::repository::RepositoryTrigger>>,
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
        #[serde(skip_serializing_if = "Option::is_none")]
        pub branches: Option<Vec<String>>,
        /// Property `CustomData`.
        #[serde(rename="CustomData")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub custom_data: Option<String>,
        /// Property `DestinationArn`.
        #[serde(rename="DestinationArn")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub destination_arn: Option<String>,
        /// Property `Events`.
        #[serde(rename="Events")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub events: Option<Vec<String>>,
        /// Property `Name`.
        #[serde(rename="Name")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }
}
