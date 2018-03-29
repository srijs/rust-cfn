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
    pub cognito_events: ::json::Value,
    /// Property `CognitoIdentityProviders`.
    #[serde(rename="CognitoIdentityProviders")]
    pub cognito_identity_providers: Vec<self::identity_pool::CognitoIdentityProvider>,
    /// Property `CognitoStreams`.
    #[serde(rename="CognitoStreams")]
    pub cognito_streams: self::identity_pool::CognitoStreams,
    /// Property `DeveloperProviderName`.
    #[serde(rename="DeveloperProviderName")]
    pub developer_provider_name: String,
    /// Property `IdentityPoolName`.
    #[serde(rename="IdentityPoolName")]
    pub identity_pool_name: String,
    /// Property `OpenIdConnectProviderARNs`.
    #[serde(rename="OpenIdConnectProviderARNs")]
    pub open_id_connect_provider_ar_ns: Vec<String>,
    /// Property `PushSync`.
    #[serde(rename="PushSync")]
    pub push_sync: self::identity_pool::PushSync,
    /// Property `SamlProviderARNs`.
    #[serde(rename="SamlProviderARNs")]
    pub saml_provider_ar_ns: Vec<String>,
    /// Property `SupportedLoginProviders`.
    #[serde(rename="SupportedLoginProviders")]
    pub supported_login_providers: ::json::Value,
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
    pub role_mappings: ::json::Value,
    /// Property `Roles`.
    #[serde(rename="Roles")]
    pub roles: ::json::Value,
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
    pub admin_create_user_config: self::user_pool::AdminCreateUserConfig,
    /// Property `AliasAttributes`.
    #[serde(rename="AliasAttributes")]
    pub alias_attributes: Vec<String>,
    /// Property `AutoVerifiedAttributes`.
    #[serde(rename="AutoVerifiedAttributes")]
    pub auto_verified_attributes: Vec<String>,
    /// Property `DeviceConfiguration`.
    #[serde(rename="DeviceConfiguration")]
    pub device_configuration: self::user_pool::DeviceConfiguration,
    /// Property `EmailConfiguration`.
    #[serde(rename="EmailConfiguration")]
    pub email_configuration: self::user_pool::EmailConfiguration,
    /// Property `EmailVerificationMessage`.
    #[serde(rename="EmailVerificationMessage")]
    pub email_verification_message: String,
    /// Property `EmailVerificationSubject`.
    #[serde(rename="EmailVerificationSubject")]
    pub email_verification_subject: String,
    /// Property `LambdaConfig`.
    #[serde(rename="LambdaConfig")]
    pub lambda_config: self::user_pool::LambdaConfig,
    /// Property `MfaConfiguration`.
    #[serde(rename="MfaConfiguration")]
    pub mfa_configuration: String,
    /// Property `Policies`.
    #[serde(rename="Policies")]
    pub policies: self::user_pool::Policies,
    /// Property `Schema`.
    #[serde(rename="Schema")]
    pub schema: Vec<self::user_pool::SchemaAttribute>,
    /// Property `SmsAuthenticationMessage`.
    #[serde(rename="SmsAuthenticationMessage")]
    pub sms_authentication_message: String,
    /// Property `SmsConfiguration`.
    #[serde(rename="SmsConfiguration")]
    pub sms_configuration: self::user_pool::SmsConfiguration,
    /// Property `SmsVerificationMessage`.
    #[serde(rename="SmsVerificationMessage")]
    pub sms_verification_message: String,
    /// Property `UserPoolName`.
    #[serde(rename="UserPoolName")]
    pub user_pool_name: String,
    /// Property `UserPoolTags`.
    #[serde(rename="UserPoolTags")]
    pub user_pool_tags: ::json::Value,
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
    pub client_name: String,
    /// Property `ExplicitAuthFlows`.
    #[serde(rename="ExplicitAuthFlows")]
    pub explicit_auth_flows: Vec<String>,
    /// Property `GenerateSecret`.
    #[serde(rename="GenerateSecret")]
    pub generate_secret: bool,
    /// Property `ReadAttributes`.
    #[serde(rename="ReadAttributes")]
    pub read_attributes: Vec<String>,
    /// Property `RefreshTokenValidity`.
    #[serde(rename="RefreshTokenValidity")]
    pub refresh_token_validity: f64,
    /// Property `UserPoolId`.
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
    /// Property `WriteAttributes`.
    #[serde(rename="WriteAttributes")]
    pub write_attributes: Vec<String>,
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
    pub description: String,
    /// Property `GroupName`.
    #[serde(rename="GroupName")]
    pub group_name: String,
    /// Property `Precedence`.
    #[serde(rename="Precedence")]
    pub precedence: f64,
    /// Property `RoleArn`.
    #[serde(rename="RoleArn")]
    pub role_arn: String,
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
    pub desired_delivery_mediums: Vec<String>,
    /// Property `ForceAliasCreation`.
    #[serde(rename="ForceAliasCreation")]
    pub force_alias_creation: bool,
    /// Property `MessageAction`.
    #[serde(rename="MessageAction")]
    pub message_action: String,
    /// Property `UserAttributes`.
    #[serde(rename="UserAttributes")]
    pub user_attributes: Vec<self::user_pool_user::AttributeType>,
    /// Property `UserPoolId`.
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
    /// Property `Username`.
    #[serde(rename="Username")]
    pub username: String,
    /// Property `ValidationData`.
    #[serde(rename="ValidationData")]
    pub validation_data: Vec<self::user_pool_user::AttributeType>,
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
        pub client_id: String,
        /// Property `ProviderName`.
        #[serde(rename="ProviderName")]
        pub provider_name: String,
        /// Property `ServerSideTokenCheck`.
        #[serde(rename="ServerSideTokenCheck")]
        pub server_side_token_check: bool,
    }

    /// The [`AWS::Cognito::IdentityPool.CognitoStreams`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypool-cognitostreams.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CognitoStreams {
        /// Property `RoleArn`.
        #[serde(rename="RoleArn")]
        pub role_arn: String,
        /// Property `StreamName`.
        #[serde(rename="StreamName")]
        pub stream_name: String,
        /// Property `StreamingStatus`.
        #[serde(rename="StreamingStatus")]
        pub streaming_status: String,
    }

    /// The [`AWS::Cognito::IdentityPool.PushSync`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypool-pushsync.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PushSync {
        /// Property `ApplicationArns`.
        #[serde(rename="ApplicationArns")]
        pub application_arns: Vec<String>,
        /// Property `RoleArn`.
        #[serde(rename="RoleArn")]
        pub role_arn: String,
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
        pub ambiguous_role_resolution: String,
        /// Property `RulesConfiguration`.
        #[serde(rename="RulesConfiguration")]
        pub rules_configuration: RulesConfigurationType,
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
        pub allow_admin_create_user_only: bool,
        /// Property `InviteMessageTemplate`.
        #[serde(rename="InviteMessageTemplate")]
        pub invite_message_template: InviteMessageTemplate,
        /// Property `UnusedAccountValidityDays`.
        #[serde(rename="UnusedAccountValidityDays")]
        pub unused_account_validity_days: f64,
    }

    /// The [`AWS::Cognito::UserPool.DeviceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-deviceconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct DeviceConfiguration {
        /// Property `ChallengeRequiredOnNewDevice`.
        #[serde(rename="ChallengeRequiredOnNewDevice")]
        pub challenge_required_on_new_device: bool,
        /// Property `DeviceOnlyRememberedOnUserPrompt`.
        #[serde(rename="DeviceOnlyRememberedOnUserPrompt")]
        pub device_only_remembered_on_user_prompt: bool,
    }

    /// The [`AWS::Cognito::UserPool.EmailConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-emailconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct EmailConfiguration {
        /// Property `ReplyToEmailAddress`.
        #[serde(rename="ReplyToEmailAddress")]
        pub reply_to_email_address: String,
        /// Property `SourceArn`.
        #[serde(rename="SourceArn")]
        pub source_arn: String,
    }

    /// The [`AWS::Cognito::UserPool.InviteMessageTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-invitemessagetemplate.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct InviteMessageTemplate {
        /// Property `EmailMessage`.
        #[serde(rename="EmailMessage")]
        pub email_message: String,
        /// Property `EmailSubject`.
        #[serde(rename="EmailSubject")]
        pub email_subject: String,
        /// Property `SMSMessage`.
        #[serde(rename="SMSMessage")]
        pub sms_message: String,
    }

    /// The [`AWS::Cognito::UserPool.LambdaConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-lambdaconfig.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LambdaConfig {
        /// Property `CreateAuthChallenge`.
        #[serde(rename="CreateAuthChallenge")]
        pub create_auth_challenge: String,
        /// Property `CustomMessage`.
        #[serde(rename="CustomMessage")]
        pub custom_message: String,
        /// Property `DefineAuthChallenge`.
        #[serde(rename="DefineAuthChallenge")]
        pub define_auth_challenge: String,
        /// Property `PostAuthentication`.
        #[serde(rename="PostAuthentication")]
        pub post_authentication: String,
        /// Property `PostConfirmation`.
        #[serde(rename="PostConfirmation")]
        pub post_confirmation: String,
        /// Property `PreAuthentication`.
        #[serde(rename="PreAuthentication")]
        pub pre_authentication: String,
        /// Property `PreSignUp`.
        #[serde(rename="PreSignUp")]
        pub pre_sign_up: String,
        /// Property `VerifyAuthChallengeResponse`.
        #[serde(rename="VerifyAuthChallengeResponse")]
        pub verify_auth_challenge_response: String,
    }

    /// The [`AWS::Cognito::UserPool.NumberAttributeConstraints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-numberattributeconstraints.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct NumberAttributeConstraints {
        /// Property `MaxValue`.
        #[serde(rename="MaxValue")]
        pub max_value: String,
        /// Property `MinValue`.
        #[serde(rename="MinValue")]
        pub min_value: String,
    }

    /// The [`AWS::Cognito::UserPool.PasswordPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-passwordpolicy.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct PasswordPolicy {
        /// Property `MinimumLength`.
        #[serde(rename="MinimumLength")]
        pub minimum_length: u32,
        /// Property `RequireLowercase`.
        #[serde(rename="RequireLowercase")]
        pub require_lowercase: bool,
        /// Property `RequireNumbers`.
        #[serde(rename="RequireNumbers")]
        pub require_numbers: bool,
        /// Property `RequireSymbols`.
        #[serde(rename="RequireSymbols")]
        pub require_symbols: bool,
        /// Property `RequireUppercase`.
        #[serde(rename="RequireUppercase")]
        pub require_uppercase: bool,
    }

    /// The [`AWS::Cognito::UserPool.Policies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-policies.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Policies {
        /// Property `PasswordPolicy`.
        #[serde(rename="PasswordPolicy")]
        pub password_policy: PasswordPolicy,
    }

    /// The [`AWS::Cognito::UserPool.SchemaAttribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-schemaattribute.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SchemaAttribute {
        /// Property `AttributeDataType`.
        #[serde(rename="AttributeDataType")]
        pub attribute_data_type: String,
        /// Property `DeveloperOnlyAttribute`.
        #[serde(rename="DeveloperOnlyAttribute")]
        pub developer_only_attribute: bool,
        /// Property `Mutable`.
        #[serde(rename="Mutable")]
        pub mutable: bool,
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
        /// Property `NumberAttributeConstraints`.
        #[serde(rename="NumberAttributeConstraints")]
        pub number_attribute_constraints: NumberAttributeConstraints,
        /// Property `Required`.
        #[serde(rename="Required")]
        pub required: bool,
        /// Property `StringAttributeConstraints`.
        #[serde(rename="StringAttributeConstraints")]
        pub string_attribute_constraints: StringAttributeConstraints,
    }

    /// The [`AWS::Cognito::UserPool.SmsConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-smsconfiguration.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct SmsConfiguration {
        /// Property `ExternalId`.
        #[serde(rename="ExternalId")]
        pub external_id: String,
        /// Property `SnsCallerArn`.
        #[serde(rename="SnsCallerArn")]
        pub sns_caller_arn: String,
    }

    /// The [`AWS::Cognito::UserPool.StringAttributeConstraints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-stringattributeconstraints.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StringAttributeConstraints {
        /// Property `MaxLength`.
        #[serde(rename="MaxLength")]
        pub max_length: String,
        /// Property `MinLength`.
        #[serde(rename="MinLength")]
        pub min_length: String,
    }
}

pub mod user_pool_user {
    //! Property types for the `UserPoolUser` resource.

    /// The [`AWS::Cognito::UserPoolUser.AttributeType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpooluser-attributetype.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct AttributeType {
        /// Property `Name`.
        #[serde(rename="Name")]
        pub name: String,
        /// Property `Value`.
        #[serde(rename="Value")]
        pub value: String,
    }
}
