//! Types for the `ECR` service.

/// The [`AWS::ECR::Repository`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecr-repository.html) resource type.
#[derive(Debug)]
pub struct Repository {
    properties: RepositoryProperties
}

/// Properties for the `Repository` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct RepositoryProperties {
    /// Property `LifecyclePolicy`.
    #[serde(rename="LifecyclePolicy")]
    pub lifecycle_policy: self::repository::LifecyclePolicy,
    /// Property `RepositoryName`.
    #[serde(rename="RepositoryName")]
    pub repository_name: String,
    /// Property `RepositoryPolicyText`.
    #[serde(rename="RepositoryPolicyText")]
    pub repository_policy_text: ::json::Value,
}

impl<'a> ::Resource<'a> for Repository {
    type Properties = RepositoryProperties;
    const TYPE: &'static str = "AWS::ECR::Repository";
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

    /// The [`AWS::ECR::Repository.LifecyclePolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecr-repository-lifecyclepolicy.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LifecyclePolicy {
        /// Property `LifecyclePolicyText`.
        #[serde(rename="LifecyclePolicyText")]
        pub lifecycle_policy_text: String,
        /// Property `RegistryId`.
        #[serde(rename="RegistryId")]
        pub registry_id: String,
    }
}
