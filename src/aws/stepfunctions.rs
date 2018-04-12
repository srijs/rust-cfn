//! Types for the `StepFunctions` service.

/// The [`AWS::StepFunctions::Activity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-stepfunctions-activity.html) resource type.
#[derive(Debug)]
pub struct Activity {
    properties: ActivityProperties
}

/// Properties for the `Activity` resource.
#[derive(Debug)]
pub struct ActivityProperties {
    /// Property `Name`.
    pub name: ::Value<String>,
}

impl ::serde::Serialize for ActivityProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ActivityProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ActivityProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ActivityProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ActivityProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ActivityProperties {
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Activity {
    type Properties = ActivityProperties;
    const TYPE: &'static str = "AWS::StepFunctions::Activity";
    fn properties(&self) -> &ActivityProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ActivityProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Activity {}

impl From<ActivityProperties> for Activity {
    fn from(properties: ActivityProperties) -> Activity {
        Activity { properties }
    }
}

/// The [`AWS::StepFunctions::StateMachine`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-stepfunctions-statemachine.html) resource type.
#[derive(Debug)]
pub struct StateMachine {
    properties: StateMachineProperties
}

/// Properties for the `StateMachine` resource.
#[derive(Debug)]
pub struct StateMachineProperties {
    /// Property `DefinitionString`.
    pub definition_string: ::Value<String>,
    /// Property `RoleArn`.
    pub role_arn: ::Value<String>,
    /// Property `StateMachineName`.
    pub state_machine_name: Option<::Value<String>>,
}

impl ::serde::Serialize for StateMachineProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefinitionString", &self.definition_string)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        if let Some(ref state_machine_name) = self.state_machine_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StateMachineName", state_machine_name)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for StateMachineProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<StateMachineProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = StateMachineProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type StateMachineProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut definition_string: Option<::Value<String>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut state_machine_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DefinitionString" => {
                            definition_string = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StateMachineName" => {
                            state_machine_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(StateMachineProperties {
                    definition_string: definition_string.ok_or(::serde::de::Error::missing_field("DefinitionString"))?,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    state_machine_name: state_machine_name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for StateMachine {
    type Properties = StateMachineProperties;
    const TYPE: &'static str = "AWS::StepFunctions::StateMachine";
    fn properties(&self) -> &StateMachineProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut StateMachineProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for StateMachine {}

impl From<StateMachineProperties> for StateMachine {
    fn from(properties: StateMachineProperties) -> StateMachine {
        StateMachine { properties }
    }
}
