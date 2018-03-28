/// The [`AWS::CloudFormation::CustomResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cfn-customresource.html) resource.
#[derive(Serialize, Deserialize)]
pub struct CustomResource {
    properties: CustomResourceProperties
}

/// Properties for the `CustomResource` resource.
#[derive(Serialize, Deserialize)]
pub struct CustomResourceProperties {
    #[serde(rename="ServiceToken")]
    pub service_token: String,
}

impl<'a> ::Resource<'a> for CustomResource {
    type Properties = CustomResourceProperties;
    const TYPE: &'static str = "AWS::CloudFormation::CustomResource";
    fn properties(&self) -> &CustomResourceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CustomResourceProperties {
        &mut self.properties
    }
}

impl From<CustomResourceProperties> for CustomResource {
    fn from(properties: CustomResourceProperties) -> CustomResource {
        CustomResource { properties }
    }
}

/// The [`AWS::CloudFormation::Stack`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-stack.html) resource.
#[derive(Serialize, Deserialize)]
pub struct Stack {
    properties: StackProperties
}

/// Properties for the `Stack` resource.
#[derive(Serialize, Deserialize)]
pub struct StackProperties {
    #[serde(rename="NotificationARNs")]
    pub notification_ar_ns: Vec<String>,
    #[serde(rename="Parameters")]
    pub parameters: ::std::collections::HashMap<String, String>,
    #[serde(rename="Tags")]
    pub tags: Vec<()>,
    #[serde(rename="TemplateURL")]
    pub template_url: String,
    #[serde(rename="TimeoutInMinutes")]
    pub timeout_in_minutes: u32,
}

impl<'a> ::Resource<'a> for Stack {
    type Properties = StackProperties;
    const TYPE: &'static str = "AWS::CloudFormation::Stack";
    fn properties(&self) -> &StackProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut StackProperties {
        &mut self.properties
    }
}

impl From<StackProperties> for Stack {
    fn from(properties: StackProperties) -> Stack {
        Stack { properties }
    }
}

/// The [`AWS::CloudFormation::WaitCondition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waitcondition.html) resource.
#[derive(Serialize, Deserialize)]
pub struct WaitCondition {
    properties: WaitConditionProperties
}

/// Properties for the `WaitCondition` resource.
#[derive(Serialize, Deserialize)]
pub struct WaitConditionProperties {
    #[serde(rename="Count")]
    pub count: u32,
    #[serde(rename="Handle")]
    pub handle: String,
    #[serde(rename="Timeout")]
    pub timeout: String,
}

impl<'a> ::Resource<'a> for WaitCondition {
    type Properties = WaitConditionProperties;
    const TYPE: &'static str = "AWS::CloudFormation::WaitCondition";
    fn properties(&self) -> &WaitConditionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut WaitConditionProperties {
        &mut self.properties
    }
}

impl From<WaitConditionProperties> for WaitCondition {
    fn from(properties: WaitConditionProperties) -> WaitCondition {
        WaitCondition { properties }
    }
}

/// The [`AWS::CloudFormation::WaitConditionHandle`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waitconditionhandle.html) resource.
#[derive(Serialize, Deserialize)]
pub struct WaitConditionHandle {
    properties: WaitConditionHandleProperties
}

/// Properties for the `WaitConditionHandle` resource.
#[derive(Serialize, Deserialize)]
pub struct WaitConditionHandleProperties {
}

impl<'a> ::Resource<'a> for WaitConditionHandle {
    type Properties = WaitConditionHandleProperties;
    const TYPE: &'static str = "AWS::CloudFormation::WaitConditionHandle";
    fn properties(&self) -> &WaitConditionHandleProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut WaitConditionHandleProperties {
        &mut self.properties
    }
}

impl From<WaitConditionHandleProperties> for WaitConditionHandle {
    fn from(properties: WaitConditionHandleProperties) -> WaitConditionHandle {
        WaitConditionHandle { properties }
    }
}

