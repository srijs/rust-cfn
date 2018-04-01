//! Types for the `CloudFormation` service.

/// The [`AWS::CloudFormation::CustomResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cfn-customresource.html) resource type.
#[derive(Debug)]
pub struct CustomResource {
    properties: CustomResourceProperties
}

/// Properties for the `CustomResource` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomResourceProperties {
    /// Property `ServiceToken`.
    #[serde(rename = "ServiceToken")]
    pub service_token: ::Value<String>,
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

impl ::private::Sealed for CustomResource {}

impl From<CustomResourceProperties> for CustomResource {
    fn from(properties: CustomResourceProperties) -> CustomResource {
        CustomResource { properties }
    }
}

/// The [`AWS::CloudFormation::Stack`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-stack.html) resource type.
#[derive(Debug)]
pub struct Stack {
    properties: StackProperties
}

/// Properties for the `Stack` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct StackProperties {
    /// Property `NotificationARNs`.
    #[serde(rename = "NotificationARNs")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_ar_ns: Option<::ValueList<String>>,
    /// Property `Parameters`.
    #[serde(rename = "Parameters")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, ::Value<String>>>,
    /// Property `Tags`.
    #[serde(rename = "Tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `TemplateURL`.
    #[serde(rename = "TemplateURL")]
    pub template_url: ::Value<String>,
    /// Property `TimeoutInMinutes`.
    #[serde(rename = "TimeoutInMinutes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_in_minutes: Option<::Value<u32>>,
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

impl ::private::Sealed for Stack {}

impl From<StackProperties> for Stack {
    fn from(properties: StackProperties) -> Stack {
        Stack { properties }
    }
}

/// The [`AWS::CloudFormation::WaitCondition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waitcondition.html) resource type.
#[derive(Debug)]
pub struct WaitCondition {
    properties: WaitConditionProperties
}

/// Properties for the `WaitCondition` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct WaitConditionProperties {
    /// Property `Count`.
    #[serde(rename = "Count")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<::Value<u32>>,
    /// Property `Handle`.
    #[serde(rename = "Handle")]
    pub handle: ::Value<String>,
    /// Property `Timeout`.
    #[serde(rename = "Timeout")]
    pub timeout: ::Value<String>,
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

impl ::private::Sealed for WaitCondition {}

impl From<WaitConditionProperties> for WaitCondition {
    fn from(properties: WaitConditionProperties) -> WaitCondition {
        WaitCondition { properties }
    }
}

/// The [`AWS::CloudFormation::WaitConditionHandle`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waitconditionhandle.html) resource type.
#[derive(Debug)]
pub struct WaitConditionHandle {
    properties: WaitConditionHandleProperties
}

/// Properties for the `WaitConditionHandle` resource.
#[derive(Debug, Serialize, Deserialize)]
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

impl ::private::Sealed for WaitConditionHandle {}

impl From<WaitConditionHandleProperties> for WaitConditionHandle {
    fn from(properties: WaitConditionHandleProperties) -> WaitConditionHandle {
        WaitConditionHandle { properties }
    }
}
