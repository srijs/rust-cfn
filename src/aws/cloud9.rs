//! Types for the `Cloud9` service.

/// The [`AWS::Cloud9::EnvironmentEC2`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloud9-environmentec2.html) resource type.
#[derive(Debug)]
pub struct EnvironmentEC2 {
    properties: EnvironmentEC2Properties
}

/// Properties for the `EnvironmentEC2` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentEC2Properties {
    /// Property `AutomaticStopTimeMinutes`.
    #[serde(rename="AutomaticStopTimeMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_stop_time_minutes: Option<u32>,
    /// Property `Description`.
    #[serde(rename="Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Property `InstanceType`.
    #[serde(rename="InstanceType")]
    pub instance_type: String,
    /// Property `Name`.
    #[serde(rename="Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Property `OwnerArn`.
    #[serde(rename="OwnerArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_arn: Option<String>,
    /// Property `Repositories`.
    #[serde(rename="Repositories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repositories: Option<Vec<self::environment_ec2::Repository>>,
    /// Property `SubnetId`.
    #[serde(rename="SubnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}

impl<'a> ::Resource<'a> for EnvironmentEC2 {
    type Properties = EnvironmentEC2Properties;
    const TYPE: &'static str = "AWS::Cloud9::EnvironmentEC2";
    fn properties(&self) -> &EnvironmentEC2Properties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EnvironmentEC2Properties {
        &mut self.properties
    }
}

impl ::private::Sealed for EnvironmentEC2 {}

impl From<EnvironmentEC2Properties> for EnvironmentEC2 {
    fn from(properties: EnvironmentEC2Properties) -> EnvironmentEC2 {
        EnvironmentEC2 { properties }
    }
}

pub mod environment_ec2 {
    //! Property types for the `EnvironmentEC2` resource.

    /// The [`AWS::Cloud9::EnvironmentEC2.Repository`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloud9-environmentec2-repository.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Repository {
        /// Property `PathComponent`.
        #[serde(rename="PathComponent")]
        pub path_component: String,
        /// Property `RepositoryUrl`.
        #[serde(rename="RepositoryUrl")]
        pub repository_url: String,
    }
}
