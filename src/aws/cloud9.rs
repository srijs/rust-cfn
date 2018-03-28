/// The [`AWS::Cloud9::EnvironmentEC2`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloud9-environmentec2.html) resource.
#[derive(Serialize, Deserialize)]
pub struct EnvironmentEC2 {
    properties: EnvironmentEC2Properties
}

/// Properties for the `EnvironmentEC2` resource.
#[derive(Serialize, Deserialize)]
pub struct EnvironmentEC2Properties {
    #[serde(rename="AutomaticStopTimeMinutes")]
    pub automatic_stop_time_minutes: (),
    #[serde(rename="Description")]
    pub description: (),
    #[serde(rename="InstanceType")]
    pub instance_type: (),
    #[serde(rename="Name")]
    pub name: (),
    #[serde(rename="OwnerArn")]
    pub owner_arn: (),
    #[serde(rename="Repositories")]
    pub repositories: (),
    #[serde(rename="SubnetId")]
    pub subnet_id: (),
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

