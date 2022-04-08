//! Types for the `Chatbot` service.

/// The [`AWS::Chatbot::SlackChannelConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-chatbot-slackchannelconfiguration.html) resource type.
#[derive(Debug, Default)]
pub struct SlackChannelConfiguration {
    properties: SlackChannelConfigurationProperties
}

/// Properties for the `SlackChannelConfiguration` resource.
#[derive(Debug, Default)]
pub struct SlackChannelConfigurationProperties {
    /// Property [`ConfigurationName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-chatbot-slackchannelconfiguration.html#cfn-chatbot-slackchannelconfiguration-configurationname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub configuration_name: ::Value<String>,
    /// Property [`GuardrailPolicies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-chatbot-slackchannelconfiguration.html#cfn-chatbot-slackchannelconfiguration-guardrailpolicies).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub guardrail_policies: Option<::ValueList<String>>,
    /// Property [`IamRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-chatbot-slackchannelconfiguration.html#cfn-chatbot-slackchannelconfiguration-iamrolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub iam_role_arn: ::Value<String>,
    /// Property [`LoggingLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-chatbot-slackchannelconfiguration.html#cfn-chatbot-slackchannelconfiguration-logginglevel).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub logging_level: Option<::Value<String>>,
    /// Property [`SlackChannelId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-chatbot-slackchannelconfiguration.html#cfn-chatbot-slackchannelconfiguration-slackchannelid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub slack_channel_id: ::Value<String>,
    /// Property [`SlackWorkspaceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-chatbot-slackchannelconfiguration.html#cfn-chatbot-slackchannelconfiguration-slackworkspaceid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub slack_workspace_id: ::Value<String>,
    /// Property [`SnsTopicArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-chatbot-slackchannelconfiguration.html#cfn-chatbot-slackchannelconfiguration-snstopicarns).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sns_topic_arns: Option<::ValueList<String>>,
    /// Property [`UserRoleRequired`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-chatbot-slackchannelconfiguration.html#cfn-chatbot-slackchannelconfiguration-userrolerequired).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub user_role_required: Option<::Value<bool>>,
}

impl ::serde::Serialize for SlackChannelConfigurationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfigurationName", &self.configuration_name)?;
        if let Some(ref guardrail_policies) = self.guardrail_policies {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GuardrailPolicies", guardrail_policies)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IamRoleArn", &self.iam_role_arn)?;
        if let Some(ref logging_level) = self.logging_level {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoggingLevel", logging_level)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SlackChannelId", &self.slack_channel_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SlackWorkspaceId", &self.slack_workspace_id)?;
        if let Some(ref sns_topic_arns) = self.sns_topic_arns {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnsTopicArns", sns_topic_arns)?;
        }
        if let Some(ref user_role_required) = self.user_role_required {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserRoleRequired", user_role_required)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SlackChannelConfigurationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SlackChannelConfigurationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SlackChannelConfigurationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SlackChannelConfigurationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut configuration_name: Option<::Value<String>> = None;
                let mut guardrail_policies: Option<::ValueList<String>> = None;
                let mut iam_role_arn: Option<::Value<String>> = None;
                let mut logging_level: Option<::Value<String>> = None;
                let mut slack_channel_id: Option<::Value<String>> = None;
                let mut slack_workspace_id: Option<::Value<String>> = None;
                let mut sns_topic_arns: Option<::ValueList<String>> = None;
                let mut user_role_required: Option<::Value<bool>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ConfigurationName" => {
                            configuration_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GuardrailPolicies" => {
                            guardrail_policies = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IamRoleArn" => {
                            iam_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LoggingLevel" => {
                            logging_level = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SlackChannelId" => {
                            slack_channel_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SlackWorkspaceId" => {
                            slack_workspace_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SnsTopicArns" => {
                            sns_topic_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserRoleRequired" => {
                            user_role_required = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SlackChannelConfigurationProperties {
                    configuration_name: configuration_name.ok_or(::serde::de::Error::missing_field("ConfigurationName"))?,
                    guardrail_policies: guardrail_policies,
                    iam_role_arn: iam_role_arn.ok_or(::serde::de::Error::missing_field("IamRoleArn"))?,
                    logging_level: logging_level,
                    slack_channel_id: slack_channel_id.ok_or(::serde::de::Error::missing_field("SlackChannelId"))?,
                    slack_workspace_id: slack_workspace_id.ok_or(::serde::de::Error::missing_field("SlackWorkspaceId"))?,
                    sns_topic_arns: sns_topic_arns,
                    user_role_required: user_role_required,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for SlackChannelConfiguration {
    type Properties = SlackChannelConfigurationProperties;
    const TYPE: &'static str = "AWS::Chatbot::SlackChannelConfiguration";
    fn properties(&self) -> &SlackChannelConfigurationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SlackChannelConfigurationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SlackChannelConfiguration {}

impl From<SlackChannelConfigurationProperties> for SlackChannelConfiguration {
    fn from(properties: SlackChannelConfigurationProperties) -> SlackChannelConfiguration {
        SlackChannelConfiguration { properties }
    }
}
