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
    #[serde(rename="AllowUnauthenticatedIdentities")]
    pub allow_unauthenticated_identities: bool,
    /// Property `CognitoEvents`.
    #[serde(rename="CognitoEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cognito_events: Option<::json::Value>,
    /// Property `CognitoIdentityProviders`.
    #[serde(rename="CognitoIdentityProviders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cognito_identity_providers: Option<Vec<self::identity_pool::CognitoIdentityProvider>>,
    /// Property `CognitoStreams`.
    #[serde(rename="CognitoStreams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cognito_streams: Option<self::identity_pool::CognitoStreams>,
    /// Property `DeveloperProviderName`.
    #[serde(rename="DeveloperProviderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub developer_provider_name: Option<String>,
    /// Property `IdentityPoolName`.
    #[serde(rename="IdentityPoolName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_pool_name: Option<String>,
    /// Property `OpenIdConnectProviderARNs`.
    #[serde(rename="OpenIdConnectProviderARNs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id_connect_provider_ar_ns: Option<Vec<String>>,
    /// Property `PushSync`.
    #[serde(rename="PushSync")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_sync: Option<self::identity_pool::PushSync>,
    /// Property `SamlProviderARNs`.
    #[serde(rename="SamlProviderARNs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saml_provider_ar_ns: Option<Vec<String>>,
    /// Property `SupportedLoginProviders`.
    #[serde(rename="SupportedLoginProviders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_login_providers: Option<::json::Value>,
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
    #[serde(rename="IdentityPoolId")]
    pub identity_pool_id: String,
    /// Property `RoleMappings`.
    #[serde(rename="RoleMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_mappings: Option<::json::Value>,
    /// Property `Roles`.
    #[serde(rename="Roles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<::json::Value>,
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
    #[serde(rename="AdminCreateUserConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_create_user_config: Option<self::user_pool::AdminCreateUserConfig>,
    /// Property `AliasAttributes`.
    #[serde(rename="AliasAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_attributes: Option<Vec<String>>,
    /// Property `AutoVerifiedAttributes`.
    #[serde(rename="AutoVerifiedAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_verified_attributes: Option<Vec<String>>,
    /// Property `DeviceConfiguration`.
    #[serde(rename="DeviceConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_configuration: Option<self::user_pool::DeviceConfiguration>,
    /// Property `EmailConfiguration`.
    #[serde(rename="EmailConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_configuration: Option<self::user_pool::EmailConfiguration>,
    /// Property `EmailVerificationMessage`.
    #[serde(rename="EmailVerificationMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verification_message: Option<String>,
    /// Property `EmailVerificationSubject`.
    #[serde(rename="EmailVerificationSubject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verification_subject: Option<String>,
    /// Property `LambdaConfig`.
    #[serde(rename="LambdaConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_config: Option<self::user_pool::LambdaConfig>,
    /// Property `MfaConfiguration`.
    #[serde(rename="MfaConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_configuration: Option<String>,
    /// Property `Policies`.
    #[serde(rename="Policies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<self::user_pool::Policies>,
    /// Property `Schema`.
    #[serde(rename="Schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Vec<self::user_pool::SchemaAttribute>>,
    /// Property `SmsAuthenticationMessage`.
    #[serde(rename="SmsAuthenticationMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_authentication_message: Option<String>,
    /// Property `SmsConfiguration`.
    #[serde(rename="SmsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_configuration: Option<self::user_pool::SmsConfiguration>,
    /// Property `SmsVerificationMessage`.
    #[serde(rename="SmsVerificationMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_verification_message: Option<String>,
    /// Property `UserPoolName`.
    #[serde(rename="UserPoolName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_name: Option<String>,
    /// Property `UserPoolTags`.
    #[serde(rename="UserPoolTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_tags: Option<::json::Value>,
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
    #[serde(rename="ClientName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_name: Option<String>,
    /// Property `ExplicitAuthFlows`.
    #[serde(rename="ExplicitAuthFlows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_auth_flows: Option<Vec<String>>,
    /// Property `GenerateSecret`.
    #[serde(rename="GenerateSecret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_secret: Option<bool>,
    /// Property `ReadAttributes`.
    #[serde(rename="ReadAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_attributes: Option<Vec<String>>,
    /// Property `RefreshTokenValidity`.
    #[serde(rename="RefreshTokenValidity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token_validity: Option<f64>,
    /// Property `UserPoolId`.
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
    /// Property `WriteAttributes`.
    #[serde(rename="WriteAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_attributes: Option<Vec<String>>,
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
    #[serde(rename="Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Property `GroupName`.
    #[serde(rename="GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// Property `Precedence`.
    #[serde(rename="Precedence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precedence: Option<f64>,
    /// Property `RoleArn`.
    #[serde(rename="RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// Property `UserPoolId`.
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
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
    #[serde(rename="DesiredDeliveryMediums")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_delivery_mediums: Option<Vec<String>>,
    /// Property `ForceAliasCreation`.
    #[serde(rename="ForceAliasCreation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_alias_creation: Option<bool>,
    /// Property `MessageAction`.
    #[serde(rename="MessageAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_action: Option<String>,
    /// Property `UserAttributes`.
    #[serde(rename="UserAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_attributes: Option<Vec<self::user_pool_user::AttributeType>>,
    /// Property `UserPoolId`.
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
    /// Property `Username`.
    #[serde(rename="Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Property `ValidationData`.
    #[serde(rename="ValidationData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_data: Option<Vec<self::user_pool_user::AttributeType>>,
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
    #[serde(rename="GroupName")]
    pub group_name: String,
    /// Property `UserPoolId`.
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
    /// Property `Username`.
    #[serde(rename="Username")]
    pub username: String,
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
        #[serde(rename="ClientId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub client_id: Option<String>,
        /// Property `ProviderName`.
        #[serde(rename="ProviderName")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub provider_name: Option<String>,
        /// Property `ServerSideTokenCheck`.
        #[serde(rename="ServerSideTokenCheck")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub server_side_token_check: Option<bool>,
    }

    /// The [`AWS::Cognito::IdentityPool.CognitoStreams`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypool-cognitostreams.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CognitoStreams {
        /// Property `RoleArn`.
        #[serde(rename="RoleArn")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub role_arn: Option<String>,
        /// Property `StreamName`.
        #[serde(rename="StreamName")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub stream_name: Option<String>,
        /// Property `StreamingStatus`.
        #[serde(rename="StreamingStatus")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub streaming_status: Option<String>,
    }

    /// The [`AWS::Cognito::IdentityPool.PushSync`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypool-pushsync.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PushSync {
        /// Property `ApplicationArns`.
        #[serde(rename="ApplicationArns")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub application_arns: Option<Vec<String>>,
        /// Property `RoleArn`.
        #[serde(rename="RoleArn")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub role_arn: Option<String>,
    }
}

pub mod identity_pool_role_attachment {
    //! Property types for the `IdentityPoolRoleAttachment` resource.

    /// The [`AWS::Cognito::IdentityPoolRoleAttachment.MappingRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypoolroleattachment-mappingrule.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct MappingRule {
        /// Property `Claim`.
        #[serde(rename="Claim")]
        pub claim: String,
        /// Property `MatchType`.
        #[serde(rename="MatchType")]
        pub match_type: String,
        /// Property `RoleARN`.
        #[serde(rename="RoleARN")]
        pub role_arn: String,
        /// Property `Value`.
        #[serde(rename="Value")]
        pub value: String,
    }

    /// The [`AWS::Cognito::IdentityPoolRoleAttachment.RoleMapping`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypoolroleattachment-rolemapping.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RoleMapping {
        /// Property `AmbiguousRoleResolution`.
        #[serde(rename="AmbiguousRoleResolution")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub ambiguous_role_resolution: Option<String>,
        /// Property `RulesConfiguration`.
        #[serde(rename="RulesConfiguration")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub rules_configuration: Option<RulesConfigurationType>,
        /// Property `Type`.
        #[serde(rename="Type")]
        pub type_: String,
    }

    /// The [`AWS::Cognito::IdentityPoolRoleAttachment.RulesConfigurationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypoolroleattachment-rulesconfigurationtype.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RulesConfigurationType {
        /// Property `Rules`.
        #[serde(rename="Rules")]
        pub rules: Vec<MappingRule>,
    }
}

pub mod user_pool {
    //! Property types for the `UserPool` resource.

    /// The [`AWS::Cognito::UserPool.AdminCreateUserConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-admincreateuserconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AdminCreateUserConfig {
        /// Property `AllowAdminCreateUserOnly`.
        #[serde(rename="AllowAdminCreateUserOnly")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub allow_admin_create_user_only: Option<bool>,
        /// Property `InviteMessageTemplate`.
        #[serde(rename="InviteMessageTemplate")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub invite_message_template: Option<InviteMessageTemplate>,
        /// Property `UnusedAccountValidityDays`.
        #[serde(rename="UnusedAccountValidityDays")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub unused_account_validity_days: Option<f64>,
    }

    /// The [`AWS::Cognito::UserPool.DeviceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-deviceconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DeviceConfiguration {
        /// Property `ChallengeRequiredOnNewDevice`.
        #[serde(rename="ChallengeRequiredOnNewDevice")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub challenge_required_on_new_device: Option<bool>,
        /// Property `DeviceOnlyRememberedOnUserPrompt`.
        #[serde(rename="DeviceOnlyRememberedOnUserPrompt")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub device_only_remembered_on_user_prompt: Option<bool>,
    }

    /// The [`AWS::Cognito::UserPool.EmailConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-emailconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EmailConfiguration {
        /// Property `ReplyToEmailAddress`.
        #[serde(rename="ReplyToEmailAddress")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub reply_to_email_address: Option<String>,
        /// Property `SourceArn`.
        #[serde(rename="SourceArn")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub source_arn: Option<String>,
    }

    /// The [`AWS::Cognito::UserPool.InviteMessageTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-invitemessagetemplate.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct InviteMessageTemplate {
        /// Property `EmailMessage`.
        #[serde(rename="EmailMessage")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub email_message: Option<String>,
        /// Property `EmailSubject`.
        #[serde(rename="EmailSubject")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub email_subject: Option<String>,
        /// Property `SMSMessage`.
        #[serde(rename="SMSMessage")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sms_message: Option<String>,
    }

    /// The [`AWS::Cognito::UserPool.LambdaConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-lambdaconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LambdaConfig {
        /// Property `CreateAuthChallenge`.
        #[serde(rename="CreateAuthChallenge")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub create_auth_challenge: Option<String>,
        /// Property `CustomMessage`.
        #[serde(rename="CustomMessage")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub custom_message: Option<String>,
        /// Property `DefineAuthChallenge`.
        #[serde(rename="DefineAuthChallenge")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub define_auth_challenge: Option<String>,
        /// Property `PostAuthentication`.
        #[serde(rename="PostAuthentication")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub post_authentication: Option<String>,
        /// Property `PostConfirmation`.
        #[serde(rename="PostConfirmation")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub post_confirmation: Option<String>,
        /// Property `PreAuthentication`.
        #[serde(rename="PreAuthentication")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub pre_authentication: Option<String>,
        /// Property `PreSignUp`.
        #[serde(rename="PreSignUp")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub pre_sign_up: Option<String>,
        /// Property `VerifyAuthChallengeResponse`.
        #[serde(rename="VerifyAuthChallengeResponse")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub verify_auth_challenge_response: Option<String>,
    }

    /// The [`AWS::Cognito::UserPool.NumberAttributeConstraints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-numberattributeconstraints.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct NumberAttributeConstraints {
        /// Property `MaxValue`.
        #[serde(rename="MaxValue")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub max_value: Option<String>,
        /// Property `MinValue`.
        #[serde(rename="MinValue")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub min_value: Option<String>,
    }

    /// The [`AWS::Cognito::UserPool.PasswordPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-passwordpolicy.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PasswordPolicy {
        /// Property `MinimumLength`.
        #[serde(rename="MinimumLength")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub minimum_length: Option<u32>,
        /// Property `RequireLowercase`.
        #[serde(rename="RequireLowercase")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub require_lowercase: Option<bool>,
        /// Property `RequireNumbers`.
        #[serde(rename="RequireNumbers")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub require_numbers: Option<bool>,
        /// Property `RequireSymbols`.
        #[serde(rename="RequireSymbols")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub require_symbols: Option<bool>,
        /// Property `RequireUppercase`.
        #[serde(rename="RequireUppercase")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub require_uppercase: Option<bool>,
    }

    /// The [`AWS::Cognito::UserPool.Policies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-policies.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Policies {
        /// Property `PasswordPolicy`.
        #[serde(rename="PasswordPolicy")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub password_policy: Option<PasswordPolicy>,
    }

    /// The [`AWS::Cognito::UserPool.SchemaAttribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-schemaattribute.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SchemaAttribute {
        /// Property `AttributeDataType`.
        #[serde(rename="AttributeDataType")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub attribute_data_type: Option<String>,
        /// Property `DeveloperOnlyAttribute`.
        #[serde(rename="DeveloperOnlyAttribute")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub developer_only_attribute: Option<bool>,
        /// Property `Mutable`.
        #[serde(rename="Mutable")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub mutable: Option<bool>,
        /// Property `Name`.
        #[serde(rename="Name")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        /// Property `NumberAttributeConstraints`.
        #[serde(rename="NumberAttributeConstraints")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub number_attribute_constraints: Option<NumberAttributeConstraints>,
        /// Property `Required`.
        #[serde(rename="Required")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub required: Option<bool>,
        /// Property `StringAttributeConstraints`.
        #[serde(rename="StringAttributeConstraints")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub string_attribute_constraints: Option<StringAttributeConstraints>,
    }

    /// The [`AWS::Cognito::UserPool.SmsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-smsconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SmsConfiguration {
        /// Property `ExternalId`.
        #[serde(rename="ExternalId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub external_id: Option<String>,
        /// Property `SnsCallerArn`.
        #[serde(rename="SnsCallerArn")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sns_caller_arn: Option<String>,
    }

    /// The [`AWS::Cognito::UserPool.StringAttributeConstraints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-stringattributeconstraints.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StringAttributeConstraints {
        /// Property `MaxLength`.
        #[serde(rename="MaxLength")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub max_length: Option<String>,
        /// Property `MinLength`.
        #[serde(rename="MinLength")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub min_length: Option<String>,
    }
}

pub mod user_pool_user {
    //! Property types for the `UserPoolUser` resource.

    /// The [`AWS::Cognito::UserPoolUser.AttributeType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpooluser-attributetype.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AttributeType {
        /// Property `Name`.
        #[serde(rename="Name")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        /// Property `Value`.
        #[serde(rename="Value")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }
}
