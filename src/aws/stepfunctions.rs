/// The [`AWS::StepFunctions::Activity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-stepfunctions-activity.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Activity {
    properties: ActivityProperties
}

/// Properties for the `Activity` resource.
#[derive(Serialize, Deserialize)]
pub struct ActivityProperties {
    #[serde(rename="Name")]
    pub name: (),
}

impl<'a> ::Resource<'a> for Activity {
    type Properties = ActivityProperties;
    const TYPE: &'static str = "AWS::StepFunctions::Activity";
    fn properties(&self) -> &ActivityProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ActivityProperties {
        &mut self.properties
    }
}

impl From<ActivityProperties> for Activity {
    fn from(properties: ActivityProperties) -> Activity {
        Activity { properties }
    }
}

/// The [`AWS::StepFunctions::StateMachine`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-stepfunctions-statemachine.html) resource.
#[derive(Serialize, Deserialize)]
pub struct StateMachine {
    properties: StateMachineProperties
}

/// Properties for the `StateMachine` resource.
#[derive(Serialize, Deserialize)]
pub struct StateMachineProperties {
    #[serde(rename="DefinitionString")]
    pub definition_string: (),
    #[serde(rename="RoleArn")]
    pub role_arn: (),
    #[serde(rename="StateMachineName")]
    pub state_machine_name: (),
}

impl<'a> ::Resource<'a> for StateMachine {
    type Properties = StateMachineProperties;
    const TYPE: &'static str = "AWS::StepFunctions::StateMachine";
    fn properties(&self) -> &StateMachineProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut StateMachineProperties {
        &mut self.properties
    }
}

impl From<StateMachineProperties> for StateMachine {
    fn from(properties: StateMachineProperties) -> StateMachine {
        StateMachine { properties }
    }
}

