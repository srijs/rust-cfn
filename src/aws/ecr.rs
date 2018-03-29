/// The [`AWS::ECR::Repository`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecr-repository.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Repository {
    properties: RepositoryProperties
}

/// Properties for the `Repository` resource.
#[derive(Serialize, Deserialize)]
pub struct RepositoryProperties {
    #[serde(rename="LifecyclePolicy")]
    pub lifecycle_policy: self::repository::LifecyclePolicy,
    #[serde(rename="RepositoryName")]
    pub repository_name: String,
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

impl From<RepositoryProperties> for Repository {
    fn from(properties: RepositoryProperties) -> Repository {
        Repository { properties }
    }
}

pub mod repository {
    #[derive(Serialize, Deserialize)]
    pub struct LifecyclePolicy {
        #[serde(rename="LifecyclePolicyText")]
        pub lifecycle_policy_text: String,
        #[serde(rename="RegistryId")]
        pub registry_id: String,
    }

}

