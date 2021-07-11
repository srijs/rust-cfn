//! Types for the `Cognito` service.

/// The [`AWS::Cognito::IdentityPool`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-identitypool.html) resource type.
#[derive(Debug, Default)]
pub struct IdentityPool {
    properties: IdentityPoolProperties
}

/// Properties for the `IdentityPool` resource.
#[derive(Debug, Default)]
pub struct IdentityPoolProperties {
    /// Property [`AllowClassicFlow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-identitypool.html#cfn-cognito-identitypool-allowclassicflow).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub allow_classic_flow: Option<::Value<bool>>,
    /// Property [`AllowUnauthenticatedIdentities`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-identitypool.html#cfn-cognito-identitypool-allowunauthenticatedidentities).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub allow_unauthenticated_identities: ::Value<bool>,
    /// Property [`CognitoEvents`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-identitypool.html#cfn-cognito-identitypool-cognitoevents).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cognito_events: Option<::Value<::json::Value>>,
    /// Property [`CognitoIdentityProviders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-identitypool.html#cfn-cognito-identitypool-cognitoidentityproviders).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cognito_identity_providers: Option<::ValueList<self::identity_pool::CognitoIdentityProvider>>,
    /// Property [`CognitoStreams`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-identitypool.html#cfn-cognito-identitypool-cognitostreams).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cognito_streams: Option<::Value<self::identity_pool::CognitoStreams>>,
    /// Property [`DeveloperProviderName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-identitypool.html#cfn-cognito-identitypool-developerprovidername).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub developer_provider_name: Option<::Value<String>>,
    /// Property [`IdentityPoolName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-identitypool.html#cfn-cognito-identitypool-identitypoolname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub identity_pool_name: Option<::Value<String>>,
    /// Property [`OpenIdConnectProviderARNs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-identitypool.html#cfn-cognito-identitypool-openidconnectproviderarns).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub open_id_connect_provider_ar_ns: Option<::ValueList<String>>,
    /// Property [`PushSync`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-identitypool.html#cfn-cognito-identitypool-pushsync).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub push_sync: Option<::Value<self::identity_pool::PushSync>>,
    /// Property [`SamlProviderARNs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-identitypool.html#cfn-cognito-identitypool-samlproviderarns).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub saml_provider_ar_ns: Option<::ValueList<String>>,
    /// Property [`SupportedLoginProviders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-identitypool.html#cfn-cognito-identitypool-supportedloginproviders).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub supported_login_providers: Option<::Value<::json::Value>>,
}

impl ::serde::Serialize for IdentityPoolProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref allow_classic_flow) = self.allow_classic_flow {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowClassicFlow", allow_classic_flow)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowUnauthenticatedIdentities", &self.allow_unauthenticated_identities)?;
        if let Some(ref cognito_events) = self.cognito_events {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CognitoEvents", cognito_events)?;
        }
        if let Some(ref cognito_identity_providers) = self.cognito_identity_providers {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CognitoIdentityProviders", cognito_identity_providers)?;
        }
        if let Some(ref cognito_streams) = self.cognito_streams {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CognitoStreams", cognito_streams)?;
        }
        if let Some(ref developer_provider_name) = self.developer_provider_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeveloperProviderName", developer_provider_name)?;
        }
        if let Some(ref identity_pool_name) = self.identity_pool_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdentityPoolName", identity_pool_name)?;
        }
        if let Some(ref open_id_connect_provider_ar_ns) = self.open_id_connect_provider_ar_ns {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OpenIdConnectProviderARNs", open_id_connect_provider_ar_ns)?;
        }
        if let Some(ref push_sync) = self.push_sync {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PushSync", push_sync)?;
        }
        if let Some(ref saml_provider_ar_ns) = self.saml_provider_ar_ns {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SamlProviderARNs", saml_provider_ar_ns)?;
        }
        if let Some(ref supported_login_providers) = self.supported_login_providers {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SupportedLoginProviders", supported_login_providers)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for IdentityPoolProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<IdentityPoolProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = IdentityPoolProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type IdentityPoolProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut allow_classic_flow: Option<::Value<bool>> = None;
                let mut allow_unauthenticated_identities: Option<::Value<bool>> = None;
                let mut cognito_events: Option<::Value<::json::Value>> = None;
                let mut cognito_identity_providers: Option<::ValueList<self::identity_pool::CognitoIdentityProvider>> = None;
                let mut cognito_streams: Option<::Value<self::identity_pool::CognitoStreams>> = None;
                let mut developer_provider_name: Option<::Value<String>> = None;
                let mut identity_pool_name: Option<::Value<String>> = None;
                let mut open_id_connect_provider_ar_ns: Option<::ValueList<String>> = None;
                let mut push_sync: Option<::Value<self::identity_pool::PushSync>> = None;
                let mut saml_provider_ar_ns: Option<::ValueList<String>> = None;
                let mut supported_login_providers: Option<::Value<::json::Value>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AllowClassicFlow" => {
                            allow_classic_flow = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AllowUnauthenticatedIdentities" => {
                            allow_unauthenticated_identities = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CognitoEvents" => {
                            cognito_events = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CognitoIdentityProviders" => {
                            cognito_identity_providers = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CognitoStreams" => {
                            cognito_streams = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DeveloperProviderName" => {
                            developer_provider_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IdentityPoolName" => {
                            identity_pool_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OpenIdConnectProviderARNs" => {
                            open_id_connect_provider_ar_ns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PushSync" => {
                            push_sync = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SamlProviderARNs" => {
                            saml_provider_ar_ns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SupportedLoginProviders" => {
                            supported_login_providers = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(IdentityPoolProperties {
                    allow_classic_flow: allow_classic_flow,
                    allow_unauthenticated_identities: allow_unauthenticated_identities.ok_or(::serde::de::Error::missing_field("AllowUnauthenticatedIdentities"))?,
                    cognito_events: cognito_events,
                    cognito_identity_providers: cognito_identity_providers,
                    cognito_streams: cognito_streams,
                    developer_provider_name: developer_provider_name,
                    identity_pool_name: identity_pool_name,
                    open_id_connect_provider_ar_ns: open_id_connect_provider_ar_ns,
                    push_sync: push_sync,
                    saml_provider_ar_ns: saml_provider_ar_ns,
                    supported_login_providers: supported_login_providers,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for IdentityPool {
    type Properties = IdentityPoolProperties;
    const TYPE: &'static str = "AWS::Cognito::IdentityPool";
    fn properties(&self) -> &IdentityPoolProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut IdentityPoolProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for IdentityPool {}

impl From<IdentityPoolProperties> for IdentityPool {
    fn from(properties: IdentityPoolProperties) -> IdentityPool {
        IdentityPool { properties }
    }
}

/// The [`AWS::Cognito::IdentityPoolRoleAttachment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-identitypoolroleattachment.html) resource type.
#[derive(Debug, Default)]
pub struct IdentityPoolRoleAttachment {
    properties: IdentityPoolRoleAttachmentProperties
}

/// Properties for the `IdentityPoolRoleAttachment` resource.
#[derive(Debug, Default)]
pub struct IdentityPoolRoleAttachmentProperties {
    /// Property [`IdentityPoolId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-identitypoolroleattachment.html#cfn-cognito-identitypoolroleattachment-identitypoolid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub identity_pool_id: ::Value<String>,
    /// Property [`RoleMappings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-identitypoolroleattachment.html#cfn-cognito-identitypoolroleattachment-rolemappings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_mappings: Option<::Value<::json::Value>>,
    /// Property [`Roles`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-identitypoolroleattachment.html#cfn-cognito-identitypoolroleattachment-roles).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub roles: Option<::Value<::json::Value>>,
}

impl ::serde::Serialize for IdentityPoolRoleAttachmentProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdentityPoolId", &self.identity_pool_id)?;
        if let Some(ref role_mappings) = self.role_mappings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleMappings", role_mappings)?;
        }
        if let Some(ref roles) = self.roles {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Roles", roles)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for IdentityPoolRoleAttachmentProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<IdentityPoolRoleAttachmentProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = IdentityPoolRoleAttachmentProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type IdentityPoolRoleAttachmentProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut identity_pool_id: Option<::Value<String>> = None;
                let mut role_mappings: Option<::Value<::json::Value>> = None;
                let mut roles: Option<::Value<::json::Value>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "IdentityPoolId" => {
                            identity_pool_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleMappings" => {
                            role_mappings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Roles" => {
                            roles = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(IdentityPoolRoleAttachmentProperties {
                    identity_pool_id: identity_pool_id.ok_or(::serde::de::Error::missing_field("IdentityPoolId"))?,
                    role_mappings: role_mappings,
                    roles: roles,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for IdentityPoolRoleAttachment {
    type Properties = IdentityPoolRoleAttachmentProperties;
    const TYPE: &'static str = "AWS::Cognito::IdentityPoolRoleAttachment";
    fn properties(&self) -> &IdentityPoolRoleAttachmentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut IdentityPoolRoleAttachmentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for IdentityPoolRoleAttachment {}

impl From<IdentityPoolRoleAttachmentProperties> for IdentityPoolRoleAttachment {
    fn from(properties: IdentityPoolRoleAttachmentProperties) -> IdentityPoolRoleAttachment {
        IdentityPoolRoleAttachment { properties }
    }
}

/// The [`AWS::Cognito::UserPool`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpool.html) resource type.
#[derive(Debug, Default)]
pub struct UserPool {
    properties: UserPoolProperties
}

/// Properties for the `UserPool` resource.
#[derive(Debug, Default)]
pub struct UserPoolProperties {
    /// Property [`AccountRecoverySetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpool.html#cfn-cognito-userpool-accountrecoverysetting).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub account_recovery_setting: Option<::Value<self::user_pool::AccountRecoverySetting>>,
    /// Property [`AdminCreateUserConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpool.html#cfn-cognito-userpool-admincreateuserconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub admin_create_user_config: Option<::Value<self::user_pool::AdminCreateUserConfig>>,
    /// Property [`AliasAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpool.html#cfn-cognito-userpool-aliasattributes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub alias_attributes: Option<::ValueList<String>>,
    /// Property [`AutoVerifiedAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpool.html#cfn-cognito-userpool-autoverifiedattributes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub auto_verified_attributes: Option<::ValueList<String>>,
    /// Property [`DeviceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpool.html#cfn-cognito-userpool-deviceconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub device_configuration: Option<::Value<self::user_pool::DeviceConfiguration>>,
    /// Property [`EmailConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpool.html#cfn-cognito-userpool-emailconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub email_configuration: Option<::Value<self::user_pool::EmailConfiguration>>,
    /// Property [`EmailVerificationMessage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpool.html#cfn-cognito-userpool-emailverificationmessage).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub email_verification_message: Option<::Value<String>>,
    /// Property [`EmailVerificationSubject`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpool.html#cfn-cognito-userpool-emailverificationsubject).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub email_verification_subject: Option<::Value<String>>,
    /// Property [`EnabledMfas`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpool.html#cfn-cognito-userpool-enabledmfas).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enabled_mfas: Option<::ValueList<String>>,
    /// Property [`LambdaConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpool.html#cfn-cognito-userpool-lambdaconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub lambda_config: Option<::Value<self::user_pool::LambdaConfig>>,
    /// Property [`MfaConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpool.html#cfn-cognito-userpool-mfaconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub mfa_configuration: Option<::Value<String>>,
    /// Property [`Policies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpool.html#cfn-cognito-userpool-policies).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub policies: Option<::Value<self::user_pool::Policies>>,
    /// Property [`Schema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpool.html#cfn-cognito-userpool-schema).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub schema: Option<::ValueList<self::user_pool::SchemaAttribute>>,
    /// Property [`SmsAuthenticationMessage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpool.html#cfn-cognito-userpool-smsauthenticationmessage).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sms_authentication_message: Option<::Value<String>>,
    /// Property [`SmsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpool.html#cfn-cognito-userpool-smsconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sms_configuration: Option<::Value<self::user_pool::SmsConfiguration>>,
    /// Property [`SmsVerificationMessage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpool.html#cfn-cognito-userpool-smsverificationmessage).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub sms_verification_message: Option<::Value<String>>,
    /// Property [`UserPoolAddOns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpool.html#cfn-cognito-userpool-userpooladdons).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub user_pool_add_ons: Option<::Value<self::user_pool::UserPoolAddOns>>,
    /// Property [`UserPoolName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpool.html#cfn-cognito-userpool-userpoolname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub user_pool_name: Option<::Value<String>>,
    /// Property [`UserPoolTags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpool.html#cfn-cognito-userpool-userpooltags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub user_pool_tags: Option<::Value<::json::Value>>,
    /// Property [`UsernameAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpool.html#cfn-cognito-userpool-usernameattributes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub username_attributes: Option<::ValueList<String>>,
    /// Property [`UsernameConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpool.html#cfn-cognito-userpool-usernameconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub username_configuration: Option<::Value<self::user_pool::UsernameConfiguration>>,
    /// Property [`VerificationMessageTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpool.html#cfn-cognito-userpool-verificationmessagetemplate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub verification_message_template: Option<::Value<self::user_pool::VerificationMessageTemplate>>,
}

impl ::serde::Serialize for UserPoolProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref account_recovery_setting) = self.account_recovery_setting {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccountRecoverySetting", account_recovery_setting)?;
        }
        if let Some(ref admin_create_user_config) = self.admin_create_user_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdminCreateUserConfig", admin_create_user_config)?;
        }
        if let Some(ref alias_attributes) = self.alias_attributes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AliasAttributes", alias_attributes)?;
        }
        if let Some(ref auto_verified_attributes) = self.auto_verified_attributes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoVerifiedAttributes", auto_verified_attributes)?;
        }
        if let Some(ref device_configuration) = self.device_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceConfiguration", device_configuration)?;
        }
        if let Some(ref email_configuration) = self.email_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EmailConfiguration", email_configuration)?;
        }
        if let Some(ref email_verification_message) = self.email_verification_message {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EmailVerificationMessage", email_verification_message)?;
        }
        if let Some(ref email_verification_subject) = self.email_verification_subject {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EmailVerificationSubject", email_verification_subject)?;
        }
        if let Some(ref enabled_mfas) = self.enabled_mfas {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnabledMfas", enabled_mfas)?;
        }
        if let Some(ref lambda_config) = self.lambda_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaConfig", lambda_config)?;
        }
        if let Some(ref mfa_configuration) = self.mfa_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MfaConfiguration", mfa_configuration)?;
        }
        if let Some(ref policies) = self.policies {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Policies", policies)?;
        }
        if let Some(ref schema) = self.schema {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Schema", schema)?;
        }
        if let Some(ref sms_authentication_message) = self.sms_authentication_message {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SmsAuthenticationMessage", sms_authentication_message)?;
        }
        if let Some(ref sms_configuration) = self.sms_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SmsConfiguration", sms_configuration)?;
        }
        if let Some(ref sms_verification_message) = self.sms_verification_message {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SmsVerificationMessage", sms_verification_message)?;
        }
        if let Some(ref user_pool_add_ons) = self.user_pool_add_ons {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserPoolAddOns", user_pool_add_ons)?;
        }
        if let Some(ref user_pool_name) = self.user_pool_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserPoolName", user_pool_name)?;
        }
        if let Some(ref user_pool_tags) = self.user_pool_tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserPoolTags", user_pool_tags)?;
        }
        if let Some(ref username_attributes) = self.username_attributes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UsernameAttributes", username_attributes)?;
        }
        if let Some(ref username_configuration) = self.username_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UsernameConfiguration", username_configuration)?;
        }
        if let Some(ref verification_message_template) = self.verification_message_template {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VerificationMessageTemplate", verification_message_template)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for UserPoolProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<UserPoolProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = UserPoolProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type UserPoolProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut account_recovery_setting: Option<::Value<self::user_pool::AccountRecoverySetting>> = None;
                let mut admin_create_user_config: Option<::Value<self::user_pool::AdminCreateUserConfig>> = None;
                let mut alias_attributes: Option<::ValueList<String>> = None;
                let mut auto_verified_attributes: Option<::ValueList<String>> = None;
                let mut device_configuration: Option<::Value<self::user_pool::DeviceConfiguration>> = None;
                let mut email_configuration: Option<::Value<self::user_pool::EmailConfiguration>> = None;
                let mut email_verification_message: Option<::Value<String>> = None;
                let mut email_verification_subject: Option<::Value<String>> = None;
                let mut enabled_mfas: Option<::ValueList<String>> = None;
                let mut lambda_config: Option<::Value<self::user_pool::LambdaConfig>> = None;
                let mut mfa_configuration: Option<::Value<String>> = None;
                let mut policies: Option<::Value<self::user_pool::Policies>> = None;
                let mut schema: Option<::ValueList<self::user_pool::SchemaAttribute>> = None;
                let mut sms_authentication_message: Option<::Value<String>> = None;
                let mut sms_configuration: Option<::Value<self::user_pool::SmsConfiguration>> = None;
                let mut sms_verification_message: Option<::Value<String>> = None;
                let mut user_pool_add_ons: Option<::Value<self::user_pool::UserPoolAddOns>> = None;
                let mut user_pool_name: Option<::Value<String>> = None;
                let mut user_pool_tags: Option<::Value<::json::Value>> = None;
                let mut username_attributes: Option<::ValueList<String>> = None;
                let mut username_configuration: Option<::Value<self::user_pool::UsernameConfiguration>> = None;
                let mut verification_message_template: Option<::Value<self::user_pool::VerificationMessageTemplate>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AccountRecoverySetting" => {
                            account_recovery_setting = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AdminCreateUserConfig" => {
                            admin_create_user_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AliasAttributes" => {
                            alias_attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AutoVerifiedAttributes" => {
                            auto_verified_attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DeviceConfiguration" => {
                            device_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EmailConfiguration" => {
                            email_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EmailVerificationMessage" => {
                            email_verification_message = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EmailVerificationSubject" => {
                            email_verification_subject = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnabledMfas" => {
                            enabled_mfas = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LambdaConfig" => {
                            lambda_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MfaConfiguration" => {
                            mfa_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Policies" => {
                            policies = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Schema" => {
                            schema = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SmsAuthenticationMessage" => {
                            sms_authentication_message = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SmsConfiguration" => {
                            sms_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SmsVerificationMessage" => {
                            sms_verification_message = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserPoolAddOns" => {
                            user_pool_add_ons = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserPoolName" => {
                            user_pool_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserPoolTags" => {
                            user_pool_tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UsernameAttributes" => {
                            username_attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UsernameConfiguration" => {
                            username_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VerificationMessageTemplate" => {
                            verification_message_template = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(UserPoolProperties {
                    account_recovery_setting: account_recovery_setting,
                    admin_create_user_config: admin_create_user_config,
                    alias_attributes: alias_attributes,
                    auto_verified_attributes: auto_verified_attributes,
                    device_configuration: device_configuration,
                    email_configuration: email_configuration,
                    email_verification_message: email_verification_message,
                    email_verification_subject: email_verification_subject,
                    enabled_mfas: enabled_mfas,
                    lambda_config: lambda_config,
                    mfa_configuration: mfa_configuration,
                    policies: policies,
                    schema: schema,
                    sms_authentication_message: sms_authentication_message,
                    sms_configuration: sms_configuration,
                    sms_verification_message: sms_verification_message,
                    user_pool_add_ons: user_pool_add_ons,
                    user_pool_name: user_pool_name,
                    user_pool_tags: user_pool_tags,
                    username_attributes: username_attributes,
                    username_configuration: username_configuration,
                    verification_message_template: verification_message_template,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for UserPool {
    type Properties = UserPoolProperties;
    const TYPE: &'static str = "AWS::Cognito::UserPool";
    fn properties(&self) -> &UserPoolProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut UserPoolProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for UserPool {}

impl From<UserPoolProperties> for UserPool {
    fn from(properties: UserPoolProperties) -> UserPool {
        UserPool { properties }
    }
}

/// The [`AWS::Cognito::UserPoolClient`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolclient.html) resource type.
#[derive(Debug, Default)]
pub struct UserPoolClient {
    properties: UserPoolClientProperties
}

/// Properties for the `UserPoolClient` resource.
#[derive(Debug, Default)]
pub struct UserPoolClientProperties {
    /// Property [`AccessTokenValidity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolclient.html#cfn-cognito-userpoolclient-accesstokenvalidity).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub access_token_validity: Option<::Value<u32>>,
    /// Property [`AllowedOAuthFlows`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolclient.html#cfn-cognito-userpoolclient-allowedoauthflows).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub allowed_o_auth_flows: Option<::ValueList<String>>,
    /// Property [`AllowedOAuthFlowsUserPoolClient`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolclient.html#cfn-cognito-userpoolclient-allowedoauthflowsuserpoolclient).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub allowed_o_auth_flows_user_pool_client: Option<::Value<bool>>,
    /// Property [`AllowedOAuthScopes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolclient.html#cfn-cognito-userpoolclient-allowedoauthscopes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub allowed_o_auth_scopes: Option<::ValueList<String>>,
    /// Property [`AnalyticsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolclient.html#cfn-cognito-userpoolclient-analyticsconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub analytics_configuration: Option<::Value<self::user_pool_client::AnalyticsConfiguration>>,
    /// Property [`CallbackURLs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolclient.html#cfn-cognito-userpoolclient-callbackurls).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub callback_ur_ls: Option<::ValueList<String>>,
    /// Property [`ClientName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolclient.html#cfn-cognito-userpoolclient-clientname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub client_name: Option<::Value<String>>,
    /// Property [`DefaultRedirectURI`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolclient.html#cfn-cognito-userpoolclient-defaultredirecturi).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub default_redirect_uri: Option<::Value<String>>,
    /// Property [`EnableTokenRevocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolclient.html#cfn-cognito-userpoolclient-enabletokenrevocation).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enable_token_revocation: Option<::Value<bool>>,
    /// Property [`ExplicitAuthFlows`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolclient.html#cfn-cognito-userpoolclient-explicitauthflows).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub explicit_auth_flows: Option<::ValueList<String>>,
    /// Property [`GenerateSecret`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolclient.html#cfn-cognito-userpoolclient-generatesecret).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub generate_secret: Option<::Value<bool>>,
    /// Property [`IdTokenValidity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolclient.html#cfn-cognito-userpoolclient-idtokenvalidity).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub id_token_validity: Option<::Value<u32>>,
    /// Property [`LogoutURLs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolclient.html#cfn-cognito-userpoolclient-logouturls).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub logout_ur_ls: Option<::ValueList<String>>,
    /// Property [`PreventUserExistenceErrors`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolclient.html#cfn-cognito-userpoolclient-preventuserexistenceerrors).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub prevent_user_existence_errors: Option<::Value<String>>,
    /// Property [`ReadAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolclient.html#cfn-cognito-userpoolclient-readattributes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub read_attributes: Option<::ValueList<String>>,
    /// Property [`RefreshTokenValidity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolclient.html#cfn-cognito-userpoolclient-refreshtokenvalidity).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub refresh_token_validity: Option<::Value<u32>>,
    /// Property [`SupportedIdentityProviders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolclient.html#cfn-cognito-userpoolclient-supportedidentityproviders).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub supported_identity_providers: Option<::ValueList<String>>,
    /// Property [`TokenValidityUnits`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolclient.html#cfn-cognito-userpoolclient-tokenvalidityunits).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub token_validity_units: Option<::Value<self::user_pool_client::TokenValidityUnits>>,
    /// Property [`UserPoolId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolclient.html#cfn-cognito-userpoolclient-userpoolid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub user_pool_id: ::Value<String>,
    /// Property [`WriteAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolclient.html#cfn-cognito-userpoolclient-writeattributes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub write_attributes: Option<::ValueList<String>>,
}

impl ::serde::Serialize for UserPoolClientProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref access_token_validity) = self.access_token_validity {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessTokenValidity", access_token_validity)?;
        }
        if let Some(ref allowed_o_auth_flows) = self.allowed_o_auth_flows {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowedOAuthFlows", allowed_o_auth_flows)?;
        }
        if let Some(ref allowed_o_auth_flows_user_pool_client) = self.allowed_o_auth_flows_user_pool_client {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowedOAuthFlowsUserPoolClient", allowed_o_auth_flows_user_pool_client)?;
        }
        if let Some(ref allowed_o_auth_scopes) = self.allowed_o_auth_scopes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowedOAuthScopes", allowed_o_auth_scopes)?;
        }
        if let Some(ref analytics_configuration) = self.analytics_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AnalyticsConfiguration", analytics_configuration)?;
        }
        if let Some(ref callback_ur_ls) = self.callback_ur_ls {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CallbackURLs", callback_ur_ls)?;
        }
        if let Some(ref client_name) = self.client_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientName", client_name)?;
        }
        if let Some(ref default_redirect_uri) = self.default_redirect_uri {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultRedirectURI", default_redirect_uri)?;
        }
        if let Some(ref enable_token_revocation) = self.enable_token_revocation {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableTokenRevocation", enable_token_revocation)?;
        }
        if let Some(ref explicit_auth_flows) = self.explicit_auth_flows {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExplicitAuthFlows", explicit_auth_flows)?;
        }
        if let Some(ref generate_secret) = self.generate_secret {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GenerateSecret", generate_secret)?;
        }
        if let Some(ref id_token_validity) = self.id_token_validity {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdTokenValidity", id_token_validity)?;
        }
        if let Some(ref logout_ur_ls) = self.logout_ur_ls {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogoutURLs", logout_ur_ls)?;
        }
        if let Some(ref prevent_user_existence_errors) = self.prevent_user_existence_errors {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreventUserExistenceErrors", prevent_user_existence_errors)?;
        }
        if let Some(ref read_attributes) = self.read_attributes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReadAttributes", read_attributes)?;
        }
        if let Some(ref refresh_token_validity) = self.refresh_token_validity {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RefreshTokenValidity", refresh_token_validity)?;
        }
        if let Some(ref supported_identity_providers) = self.supported_identity_providers {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SupportedIdentityProviders", supported_identity_providers)?;
        }
        if let Some(ref token_validity_units) = self.token_validity_units {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TokenValidityUnits", token_validity_units)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserPoolId", &self.user_pool_id)?;
        if let Some(ref write_attributes) = self.write_attributes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WriteAttributes", write_attributes)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for UserPoolClientProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<UserPoolClientProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = UserPoolClientProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type UserPoolClientProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut access_token_validity: Option<::Value<u32>> = None;
                let mut allowed_o_auth_flows: Option<::ValueList<String>> = None;
                let mut allowed_o_auth_flows_user_pool_client: Option<::Value<bool>> = None;
                let mut allowed_o_auth_scopes: Option<::ValueList<String>> = None;
                let mut analytics_configuration: Option<::Value<self::user_pool_client::AnalyticsConfiguration>> = None;
                let mut callback_ur_ls: Option<::ValueList<String>> = None;
                let mut client_name: Option<::Value<String>> = None;
                let mut default_redirect_uri: Option<::Value<String>> = None;
                let mut enable_token_revocation: Option<::Value<bool>> = None;
                let mut explicit_auth_flows: Option<::ValueList<String>> = None;
                let mut generate_secret: Option<::Value<bool>> = None;
                let mut id_token_validity: Option<::Value<u32>> = None;
                let mut logout_ur_ls: Option<::ValueList<String>> = None;
                let mut prevent_user_existence_errors: Option<::Value<String>> = None;
                let mut read_attributes: Option<::ValueList<String>> = None;
                let mut refresh_token_validity: Option<::Value<u32>> = None;
                let mut supported_identity_providers: Option<::ValueList<String>> = None;
                let mut token_validity_units: Option<::Value<self::user_pool_client::TokenValidityUnits>> = None;
                let mut user_pool_id: Option<::Value<String>> = None;
                let mut write_attributes: Option<::ValueList<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AccessTokenValidity" => {
                            access_token_validity = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AllowedOAuthFlows" => {
                            allowed_o_auth_flows = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AllowedOAuthFlowsUserPoolClient" => {
                            allowed_o_auth_flows_user_pool_client = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AllowedOAuthScopes" => {
                            allowed_o_auth_scopes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AnalyticsConfiguration" => {
                            analytics_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CallbackURLs" => {
                            callback_ur_ls = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClientName" => {
                            client_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DefaultRedirectURI" => {
                            default_redirect_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnableTokenRevocation" => {
                            enable_token_revocation = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ExplicitAuthFlows" => {
                            explicit_auth_flows = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GenerateSecret" => {
                            generate_secret = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IdTokenValidity" => {
                            id_token_validity = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LogoutURLs" => {
                            logout_ur_ls = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PreventUserExistenceErrors" => {
                            prevent_user_existence_errors = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReadAttributes" => {
                            read_attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RefreshTokenValidity" => {
                            refresh_token_validity = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SupportedIdentityProviders" => {
                            supported_identity_providers = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TokenValidityUnits" => {
                            token_validity_units = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserPoolId" => {
                            user_pool_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "WriteAttributes" => {
                            write_attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(UserPoolClientProperties {
                    access_token_validity: access_token_validity,
                    allowed_o_auth_flows: allowed_o_auth_flows,
                    allowed_o_auth_flows_user_pool_client: allowed_o_auth_flows_user_pool_client,
                    allowed_o_auth_scopes: allowed_o_auth_scopes,
                    analytics_configuration: analytics_configuration,
                    callback_ur_ls: callback_ur_ls,
                    client_name: client_name,
                    default_redirect_uri: default_redirect_uri,
                    enable_token_revocation: enable_token_revocation,
                    explicit_auth_flows: explicit_auth_flows,
                    generate_secret: generate_secret,
                    id_token_validity: id_token_validity,
                    logout_ur_ls: logout_ur_ls,
                    prevent_user_existence_errors: prevent_user_existence_errors,
                    read_attributes: read_attributes,
                    refresh_token_validity: refresh_token_validity,
                    supported_identity_providers: supported_identity_providers,
                    token_validity_units: token_validity_units,
                    user_pool_id: user_pool_id.ok_or(::serde::de::Error::missing_field("UserPoolId"))?,
                    write_attributes: write_attributes,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for UserPoolClient {
    type Properties = UserPoolClientProperties;
    const TYPE: &'static str = "AWS::Cognito::UserPoolClient";
    fn properties(&self) -> &UserPoolClientProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut UserPoolClientProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for UserPoolClient {}

impl From<UserPoolClientProperties> for UserPoolClient {
    fn from(properties: UserPoolClientProperties) -> UserPoolClient {
        UserPoolClient { properties }
    }
}

/// The [`AWS::Cognito::UserPoolDomain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpooldomain.html) resource type.
#[derive(Debug, Default)]
pub struct UserPoolDomain {
    properties: UserPoolDomainProperties
}

/// Properties for the `UserPoolDomain` resource.
#[derive(Debug, Default)]
pub struct UserPoolDomainProperties {
    /// Property [`CustomDomainConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpooldomain.html#cfn-cognito-userpooldomain-customdomainconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub custom_domain_config: Option<::Value<self::user_pool_domain::CustomDomainConfigType>>,
    /// Property [`Domain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpooldomain.html#cfn-cognito-userpooldomain-domain).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub domain: ::Value<String>,
    /// Property [`UserPoolId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpooldomain.html#cfn-cognito-userpooldomain-userpoolid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub user_pool_id: ::Value<String>,
}

impl ::serde::Serialize for UserPoolDomainProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref custom_domain_config) = self.custom_domain_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomDomainConfig", custom_domain_config)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Domain", &self.domain)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserPoolId", &self.user_pool_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for UserPoolDomainProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<UserPoolDomainProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = UserPoolDomainProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type UserPoolDomainProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut custom_domain_config: Option<::Value<self::user_pool_domain::CustomDomainConfigType>> = None;
                let mut domain: Option<::Value<String>> = None;
                let mut user_pool_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CustomDomainConfig" => {
                            custom_domain_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Domain" => {
                            domain = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserPoolId" => {
                            user_pool_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(UserPoolDomainProperties {
                    custom_domain_config: custom_domain_config,
                    domain: domain.ok_or(::serde::de::Error::missing_field("Domain"))?,
                    user_pool_id: user_pool_id.ok_or(::serde::de::Error::missing_field("UserPoolId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for UserPoolDomain {
    type Properties = UserPoolDomainProperties;
    const TYPE: &'static str = "AWS::Cognito::UserPoolDomain";
    fn properties(&self) -> &UserPoolDomainProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut UserPoolDomainProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for UserPoolDomain {}

impl From<UserPoolDomainProperties> for UserPoolDomain {
    fn from(properties: UserPoolDomainProperties) -> UserPoolDomain {
        UserPoolDomain { properties }
    }
}

/// The [`AWS::Cognito::UserPoolGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolgroup.html) resource type.
#[derive(Debug, Default)]
pub struct UserPoolGroup {
    properties: UserPoolGroupProperties
}

/// Properties for the `UserPoolGroup` resource.
#[derive(Debug, Default)]
pub struct UserPoolGroupProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolgroup.html#cfn-cognito-userpoolgroup-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`GroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolgroup.html#cfn-cognito-userpoolgroup-groupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub group_name: Option<::Value<String>>,
    /// Property [`Precedence`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolgroup.html#cfn-cognito-userpoolgroup-precedence).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub precedence: Option<::Value<f64>>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolgroup.html#cfn-cognito-userpoolgroup-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: Option<::Value<String>>,
    /// Property [`UserPoolId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolgroup.html#cfn-cognito-userpoolgroup-userpoolid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub user_pool_id: ::Value<String>,
}

impl ::serde::Serialize for UserPoolGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref group_name) = self.group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupName", group_name)?;
        }
        if let Some(ref precedence) = self.precedence {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Precedence", precedence)?;
        }
        if let Some(ref role_arn) = self.role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", role_arn)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserPoolId", &self.user_pool_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for UserPoolGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<UserPoolGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = UserPoolGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type UserPoolGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut group_name: Option<::Value<String>> = None;
                let mut precedence: Option<::Value<f64>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut user_pool_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GroupName" => {
                            group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Precedence" => {
                            precedence = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserPoolId" => {
                            user_pool_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(UserPoolGroupProperties {
                    description: description,
                    group_name: group_name,
                    precedence: precedence,
                    role_arn: role_arn,
                    user_pool_id: user_pool_id.ok_or(::serde::de::Error::missing_field("UserPoolId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for UserPoolGroup {
    type Properties = UserPoolGroupProperties;
    const TYPE: &'static str = "AWS::Cognito::UserPoolGroup";
    fn properties(&self) -> &UserPoolGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut UserPoolGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for UserPoolGroup {}

impl From<UserPoolGroupProperties> for UserPoolGroup {
    fn from(properties: UserPoolGroupProperties) -> UserPoolGroup {
        UserPoolGroup { properties }
    }
}

/// The [`AWS::Cognito::UserPoolIdentityProvider`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolidentityprovider.html) resource type.
#[derive(Debug, Default)]
pub struct UserPoolIdentityProvider {
    properties: UserPoolIdentityProviderProperties
}

/// Properties for the `UserPoolIdentityProvider` resource.
#[derive(Debug, Default)]
pub struct UserPoolIdentityProviderProperties {
    /// Property [`AttributeMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolidentityprovider.html#cfn-cognito-userpoolidentityprovider-attributemapping).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub attribute_mapping: Option<::Value<::json::Value>>,
    /// Property [`IdpIdentifiers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolidentityprovider.html#cfn-cognito-userpoolidentityprovider-idpidentifiers).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub idp_identifiers: Option<::ValueList<String>>,
    /// Property [`ProviderDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolidentityprovider.html#cfn-cognito-userpoolidentityprovider-providerdetails).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub provider_details: Option<::Value<::json::Value>>,
    /// Property [`ProviderName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolidentityprovider.html#cfn-cognito-userpoolidentityprovider-providername).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub provider_name: ::Value<String>,
    /// Property [`ProviderType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolidentityprovider.html#cfn-cognito-userpoolidentityprovider-providertype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub provider_type: ::Value<String>,
    /// Property [`UserPoolId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolidentityprovider.html#cfn-cognito-userpoolidentityprovider-userpoolid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub user_pool_id: ::Value<String>,
}

impl ::serde::Serialize for UserPoolIdentityProviderProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref attribute_mapping) = self.attribute_mapping {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttributeMapping", attribute_mapping)?;
        }
        if let Some(ref idp_identifiers) = self.idp_identifiers {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdpIdentifiers", idp_identifiers)?;
        }
        if let Some(ref provider_details) = self.provider_details {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProviderDetails", provider_details)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProviderName", &self.provider_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProviderType", &self.provider_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserPoolId", &self.user_pool_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for UserPoolIdentityProviderProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<UserPoolIdentityProviderProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = UserPoolIdentityProviderProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type UserPoolIdentityProviderProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut attribute_mapping: Option<::Value<::json::Value>> = None;
                let mut idp_identifiers: Option<::ValueList<String>> = None;
                let mut provider_details: Option<::Value<::json::Value>> = None;
                let mut provider_name: Option<::Value<String>> = None;
                let mut provider_type: Option<::Value<String>> = None;
                let mut user_pool_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AttributeMapping" => {
                            attribute_mapping = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IdpIdentifiers" => {
                            idp_identifiers = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProviderDetails" => {
                            provider_details = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProviderName" => {
                            provider_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProviderType" => {
                            provider_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserPoolId" => {
                            user_pool_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(UserPoolIdentityProviderProperties {
                    attribute_mapping: attribute_mapping,
                    idp_identifiers: idp_identifiers,
                    provider_details: provider_details,
                    provider_name: provider_name.ok_or(::serde::de::Error::missing_field("ProviderName"))?,
                    provider_type: provider_type.ok_or(::serde::de::Error::missing_field("ProviderType"))?,
                    user_pool_id: user_pool_id.ok_or(::serde::de::Error::missing_field("UserPoolId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for UserPoolIdentityProvider {
    type Properties = UserPoolIdentityProviderProperties;
    const TYPE: &'static str = "AWS::Cognito::UserPoolIdentityProvider";
    fn properties(&self) -> &UserPoolIdentityProviderProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut UserPoolIdentityProviderProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for UserPoolIdentityProvider {}

impl From<UserPoolIdentityProviderProperties> for UserPoolIdentityProvider {
    fn from(properties: UserPoolIdentityProviderProperties) -> UserPoolIdentityProvider {
        UserPoolIdentityProvider { properties }
    }
}

/// The [`AWS::Cognito::UserPoolResourceServer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolresourceserver.html) resource type.
#[derive(Debug, Default)]
pub struct UserPoolResourceServer {
    properties: UserPoolResourceServerProperties
}

/// Properties for the `UserPoolResourceServer` resource.
#[derive(Debug, Default)]
pub struct UserPoolResourceServerProperties {
    /// Property [`Identifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolresourceserver.html#cfn-cognito-userpoolresourceserver-identifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub identifier: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolresourceserver.html#cfn-cognito-userpoolresourceserver-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Scopes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolresourceserver.html#cfn-cognito-userpoolresourceserver-scopes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub scopes: Option<::ValueList<self::user_pool_resource_server::ResourceServerScopeType>>,
    /// Property [`UserPoolId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolresourceserver.html#cfn-cognito-userpoolresourceserver-userpoolid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub user_pool_id: ::Value<String>,
}

impl ::serde::Serialize for UserPoolResourceServerProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Identifier", &self.identifier)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref scopes) = self.scopes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Scopes", scopes)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserPoolId", &self.user_pool_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for UserPoolResourceServerProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<UserPoolResourceServerProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = UserPoolResourceServerProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type UserPoolResourceServerProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut identifier: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut scopes: Option<::ValueList<self::user_pool_resource_server::ResourceServerScopeType>> = None;
                let mut user_pool_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Identifier" => {
                            identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Scopes" => {
                            scopes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserPoolId" => {
                            user_pool_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(UserPoolResourceServerProperties {
                    identifier: identifier.ok_or(::serde::de::Error::missing_field("Identifier"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    scopes: scopes,
                    user_pool_id: user_pool_id.ok_or(::serde::de::Error::missing_field("UserPoolId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for UserPoolResourceServer {
    type Properties = UserPoolResourceServerProperties;
    const TYPE: &'static str = "AWS::Cognito::UserPoolResourceServer";
    fn properties(&self) -> &UserPoolResourceServerProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut UserPoolResourceServerProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for UserPoolResourceServer {}

impl From<UserPoolResourceServerProperties> for UserPoolResourceServer {
    fn from(properties: UserPoolResourceServerProperties) -> UserPoolResourceServer {
        UserPoolResourceServer { properties }
    }
}

/// The [`AWS::Cognito::UserPoolRiskConfigurationAttachment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolriskconfigurationattachment.html) resource type.
#[derive(Debug, Default)]
pub struct UserPoolRiskConfigurationAttachment {
    properties: UserPoolRiskConfigurationAttachmentProperties
}

/// Properties for the `UserPoolRiskConfigurationAttachment` resource.
#[derive(Debug, Default)]
pub struct UserPoolRiskConfigurationAttachmentProperties {
    /// Property [`AccountTakeoverRiskConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolriskconfigurationattachment.html#cfn-cognito-userpoolriskconfigurationattachment-accounttakeoverriskconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub account_takeover_risk_configuration: Option<::Value<self::user_pool_risk_configuration_attachment::AccountTakeoverRiskConfigurationType>>,
    /// Property [`ClientId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolriskconfigurationattachment.html#cfn-cognito-userpoolriskconfigurationattachment-clientid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub client_id: ::Value<String>,
    /// Property [`CompromisedCredentialsRiskConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolriskconfigurationattachment.html#cfn-cognito-userpoolriskconfigurationattachment-compromisedcredentialsriskconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub compromised_credentials_risk_configuration: Option<::Value<self::user_pool_risk_configuration_attachment::CompromisedCredentialsRiskConfigurationType>>,
    /// Property [`RiskExceptionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolriskconfigurationattachment.html#cfn-cognito-userpoolriskconfigurationattachment-riskexceptionconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub risk_exception_configuration: Option<::Value<self::user_pool_risk_configuration_attachment::RiskExceptionConfigurationType>>,
    /// Property [`UserPoolId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolriskconfigurationattachment.html#cfn-cognito-userpoolriskconfigurationattachment-userpoolid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub user_pool_id: ::Value<String>,
}

impl ::serde::Serialize for UserPoolRiskConfigurationAttachmentProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref account_takeover_risk_configuration) = self.account_takeover_risk_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccountTakeoverRiskConfiguration", account_takeover_risk_configuration)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientId", &self.client_id)?;
        if let Some(ref compromised_credentials_risk_configuration) = self.compromised_credentials_risk_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CompromisedCredentialsRiskConfiguration", compromised_credentials_risk_configuration)?;
        }
        if let Some(ref risk_exception_configuration) = self.risk_exception_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RiskExceptionConfiguration", risk_exception_configuration)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserPoolId", &self.user_pool_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for UserPoolRiskConfigurationAttachmentProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<UserPoolRiskConfigurationAttachmentProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = UserPoolRiskConfigurationAttachmentProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type UserPoolRiskConfigurationAttachmentProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut account_takeover_risk_configuration: Option<::Value<self::user_pool_risk_configuration_attachment::AccountTakeoverRiskConfigurationType>> = None;
                let mut client_id: Option<::Value<String>> = None;
                let mut compromised_credentials_risk_configuration: Option<::Value<self::user_pool_risk_configuration_attachment::CompromisedCredentialsRiskConfigurationType>> = None;
                let mut risk_exception_configuration: Option<::Value<self::user_pool_risk_configuration_attachment::RiskExceptionConfigurationType>> = None;
                let mut user_pool_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AccountTakeoverRiskConfiguration" => {
                            account_takeover_risk_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClientId" => {
                            client_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CompromisedCredentialsRiskConfiguration" => {
                            compromised_credentials_risk_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RiskExceptionConfiguration" => {
                            risk_exception_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserPoolId" => {
                            user_pool_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(UserPoolRiskConfigurationAttachmentProperties {
                    account_takeover_risk_configuration: account_takeover_risk_configuration,
                    client_id: client_id.ok_or(::serde::de::Error::missing_field("ClientId"))?,
                    compromised_credentials_risk_configuration: compromised_credentials_risk_configuration,
                    risk_exception_configuration: risk_exception_configuration,
                    user_pool_id: user_pool_id.ok_or(::serde::de::Error::missing_field("UserPoolId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for UserPoolRiskConfigurationAttachment {
    type Properties = UserPoolRiskConfigurationAttachmentProperties;
    const TYPE: &'static str = "AWS::Cognito::UserPoolRiskConfigurationAttachment";
    fn properties(&self) -> &UserPoolRiskConfigurationAttachmentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut UserPoolRiskConfigurationAttachmentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for UserPoolRiskConfigurationAttachment {}

impl From<UserPoolRiskConfigurationAttachmentProperties> for UserPoolRiskConfigurationAttachment {
    fn from(properties: UserPoolRiskConfigurationAttachmentProperties) -> UserPoolRiskConfigurationAttachment {
        UserPoolRiskConfigurationAttachment { properties }
    }
}

/// The [`AWS::Cognito::UserPoolUICustomizationAttachment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpooluicustomizationattachment.html) resource type.
#[derive(Debug, Default)]
pub struct UserPoolUICustomizationAttachment {
    properties: UserPoolUICustomizationAttachmentProperties
}

/// Properties for the `UserPoolUICustomizationAttachment` resource.
#[derive(Debug, Default)]
pub struct UserPoolUICustomizationAttachmentProperties {
    /// Property [`CSS`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpooluicustomizationattachment.html#cfn-cognito-userpooluicustomizationattachment-css).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub css: Option<::Value<String>>,
    /// Property [`ClientId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpooluicustomizationattachment.html#cfn-cognito-userpooluicustomizationattachment-clientid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub client_id: ::Value<String>,
    /// Property [`UserPoolId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpooluicustomizationattachment.html#cfn-cognito-userpooluicustomizationattachment-userpoolid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub user_pool_id: ::Value<String>,
}

impl ::serde::Serialize for UserPoolUICustomizationAttachmentProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref css) = self.css {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CSS", css)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientId", &self.client_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserPoolId", &self.user_pool_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for UserPoolUICustomizationAttachmentProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<UserPoolUICustomizationAttachmentProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = UserPoolUICustomizationAttachmentProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type UserPoolUICustomizationAttachmentProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut css: Option<::Value<String>> = None;
                let mut client_id: Option<::Value<String>> = None;
                let mut user_pool_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CSS" => {
                            css = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClientId" => {
                            client_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserPoolId" => {
                            user_pool_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(UserPoolUICustomizationAttachmentProperties {
                    css: css,
                    client_id: client_id.ok_or(::serde::de::Error::missing_field("ClientId"))?,
                    user_pool_id: user_pool_id.ok_or(::serde::de::Error::missing_field("UserPoolId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for UserPoolUICustomizationAttachment {
    type Properties = UserPoolUICustomizationAttachmentProperties;
    const TYPE: &'static str = "AWS::Cognito::UserPoolUICustomizationAttachment";
    fn properties(&self) -> &UserPoolUICustomizationAttachmentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut UserPoolUICustomizationAttachmentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for UserPoolUICustomizationAttachment {}

impl From<UserPoolUICustomizationAttachmentProperties> for UserPoolUICustomizationAttachment {
    fn from(properties: UserPoolUICustomizationAttachmentProperties) -> UserPoolUICustomizationAttachment {
        UserPoolUICustomizationAttachment { properties }
    }
}

/// The [`AWS::Cognito::UserPoolUser`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpooluser.html) resource type.
#[derive(Debug, Default)]
pub struct UserPoolUser {
    properties: UserPoolUserProperties
}

/// Properties for the `UserPoolUser` resource.
#[derive(Debug, Default)]
pub struct UserPoolUserProperties {
    /// Property [`ClientMetadata`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpooluser.html#cfn-cognito-userpooluser-clientmetadata).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub client_metadata: Option<::Value<::json::Value>>,
    /// Property [`DesiredDeliveryMediums`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpooluser.html#cfn-cognito-userpooluser-desireddeliverymediums).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub desired_delivery_mediums: Option<::ValueList<String>>,
    /// Property [`ForceAliasCreation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpooluser.html#cfn-cognito-userpooluser-forcealiascreation).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub force_alias_creation: Option<::Value<bool>>,
    /// Property [`MessageAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpooluser.html#cfn-cognito-userpooluser-messageaction).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub message_action: Option<::Value<String>>,
    /// Property [`UserAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpooluser.html#cfn-cognito-userpooluser-userattributes).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub user_attributes: Option<::ValueList<self::user_pool_user::AttributeType>>,
    /// Property [`UserPoolId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpooluser.html#cfn-cognito-userpooluser-userpoolid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub user_pool_id: ::Value<String>,
    /// Property [`Username`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpooluser.html#cfn-cognito-userpooluser-username).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub username: Option<::Value<String>>,
    /// Property [`ValidationData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpooluser.html#cfn-cognito-userpooluser-validationdata).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub validation_data: Option<::ValueList<self::user_pool_user::AttributeType>>,
}

impl ::serde::Serialize for UserPoolUserProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref client_metadata) = self.client_metadata {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientMetadata", client_metadata)?;
        }
        if let Some(ref desired_delivery_mediums) = self.desired_delivery_mediums {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DesiredDeliveryMediums", desired_delivery_mediums)?;
        }
        if let Some(ref force_alias_creation) = self.force_alias_creation {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ForceAliasCreation", force_alias_creation)?;
        }
        if let Some(ref message_action) = self.message_action {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MessageAction", message_action)?;
        }
        if let Some(ref user_attributes) = self.user_attributes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserAttributes", user_attributes)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserPoolId", &self.user_pool_id)?;
        if let Some(ref username) = self.username {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Username", username)?;
        }
        if let Some(ref validation_data) = self.validation_data {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ValidationData", validation_data)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for UserPoolUserProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<UserPoolUserProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = UserPoolUserProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type UserPoolUserProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut client_metadata: Option<::Value<::json::Value>> = None;
                let mut desired_delivery_mediums: Option<::ValueList<String>> = None;
                let mut force_alias_creation: Option<::Value<bool>> = None;
                let mut message_action: Option<::Value<String>> = None;
                let mut user_attributes: Option<::ValueList<self::user_pool_user::AttributeType>> = None;
                let mut user_pool_id: Option<::Value<String>> = None;
                let mut username: Option<::Value<String>> = None;
                let mut validation_data: Option<::ValueList<self::user_pool_user::AttributeType>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ClientMetadata" => {
                            client_metadata = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DesiredDeliveryMediums" => {
                            desired_delivery_mediums = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ForceAliasCreation" => {
                            force_alias_creation = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MessageAction" => {
                            message_action = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserAttributes" => {
                            user_attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserPoolId" => {
                            user_pool_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Username" => {
                            username = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ValidationData" => {
                            validation_data = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(UserPoolUserProperties {
                    client_metadata: client_metadata,
                    desired_delivery_mediums: desired_delivery_mediums,
                    force_alias_creation: force_alias_creation,
                    message_action: message_action,
                    user_attributes: user_attributes,
                    user_pool_id: user_pool_id.ok_or(::serde::de::Error::missing_field("UserPoolId"))?,
                    username: username,
                    validation_data: validation_data,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for UserPoolUser {
    type Properties = UserPoolUserProperties;
    const TYPE: &'static str = "AWS::Cognito::UserPoolUser";
    fn properties(&self) -> &UserPoolUserProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut UserPoolUserProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for UserPoolUser {}

impl From<UserPoolUserProperties> for UserPoolUser {
    fn from(properties: UserPoolUserProperties) -> UserPoolUser {
        UserPoolUser { properties }
    }
}

/// The [`AWS::Cognito::UserPoolUserToGroupAttachment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolusertogroupattachment.html) resource type.
#[derive(Debug, Default)]
pub struct UserPoolUserToGroupAttachment {
    properties: UserPoolUserToGroupAttachmentProperties
}

/// Properties for the `UserPoolUserToGroupAttachment` resource.
#[derive(Debug, Default)]
pub struct UserPoolUserToGroupAttachmentProperties {
    /// Property [`GroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolusertogroupattachment.html#cfn-cognito-userpoolusertogroupattachment-groupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub group_name: ::Value<String>,
    /// Property [`UserPoolId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolusertogroupattachment.html#cfn-cognito-userpoolusertogroupattachment-userpoolid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub user_pool_id: ::Value<String>,
    /// Property [`Username`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolusertogroupattachment.html#cfn-cognito-userpoolusertogroupattachment-username).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub username: ::Value<String>,
}

impl ::serde::Serialize for UserPoolUserToGroupAttachmentProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupName", &self.group_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserPoolId", &self.user_pool_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Username", &self.username)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for UserPoolUserToGroupAttachmentProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<UserPoolUserToGroupAttachmentProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = UserPoolUserToGroupAttachmentProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type UserPoolUserToGroupAttachmentProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut group_name: Option<::Value<String>> = None;
                let mut user_pool_id: Option<::Value<String>> = None;
                let mut username: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "GroupName" => {
                            group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserPoolId" => {
                            user_pool_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Username" => {
                            username = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(UserPoolUserToGroupAttachmentProperties {
                    group_name: group_name.ok_or(::serde::de::Error::missing_field("GroupName"))?,
                    user_pool_id: user_pool_id.ok_or(::serde::de::Error::missing_field("UserPoolId"))?,
                    username: username.ok_or(::serde::de::Error::missing_field("Username"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for UserPoolUserToGroupAttachment {
    type Properties = UserPoolUserToGroupAttachmentProperties;
    const TYPE: &'static str = "AWS::Cognito::UserPoolUserToGroupAttachment";
    fn properties(&self) -> &UserPoolUserToGroupAttachmentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut UserPoolUserToGroupAttachmentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for UserPoolUserToGroupAttachment {}

impl From<UserPoolUserToGroupAttachmentProperties> for UserPoolUserToGroupAttachment {
    fn from(properties: UserPoolUserToGroupAttachmentProperties) -> UserPoolUserToGroupAttachment {
        UserPoolUserToGroupAttachment { properties }
    }
}

pub mod identity_pool {
    //! Property types for the `IdentityPool` resource.

    /// The [`AWS::Cognito::IdentityPool.CognitoIdentityProvider`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypool-cognitoidentityprovider.html) property type.
    #[derive(Debug, Default)]
    pub struct CognitoIdentityProvider {
        /// Property [`ClientId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypool-cognitoidentityprovider.html#cfn-cognito-identitypool-cognitoidentityprovider-clientid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_id: Option<::Value<String>>,
        /// Property [`ProviderName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypool-cognitoidentityprovider.html#cfn-cognito-identitypool-cognitoidentityprovider-providername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub provider_name: Option<::Value<String>>,
        /// Property [`ServerSideTokenCheck`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypool-cognitoidentityprovider.html#cfn-cognito-identitypool-cognitoidentityprovider-serversidetokencheck).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub server_side_token_check: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for CognitoIdentityProvider {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref client_id) = self.client_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientId", client_id)?;
            }
            if let Some(ref provider_name) = self.provider_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProviderName", provider_name)?;
            }
            if let Some(ref server_side_token_check) = self.server_side_token_check {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerSideTokenCheck", server_side_token_check)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CognitoIdentityProvider {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CognitoIdentityProvider, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CognitoIdentityProvider;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CognitoIdentityProvider")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut client_id: Option<::Value<String>> = None;
                    let mut provider_name: Option<::Value<String>> = None;
                    let mut server_side_token_check: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ClientId" => {
                                client_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProviderName" => {
                                provider_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServerSideTokenCheck" => {
                                server_side_token_check = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CognitoIdentityProvider {
                        client_id: client_id,
                        provider_name: provider_name,
                        server_side_token_check: server_side_token_check,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Cognito::IdentityPool.CognitoStreams`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypool-cognitostreams.html) property type.
    #[derive(Debug, Default)]
    pub struct CognitoStreams {
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypool-cognitostreams.html#cfn-cognito-identitypool-cognitostreams-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: Option<::Value<String>>,
        /// Property [`StreamName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypool-cognitostreams.html#cfn-cognito-identitypool-cognitostreams-streamname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stream_name: Option<::Value<String>>,
        /// Property [`StreamingStatus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypool-cognitostreams.html#cfn-cognito-identitypool-cognitostreams-streamingstatus).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub streaming_status: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CognitoStreams {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref role_arn) = self.role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", role_arn)?;
            }
            if let Some(ref stream_name) = self.stream_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamName", stream_name)?;
            }
            if let Some(ref streaming_status) = self.streaming_status {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamingStatus", streaming_status)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CognitoStreams {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CognitoStreams, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CognitoStreams;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CognitoStreams")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut stream_name: Option<::Value<String>> = None;
                    let mut streaming_status: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StreamName" => {
                                stream_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StreamingStatus" => {
                                streaming_status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CognitoStreams {
                        role_arn: role_arn,
                        stream_name: stream_name,
                        streaming_status: streaming_status,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Cognito::IdentityPool.PushSync`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypool-pushsync.html) property type.
    #[derive(Debug, Default)]
    pub struct PushSync {
        /// Property [`ApplicationArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypool-pushsync.html#cfn-cognito-identitypool-pushsync-applicationarns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub application_arns: Option<::ValueList<String>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypool-pushsync.html#cfn-cognito-identitypool-pushsync-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for PushSync {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref application_arns) = self.application_arns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationArns", application_arns)?;
            }
            if let Some(ref role_arn) = self.role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", role_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PushSync {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PushSync, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PushSync;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PushSync")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut application_arns: Option<::ValueList<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ApplicationArns" => {
                                application_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PushSync {
                        application_arns: application_arns,
                        role_arn: role_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod identity_pool_role_attachment {
    //! Property types for the `IdentityPoolRoleAttachment` resource.

    /// The [`AWS::Cognito::IdentityPoolRoleAttachment.MappingRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypoolroleattachment-mappingrule.html) property type.
    #[derive(Debug, Default)]
    pub struct MappingRule {
        /// Property [`Claim`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypoolroleattachment-mappingrule.html#cfn-cognito-identitypoolroleattachment-mappingrule-claim).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub claim: ::Value<String>,
        /// Property [`MatchType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypoolroleattachment-mappingrule.html#cfn-cognito-identitypoolroleattachment-mappingrule-matchtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub match_type: ::Value<String>,
        /// Property [`RoleARN`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypoolroleattachment-mappingrule.html#cfn-cognito-identitypoolroleattachment-mappingrule-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypoolroleattachment-mappingrule.html#cfn-cognito-identitypoolroleattachment-mappingrule-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for MappingRule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Claim", &self.claim)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MatchType", &self.match_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleARN", &self.role_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MappingRule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MappingRule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MappingRule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MappingRule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut claim: Option<::Value<String>> = None;
                    let mut match_type: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Claim" => {
                                claim = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MatchType" => {
                                match_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleARN" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MappingRule {
                        claim: claim.ok_or(::serde::de::Error::missing_field("Claim"))?,
                        match_type: match_type.ok_or(::serde::de::Error::missing_field("MatchType"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleARN"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Cognito::IdentityPoolRoleAttachment.RoleMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypoolroleattachment-rolemapping.html) property type.
    #[derive(Debug, Default)]
    pub struct RoleMapping {
        /// Property [`AmbiguousRoleResolution`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypoolroleattachment-rolemapping.html#cfn-cognito-identitypoolroleattachment-rolemapping-ambiguousroleresolution).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ambiguous_role_resolution: Option<::Value<String>>,
        /// Property [`IdentityProvider`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypoolroleattachment-rolemapping.html#cfn-cognito-identitypoolroleattachment-rolemapping-identityprovider).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub identity_provider: Option<::Value<String>>,
        /// Property [`RulesConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypoolroleattachment-rolemapping.html#cfn-cognito-identitypoolroleattachment-rolemapping-rulesconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rules_configuration: Option<::Value<RulesConfigurationType>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypoolroleattachment-rolemapping.html#cfn-cognito-identitypoolroleattachment-rolemapping-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for RoleMapping {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref ambiguous_role_resolution) = self.ambiguous_role_resolution {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AmbiguousRoleResolution", ambiguous_role_resolution)?;
            }
            if let Some(ref identity_provider) = self.identity_provider {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdentityProvider", identity_provider)?;
            }
            if let Some(ref rules_configuration) = self.rules_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RulesConfiguration", rules_configuration)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RoleMapping {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RoleMapping, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RoleMapping;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RoleMapping")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut ambiguous_role_resolution: Option<::Value<String>> = None;
                    let mut identity_provider: Option<::Value<String>> = None;
                    let mut rules_configuration: Option<::Value<RulesConfigurationType>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AmbiguousRoleResolution" => {
                                ambiguous_role_resolution = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IdentityProvider" => {
                                identity_provider = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RulesConfiguration" => {
                                rules_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RoleMapping {
                        ambiguous_role_resolution: ambiguous_role_resolution,
                        identity_provider: identity_provider,
                        rules_configuration: rules_configuration,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Cognito::IdentityPoolRoleAttachment.RulesConfigurationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypoolroleattachment-rulesconfigurationtype.html) property type.
    #[derive(Debug, Default)]
    pub struct RulesConfigurationType {
        /// Property [`Rules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypoolroleattachment-rulesconfigurationtype.html#cfn-cognito-identitypoolroleattachment-rulesconfigurationtype-rules).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rules: ::ValueList<MappingRule>,
    }

    impl ::codec::SerializeValue for RulesConfigurationType {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Rules", &self.rules)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RulesConfigurationType {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RulesConfigurationType, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RulesConfigurationType;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RulesConfigurationType")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut rules: Option<::ValueList<MappingRule>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Rules" => {
                                rules = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RulesConfigurationType {
                        rules: rules.ok_or(::serde::de::Error::missing_field("Rules"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod user_pool {
    //! Property types for the `UserPool` resource.

    /// The [`AWS::Cognito::UserPool.AccountRecoverySetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-accountrecoverysetting.html) property type.
    #[derive(Debug, Default)]
    pub struct AccountRecoverySetting {
        /// Property [`RecoveryMechanisms`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-accountrecoverysetting.html#cfn-cognito-userpool-accountrecoverysetting-recoverymechanisms).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub recovery_mechanisms: Option<::ValueList<RecoveryOption>>,
    }

    impl ::codec::SerializeValue for AccountRecoverySetting {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref recovery_mechanisms) = self.recovery_mechanisms {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecoveryMechanisms", recovery_mechanisms)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AccountRecoverySetting {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AccountRecoverySetting, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AccountRecoverySetting;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AccountRecoverySetting")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut recovery_mechanisms: Option<::ValueList<RecoveryOption>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RecoveryMechanisms" => {
                                recovery_mechanisms = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AccountRecoverySetting {
                        recovery_mechanisms: recovery_mechanisms,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Cognito::UserPool.AdminCreateUserConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-admincreateuserconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct AdminCreateUserConfig {
        /// Property [`AllowAdminCreateUserOnly`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-admincreateuserconfig.html#cfn-cognito-userpool-admincreateuserconfig-allowadmincreateuseronly).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allow_admin_create_user_only: Option<::Value<bool>>,
        /// Property [`InviteMessageTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-admincreateuserconfig.html#cfn-cognito-userpool-admincreateuserconfig-invitemessagetemplate).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub invite_message_template: Option<::Value<InviteMessageTemplate>>,
        /// Property [`UnusedAccountValidityDays`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-admincreateuserconfig.html#cfn-cognito-userpool-admincreateuserconfig-unusedaccountvaliditydays).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unused_account_validity_days: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for AdminCreateUserConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref allow_admin_create_user_only) = self.allow_admin_create_user_only {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowAdminCreateUserOnly", allow_admin_create_user_only)?;
            }
            if let Some(ref invite_message_template) = self.invite_message_template {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InviteMessageTemplate", invite_message_template)?;
            }
            if let Some(ref unused_account_validity_days) = self.unused_account_validity_days {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UnusedAccountValidityDays", unused_account_validity_days)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AdminCreateUserConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AdminCreateUserConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AdminCreateUserConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AdminCreateUserConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allow_admin_create_user_only: Option<::Value<bool>> = None;
                    let mut invite_message_template: Option<::Value<InviteMessageTemplate>> = None;
                    let mut unused_account_validity_days: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllowAdminCreateUserOnly" => {
                                allow_admin_create_user_only = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InviteMessageTemplate" => {
                                invite_message_template = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UnusedAccountValidityDays" => {
                                unused_account_validity_days = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AdminCreateUserConfig {
                        allow_admin_create_user_only: allow_admin_create_user_only,
                        invite_message_template: invite_message_template,
                        unused_account_validity_days: unused_account_validity_days,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Cognito::UserPool.CustomEmailSender`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-customemailsender.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomEmailSender {
        /// Property [`LambdaArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-customemailsender.html#cfn-cognito-userpool-customemailsender-lambdaarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lambda_arn: Option<::Value<String>>,
        /// Property [`LambdaVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-customemailsender.html#cfn-cognito-userpool-customemailsender-lambdaversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lambda_version: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CustomEmailSender {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref lambda_arn) = self.lambda_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaArn", lambda_arn)?;
            }
            if let Some(ref lambda_version) = self.lambda_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaVersion", lambda_version)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomEmailSender {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomEmailSender, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomEmailSender;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomEmailSender")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut lambda_arn: Option<::Value<String>> = None;
                    let mut lambda_version: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LambdaArn" => {
                                lambda_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LambdaVersion" => {
                                lambda_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomEmailSender {
                        lambda_arn: lambda_arn,
                        lambda_version: lambda_version,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Cognito::UserPool.CustomSMSSender`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-customsmssender.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomSMSSender {
        /// Property [`LambdaArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-customsmssender.html#cfn-cognito-userpool-customsmssender-lambdaarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lambda_arn: Option<::Value<String>>,
        /// Property [`LambdaVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-customsmssender.html#cfn-cognito-userpool-customsmssender-lambdaversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lambda_version: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CustomSMSSender {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref lambda_arn) = self.lambda_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaArn", lambda_arn)?;
            }
            if let Some(ref lambda_version) = self.lambda_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaVersion", lambda_version)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomSMSSender {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomSMSSender, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomSMSSender;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomSMSSender")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut lambda_arn: Option<::Value<String>> = None;
                    let mut lambda_version: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LambdaArn" => {
                                lambda_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LambdaVersion" => {
                                lambda_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomSMSSender {
                        lambda_arn: lambda_arn,
                        lambda_version: lambda_version,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Cognito::UserPool.DeviceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-deviceconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct DeviceConfiguration {
        /// Property [`ChallengeRequiredOnNewDevice`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-deviceconfiguration.html#cfn-cognito-userpool-deviceconfiguration-challengerequiredonnewdevice).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub challenge_required_on_new_device: Option<::Value<bool>>,
        /// Property [`DeviceOnlyRememberedOnUserPrompt`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-deviceconfiguration.html#cfn-cognito-userpool-deviceconfiguration-deviceonlyrememberedonuserprompt).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub device_only_remembered_on_user_prompt: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for DeviceConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref challenge_required_on_new_device) = self.challenge_required_on_new_device {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChallengeRequiredOnNewDevice", challenge_required_on_new_device)?;
            }
            if let Some(ref device_only_remembered_on_user_prompt) = self.device_only_remembered_on_user_prompt {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceOnlyRememberedOnUserPrompt", device_only_remembered_on_user_prompt)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DeviceConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DeviceConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DeviceConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DeviceConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut challenge_required_on_new_device: Option<::Value<bool>> = None;
                    let mut device_only_remembered_on_user_prompt: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ChallengeRequiredOnNewDevice" => {
                                challenge_required_on_new_device = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeviceOnlyRememberedOnUserPrompt" => {
                                device_only_remembered_on_user_prompt = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DeviceConfiguration {
                        challenge_required_on_new_device: challenge_required_on_new_device,
                        device_only_remembered_on_user_prompt: device_only_remembered_on_user_prompt,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Cognito::UserPool.EmailConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-emailconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct EmailConfiguration {
        /// Property [`ConfigurationSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-emailconfiguration.html#cfn-cognito-userpool-emailconfiguration-configurationset).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub configuration_set: Option<::Value<String>>,
        /// Property [`EmailSendingAccount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-emailconfiguration.html#cfn-cognito-userpool-emailconfiguration-emailsendingaccount).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub email_sending_account: Option<::Value<String>>,
        /// Property [`From`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-emailconfiguration.html#cfn-cognito-userpool-emailconfiguration-from).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub from: Option<::Value<String>>,
        /// Property [`ReplyToEmailAddress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-emailconfiguration.html#cfn-cognito-userpool-emailconfiguration-replytoemailaddress).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub reply_to_email_address: Option<::Value<String>>,
        /// Property [`SourceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-emailconfiguration.html#cfn-cognito-userpool-emailconfiguration-sourcearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EmailConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref configuration_set) = self.configuration_set {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfigurationSet", configuration_set)?;
            }
            if let Some(ref email_sending_account) = self.email_sending_account {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EmailSendingAccount", email_sending_account)?;
            }
            if let Some(ref from) = self.from {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "From", from)?;
            }
            if let Some(ref reply_to_email_address) = self.reply_to_email_address {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplyToEmailAddress", reply_to_email_address)?;
            }
            if let Some(ref source_arn) = self.source_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceArn", source_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EmailConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EmailConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EmailConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EmailConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut configuration_set: Option<::Value<String>> = None;
                    let mut email_sending_account: Option<::Value<String>> = None;
                    let mut from: Option<::Value<String>> = None;
                    let mut reply_to_email_address: Option<::Value<String>> = None;
                    let mut source_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConfigurationSet" => {
                                configuration_set = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EmailSendingAccount" => {
                                email_sending_account = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "From" => {
                                from = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReplyToEmailAddress" => {
                                reply_to_email_address = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceArn" => {
                                source_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EmailConfiguration {
                        configuration_set: configuration_set,
                        email_sending_account: email_sending_account,
                        from: from,
                        reply_to_email_address: reply_to_email_address,
                        source_arn: source_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Cognito::UserPool.InviteMessageTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-invitemessagetemplate.html) property type.
    #[derive(Debug, Default)]
    pub struct InviteMessageTemplate {
        /// Property [`EmailMessage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-invitemessagetemplate.html#cfn-cognito-userpool-invitemessagetemplate-emailmessage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub email_message: Option<::Value<String>>,
        /// Property [`EmailSubject`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-invitemessagetemplate.html#cfn-cognito-userpool-invitemessagetemplate-emailsubject).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub email_subject: Option<::Value<String>>,
        /// Property [`SMSMessage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-invitemessagetemplate.html#cfn-cognito-userpool-invitemessagetemplate-smsmessage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sms_message: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for InviteMessageTemplate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref email_message) = self.email_message {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EmailMessage", email_message)?;
            }
            if let Some(ref email_subject) = self.email_subject {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EmailSubject", email_subject)?;
            }
            if let Some(ref sms_message) = self.sms_message {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SMSMessage", sms_message)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InviteMessageTemplate {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InviteMessageTemplate, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InviteMessageTemplate;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InviteMessageTemplate")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut email_message: Option<::Value<String>> = None;
                    let mut email_subject: Option<::Value<String>> = None;
                    let mut sms_message: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EmailMessage" => {
                                email_message = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EmailSubject" => {
                                email_subject = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SMSMessage" => {
                                sms_message = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InviteMessageTemplate {
                        email_message: email_message,
                        email_subject: email_subject,
                        sms_message: sms_message,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Cognito::UserPool.LambdaConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-lambdaconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct LambdaConfig {
        /// Property [`CreateAuthChallenge`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-lambdaconfig.html#cfn-cognito-userpool-lambdaconfig-createauthchallenge).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub create_auth_challenge: Option<::Value<String>>,
        /// Property [`CustomEmailSender`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-lambdaconfig.html#cfn-cognito-userpool-lambdaconfig-customemailsender).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_email_sender: Option<::Value<CustomEmailSender>>,
        /// Property [`CustomMessage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-lambdaconfig.html#cfn-cognito-userpool-lambdaconfig-custommessage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_message: Option<::Value<String>>,
        /// Property [`CustomSMSSender`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-lambdaconfig.html#cfn-cognito-userpool-lambdaconfig-customsmssender).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_sms_sender: Option<::Value<CustomSMSSender>>,
        /// Property [`DefineAuthChallenge`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-lambdaconfig.html#cfn-cognito-userpool-lambdaconfig-defineauthchallenge).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub define_auth_challenge: Option<::Value<String>>,
        /// Property [`KMSKeyID`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-lambdaconfig.html#cfn-cognito-userpool-lambdaconfig-kmskeyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kms_key_id: Option<::Value<String>>,
        /// Property [`PostAuthentication`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-lambdaconfig.html#cfn-cognito-userpool-lambdaconfig-postauthentication).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub post_authentication: Option<::Value<String>>,
        /// Property [`PostConfirmation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-lambdaconfig.html#cfn-cognito-userpool-lambdaconfig-postconfirmation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub post_confirmation: Option<::Value<String>>,
        /// Property [`PreAuthentication`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-lambdaconfig.html#cfn-cognito-userpool-lambdaconfig-preauthentication).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pre_authentication: Option<::Value<String>>,
        /// Property [`PreSignUp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-lambdaconfig.html#cfn-cognito-userpool-lambdaconfig-presignup).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pre_sign_up: Option<::Value<String>>,
        /// Property [`PreTokenGeneration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-lambdaconfig.html#cfn-cognito-userpool-lambdaconfig-pretokengeneration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub pre_token_generation: Option<::Value<String>>,
        /// Property [`UserMigration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-lambdaconfig.html#cfn-cognito-userpool-lambdaconfig-usermigration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_migration: Option<::Value<String>>,
        /// Property [`VerifyAuthChallengeResponse`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-lambdaconfig.html#cfn-cognito-userpool-lambdaconfig-verifyauthchallengeresponse).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub verify_auth_challenge_response: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LambdaConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref create_auth_challenge) = self.create_auth_challenge {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CreateAuthChallenge", create_auth_challenge)?;
            }
            if let Some(ref custom_email_sender) = self.custom_email_sender {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomEmailSender", custom_email_sender)?;
            }
            if let Some(ref custom_message) = self.custom_message {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomMessage", custom_message)?;
            }
            if let Some(ref custom_sms_sender) = self.custom_sms_sender {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomSMSSender", custom_sms_sender)?;
            }
            if let Some(ref define_auth_challenge) = self.define_auth_challenge {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefineAuthChallenge", define_auth_challenge)?;
            }
            if let Some(ref kms_key_id) = self.kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KMSKeyID", kms_key_id)?;
            }
            if let Some(ref post_authentication) = self.post_authentication {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PostAuthentication", post_authentication)?;
            }
            if let Some(ref post_confirmation) = self.post_confirmation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PostConfirmation", post_confirmation)?;
            }
            if let Some(ref pre_authentication) = self.pre_authentication {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreAuthentication", pre_authentication)?;
            }
            if let Some(ref pre_sign_up) = self.pre_sign_up {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreSignUp", pre_sign_up)?;
            }
            if let Some(ref pre_token_generation) = self.pre_token_generation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreTokenGeneration", pre_token_generation)?;
            }
            if let Some(ref user_migration) = self.user_migration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserMigration", user_migration)?;
            }
            if let Some(ref verify_auth_challenge_response) = self.verify_auth_challenge_response {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VerifyAuthChallengeResponse", verify_auth_challenge_response)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LambdaConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LambdaConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LambdaConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LambdaConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut create_auth_challenge: Option<::Value<String>> = None;
                    let mut custom_email_sender: Option<::Value<CustomEmailSender>> = None;
                    let mut custom_message: Option<::Value<String>> = None;
                    let mut custom_sms_sender: Option<::Value<CustomSMSSender>> = None;
                    let mut define_auth_challenge: Option<::Value<String>> = None;
                    let mut kms_key_id: Option<::Value<String>> = None;
                    let mut post_authentication: Option<::Value<String>> = None;
                    let mut post_confirmation: Option<::Value<String>> = None;
                    let mut pre_authentication: Option<::Value<String>> = None;
                    let mut pre_sign_up: Option<::Value<String>> = None;
                    let mut pre_token_generation: Option<::Value<String>> = None;
                    let mut user_migration: Option<::Value<String>> = None;
                    let mut verify_auth_challenge_response: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CreateAuthChallenge" => {
                                create_auth_challenge = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CustomEmailSender" => {
                                custom_email_sender = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CustomMessage" => {
                                custom_message = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CustomSMSSender" => {
                                custom_sms_sender = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DefineAuthChallenge" => {
                                define_auth_challenge = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KMSKeyID" => {
                                kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PostAuthentication" => {
                                post_authentication = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PostConfirmation" => {
                                post_confirmation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PreAuthentication" => {
                                pre_authentication = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PreSignUp" => {
                                pre_sign_up = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PreTokenGeneration" => {
                                pre_token_generation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserMigration" => {
                                user_migration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VerifyAuthChallengeResponse" => {
                                verify_auth_challenge_response = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LambdaConfig {
                        create_auth_challenge: create_auth_challenge,
                        custom_email_sender: custom_email_sender,
                        custom_message: custom_message,
                        custom_sms_sender: custom_sms_sender,
                        define_auth_challenge: define_auth_challenge,
                        kms_key_id: kms_key_id,
                        post_authentication: post_authentication,
                        post_confirmation: post_confirmation,
                        pre_authentication: pre_authentication,
                        pre_sign_up: pre_sign_up,
                        pre_token_generation: pre_token_generation,
                        user_migration: user_migration,
                        verify_auth_challenge_response: verify_auth_challenge_response,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Cognito::UserPool.NumberAttributeConstraints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-numberattributeconstraints.html) property type.
    #[derive(Debug, Default)]
    pub struct NumberAttributeConstraints {
        /// Property [`MaxValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-numberattributeconstraints.html#cfn-cognito-userpool-numberattributeconstraints-maxvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_value: Option<::Value<String>>,
        /// Property [`MinValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-numberattributeconstraints.html#cfn-cognito-userpool-numberattributeconstraints-minvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min_value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for NumberAttributeConstraints {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref max_value) = self.max_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxValue", max_value)?;
            }
            if let Some(ref min_value) = self.min_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinValue", min_value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NumberAttributeConstraints {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NumberAttributeConstraints, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NumberAttributeConstraints;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NumberAttributeConstraints")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut max_value: Option<::Value<String>> = None;
                    let mut min_value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxValue" => {
                                max_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinValue" => {
                                min_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NumberAttributeConstraints {
                        max_value: max_value,
                        min_value: min_value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Cognito::UserPool.PasswordPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-passwordpolicy.html) property type.
    #[derive(Debug, Default)]
    pub struct PasswordPolicy {
        /// Property [`MinimumLength`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-passwordpolicy.html#cfn-cognito-userpool-passwordpolicy-minimumlength).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub minimum_length: Option<::Value<u32>>,
        /// Property [`RequireLowercase`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-passwordpolicy.html#cfn-cognito-userpool-passwordpolicy-requirelowercase).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub require_lowercase: Option<::Value<bool>>,
        /// Property [`RequireNumbers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-passwordpolicy.html#cfn-cognito-userpool-passwordpolicy-requirenumbers).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub require_numbers: Option<::Value<bool>>,
        /// Property [`RequireSymbols`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-passwordpolicy.html#cfn-cognito-userpool-passwordpolicy-requiresymbols).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub require_symbols: Option<::Value<bool>>,
        /// Property [`RequireUppercase`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-passwordpolicy.html#cfn-cognito-userpool-passwordpolicy-requireuppercase).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub require_uppercase: Option<::Value<bool>>,
        /// Property [`TemporaryPasswordValidityDays`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-passwordpolicy.html#cfn-cognito-userpool-passwordpolicy-temporarypasswordvaliditydays).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub temporary_password_validity_days: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for PasswordPolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref minimum_length) = self.minimum_length {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinimumLength", minimum_length)?;
            }
            if let Some(ref require_lowercase) = self.require_lowercase {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequireLowercase", require_lowercase)?;
            }
            if let Some(ref require_numbers) = self.require_numbers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequireNumbers", require_numbers)?;
            }
            if let Some(ref require_symbols) = self.require_symbols {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequireSymbols", require_symbols)?;
            }
            if let Some(ref require_uppercase) = self.require_uppercase {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequireUppercase", require_uppercase)?;
            }
            if let Some(ref temporary_password_validity_days) = self.temporary_password_validity_days {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemporaryPasswordValidityDays", temporary_password_validity_days)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PasswordPolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PasswordPolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PasswordPolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PasswordPolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut minimum_length: Option<::Value<u32>> = None;
                    let mut require_lowercase: Option<::Value<bool>> = None;
                    let mut require_numbers: Option<::Value<bool>> = None;
                    let mut require_symbols: Option<::Value<bool>> = None;
                    let mut require_uppercase: Option<::Value<bool>> = None;
                    let mut temporary_password_validity_days: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MinimumLength" => {
                                minimum_length = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RequireLowercase" => {
                                require_lowercase = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RequireNumbers" => {
                                require_numbers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RequireSymbols" => {
                                require_symbols = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RequireUppercase" => {
                                require_uppercase = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TemporaryPasswordValidityDays" => {
                                temporary_password_validity_days = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PasswordPolicy {
                        minimum_length: minimum_length,
                        require_lowercase: require_lowercase,
                        require_numbers: require_numbers,
                        require_symbols: require_symbols,
                        require_uppercase: require_uppercase,
                        temporary_password_validity_days: temporary_password_validity_days,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Cognito::UserPool.Policies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-policies.html) property type.
    #[derive(Debug, Default)]
    pub struct Policies {
        /// Property [`PasswordPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-policies.html#cfn-cognito-userpool-policies-passwordpolicy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub password_policy: Option<::Value<PasswordPolicy>>,
    }

    impl ::codec::SerializeValue for Policies {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref password_policy) = self.password_policy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PasswordPolicy", password_policy)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Policies {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Policies, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Policies;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Policies")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut password_policy: Option<::Value<PasswordPolicy>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PasswordPolicy" => {
                                password_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Policies {
                        password_policy: password_policy,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Cognito::UserPool.RecoveryOption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-recoveryoption.html) property type.
    #[derive(Debug, Default)]
    pub struct RecoveryOption {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-recoveryoption.html#cfn-cognito-userpool-recoveryoption-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Priority`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-recoveryoption.html#cfn-cognito-userpool-recoveryoption-priority).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub priority: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for RecoveryOption {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref priority) = self.priority {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Priority", priority)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RecoveryOption {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RecoveryOption, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RecoveryOption;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RecoveryOption")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut priority: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Priority" => {
                                priority = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RecoveryOption {
                        name: name,
                        priority: priority,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Cognito::UserPool.SchemaAttribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-schemaattribute.html) property type.
    #[derive(Debug, Default)]
    pub struct SchemaAttribute {
        /// Property [`AttributeDataType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-schemaattribute.html#cfn-cognito-userpool-schemaattribute-attributedatatype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attribute_data_type: Option<::Value<String>>,
        /// Property [`DeveloperOnlyAttribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-schemaattribute.html#cfn-cognito-userpool-schemaattribute-developeronlyattribute).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub developer_only_attribute: Option<::Value<bool>>,
        /// Property [`Mutable`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-schemaattribute.html#cfn-cognito-userpool-schemaattribute-mutable).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mutable: Option<::Value<bool>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-schemaattribute.html#cfn-cognito-userpool-schemaattribute-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`NumberAttributeConstraints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-schemaattribute.html#cfn-cognito-userpool-schemaattribute-numberattributeconstraints).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub number_attribute_constraints: Option<::Value<NumberAttributeConstraints>>,
        /// Property [`Required`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-schemaattribute.html#cfn-cognito-userpool-schemaattribute-required).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub required: Option<::Value<bool>>,
        /// Property [`StringAttributeConstraints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-schemaattribute.html#cfn-cognito-userpool-schemaattribute-stringattributeconstraints).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub string_attribute_constraints: Option<::Value<StringAttributeConstraints>>,
    }

    impl ::codec::SerializeValue for SchemaAttribute {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref attribute_data_type) = self.attribute_data_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttributeDataType", attribute_data_type)?;
            }
            if let Some(ref developer_only_attribute) = self.developer_only_attribute {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeveloperOnlyAttribute", developer_only_attribute)?;
            }
            if let Some(ref mutable) = self.mutable {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Mutable", mutable)?;
            }
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref number_attribute_constraints) = self.number_attribute_constraints {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumberAttributeConstraints", number_attribute_constraints)?;
            }
            if let Some(ref required) = self.required {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Required", required)?;
            }
            if let Some(ref string_attribute_constraints) = self.string_attribute_constraints {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StringAttributeConstraints", string_attribute_constraints)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SchemaAttribute {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SchemaAttribute, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SchemaAttribute;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SchemaAttribute")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attribute_data_type: Option<::Value<String>> = None;
                    let mut developer_only_attribute: Option<::Value<bool>> = None;
                    let mut mutable: Option<::Value<bool>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut number_attribute_constraints: Option<::Value<NumberAttributeConstraints>> = None;
                    let mut required: Option<::Value<bool>> = None;
                    let mut string_attribute_constraints: Option<::Value<StringAttributeConstraints>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AttributeDataType" => {
                                attribute_data_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeveloperOnlyAttribute" => {
                                developer_only_attribute = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Mutable" => {
                                mutable = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NumberAttributeConstraints" => {
                                number_attribute_constraints = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Required" => {
                                required = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StringAttributeConstraints" => {
                                string_attribute_constraints = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SchemaAttribute {
                        attribute_data_type: attribute_data_type,
                        developer_only_attribute: developer_only_attribute,
                        mutable: mutable,
                        name: name,
                        number_attribute_constraints: number_attribute_constraints,
                        required: required,
                        string_attribute_constraints: string_attribute_constraints,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Cognito::UserPool.SmsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-smsconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct SmsConfiguration {
        /// Property [`ExternalId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-smsconfiguration.html#cfn-cognito-userpool-smsconfiguration-externalid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub external_id: Option<::Value<String>>,
        /// Property [`SnsCallerArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-smsconfiguration.html#cfn-cognito-userpool-smsconfiguration-snscallerarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sns_caller_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SmsConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref external_id) = self.external_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExternalId", external_id)?;
            }
            if let Some(ref sns_caller_arn) = self.sns_caller_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnsCallerArn", sns_caller_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SmsConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SmsConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SmsConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SmsConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut external_id: Option<::Value<String>> = None;
                    let mut sns_caller_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ExternalId" => {
                                external_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SnsCallerArn" => {
                                sns_caller_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SmsConfiguration {
                        external_id: external_id,
                        sns_caller_arn: sns_caller_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Cognito::UserPool.StringAttributeConstraints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-stringattributeconstraints.html) property type.
    #[derive(Debug, Default)]
    pub struct StringAttributeConstraints {
        /// Property [`MaxLength`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-stringattributeconstraints.html#cfn-cognito-userpool-stringattributeconstraints-maxlength).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_length: Option<::Value<String>>,
        /// Property [`MinLength`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-stringattributeconstraints.html#cfn-cognito-userpool-stringattributeconstraints-minlength).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub min_length: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for StringAttributeConstraints {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref max_length) = self.max_length {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxLength", max_length)?;
            }
            if let Some(ref min_length) = self.min_length {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinLength", min_length)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StringAttributeConstraints {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StringAttributeConstraints, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StringAttributeConstraints;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StringAttributeConstraints")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut max_length: Option<::Value<String>> = None;
                    let mut min_length: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxLength" => {
                                max_length = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinLength" => {
                                min_length = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StringAttributeConstraints {
                        max_length: max_length,
                        min_length: min_length,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Cognito::UserPool.UserPoolAddOns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-userpooladdons.html) property type.
    #[derive(Debug, Default)]
    pub struct UserPoolAddOns {
        /// Property [`AdvancedSecurityMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-userpooladdons.html#cfn-cognito-userpool-userpooladdons-advancedsecuritymode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub advanced_security_mode: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for UserPoolAddOns {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref advanced_security_mode) = self.advanced_security_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdvancedSecurityMode", advanced_security_mode)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for UserPoolAddOns {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<UserPoolAddOns, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = UserPoolAddOns;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type UserPoolAddOns")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut advanced_security_mode: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AdvancedSecurityMode" => {
                                advanced_security_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(UserPoolAddOns {
                        advanced_security_mode: advanced_security_mode,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Cognito::UserPool.UsernameConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-usernameconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct UsernameConfiguration {
        /// Property [`CaseSensitive`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-usernameconfiguration.html#cfn-cognito-userpool-usernameconfiguration-casesensitive).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub case_sensitive: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for UsernameConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref case_sensitive) = self.case_sensitive {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CaseSensitive", case_sensitive)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for UsernameConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<UsernameConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = UsernameConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type UsernameConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut case_sensitive: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CaseSensitive" => {
                                case_sensitive = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(UsernameConfiguration {
                        case_sensitive: case_sensitive,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Cognito::UserPool.VerificationMessageTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-verificationmessagetemplate.html) property type.
    #[derive(Debug, Default)]
    pub struct VerificationMessageTemplate {
        /// Property [`DefaultEmailOption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-verificationmessagetemplate.html#cfn-cognito-userpool-verificationmessagetemplate-defaultemailoption).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_email_option: Option<::Value<String>>,
        /// Property [`EmailMessage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-verificationmessagetemplate.html#cfn-cognito-userpool-verificationmessagetemplate-emailmessage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub email_message: Option<::Value<String>>,
        /// Property [`EmailMessageByLink`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-verificationmessagetemplate.html#cfn-cognito-userpool-verificationmessagetemplate-emailmessagebylink).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub email_message_by_link: Option<::Value<String>>,
        /// Property [`EmailSubject`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-verificationmessagetemplate.html#cfn-cognito-userpool-verificationmessagetemplate-emailsubject).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub email_subject: Option<::Value<String>>,
        /// Property [`EmailSubjectByLink`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-verificationmessagetemplate.html#cfn-cognito-userpool-verificationmessagetemplate-emailsubjectbylink).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub email_subject_by_link: Option<::Value<String>>,
        /// Property [`SmsMessage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-verificationmessagetemplate.html#cfn-cognito-userpool-verificationmessagetemplate-smsmessage).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sms_message: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for VerificationMessageTemplate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref default_email_option) = self.default_email_option {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultEmailOption", default_email_option)?;
            }
            if let Some(ref email_message) = self.email_message {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EmailMessage", email_message)?;
            }
            if let Some(ref email_message_by_link) = self.email_message_by_link {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EmailMessageByLink", email_message_by_link)?;
            }
            if let Some(ref email_subject) = self.email_subject {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EmailSubject", email_subject)?;
            }
            if let Some(ref email_subject_by_link) = self.email_subject_by_link {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EmailSubjectByLink", email_subject_by_link)?;
            }
            if let Some(ref sms_message) = self.sms_message {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SmsMessage", sms_message)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VerificationMessageTemplate {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VerificationMessageTemplate, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VerificationMessageTemplate;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VerificationMessageTemplate")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut default_email_option: Option<::Value<String>> = None;
                    let mut email_message: Option<::Value<String>> = None;
                    let mut email_message_by_link: Option<::Value<String>> = None;
                    let mut email_subject: Option<::Value<String>> = None;
                    let mut email_subject_by_link: Option<::Value<String>> = None;
                    let mut sms_message: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DefaultEmailOption" => {
                                default_email_option = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EmailMessage" => {
                                email_message = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EmailMessageByLink" => {
                                email_message_by_link = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EmailSubject" => {
                                email_subject = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EmailSubjectByLink" => {
                                email_subject_by_link = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SmsMessage" => {
                                sms_message = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VerificationMessageTemplate {
                        default_email_option: default_email_option,
                        email_message: email_message,
                        email_message_by_link: email_message_by_link,
                        email_subject: email_subject,
                        email_subject_by_link: email_subject_by_link,
                        sms_message: sms_message,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod user_pool_client {
    //! Property types for the `UserPoolClient` resource.

    /// The [`AWS::Cognito::UserPoolClient.AnalyticsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolclient-analyticsconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct AnalyticsConfiguration {
        /// Property [`ApplicationArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolclient-analyticsconfiguration.html#cfn-cognito-userpoolclient-analyticsconfiguration-applicationarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub application_arn: Option<::Value<String>>,
        /// Property [`ApplicationId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolclient-analyticsconfiguration.html#cfn-cognito-userpoolclient-analyticsconfiguration-applicationid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub application_id: Option<::Value<String>>,
        /// Property [`ExternalId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolclient-analyticsconfiguration.html#cfn-cognito-userpoolclient-analyticsconfiguration-externalid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub external_id: Option<::Value<String>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolclient-analyticsconfiguration.html#cfn-cognito-userpoolclient-analyticsconfiguration-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: Option<::Value<String>>,
        /// Property [`UserDataShared`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolclient-analyticsconfiguration.html#cfn-cognito-userpoolclient-analyticsconfiguration-userdatashared).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_data_shared: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for AnalyticsConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref application_arn) = self.application_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationArn", application_arn)?;
            }
            if let Some(ref application_id) = self.application_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationId", application_id)?;
            }
            if let Some(ref external_id) = self.external_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExternalId", external_id)?;
            }
            if let Some(ref role_arn) = self.role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", role_arn)?;
            }
            if let Some(ref user_data_shared) = self.user_data_shared {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserDataShared", user_data_shared)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AnalyticsConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AnalyticsConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AnalyticsConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AnalyticsConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut application_arn: Option<::Value<String>> = None;
                    let mut application_id: Option<::Value<String>> = None;
                    let mut external_id: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut user_data_shared: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ApplicationArn" => {
                                application_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ApplicationId" => {
                                application_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExternalId" => {
                                external_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserDataShared" => {
                                user_data_shared = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AnalyticsConfiguration {
                        application_arn: application_arn,
                        application_id: application_id,
                        external_id: external_id,
                        role_arn: role_arn,
                        user_data_shared: user_data_shared,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Cognito::UserPoolClient.TokenValidityUnits`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolclient-tokenvalidityunits.html) property type.
    #[derive(Debug, Default)]
    pub struct TokenValidityUnits {
        /// Property [`AccessToken`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolclient-tokenvalidityunits.html#cfn-cognito-userpoolclient-tokenvalidityunits-accesstoken).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub access_token: Option<::Value<String>>,
        /// Property [`IdToken`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolclient-tokenvalidityunits.html#cfn-cognito-userpoolclient-tokenvalidityunits-idtoken).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id_token: Option<::Value<String>>,
        /// Property [`RefreshToken`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolclient-tokenvalidityunits.html#cfn-cognito-userpoolclient-tokenvalidityunits-refreshtoken).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub refresh_token: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for TokenValidityUnits {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref access_token) = self.access_token {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessToken", access_token)?;
            }
            if let Some(ref id_token) = self.id_token {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdToken", id_token)?;
            }
            if let Some(ref refresh_token) = self.refresh_token {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RefreshToken", refresh_token)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TokenValidityUnits {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TokenValidityUnits, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TokenValidityUnits;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TokenValidityUnits")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut access_token: Option<::Value<String>> = None;
                    let mut id_token: Option<::Value<String>> = None;
                    let mut refresh_token: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AccessToken" => {
                                access_token = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IdToken" => {
                                id_token = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RefreshToken" => {
                                refresh_token = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TokenValidityUnits {
                        access_token: access_token,
                        id_token: id_token,
                        refresh_token: refresh_token,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod user_pool_domain {
    //! Property types for the `UserPoolDomain` resource.

    /// The [`AWS::Cognito::UserPoolDomain.CustomDomainConfigType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpooldomain-customdomainconfigtype.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomDomainConfigType {
        /// Property [`CertificateArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpooldomain-customdomainconfigtype.html#cfn-cognito-userpooldomain-customdomainconfigtype-certificatearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub certificate_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CustomDomainConfigType {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref certificate_arn) = self.certificate_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateArn", certificate_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomDomainConfigType {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomDomainConfigType, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomDomainConfigType;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomDomainConfigType")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut certificate_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CertificateArn" => {
                                certificate_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomDomainConfigType {
                        certificate_arn: certificate_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod user_pool_resource_server {
    //! Property types for the `UserPoolResourceServer` resource.

    /// The [`AWS::Cognito::UserPoolResourceServer.ResourceServerScopeType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolresourceserver-resourceserverscopetype.html) property type.
    #[derive(Debug, Default)]
    pub struct ResourceServerScopeType {
        /// Property [`ScopeDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolresourceserver-resourceserverscopetype.html#cfn-cognito-userpoolresourceserver-resourceserverscopetype-scopedescription).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scope_description: ::Value<String>,
        /// Property [`ScopeName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolresourceserver-resourceserverscopetype.html#cfn-cognito-userpoolresourceserver-resourceserverscopetype-scopename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scope_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for ResourceServerScopeType {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScopeDescription", &self.scope_description)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScopeName", &self.scope_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResourceServerScopeType {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourceServerScopeType, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResourceServerScopeType;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResourceServerScopeType")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut scope_description: Option<::Value<String>> = None;
                    let mut scope_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ScopeDescription" => {
                                scope_description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScopeName" => {
                                scope_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResourceServerScopeType {
                        scope_description: scope_description.ok_or(::serde::de::Error::missing_field("ScopeDescription"))?,
                        scope_name: scope_name.ok_or(::serde::de::Error::missing_field("ScopeName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod user_pool_risk_configuration_attachment {
    //! Property types for the `UserPoolRiskConfigurationAttachment` resource.

    /// The [`AWS::Cognito::UserPoolRiskConfigurationAttachment.AccountTakeoverActionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolriskconfigurationattachment-accounttakeoveractiontype.html) property type.
    #[derive(Debug, Default)]
    pub struct AccountTakeoverActionType {
        /// Property [`EventAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolriskconfigurationattachment-accounttakeoveractiontype.html#cfn-cognito-userpoolriskconfigurationattachment-accounttakeoveractiontype-eventaction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub event_action: ::Value<String>,
        /// Property [`Notify`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolriskconfigurationattachment-accounttakeoveractiontype.html#cfn-cognito-userpoolriskconfigurationattachment-accounttakeoveractiontype-notify).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub notify: ::Value<bool>,
    }

    impl ::codec::SerializeValue for AccountTakeoverActionType {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventAction", &self.event_action)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Notify", &self.notify)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AccountTakeoverActionType {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AccountTakeoverActionType, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AccountTakeoverActionType;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AccountTakeoverActionType")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut event_action: Option<::Value<String>> = None;
                    let mut notify: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EventAction" => {
                                event_action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Notify" => {
                                notify = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AccountTakeoverActionType {
                        event_action: event_action.ok_or(::serde::de::Error::missing_field("EventAction"))?,
                        notify: notify.ok_or(::serde::de::Error::missing_field("Notify"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Cognito::UserPoolRiskConfigurationAttachment.AccountTakeoverActionsType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolriskconfigurationattachment-accounttakeoveractionstype.html) property type.
    #[derive(Debug, Default)]
    pub struct AccountTakeoverActionsType {
        /// Property [`HighAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolriskconfigurationattachment-accounttakeoveractionstype.html#cfn-cognito-userpoolriskconfigurationattachment-accounttakeoveractionstype-highaction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub high_action: Option<::Value<AccountTakeoverActionType>>,
        /// Property [`LowAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolriskconfigurationattachment-accounttakeoveractionstype.html#cfn-cognito-userpoolriskconfigurationattachment-accounttakeoveractionstype-lowaction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub low_action: Option<::Value<AccountTakeoverActionType>>,
        /// Property [`MediumAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolriskconfigurationattachment-accounttakeoveractionstype.html#cfn-cognito-userpoolriskconfigurationattachment-accounttakeoveractionstype-mediumaction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub medium_action: Option<::Value<AccountTakeoverActionType>>,
    }

    impl ::codec::SerializeValue for AccountTakeoverActionsType {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref high_action) = self.high_action {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HighAction", high_action)?;
            }
            if let Some(ref low_action) = self.low_action {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LowAction", low_action)?;
            }
            if let Some(ref medium_action) = self.medium_action {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MediumAction", medium_action)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AccountTakeoverActionsType {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AccountTakeoverActionsType, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AccountTakeoverActionsType;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AccountTakeoverActionsType")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut high_action: Option<::Value<AccountTakeoverActionType>> = None;
                    let mut low_action: Option<::Value<AccountTakeoverActionType>> = None;
                    let mut medium_action: Option<::Value<AccountTakeoverActionType>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HighAction" => {
                                high_action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LowAction" => {
                                low_action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MediumAction" => {
                                medium_action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AccountTakeoverActionsType {
                        high_action: high_action,
                        low_action: low_action,
                        medium_action: medium_action,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Cognito::UserPoolRiskConfigurationAttachment.AccountTakeoverRiskConfigurationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolriskconfigurationattachment-accounttakeoverriskconfigurationtype.html) property type.
    #[derive(Debug, Default)]
    pub struct AccountTakeoverRiskConfigurationType {
        /// Property [`Actions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolriskconfigurationattachment-accounttakeoverriskconfigurationtype.html#cfn-cognito-userpoolriskconfigurationattachment-accounttakeoverriskconfigurationtype-actions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub actions: ::Value<AccountTakeoverActionsType>,
        /// Property [`NotifyConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolriskconfigurationattachment-accounttakeoverriskconfigurationtype.html#cfn-cognito-userpoolriskconfigurationattachment-accounttakeoverriskconfigurationtype-notifyconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub notify_configuration: Option<::Value<NotifyConfigurationType>>,
    }

    impl ::codec::SerializeValue for AccountTakeoverRiskConfigurationType {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Actions", &self.actions)?;
            if let Some(ref notify_configuration) = self.notify_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotifyConfiguration", notify_configuration)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AccountTakeoverRiskConfigurationType {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AccountTakeoverRiskConfigurationType, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AccountTakeoverRiskConfigurationType;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AccountTakeoverRiskConfigurationType")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut actions: Option<::Value<AccountTakeoverActionsType>> = None;
                    let mut notify_configuration: Option<::Value<NotifyConfigurationType>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Actions" => {
                                actions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NotifyConfiguration" => {
                                notify_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AccountTakeoverRiskConfigurationType {
                        actions: actions.ok_or(::serde::de::Error::missing_field("Actions"))?,
                        notify_configuration: notify_configuration,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Cognito::UserPoolRiskConfigurationAttachment.CompromisedCredentialsActionsType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolriskconfigurationattachment-compromisedcredentialsactionstype.html) property type.
    #[derive(Debug, Default)]
    pub struct CompromisedCredentialsActionsType {
        /// Property [`EventAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolriskconfigurationattachment-compromisedcredentialsactionstype.html#cfn-cognito-userpoolriskconfigurationattachment-compromisedcredentialsactionstype-eventaction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub event_action: ::Value<String>,
    }

    impl ::codec::SerializeValue for CompromisedCredentialsActionsType {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventAction", &self.event_action)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CompromisedCredentialsActionsType {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CompromisedCredentialsActionsType, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CompromisedCredentialsActionsType;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CompromisedCredentialsActionsType")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut event_action: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EventAction" => {
                                event_action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CompromisedCredentialsActionsType {
                        event_action: event_action.ok_or(::serde::de::Error::missing_field("EventAction"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Cognito::UserPoolRiskConfigurationAttachment.CompromisedCredentialsRiskConfigurationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolriskconfigurationattachment-compromisedcredentialsriskconfigurationtype.html) property type.
    #[derive(Debug, Default)]
    pub struct CompromisedCredentialsRiskConfigurationType {
        /// Property [`Actions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolriskconfigurationattachment-compromisedcredentialsriskconfigurationtype.html#cfn-cognito-userpoolriskconfigurationattachment-compromisedcredentialsriskconfigurationtype-actions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub actions: ::Value<CompromisedCredentialsActionsType>,
        /// Property [`EventFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolriskconfigurationattachment-compromisedcredentialsriskconfigurationtype.html#cfn-cognito-userpoolriskconfigurationattachment-compromisedcredentialsriskconfigurationtype-eventfilter).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub event_filter: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for CompromisedCredentialsRiskConfigurationType {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Actions", &self.actions)?;
            if let Some(ref event_filter) = self.event_filter {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventFilter", event_filter)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CompromisedCredentialsRiskConfigurationType {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CompromisedCredentialsRiskConfigurationType, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CompromisedCredentialsRiskConfigurationType;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CompromisedCredentialsRiskConfigurationType")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut actions: Option<::Value<CompromisedCredentialsActionsType>> = None;
                    let mut event_filter: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Actions" => {
                                actions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EventFilter" => {
                                event_filter = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CompromisedCredentialsRiskConfigurationType {
                        actions: actions.ok_or(::serde::de::Error::missing_field("Actions"))?,
                        event_filter: event_filter,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Cognito::UserPoolRiskConfigurationAttachment.NotifyConfigurationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolriskconfigurationattachment-notifyconfigurationtype.html) property type.
    #[derive(Debug, Default)]
    pub struct NotifyConfigurationType {
        /// Property [`BlockEmail`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolriskconfigurationattachment-notifyconfigurationtype.html#cfn-cognito-userpoolriskconfigurationattachment-notifyconfigurationtype-blockemail).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub block_email: Option<::Value<NotifyEmailType>>,
        /// Property [`From`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolriskconfigurationattachment-notifyconfigurationtype.html#cfn-cognito-userpoolriskconfigurationattachment-notifyconfigurationtype-from).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub from: Option<::Value<String>>,
        /// Property [`MfaEmail`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolriskconfigurationattachment-notifyconfigurationtype.html#cfn-cognito-userpoolriskconfigurationattachment-notifyconfigurationtype-mfaemail).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mfa_email: Option<::Value<NotifyEmailType>>,
        /// Property [`NoActionEmail`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolriskconfigurationattachment-notifyconfigurationtype.html#cfn-cognito-userpoolriskconfigurationattachment-notifyconfigurationtype-noactionemail).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub no_action_email: Option<::Value<NotifyEmailType>>,
        /// Property [`ReplyTo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolriskconfigurationattachment-notifyconfigurationtype.html#cfn-cognito-userpoolriskconfigurationattachment-notifyconfigurationtype-replyto).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub reply_to: Option<::Value<String>>,
        /// Property [`SourceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolriskconfigurationattachment-notifyconfigurationtype.html#cfn-cognito-userpoolriskconfigurationattachment-notifyconfigurationtype-sourcearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for NotifyConfigurationType {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref block_email) = self.block_email {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BlockEmail", block_email)?;
            }
            if let Some(ref from) = self.from {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "From", from)?;
            }
            if let Some(ref mfa_email) = self.mfa_email {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MfaEmail", mfa_email)?;
            }
            if let Some(ref no_action_email) = self.no_action_email {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NoActionEmail", no_action_email)?;
            }
            if let Some(ref reply_to) = self.reply_to {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplyTo", reply_to)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceArn", &self.source_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NotifyConfigurationType {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NotifyConfigurationType, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NotifyConfigurationType;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NotifyConfigurationType")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut block_email: Option<::Value<NotifyEmailType>> = None;
                    let mut from: Option<::Value<String>> = None;
                    let mut mfa_email: Option<::Value<NotifyEmailType>> = None;
                    let mut no_action_email: Option<::Value<NotifyEmailType>> = None;
                    let mut reply_to: Option<::Value<String>> = None;
                    let mut source_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BlockEmail" => {
                                block_email = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "From" => {
                                from = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MfaEmail" => {
                                mfa_email = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NoActionEmail" => {
                                no_action_email = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReplyTo" => {
                                reply_to = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SourceArn" => {
                                source_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NotifyConfigurationType {
                        block_email: block_email,
                        from: from,
                        mfa_email: mfa_email,
                        no_action_email: no_action_email,
                        reply_to: reply_to,
                        source_arn: source_arn.ok_or(::serde::de::Error::missing_field("SourceArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Cognito::UserPoolRiskConfigurationAttachment.NotifyEmailType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolriskconfigurationattachment-notifyemailtype.html) property type.
    #[derive(Debug, Default)]
    pub struct NotifyEmailType {
        /// Property [`HtmlBody`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolriskconfigurationattachment-notifyemailtype.html#cfn-cognito-userpoolriskconfigurationattachment-notifyemailtype-htmlbody).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub html_body: Option<::Value<String>>,
        /// Property [`Subject`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolriskconfigurationattachment-notifyemailtype.html#cfn-cognito-userpoolriskconfigurationattachment-notifyemailtype-subject).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subject: ::Value<String>,
        /// Property [`TextBody`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolriskconfigurationattachment-notifyemailtype.html#cfn-cognito-userpoolriskconfigurationattachment-notifyemailtype-textbody).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub text_body: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for NotifyEmailType {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref html_body) = self.html_body {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HtmlBody", html_body)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subject", &self.subject)?;
            if let Some(ref text_body) = self.text_body {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextBody", text_body)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NotifyEmailType {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NotifyEmailType, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NotifyEmailType;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NotifyEmailType")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut html_body: Option<::Value<String>> = None;
                    let mut subject: Option<::Value<String>> = None;
                    let mut text_body: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HtmlBody" => {
                                html_body = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Subject" => {
                                subject = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TextBody" => {
                                text_body = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NotifyEmailType {
                        html_body: html_body,
                        subject: subject.ok_or(::serde::de::Error::missing_field("Subject"))?,
                        text_body: text_body,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Cognito::UserPoolRiskConfigurationAttachment.RiskExceptionConfigurationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolriskconfigurationattachment-riskexceptionconfigurationtype.html) property type.
    #[derive(Debug, Default)]
    pub struct RiskExceptionConfigurationType {
        /// Property [`BlockedIPRangeList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolriskconfigurationattachment-riskexceptionconfigurationtype.html#cfn-cognito-userpoolriskconfigurationattachment-riskexceptionconfigurationtype-blockediprangelist).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub blocked_ip_range_list: Option<::ValueList<String>>,
        /// Property [`SkippedIPRangeList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolriskconfigurationattachment-riskexceptionconfigurationtype.html#cfn-cognito-userpoolriskconfigurationattachment-riskexceptionconfigurationtype-skippediprangelist).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub skipped_ip_range_list: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for RiskExceptionConfigurationType {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref blocked_ip_range_list) = self.blocked_ip_range_list {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BlockedIPRangeList", blocked_ip_range_list)?;
            }
            if let Some(ref skipped_ip_range_list) = self.skipped_ip_range_list {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SkippedIPRangeList", skipped_ip_range_list)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RiskExceptionConfigurationType {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RiskExceptionConfigurationType, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RiskExceptionConfigurationType;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RiskExceptionConfigurationType")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut blocked_ip_range_list: Option<::ValueList<String>> = None;
                    let mut skipped_ip_range_list: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BlockedIPRangeList" => {
                                blocked_ip_range_list = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SkippedIPRangeList" => {
                                skipped_ip_range_list = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RiskExceptionConfigurationType {
                        blocked_ip_range_list: blocked_ip_range_list,
                        skipped_ip_range_list: skipped_ip_range_list,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod user_pool_user {
    //! Property types for the `UserPoolUser` resource.

    /// The [`AWS::Cognito::UserPoolUser.AttributeType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpooluser-attributetype.html) property type.
    #[derive(Debug, Default)]
    pub struct AttributeType {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpooluser-attributetype.html#cfn-cognito-userpooluser-attributetype-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpooluser-attributetype.html#cfn-cognito-userpooluser-attributetype-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AttributeType {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AttributeType {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AttributeType, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AttributeType;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AttributeType")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AttributeType {
                        name: name,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
