//! Types for the `CloudFormation` service.

/// The [`AWS::CloudFormation::CustomResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cfn-customresource.html) resource type.
#[derive(Debug)]
pub struct CustomResource {
    properties: CustomResourceProperties
}

/// Properties for the `CustomResource` resource.
#[derive(Debug)]
pub struct CustomResourceProperties {
    /// Property `ServiceToken`.
    pub service_token: ::Value<String>,
}

impl ::serde::Serialize for CustomResourceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceToken", &self.service_token)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CustomResourceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomResourceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CustomResourceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CustomResourceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut service_token = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ServiceToken" => {
                            service_token = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(CustomResourceProperties {
                    service_token: service_token.ok_or(::serde::de::Error::missing_field("ServiceToken"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct StackProperties {
    /// Property `NotificationARNs`.
    pub notification_ar_ns: Option<::ValueList<String>>,
    /// Property `Parameters`.
    pub parameters: Option<::ValueMap<String>>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `TemplateURL`.
    pub template_url: ::Value<String>,
    /// Property `TimeoutInMinutes`.
    pub timeout_in_minutes: Option<::Value<u32>>,
}

impl ::serde::Serialize for StackProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationARNs", &self.notification_ar_ns)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", &self.parameters)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemplateURL", &self.template_url)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeoutInMinutes", &self.timeout_in_minutes)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for StackProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<StackProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = StackProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type StackProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut notification_ar_ns = None;
                let mut parameters = None;
                let mut tags = None;
                let mut template_url = None;
                let mut timeout_in_minutes = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "NotificationARNs" => {
                            notification_ar_ns = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Parameters" => {
                            parameters = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "TemplateURL" => {
                            template_url = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "TimeoutInMinutes" => {
                            timeout_in_minutes = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(StackProperties {
                    notification_ar_ns: notification_ar_ns,
                    parameters: parameters,
                    tags: tags,
                    template_url: template_url.ok_or(::serde::de::Error::missing_field("TemplateURL"))?,
                    timeout_in_minutes: timeout_in_minutes,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct WaitConditionProperties {
    /// Property `Count`.
    pub count: Option<::Value<u32>>,
    /// Property `Handle`.
    pub handle: ::Value<String>,
    /// Property `Timeout`.
    pub timeout: ::Value<String>,
}

impl ::serde::Serialize for WaitConditionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Count", &self.count)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Handle", &self.handle)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Timeout", &self.timeout)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for WaitConditionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<WaitConditionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = WaitConditionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type WaitConditionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut count = None;
                let mut handle = None;
                let mut timeout = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Count" => {
                            count = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Handle" => {
                            handle = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Timeout" => {
                            timeout = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(WaitConditionProperties {
                    count: count,
                    handle: handle.ok_or(::serde::de::Error::missing_field("Handle"))?,
                    timeout: timeout.ok_or(::serde::de::Error::missing_field("Timeout"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
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
#[derive(Debug)]
pub struct WaitConditionHandleProperties {
}

impl ::serde::Serialize for WaitConditionHandleProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[allow(unused_mut)]
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for WaitConditionHandleProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<WaitConditionHandleProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = WaitConditionHandleProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type WaitConditionHandleProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                Ok(WaitConditionHandleProperties {})
            }
        }

        d.deserialize_map(Visitor)
    }
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
