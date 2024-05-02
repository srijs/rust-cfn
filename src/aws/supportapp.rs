//! Types for the `SupportApp` service.

/// The [`AWS::SupportApp::AccountAlias`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-supportapp-accountalias.html) resource type.
#[derive(Debug, Default)]
pub struct AccountAlias {
    properties: AccountAliasProperties
}

/// Properties for the `AccountAlias` resource.
#[derive(Debug, Default)]
pub struct AccountAliasProperties {
    /// Property [`AccountAlias`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-supportapp-accountalias.html#cfn-supportapp-accountalias-accountalias).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub account_alias: ::Value<String>,
}

impl ::serde::Serialize for AccountAliasProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccountAlias", &self.account_alias)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AccountAliasProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AccountAliasProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AccountAliasProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AccountAliasProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut account_alias: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AccountAlias" => {
                            account_alias = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AccountAliasProperties {
                    account_alias: account_alias.ok_or(::serde::de::Error::missing_field("AccountAlias"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for AccountAlias {
    type Properties = AccountAliasProperties;
    const TYPE: &'static str = "AWS::SupportApp::AccountAlias";
    fn properties(&self) -> &AccountAliasProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AccountAliasProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for AccountAlias {}

impl From<AccountAliasProperties> for AccountAlias {
    fn from(properties: AccountAliasProperties) -> AccountAlias {
        AccountAlias { properties }
    }
}

/// The [`AWS::SupportApp::SlackChannelConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-supportapp-slackchannelconfiguration.html) resource type.
#[derive(Debug, Default)]
pub struct SlackChannelConfiguration {
    properties: SlackChannelConfigurationProperties
}

/// Properties for the `SlackChannelConfiguration` resource.
#[derive(Debug, Default)]
pub struct SlackChannelConfigurationProperties {
    /// Property [`ChannelId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-supportapp-slackchannelconfiguration.html#cfn-supportapp-slackchannelconfiguration-channelid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub channel_id: ::Value<String>,
    /// Property [`ChannelName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-supportapp-slackchannelconfiguration.html#cfn-supportapp-slackchannelconfiguration-channelname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub channel_name: Option<::Value<String>>,
    /// Property [`ChannelRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-supportapp-slackchannelconfiguration.html#cfn-supportapp-slackchannelconfiguration-channelrolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub channel_role_arn: ::Value<String>,
    /// Property [`NotifyOnAddCorrespondenceToCase`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-supportapp-slackchannelconfiguration.html#cfn-supportapp-slackchannelconfiguration-notifyonaddcorrespondencetocase).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub notify_on_add_correspondence_to_case: Option<::Value<bool>>,
    /// Property [`NotifyOnCaseSeverity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-supportapp-slackchannelconfiguration.html#cfn-supportapp-slackchannelconfiguration-notifyoncaseseverity).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub notify_on_case_severity: ::Value<String>,
    /// Property [`NotifyOnCreateOrReopenCase`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-supportapp-slackchannelconfiguration.html#cfn-supportapp-slackchannelconfiguration-notifyoncreateorreopencase).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub notify_on_create_or_reopen_case: Option<::Value<bool>>,
    /// Property [`NotifyOnResolveCase`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-supportapp-slackchannelconfiguration.html#cfn-supportapp-slackchannelconfiguration-notifyonresolvecase).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub notify_on_resolve_case: Option<::Value<bool>>,
    /// Property [`TeamId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-supportapp-slackchannelconfiguration.html#cfn-supportapp-slackchannelconfiguration-teamid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub team_id: ::Value<String>,
}

impl ::serde::Serialize for SlackChannelConfigurationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChannelId", &self.channel_id)?;
        if let Some(ref channel_name) = self.channel_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChannelName", channel_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChannelRoleArn", &self.channel_role_arn)?;
        if let Some(ref notify_on_add_correspondence_to_case) = self.notify_on_add_correspondence_to_case {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotifyOnAddCorrespondenceToCase", notify_on_add_correspondence_to_case)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotifyOnCaseSeverity", &self.notify_on_case_severity)?;
        if let Some(ref notify_on_create_or_reopen_case) = self.notify_on_create_or_reopen_case {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotifyOnCreateOrReopenCase", notify_on_create_or_reopen_case)?;
        }
        if let Some(ref notify_on_resolve_case) = self.notify_on_resolve_case {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotifyOnResolveCase", notify_on_resolve_case)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TeamId", &self.team_id)?;
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
                let mut channel_id: Option<::Value<String>> = None;
                let mut channel_name: Option<::Value<String>> = None;
                let mut channel_role_arn: Option<::Value<String>> = None;
                let mut notify_on_add_correspondence_to_case: Option<::Value<bool>> = None;
                let mut notify_on_case_severity: Option<::Value<String>> = None;
                let mut notify_on_create_or_reopen_case: Option<::Value<bool>> = None;
                let mut notify_on_resolve_case: Option<::Value<bool>> = None;
                let mut team_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ChannelId" => {
                            channel_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ChannelName" => {
                            channel_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ChannelRoleArn" => {
                            channel_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NotifyOnAddCorrespondenceToCase" => {
                            notify_on_add_correspondence_to_case = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NotifyOnCaseSeverity" => {
                            notify_on_case_severity = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NotifyOnCreateOrReopenCase" => {
                            notify_on_create_or_reopen_case = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NotifyOnResolveCase" => {
                            notify_on_resolve_case = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TeamId" => {
                            team_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SlackChannelConfigurationProperties {
                    channel_id: channel_id.ok_or(::serde::de::Error::missing_field("ChannelId"))?,
                    channel_name: channel_name,
                    channel_role_arn: channel_role_arn.ok_or(::serde::de::Error::missing_field("ChannelRoleArn"))?,
                    notify_on_add_correspondence_to_case: notify_on_add_correspondence_to_case,
                    notify_on_case_severity: notify_on_case_severity.ok_or(::serde::de::Error::missing_field("NotifyOnCaseSeverity"))?,
                    notify_on_create_or_reopen_case: notify_on_create_or_reopen_case,
                    notify_on_resolve_case: notify_on_resolve_case,
                    team_id: team_id.ok_or(::serde::de::Error::missing_field("TeamId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for SlackChannelConfiguration {
    type Properties = SlackChannelConfigurationProperties;
    const TYPE: &'static str = "AWS::SupportApp::SlackChannelConfiguration";
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

/// The [`AWS::SupportApp::SlackWorkspaceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-supportapp-slackworkspaceconfiguration.html) resource type.
#[derive(Debug, Default)]
pub struct SlackWorkspaceConfiguration {
    properties: SlackWorkspaceConfigurationProperties
}

/// Properties for the `SlackWorkspaceConfiguration` resource.
#[derive(Debug, Default)]
pub struct SlackWorkspaceConfigurationProperties {
    /// Property [`TeamId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-supportapp-slackworkspaceconfiguration.html#cfn-supportapp-slackworkspaceconfiguration-teamid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub team_id: ::Value<String>,
    /// Property [`VersionId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-supportapp-slackworkspaceconfiguration.html#cfn-supportapp-slackworkspaceconfiguration-versionid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub version_id: Option<::Value<String>>,
}

impl ::serde::Serialize for SlackWorkspaceConfigurationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TeamId", &self.team_id)?;
        if let Some(ref version_id) = self.version_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VersionId", version_id)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SlackWorkspaceConfigurationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SlackWorkspaceConfigurationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SlackWorkspaceConfigurationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SlackWorkspaceConfigurationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut team_id: Option<::Value<String>> = None;
                let mut version_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "TeamId" => {
                            team_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VersionId" => {
                            version_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SlackWorkspaceConfigurationProperties {
                    team_id: team_id.ok_or(::serde::de::Error::missing_field("TeamId"))?,
                    version_id: version_id,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for SlackWorkspaceConfiguration {
    type Properties = SlackWorkspaceConfigurationProperties;
    const TYPE: &'static str = "AWS::SupportApp::SlackWorkspaceConfiguration";
    fn properties(&self) -> &SlackWorkspaceConfigurationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SlackWorkspaceConfigurationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SlackWorkspaceConfiguration {}

impl From<SlackWorkspaceConfigurationProperties> for SlackWorkspaceConfiguration {
    fn from(properties: SlackWorkspaceConfigurationProperties) -> SlackWorkspaceConfiguration {
        SlackWorkspaceConfiguration { properties }
    }
}
