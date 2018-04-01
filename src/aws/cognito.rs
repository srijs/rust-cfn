//! Types for the `Cognito` service.

/// The [`AWS::Cognito::IdentityPool`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-identitypool.html) resource type.
#[derive(Debug)]
pub struct IdentityPool {
    properties: IdentityPoolProperties
}

/// Properties for the `IdentityPool` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityPoolProperties {
    /// Property `AllowUnauthenticatedIdentities`.
    #[serde(rename = "AllowUnauthenticatedIdentities")]
    pub allow_unauthenticated_identities: ::Value<bool>,
    /// Property `CognitoEvents`.
    #[serde(rename = "CognitoEvents")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cognito_events: Option<::Value<::json::Value>>,
    /// Property `CognitoIdentityProviders`.
    #[serde(rename = "CognitoIdentityProviders")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cognito_identity_providers: Option<::ValueList<self::identity_pool::CognitoIdentityProvider>>,
    /// Property `CognitoStreams`.
    #[serde(rename = "CognitoStreams")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cognito_streams: Option<::Value<self::identity_pool::CognitoStreams>>,
    /// Property `DeveloperProviderName`.
    #[serde(rename = "DeveloperProviderName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub developer_provider_name: Option<::Value<String>>,
    /// Property `IdentityPoolName`.
    #[serde(rename = "IdentityPoolName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity_pool_name: Option<::Value<String>>,
    /// Property `OpenIdConnectProviderARNs`.
    #[serde(rename = "OpenIdConnectProviderARNs")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_id_connect_provider_ar_ns: Option<::ValueList<String>>,
    /// Property `PushSync`.
    #[serde(rename = "PushSync")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub push_sync: Option<::Value<self::identity_pool::PushSync>>,
    /// Property `SamlProviderARNs`.
    #[serde(rename = "SamlProviderARNs")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub saml_provider_ar_ns: Option<::ValueList<String>>,
    /// Property `SupportedLoginProviders`.
    #[serde(rename = "SupportedLoginProviders")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supported_login_providers: Option<::Value<::json::Value>>,
}

impl<'a> ::Resource<'a> for IdentityPool {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityPoolRoleAttachmentProperties {
    /// Property `IdentityPoolId`.
    #[serde(rename = "IdentityPoolId")]
    pub identity_pool_id: ::Value<String>,
    /// Property `RoleMappings`.
    #[serde(rename = "RoleMappings")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_mappings: Option<::Value<::json::Value>>,
    /// Property `Roles`.
    #[serde(rename = "Roles")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<::Value<::json::Value>>,
}

impl<'a> ::Resource<'a> for IdentityPoolRoleAttachment {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct UserPoolProperties {
    /// Property `AdminCreateUserConfig`.
    #[serde(rename = "AdminCreateUserConfig")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admin_create_user_config: Option<::Value<self::user_pool::AdminCreateUserConfig>>,
    /// Property `AliasAttributes`.
    #[serde(rename = "AliasAttributes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias_attributes: Option<::ValueList<String>>,
    /// Property `AutoVerifiedAttributes`.
    #[serde(rename = "AutoVerifiedAttributes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_verified_attributes: Option<::ValueList<String>>,
    /// Property `DeviceConfiguration`.
    #[serde(rename = "DeviceConfiguration")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_configuration: Option<::Value<self::user_pool::DeviceConfiguration>>,
    /// Property `EmailConfiguration`.
    #[serde(rename = "EmailConfiguration")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_configuration: Option<::Value<self::user_pool::EmailConfiguration>>,
    /// Property `EmailVerificationMessage`.
    #[serde(rename = "EmailVerificationMessage")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_verification_message: Option<::Value<String>>,
    /// Property `EmailVerificationSubject`.
    #[serde(rename = "EmailVerificationSubject")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_verification_subject: Option<::Value<String>>,
    /// Property `LambdaConfig`.
    #[serde(rename = "LambdaConfig")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lambda_config: Option<::Value<self::user_pool::LambdaConfig>>,
    /// Property `MfaConfiguration`.
    #[serde(rename = "MfaConfiguration")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mfa_configuration: Option<::Value<String>>,
    /// Property `Policies`.
    #[serde(rename = "Policies")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policies: Option<::Value<self::user_pool::Policies>>,
    /// Property `Schema`.
    #[serde(rename = "Schema")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<::ValueList<self::user_pool::SchemaAttribute>>,
    /// Property `SmsAuthenticationMessage`.
    #[serde(rename = "SmsAuthenticationMessage")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sms_authentication_message: Option<::Value<String>>,
    /// Property `SmsConfiguration`.
    #[serde(rename = "SmsConfiguration")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sms_configuration: Option<::Value<self::user_pool::SmsConfiguration>>,
    /// Property `SmsVerificationMessage`.
    #[serde(rename = "SmsVerificationMessage")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sms_verification_message: Option<::Value<String>>,
    /// Property `UserPoolName`.
    #[serde(rename = "UserPoolName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_pool_name: Option<::Value<String>>,
    /// Property `UserPoolTags`.
    #[serde(rename = "UserPoolTags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_pool_tags: Option<::Value<::json::Value>>,
}

impl<'a> ::Resource<'a> for UserPool {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct UserPoolClientProperties {
    /// Property `ClientName`.
    #[serde(rename = "ClientName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_name: Option<::Value<String>>,
    /// Property `ExplicitAuthFlows`.
    #[serde(rename = "ExplicitAuthFlows")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explicit_auth_flows: Option<::ValueList<String>>,
    /// Property `GenerateSecret`.
    #[serde(rename = "GenerateSecret")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generate_secret: Option<::Value<bool>>,
    /// Property `ReadAttributes`.
    #[serde(rename = "ReadAttributes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_attributes: Option<::ValueList<String>>,
    /// Property `RefreshTokenValidity`.
    #[serde(rename = "RefreshTokenValidity")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_token_validity: Option<::Value<f64>>,
    /// Property `UserPoolId`.
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: ::Value<String>,
    /// Property `WriteAttributes`.
    #[serde(rename = "WriteAttributes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub write_attributes: Option<::ValueList<String>>,
}

impl<'a> ::Resource<'a> for UserPoolClient {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct UserPoolGroupProperties {
    /// Property `Description`.
    #[serde(rename = "Description")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<::Value<String>>,
    /// Property `GroupName`.
    #[serde(rename = "GroupName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_name: Option<::Value<String>>,
    /// Property `Precedence`.
    #[serde(rename = "Precedence")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub precedence: Option<::Value<f64>>,
    /// Property `RoleArn`.
    #[serde(rename = "RoleArn")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<::Value<String>>,
    /// Property `UserPoolId`.
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: ::Value<String>,
}

impl<'a> ::Resource<'a> for UserPoolGroup {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct UserPoolUserProperties {
    /// Property `DesiredDeliveryMediums`.
    #[serde(rename = "DesiredDeliveryMediums")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desired_delivery_mediums: Option<::ValueList<String>>,
    /// Property `ForceAliasCreation`.
    #[serde(rename = "ForceAliasCreation")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub force_alias_creation: Option<::Value<bool>>,
    /// Property `MessageAction`.
    #[serde(rename = "MessageAction")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_action: Option<::Value<String>>,
    /// Property `UserAttributes`.
    #[serde(rename = "UserAttributes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_attributes: Option<::ValueList<self::user_pool_user::AttributeType>>,
    /// Property `UserPoolId`.
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: ::Value<String>,
    /// Property `Username`.
    #[serde(rename = "Username")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<::Value<String>>,
    /// Property `ValidationData`.
    #[serde(rename = "ValidationData")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validation_data: Option<::ValueList<self::user_pool_user::AttributeType>>,
}

impl<'a> ::Resource<'a> for UserPoolUser {
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
#[derive(Debug, Serialize, Deserialize)]
pub struct UserPoolUserToGroupAttachmentProperties {
    /// Property `GroupName`.
    #[serde(rename = "GroupName")]
    pub group_name: ::Value<String>,
    /// Property `UserPoolId`.
    #[serde(rename = "UserPoolId")]
    pub user_pool_id: ::Value<String>,
    /// Property `Username`.
    #[serde(rename = "Username")]
    pub username: ::Value<String>,
}

impl<'a> ::Resource<'a> for UserPoolUserToGroupAttachment {
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
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CognitoIdentityProvider {
        /// Property `ClientId`.
        #[serde(rename = "ClientId")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub client_id: Option<::Value<String>>,
        /// Property `ProviderName`.
        #[serde(rename = "ProviderName")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider_name: Option<::Value<String>>,
        /// Property `ServerSideTokenCheck`.
        #[serde(rename = "ServerSideTokenCheck")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub server_side_token_check: Option<::Value<bool>>,
    }

    cfn_internal__inherit_codec_impls!(CognitoIdentityProvider);

    /// The [`AWS::Cognito::IdentityPool.CognitoStreams`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypool-cognitostreams.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CognitoStreams {
        /// Property `RoleArn`.
        #[serde(rename = "RoleArn")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub role_arn: Option<::Value<String>>,
        /// Property `StreamName`.
        #[serde(rename = "StreamName")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub stream_name: Option<::Value<String>>,
        /// Property `StreamingStatus`.
        #[serde(rename = "StreamingStatus")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub streaming_status: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(CognitoStreams);

    /// The [`AWS::Cognito::IdentityPool.PushSync`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypool-pushsync.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PushSync {
        /// Property `ApplicationArns`.
        #[serde(rename = "ApplicationArns")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub application_arns: Option<::ValueList<String>>,
        /// Property `RoleArn`.
        #[serde(rename = "RoleArn")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub role_arn: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(PushSync);
}

pub mod identity_pool_role_attachment {
    //! Property types for the `IdentityPoolRoleAttachment` resource.

    /// The [`AWS::Cognito::IdentityPoolRoleAttachment.MappingRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypoolroleattachment-mappingrule.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MappingRule {
        /// Property `Claim`.
        #[serde(rename = "Claim")]
        pub claim: ::Value<String>,
        /// Property `MatchType`.
        #[serde(rename = "MatchType")]
        pub match_type: ::Value<String>,
        /// Property `RoleARN`.
        #[serde(rename = "RoleARN")]
        pub role_arn: ::Value<String>,
        /// Property `Value`.
        #[serde(rename = "Value")]
        pub value: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(MappingRule);

    /// The [`AWS::Cognito::IdentityPoolRoleAttachment.RoleMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypoolroleattachment-rolemapping.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RoleMapping {
        /// Property `AmbiguousRoleResolution`.
        #[serde(rename = "AmbiguousRoleResolution")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub ambiguous_role_resolution: Option<::Value<String>>,
        /// Property `RulesConfiguration`.
        #[serde(rename = "RulesConfiguration")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub rules_configuration: Option<::Value<RulesConfigurationType>>,
        /// Property `Type`.
        #[serde(rename = "Type")]
        pub type_: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(RoleMapping);

    /// The [`AWS::Cognito::IdentityPoolRoleAttachment.RulesConfigurationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypoolroleattachment-rulesconfigurationtype.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RulesConfigurationType {
        /// Property `Rules`.
        #[serde(rename = "Rules")]
        pub rules: ::ValueList<MappingRule>,
    }

    cfn_internal__inherit_codec_impls!(RulesConfigurationType);
}

pub mod user_pool {
    //! Property types for the `UserPool` resource.

    /// The [`AWS::Cognito::UserPool.AdminCreateUserConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-admincreateuserconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AdminCreateUserConfig {
        /// Property `AllowAdminCreateUserOnly`.
        #[serde(rename = "AllowAdminCreateUserOnly")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub allow_admin_create_user_only: Option<::Value<bool>>,
        /// Property `InviteMessageTemplate`.
        #[serde(rename = "InviteMessageTemplate")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub invite_message_template: Option<::Value<InviteMessageTemplate>>,
        /// Property `UnusedAccountValidityDays`.
        #[serde(rename = "UnusedAccountValidityDays")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub unused_account_validity_days: Option<::Value<f64>>,
    }

    cfn_internal__inherit_codec_impls!(AdminCreateUserConfig);

    /// The [`AWS::Cognito::UserPool.DeviceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-deviceconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DeviceConfiguration {
        /// Property `ChallengeRequiredOnNewDevice`.
        #[serde(rename = "ChallengeRequiredOnNewDevice")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub challenge_required_on_new_device: Option<::Value<bool>>,
        /// Property `DeviceOnlyRememberedOnUserPrompt`.
        #[serde(rename = "DeviceOnlyRememberedOnUserPrompt")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub device_only_remembered_on_user_prompt: Option<::Value<bool>>,
    }

    cfn_internal__inherit_codec_impls!(DeviceConfiguration);

    /// The [`AWS::Cognito::UserPool.EmailConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-emailconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EmailConfiguration {
        /// Property `ReplyToEmailAddress`.
        #[serde(rename = "ReplyToEmailAddress")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub reply_to_email_address: Option<::Value<String>>,
        /// Property `SourceArn`.
        #[serde(rename = "SourceArn")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub source_arn: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(EmailConfiguration);

    /// The [`AWS::Cognito::UserPool.InviteMessageTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-invitemessagetemplate.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct InviteMessageTemplate {
        /// Property `EmailMessage`.
        #[serde(rename = "EmailMessage")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub email_message: Option<::Value<String>>,
        /// Property `EmailSubject`.
        #[serde(rename = "EmailSubject")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub email_subject: Option<::Value<String>>,
        /// Property `SMSMessage`.
        #[serde(rename = "SMSMessage")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sms_message: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(InviteMessageTemplate);

    /// The [`AWS::Cognito::UserPool.LambdaConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-lambdaconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LambdaConfig {
        /// Property `CreateAuthChallenge`.
        #[serde(rename = "CreateAuthChallenge")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub create_auth_challenge: Option<::Value<String>>,
        /// Property `CustomMessage`.
        #[serde(rename = "CustomMessage")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub custom_message: Option<::Value<String>>,
        /// Property `DefineAuthChallenge`.
        #[serde(rename = "DefineAuthChallenge")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub define_auth_challenge: Option<::Value<String>>,
        /// Property `PostAuthentication`.
        #[serde(rename = "PostAuthentication")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub post_authentication: Option<::Value<String>>,
        /// Property `PostConfirmation`.
        #[serde(rename = "PostConfirmation")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub post_confirmation: Option<::Value<String>>,
        /// Property `PreAuthentication`.
        #[serde(rename = "PreAuthentication")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub pre_authentication: Option<::Value<String>>,
        /// Property `PreSignUp`.
        #[serde(rename = "PreSignUp")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub pre_sign_up: Option<::Value<String>>,
        /// Property `VerifyAuthChallengeResponse`.
        #[serde(rename = "VerifyAuthChallengeResponse")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub verify_auth_challenge_response: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(LambdaConfig);

    /// The [`AWS::Cognito::UserPool.NumberAttributeConstraints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-numberattributeconstraints.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct NumberAttributeConstraints {
        /// Property `MaxValue`.
        #[serde(rename = "MaxValue")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub max_value: Option<::Value<String>>,
        /// Property `MinValue`.
        #[serde(rename = "MinValue")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub min_value: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(NumberAttributeConstraints);

    /// The [`AWS::Cognito::UserPool.PasswordPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-passwordpolicy.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PasswordPolicy {
        /// Property `MinimumLength`.
        #[serde(rename = "MinimumLength")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub minimum_length: Option<::Value<u32>>,
        /// Property `RequireLowercase`.
        #[serde(rename = "RequireLowercase")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub require_lowercase: Option<::Value<bool>>,
        /// Property `RequireNumbers`.
        #[serde(rename = "RequireNumbers")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub require_numbers: Option<::Value<bool>>,
        /// Property `RequireSymbols`.
        #[serde(rename = "RequireSymbols")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub require_symbols: Option<::Value<bool>>,
        /// Property `RequireUppercase`.
        #[serde(rename = "RequireUppercase")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub require_uppercase: Option<::Value<bool>>,
    }

    cfn_internal__inherit_codec_impls!(PasswordPolicy);

    /// The [`AWS::Cognito::UserPool.Policies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-policies.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Policies {
        /// Property `PasswordPolicy`.
        #[serde(rename = "PasswordPolicy")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub password_policy: Option<::Value<PasswordPolicy>>,
    }

    cfn_internal__inherit_codec_impls!(Policies);

    /// The [`AWS::Cognito::UserPool.SchemaAttribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-schemaattribute.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SchemaAttribute {
        /// Property `AttributeDataType`.
        #[serde(rename = "AttributeDataType")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub attribute_data_type: Option<::Value<String>>,
        /// Property `DeveloperOnlyAttribute`.
        #[serde(rename = "DeveloperOnlyAttribute")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub developer_only_attribute: Option<::Value<bool>>,
        /// Property `Mutable`.
        #[serde(rename = "Mutable")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub mutable: Option<::Value<bool>>,
        /// Property `Name`.
        #[serde(rename = "Name")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<::Value<String>>,
        /// Property `NumberAttributeConstraints`.
        #[serde(rename = "NumberAttributeConstraints")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub number_attribute_constraints: Option<::Value<NumberAttributeConstraints>>,
        /// Property `Required`.
        #[serde(rename = "Required")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub required: Option<::Value<bool>>,
        /// Property `StringAttributeConstraints`.
        #[serde(rename = "StringAttributeConstraints")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub string_attribute_constraints: Option<::Value<StringAttributeConstraints>>,
    }

    cfn_internal__inherit_codec_impls!(SchemaAttribute);

    /// The [`AWS::Cognito::UserPool.SmsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-smsconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SmsConfiguration {
        /// Property `ExternalId`.
        #[serde(rename = "ExternalId")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub external_id: Option<::Value<String>>,
        /// Property `SnsCallerArn`.
        #[serde(rename = "SnsCallerArn")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sns_caller_arn: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(SmsConfiguration);

    /// The [`AWS::Cognito::UserPool.StringAttributeConstraints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-stringattributeconstraints.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StringAttributeConstraints {
        /// Property `MaxLength`.
        #[serde(rename = "MaxLength")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub max_length: Option<::Value<String>>,
        /// Property `MinLength`.
        #[serde(rename = "MinLength")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub min_length: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(StringAttributeConstraints);
}

pub mod user_pool_user {
    //! Property types for the `UserPoolUser` resource.

    /// The [`AWS::Cognito::UserPoolUser.AttributeType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpooluser-attributetype.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AttributeType {
        /// Property `Name`.
        #[serde(rename = "Name")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<::Value<String>>,
        /// Property `Value`.
        #[serde(rename = "Value")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub value: Option<::Value<String>>,
    }

    cfn_internal__inherit_codec_impls!(AttributeType);
}
