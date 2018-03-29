/// The [`AWS::Cloud9::EnvironmentEC2`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloud9-environmentec2.html) resource type.
pub struct EnvironmentEC2 {
    properties: EnvironmentEC2Properties
}

/// Properties for the `EnvironmentEC2` resource.
#[derive(Serialize, Deserialize)]
pub struct EnvironmentEC2Properties {
    #[serde(rename="AutomaticStopTimeMinutes")]
    pub automatic_stop_time_minutes: u32,
    #[serde(rename="Description")]
    pub description: String,
    #[serde(rename="InstanceType")]
    pub instance_type: String,
    #[serde(rename="Name")]
    pub name: String,
    #[serde(rename="OwnerArn")]
    pub owner_arn: String,
    #[serde(rename="Repositories")]
    pub repositories: Vec<self::environment_ec2::Repository>,
    #[serde(rename="SubnetId")]
    pub subnet_id: String,
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

impl From<EnvironmentEC2Properties> for EnvironmentEC2 {
    fn from(properties: EnvironmentEC2Properties) -> EnvironmentEC2 {
        EnvironmentEC2 { properties }
    }
}

pub mod environment_ec2 {
    /// The [`AWS::Cloud9::EnvironmentEC2.Repository`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloud9-environmentec2-repository.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Repository {
        #[serde(rename="PathComponent")]
        pub path_component: String,
        #[serde(rename="RepositoryUrl")]
        pub repository_url: String,
    }

}

