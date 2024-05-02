//! Types for the `Lex` service.

/// The [`AWS::Lex::Bot`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lex-bot.html) resource type.
#[derive(Debug, Default)]
pub struct Bot {
    properties: BotProperties
}

/// Properties for the `Bot` resource.
#[derive(Debug, Default)]
pub struct BotProperties {
    /// Property [`AutoBuildBotLocales`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lex-bot.html#cfn-lex-bot-autobuildbotlocales).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub auto_build_bot_locales: Option<::Value<bool>>,
    /// Property [`BotFileS3Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lex-bot.html#cfn-lex-bot-botfiles3location).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub bot_file_s3_location: Option<::Value<self::bot::S3Location>>,
    /// Property [`BotLocales`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lex-bot.html#cfn-lex-bot-botlocales).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub bot_locales: Option<::ValueList<self::bot::BotLocale>>,
    /// Property [`BotTags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lex-bot.html#cfn-lex-bot-bottags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub bot_tags: Option<::ValueList<::Tag>>,
    /// Property [`DataPrivacy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lex-bot.html#cfn-lex-bot-dataprivacy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub data_privacy: ::Value<self::bot::DataPrivacy>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lex-bot.html#cfn-lex-bot-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`IdleSessionTTLInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lex-bot.html#cfn-lex-bot-idlesessionttlinseconds).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub idle_session_ttl_in_seconds: ::Value<u32>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lex-bot.html#cfn-lex-bot-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lex-bot.html#cfn-lex-bot-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: ::Value<String>,
    /// Property [`TestBotAliasSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lex-bot.html#cfn-lex-bot-testbotaliassettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub test_bot_alias_settings: Option<::Value<self::bot::TestBotAliasSettings>>,
    /// Property [`TestBotAliasTags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lex-bot.html#cfn-lex-bot-testbotaliastags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub test_bot_alias_tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for BotProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref auto_build_bot_locales) = self.auto_build_bot_locales {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoBuildBotLocales", auto_build_bot_locales)?;
        }
        if let Some(ref bot_file_s3_location) = self.bot_file_s3_location {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BotFileS3Location", bot_file_s3_location)?;
        }
        if let Some(ref bot_locales) = self.bot_locales {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BotLocales", bot_locales)?;
        }
        if let Some(ref bot_tags) = self.bot_tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BotTags", bot_tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataPrivacy", &self.data_privacy)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdleSessionTTLInSeconds", &self.idle_session_ttl_in_seconds)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        if let Some(ref test_bot_alias_settings) = self.test_bot_alias_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TestBotAliasSettings", test_bot_alias_settings)?;
        }
        if let Some(ref test_bot_alias_tags) = self.test_bot_alias_tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TestBotAliasTags", test_bot_alias_tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for BotProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<BotProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = BotProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type BotProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut auto_build_bot_locales: Option<::Value<bool>> = None;
                let mut bot_file_s3_location: Option<::Value<self::bot::S3Location>> = None;
                let mut bot_locales: Option<::ValueList<self::bot::BotLocale>> = None;
                let mut bot_tags: Option<::ValueList<::Tag>> = None;
                let mut data_privacy: Option<::Value<self::bot::DataPrivacy>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut idle_session_ttl_in_seconds: Option<::Value<u32>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut test_bot_alias_settings: Option<::Value<self::bot::TestBotAliasSettings>> = None;
                let mut test_bot_alias_tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AutoBuildBotLocales" => {
                            auto_build_bot_locales = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BotFileS3Location" => {
                            bot_file_s3_location = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BotLocales" => {
                            bot_locales = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BotTags" => {
                            bot_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DataPrivacy" => {
                            data_privacy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IdleSessionTTLInSeconds" => {
                            idle_session_ttl_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TestBotAliasSettings" => {
                            test_bot_alias_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TestBotAliasTags" => {
                            test_bot_alias_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(BotProperties {
                    auto_build_bot_locales: auto_build_bot_locales,
                    bot_file_s3_location: bot_file_s3_location,
                    bot_locales: bot_locales,
                    bot_tags: bot_tags,
                    data_privacy: data_privacy.ok_or(::serde::de::Error::missing_field("DataPrivacy"))?,
                    description: description,
                    idle_session_ttl_in_seconds: idle_session_ttl_in_seconds.ok_or(::serde::de::Error::missing_field("IdleSessionTTLInSeconds"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    test_bot_alias_settings: test_bot_alias_settings,
                    test_bot_alias_tags: test_bot_alias_tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Bot {
    type Properties = BotProperties;
    const TYPE: &'static str = "AWS::Lex::Bot";
    fn properties(&self) -> &BotProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut BotProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Bot {}

impl From<BotProperties> for Bot {
    fn from(properties: BotProperties) -> Bot {
        Bot { properties }
    }
}

/// The [`AWS::Lex::BotAlias`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lex-botalias.html) resource type.
#[derive(Debug, Default)]
pub struct BotAlias {
    properties: BotAliasProperties
}

/// Properties for the `BotAlias` resource.
#[derive(Debug, Default)]
pub struct BotAliasProperties {
    /// Property [`BotAliasLocaleSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lex-botalias.html#cfn-lex-botalias-botaliaslocalesettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub bot_alias_locale_settings: Option<::ValueList<self::bot_alias::BotAliasLocaleSettingsItem>>,
    /// Property [`BotAliasName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lex-botalias.html#cfn-lex-botalias-botaliasname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub bot_alias_name: ::Value<String>,
    /// Property [`BotAliasTags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lex-botalias.html#cfn-lex-botalias-botaliastags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub bot_alias_tags: Option<::ValueList<::Tag>>,
    /// Property [`BotId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lex-botalias.html#cfn-lex-botalias-botid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub bot_id: ::Value<String>,
    /// Property [`BotVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lex-botalias.html#cfn-lex-botalias-botversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub bot_version: Option<::Value<String>>,
    /// Property [`ConversationLogSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lex-botalias.html#cfn-lex-botalias-conversationlogsettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub conversation_log_settings: Option<::Value<self::bot_alias::ConversationLogSettings>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lex-botalias.html#cfn-lex-botalias-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`SentimentAnalysisSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lex-botalias.html#cfn-lex-botalias-sentimentanalysissettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sentiment_analysis_settings: Option<::Value<self::bot_alias::SentimentAnalysisSettings>>,
}

impl ::serde::Serialize for BotAliasProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref bot_alias_locale_settings) = self.bot_alias_locale_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BotAliasLocaleSettings", bot_alias_locale_settings)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BotAliasName", &self.bot_alias_name)?;
        if let Some(ref bot_alias_tags) = self.bot_alias_tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BotAliasTags", bot_alias_tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BotId", &self.bot_id)?;
        if let Some(ref bot_version) = self.bot_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BotVersion", bot_version)?;
        }
        if let Some(ref conversation_log_settings) = self.conversation_log_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConversationLogSettings", conversation_log_settings)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref sentiment_analysis_settings) = self.sentiment_analysis_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SentimentAnalysisSettings", sentiment_analysis_settings)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for BotAliasProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<BotAliasProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = BotAliasProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type BotAliasProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut bot_alias_locale_settings: Option<::ValueList<self::bot_alias::BotAliasLocaleSettingsItem>> = None;
                let mut bot_alias_name: Option<::Value<String>> = None;
                let mut bot_alias_tags: Option<::ValueList<::Tag>> = None;
                let mut bot_id: Option<::Value<String>> = None;
                let mut bot_version: Option<::Value<String>> = None;
                let mut conversation_log_settings: Option<::Value<self::bot_alias::ConversationLogSettings>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut sentiment_analysis_settings: Option<::Value<self::bot_alias::SentimentAnalysisSettings>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "BotAliasLocaleSettings" => {
                            bot_alias_locale_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BotAliasName" => {
                            bot_alias_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BotAliasTags" => {
                            bot_alias_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BotId" => {
                            bot_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BotVersion" => {
                            bot_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ConversationLogSettings" => {
                            conversation_log_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SentimentAnalysisSettings" => {
                            sentiment_analysis_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(BotAliasProperties {
                    bot_alias_locale_settings: bot_alias_locale_settings,
                    bot_alias_name: bot_alias_name.ok_or(::serde::de::Error::missing_field("BotAliasName"))?,
                    bot_alias_tags: bot_alias_tags,
                    bot_id: bot_id.ok_or(::serde::de::Error::missing_field("BotId"))?,
                    bot_version: bot_version,
                    conversation_log_settings: conversation_log_settings,
                    description: description,
                    sentiment_analysis_settings: sentiment_analysis_settings,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for BotAlias {
    type Properties = BotAliasProperties;
    const TYPE: &'static str = "AWS::Lex::BotAlias";
    fn properties(&self) -> &BotAliasProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut BotAliasProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for BotAlias {}

impl From<BotAliasProperties> for BotAlias {
    fn from(properties: BotAliasProperties) -> BotAlias {
        BotAlias { properties }
    }
}

/// The [`AWS::Lex::BotVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lex-botversion.html) resource type.
#[derive(Debug, Default)]
pub struct BotVersion {
    properties: BotVersionProperties
}

/// Properties for the `BotVersion` resource.
#[derive(Debug, Default)]
pub struct BotVersionProperties {
    /// Property [`BotId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lex-botversion.html#cfn-lex-botversion-botid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub bot_id: ::Value<String>,
    /// Property [`BotVersionLocaleSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lex-botversion.html#cfn-lex-botversion-botversionlocalespecification).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub bot_version_locale_specification: ::ValueList<self::bot_version::BotVersionLocaleSpecification>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lex-botversion.html#cfn-lex-botversion-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
}

impl ::serde::Serialize for BotVersionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BotId", &self.bot_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BotVersionLocaleSpecification", &self.bot_version_locale_specification)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for BotVersionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<BotVersionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = BotVersionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type BotVersionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut bot_id: Option<::Value<String>> = None;
                let mut bot_version_locale_specification: Option<::ValueList<self::bot_version::BotVersionLocaleSpecification>> = None;
                let mut description: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "BotId" => {
                            bot_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BotVersionLocaleSpecification" => {
                            bot_version_locale_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(BotVersionProperties {
                    bot_id: bot_id.ok_or(::serde::de::Error::missing_field("BotId"))?,
                    bot_version_locale_specification: bot_version_locale_specification.ok_or(::serde::de::Error::missing_field("BotVersionLocaleSpecification"))?,
                    description: description,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for BotVersion {
    type Properties = BotVersionProperties;
    const TYPE: &'static str = "AWS::Lex::BotVersion";
    fn properties(&self) -> &BotVersionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut BotVersionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for BotVersion {}

impl From<BotVersionProperties> for BotVersion {
    fn from(properties: BotVersionProperties) -> BotVersion {
        BotVersion { properties }
    }
}

/// The [`AWS::Lex::ResourcePolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lex-resourcepolicy.html) resource type.
#[derive(Debug, Default)]
pub struct ResourcePolicy {
    properties: ResourcePolicyProperties
}

/// Properties for the `ResourcePolicy` resource.
#[derive(Debug, Default)]
pub struct ResourcePolicyProperties {
    /// Property [`Policy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lex-resourcepolicy.html#cfn-lex-resourcepolicy-policy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub policy: ::Value<::json::Value>,
    /// Property [`ResourceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lex-resourcepolicy.html#cfn-lex-resourcepolicy-resourcearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub resource_arn: ::Value<String>,
}

impl ::serde::Serialize for ResourcePolicyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Policy", &self.policy)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceArn", &self.resource_arn)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ResourcePolicyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourcePolicyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ResourcePolicyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ResourcePolicyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut policy: Option<::Value<::json::Value>> = None;
                let mut resource_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Policy" => {
                            policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourceArn" => {
                            resource_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ResourcePolicyProperties {
                    policy: policy.ok_or(::serde::de::Error::missing_field("Policy"))?,
                    resource_arn: resource_arn.ok_or(::serde::de::Error::missing_field("ResourceArn"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ResourcePolicy {
    type Properties = ResourcePolicyProperties;
    const TYPE: &'static str = "AWS::Lex::ResourcePolicy";
    fn properties(&self) -> &ResourcePolicyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ResourcePolicyProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ResourcePolicy {}

impl From<ResourcePolicyProperties> for ResourcePolicy {
    fn from(properties: ResourcePolicyProperties) -> ResourcePolicy {
        ResourcePolicy { properties }
    }
}

pub mod bot {
    //! Property types for the `Bot` resource.

    /// The [`AWS::Lex::Bot.AdvancedRecognitionSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-advancedrecognitionsetting.html) property type.
    #[derive(Debug, Default)]
    pub struct AdvancedRecognitionSetting {
        /// Property [`AudioRecognitionStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-advancedrecognitionsetting.html#cfn-lex-bot-advancedrecognitionsetting-audiorecognitionstrategy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub audio_recognition_strategy: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AdvancedRecognitionSetting {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref audio_recognition_strategy) = self.audio_recognition_strategy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AudioRecognitionStrategy", audio_recognition_strategy)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AdvancedRecognitionSetting {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AdvancedRecognitionSetting, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AdvancedRecognitionSetting;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AdvancedRecognitionSetting")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut audio_recognition_strategy: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AudioRecognitionStrategy" => {
                                audio_recognition_strategy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AdvancedRecognitionSetting {
                        audio_recognition_strategy: audio_recognition_strategy,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.AllowedInputTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-allowedinputtypes.html) property type.
    #[derive(Debug, Default)]
    pub struct AllowedInputTypes {
        /// Property [`AllowAudioInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-allowedinputtypes.html#cfn-lex-bot-allowedinputtypes-allowaudioinput).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allow_audio_input: ::Value<bool>,
        /// Property [`AllowDTMFInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-allowedinputtypes.html#cfn-lex-bot-allowedinputtypes-allowdtmfinput).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allow_dtmf_input: ::Value<bool>,
    }

    impl ::codec::SerializeValue for AllowedInputTypes {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowAudioInput", &self.allow_audio_input)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowDTMFInput", &self.allow_dtmf_input)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AllowedInputTypes {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AllowedInputTypes, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AllowedInputTypes;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AllowedInputTypes")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allow_audio_input: Option<::Value<bool>> = None;
                    let mut allow_dtmf_input: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllowAudioInput" => {
                                allow_audio_input = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AllowDTMFInput" => {
                                allow_dtmf_input = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AllowedInputTypes {
                        allow_audio_input: allow_audio_input.ok_or(::serde::de::Error::missing_field("AllowAudioInput"))?,
                        allow_dtmf_input: allow_dtmf_input.ok_or(::serde::de::Error::missing_field("AllowDTMFInput"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.AudioAndDTMFInputSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-audioanddtmfinputspecification.html) property type.
    #[derive(Debug, Default)]
    pub struct AudioAndDTMFInputSpecification {
        /// Property [`AudioSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-audioanddtmfinputspecification.html#cfn-lex-bot-audioanddtmfinputspecification-audiospecification).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub audio_specification: Option<::Value<AudioSpecification>>,
        /// Property [`DTMFSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-audioanddtmfinputspecification.html#cfn-lex-bot-audioanddtmfinputspecification-dtmfspecification).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dtmf_specification: Option<::Value<DTMFSpecification>>,
        /// Property [`StartTimeoutMs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-audioanddtmfinputspecification.html#cfn-lex-bot-audioanddtmfinputspecification-starttimeoutms).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub start_timeout_ms: ::Value<u32>,
    }

    impl ::codec::SerializeValue for AudioAndDTMFInputSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref audio_specification) = self.audio_specification {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AudioSpecification", audio_specification)?;
            }
            if let Some(ref dtmf_specification) = self.dtmf_specification {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DTMFSpecification", dtmf_specification)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartTimeoutMs", &self.start_timeout_ms)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AudioAndDTMFInputSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AudioAndDTMFInputSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AudioAndDTMFInputSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AudioAndDTMFInputSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut audio_specification: Option<::Value<AudioSpecification>> = None;
                    let mut dtmf_specification: Option<::Value<DTMFSpecification>> = None;
                    let mut start_timeout_ms: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AudioSpecification" => {
                                audio_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DTMFSpecification" => {
                                dtmf_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StartTimeoutMs" => {
                                start_timeout_ms = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AudioAndDTMFInputSpecification {
                        audio_specification: audio_specification,
                        dtmf_specification: dtmf_specification,
                        start_timeout_ms: start_timeout_ms.ok_or(::serde::de::Error::missing_field("StartTimeoutMs"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.AudioLogDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-audiologdestination.html) property type.
    #[derive(Debug, Default)]
    pub struct AudioLogDestination {
        /// Property [`S3Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-audiologdestination.html#cfn-lex-bot-audiologdestination-s3bucket).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_bucket: ::Value<S3BucketLogDestination>,
    }

    impl ::codec::SerializeValue for AudioLogDestination {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Bucket", &self.s3_bucket)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AudioLogDestination {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AudioLogDestination, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AudioLogDestination;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AudioLogDestination")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut s3_bucket: Option<::Value<S3BucketLogDestination>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3Bucket" => {
                                s3_bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AudioLogDestination {
                        s3_bucket: s3_bucket.ok_or(::serde::de::Error::missing_field("S3Bucket"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.AudioLogSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-audiologsetting.html) property type.
    #[derive(Debug, Default)]
    pub struct AudioLogSetting {
        /// Property [`Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-audiologsetting.html#cfn-lex-bot-audiologsetting-destination).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination: ::Value<AudioLogDestination>,
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-audiologsetting.html#cfn-lex-bot-audiologsetting-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: ::Value<bool>,
    }

    impl ::codec::SerializeValue for AudioLogSetting {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Destination", &self.destination)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AudioLogSetting {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AudioLogSetting, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AudioLogSetting;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AudioLogSetting")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destination: Option<::Value<AudioLogDestination>> = None;
                    let mut enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Destination" => {
                                destination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AudioLogSetting {
                        destination: destination.ok_or(::serde::de::Error::missing_field("Destination"))?,
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.AudioSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-audiospecification.html) property type.
    #[derive(Debug, Default)]
    pub struct AudioSpecification {
        /// Property [`EndTimeoutMs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-audiospecification.html#cfn-lex-bot-audiospecification-endtimeoutms).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub end_timeout_ms: ::Value<u32>,
        /// Property [`MaxLengthMs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-audiospecification.html#cfn-lex-bot-audiospecification-maxlengthms).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_length_ms: ::Value<u32>,
    }

    impl ::codec::SerializeValue for AudioSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndTimeoutMs", &self.end_timeout_ms)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxLengthMs", &self.max_length_ms)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AudioSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AudioSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AudioSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AudioSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut end_timeout_ms: Option<::Value<u32>> = None;
                    let mut max_length_ms: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EndTimeoutMs" => {
                                end_timeout_ms = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxLengthMs" => {
                                max_length_ms = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AudioSpecification {
                        end_timeout_ms: end_timeout_ms.ok_or(::serde::de::Error::missing_field("EndTimeoutMs"))?,
                        max_length_ms: max_length_ms.ok_or(::serde::de::Error::missing_field("MaxLengthMs"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.BotAliasLocaleSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-botaliaslocalesettings.html) property type.
    #[derive(Debug, Default)]
    pub struct BotAliasLocaleSettings {
        /// Property [`CodeHookSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-botaliaslocalesettings.html#cfn-lex-bot-botaliaslocalesettings-codehookspecification).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub code_hook_specification: Option<::Value<CodeHookSpecification>>,
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-botaliaslocalesettings.html#cfn-lex-bot-botaliaslocalesettings-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: ::Value<bool>,
    }

    impl ::codec::SerializeValue for BotAliasLocaleSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref code_hook_specification) = self.code_hook_specification {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CodeHookSpecification", code_hook_specification)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BotAliasLocaleSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BotAliasLocaleSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BotAliasLocaleSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BotAliasLocaleSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut code_hook_specification: Option<::Value<CodeHookSpecification>> = None;
                    let mut enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CodeHookSpecification" => {
                                code_hook_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BotAliasLocaleSettings {
                        code_hook_specification: code_hook_specification,
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.BotAliasLocaleSettingsItem`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-botaliaslocalesettingsitem.html) property type.
    #[derive(Debug, Default)]
    pub struct BotAliasLocaleSettingsItem {
        /// Property [`BotAliasLocaleSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-botaliaslocalesettingsitem.html#cfn-lex-bot-botaliaslocalesettingsitem-botaliaslocalesetting).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bot_alias_locale_setting: ::Value<BotAliasLocaleSettings>,
        /// Property [`LocaleId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-botaliaslocalesettingsitem.html#cfn-lex-bot-botaliaslocalesettingsitem-localeid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub locale_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for BotAliasLocaleSettingsItem {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BotAliasLocaleSetting", &self.bot_alias_locale_setting)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LocaleId", &self.locale_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BotAliasLocaleSettingsItem {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BotAliasLocaleSettingsItem, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BotAliasLocaleSettingsItem;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BotAliasLocaleSettingsItem")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bot_alias_locale_setting: Option<::Value<BotAliasLocaleSettings>> = None;
                    let mut locale_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BotAliasLocaleSetting" => {
                                bot_alias_locale_setting = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LocaleId" => {
                                locale_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BotAliasLocaleSettingsItem {
                        bot_alias_locale_setting: bot_alias_locale_setting.ok_or(::serde::de::Error::missing_field("BotAliasLocaleSetting"))?,
                        locale_id: locale_id.ok_or(::serde::de::Error::missing_field("LocaleId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.BotLocale`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-botlocale.html) property type.
    #[derive(Debug, Default)]
    pub struct BotLocale {
        /// Property [`CustomVocabulary`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-botlocale.html#cfn-lex-bot-botlocale-customvocabulary).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_vocabulary: Option<::Value<CustomVocabulary>>,
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-botlocale.html#cfn-lex-bot-botlocale-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`Intents`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-botlocale.html#cfn-lex-bot-botlocale-intents).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub intents: Option<::ValueList<Intent>>,
        /// Property [`LocaleId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-botlocale.html#cfn-lex-bot-botlocale-localeid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub locale_id: ::Value<String>,
        /// Property [`NluConfidenceThreshold`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-botlocale.html#cfn-lex-bot-botlocale-nluconfidencethreshold).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub nlu_confidence_threshold: ::Value<f64>,
        /// Property [`SlotTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-botlocale.html#cfn-lex-bot-botlocale-slottypes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub slot_types: Option<::ValueList<SlotType>>,
        /// Property [`VoiceSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-botlocale.html#cfn-lex-bot-botlocale-voicesettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub voice_settings: Option<::Value<VoiceSettings>>,
    }

    impl ::codec::SerializeValue for BotLocale {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref custom_vocabulary) = self.custom_vocabulary {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomVocabulary", custom_vocabulary)?;
            }
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            if let Some(ref intents) = self.intents {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Intents", intents)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LocaleId", &self.locale_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NluConfidenceThreshold", &self.nlu_confidence_threshold)?;
            if let Some(ref slot_types) = self.slot_types {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SlotTypes", slot_types)?;
            }
            if let Some(ref voice_settings) = self.voice_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VoiceSettings", voice_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BotLocale {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BotLocale, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BotLocale;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BotLocale")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut custom_vocabulary: Option<::Value<CustomVocabulary>> = None;
                    let mut description: Option<::Value<String>> = None;
                    let mut intents: Option<::ValueList<Intent>> = None;
                    let mut locale_id: Option<::Value<String>> = None;
                    let mut nlu_confidence_threshold: Option<::Value<f64>> = None;
                    let mut slot_types: Option<::ValueList<SlotType>> = None;
                    let mut voice_settings: Option<::Value<VoiceSettings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CustomVocabulary" => {
                                custom_vocabulary = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Intents" => {
                                intents = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LocaleId" => {
                                locale_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NluConfidenceThreshold" => {
                                nlu_confidence_threshold = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SlotTypes" => {
                                slot_types = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VoiceSettings" => {
                                voice_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BotLocale {
                        custom_vocabulary: custom_vocabulary,
                        description: description,
                        intents: intents,
                        locale_id: locale_id.ok_or(::serde::de::Error::missing_field("LocaleId"))?,
                        nlu_confidence_threshold: nlu_confidence_threshold.ok_or(::serde::de::Error::missing_field("NluConfidenceThreshold"))?,
                        slot_types: slot_types,
                        voice_settings: voice_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.Button`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-button.html) property type.
    #[derive(Debug, Default)]
    pub struct Button {
        /// Property [`Text`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-button.html#cfn-lex-bot-button-text).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-button.html#cfn-lex-bot-button-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for Button {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Text", &self.text)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Button {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Button, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Button;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Button")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut text: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Text" => {
                                text = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Button {
                        text: text.ok_or(::serde::de::Error::missing_field("Text"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.CloudWatchLogGroupLogDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-cloudwatchloggrouplogdestination.html) property type.
    #[derive(Debug, Default)]
    pub struct CloudWatchLogGroupLogDestination {
        /// Property [`CloudWatchLogGroupArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-cloudwatchloggrouplogdestination.html#cfn-lex-bot-cloudwatchloggrouplogdestination-cloudwatchloggrouparn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloud_watch_log_group_arn: ::Value<String>,
        /// Property [`LogPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-cloudwatchloggrouplogdestination.html#cfn-lex-bot-cloudwatchloggrouplogdestination-logprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_prefix: ::Value<String>,
    }

    impl ::codec::SerializeValue for CloudWatchLogGroupLogDestination {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLogGroupArn", &self.cloud_watch_log_group_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogPrefix", &self.log_prefix)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CloudWatchLogGroupLogDestination {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CloudWatchLogGroupLogDestination, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CloudWatchLogGroupLogDestination;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CloudWatchLogGroupLogDestination")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cloud_watch_log_group_arn: Option<::Value<String>> = None;
                    let mut log_prefix: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CloudWatchLogGroupArn" => {
                                cloud_watch_log_group_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogPrefix" => {
                                log_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CloudWatchLogGroupLogDestination {
                        cloud_watch_log_group_arn: cloud_watch_log_group_arn.ok_or(::serde::de::Error::missing_field("CloudWatchLogGroupArn"))?,
                        log_prefix: log_prefix.ok_or(::serde::de::Error::missing_field("LogPrefix"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.CodeHookSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-codehookspecification.html) property type.
    #[derive(Debug, Default)]
    pub struct CodeHookSpecification {
        /// Property [`LambdaCodeHook`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-codehookspecification.html#cfn-lex-bot-codehookspecification-lambdacodehook).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lambda_code_hook: ::Value<LambdaCodeHook>,
    }

    impl ::codec::SerializeValue for CodeHookSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaCodeHook", &self.lambda_code_hook)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CodeHookSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CodeHookSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CodeHookSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CodeHookSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut lambda_code_hook: Option<::Value<LambdaCodeHook>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LambdaCodeHook" => {
                                lambda_code_hook = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CodeHookSpecification {
                        lambda_code_hook: lambda_code_hook.ok_or(::serde::de::Error::missing_field("LambdaCodeHook"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.Condition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-condition.html) property type.
    #[derive(Debug, Default)]
    pub struct Condition {
        /// Property [`ExpressionString`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-condition.html#cfn-lex-bot-condition-expressionstring).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub expression_string: ::Value<String>,
    }

    impl ::codec::SerializeValue for Condition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExpressionString", &self.expression_string)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Condition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Condition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Condition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Condition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut expression_string: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ExpressionString" => {
                                expression_string = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Condition {
                        expression_string: expression_string.ok_or(::serde::de::Error::missing_field("ExpressionString"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.ConditionalBranch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-conditionalbranch.html) property type.
    #[derive(Debug, Default)]
    pub struct ConditionalBranch {
        /// Property [`Condition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-conditionalbranch.html#cfn-lex-bot-conditionalbranch-condition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub condition: ::Value<Condition>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-conditionalbranch.html#cfn-lex-bot-conditionalbranch-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`NextStep`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-conditionalbranch.html#cfn-lex-bot-conditionalbranch-nextstep).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub next_step: ::Value<DialogState>,
        /// Property [`Response`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-conditionalbranch.html#cfn-lex-bot-conditionalbranch-response).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub response: Option<::Value<ResponseSpecification>>,
    }

    impl ::codec::SerializeValue for ConditionalBranch {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Condition", &self.condition)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NextStep", &self.next_step)?;
            if let Some(ref response) = self.response {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Response", response)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConditionalBranch {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConditionalBranch, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConditionalBranch;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConditionalBranch")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut condition: Option<::Value<Condition>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut next_step: Option<::Value<DialogState>> = None;
                    let mut response: Option<::Value<ResponseSpecification>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Condition" => {
                                condition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NextStep" => {
                                next_step = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Response" => {
                                response = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConditionalBranch {
                        condition: condition.ok_or(::serde::de::Error::missing_field("Condition"))?,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        next_step: next_step.ok_or(::serde::de::Error::missing_field("NextStep"))?,
                        response: response,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.ConditionalSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-conditionalspecification.html) property type.
    #[derive(Debug, Default)]
    pub struct ConditionalSpecification {
        /// Property [`ConditionalBranches`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-conditionalspecification.html#cfn-lex-bot-conditionalspecification-conditionalbranches).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub conditional_branches: ::ValueList<ConditionalBranch>,
        /// Property [`DefaultBranch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-conditionalspecification.html#cfn-lex-bot-conditionalspecification-defaultbranch).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_branch: ::Value<DefaultConditionalBranch>,
        /// Property [`IsActive`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-conditionalspecification.html#cfn-lex-bot-conditionalspecification-isactive).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub is_active: ::Value<bool>,
    }

    impl ::codec::SerializeValue for ConditionalSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConditionalBranches", &self.conditional_branches)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultBranch", &self.default_branch)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsActive", &self.is_active)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConditionalSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConditionalSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConditionalSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConditionalSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut conditional_branches: Option<::ValueList<ConditionalBranch>> = None;
                    let mut default_branch: Option<::Value<DefaultConditionalBranch>> = None;
                    let mut is_active: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConditionalBranches" => {
                                conditional_branches = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DefaultBranch" => {
                                default_branch = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IsActive" => {
                                is_active = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConditionalSpecification {
                        conditional_branches: conditional_branches.ok_or(::serde::de::Error::missing_field("ConditionalBranches"))?,
                        default_branch: default_branch.ok_or(::serde::de::Error::missing_field("DefaultBranch"))?,
                        is_active: is_active.ok_or(::serde::de::Error::missing_field("IsActive"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.ConversationLogSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-conversationlogsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct ConversationLogSettings {
        /// Property [`AudioLogSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-conversationlogsettings.html#cfn-lex-bot-conversationlogsettings-audiologsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub audio_log_settings: Option<::ValueList<AudioLogSetting>>,
        /// Property [`TextLogSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-conversationlogsettings.html#cfn-lex-bot-conversationlogsettings-textlogsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text_log_settings: Option<::ValueList<TextLogSetting>>,
    }

    impl ::codec::SerializeValue for ConversationLogSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref audio_log_settings) = self.audio_log_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AudioLogSettings", audio_log_settings)?;
            }
            if let Some(ref text_log_settings) = self.text_log_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextLogSettings", text_log_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConversationLogSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConversationLogSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConversationLogSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConversationLogSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut audio_log_settings: Option<::ValueList<AudioLogSetting>> = None;
                    let mut text_log_settings: Option<::ValueList<TextLogSetting>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AudioLogSettings" => {
                                audio_log_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TextLogSettings" => {
                                text_log_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConversationLogSettings {
                        audio_log_settings: audio_log_settings,
                        text_log_settings: text_log_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.CustomPayload`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-custompayload.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomPayload {
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-custompayload.html#cfn-lex-bot-custompayload-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for CustomPayload {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomPayload {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomPayload, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomPayload;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomPayload")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomPayload {
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.CustomVocabulary`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-customvocabulary.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomVocabulary {
        /// Property [`CustomVocabularyItems`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-customvocabulary.html#cfn-lex-bot-customvocabulary-customvocabularyitems).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_vocabulary_items: ::ValueList<CustomVocabularyItem>,
    }

    impl ::codec::SerializeValue for CustomVocabulary {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomVocabularyItems", &self.custom_vocabulary_items)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomVocabulary {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomVocabulary, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomVocabulary;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomVocabulary")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut custom_vocabulary_items: Option<::ValueList<CustomVocabularyItem>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CustomVocabularyItems" => {
                                custom_vocabulary_items = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomVocabulary {
                        custom_vocabulary_items: custom_vocabulary_items.ok_or(::serde::de::Error::missing_field("CustomVocabularyItems"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.CustomVocabularyItem`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-customvocabularyitem.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomVocabularyItem {
        /// Property [`DisplayAs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-customvocabularyitem.html#cfn-lex-bot-customvocabularyitem-displayas).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub display_as: Option<::Value<String>>,
        /// Property [`Phrase`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-customvocabularyitem.html#cfn-lex-bot-customvocabularyitem-phrase).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub phrase: ::Value<String>,
        /// Property [`Weight`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-customvocabularyitem.html#cfn-lex-bot-customvocabularyitem-weight).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub weight: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for CustomVocabularyItem {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref display_as) = self.display_as {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisplayAs", display_as)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Phrase", &self.phrase)?;
            if let Some(ref weight) = self.weight {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Weight", weight)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomVocabularyItem {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomVocabularyItem, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomVocabularyItem;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomVocabularyItem")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut display_as: Option<::Value<String>> = None;
                    let mut phrase: Option<::Value<String>> = None;
                    let mut weight: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DisplayAs" => {
                                display_as = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Phrase" => {
                                phrase = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Weight" => {
                                weight = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomVocabularyItem {
                        display_as: display_as,
                        phrase: phrase.ok_or(::serde::de::Error::missing_field("Phrase"))?,
                        weight: weight,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.DTMFSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-dtmfspecification.html) property type.
    #[derive(Debug, Default)]
    pub struct DTMFSpecification {
        /// Property [`DeletionCharacter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-dtmfspecification.html#cfn-lex-bot-dtmfspecification-deletioncharacter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub deletion_character: ::Value<String>,
        /// Property [`EndCharacter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-dtmfspecification.html#cfn-lex-bot-dtmfspecification-endcharacter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub end_character: ::Value<String>,
        /// Property [`EndTimeoutMs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-dtmfspecification.html#cfn-lex-bot-dtmfspecification-endtimeoutms).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub end_timeout_ms: ::Value<u32>,
        /// Property [`MaxLength`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-dtmfspecification.html#cfn-lex-bot-dtmfspecification-maxlength).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_length: ::Value<u32>,
    }

    impl ::codec::SerializeValue for DTMFSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeletionCharacter", &self.deletion_character)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndCharacter", &self.end_character)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndTimeoutMs", &self.end_timeout_ms)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxLength", &self.max_length)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DTMFSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DTMFSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DTMFSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DTMFSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut deletion_character: Option<::Value<String>> = None;
                    let mut end_character: Option<::Value<String>> = None;
                    let mut end_timeout_ms: Option<::Value<u32>> = None;
                    let mut max_length: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DeletionCharacter" => {
                                deletion_character = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EndCharacter" => {
                                end_character = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EndTimeoutMs" => {
                                end_timeout_ms = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxLength" => {
                                max_length = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DTMFSpecification {
                        deletion_character: deletion_character.ok_or(::serde::de::Error::missing_field("DeletionCharacter"))?,
                        end_character: end_character.ok_or(::serde::de::Error::missing_field("EndCharacter"))?,
                        end_timeout_ms: end_timeout_ms.ok_or(::serde::de::Error::missing_field("EndTimeoutMs"))?,
                        max_length: max_length.ok_or(::serde::de::Error::missing_field("MaxLength"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.DataPrivacy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-dataprivacy.html) property type.
    #[derive(Debug, Default)]
    pub struct DataPrivacy {
        /// Property [`ChildDirected`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-dataprivacy.html#cfn-lex-bot-dataprivacy-childdirected).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub child_directed: ::Value<bool>,
    }

    impl ::codec::SerializeValue for DataPrivacy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChildDirected", &self.child_directed)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataPrivacy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataPrivacy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataPrivacy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataPrivacy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut child_directed: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ChildDirected" => {
                                child_directed = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataPrivacy {
                        child_directed: child_directed.ok_or(::serde::de::Error::missing_field("ChildDirected"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.DefaultConditionalBranch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-defaultconditionalbranch.html) property type.
    #[derive(Debug, Default)]
    pub struct DefaultConditionalBranch {
        /// Property [`NextStep`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-defaultconditionalbranch.html#cfn-lex-bot-defaultconditionalbranch-nextstep).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub next_step: Option<::Value<DialogState>>,
        /// Property [`Response`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-defaultconditionalbranch.html#cfn-lex-bot-defaultconditionalbranch-response).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub response: Option<::Value<ResponseSpecification>>,
    }

    impl ::codec::SerializeValue for DefaultConditionalBranch {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref next_step) = self.next_step {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NextStep", next_step)?;
            }
            if let Some(ref response) = self.response {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Response", response)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DefaultConditionalBranch {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DefaultConditionalBranch, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DefaultConditionalBranch;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DefaultConditionalBranch")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut next_step: Option<::Value<DialogState>> = None;
                    let mut response: Option<::Value<ResponseSpecification>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "NextStep" => {
                                next_step = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Response" => {
                                response = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DefaultConditionalBranch {
                        next_step: next_step,
                        response: response,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.DialogAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-dialogaction.html) property type.
    #[derive(Debug, Default)]
    pub struct DialogAction {
        /// Property [`SlotToElicit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-dialogaction.html#cfn-lex-bot-dialogaction-slottoelicit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub slot_to_elicit: Option<::Value<String>>,
        /// Property [`SuppressNextMessage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-dialogaction.html#cfn-lex-bot-dialogaction-suppressnextmessage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub suppress_next_message: Option<::Value<bool>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-dialogaction.html#cfn-lex-bot-dialogaction-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for DialogAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref slot_to_elicit) = self.slot_to_elicit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SlotToElicit", slot_to_elicit)?;
            }
            if let Some(ref suppress_next_message) = self.suppress_next_message {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SuppressNextMessage", suppress_next_message)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DialogAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DialogAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DialogAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DialogAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut slot_to_elicit: Option<::Value<String>> = None;
                    let mut suppress_next_message: Option<::Value<bool>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SlotToElicit" => {
                                slot_to_elicit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SuppressNextMessage" => {
                                suppress_next_message = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DialogAction {
                        slot_to_elicit: slot_to_elicit,
                        suppress_next_message: suppress_next_message,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.DialogCodeHookInvocationSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-dialogcodehookinvocationsetting.html) property type.
    #[derive(Debug, Default)]
    pub struct DialogCodeHookInvocationSetting {
        /// Property [`EnableCodeHookInvocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-dialogcodehookinvocationsetting.html#cfn-lex-bot-dialogcodehookinvocationsetting-enablecodehookinvocation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable_code_hook_invocation: ::Value<bool>,
        /// Property [`InvocationLabel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-dialogcodehookinvocationsetting.html#cfn-lex-bot-dialogcodehookinvocationsetting-invocationlabel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub invocation_label: Option<::Value<String>>,
        /// Property [`IsActive`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-dialogcodehookinvocationsetting.html#cfn-lex-bot-dialogcodehookinvocationsetting-isactive).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub is_active: ::Value<bool>,
        /// Property [`PostCodeHookSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-dialogcodehookinvocationsetting.html#cfn-lex-bot-dialogcodehookinvocationsetting-postcodehookspecification).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub post_code_hook_specification: ::Value<PostDialogCodeHookInvocationSpecification>,
    }

    impl ::codec::SerializeValue for DialogCodeHookInvocationSetting {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableCodeHookInvocation", &self.enable_code_hook_invocation)?;
            if let Some(ref invocation_label) = self.invocation_label {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InvocationLabel", invocation_label)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsActive", &self.is_active)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PostCodeHookSpecification", &self.post_code_hook_specification)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DialogCodeHookInvocationSetting {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DialogCodeHookInvocationSetting, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DialogCodeHookInvocationSetting;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DialogCodeHookInvocationSetting")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enable_code_hook_invocation: Option<::Value<bool>> = None;
                    let mut invocation_label: Option<::Value<String>> = None;
                    let mut is_active: Option<::Value<bool>> = None;
                    let mut post_code_hook_specification: Option<::Value<PostDialogCodeHookInvocationSpecification>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EnableCodeHookInvocation" => {
                                enable_code_hook_invocation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InvocationLabel" => {
                                invocation_label = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IsActive" => {
                                is_active = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PostCodeHookSpecification" => {
                                post_code_hook_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DialogCodeHookInvocationSetting {
                        enable_code_hook_invocation: enable_code_hook_invocation.ok_or(::serde::de::Error::missing_field("EnableCodeHookInvocation"))?,
                        invocation_label: invocation_label,
                        is_active: is_active.ok_or(::serde::de::Error::missing_field("IsActive"))?,
                        post_code_hook_specification: post_code_hook_specification.ok_or(::serde::de::Error::missing_field("PostCodeHookSpecification"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.DialogCodeHookSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-dialogcodehooksetting.html) property type.
    #[derive(Debug, Default)]
    pub struct DialogCodeHookSetting {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-dialogcodehooksetting.html#cfn-lex-bot-dialogcodehooksetting-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: ::Value<bool>,
    }

    impl ::codec::SerializeValue for DialogCodeHookSetting {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DialogCodeHookSetting {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DialogCodeHookSetting, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DialogCodeHookSetting;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DialogCodeHookSetting")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DialogCodeHookSetting {
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.DialogState`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-dialogstate.html) property type.
    #[derive(Debug, Default)]
    pub struct DialogState {
        /// Property [`DialogAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-dialogstate.html#cfn-lex-bot-dialogstate-dialogaction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dialog_action: Option<::Value<DialogAction>>,
        /// Property [`Intent`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-dialogstate.html#cfn-lex-bot-dialogstate-intent).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub intent: Option<::Value<IntentOverride>>,
        /// Property [`SessionAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-dialogstate.html#cfn-lex-bot-dialogstate-sessionattributes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub session_attributes: Option<::ValueList<SessionAttribute>>,
    }

    impl ::codec::SerializeValue for DialogState {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref dialog_action) = self.dialog_action {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DialogAction", dialog_action)?;
            }
            if let Some(ref intent) = self.intent {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Intent", intent)?;
            }
            if let Some(ref session_attributes) = self.session_attributes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SessionAttributes", session_attributes)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DialogState {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DialogState, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DialogState;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DialogState")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dialog_action: Option<::Value<DialogAction>> = None;
                    let mut intent: Option<::Value<IntentOverride>> = None;
                    let mut session_attributes: Option<::ValueList<SessionAttribute>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DialogAction" => {
                                dialog_action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Intent" => {
                                intent = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SessionAttributes" => {
                                session_attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DialogState {
                        dialog_action: dialog_action,
                        intent: intent,
                        session_attributes: session_attributes,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.ElicitationCodeHookInvocationSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-elicitationcodehookinvocationsetting.html) property type.
    #[derive(Debug, Default)]
    pub struct ElicitationCodeHookInvocationSetting {
        /// Property [`EnableCodeHookInvocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-elicitationcodehookinvocationsetting.html#cfn-lex-bot-elicitationcodehookinvocationsetting-enablecodehookinvocation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable_code_hook_invocation: ::Value<bool>,
        /// Property [`InvocationLabel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-elicitationcodehookinvocationsetting.html#cfn-lex-bot-elicitationcodehookinvocationsetting-invocationlabel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub invocation_label: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ElicitationCodeHookInvocationSetting {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableCodeHookInvocation", &self.enable_code_hook_invocation)?;
            if let Some(ref invocation_label) = self.invocation_label {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InvocationLabel", invocation_label)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ElicitationCodeHookInvocationSetting {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ElicitationCodeHookInvocationSetting, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ElicitationCodeHookInvocationSetting;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ElicitationCodeHookInvocationSetting")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enable_code_hook_invocation: Option<::Value<bool>> = None;
                    let mut invocation_label: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EnableCodeHookInvocation" => {
                                enable_code_hook_invocation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InvocationLabel" => {
                                invocation_label = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ElicitationCodeHookInvocationSetting {
                        enable_code_hook_invocation: enable_code_hook_invocation.ok_or(::serde::de::Error::missing_field("EnableCodeHookInvocation"))?,
                        invocation_label: invocation_label,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.ExternalSourceSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-externalsourcesetting.html) property type.
    #[derive(Debug, Default)]
    pub struct ExternalSourceSetting {
        /// Property [`GrammarSlotTypeSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-externalsourcesetting.html#cfn-lex-bot-externalsourcesetting-grammarslottypesetting).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub grammar_slot_type_setting: Option<::Value<GrammarSlotTypeSetting>>,
    }

    impl ::codec::SerializeValue for ExternalSourceSetting {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref grammar_slot_type_setting) = self.grammar_slot_type_setting {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GrammarSlotTypeSetting", grammar_slot_type_setting)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ExternalSourceSetting {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ExternalSourceSetting, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ExternalSourceSetting;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ExternalSourceSetting")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut grammar_slot_type_setting: Option<::Value<GrammarSlotTypeSetting>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "GrammarSlotTypeSetting" => {
                                grammar_slot_type_setting = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ExternalSourceSetting {
                        grammar_slot_type_setting: grammar_slot_type_setting,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.FulfillmentCodeHookSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-fulfillmentcodehooksetting.html) property type.
    #[derive(Debug, Default)]
    pub struct FulfillmentCodeHookSetting {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-fulfillmentcodehooksetting.html#cfn-lex-bot-fulfillmentcodehooksetting-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: ::Value<bool>,
        /// Property [`FulfillmentUpdatesSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-fulfillmentcodehooksetting.html#cfn-lex-bot-fulfillmentcodehooksetting-fulfillmentupdatesspecification).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub fulfillment_updates_specification: Option<::Value<FulfillmentUpdatesSpecification>>,
        /// Property [`IsActive`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-fulfillmentcodehooksetting.html#cfn-lex-bot-fulfillmentcodehooksetting-isactive).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub is_active: Option<::Value<bool>>,
        /// Property [`PostFulfillmentStatusSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-fulfillmentcodehooksetting.html#cfn-lex-bot-fulfillmentcodehooksetting-postfulfillmentstatusspecification).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub post_fulfillment_status_specification: Option<::Value<PostFulfillmentStatusSpecification>>,
    }

    impl ::codec::SerializeValue for FulfillmentCodeHookSetting {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            if let Some(ref fulfillment_updates_specification) = self.fulfillment_updates_specification {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FulfillmentUpdatesSpecification", fulfillment_updates_specification)?;
            }
            if let Some(ref is_active) = self.is_active {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsActive", is_active)?;
            }
            if let Some(ref post_fulfillment_status_specification) = self.post_fulfillment_status_specification {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PostFulfillmentStatusSpecification", post_fulfillment_status_specification)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FulfillmentCodeHookSetting {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FulfillmentCodeHookSetting, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FulfillmentCodeHookSetting;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FulfillmentCodeHookSetting")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut fulfillment_updates_specification: Option<::Value<FulfillmentUpdatesSpecification>> = None;
                    let mut is_active: Option<::Value<bool>> = None;
                    let mut post_fulfillment_status_specification: Option<::Value<PostFulfillmentStatusSpecification>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FulfillmentUpdatesSpecification" => {
                                fulfillment_updates_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IsActive" => {
                                is_active = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PostFulfillmentStatusSpecification" => {
                                post_fulfillment_status_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FulfillmentCodeHookSetting {
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                        fulfillment_updates_specification: fulfillment_updates_specification,
                        is_active: is_active,
                        post_fulfillment_status_specification: post_fulfillment_status_specification,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.FulfillmentStartResponseSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-fulfillmentstartresponsespecification.html) property type.
    #[derive(Debug, Default)]
    pub struct FulfillmentStartResponseSpecification {
        /// Property [`AllowInterrupt`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-fulfillmentstartresponsespecification.html#cfn-lex-bot-fulfillmentstartresponsespecification-allowinterrupt).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allow_interrupt: Option<::Value<bool>>,
        /// Property [`DelayInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-fulfillmentstartresponsespecification.html#cfn-lex-bot-fulfillmentstartresponsespecification-delayinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub delay_in_seconds: ::Value<u32>,
        /// Property [`MessageGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-fulfillmentstartresponsespecification.html#cfn-lex-bot-fulfillmentstartresponsespecification-messagegroups).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub message_groups: ::ValueList<MessageGroup>,
    }

    impl ::codec::SerializeValue for FulfillmentStartResponseSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref allow_interrupt) = self.allow_interrupt {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowInterrupt", allow_interrupt)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DelayInSeconds", &self.delay_in_seconds)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MessageGroups", &self.message_groups)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FulfillmentStartResponseSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FulfillmentStartResponseSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FulfillmentStartResponseSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FulfillmentStartResponseSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allow_interrupt: Option<::Value<bool>> = None;
                    let mut delay_in_seconds: Option<::Value<u32>> = None;
                    let mut message_groups: Option<::ValueList<MessageGroup>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllowInterrupt" => {
                                allow_interrupt = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DelayInSeconds" => {
                                delay_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MessageGroups" => {
                                message_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FulfillmentStartResponseSpecification {
                        allow_interrupt: allow_interrupt,
                        delay_in_seconds: delay_in_seconds.ok_or(::serde::de::Error::missing_field("DelayInSeconds"))?,
                        message_groups: message_groups.ok_or(::serde::de::Error::missing_field("MessageGroups"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.FulfillmentUpdateResponseSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-fulfillmentupdateresponsespecification.html) property type.
    #[derive(Debug, Default)]
    pub struct FulfillmentUpdateResponseSpecification {
        /// Property [`AllowInterrupt`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-fulfillmentupdateresponsespecification.html#cfn-lex-bot-fulfillmentupdateresponsespecification-allowinterrupt).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allow_interrupt: Option<::Value<bool>>,
        /// Property [`FrequencyInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-fulfillmentupdateresponsespecification.html#cfn-lex-bot-fulfillmentupdateresponsespecification-frequencyinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub frequency_in_seconds: ::Value<u32>,
        /// Property [`MessageGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-fulfillmentupdateresponsespecification.html#cfn-lex-bot-fulfillmentupdateresponsespecification-messagegroups).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub message_groups: ::ValueList<MessageGroup>,
    }

    impl ::codec::SerializeValue for FulfillmentUpdateResponseSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref allow_interrupt) = self.allow_interrupt {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowInterrupt", allow_interrupt)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FrequencyInSeconds", &self.frequency_in_seconds)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MessageGroups", &self.message_groups)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FulfillmentUpdateResponseSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FulfillmentUpdateResponseSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FulfillmentUpdateResponseSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FulfillmentUpdateResponseSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allow_interrupt: Option<::Value<bool>> = None;
                    let mut frequency_in_seconds: Option<::Value<u32>> = None;
                    let mut message_groups: Option<::ValueList<MessageGroup>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllowInterrupt" => {
                                allow_interrupt = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FrequencyInSeconds" => {
                                frequency_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MessageGroups" => {
                                message_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FulfillmentUpdateResponseSpecification {
                        allow_interrupt: allow_interrupt,
                        frequency_in_seconds: frequency_in_seconds.ok_or(::serde::de::Error::missing_field("FrequencyInSeconds"))?,
                        message_groups: message_groups.ok_or(::serde::de::Error::missing_field("MessageGroups"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.FulfillmentUpdatesSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-fulfillmentupdatesspecification.html) property type.
    #[derive(Debug, Default)]
    pub struct FulfillmentUpdatesSpecification {
        /// Property [`Active`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-fulfillmentupdatesspecification.html#cfn-lex-bot-fulfillmentupdatesspecification-active).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub active: ::Value<bool>,
        /// Property [`StartResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-fulfillmentupdatesspecification.html#cfn-lex-bot-fulfillmentupdatesspecification-startresponse).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub start_response: Option<::Value<FulfillmentStartResponseSpecification>>,
        /// Property [`TimeoutInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-fulfillmentupdatesspecification.html#cfn-lex-bot-fulfillmentupdatesspecification-timeoutinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timeout_in_seconds: Option<::Value<u32>>,
        /// Property [`UpdateResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-fulfillmentupdatesspecification.html#cfn-lex-bot-fulfillmentupdatesspecification-updateresponse).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub update_response: Option<::Value<FulfillmentUpdateResponseSpecification>>,
    }

    impl ::codec::SerializeValue for FulfillmentUpdatesSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Active", &self.active)?;
            if let Some(ref start_response) = self.start_response {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartResponse", start_response)?;
            }
            if let Some(ref timeout_in_seconds) = self.timeout_in_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeoutInSeconds", timeout_in_seconds)?;
            }
            if let Some(ref update_response) = self.update_response {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UpdateResponse", update_response)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FulfillmentUpdatesSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FulfillmentUpdatesSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FulfillmentUpdatesSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FulfillmentUpdatesSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut active: Option<::Value<bool>> = None;
                    let mut start_response: Option<::Value<FulfillmentStartResponseSpecification>> = None;
                    let mut timeout_in_seconds: Option<::Value<u32>> = None;
                    let mut update_response: Option<::Value<FulfillmentUpdateResponseSpecification>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Active" => {
                                active = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StartResponse" => {
                                start_response = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimeoutInSeconds" => {
                                timeout_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UpdateResponse" => {
                                update_response = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FulfillmentUpdatesSpecification {
                        active: active.ok_or(::serde::de::Error::missing_field("Active"))?,
                        start_response: start_response,
                        timeout_in_seconds: timeout_in_seconds,
                        update_response: update_response,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.GrammarSlotTypeSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-grammarslottypesetting.html) property type.
    #[derive(Debug, Default)]
    pub struct GrammarSlotTypeSetting {
        /// Property [`Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-grammarslottypesetting.html#cfn-lex-bot-grammarslottypesetting-source).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source: Option<::Value<GrammarSlotTypeSource>>,
    }

    impl ::codec::SerializeValue for GrammarSlotTypeSetting {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref source) = self.source {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Source", source)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GrammarSlotTypeSetting {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GrammarSlotTypeSetting, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GrammarSlotTypeSetting;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GrammarSlotTypeSetting")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut source: Option<::Value<GrammarSlotTypeSource>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Source" => {
                                source = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GrammarSlotTypeSetting {
                        source: source,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.GrammarSlotTypeSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-grammarslottypesource.html) property type.
    #[derive(Debug, Default)]
    pub struct GrammarSlotTypeSource {
        /// Property [`KmsKeyArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-grammarslottypesource.html#cfn-lex-bot-grammarslottypesource-kmskeyarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kms_key_arn: Option<::Value<String>>,
        /// Property [`S3BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-grammarslottypesource.html#cfn-lex-bot-grammarslottypesource-s3bucketname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_bucket_name: ::Value<String>,
        /// Property [`S3ObjectKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-grammarslottypesource.html#cfn-lex-bot-grammarslottypesource-s3objectkey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_object_key: ::Value<String>,
    }

    impl ::codec::SerializeValue for GrammarSlotTypeSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref kms_key_arn) = self.kms_key_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyArn", kms_key_arn)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3BucketName", &self.s3_bucket_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3ObjectKey", &self.s3_object_key)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GrammarSlotTypeSource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GrammarSlotTypeSource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GrammarSlotTypeSource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GrammarSlotTypeSource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut kms_key_arn: Option<::Value<String>> = None;
                    let mut s3_bucket_name: Option<::Value<String>> = None;
                    let mut s3_object_key: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KmsKeyArn" => {
                                kms_key_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3BucketName" => {
                                s3_bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3ObjectKey" => {
                                s3_object_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GrammarSlotTypeSource {
                        kms_key_arn: kms_key_arn,
                        s3_bucket_name: s3_bucket_name.ok_or(::serde::de::Error::missing_field("S3BucketName"))?,
                        s3_object_key: s3_object_key.ok_or(::serde::de::Error::missing_field("S3ObjectKey"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.ImageResponseCard`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-imageresponsecard.html) property type.
    #[derive(Debug, Default)]
    pub struct ImageResponseCard {
        /// Property [`Buttons`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-imageresponsecard.html#cfn-lex-bot-imageresponsecard-buttons).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub buttons: Option<::ValueList<Button>>,
        /// Property [`ImageUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-imageresponsecard.html#cfn-lex-bot-imageresponsecard-imageurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub image_url: Option<::Value<String>>,
        /// Property [`Subtitle`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-imageresponsecard.html#cfn-lex-bot-imageresponsecard-subtitle).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subtitle: Option<::Value<String>>,
        /// Property [`Title`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-imageresponsecard.html#cfn-lex-bot-imageresponsecard-title).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub title: ::Value<String>,
    }

    impl ::codec::SerializeValue for ImageResponseCard {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref buttons) = self.buttons {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Buttons", buttons)?;
            }
            if let Some(ref image_url) = self.image_url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageUrl", image_url)?;
            }
            if let Some(ref subtitle) = self.subtitle {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subtitle", subtitle)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Title", &self.title)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ImageResponseCard {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ImageResponseCard, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ImageResponseCard;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ImageResponseCard")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut buttons: Option<::ValueList<Button>> = None;
                    let mut image_url: Option<::Value<String>> = None;
                    let mut subtitle: Option<::Value<String>> = None;
                    let mut title: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Buttons" => {
                                buttons = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ImageUrl" => {
                                image_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Subtitle" => {
                                subtitle = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Title" => {
                                title = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ImageResponseCard {
                        buttons: buttons,
                        image_url: image_url,
                        subtitle: subtitle,
                        title: title.ok_or(::serde::de::Error::missing_field("Title"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.InitialResponseSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-initialresponsesetting.html) property type.
    #[derive(Debug, Default)]
    pub struct InitialResponseSetting {
        /// Property [`CodeHook`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-initialresponsesetting.html#cfn-lex-bot-initialresponsesetting-codehook).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub code_hook: Option<::Value<DialogCodeHookInvocationSetting>>,
        /// Property [`Conditional`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-initialresponsesetting.html#cfn-lex-bot-initialresponsesetting-conditional).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub conditional: Option<::Value<ConditionalSpecification>>,
        /// Property [`InitialResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-initialresponsesetting.html#cfn-lex-bot-initialresponsesetting-initialresponse).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub initial_response: Option<::Value<ResponseSpecification>>,
        /// Property [`NextStep`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-initialresponsesetting.html#cfn-lex-bot-initialresponsesetting-nextstep).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub next_step: Option<::Value<DialogState>>,
    }

    impl ::codec::SerializeValue for InitialResponseSetting {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref code_hook) = self.code_hook {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CodeHook", code_hook)?;
            }
            if let Some(ref conditional) = self.conditional {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Conditional", conditional)?;
            }
            if let Some(ref initial_response) = self.initial_response {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InitialResponse", initial_response)?;
            }
            if let Some(ref next_step) = self.next_step {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NextStep", next_step)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InitialResponseSetting {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InitialResponseSetting, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InitialResponseSetting;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InitialResponseSetting")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut code_hook: Option<::Value<DialogCodeHookInvocationSetting>> = None;
                    let mut conditional: Option<::Value<ConditionalSpecification>> = None;
                    let mut initial_response: Option<::Value<ResponseSpecification>> = None;
                    let mut next_step: Option<::Value<DialogState>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CodeHook" => {
                                code_hook = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Conditional" => {
                                conditional = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InitialResponse" => {
                                initial_response = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NextStep" => {
                                next_step = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InitialResponseSetting {
                        code_hook: code_hook,
                        conditional: conditional,
                        initial_response: initial_response,
                        next_step: next_step,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.InputContext`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-inputcontext.html) property type.
    #[derive(Debug, Default)]
    pub struct InputContext {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-inputcontext.html#cfn-lex-bot-inputcontext-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for InputContext {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InputContext {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InputContext, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InputContext;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InputContext")
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

                    Ok(InputContext {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.Intent`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intent.html) property type.
    #[derive(Debug, Default)]
    pub struct Intent {
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intent.html#cfn-lex-bot-intent-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`DialogCodeHook`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intent.html#cfn-lex-bot-intent-dialogcodehook).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dialog_code_hook: Option<::Value<DialogCodeHookSetting>>,
        /// Property [`FulfillmentCodeHook`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intent.html#cfn-lex-bot-intent-fulfillmentcodehook).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub fulfillment_code_hook: Option<::Value<FulfillmentCodeHookSetting>>,
        /// Property [`InitialResponseSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intent.html#cfn-lex-bot-intent-initialresponsesetting).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub initial_response_setting: Option<::Value<InitialResponseSetting>>,
        /// Property [`InputContexts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intent.html#cfn-lex-bot-intent-inputcontexts).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_contexts: Option<::ValueList<InputContext>>,
        /// Property [`IntentClosingSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intent.html#cfn-lex-bot-intent-intentclosingsetting).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub intent_closing_setting: Option<::Value<IntentClosingSetting>>,
        /// Property [`IntentConfirmationSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intent.html#cfn-lex-bot-intent-intentconfirmationsetting).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub intent_confirmation_setting: Option<::Value<IntentConfirmationSetting>>,
        /// Property [`KendraConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intent.html#cfn-lex-bot-intent-kendraconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kendra_configuration: Option<::Value<KendraConfiguration>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intent.html#cfn-lex-bot-intent-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`OutputContexts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intent.html#cfn-lex-bot-intent-outputcontexts).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub output_contexts: Option<::ValueList<OutputContext>>,
        /// Property [`ParentIntentSignature`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intent.html#cfn-lex-bot-intent-parentintentsignature).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parent_intent_signature: Option<::Value<String>>,
        /// Property [`SampleUtterances`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intent.html#cfn-lex-bot-intent-sampleutterances).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sample_utterances: Option<::ValueList<SampleUtterance>>,
        /// Property [`SlotPriorities`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intent.html#cfn-lex-bot-intent-slotpriorities).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub slot_priorities: Option<::ValueList<SlotPriority>>,
        /// Property [`Slots`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intent.html#cfn-lex-bot-intent-slots).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub slots: Option<::ValueList<Slot>>,
    }

    impl ::codec::SerializeValue for Intent {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            if let Some(ref dialog_code_hook) = self.dialog_code_hook {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DialogCodeHook", dialog_code_hook)?;
            }
            if let Some(ref fulfillment_code_hook) = self.fulfillment_code_hook {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FulfillmentCodeHook", fulfillment_code_hook)?;
            }
            if let Some(ref initial_response_setting) = self.initial_response_setting {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InitialResponseSetting", initial_response_setting)?;
            }
            if let Some(ref input_contexts) = self.input_contexts {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputContexts", input_contexts)?;
            }
            if let Some(ref intent_closing_setting) = self.intent_closing_setting {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntentClosingSetting", intent_closing_setting)?;
            }
            if let Some(ref intent_confirmation_setting) = self.intent_confirmation_setting {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntentConfirmationSetting", intent_confirmation_setting)?;
            }
            if let Some(ref kendra_configuration) = self.kendra_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KendraConfiguration", kendra_configuration)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            if let Some(ref output_contexts) = self.output_contexts {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutputContexts", output_contexts)?;
            }
            if let Some(ref parent_intent_signature) = self.parent_intent_signature {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParentIntentSignature", parent_intent_signature)?;
            }
            if let Some(ref sample_utterances) = self.sample_utterances {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SampleUtterances", sample_utterances)?;
            }
            if let Some(ref slot_priorities) = self.slot_priorities {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SlotPriorities", slot_priorities)?;
            }
            if let Some(ref slots) = self.slots {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Slots", slots)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Intent {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Intent, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Intent;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Intent")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut description: Option<::Value<String>> = None;
                    let mut dialog_code_hook: Option<::Value<DialogCodeHookSetting>> = None;
                    let mut fulfillment_code_hook: Option<::Value<FulfillmentCodeHookSetting>> = None;
                    let mut initial_response_setting: Option<::Value<InitialResponseSetting>> = None;
                    let mut input_contexts: Option<::ValueList<InputContext>> = None;
                    let mut intent_closing_setting: Option<::Value<IntentClosingSetting>> = None;
                    let mut intent_confirmation_setting: Option<::Value<IntentConfirmationSetting>> = None;
                    let mut kendra_configuration: Option<::Value<KendraConfiguration>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut output_contexts: Option<::ValueList<OutputContext>> = None;
                    let mut parent_intent_signature: Option<::Value<String>> = None;
                    let mut sample_utterances: Option<::ValueList<SampleUtterance>> = None;
                    let mut slot_priorities: Option<::ValueList<SlotPriority>> = None;
                    let mut slots: Option<::ValueList<Slot>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DialogCodeHook" => {
                                dialog_code_hook = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FulfillmentCodeHook" => {
                                fulfillment_code_hook = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InitialResponseSetting" => {
                                initial_response_setting = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InputContexts" => {
                                input_contexts = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IntentClosingSetting" => {
                                intent_closing_setting = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IntentConfirmationSetting" => {
                                intent_confirmation_setting = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KendraConfiguration" => {
                                kendra_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OutputContexts" => {
                                output_contexts = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ParentIntentSignature" => {
                                parent_intent_signature = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SampleUtterances" => {
                                sample_utterances = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SlotPriorities" => {
                                slot_priorities = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Slots" => {
                                slots = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Intent {
                        description: description,
                        dialog_code_hook: dialog_code_hook,
                        fulfillment_code_hook: fulfillment_code_hook,
                        initial_response_setting: initial_response_setting,
                        input_contexts: input_contexts,
                        intent_closing_setting: intent_closing_setting,
                        intent_confirmation_setting: intent_confirmation_setting,
                        kendra_configuration: kendra_configuration,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        output_contexts: output_contexts,
                        parent_intent_signature: parent_intent_signature,
                        sample_utterances: sample_utterances,
                        slot_priorities: slot_priorities,
                        slots: slots,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.IntentClosingSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intentclosingsetting.html) property type.
    #[derive(Debug, Default)]
    pub struct IntentClosingSetting {
        /// Property [`ClosingResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intentclosingsetting.html#cfn-lex-bot-intentclosingsetting-closingresponse).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub closing_response: Option<::Value<ResponseSpecification>>,
        /// Property [`Conditional`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intentclosingsetting.html#cfn-lex-bot-intentclosingsetting-conditional).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub conditional: Option<::Value<ConditionalSpecification>>,
        /// Property [`IsActive`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intentclosingsetting.html#cfn-lex-bot-intentclosingsetting-isactive).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub is_active: Option<::Value<bool>>,
        /// Property [`NextStep`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intentclosingsetting.html#cfn-lex-bot-intentclosingsetting-nextstep).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub next_step: Option<::Value<DialogState>>,
    }

    impl ::codec::SerializeValue for IntentClosingSetting {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref closing_response) = self.closing_response {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClosingResponse", closing_response)?;
            }
            if let Some(ref conditional) = self.conditional {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Conditional", conditional)?;
            }
            if let Some(ref is_active) = self.is_active {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsActive", is_active)?;
            }
            if let Some(ref next_step) = self.next_step {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NextStep", next_step)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IntentClosingSetting {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IntentClosingSetting, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IntentClosingSetting;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IntentClosingSetting")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut closing_response: Option<::Value<ResponseSpecification>> = None;
                    let mut conditional: Option<::Value<ConditionalSpecification>> = None;
                    let mut is_active: Option<::Value<bool>> = None;
                    let mut next_step: Option<::Value<DialogState>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ClosingResponse" => {
                                closing_response = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Conditional" => {
                                conditional = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IsActive" => {
                                is_active = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NextStep" => {
                                next_step = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IntentClosingSetting {
                        closing_response: closing_response,
                        conditional: conditional,
                        is_active: is_active,
                        next_step: next_step,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.IntentConfirmationSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intentconfirmationsetting.html) property type.
    #[derive(Debug, Default)]
    pub struct IntentConfirmationSetting {
        /// Property [`CodeHook`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intentconfirmationsetting.html#cfn-lex-bot-intentconfirmationsetting-codehook).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub code_hook: Option<::Value<DialogCodeHookInvocationSetting>>,
        /// Property [`ConfirmationConditional`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intentconfirmationsetting.html#cfn-lex-bot-intentconfirmationsetting-confirmationconditional).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub confirmation_conditional: Option<::Value<ConditionalSpecification>>,
        /// Property [`ConfirmationNextStep`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intentconfirmationsetting.html#cfn-lex-bot-intentconfirmationsetting-confirmationnextstep).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub confirmation_next_step: Option<::Value<DialogState>>,
        /// Property [`ConfirmationResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intentconfirmationsetting.html#cfn-lex-bot-intentconfirmationsetting-confirmationresponse).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub confirmation_response: Option<::Value<ResponseSpecification>>,
        /// Property [`DeclinationConditional`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intentconfirmationsetting.html#cfn-lex-bot-intentconfirmationsetting-declinationconditional).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub declination_conditional: Option<::Value<ConditionalSpecification>>,
        /// Property [`DeclinationNextStep`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intentconfirmationsetting.html#cfn-lex-bot-intentconfirmationsetting-declinationnextstep).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub declination_next_step: Option<::Value<DialogState>>,
        /// Property [`DeclinationResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intentconfirmationsetting.html#cfn-lex-bot-intentconfirmationsetting-declinationresponse).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub declination_response: Option<::Value<ResponseSpecification>>,
        /// Property [`ElicitationCodeHook`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intentconfirmationsetting.html#cfn-lex-bot-intentconfirmationsetting-elicitationcodehook).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub elicitation_code_hook: Option<::Value<ElicitationCodeHookInvocationSetting>>,
        /// Property [`FailureConditional`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intentconfirmationsetting.html#cfn-lex-bot-intentconfirmationsetting-failureconditional).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub failure_conditional: Option<::Value<ConditionalSpecification>>,
        /// Property [`FailureNextStep`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intentconfirmationsetting.html#cfn-lex-bot-intentconfirmationsetting-failurenextstep).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub failure_next_step: Option<::Value<DialogState>>,
        /// Property [`FailureResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intentconfirmationsetting.html#cfn-lex-bot-intentconfirmationsetting-failureresponse).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub failure_response: Option<::Value<ResponseSpecification>>,
        /// Property [`IsActive`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intentconfirmationsetting.html#cfn-lex-bot-intentconfirmationsetting-isactive).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub is_active: Option<::Value<bool>>,
        /// Property [`PromptSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intentconfirmationsetting.html#cfn-lex-bot-intentconfirmationsetting-promptspecification).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub prompt_specification: ::Value<PromptSpecification>,
    }

    impl ::codec::SerializeValue for IntentConfirmationSetting {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref code_hook) = self.code_hook {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CodeHook", code_hook)?;
            }
            if let Some(ref confirmation_conditional) = self.confirmation_conditional {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfirmationConditional", confirmation_conditional)?;
            }
            if let Some(ref confirmation_next_step) = self.confirmation_next_step {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfirmationNextStep", confirmation_next_step)?;
            }
            if let Some(ref confirmation_response) = self.confirmation_response {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfirmationResponse", confirmation_response)?;
            }
            if let Some(ref declination_conditional) = self.declination_conditional {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeclinationConditional", declination_conditional)?;
            }
            if let Some(ref declination_next_step) = self.declination_next_step {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeclinationNextStep", declination_next_step)?;
            }
            if let Some(ref declination_response) = self.declination_response {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeclinationResponse", declination_response)?;
            }
            if let Some(ref elicitation_code_hook) = self.elicitation_code_hook {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ElicitationCodeHook", elicitation_code_hook)?;
            }
            if let Some(ref failure_conditional) = self.failure_conditional {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FailureConditional", failure_conditional)?;
            }
            if let Some(ref failure_next_step) = self.failure_next_step {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FailureNextStep", failure_next_step)?;
            }
            if let Some(ref failure_response) = self.failure_response {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FailureResponse", failure_response)?;
            }
            if let Some(ref is_active) = self.is_active {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsActive", is_active)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PromptSpecification", &self.prompt_specification)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IntentConfirmationSetting {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IntentConfirmationSetting, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IntentConfirmationSetting;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IntentConfirmationSetting")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut code_hook: Option<::Value<DialogCodeHookInvocationSetting>> = None;
                    let mut confirmation_conditional: Option<::Value<ConditionalSpecification>> = None;
                    let mut confirmation_next_step: Option<::Value<DialogState>> = None;
                    let mut confirmation_response: Option<::Value<ResponseSpecification>> = None;
                    let mut declination_conditional: Option<::Value<ConditionalSpecification>> = None;
                    let mut declination_next_step: Option<::Value<DialogState>> = None;
                    let mut declination_response: Option<::Value<ResponseSpecification>> = None;
                    let mut elicitation_code_hook: Option<::Value<ElicitationCodeHookInvocationSetting>> = None;
                    let mut failure_conditional: Option<::Value<ConditionalSpecification>> = None;
                    let mut failure_next_step: Option<::Value<DialogState>> = None;
                    let mut failure_response: Option<::Value<ResponseSpecification>> = None;
                    let mut is_active: Option<::Value<bool>> = None;
                    let mut prompt_specification: Option<::Value<PromptSpecification>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CodeHook" => {
                                code_hook = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConfirmationConditional" => {
                                confirmation_conditional = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConfirmationNextStep" => {
                                confirmation_next_step = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConfirmationResponse" => {
                                confirmation_response = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeclinationConditional" => {
                                declination_conditional = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeclinationNextStep" => {
                                declination_next_step = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeclinationResponse" => {
                                declination_response = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ElicitationCodeHook" => {
                                elicitation_code_hook = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FailureConditional" => {
                                failure_conditional = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FailureNextStep" => {
                                failure_next_step = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FailureResponse" => {
                                failure_response = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IsActive" => {
                                is_active = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PromptSpecification" => {
                                prompt_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IntentConfirmationSetting {
                        code_hook: code_hook,
                        confirmation_conditional: confirmation_conditional,
                        confirmation_next_step: confirmation_next_step,
                        confirmation_response: confirmation_response,
                        declination_conditional: declination_conditional,
                        declination_next_step: declination_next_step,
                        declination_response: declination_response,
                        elicitation_code_hook: elicitation_code_hook,
                        failure_conditional: failure_conditional,
                        failure_next_step: failure_next_step,
                        failure_response: failure_response,
                        is_active: is_active,
                        prompt_specification: prompt_specification.ok_or(::serde::de::Error::missing_field("PromptSpecification"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.IntentOverride`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intentoverride.html) property type.
    #[derive(Debug, Default)]
    pub struct IntentOverride {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intentoverride.html#cfn-lex-bot-intentoverride-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Slots`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intentoverride.html#cfn-lex-bot-intentoverride-slots).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub slots: Option<::ValueList<SlotValueOverrideMap>>,
    }

    impl ::codec::SerializeValue for IntentOverride {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref slots) = self.slots {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Slots", slots)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IntentOverride {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IntentOverride, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IntentOverride;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IntentOverride")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut slots: Option<::ValueList<SlotValueOverrideMap>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Slots" => {
                                slots = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IntentOverride {
                        name: name,
                        slots: slots,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.KendraConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-kendraconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct KendraConfiguration {
        /// Property [`KendraIndex`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-kendraconfiguration.html#cfn-lex-bot-kendraconfiguration-kendraindex).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kendra_index: ::Value<String>,
        /// Property [`QueryFilterString`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-kendraconfiguration.html#cfn-lex-bot-kendraconfiguration-queryfilterstring).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub query_filter_string: Option<::Value<String>>,
        /// Property [`QueryFilterStringEnabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-kendraconfiguration.html#cfn-lex-bot-kendraconfiguration-queryfilterstringenabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub query_filter_string_enabled: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for KendraConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KendraIndex", &self.kendra_index)?;
            if let Some(ref query_filter_string) = self.query_filter_string {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryFilterString", query_filter_string)?;
            }
            if let Some(ref query_filter_string_enabled) = self.query_filter_string_enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryFilterStringEnabled", query_filter_string_enabled)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KendraConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KendraConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KendraConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KendraConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut kendra_index: Option<::Value<String>> = None;
                    let mut query_filter_string: Option<::Value<String>> = None;
                    let mut query_filter_string_enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KendraIndex" => {
                                kendra_index = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QueryFilterString" => {
                                query_filter_string = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QueryFilterStringEnabled" => {
                                query_filter_string_enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KendraConfiguration {
                        kendra_index: kendra_index.ok_or(::serde::de::Error::missing_field("KendraIndex"))?,
                        query_filter_string: query_filter_string,
                        query_filter_string_enabled: query_filter_string_enabled,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.LambdaCodeHook`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-lambdacodehook.html) property type.
    #[derive(Debug, Default)]
    pub struct LambdaCodeHook {
        /// Property [`CodeHookInterfaceVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-lambdacodehook.html#cfn-lex-bot-lambdacodehook-codehookinterfaceversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub code_hook_interface_version: ::Value<String>,
        /// Property [`LambdaArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-lambdacodehook.html#cfn-lex-bot-lambdacodehook-lambdaarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lambda_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for LambdaCodeHook {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CodeHookInterfaceVersion", &self.code_hook_interface_version)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaArn", &self.lambda_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LambdaCodeHook {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LambdaCodeHook, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LambdaCodeHook;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LambdaCodeHook")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut code_hook_interface_version: Option<::Value<String>> = None;
                    let mut lambda_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CodeHookInterfaceVersion" => {
                                code_hook_interface_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LambdaArn" => {
                                lambda_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LambdaCodeHook {
                        code_hook_interface_version: code_hook_interface_version.ok_or(::serde::de::Error::missing_field("CodeHookInterfaceVersion"))?,
                        lambda_arn: lambda_arn.ok_or(::serde::de::Error::missing_field("LambdaArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.Message`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-message.html) property type.
    #[derive(Debug, Default)]
    pub struct Message {
        /// Property [`CustomPayload`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-message.html#cfn-lex-bot-message-custompayload).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_payload: Option<::Value<CustomPayload>>,
        /// Property [`ImageResponseCard`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-message.html#cfn-lex-bot-message-imageresponsecard).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub image_response_card: Option<::Value<ImageResponseCard>>,
        /// Property [`PlainTextMessage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-message.html#cfn-lex-bot-message-plaintextmessage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub plain_text_message: Option<::Value<PlainTextMessage>>,
        /// Property [`SSMLMessage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-message.html#cfn-lex-bot-message-ssmlmessage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ssml_message: Option<::Value<SSMLMessage>>,
    }

    impl ::codec::SerializeValue for Message {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref custom_payload) = self.custom_payload {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomPayload", custom_payload)?;
            }
            if let Some(ref image_response_card) = self.image_response_card {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageResponseCard", image_response_card)?;
            }
            if let Some(ref plain_text_message) = self.plain_text_message {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PlainTextMessage", plain_text_message)?;
            }
            if let Some(ref ssml_message) = self.ssml_message {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SSMLMessage", ssml_message)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Message {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Message, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Message;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Message")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut custom_payload: Option<::Value<CustomPayload>> = None;
                    let mut image_response_card: Option<::Value<ImageResponseCard>> = None;
                    let mut plain_text_message: Option<::Value<PlainTextMessage>> = None;
                    let mut ssml_message: Option<::Value<SSMLMessage>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CustomPayload" => {
                                custom_payload = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ImageResponseCard" => {
                                image_response_card = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PlainTextMessage" => {
                                plain_text_message = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SSMLMessage" => {
                                ssml_message = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Message {
                        custom_payload: custom_payload,
                        image_response_card: image_response_card,
                        plain_text_message: plain_text_message,
                        ssml_message: ssml_message,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.MessageGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-messagegroup.html) property type.
    #[derive(Debug, Default)]
    pub struct MessageGroup {
        /// Property [`Message`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-messagegroup.html#cfn-lex-bot-messagegroup-message).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub message: ::Value<Message>,
        /// Property [`Variations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-messagegroup.html#cfn-lex-bot-messagegroup-variations).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub variations: Option<::ValueList<Message>>,
    }

    impl ::codec::SerializeValue for MessageGroup {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Message", &self.message)?;
            if let Some(ref variations) = self.variations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Variations", variations)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MessageGroup {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MessageGroup, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MessageGroup;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MessageGroup")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut message: Option<::Value<Message>> = None;
                    let mut variations: Option<::ValueList<Message>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Message" => {
                                message = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Variations" => {
                                variations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MessageGroup {
                        message: message.ok_or(::serde::de::Error::missing_field("Message"))?,
                        variations: variations,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.MultipleValuesSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-multiplevaluessetting.html) property type.
    #[derive(Debug, Default)]
    pub struct MultipleValuesSetting {
        /// Property [`AllowMultipleValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-multiplevaluessetting.html#cfn-lex-bot-multiplevaluessetting-allowmultiplevalues).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allow_multiple_values: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for MultipleValuesSetting {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref allow_multiple_values) = self.allow_multiple_values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowMultipleValues", allow_multiple_values)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MultipleValuesSetting {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MultipleValuesSetting, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MultipleValuesSetting;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MultipleValuesSetting")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allow_multiple_values: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllowMultipleValues" => {
                                allow_multiple_values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MultipleValuesSetting {
                        allow_multiple_values: allow_multiple_values,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.ObfuscationSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-obfuscationsetting.html) property type.
    #[derive(Debug, Default)]
    pub struct ObfuscationSetting {
        /// Property [`ObfuscationSettingType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-obfuscationsetting.html#cfn-lex-bot-obfuscationsetting-obfuscationsettingtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub obfuscation_setting_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for ObfuscationSetting {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ObfuscationSettingType", &self.obfuscation_setting_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ObfuscationSetting {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ObfuscationSetting, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ObfuscationSetting;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ObfuscationSetting")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut obfuscation_setting_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ObfuscationSettingType" => {
                                obfuscation_setting_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ObfuscationSetting {
                        obfuscation_setting_type: obfuscation_setting_type.ok_or(::serde::de::Error::missing_field("ObfuscationSettingType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.OutputContext`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-outputcontext.html) property type.
    #[derive(Debug, Default)]
    pub struct OutputContext {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-outputcontext.html#cfn-lex-bot-outputcontext-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`TimeToLiveInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-outputcontext.html#cfn-lex-bot-outputcontext-timetoliveinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub time_to_live_in_seconds: ::Value<u32>,
        /// Property [`TurnsToLive`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-outputcontext.html#cfn-lex-bot-outputcontext-turnstolive).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub turns_to_live: ::Value<u32>,
    }

    impl ::codec::SerializeValue for OutputContext {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeToLiveInSeconds", &self.time_to_live_in_seconds)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TurnsToLive", &self.turns_to_live)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OutputContext {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OutputContext, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OutputContext;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OutputContext")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut time_to_live_in_seconds: Option<::Value<u32>> = None;
                    let mut turns_to_live: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimeToLiveInSeconds" => {
                                time_to_live_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TurnsToLive" => {
                                turns_to_live = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OutputContext {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        time_to_live_in_seconds: time_to_live_in_seconds.ok_or(::serde::de::Error::missing_field("TimeToLiveInSeconds"))?,
                        turns_to_live: turns_to_live.ok_or(::serde::de::Error::missing_field("TurnsToLive"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.PlainTextMessage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-plaintextmessage.html) property type.
    #[derive(Debug, Default)]
    pub struct PlainTextMessage {
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-plaintextmessage.html#cfn-lex-bot-plaintextmessage-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for PlainTextMessage {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PlainTextMessage {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PlainTextMessage, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PlainTextMessage;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PlainTextMessage")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PlainTextMessage {
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.PostDialogCodeHookInvocationSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-postdialogcodehookinvocationspecification.html) property type.
    #[derive(Debug, Default)]
    pub struct PostDialogCodeHookInvocationSpecification {
        /// Property [`FailureConditional`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-postdialogcodehookinvocationspecification.html#cfn-lex-bot-postdialogcodehookinvocationspecification-failureconditional).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub failure_conditional: Option<::Value<ConditionalSpecification>>,
        /// Property [`FailureNextStep`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-postdialogcodehookinvocationspecification.html#cfn-lex-bot-postdialogcodehookinvocationspecification-failurenextstep).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub failure_next_step: Option<::Value<DialogState>>,
        /// Property [`FailureResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-postdialogcodehookinvocationspecification.html#cfn-lex-bot-postdialogcodehookinvocationspecification-failureresponse).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub failure_response: Option<::Value<ResponseSpecification>>,
        /// Property [`SuccessConditional`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-postdialogcodehookinvocationspecification.html#cfn-lex-bot-postdialogcodehookinvocationspecification-successconditional).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub success_conditional: Option<::Value<ConditionalSpecification>>,
        /// Property [`SuccessNextStep`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-postdialogcodehookinvocationspecification.html#cfn-lex-bot-postdialogcodehookinvocationspecification-successnextstep).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub success_next_step: Option<::Value<DialogState>>,
        /// Property [`SuccessResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-postdialogcodehookinvocationspecification.html#cfn-lex-bot-postdialogcodehookinvocationspecification-successresponse).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub success_response: Option<::Value<ResponseSpecification>>,
        /// Property [`TimeoutConditional`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-postdialogcodehookinvocationspecification.html#cfn-lex-bot-postdialogcodehookinvocationspecification-timeoutconditional).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timeout_conditional: Option<::Value<ConditionalSpecification>>,
        /// Property [`TimeoutNextStep`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-postdialogcodehookinvocationspecification.html#cfn-lex-bot-postdialogcodehookinvocationspecification-timeoutnextstep).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timeout_next_step: Option<::Value<DialogState>>,
        /// Property [`TimeoutResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-postdialogcodehookinvocationspecification.html#cfn-lex-bot-postdialogcodehookinvocationspecification-timeoutresponse).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timeout_response: Option<::Value<ResponseSpecification>>,
    }

    impl ::codec::SerializeValue for PostDialogCodeHookInvocationSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref failure_conditional) = self.failure_conditional {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FailureConditional", failure_conditional)?;
            }
            if let Some(ref failure_next_step) = self.failure_next_step {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FailureNextStep", failure_next_step)?;
            }
            if let Some(ref failure_response) = self.failure_response {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FailureResponse", failure_response)?;
            }
            if let Some(ref success_conditional) = self.success_conditional {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SuccessConditional", success_conditional)?;
            }
            if let Some(ref success_next_step) = self.success_next_step {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SuccessNextStep", success_next_step)?;
            }
            if let Some(ref success_response) = self.success_response {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SuccessResponse", success_response)?;
            }
            if let Some(ref timeout_conditional) = self.timeout_conditional {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeoutConditional", timeout_conditional)?;
            }
            if let Some(ref timeout_next_step) = self.timeout_next_step {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeoutNextStep", timeout_next_step)?;
            }
            if let Some(ref timeout_response) = self.timeout_response {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeoutResponse", timeout_response)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PostDialogCodeHookInvocationSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PostDialogCodeHookInvocationSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PostDialogCodeHookInvocationSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PostDialogCodeHookInvocationSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut failure_conditional: Option<::Value<ConditionalSpecification>> = None;
                    let mut failure_next_step: Option<::Value<DialogState>> = None;
                    let mut failure_response: Option<::Value<ResponseSpecification>> = None;
                    let mut success_conditional: Option<::Value<ConditionalSpecification>> = None;
                    let mut success_next_step: Option<::Value<DialogState>> = None;
                    let mut success_response: Option<::Value<ResponseSpecification>> = None;
                    let mut timeout_conditional: Option<::Value<ConditionalSpecification>> = None;
                    let mut timeout_next_step: Option<::Value<DialogState>> = None;
                    let mut timeout_response: Option<::Value<ResponseSpecification>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FailureConditional" => {
                                failure_conditional = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FailureNextStep" => {
                                failure_next_step = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FailureResponse" => {
                                failure_response = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SuccessConditional" => {
                                success_conditional = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SuccessNextStep" => {
                                success_next_step = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SuccessResponse" => {
                                success_response = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimeoutConditional" => {
                                timeout_conditional = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimeoutNextStep" => {
                                timeout_next_step = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimeoutResponse" => {
                                timeout_response = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PostDialogCodeHookInvocationSpecification {
                        failure_conditional: failure_conditional,
                        failure_next_step: failure_next_step,
                        failure_response: failure_response,
                        success_conditional: success_conditional,
                        success_next_step: success_next_step,
                        success_response: success_response,
                        timeout_conditional: timeout_conditional,
                        timeout_next_step: timeout_next_step,
                        timeout_response: timeout_response,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.PostFulfillmentStatusSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-postfulfillmentstatusspecification.html) property type.
    #[derive(Debug, Default)]
    pub struct PostFulfillmentStatusSpecification {
        /// Property [`FailureConditional`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-postfulfillmentstatusspecification.html#cfn-lex-bot-postfulfillmentstatusspecification-failureconditional).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub failure_conditional: Option<::Value<ConditionalSpecification>>,
        /// Property [`FailureNextStep`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-postfulfillmentstatusspecification.html#cfn-lex-bot-postfulfillmentstatusspecification-failurenextstep).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub failure_next_step: Option<::Value<DialogState>>,
        /// Property [`FailureResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-postfulfillmentstatusspecification.html#cfn-lex-bot-postfulfillmentstatusspecification-failureresponse).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub failure_response: Option<::Value<ResponseSpecification>>,
        /// Property [`SuccessConditional`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-postfulfillmentstatusspecification.html#cfn-lex-bot-postfulfillmentstatusspecification-successconditional).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub success_conditional: Option<::Value<ConditionalSpecification>>,
        /// Property [`SuccessNextStep`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-postfulfillmentstatusspecification.html#cfn-lex-bot-postfulfillmentstatusspecification-successnextstep).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub success_next_step: Option<::Value<DialogState>>,
        /// Property [`SuccessResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-postfulfillmentstatusspecification.html#cfn-lex-bot-postfulfillmentstatusspecification-successresponse).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub success_response: Option<::Value<ResponseSpecification>>,
        /// Property [`TimeoutConditional`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-postfulfillmentstatusspecification.html#cfn-lex-bot-postfulfillmentstatusspecification-timeoutconditional).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timeout_conditional: Option<::Value<ConditionalSpecification>>,
        /// Property [`TimeoutNextStep`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-postfulfillmentstatusspecification.html#cfn-lex-bot-postfulfillmentstatusspecification-timeoutnextstep).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timeout_next_step: Option<::Value<DialogState>>,
        /// Property [`TimeoutResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-postfulfillmentstatusspecification.html#cfn-lex-bot-postfulfillmentstatusspecification-timeoutresponse).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timeout_response: Option<::Value<ResponseSpecification>>,
    }

    impl ::codec::SerializeValue for PostFulfillmentStatusSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref failure_conditional) = self.failure_conditional {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FailureConditional", failure_conditional)?;
            }
            if let Some(ref failure_next_step) = self.failure_next_step {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FailureNextStep", failure_next_step)?;
            }
            if let Some(ref failure_response) = self.failure_response {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FailureResponse", failure_response)?;
            }
            if let Some(ref success_conditional) = self.success_conditional {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SuccessConditional", success_conditional)?;
            }
            if let Some(ref success_next_step) = self.success_next_step {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SuccessNextStep", success_next_step)?;
            }
            if let Some(ref success_response) = self.success_response {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SuccessResponse", success_response)?;
            }
            if let Some(ref timeout_conditional) = self.timeout_conditional {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeoutConditional", timeout_conditional)?;
            }
            if let Some(ref timeout_next_step) = self.timeout_next_step {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeoutNextStep", timeout_next_step)?;
            }
            if let Some(ref timeout_response) = self.timeout_response {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeoutResponse", timeout_response)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PostFulfillmentStatusSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PostFulfillmentStatusSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PostFulfillmentStatusSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PostFulfillmentStatusSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut failure_conditional: Option<::Value<ConditionalSpecification>> = None;
                    let mut failure_next_step: Option<::Value<DialogState>> = None;
                    let mut failure_response: Option<::Value<ResponseSpecification>> = None;
                    let mut success_conditional: Option<::Value<ConditionalSpecification>> = None;
                    let mut success_next_step: Option<::Value<DialogState>> = None;
                    let mut success_response: Option<::Value<ResponseSpecification>> = None;
                    let mut timeout_conditional: Option<::Value<ConditionalSpecification>> = None;
                    let mut timeout_next_step: Option<::Value<DialogState>> = None;
                    let mut timeout_response: Option<::Value<ResponseSpecification>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FailureConditional" => {
                                failure_conditional = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FailureNextStep" => {
                                failure_next_step = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FailureResponse" => {
                                failure_response = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SuccessConditional" => {
                                success_conditional = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SuccessNextStep" => {
                                success_next_step = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SuccessResponse" => {
                                success_response = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimeoutConditional" => {
                                timeout_conditional = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimeoutNextStep" => {
                                timeout_next_step = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimeoutResponse" => {
                                timeout_response = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PostFulfillmentStatusSpecification {
                        failure_conditional: failure_conditional,
                        failure_next_step: failure_next_step,
                        failure_response: failure_response,
                        success_conditional: success_conditional,
                        success_next_step: success_next_step,
                        success_response: success_response,
                        timeout_conditional: timeout_conditional,
                        timeout_next_step: timeout_next_step,
                        timeout_response: timeout_response,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.PromptAttemptSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-promptattemptspecification.html) property type.
    #[derive(Debug, Default)]
    pub struct PromptAttemptSpecification {
        /// Property [`AllowInterrupt`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-promptattemptspecification.html#cfn-lex-bot-promptattemptspecification-allowinterrupt).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allow_interrupt: Option<::Value<bool>>,
        /// Property [`AllowedInputTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-promptattemptspecification.html#cfn-lex-bot-promptattemptspecification-allowedinputtypes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allowed_input_types: ::Value<AllowedInputTypes>,
        /// Property [`AudioAndDTMFInputSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-promptattemptspecification.html#cfn-lex-bot-promptattemptspecification-audioanddtmfinputspecification).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub audio_and_dtmf_input_specification: Option<::Value<AudioAndDTMFInputSpecification>>,
        /// Property [`TextInputSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-promptattemptspecification.html#cfn-lex-bot-promptattemptspecification-textinputspecification).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text_input_specification: Option<::Value<TextInputSpecification>>,
    }

    impl ::codec::SerializeValue for PromptAttemptSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref allow_interrupt) = self.allow_interrupt {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowInterrupt", allow_interrupt)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowedInputTypes", &self.allowed_input_types)?;
            if let Some(ref audio_and_dtmf_input_specification) = self.audio_and_dtmf_input_specification {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AudioAndDTMFInputSpecification", audio_and_dtmf_input_specification)?;
            }
            if let Some(ref text_input_specification) = self.text_input_specification {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextInputSpecification", text_input_specification)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PromptAttemptSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PromptAttemptSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PromptAttemptSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PromptAttemptSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allow_interrupt: Option<::Value<bool>> = None;
                    let mut allowed_input_types: Option<::Value<AllowedInputTypes>> = None;
                    let mut audio_and_dtmf_input_specification: Option<::Value<AudioAndDTMFInputSpecification>> = None;
                    let mut text_input_specification: Option<::Value<TextInputSpecification>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllowInterrupt" => {
                                allow_interrupt = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AllowedInputTypes" => {
                                allowed_input_types = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AudioAndDTMFInputSpecification" => {
                                audio_and_dtmf_input_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TextInputSpecification" => {
                                text_input_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PromptAttemptSpecification {
                        allow_interrupt: allow_interrupt,
                        allowed_input_types: allowed_input_types.ok_or(::serde::de::Error::missing_field("AllowedInputTypes"))?,
                        audio_and_dtmf_input_specification: audio_and_dtmf_input_specification,
                        text_input_specification: text_input_specification,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.PromptSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-promptspecification.html) property type.
    #[derive(Debug, Default)]
    pub struct PromptSpecification {
        /// Property [`AllowInterrupt`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-promptspecification.html#cfn-lex-bot-promptspecification-allowinterrupt).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allow_interrupt: Option<::Value<bool>>,
        /// Property [`MaxRetries`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-promptspecification.html#cfn-lex-bot-promptspecification-maxretries).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_retries: ::Value<u32>,
        /// Property [`MessageGroupsList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-promptspecification.html#cfn-lex-bot-promptspecification-messagegroupslist).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub message_groups_list: ::ValueList<MessageGroup>,
        /// Property [`MessageSelectionStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-promptspecification.html#cfn-lex-bot-promptspecification-messageselectionstrategy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub message_selection_strategy: Option<::Value<String>>,
        /// Property [`PromptAttemptsSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-promptspecification.html#cfn-lex-bot-promptspecification-promptattemptsspecification).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub prompt_attempts_specification: Option<::ValueMap<PromptAttemptSpecification>>,
    }

    impl ::codec::SerializeValue for PromptSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref allow_interrupt) = self.allow_interrupt {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowInterrupt", allow_interrupt)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxRetries", &self.max_retries)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MessageGroupsList", &self.message_groups_list)?;
            if let Some(ref message_selection_strategy) = self.message_selection_strategy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MessageSelectionStrategy", message_selection_strategy)?;
            }
            if let Some(ref prompt_attempts_specification) = self.prompt_attempts_specification {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PromptAttemptsSpecification", prompt_attempts_specification)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PromptSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PromptSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PromptSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PromptSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allow_interrupt: Option<::Value<bool>> = None;
                    let mut max_retries: Option<::Value<u32>> = None;
                    let mut message_groups_list: Option<::ValueList<MessageGroup>> = None;
                    let mut message_selection_strategy: Option<::Value<String>> = None;
                    let mut prompt_attempts_specification: Option<::ValueMap<PromptAttemptSpecification>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllowInterrupt" => {
                                allow_interrupt = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxRetries" => {
                                max_retries = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MessageGroupsList" => {
                                message_groups_list = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MessageSelectionStrategy" => {
                                message_selection_strategy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PromptAttemptsSpecification" => {
                                prompt_attempts_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PromptSpecification {
                        allow_interrupt: allow_interrupt,
                        max_retries: max_retries.ok_or(::serde::de::Error::missing_field("MaxRetries"))?,
                        message_groups_list: message_groups_list.ok_or(::serde::de::Error::missing_field("MessageGroupsList"))?,
                        message_selection_strategy: message_selection_strategy,
                        prompt_attempts_specification: prompt_attempts_specification,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.ResponseSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-responsespecification.html) property type.
    #[derive(Debug, Default)]
    pub struct ResponseSpecification {
        /// Property [`AllowInterrupt`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-responsespecification.html#cfn-lex-bot-responsespecification-allowinterrupt).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allow_interrupt: Option<::Value<bool>>,
        /// Property [`MessageGroupsList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-responsespecification.html#cfn-lex-bot-responsespecification-messagegroupslist).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub message_groups_list: ::ValueList<MessageGroup>,
    }

    impl ::codec::SerializeValue for ResponseSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref allow_interrupt) = self.allow_interrupt {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowInterrupt", allow_interrupt)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MessageGroupsList", &self.message_groups_list)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResponseSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResponseSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResponseSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResponseSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allow_interrupt: Option<::Value<bool>> = None;
                    let mut message_groups_list: Option<::ValueList<MessageGroup>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllowInterrupt" => {
                                allow_interrupt = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MessageGroupsList" => {
                                message_groups_list = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResponseSpecification {
                        allow_interrupt: allow_interrupt,
                        message_groups_list: message_groups_list.ok_or(::serde::de::Error::missing_field("MessageGroupsList"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.S3BucketLogDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-s3bucketlogdestination.html) property type.
    #[derive(Debug, Default)]
    pub struct S3BucketLogDestination {
        /// Property [`KmsKeyArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-s3bucketlogdestination.html#cfn-lex-bot-s3bucketlogdestination-kmskeyarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kms_key_arn: Option<::Value<String>>,
        /// Property [`LogPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-s3bucketlogdestination.html#cfn-lex-bot-s3bucketlogdestination-logprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_prefix: ::Value<String>,
        /// Property [`S3BucketArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-s3bucketlogdestination.html#cfn-lex-bot-s3bucketlogdestination-s3bucketarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_bucket_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for S3BucketLogDestination {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref kms_key_arn) = self.kms_key_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyArn", kms_key_arn)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogPrefix", &self.log_prefix)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3BucketArn", &self.s3_bucket_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3BucketLogDestination {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3BucketLogDestination, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3BucketLogDestination;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3BucketLogDestination")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut kms_key_arn: Option<::Value<String>> = None;
                    let mut log_prefix: Option<::Value<String>> = None;
                    let mut s3_bucket_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KmsKeyArn" => {
                                kms_key_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogPrefix" => {
                                log_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3BucketArn" => {
                                s3_bucket_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3BucketLogDestination {
                        kms_key_arn: kms_key_arn,
                        log_prefix: log_prefix.ok_or(::serde::de::Error::missing_field("LogPrefix"))?,
                        s3_bucket_arn: s3_bucket_arn.ok_or(::serde::de::Error::missing_field("S3BucketArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.S3Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-s3location.html) property type.
    #[derive(Debug, Default)]
    pub struct S3Location {
        /// Property [`S3Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-s3location.html#cfn-lex-bot-s3location-s3bucket).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_bucket: ::Value<String>,
        /// Property [`S3ObjectKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-s3location.html#cfn-lex-bot-s3location-s3objectkey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_object_key: ::Value<String>,
        /// Property [`S3ObjectVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-s3location.html#cfn-lex-bot-s3location-s3objectversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_object_version: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for S3Location {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Bucket", &self.s3_bucket)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3ObjectKey", &self.s3_object_key)?;
            if let Some(ref s3_object_version) = self.s3_object_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3ObjectVersion", s3_object_version)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3Location {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3Location, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3Location;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3Location")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut s3_bucket: Option<::Value<String>> = None;
                    let mut s3_object_key: Option<::Value<String>> = None;
                    let mut s3_object_version: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3Bucket" => {
                                s3_bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3ObjectKey" => {
                                s3_object_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3ObjectVersion" => {
                                s3_object_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Location {
                        s3_bucket: s3_bucket.ok_or(::serde::de::Error::missing_field("S3Bucket"))?,
                        s3_object_key: s3_object_key.ok_or(::serde::de::Error::missing_field("S3ObjectKey"))?,
                        s3_object_version: s3_object_version,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.SSMLMessage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-ssmlmessage.html) property type.
    #[derive(Debug, Default)]
    pub struct SSMLMessage {
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-ssmlmessage.html#cfn-lex-bot-ssmlmessage-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for SSMLMessage {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SSMLMessage {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SSMLMessage, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SSMLMessage;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SSMLMessage")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SSMLMessage {
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.SampleUtterance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-sampleutterance.html) property type.
    #[derive(Debug, Default)]
    pub struct SampleUtterance {
        /// Property [`Utterance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-sampleutterance.html#cfn-lex-bot-sampleutterance-utterance).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub utterance: ::Value<String>,
    }

    impl ::codec::SerializeValue for SampleUtterance {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Utterance", &self.utterance)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SampleUtterance {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SampleUtterance, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SampleUtterance;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SampleUtterance")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut utterance: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Utterance" => {
                                utterance = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SampleUtterance {
                        utterance: utterance.ok_or(::serde::de::Error::missing_field("Utterance"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.SampleValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-samplevalue.html) property type.
    #[derive(Debug, Default)]
    pub struct SampleValue {
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-samplevalue.html#cfn-lex-bot-samplevalue-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for SampleValue {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SampleValue {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SampleValue, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SampleValue;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SampleValue")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SampleValue {
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.SentimentAnalysisSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-sentimentanalysissettings.html) property type.
    #[derive(Debug, Default)]
    pub struct SentimentAnalysisSettings {
        /// Property [`DetectSentiment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-sentimentanalysissettings.html#cfn-lex-bot-sentimentanalysissettings-detectsentiment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub detect_sentiment: ::Value<bool>,
    }

    impl ::codec::SerializeValue for SentimentAnalysisSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DetectSentiment", &self.detect_sentiment)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SentimentAnalysisSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SentimentAnalysisSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SentimentAnalysisSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SentimentAnalysisSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut detect_sentiment: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DetectSentiment" => {
                                detect_sentiment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SentimentAnalysisSettings {
                        detect_sentiment: detect_sentiment.ok_or(::serde::de::Error::missing_field("DetectSentiment"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.SessionAttribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-sessionattribute.html) property type.
    #[derive(Debug, Default)]
    pub struct SessionAttribute {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-sessionattribute.html#cfn-lex-bot-sessionattribute-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-sessionattribute.html#cfn-lex-bot-sessionattribute-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SessionAttribute {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SessionAttribute {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SessionAttribute, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SessionAttribute;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SessionAttribute")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SessionAttribute {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.Slot`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slot.html) property type.
    #[derive(Debug, Default)]
    pub struct Slot {
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slot.html#cfn-lex-bot-slot-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`MultipleValuesSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slot.html#cfn-lex-bot-slot-multiplevaluessetting).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub multiple_values_setting: Option<::Value<MultipleValuesSetting>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slot.html#cfn-lex-bot-slot-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`ObfuscationSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slot.html#cfn-lex-bot-slot-obfuscationsetting).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub obfuscation_setting: Option<::Value<ObfuscationSetting>>,
        /// Property [`SlotTypeName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slot.html#cfn-lex-bot-slot-slottypename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub slot_type_name: ::Value<String>,
        /// Property [`ValueElicitationSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slot.html#cfn-lex-bot-slot-valueelicitationsetting).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value_elicitation_setting: ::Value<SlotValueElicitationSetting>,
    }

    impl ::codec::SerializeValue for Slot {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            if let Some(ref multiple_values_setting) = self.multiple_values_setting {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MultipleValuesSetting", multiple_values_setting)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            if let Some(ref obfuscation_setting) = self.obfuscation_setting {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ObfuscationSetting", obfuscation_setting)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SlotTypeName", &self.slot_type_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ValueElicitationSetting", &self.value_elicitation_setting)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Slot {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Slot, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Slot;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Slot")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut description: Option<::Value<String>> = None;
                    let mut multiple_values_setting: Option<::Value<MultipleValuesSetting>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut obfuscation_setting: Option<::Value<ObfuscationSetting>> = None;
                    let mut slot_type_name: Option<::Value<String>> = None;
                    let mut value_elicitation_setting: Option<::Value<SlotValueElicitationSetting>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MultipleValuesSetting" => {
                                multiple_values_setting = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ObfuscationSetting" => {
                                obfuscation_setting = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SlotTypeName" => {
                                slot_type_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ValueElicitationSetting" => {
                                value_elicitation_setting = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Slot {
                        description: description,
                        multiple_values_setting: multiple_values_setting,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        obfuscation_setting: obfuscation_setting,
                        slot_type_name: slot_type_name.ok_or(::serde::de::Error::missing_field("SlotTypeName"))?,
                        value_elicitation_setting: value_elicitation_setting.ok_or(::serde::de::Error::missing_field("ValueElicitationSetting"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.SlotCaptureSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotcapturesetting.html) property type.
    #[derive(Debug, Default)]
    pub struct SlotCaptureSetting {
        /// Property [`CaptureConditional`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotcapturesetting.html#cfn-lex-bot-slotcapturesetting-captureconditional).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub capture_conditional: Option<::Value<ConditionalSpecification>>,
        /// Property [`CaptureNextStep`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotcapturesetting.html#cfn-lex-bot-slotcapturesetting-capturenextstep).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub capture_next_step: Option<::Value<DialogState>>,
        /// Property [`CaptureResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotcapturesetting.html#cfn-lex-bot-slotcapturesetting-captureresponse).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub capture_response: Option<::Value<ResponseSpecification>>,
        /// Property [`CodeHook`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotcapturesetting.html#cfn-lex-bot-slotcapturesetting-codehook).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub code_hook: Option<::Value<DialogCodeHookInvocationSetting>>,
        /// Property [`ElicitationCodeHook`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotcapturesetting.html#cfn-lex-bot-slotcapturesetting-elicitationcodehook).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub elicitation_code_hook: Option<::Value<ElicitationCodeHookInvocationSetting>>,
        /// Property [`FailureConditional`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotcapturesetting.html#cfn-lex-bot-slotcapturesetting-failureconditional).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub failure_conditional: Option<::Value<ConditionalSpecification>>,
        /// Property [`FailureNextStep`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotcapturesetting.html#cfn-lex-bot-slotcapturesetting-failurenextstep).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub failure_next_step: Option<::Value<DialogState>>,
        /// Property [`FailureResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotcapturesetting.html#cfn-lex-bot-slotcapturesetting-failureresponse).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub failure_response: Option<::Value<ResponseSpecification>>,
    }

    impl ::codec::SerializeValue for SlotCaptureSetting {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref capture_conditional) = self.capture_conditional {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CaptureConditional", capture_conditional)?;
            }
            if let Some(ref capture_next_step) = self.capture_next_step {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CaptureNextStep", capture_next_step)?;
            }
            if let Some(ref capture_response) = self.capture_response {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CaptureResponse", capture_response)?;
            }
            if let Some(ref code_hook) = self.code_hook {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CodeHook", code_hook)?;
            }
            if let Some(ref elicitation_code_hook) = self.elicitation_code_hook {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ElicitationCodeHook", elicitation_code_hook)?;
            }
            if let Some(ref failure_conditional) = self.failure_conditional {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FailureConditional", failure_conditional)?;
            }
            if let Some(ref failure_next_step) = self.failure_next_step {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FailureNextStep", failure_next_step)?;
            }
            if let Some(ref failure_response) = self.failure_response {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FailureResponse", failure_response)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SlotCaptureSetting {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SlotCaptureSetting, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SlotCaptureSetting;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SlotCaptureSetting")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut capture_conditional: Option<::Value<ConditionalSpecification>> = None;
                    let mut capture_next_step: Option<::Value<DialogState>> = None;
                    let mut capture_response: Option<::Value<ResponseSpecification>> = None;
                    let mut code_hook: Option<::Value<DialogCodeHookInvocationSetting>> = None;
                    let mut elicitation_code_hook: Option<::Value<ElicitationCodeHookInvocationSetting>> = None;
                    let mut failure_conditional: Option<::Value<ConditionalSpecification>> = None;
                    let mut failure_next_step: Option<::Value<DialogState>> = None;
                    let mut failure_response: Option<::Value<ResponseSpecification>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CaptureConditional" => {
                                capture_conditional = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CaptureNextStep" => {
                                capture_next_step = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CaptureResponse" => {
                                capture_response = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CodeHook" => {
                                code_hook = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ElicitationCodeHook" => {
                                elicitation_code_hook = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FailureConditional" => {
                                failure_conditional = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FailureNextStep" => {
                                failure_next_step = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FailureResponse" => {
                                failure_response = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SlotCaptureSetting {
                        capture_conditional: capture_conditional,
                        capture_next_step: capture_next_step,
                        capture_response: capture_response,
                        code_hook: code_hook,
                        elicitation_code_hook: elicitation_code_hook,
                        failure_conditional: failure_conditional,
                        failure_next_step: failure_next_step,
                        failure_response: failure_response,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.SlotDefaultValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotdefaultvalue.html) property type.
    #[derive(Debug, Default)]
    pub struct SlotDefaultValue {
        /// Property [`DefaultValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotdefaultvalue.html#cfn-lex-bot-slotdefaultvalue-defaultvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_value: ::Value<String>,
    }

    impl ::codec::SerializeValue for SlotDefaultValue {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultValue", &self.default_value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SlotDefaultValue {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SlotDefaultValue, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SlotDefaultValue;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SlotDefaultValue")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut default_value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DefaultValue" => {
                                default_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SlotDefaultValue {
                        default_value: default_value.ok_or(::serde::de::Error::missing_field("DefaultValue"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.SlotDefaultValueSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotdefaultvaluespecification.html) property type.
    #[derive(Debug, Default)]
    pub struct SlotDefaultValueSpecification {
        /// Property [`DefaultValueList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotdefaultvaluespecification.html#cfn-lex-bot-slotdefaultvaluespecification-defaultvaluelist).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_value_list: ::ValueList<SlotDefaultValue>,
    }

    impl ::codec::SerializeValue for SlotDefaultValueSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultValueList", &self.default_value_list)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SlotDefaultValueSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SlotDefaultValueSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SlotDefaultValueSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SlotDefaultValueSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut default_value_list: Option<::ValueList<SlotDefaultValue>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DefaultValueList" => {
                                default_value_list = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SlotDefaultValueSpecification {
                        default_value_list: default_value_list.ok_or(::serde::de::Error::missing_field("DefaultValueList"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.SlotPriority`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotpriority.html) property type.
    #[derive(Debug, Default)]
    pub struct SlotPriority {
        /// Property [`Priority`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotpriority.html#cfn-lex-bot-slotpriority-priority).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub priority: ::Value<u32>,
        /// Property [`SlotName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotpriority.html#cfn-lex-bot-slotpriority-slotname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub slot_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for SlotPriority {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Priority", &self.priority)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SlotName", &self.slot_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SlotPriority {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SlotPriority, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SlotPriority;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SlotPriority")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut priority: Option<::Value<u32>> = None;
                    let mut slot_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Priority" => {
                                priority = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SlotName" => {
                                slot_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SlotPriority {
                        priority: priority.ok_or(::serde::de::Error::missing_field("Priority"))?,
                        slot_name: slot_name.ok_or(::serde::de::Error::missing_field("SlotName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.SlotType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slottype.html) property type.
    #[derive(Debug, Default)]
    pub struct SlotType {
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slottype.html#cfn-lex-bot-slottype-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`ExternalSourceSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slottype.html#cfn-lex-bot-slottype-externalsourcesetting).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub external_source_setting: Option<::Value<ExternalSourceSetting>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slottype.html#cfn-lex-bot-slottype-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`ParentSlotTypeSignature`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slottype.html#cfn-lex-bot-slottype-parentslottypesignature).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parent_slot_type_signature: Option<::Value<String>>,
        /// Property [`SlotTypeValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slottype.html#cfn-lex-bot-slottype-slottypevalues).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub slot_type_values: Option<::ValueList<SlotTypeValue>>,
        /// Property [`ValueSelectionSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slottype.html#cfn-lex-bot-slottype-valueselectionsetting).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value_selection_setting: Option<::Value<SlotValueSelectionSetting>>,
    }

    impl ::codec::SerializeValue for SlotType {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            if let Some(ref external_source_setting) = self.external_source_setting {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExternalSourceSetting", external_source_setting)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            if let Some(ref parent_slot_type_signature) = self.parent_slot_type_signature {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParentSlotTypeSignature", parent_slot_type_signature)?;
            }
            if let Some(ref slot_type_values) = self.slot_type_values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SlotTypeValues", slot_type_values)?;
            }
            if let Some(ref value_selection_setting) = self.value_selection_setting {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ValueSelectionSetting", value_selection_setting)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SlotType {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SlotType, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SlotType;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SlotType")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut description: Option<::Value<String>> = None;
                    let mut external_source_setting: Option<::Value<ExternalSourceSetting>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut parent_slot_type_signature: Option<::Value<String>> = None;
                    let mut slot_type_values: Option<::ValueList<SlotTypeValue>> = None;
                    let mut value_selection_setting: Option<::Value<SlotValueSelectionSetting>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExternalSourceSetting" => {
                                external_source_setting = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ParentSlotTypeSignature" => {
                                parent_slot_type_signature = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SlotTypeValues" => {
                                slot_type_values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ValueSelectionSetting" => {
                                value_selection_setting = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SlotType {
                        description: description,
                        external_source_setting: external_source_setting,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        parent_slot_type_signature: parent_slot_type_signature,
                        slot_type_values: slot_type_values,
                        value_selection_setting: value_selection_setting,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.SlotTypeValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slottypevalue.html) property type.
    #[derive(Debug, Default)]
    pub struct SlotTypeValue {
        /// Property [`SampleValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slottypevalue.html#cfn-lex-bot-slottypevalue-samplevalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sample_value: ::Value<SampleValue>,
        /// Property [`Synonyms`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slottypevalue.html#cfn-lex-bot-slottypevalue-synonyms).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub synonyms: Option<::ValueList<SampleValue>>,
    }

    impl ::codec::SerializeValue for SlotTypeValue {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SampleValue", &self.sample_value)?;
            if let Some(ref synonyms) = self.synonyms {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Synonyms", synonyms)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SlotTypeValue {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SlotTypeValue, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SlotTypeValue;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SlotTypeValue")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut sample_value: Option<::Value<SampleValue>> = None;
                    let mut synonyms: Option<::ValueList<SampleValue>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SampleValue" => {
                                sample_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Synonyms" => {
                                synonyms = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SlotTypeValue {
                        sample_value: sample_value.ok_or(::serde::de::Error::missing_field("SampleValue"))?,
                        synonyms: synonyms,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.SlotValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotvalue.html) property type.
    #[derive(Debug, Default)]
    pub struct SlotValue {
        /// Property [`InterpretedValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotvalue.html#cfn-lex-bot-slotvalue-interpretedvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub interpreted_value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SlotValue {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref interpreted_value) = self.interpreted_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InterpretedValue", interpreted_value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SlotValue {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SlotValue, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SlotValue;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SlotValue")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut interpreted_value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InterpretedValue" => {
                                interpreted_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SlotValue {
                        interpreted_value: interpreted_value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.SlotValueElicitationSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotvalueelicitationsetting.html) property type.
    #[derive(Debug, Default)]
    pub struct SlotValueElicitationSetting {
        /// Property [`DefaultValueSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotvalueelicitationsetting.html#cfn-lex-bot-slotvalueelicitationsetting-defaultvaluespecification).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_value_specification: Option<::Value<SlotDefaultValueSpecification>>,
        /// Property [`PromptSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotvalueelicitationsetting.html#cfn-lex-bot-slotvalueelicitationsetting-promptspecification).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub prompt_specification: Option<::Value<PromptSpecification>>,
        /// Property [`SampleUtterances`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotvalueelicitationsetting.html#cfn-lex-bot-slotvalueelicitationsetting-sampleutterances).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sample_utterances: Option<::ValueList<SampleUtterance>>,
        /// Property [`SlotCaptureSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotvalueelicitationsetting.html#cfn-lex-bot-slotvalueelicitationsetting-slotcapturesetting).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub slot_capture_setting: Option<::Value<SlotCaptureSetting>>,
        /// Property [`SlotConstraint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotvalueelicitationsetting.html#cfn-lex-bot-slotvalueelicitationsetting-slotconstraint).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub slot_constraint: ::Value<String>,
        /// Property [`WaitAndContinueSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotvalueelicitationsetting.html#cfn-lex-bot-slotvalueelicitationsetting-waitandcontinuespecification).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub wait_and_continue_specification: Option<::Value<WaitAndContinueSpecification>>,
    }

    impl ::codec::SerializeValue for SlotValueElicitationSetting {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref default_value_specification) = self.default_value_specification {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultValueSpecification", default_value_specification)?;
            }
            if let Some(ref prompt_specification) = self.prompt_specification {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PromptSpecification", prompt_specification)?;
            }
            if let Some(ref sample_utterances) = self.sample_utterances {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SampleUtterances", sample_utterances)?;
            }
            if let Some(ref slot_capture_setting) = self.slot_capture_setting {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SlotCaptureSetting", slot_capture_setting)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SlotConstraint", &self.slot_constraint)?;
            if let Some(ref wait_and_continue_specification) = self.wait_and_continue_specification {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WaitAndContinueSpecification", wait_and_continue_specification)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SlotValueElicitationSetting {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SlotValueElicitationSetting, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SlotValueElicitationSetting;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SlotValueElicitationSetting")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut default_value_specification: Option<::Value<SlotDefaultValueSpecification>> = None;
                    let mut prompt_specification: Option<::Value<PromptSpecification>> = None;
                    let mut sample_utterances: Option<::ValueList<SampleUtterance>> = None;
                    let mut slot_capture_setting: Option<::Value<SlotCaptureSetting>> = None;
                    let mut slot_constraint: Option<::Value<String>> = None;
                    let mut wait_and_continue_specification: Option<::Value<WaitAndContinueSpecification>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DefaultValueSpecification" => {
                                default_value_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PromptSpecification" => {
                                prompt_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SampleUtterances" => {
                                sample_utterances = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SlotCaptureSetting" => {
                                slot_capture_setting = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SlotConstraint" => {
                                slot_constraint = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WaitAndContinueSpecification" => {
                                wait_and_continue_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SlotValueElicitationSetting {
                        default_value_specification: default_value_specification,
                        prompt_specification: prompt_specification,
                        sample_utterances: sample_utterances,
                        slot_capture_setting: slot_capture_setting,
                        slot_constraint: slot_constraint.ok_or(::serde::de::Error::missing_field("SlotConstraint"))?,
                        wait_and_continue_specification: wait_and_continue_specification,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.SlotValueOverride`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotvalueoverride.html) property type.
    #[derive(Debug, Default)]
    pub struct SlotValueOverride {
        /// Property [`Shape`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotvalueoverride.html#cfn-lex-bot-slotvalueoverride-shape).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub shape: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotvalueoverride.html#cfn-lex-bot-slotvalueoverride-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<SlotValue>>,
        /// Property [`Values`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotvalueoverride.html#cfn-lex-bot-slotvalueoverride-values).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub values: Option<::ValueList<SlotValueOverride>>,
    }

    impl ::codec::SerializeValue for SlotValueOverride {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref shape) = self.shape {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Shape", shape)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            if let Some(ref values) = self.values {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", values)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SlotValueOverride {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SlotValueOverride, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SlotValueOverride;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SlotValueOverride")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut shape: Option<::Value<String>> = None;
                    let mut value: Option<::Value<SlotValue>> = None;
                    let mut values: Option<::ValueList<SlotValueOverride>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Shape" => {
                                shape = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Values" => {
                                values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SlotValueOverride {
                        shape: shape,
                        value: value,
                        values: values,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.SlotValueOverrideMap`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotvalueoverridemap.html) property type.
    #[derive(Debug, Default)]
    pub struct SlotValueOverrideMap {
        /// Property [`SlotName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotvalueoverridemap.html#cfn-lex-bot-slotvalueoverridemap-slotname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub slot_name: Option<::Value<String>>,
        /// Property [`SlotValueOverride`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotvalueoverridemap.html#cfn-lex-bot-slotvalueoverridemap-slotvalueoverride).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub slot_value_override: Option<::Value<SlotValueOverride>>,
    }

    impl ::codec::SerializeValue for SlotValueOverrideMap {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref slot_name) = self.slot_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SlotName", slot_name)?;
            }
            if let Some(ref slot_value_override) = self.slot_value_override {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SlotValueOverride", slot_value_override)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SlotValueOverrideMap {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SlotValueOverrideMap, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SlotValueOverrideMap;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SlotValueOverrideMap")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut slot_name: Option<::Value<String>> = None;
                    let mut slot_value_override: Option<::Value<SlotValueOverride>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SlotName" => {
                                slot_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SlotValueOverride" => {
                                slot_value_override = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SlotValueOverrideMap {
                        slot_name: slot_name,
                        slot_value_override: slot_value_override,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.SlotValueRegexFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotvalueregexfilter.html) property type.
    #[derive(Debug, Default)]
    pub struct SlotValueRegexFilter {
        /// Property [`Pattern`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotvalueregexfilter.html#cfn-lex-bot-slotvalueregexfilter-pattern).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pattern: ::Value<String>,
    }

    impl ::codec::SerializeValue for SlotValueRegexFilter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Pattern", &self.pattern)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SlotValueRegexFilter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SlotValueRegexFilter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SlotValueRegexFilter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SlotValueRegexFilter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut pattern: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Pattern" => {
                                pattern = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SlotValueRegexFilter {
                        pattern: pattern.ok_or(::serde::de::Error::missing_field("Pattern"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.SlotValueSelectionSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotvalueselectionsetting.html) property type.
    #[derive(Debug, Default)]
    pub struct SlotValueSelectionSetting {
        /// Property [`AdvancedRecognitionSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotvalueselectionsetting.html#cfn-lex-bot-slotvalueselectionsetting-advancedrecognitionsetting).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub advanced_recognition_setting: Option<::Value<AdvancedRecognitionSetting>>,
        /// Property [`RegexFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotvalueselectionsetting.html#cfn-lex-bot-slotvalueselectionsetting-regexfilter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub regex_filter: Option<::Value<SlotValueRegexFilter>>,
        /// Property [`ResolutionStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotvalueselectionsetting.html#cfn-lex-bot-slotvalueselectionsetting-resolutionstrategy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resolution_strategy: ::Value<String>,
    }

    impl ::codec::SerializeValue for SlotValueSelectionSetting {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref advanced_recognition_setting) = self.advanced_recognition_setting {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdvancedRecognitionSetting", advanced_recognition_setting)?;
            }
            if let Some(ref regex_filter) = self.regex_filter {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RegexFilter", regex_filter)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResolutionStrategy", &self.resolution_strategy)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SlotValueSelectionSetting {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SlotValueSelectionSetting, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SlotValueSelectionSetting;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SlotValueSelectionSetting")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut advanced_recognition_setting: Option<::Value<AdvancedRecognitionSetting>> = None;
                    let mut regex_filter: Option<::Value<SlotValueRegexFilter>> = None;
                    let mut resolution_strategy: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AdvancedRecognitionSetting" => {
                                advanced_recognition_setting = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RegexFilter" => {
                                regex_filter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResolutionStrategy" => {
                                resolution_strategy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SlotValueSelectionSetting {
                        advanced_recognition_setting: advanced_recognition_setting,
                        regex_filter: regex_filter,
                        resolution_strategy: resolution_strategy.ok_or(::serde::de::Error::missing_field("ResolutionStrategy"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.StillWaitingResponseSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-stillwaitingresponsespecification.html) property type.
    #[derive(Debug, Default)]
    pub struct StillWaitingResponseSpecification {
        /// Property [`AllowInterrupt`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-stillwaitingresponsespecification.html#cfn-lex-bot-stillwaitingresponsespecification-allowinterrupt).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allow_interrupt: Option<::Value<bool>>,
        /// Property [`FrequencyInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-stillwaitingresponsespecification.html#cfn-lex-bot-stillwaitingresponsespecification-frequencyinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub frequency_in_seconds: ::Value<u32>,
        /// Property [`MessageGroupsList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-stillwaitingresponsespecification.html#cfn-lex-bot-stillwaitingresponsespecification-messagegroupslist).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub message_groups_list: ::ValueList<MessageGroup>,
        /// Property [`TimeoutInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-stillwaitingresponsespecification.html#cfn-lex-bot-stillwaitingresponsespecification-timeoutinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timeout_in_seconds: ::Value<u32>,
    }

    impl ::codec::SerializeValue for StillWaitingResponseSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref allow_interrupt) = self.allow_interrupt {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowInterrupt", allow_interrupt)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FrequencyInSeconds", &self.frequency_in_seconds)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MessageGroupsList", &self.message_groups_list)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeoutInSeconds", &self.timeout_in_seconds)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StillWaitingResponseSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StillWaitingResponseSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StillWaitingResponseSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StillWaitingResponseSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allow_interrupt: Option<::Value<bool>> = None;
                    let mut frequency_in_seconds: Option<::Value<u32>> = None;
                    let mut message_groups_list: Option<::ValueList<MessageGroup>> = None;
                    let mut timeout_in_seconds: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllowInterrupt" => {
                                allow_interrupt = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FrequencyInSeconds" => {
                                frequency_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MessageGroupsList" => {
                                message_groups_list = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimeoutInSeconds" => {
                                timeout_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StillWaitingResponseSpecification {
                        allow_interrupt: allow_interrupt,
                        frequency_in_seconds: frequency_in_seconds.ok_or(::serde::de::Error::missing_field("FrequencyInSeconds"))?,
                        message_groups_list: message_groups_list.ok_or(::serde::de::Error::missing_field("MessageGroupsList"))?,
                        timeout_in_seconds: timeout_in_seconds.ok_or(::serde::de::Error::missing_field("TimeoutInSeconds"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.TestBotAliasSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-testbotaliassettings.html) property type.
    #[derive(Debug, Default)]
    pub struct TestBotAliasSettings {
        /// Property [`BotAliasLocaleSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-testbotaliassettings.html#cfn-lex-bot-testbotaliassettings-botaliaslocalesettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bot_alias_locale_settings: Option<::ValueList<BotAliasLocaleSettingsItem>>,
        /// Property [`ConversationLogSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-testbotaliassettings.html#cfn-lex-bot-testbotaliassettings-conversationlogsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub conversation_log_settings: Option<::Value<ConversationLogSettings>>,
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-testbotaliassettings.html#cfn-lex-bot-testbotaliassettings-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`SentimentAnalysisSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-testbotaliassettings.html#cfn-lex-bot-testbotaliassettings-sentimentanalysissettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sentiment_analysis_settings: Option<::Value<SentimentAnalysisSettings>>,
    }

    impl ::codec::SerializeValue for TestBotAliasSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref bot_alias_locale_settings) = self.bot_alias_locale_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BotAliasLocaleSettings", bot_alias_locale_settings)?;
            }
            if let Some(ref conversation_log_settings) = self.conversation_log_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConversationLogSettings", conversation_log_settings)?;
            }
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            if let Some(ref sentiment_analysis_settings) = self.sentiment_analysis_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SentimentAnalysisSettings", sentiment_analysis_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TestBotAliasSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TestBotAliasSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TestBotAliasSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TestBotAliasSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bot_alias_locale_settings: Option<::ValueList<BotAliasLocaleSettingsItem>> = None;
                    let mut conversation_log_settings: Option<::Value<ConversationLogSettings>> = None;
                    let mut description: Option<::Value<String>> = None;
                    let mut sentiment_analysis_settings: Option<::Value<SentimentAnalysisSettings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BotAliasLocaleSettings" => {
                                bot_alias_locale_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConversationLogSettings" => {
                                conversation_log_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SentimentAnalysisSettings" => {
                                sentiment_analysis_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TestBotAliasSettings {
                        bot_alias_locale_settings: bot_alias_locale_settings,
                        conversation_log_settings: conversation_log_settings,
                        description: description,
                        sentiment_analysis_settings: sentiment_analysis_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.TextInputSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-textinputspecification.html) property type.
    #[derive(Debug, Default)]
    pub struct TextInputSpecification {
        /// Property [`StartTimeoutMs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-textinputspecification.html#cfn-lex-bot-textinputspecification-starttimeoutms).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub start_timeout_ms: ::Value<u32>,
    }

    impl ::codec::SerializeValue for TextInputSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartTimeoutMs", &self.start_timeout_ms)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TextInputSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TextInputSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TextInputSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TextInputSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut start_timeout_ms: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "StartTimeoutMs" => {
                                start_timeout_ms = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TextInputSpecification {
                        start_timeout_ms: start_timeout_ms.ok_or(::serde::de::Error::missing_field("StartTimeoutMs"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.TextLogDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-textlogdestination.html) property type.
    #[derive(Debug, Default)]
    pub struct TextLogDestination {
        /// Property [`CloudWatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-textlogdestination.html#cfn-lex-bot-textlogdestination-cloudwatch).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloud_watch: ::Value<CloudWatchLogGroupLogDestination>,
    }

    impl ::codec::SerializeValue for TextLogDestination {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatch", &self.cloud_watch)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TextLogDestination {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TextLogDestination, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TextLogDestination;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TextLogDestination")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cloud_watch: Option<::Value<CloudWatchLogGroupLogDestination>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CloudWatch" => {
                                cloud_watch = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TextLogDestination {
                        cloud_watch: cloud_watch.ok_or(::serde::de::Error::missing_field("CloudWatch"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.TextLogSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-textlogsetting.html) property type.
    #[derive(Debug, Default)]
    pub struct TextLogSetting {
        /// Property [`Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-textlogsetting.html#cfn-lex-bot-textlogsetting-destination).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination: ::Value<TextLogDestination>,
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-textlogsetting.html#cfn-lex-bot-textlogsetting-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: ::Value<bool>,
    }

    impl ::codec::SerializeValue for TextLogSetting {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Destination", &self.destination)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TextLogSetting {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TextLogSetting, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TextLogSetting;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TextLogSetting")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destination: Option<::Value<TextLogDestination>> = None;
                    let mut enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Destination" => {
                                destination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TextLogSetting {
                        destination: destination.ok_or(::serde::de::Error::missing_field("Destination"))?,
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.VoiceSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-voicesettings.html) property type.
    #[derive(Debug, Default)]
    pub struct VoiceSettings {
        /// Property [`Engine`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-voicesettings.html#cfn-lex-bot-voicesettings-engine).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub engine: Option<::Value<String>>,
        /// Property [`VoiceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-voicesettings.html#cfn-lex-bot-voicesettings-voiceid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub voice_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for VoiceSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref engine) = self.engine {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Engine", engine)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VoiceId", &self.voice_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VoiceSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VoiceSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VoiceSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VoiceSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut engine: Option<::Value<String>> = None;
                    let mut voice_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Engine" => {
                                engine = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VoiceId" => {
                                voice_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VoiceSettings {
                        engine: engine,
                        voice_id: voice_id.ok_or(::serde::de::Error::missing_field("VoiceId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::Bot.WaitAndContinueSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-waitandcontinuespecification.html) property type.
    #[derive(Debug, Default)]
    pub struct WaitAndContinueSpecification {
        /// Property [`ContinueResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-waitandcontinuespecification.html#cfn-lex-bot-waitandcontinuespecification-continueresponse).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub continue_response: ::Value<ResponseSpecification>,
        /// Property [`IsActive`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-waitandcontinuespecification.html#cfn-lex-bot-waitandcontinuespecification-isactive).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub is_active: Option<::Value<bool>>,
        /// Property [`StillWaitingResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-waitandcontinuespecification.html#cfn-lex-bot-waitandcontinuespecification-stillwaitingresponse).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub still_waiting_response: Option<::Value<StillWaitingResponseSpecification>>,
        /// Property [`WaitingResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-waitandcontinuespecification.html#cfn-lex-bot-waitandcontinuespecification-waitingresponse).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub waiting_response: ::Value<ResponseSpecification>,
    }

    impl ::codec::SerializeValue for WaitAndContinueSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContinueResponse", &self.continue_response)?;
            if let Some(ref is_active) = self.is_active {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IsActive", is_active)?;
            }
            if let Some(ref still_waiting_response) = self.still_waiting_response {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StillWaitingResponse", still_waiting_response)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WaitingResponse", &self.waiting_response)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for WaitAndContinueSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<WaitAndContinueSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = WaitAndContinueSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type WaitAndContinueSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut continue_response: Option<::Value<ResponseSpecification>> = None;
                    let mut is_active: Option<::Value<bool>> = None;
                    let mut still_waiting_response: Option<::Value<StillWaitingResponseSpecification>> = None;
                    let mut waiting_response: Option<::Value<ResponseSpecification>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContinueResponse" => {
                                continue_response = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IsActive" => {
                                is_active = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StillWaitingResponse" => {
                                still_waiting_response = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WaitingResponse" => {
                                waiting_response = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(WaitAndContinueSpecification {
                        continue_response: continue_response.ok_or(::serde::de::Error::missing_field("ContinueResponse"))?,
                        is_active: is_active,
                        still_waiting_response: still_waiting_response,
                        waiting_response: waiting_response.ok_or(::serde::de::Error::missing_field("WaitingResponse"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod bot_alias {
    //! Property types for the `BotAlias` resource.

    /// The [`AWS::Lex::BotAlias.AudioLogDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-audiologdestination.html) property type.
    #[derive(Debug, Default)]
    pub struct AudioLogDestination {
        /// Property [`S3Bucket`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-audiologdestination.html#cfn-lex-botalias-audiologdestination-s3bucket).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_bucket: ::Value<S3BucketLogDestination>,
    }

    impl ::codec::SerializeValue for AudioLogDestination {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Bucket", &self.s3_bucket)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AudioLogDestination {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AudioLogDestination, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AudioLogDestination;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AudioLogDestination")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut s3_bucket: Option<::Value<S3BucketLogDestination>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3Bucket" => {
                                s3_bucket = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AudioLogDestination {
                        s3_bucket: s3_bucket.ok_or(::serde::de::Error::missing_field("S3Bucket"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::BotAlias.AudioLogSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-audiologsetting.html) property type.
    #[derive(Debug, Default)]
    pub struct AudioLogSetting {
        /// Property [`Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-audiologsetting.html#cfn-lex-botalias-audiologsetting-destination).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination: ::Value<AudioLogDestination>,
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-audiologsetting.html#cfn-lex-botalias-audiologsetting-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: ::Value<bool>,
    }

    impl ::codec::SerializeValue for AudioLogSetting {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Destination", &self.destination)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AudioLogSetting {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AudioLogSetting, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AudioLogSetting;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AudioLogSetting")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destination: Option<::Value<AudioLogDestination>> = None;
                    let mut enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Destination" => {
                                destination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AudioLogSetting {
                        destination: destination.ok_or(::serde::de::Error::missing_field("Destination"))?,
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::BotAlias.BotAliasLocaleSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-botaliaslocalesettings.html) property type.
    #[derive(Debug, Default)]
    pub struct BotAliasLocaleSettings {
        /// Property [`CodeHookSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-botaliaslocalesettings.html#cfn-lex-botalias-botaliaslocalesettings-codehookspecification).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub code_hook_specification: Option<::Value<CodeHookSpecification>>,
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-botaliaslocalesettings.html#cfn-lex-botalias-botaliaslocalesettings-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: ::Value<bool>,
    }

    impl ::codec::SerializeValue for BotAliasLocaleSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref code_hook_specification) = self.code_hook_specification {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CodeHookSpecification", code_hook_specification)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BotAliasLocaleSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BotAliasLocaleSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BotAliasLocaleSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BotAliasLocaleSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut code_hook_specification: Option<::Value<CodeHookSpecification>> = None;
                    let mut enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CodeHookSpecification" => {
                                code_hook_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BotAliasLocaleSettings {
                        code_hook_specification: code_hook_specification,
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::BotAlias.BotAliasLocaleSettingsItem`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-botaliaslocalesettingsitem.html) property type.
    #[derive(Debug, Default)]
    pub struct BotAliasLocaleSettingsItem {
        /// Property [`BotAliasLocaleSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-botaliaslocalesettingsitem.html#cfn-lex-botalias-botaliaslocalesettingsitem-botaliaslocalesetting).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bot_alias_locale_setting: ::Value<BotAliasLocaleSettings>,
        /// Property [`LocaleId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-botaliaslocalesettingsitem.html#cfn-lex-botalias-botaliaslocalesettingsitem-localeid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub locale_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for BotAliasLocaleSettingsItem {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BotAliasLocaleSetting", &self.bot_alias_locale_setting)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LocaleId", &self.locale_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BotAliasLocaleSettingsItem {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BotAliasLocaleSettingsItem, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BotAliasLocaleSettingsItem;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BotAliasLocaleSettingsItem")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bot_alias_locale_setting: Option<::Value<BotAliasLocaleSettings>> = None;
                    let mut locale_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BotAliasLocaleSetting" => {
                                bot_alias_locale_setting = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LocaleId" => {
                                locale_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BotAliasLocaleSettingsItem {
                        bot_alias_locale_setting: bot_alias_locale_setting.ok_or(::serde::de::Error::missing_field("BotAliasLocaleSetting"))?,
                        locale_id: locale_id.ok_or(::serde::de::Error::missing_field("LocaleId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::BotAlias.CloudWatchLogGroupLogDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-cloudwatchloggrouplogdestination.html) property type.
    #[derive(Debug, Default)]
    pub struct CloudWatchLogGroupLogDestination {
        /// Property [`CloudWatchLogGroupArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-cloudwatchloggrouplogdestination.html#cfn-lex-botalias-cloudwatchloggrouplogdestination-cloudwatchloggrouparn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloud_watch_log_group_arn: ::Value<String>,
        /// Property [`LogPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-cloudwatchloggrouplogdestination.html#cfn-lex-botalias-cloudwatchloggrouplogdestination-logprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_prefix: ::Value<String>,
    }

    impl ::codec::SerializeValue for CloudWatchLogGroupLogDestination {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatchLogGroupArn", &self.cloud_watch_log_group_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogPrefix", &self.log_prefix)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CloudWatchLogGroupLogDestination {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CloudWatchLogGroupLogDestination, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CloudWatchLogGroupLogDestination;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CloudWatchLogGroupLogDestination")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cloud_watch_log_group_arn: Option<::Value<String>> = None;
                    let mut log_prefix: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CloudWatchLogGroupArn" => {
                                cloud_watch_log_group_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogPrefix" => {
                                log_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CloudWatchLogGroupLogDestination {
                        cloud_watch_log_group_arn: cloud_watch_log_group_arn.ok_or(::serde::de::Error::missing_field("CloudWatchLogGroupArn"))?,
                        log_prefix: log_prefix.ok_or(::serde::de::Error::missing_field("LogPrefix"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::BotAlias.CodeHookSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-codehookspecification.html) property type.
    #[derive(Debug, Default)]
    pub struct CodeHookSpecification {
        /// Property [`LambdaCodeHook`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-codehookspecification.html#cfn-lex-botalias-codehookspecification-lambdacodehook).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lambda_code_hook: ::Value<LambdaCodeHook>,
    }

    impl ::codec::SerializeValue for CodeHookSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaCodeHook", &self.lambda_code_hook)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CodeHookSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CodeHookSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CodeHookSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CodeHookSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut lambda_code_hook: Option<::Value<LambdaCodeHook>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LambdaCodeHook" => {
                                lambda_code_hook = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CodeHookSpecification {
                        lambda_code_hook: lambda_code_hook.ok_or(::serde::de::Error::missing_field("LambdaCodeHook"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::BotAlias.ConversationLogSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-conversationlogsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct ConversationLogSettings {
        /// Property [`AudioLogSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-conversationlogsettings.html#cfn-lex-botalias-conversationlogsettings-audiologsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub audio_log_settings: Option<::ValueList<AudioLogSetting>>,
        /// Property [`TextLogSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-conversationlogsettings.html#cfn-lex-botalias-conversationlogsettings-textlogsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text_log_settings: Option<::ValueList<TextLogSetting>>,
    }

    impl ::codec::SerializeValue for ConversationLogSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref audio_log_settings) = self.audio_log_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AudioLogSettings", audio_log_settings)?;
            }
            if let Some(ref text_log_settings) = self.text_log_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextLogSettings", text_log_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConversationLogSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConversationLogSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConversationLogSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConversationLogSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut audio_log_settings: Option<::ValueList<AudioLogSetting>> = None;
                    let mut text_log_settings: Option<::ValueList<TextLogSetting>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AudioLogSettings" => {
                                audio_log_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TextLogSettings" => {
                                text_log_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConversationLogSettings {
                        audio_log_settings: audio_log_settings,
                        text_log_settings: text_log_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::BotAlias.LambdaCodeHook`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-lambdacodehook.html) property type.
    #[derive(Debug, Default)]
    pub struct LambdaCodeHook {
        /// Property [`CodeHookInterfaceVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-lambdacodehook.html#cfn-lex-botalias-lambdacodehook-codehookinterfaceversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub code_hook_interface_version: ::Value<String>,
        /// Property [`LambdaArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-lambdacodehook.html#cfn-lex-botalias-lambdacodehook-lambdaarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lambda_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for LambdaCodeHook {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CodeHookInterfaceVersion", &self.code_hook_interface_version)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaArn", &self.lambda_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LambdaCodeHook {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LambdaCodeHook, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LambdaCodeHook;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LambdaCodeHook")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut code_hook_interface_version: Option<::Value<String>> = None;
                    let mut lambda_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CodeHookInterfaceVersion" => {
                                code_hook_interface_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LambdaArn" => {
                                lambda_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LambdaCodeHook {
                        code_hook_interface_version: code_hook_interface_version.ok_or(::serde::de::Error::missing_field("CodeHookInterfaceVersion"))?,
                        lambda_arn: lambda_arn.ok_or(::serde::de::Error::missing_field("LambdaArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::BotAlias.S3BucketLogDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-s3bucketlogdestination.html) property type.
    #[derive(Debug, Default)]
    pub struct S3BucketLogDestination {
        /// Property [`KmsKeyArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-s3bucketlogdestination.html#cfn-lex-botalias-s3bucketlogdestination-kmskeyarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kms_key_arn: Option<::Value<String>>,
        /// Property [`LogPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-s3bucketlogdestination.html#cfn-lex-botalias-s3bucketlogdestination-logprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_prefix: ::Value<String>,
        /// Property [`S3BucketArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-s3bucketlogdestination.html#cfn-lex-botalias-s3bucketlogdestination-s3bucketarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_bucket_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for S3BucketLogDestination {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref kms_key_arn) = self.kms_key_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyArn", kms_key_arn)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogPrefix", &self.log_prefix)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3BucketArn", &self.s3_bucket_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3BucketLogDestination {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3BucketLogDestination, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3BucketLogDestination;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3BucketLogDestination")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut kms_key_arn: Option<::Value<String>> = None;
                    let mut log_prefix: Option<::Value<String>> = None;
                    let mut s3_bucket_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KmsKeyArn" => {
                                kms_key_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogPrefix" => {
                                log_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3BucketArn" => {
                                s3_bucket_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3BucketLogDestination {
                        kms_key_arn: kms_key_arn,
                        log_prefix: log_prefix.ok_or(::serde::de::Error::missing_field("LogPrefix"))?,
                        s3_bucket_arn: s3_bucket_arn.ok_or(::serde::de::Error::missing_field("S3BucketArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::BotAlias.SentimentAnalysisSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-sentimentanalysissettings.html) property type.
    #[derive(Debug, Default)]
    pub struct SentimentAnalysisSettings {
        /// Property [`DetectSentiment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-sentimentanalysissettings.html#cfn-lex-botalias-sentimentanalysissettings-detectsentiment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub detect_sentiment: ::Value<bool>,
    }

    impl ::codec::SerializeValue for SentimentAnalysisSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DetectSentiment", &self.detect_sentiment)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SentimentAnalysisSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SentimentAnalysisSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SentimentAnalysisSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SentimentAnalysisSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut detect_sentiment: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DetectSentiment" => {
                                detect_sentiment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SentimentAnalysisSettings {
                        detect_sentiment: detect_sentiment.ok_or(::serde::de::Error::missing_field("DetectSentiment"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::BotAlias.TextLogDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-textlogdestination.html) property type.
    #[derive(Debug, Default)]
    pub struct TextLogDestination {
        /// Property [`CloudWatch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-textlogdestination.html#cfn-lex-botalias-textlogdestination-cloudwatch).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloud_watch: ::Value<CloudWatchLogGroupLogDestination>,
    }

    impl ::codec::SerializeValue for TextLogDestination {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudWatch", &self.cloud_watch)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TextLogDestination {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TextLogDestination, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TextLogDestination;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TextLogDestination")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cloud_watch: Option<::Value<CloudWatchLogGroupLogDestination>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CloudWatch" => {
                                cloud_watch = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TextLogDestination {
                        cloud_watch: cloud_watch.ok_or(::serde::de::Error::missing_field("CloudWatch"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::BotAlias.TextLogSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-textlogsetting.html) property type.
    #[derive(Debug, Default)]
    pub struct TextLogSetting {
        /// Property [`Destination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-textlogsetting.html#cfn-lex-botalias-textlogsetting-destination).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination: ::Value<TextLogDestination>,
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-textlogsetting.html#cfn-lex-botalias-textlogsetting-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: ::Value<bool>,
    }

    impl ::codec::SerializeValue for TextLogSetting {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Destination", &self.destination)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TextLogSetting {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TextLogSetting, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TextLogSetting;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TextLogSetting")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut destination: Option<::Value<TextLogDestination>> = None;
                    let mut enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Destination" => {
                                destination = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TextLogSetting {
                        destination: destination.ok_or(::serde::de::Error::missing_field("Destination"))?,
                        enabled: enabled.ok_or(::serde::de::Error::missing_field("Enabled"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod bot_version {
    //! Property types for the `BotVersion` resource.

    /// The [`AWS::Lex::BotVersion.BotVersionLocaleDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botversion-botversionlocaledetails.html) property type.
    #[derive(Debug, Default)]
    pub struct BotVersionLocaleDetails {
        /// Property [`SourceBotVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botversion-botversionlocaledetails.html#cfn-lex-botversion-botversionlocaledetails-sourcebotversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_bot_version: ::Value<String>,
    }

    impl ::codec::SerializeValue for BotVersionLocaleDetails {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceBotVersion", &self.source_bot_version)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BotVersionLocaleDetails {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BotVersionLocaleDetails, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BotVersionLocaleDetails;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BotVersionLocaleDetails")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut source_bot_version: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SourceBotVersion" => {
                                source_bot_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BotVersionLocaleDetails {
                        source_bot_version: source_bot_version.ok_or(::serde::de::Error::missing_field("SourceBotVersion"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Lex::BotVersion.BotVersionLocaleSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botversion-botversionlocalespecification.html) property type.
    #[derive(Debug, Default)]
    pub struct BotVersionLocaleSpecification {
        /// Property [`BotVersionLocaleDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botversion-botversionlocalespecification.html#cfn-lex-botversion-botversionlocalespecification-botversionlocaledetails).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bot_version_locale_details: ::Value<BotVersionLocaleDetails>,
        /// Property [`LocaleId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botversion-botversionlocalespecification.html#cfn-lex-botversion-botversionlocalespecification-localeid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub locale_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for BotVersionLocaleSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BotVersionLocaleDetails", &self.bot_version_locale_details)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LocaleId", &self.locale_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BotVersionLocaleSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BotVersionLocaleSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BotVersionLocaleSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BotVersionLocaleSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bot_version_locale_details: Option<::Value<BotVersionLocaleDetails>> = None;
                    let mut locale_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BotVersionLocaleDetails" => {
                                bot_version_locale_details = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LocaleId" => {
                                locale_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BotVersionLocaleSpecification {
                        bot_version_locale_details: bot_version_locale_details.ok_or(::serde::de::Error::missing_field("BotVersionLocaleDetails"))?,
                        locale_id: locale_id.ok_or(::serde::de::Error::missing_field("LocaleId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
