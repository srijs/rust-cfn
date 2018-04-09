//! Types for the `Cognito` service.

/// The [`AWS::Cognito::IdentityPool`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-identitypool.html) resource type.
#[derive(Debug)]
pub struct IdentityPool {
    properties: IdentityPoolProperties
}

/// Properties for the `IdentityPool` resource.
#[derive(Debug)]
pub struct IdentityPoolProperties {
    /// Property `AllowUnauthenticatedIdentities`.
    pub allow_unauthenticated_identities: ::Value<bool>,
    /// Property `CognitoEvents`.
    pub cognito_events: Option<::Value<::json::Value>>,
    /// Property `CognitoIdentityProviders`.
    pub cognito_identity_providers: Option<::ValueList<self::identity_pool::CognitoIdentityProvider>>,
    /// Property `CognitoStreams`.
    pub cognito_streams: Option<::Value<self::identity_pool::CognitoStreams>>,
    /// Property `DeveloperProviderName`.
    pub developer_provider_name: Option<::Value<String>>,
    /// Property `IdentityPoolName`.
    pub identity_pool_name: Option<::Value<String>>,
    /// Property `OpenIdConnectProviderARNs`.
    pub open_id_connect_provider_ar_ns: Option<::ValueList<String>>,
    /// Property `PushSync`.
    pub push_sync: Option<::Value<self::identity_pool::PushSync>>,
    /// Property `SamlProviderARNs`.
    pub saml_provider_ar_ns: Option<::ValueList<String>>,
    /// Property `SupportedLoginProviders`.
    pub supported_login_providers: Option<::Value<::json::Value>>,
}

impl ::serde::Serialize for IdentityPoolProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowUnauthenticatedIdentities", &self.allow_unauthenticated_identities)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CognitoEvents", &self.cognito_events)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CognitoIdentityProviders", &self.cognito_identity_providers)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CognitoStreams", &self.cognito_streams)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeveloperProviderName", &self.developer_provider_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdentityPoolName", &self.identity_pool_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "OpenIdConnectProviderARNs", &self.open_id_connect_provider_ar_ns)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PushSync", &self.push_sync)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SamlProviderARNs", &self.saml_provider_ar_ns)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SupportedLoginProviders", &self.supported_login_providers)?;
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
                let mut allow_unauthenticated_identities = None;
                let mut cognito_events = None;
                let mut cognito_identity_providers = None;
                let mut cognito_streams = None;
                let mut developer_provider_name = None;
                let mut identity_pool_name = None;
                let mut open_id_connect_provider_ar_ns = None;
                let mut push_sync = None;
                let mut saml_provider_ar_ns = None;
                let mut supported_login_providers = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AllowUnauthenticatedIdentities" => {
                            allow_unauthenticated_identities = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "CognitoEvents" => {
                            cognito_events = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "CognitoIdentityProviders" => {
                            cognito_identity_providers = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "CognitoStreams" => {
                            cognito_streams = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DeveloperProviderName" => {
                            developer_provider_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "IdentityPoolName" => {
                            identity_pool_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "OpenIdConnectProviderARNs" => {
                            open_id_connect_provider_ar_ns = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "PushSync" => {
                            push_sync = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SamlProviderARNs" => {
                            saml_provider_ar_ns = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SupportedLoginProviders" => {
                            supported_login_providers = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(IdentityPoolProperties {
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
#[derive(Debug)]
pub struct IdentityPoolRoleAttachment {
    properties: IdentityPoolRoleAttachmentProperties
}

/// Properties for the `IdentityPoolRoleAttachment` resource.
#[derive(Debug)]
pub struct IdentityPoolRoleAttachmentProperties {
    /// Property `IdentityPoolId`.
    pub identity_pool_id: ::Value<String>,
    /// Property `RoleMappings`.
    pub role_mappings: Option<::Value<::json::Value>>,
    /// Property `Roles`.
    pub roles: Option<::Value<::json::Value>>,
}

impl ::serde::Serialize for IdentityPoolRoleAttachmentProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdentityPoolId", &self.identity_pool_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleMappings", &self.role_mappings)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Roles", &self.roles)?;
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
                let mut identity_pool_id = None;
                let mut role_mappings = None;
                let mut roles = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "IdentityPoolId" => {
                            identity_pool_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RoleMappings" => {
                            role_mappings = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Roles" => {
                            roles = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
#[derive(Debug)]
pub struct UserPool {
    properties: UserPoolProperties
}

/// Properties for the `UserPool` resource.
#[derive(Debug)]
pub struct UserPoolProperties {
    /// Property `AdminCreateUserConfig`.
    pub admin_create_user_config: Option<::Value<self::user_pool::AdminCreateUserConfig>>,
    /// Property `AliasAttributes`.
    pub alias_attributes: Option<::ValueList<String>>,
    /// Property `AutoVerifiedAttributes`.
    pub auto_verified_attributes: Option<::ValueList<String>>,
    /// Property `DeviceConfiguration`.
    pub device_configuration: Option<::Value<self::user_pool::DeviceConfiguration>>,
    /// Property `EmailConfiguration`.
    pub email_configuration: Option<::Value<self::user_pool::EmailConfiguration>>,
    /// Property `EmailVerificationMessage`.
    pub email_verification_message: Option<::Value<String>>,
    /// Property `EmailVerificationSubject`.
    pub email_verification_subject: Option<::Value<String>>,
    /// Property `LambdaConfig`.
    pub lambda_config: Option<::Value<self::user_pool::LambdaConfig>>,
    /// Property `MfaConfiguration`.
    pub mfa_configuration: Option<::Value<String>>,
    /// Property `Policies`.
    pub policies: Option<::Value<self::user_pool::Policies>>,
    /// Property `Schema`.
    pub schema: Option<::ValueList<self::user_pool::SchemaAttribute>>,
    /// Property `SmsAuthenticationMessage`.
    pub sms_authentication_message: Option<::Value<String>>,
    /// Property `SmsConfiguration`.
    pub sms_configuration: Option<::Value<self::user_pool::SmsConfiguration>>,
    /// Property `SmsVerificationMessage`.
    pub sms_verification_message: Option<::Value<String>>,
    /// Property `UserPoolName`.
    pub user_pool_name: Option<::Value<String>>,
    /// Property `UserPoolTags`.
    pub user_pool_tags: Option<::Value<::json::Value>>,
}

impl ::serde::Serialize for UserPoolProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdminCreateUserConfig", &self.admin_create_user_config)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AliasAttributes", &self.alias_attributes)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoVerifiedAttributes", &self.auto_verified_attributes)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceConfiguration", &self.device_configuration)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EmailConfiguration", &self.email_configuration)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EmailVerificationMessage", &self.email_verification_message)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EmailVerificationSubject", &self.email_verification_subject)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaConfig", &self.lambda_config)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MfaConfiguration", &self.mfa_configuration)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Policies", &self.policies)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Schema", &self.schema)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SmsAuthenticationMessage", &self.sms_authentication_message)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SmsConfiguration", &self.sms_configuration)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SmsVerificationMessage", &self.sms_verification_message)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserPoolName", &self.user_pool_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserPoolTags", &self.user_pool_tags)?;
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
                let mut admin_create_user_config = None;
                let mut alias_attributes = None;
                let mut auto_verified_attributes = None;
                let mut device_configuration = None;
                let mut email_configuration = None;
                let mut email_verification_message = None;
                let mut email_verification_subject = None;
                let mut lambda_config = None;
                let mut mfa_configuration = None;
                let mut policies = None;
                let mut schema = None;
                let mut sms_authentication_message = None;
                let mut sms_configuration = None;
                let mut sms_verification_message = None;
                let mut user_pool_name = None;
                let mut user_pool_tags = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AdminCreateUserConfig" => {
                            admin_create_user_config = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "AliasAttributes" => {
                            alias_attributes = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "AutoVerifiedAttributes" => {
                            auto_verified_attributes = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DeviceConfiguration" => {
                            device_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EmailConfiguration" => {
                            email_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EmailVerificationMessage" => {
                            email_verification_message = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EmailVerificationSubject" => {
                            email_verification_subject = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "LambdaConfig" => {
                            lambda_config = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "MfaConfiguration" => {
                            mfa_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Policies" => {
                            policies = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Schema" => {
                            schema = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SmsAuthenticationMessage" => {
                            sms_authentication_message = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SmsConfiguration" => {
                            sms_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SmsVerificationMessage" => {
                            sms_verification_message = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "UserPoolName" => {
                            user_pool_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "UserPoolTags" => {
                            user_pool_tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(UserPoolProperties {
                    admin_create_user_config: admin_create_user_config,
                    alias_attributes: alias_attributes,
                    auto_verified_attributes: auto_verified_attributes,
                    device_configuration: device_configuration,
                    email_configuration: email_configuration,
                    email_verification_message: email_verification_message,
                    email_verification_subject: email_verification_subject,
                    lambda_config: lambda_config,
                    mfa_configuration: mfa_configuration,
                    policies: policies,
                    schema: schema,
                    sms_authentication_message: sms_authentication_message,
                    sms_configuration: sms_configuration,
                    sms_verification_message: sms_verification_message,
                    user_pool_name: user_pool_name,
                    user_pool_tags: user_pool_tags,
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
#[derive(Debug)]
pub struct UserPoolClient {
    properties: UserPoolClientProperties
}

/// Properties for the `UserPoolClient` resource.
#[derive(Debug)]
pub struct UserPoolClientProperties {
    /// Property `ClientName`.
    pub client_name: Option<::Value<String>>,
    /// Property `ExplicitAuthFlows`.
    pub explicit_auth_flows: Option<::ValueList<String>>,
    /// Property `GenerateSecret`.
    pub generate_secret: Option<::Value<bool>>,
    /// Property `ReadAttributes`.
    pub read_attributes: Option<::ValueList<String>>,
    /// Property `RefreshTokenValidity`.
    pub refresh_token_validity: Option<::Value<f64>>,
    /// Property `UserPoolId`.
    pub user_pool_id: ::Value<String>,
    /// Property `WriteAttributes`.
    pub write_attributes: Option<::ValueList<String>>,
}

impl ::serde::Serialize for UserPoolClientProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientName", &self.client_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExplicitAuthFlows", &self.explicit_auth_flows)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "GenerateSecret", &self.generate_secret)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReadAttributes", &self.read_attributes)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RefreshTokenValidity", &self.refresh_token_validity)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserPoolId", &self.user_pool_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "WriteAttributes", &self.write_attributes)?;
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
                let mut client_name = None;
                let mut explicit_auth_flows = None;
                let mut generate_secret = None;
                let mut read_attributes = None;
                let mut refresh_token_validity = None;
                let mut user_pool_id = None;
                let mut write_attributes = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ClientName" => {
                            client_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ExplicitAuthFlows" => {
                            explicit_auth_flows = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "GenerateSecret" => {
                            generate_secret = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ReadAttributes" => {
                            read_attributes = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RefreshTokenValidity" => {
                            refresh_token_validity = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "UserPoolId" => {
                            user_pool_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "WriteAttributes" => {
                            write_attributes = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(UserPoolClientProperties {
                    client_name: client_name,
                    explicit_auth_flows: explicit_auth_flows,
                    generate_secret: generate_secret,
                    read_attributes: read_attributes,
                    refresh_token_validity: refresh_token_validity,
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

/// The [`AWS::Cognito::UserPoolGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolgroup.html) resource type.
#[derive(Debug)]
pub struct UserPoolGroup {
    properties: UserPoolGroupProperties
}

/// Properties for the `UserPoolGroup` resource.
#[derive(Debug)]
pub struct UserPoolGroupProperties {
    /// Property `Description`.
    pub description: Option<::Value<String>>,
    /// Property `GroupName`.
    pub group_name: Option<::Value<String>>,
    /// Property `Precedence`.
    pub precedence: Option<::Value<f64>>,
    /// Property `RoleArn`.
    pub role_arn: Option<::Value<String>>,
    /// Property `UserPoolId`.
    pub user_pool_id: ::Value<String>,
}

impl ::serde::Serialize for UserPoolGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupName", &self.group_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Precedence", &self.precedence)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
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
                let mut description = None;
                let mut group_name = None;
                let mut precedence = None;
                let mut role_arn = None;
                let mut user_pool_id = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "GroupName" => {
                            group_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Precedence" => {
                            precedence = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RoleArn" => {
                            role_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "UserPoolId" => {
                            user_pool_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
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

/// The [`AWS::Cognito::UserPoolUser`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpooluser.html) resource type.
#[derive(Debug)]
pub struct UserPoolUser {
    properties: UserPoolUserProperties
}

/// Properties for the `UserPoolUser` resource.
#[derive(Debug)]
pub struct UserPoolUserProperties {
    /// Property `DesiredDeliveryMediums`.
    pub desired_delivery_mediums: Option<::ValueList<String>>,
    /// Property `ForceAliasCreation`.
    pub force_alias_creation: Option<::Value<bool>>,
    /// Property `MessageAction`.
    pub message_action: Option<::Value<String>>,
    /// Property `UserAttributes`.
    pub user_attributes: Option<::ValueList<self::user_pool_user::AttributeType>>,
    /// Property `UserPoolId`.
    pub user_pool_id: ::Value<String>,
    /// Property `Username`.
    pub username: Option<::Value<String>>,
    /// Property `ValidationData`.
    pub validation_data: Option<::ValueList<self::user_pool_user::AttributeType>>,
}

impl ::serde::Serialize for UserPoolUserProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DesiredDeliveryMediums", &self.desired_delivery_mediums)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ForceAliasCreation", &self.force_alias_creation)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MessageAction", &self.message_action)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserAttributes", &self.user_attributes)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserPoolId", &self.user_pool_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Username", &self.username)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ValidationData", &self.validation_data)?;
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
                let mut desired_delivery_mediums = None;
                let mut force_alias_creation = None;
                let mut message_action = None;
                let mut user_attributes = None;
                let mut user_pool_id = None;
                let mut username = None;
                let mut validation_data = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DesiredDeliveryMediums" => {
                            desired_delivery_mediums = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ForceAliasCreation" => {
                            force_alias_creation = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "MessageAction" => {
                            message_action = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "UserAttributes" => {
                            user_attributes = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "UserPoolId" => {
                            user_pool_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Username" => {
                            username = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ValidationData" => {
                            validation_data = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(UserPoolUserProperties {
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
#[derive(Debug)]
pub struct UserPoolUserToGroupAttachment {
    properties: UserPoolUserToGroupAttachmentProperties
}

/// Properties for the `UserPoolUserToGroupAttachment` resource.
#[derive(Debug)]
pub struct UserPoolUserToGroupAttachmentProperties {
    /// Property `GroupName`.
    pub group_name: ::Value<String>,
    /// Property `UserPoolId`.
    pub user_pool_id: ::Value<String>,
    /// Property `Username`.
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
                let mut group_name = None;
                let mut user_pool_id = None;
                let mut username = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "GroupName" => {
                            group_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "UserPoolId" => {
                            user_pool_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Username" => {
                            username = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
    #[derive(Debug)]
    pub struct CognitoIdentityProvider {
        /// Property `ClientId`.
        pub client_id: Option<::Value<String>>,
        /// Property `ProviderName`.
        pub provider_name: Option<::Value<String>>,
        /// Property `ServerSideTokenCheck`.
        pub server_side_token_check: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for CognitoIdentityProvider {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientId", &self.client_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProviderName", &self.provider_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerSideTokenCheck", &self.server_side_token_check)?;
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
                    let mut client_id = None;
                    let mut provider_name = None;
                    let mut server_side_token_check = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ClientId" => {
                                client_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ProviderName" => {
                                provider_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ServerSideTokenCheck" => {
                                server_side_token_check = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
    #[derive(Debug)]
    pub struct CognitoStreams {
        /// Property `RoleArn`.
        pub role_arn: Option<::Value<String>>,
        /// Property `StreamName`.
        pub stream_name: Option<::Value<String>>,
        /// Property `StreamingStatus`.
        pub streaming_status: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CognitoStreams {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamName", &self.stream_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamingStatus", &self.streaming_status)?;
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
                    let mut role_arn = None;
                    let mut stream_name = None;
                    let mut streaming_status = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RoleArn" => {
                                role_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "StreamName" => {
                                stream_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "StreamingStatus" => {
                                streaming_status = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
    #[derive(Debug)]
    pub struct PushSync {
        /// Property `ApplicationArns`.
        pub application_arns: Option<::ValueList<String>>,
        /// Property `RoleArn`.
        pub role_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for PushSync {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApplicationArns", &self.application_arns)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
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
                    let mut application_arns = None;
                    let mut role_arn = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ApplicationArns" => {
                                application_arns = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RoleArn" => {
                                role_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
    #[derive(Debug)]
    pub struct MappingRule {
        /// Property `Claim`.
        pub claim: ::Value<String>,
        /// Property `MatchType`.
        pub match_type: ::Value<String>,
        /// Property `RoleARN`.
        pub role_arn: ::Value<String>,
        /// Property `Value`.
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
                    let mut claim = None;
                    let mut match_type = None;
                    let mut role_arn = None;
                    let mut value = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Claim" => {
                                claim = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "MatchType" => {
                                match_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RoleARN" => {
                                role_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Value" => {
                                value = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
    #[derive(Debug)]
    pub struct RoleMapping {
        /// Property `AmbiguousRoleResolution`.
        pub ambiguous_role_resolution: Option<::Value<String>>,
        /// Property `RulesConfiguration`.
        pub rules_configuration: Option<::Value<RulesConfigurationType>>,
        /// Property `Type`.
        pub type_: ::Value<String>,
    }

    impl ::codec::SerializeValue for RoleMapping {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AmbiguousRoleResolution", &self.ambiguous_role_resolution)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RulesConfiguration", &self.rules_configuration)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
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
                    let mut ambiguous_role_resolution = None;
                    let mut rules_configuration = None;
                    let mut type_ = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AmbiguousRoleResolution" => {
                                ambiguous_role_resolution = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RulesConfiguration" => {
                                rules_configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Type" => {
                                type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(RoleMapping {
                        ambiguous_role_resolution: ambiguous_role_resolution,
                        rules_configuration: rules_configuration,
                        type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Cognito::IdentityPoolRoleAttachment.RulesConfigurationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypoolroleattachment-rulesconfigurationtype.html) property type.
    #[derive(Debug)]
    pub struct RulesConfigurationType {
        /// Property `Rules`.
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
                    let mut rules = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Rules" => {
                                rules = Some(::serde::de::MapAccess::next_value(&mut map)?);
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

    /// The [`AWS::Cognito::UserPool.AdminCreateUserConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-admincreateuserconfig.html) property type.
    #[derive(Debug)]
    pub struct AdminCreateUserConfig {
        /// Property `AllowAdminCreateUserOnly`.
        pub allow_admin_create_user_only: Option<::Value<bool>>,
        /// Property `InviteMessageTemplate`.
        pub invite_message_template: Option<::Value<InviteMessageTemplate>>,
        /// Property `UnusedAccountValidityDays`.
        pub unused_account_validity_days: Option<::Value<f64>>,
    }

    impl ::codec::SerializeValue for AdminCreateUserConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowAdminCreateUserOnly", &self.allow_admin_create_user_only)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InviteMessageTemplate", &self.invite_message_template)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UnusedAccountValidityDays", &self.unused_account_validity_days)?;
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
                    let mut allow_admin_create_user_only = None;
                    let mut invite_message_template = None;
                    let mut unused_account_validity_days = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllowAdminCreateUserOnly" => {
                                allow_admin_create_user_only = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "InviteMessageTemplate" => {
                                invite_message_template = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "UnusedAccountValidityDays" => {
                                unused_account_validity_days = Some(::serde::de::MapAccess::next_value(&mut map)?);
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

    /// The [`AWS::Cognito::UserPool.DeviceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-deviceconfiguration.html) property type.
    #[derive(Debug)]
    pub struct DeviceConfiguration {
        /// Property `ChallengeRequiredOnNewDevice`.
        pub challenge_required_on_new_device: Option<::Value<bool>>,
        /// Property `DeviceOnlyRememberedOnUserPrompt`.
        pub device_only_remembered_on_user_prompt: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for DeviceConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChallengeRequiredOnNewDevice", &self.challenge_required_on_new_device)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceOnlyRememberedOnUserPrompt", &self.device_only_remembered_on_user_prompt)?;
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
                    let mut challenge_required_on_new_device = None;
                    let mut device_only_remembered_on_user_prompt = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ChallengeRequiredOnNewDevice" => {
                                challenge_required_on_new_device = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "DeviceOnlyRememberedOnUserPrompt" => {
                                device_only_remembered_on_user_prompt = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
    #[derive(Debug)]
    pub struct EmailConfiguration {
        /// Property `ReplyToEmailAddress`.
        pub reply_to_email_address: Option<::Value<String>>,
        /// Property `SourceArn`.
        pub source_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EmailConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplyToEmailAddress", &self.reply_to_email_address)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceArn", &self.source_arn)?;
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
                    let mut reply_to_email_address = None;
                    let mut source_arn = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ReplyToEmailAddress" => {
                                reply_to_email_address = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SourceArn" => {
                                source_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(EmailConfiguration {
                        reply_to_email_address: reply_to_email_address,
                        source_arn: source_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Cognito::UserPool.InviteMessageTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-invitemessagetemplate.html) property type.
    #[derive(Debug)]
    pub struct InviteMessageTemplate {
        /// Property `EmailMessage`.
        pub email_message: Option<::Value<String>>,
        /// Property `EmailSubject`.
        pub email_subject: Option<::Value<String>>,
        /// Property `SMSMessage`.
        pub sms_message: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for InviteMessageTemplate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EmailMessage", &self.email_message)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EmailSubject", &self.email_subject)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SMSMessage", &self.sms_message)?;
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
                    let mut email_message = None;
                    let mut email_subject = None;
                    let mut sms_message = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EmailMessage" => {
                                email_message = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "EmailSubject" => {
                                email_subject = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SMSMessage" => {
                                sms_message = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
    #[derive(Debug)]
    pub struct LambdaConfig {
        /// Property `CreateAuthChallenge`.
        pub create_auth_challenge: Option<::Value<String>>,
        /// Property `CustomMessage`.
        pub custom_message: Option<::Value<String>>,
        /// Property `DefineAuthChallenge`.
        pub define_auth_challenge: Option<::Value<String>>,
        /// Property `PostAuthentication`.
        pub post_authentication: Option<::Value<String>>,
        /// Property `PostConfirmation`.
        pub post_confirmation: Option<::Value<String>>,
        /// Property `PreAuthentication`.
        pub pre_authentication: Option<::Value<String>>,
        /// Property `PreSignUp`.
        pub pre_sign_up: Option<::Value<String>>,
        /// Property `VerifyAuthChallengeResponse`.
        pub verify_auth_challenge_response: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LambdaConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CreateAuthChallenge", &self.create_auth_challenge)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomMessage", &self.custom_message)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefineAuthChallenge", &self.define_auth_challenge)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PostAuthentication", &self.post_authentication)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PostConfirmation", &self.post_confirmation)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreAuthentication", &self.pre_authentication)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreSignUp", &self.pre_sign_up)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VerifyAuthChallengeResponse", &self.verify_auth_challenge_response)?;
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
                    let mut create_auth_challenge = None;
                    let mut custom_message = None;
                    let mut define_auth_challenge = None;
                    let mut post_authentication = None;
                    let mut post_confirmation = None;
                    let mut pre_authentication = None;
                    let mut pre_sign_up = None;
                    let mut verify_auth_challenge_response = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CreateAuthChallenge" => {
                                create_auth_challenge = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "CustomMessage" => {
                                custom_message = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "DefineAuthChallenge" => {
                                define_auth_challenge = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "PostAuthentication" => {
                                post_authentication = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "PostConfirmation" => {
                                post_confirmation = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "PreAuthentication" => {
                                pre_authentication = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "PreSignUp" => {
                                pre_sign_up = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "VerifyAuthChallengeResponse" => {
                                verify_auth_challenge_response = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(LambdaConfig {
                        create_auth_challenge: create_auth_challenge,
                        custom_message: custom_message,
                        define_auth_challenge: define_auth_challenge,
                        post_authentication: post_authentication,
                        post_confirmation: post_confirmation,
                        pre_authentication: pre_authentication,
                        pre_sign_up: pre_sign_up,
                        verify_auth_challenge_response: verify_auth_challenge_response,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Cognito::UserPool.NumberAttributeConstraints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-numberattributeconstraints.html) property type.
    #[derive(Debug)]
    pub struct NumberAttributeConstraints {
        /// Property `MaxValue`.
        pub max_value: Option<::Value<String>>,
        /// Property `MinValue`.
        pub min_value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for NumberAttributeConstraints {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxValue", &self.max_value)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinValue", &self.min_value)?;
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
                    let mut max_value = None;
                    let mut min_value = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxValue" => {
                                max_value = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "MinValue" => {
                                min_value = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
    #[derive(Debug)]
    pub struct PasswordPolicy {
        /// Property `MinimumLength`.
        pub minimum_length: Option<::Value<u32>>,
        /// Property `RequireLowercase`.
        pub require_lowercase: Option<::Value<bool>>,
        /// Property `RequireNumbers`.
        pub require_numbers: Option<::Value<bool>>,
        /// Property `RequireSymbols`.
        pub require_symbols: Option<::Value<bool>>,
        /// Property `RequireUppercase`.
        pub require_uppercase: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for PasswordPolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinimumLength", &self.minimum_length)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequireLowercase", &self.require_lowercase)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequireNumbers", &self.require_numbers)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequireSymbols", &self.require_symbols)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RequireUppercase", &self.require_uppercase)?;
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
                    let mut minimum_length = None;
                    let mut require_lowercase = None;
                    let mut require_numbers = None;
                    let mut require_symbols = None;
                    let mut require_uppercase = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MinimumLength" => {
                                minimum_length = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RequireLowercase" => {
                                require_lowercase = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RequireNumbers" => {
                                require_numbers = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RequireSymbols" => {
                                require_symbols = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "RequireUppercase" => {
                                require_uppercase = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Cognito::UserPool.Policies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-policies.html) property type.
    #[derive(Debug)]
    pub struct Policies {
        /// Property `PasswordPolicy`.
        pub password_policy: Option<::Value<PasswordPolicy>>,
    }

    impl ::codec::SerializeValue for Policies {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PasswordPolicy", &self.password_policy)?;
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
                    let mut password_policy = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PasswordPolicy" => {
                                password_policy = Some(::serde::de::MapAccess::next_value(&mut map)?);
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

    /// The [`AWS::Cognito::UserPool.SchemaAttribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-schemaattribute.html) property type.
    #[derive(Debug)]
    pub struct SchemaAttribute {
        /// Property `AttributeDataType`.
        pub attribute_data_type: Option<::Value<String>>,
        /// Property `DeveloperOnlyAttribute`.
        pub developer_only_attribute: Option<::Value<bool>>,
        /// Property `Mutable`.
        pub mutable: Option<::Value<bool>>,
        /// Property `Name`.
        pub name: Option<::Value<String>>,
        /// Property `NumberAttributeConstraints`.
        pub number_attribute_constraints: Option<::Value<NumberAttributeConstraints>>,
        /// Property `Required`.
        pub required: Option<::Value<bool>>,
        /// Property `StringAttributeConstraints`.
        pub string_attribute_constraints: Option<::Value<StringAttributeConstraints>>,
    }

    impl ::codec::SerializeValue for SchemaAttribute {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttributeDataType", &self.attribute_data_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeveloperOnlyAttribute", &self.developer_only_attribute)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Mutable", &self.mutable)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumberAttributeConstraints", &self.number_attribute_constraints)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Required", &self.required)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StringAttributeConstraints", &self.string_attribute_constraints)?;
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
                    let mut attribute_data_type = None;
                    let mut developer_only_attribute = None;
                    let mut mutable = None;
                    let mut name = None;
                    let mut number_attribute_constraints = None;
                    let mut required = None;
                    let mut string_attribute_constraints = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AttributeDataType" => {
                                attribute_data_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "DeveloperOnlyAttribute" => {
                                developer_only_attribute = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Mutable" => {
                                mutable = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Name" => {
                                name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "NumberAttributeConstraints" => {
                                number_attribute_constraints = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Required" => {
                                required = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "StringAttributeConstraints" => {
                                string_attribute_constraints = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
    #[derive(Debug)]
    pub struct SmsConfiguration {
        /// Property `ExternalId`.
        pub external_id: Option<::Value<String>>,
        /// Property `SnsCallerArn`.
        pub sns_caller_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SmsConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExternalId", &self.external_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnsCallerArn", &self.sns_caller_arn)?;
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
                    let mut external_id = None;
                    let mut sns_caller_arn = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ExternalId" => {
                                external_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "SnsCallerArn" => {
                                sns_caller_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
    #[derive(Debug)]
    pub struct StringAttributeConstraints {
        /// Property `MaxLength`.
        pub max_length: Option<::Value<String>>,
        /// Property `MinLength`.
        pub min_length: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for StringAttributeConstraints {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxLength", &self.max_length)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinLength", &self.min_length)?;
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
                    let mut max_length = None;
                    let mut min_length = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxLength" => {
                                max_length = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "MinLength" => {
                                min_length = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
}

pub mod user_pool_user {
    //! Property types for the `UserPoolUser` resource.

    /// The [`AWS::Cognito::UserPoolUser.AttributeType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpooluser-attributetype.html) property type.
    #[derive(Debug)]
    pub struct AttributeType {
        /// Property `Name`.
        pub name: Option<::Value<String>>,
        /// Property `Value`.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AttributeType {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
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
                    let mut name = None;
                    let mut value = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Value" => {
                                value = Some(::serde::de::MapAccess::next_value(&mut map)?);
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
