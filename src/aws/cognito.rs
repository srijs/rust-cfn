/// The [`AWS::Cognito::IdentityPool`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-identitypool.html) resource.
pub struct IdentityPool {
    properties: IdentityPoolProperties
}

/// Properties for the `IdentityPool` resource.
#[derive(Serialize, Deserialize)]
pub struct IdentityPoolProperties {
    #[serde(rename="AllowUnauthenticatedIdentities")]
    pub allow_unauthenticated_identities: bool,
    #[serde(rename="CognitoEvents")]
    pub cognito_events: ::json::Value,
    #[serde(rename="CognitoIdentityProviders")]
    pub cognito_identity_providers: Vec<self::identity_pool::CognitoIdentityProvider>,
    #[serde(rename="CognitoStreams")]
    pub cognito_streams: self::identity_pool::CognitoStreams,
    #[serde(rename="DeveloperProviderName")]
    pub developer_provider_name: String,
    #[serde(rename="IdentityPoolName")]
    pub identity_pool_name: String,
    #[serde(rename="OpenIdConnectProviderARNs")]
    pub open_id_connect_provider_ar_ns: Vec<String>,
    #[serde(rename="PushSync")]
    pub push_sync: self::identity_pool::PushSync,
    #[serde(rename="SamlProviderARNs")]
    pub saml_provider_ar_ns: Vec<String>,
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

impl From<IdentityPoolProperties> for IdentityPool {
    fn from(properties: IdentityPoolProperties) -> IdentityPool {
        IdentityPool { properties }
    }
}

/// The [`AWS::Cognito::IdentityPoolRoleAttachment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-identitypoolroleattachment.html) resource.
pub struct IdentityPoolRoleAttachment {
    properties: IdentityPoolRoleAttachmentProperties
}

/// Properties for the `IdentityPoolRoleAttachment` resource.
#[derive(Serialize, Deserialize)]
pub struct IdentityPoolRoleAttachmentProperties {
    #[serde(rename="IdentityPoolId")]
    pub identity_pool_id: String,
    #[serde(rename="RoleMappings")]
    pub role_mappings: ::json::Value,
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

impl From<IdentityPoolRoleAttachmentProperties> for IdentityPoolRoleAttachment {
    fn from(properties: IdentityPoolRoleAttachmentProperties) -> IdentityPoolRoleAttachment {
        IdentityPoolRoleAttachment { properties }
    }
}

/// The [`AWS::Cognito::UserPool`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpool.html) resource.
pub struct UserPool {
    properties: UserPoolProperties
}

/// Properties for the `UserPool` resource.
#[derive(Serialize, Deserialize)]
pub struct UserPoolProperties {
    #[serde(rename="AdminCreateUserConfig")]
    pub admin_create_user_config: self::user_pool::AdminCreateUserConfig,
    #[serde(rename="AliasAttributes")]
    pub alias_attributes: Vec<String>,
    #[serde(rename="AutoVerifiedAttributes")]
    pub auto_verified_attributes: Vec<String>,
    #[serde(rename="DeviceConfiguration")]
    pub device_configuration: self::user_pool::DeviceConfiguration,
    #[serde(rename="EmailConfiguration")]
    pub email_configuration: self::user_pool::EmailConfiguration,
    #[serde(rename="EmailVerificationMessage")]
    pub email_verification_message: String,
    #[serde(rename="EmailVerificationSubject")]
    pub email_verification_subject: String,
    #[serde(rename="LambdaConfig")]
    pub lambda_config: self::user_pool::LambdaConfig,
    #[serde(rename="MfaConfiguration")]
    pub mfa_configuration: String,
    #[serde(rename="Policies")]
    pub policies: self::user_pool::Policies,
    #[serde(rename="Schema")]
    pub schema: Vec<self::user_pool::SchemaAttribute>,
    #[serde(rename="SmsAuthenticationMessage")]
    pub sms_authentication_message: String,
    #[serde(rename="SmsConfiguration")]
    pub sms_configuration: self::user_pool::SmsConfiguration,
    #[serde(rename="SmsVerificationMessage")]
    pub sms_verification_message: String,
    #[serde(rename="UserPoolName")]
    pub user_pool_name: String,
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

impl From<UserPoolProperties> for UserPool {
    fn from(properties: UserPoolProperties) -> UserPool {
        UserPool { properties }
    }
}

/// The [`AWS::Cognito::UserPoolClient`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolclient.html) resource.
pub struct UserPoolClient {
    properties: UserPoolClientProperties
}

/// Properties for the `UserPoolClient` resource.
#[derive(Serialize, Deserialize)]
pub struct UserPoolClientProperties {
    #[serde(rename="ClientName")]
    pub client_name: String,
    #[serde(rename="ExplicitAuthFlows")]
    pub explicit_auth_flows: Vec<String>,
    #[serde(rename="GenerateSecret")]
    pub generate_secret: bool,
    #[serde(rename="ReadAttributes")]
    pub read_attributes: Vec<String>,
    #[serde(rename="RefreshTokenValidity")]
    pub refresh_token_validity: f64,
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
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

impl From<UserPoolClientProperties> for UserPoolClient {
    fn from(properties: UserPoolClientProperties) -> UserPoolClient {
        UserPoolClient { properties }
    }
}

/// The [`AWS::Cognito::UserPoolGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolgroup.html) resource.
pub struct UserPoolGroup {
    properties: UserPoolGroupProperties
}

/// Properties for the `UserPoolGroup` resource.
#[derive(Serialize, Deserialize)]
pub struct UserPoolGroupProperties {
    #[serde(rename="Description")]
    pub description: String,
    #[serde(rename="GroupName")]
    pub group_name: String,
    #[serde(rename="Precedence")]
    pub precedence: f64,
    #[serde(rename="RoleArn")]
    pub role_arn: String,
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

impl From<UserPoolGroupProperties> for UserPoolGroup {
    fn from(properties: UserPoolGroupProperties) -> UserPoolGroup {
        UserPoolGroup { properties }
    }
}

/// The [`AWS::Cognito::UserPoolUser`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpooluser.html) resource.
pub struct UserPoolUser {
    properties: UserPoolUserProperties
}

/// Properties for the `UserPoolUser` resource.
#[derive(Serialize, Deserialize)]
pub struct UserPoolUserProperties {
    #[serde(rename="DesiredDeliveryMediums")]
    pub desired_delivery_mediums: Vec<String>,
    #[serde(rename="ForceAliasCreation")]
    pub force_alias_creation: bool,
    #[serde(rename="MessageAction")]
    pub message_action: String,
    #[serde(rename="UserAttributes")]
    pub user_attributes: Vec<self::user_pool_user::AttributeType>,
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
    #[serde(rename="Username")]
    pub username: String,
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

impl From<UserPoolUserProperties> for UserPoolUser {
    fn from(properties: UserPoolUserProperties) -> UserPoolUser {
        UserPoolUser { properties }
    }
}

/// The [`AWS::Cognito::UserPoolUserToGroupAttachment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolusertogroupattachment.html) resource.
pub struct UserPoolUserToGroupAttachment {
    properties: UserPoolUserToGroupAttachmentProperties
}

/// Properties for the `UserPoolUserToGroupAttachment` resource.
#[derive(Serialize, Deserialize)]
pub struct UserPoolUserToGroupAttachmentProperties {
    #[serde(rename="GroupName")]
    pub group_name: String,
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
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

impl From<UserPoolUserToGroupAttachmentProperties> for UserPoolUserToGroupAttachment {
    fn from(properties: UserPoolUserToGroupAttachmentProperties) -> UserPoolUserToGroupAttachment {
        UserPoolUserToGroupAttachment { properties }
    }
}

pub mod identity_pool {
    #[derive(Serialize, Deserialize)]
    pub struct CognitoIdentityProvider {
        #[serde(rename="ClientId")]
        pub client_id: String,
        #[serde(rename="ProviderName")]
        pub provider_name: String,
        #[serde(rename="ServerSideTokenCheck")]
        pub server_side_token_check: bool,
    }

    #[derive(Serialize, Deserialize)]
    pub struct CognitoStreams {
        #[serde(rename="RoleArn")]
        pub role_arn: String,
        #[serde(rename="StreamName")]
        pub stream_name: String,
        #[serde(rename="StreamingStatus")]
        pub streaming_status: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct PushSync {
        #[serde(rename="ApplicationArns")]
        pub application_arns: Vec<String>,
        #[serde(rename="RoleArn")]
        pub role_arn: String,
    }

}

pub mod identity_pool_role_attachment {
    #[derive(Serialize, Deserialize)]
    pub struct MappingRule {
        #[serde(rename="Claim")]
        pub claim: String,
        #[serde(rename="MatchType")]
        pub match_type: String,
        #[serde(rename="RoleARN")]
        pub role_arn: String,
        #[serde(rename="Value")]
        pub value: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct RoleMapping {
        #[serde(rename="AmbiguousRoleResolution")]
        pub ambiguous_role_resolution: String,
        #[serde(rename="RulesConfiguration")]
        pub rules_configuration: RulesConfigurationType,
        #[serde(rename="Type")]
        pub type_: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct RulesConfigurationType {
        #[serde(rename="Rules")]
        pub rules: Vec<MappingRule>,
    }

}

pub mod user_pool {
    #[derive(Serialize, Deserialize)]
    pub struct AdminCreateUserConfig {
        #[serde(rename="AllowAdminCreateUserOnly")]
        pub allow_admin_create_user_only: bool,
        #[serde(rename="InviteMessageTemplate")]
        pub invite_message_template: InviteMessageTemplate,
        #[serde(rename="UnusedAccountValidityDays")]
        pub unused_account_validity_days: f64,
    }

    #[derive(Serialize, Deserialize)]
    pub struct DeviceConfiguration {
        #[serde(rename="ChallengeRequiredOnNewDevice")]
        pub challenge_required_on_new_device: bool,
        #[serde(rename="DeviceOnlyRememberedOnUserPrompt")]
        pub device_only_remembered_on_user_prompt: bool,
    }

    #[derive(Serialize, Deserialize)]
    pub struct EmailConfiguration {
        #[serde(rename="ReplyToEmailAddress")]
        pub reply_to_email_address: String,
        #[serde(rename="SourceArn")]
        pub source_arn: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct InviteMessageTemplate {
        #[serde(rename="EmailMessage")]
        pub email_message: String,
        #[serde(rename="EmailSubject")]
        pub email_subject: String,
        #[serde(rename="SMSMessage")]
        pub sms_message: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct LambdaConfig {
        #[serde(rename="CreateAuthChallenge")]
        pub create_auth_challenge: String,
        #[serde(rename="CustomMessage")]
        pub custom_message: String,
        #[serde(rename="DefineAuthChallenge")]
        pub define_auth_challenge: String,
        #[serde(rename="PostAuthentication")]
        pub post_authentication: String,
        #[serde(rename="PostConfirmation")]
        pub post_confirmation: String,
        #[serde(rename="PreAuthentication")]
        pub pre_authentication: String,
        #[serde(rename="PreSignUp")]
        pub pre_sign_up: String,
        #[serde(rename="VerifyAuthChallengeResponse")]
        pub verify_auth_challenge_response: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct NumberAttributeConstraints {
        #[serde(rename="MaxValue")]
        pub max_value: String,
        #[serde(rename="MinValue")]
        pub min_value: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct PasswordPolicy {
        #[serde(rename="MinimumLength")]
        pub minimum_length: u32,
        #[serde(rename="RequireLowercase")]
        pub require_lowercase: bool,
        #[serde(rename="RequireNumbers")]
        pub require_numbers: bool,
        #[serde(rename="RequireSymbols")]
        pub require_symbols: bool,
        #[serde(rename="RequireUppercase")]
        pub require_uppercase: bool,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Policies {
        #[serde(rename="PasswordPolicy")]
        pub password_policy: PasswordPolicy,
    }

    #[derive(Serialize, Deserialize)]
    pub struct SchemaAttribute {
        #[serde(rename="AttributeDataType")]
        pub attribute_data_type: String,
        #[serde(rename="DeveloperOnlyAttribute")]
        pub developer_only_attribute: bool,
        #[serde(rename="Mutable")]
        pub mutable: bool,
        #[serde(rename="Name")]
        pub name: String,
        #[serde(rename="NumberAttributeConstraints")]
        pub number_attribute_constraints: NumberAttributeConstraints,
        #[serde(rename="Required")]
        pub required: bool,
        #[serde(rename="StringAttributeConstraints")]
        pub string_attribute_constraints: StringAttributeConstraints,
    }

    #[derive(Serialize, Deserialize)]
    pub struct SmsConfiguration {
        #[serde(rename="ExternalId")]
        pub external_id: String,
        #[serde(rename="SnsCallerArn")]
        pub sns_caller_arn: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct StringAttributeConstraints {
        #[serde(rename="MaxLength")]
        pub max_length: String,
        #[serde(rename="MinLength")]
        pub min_length: String,
    }

}

pub mod user_pool_user {
    #[derive(Serialize, Deserialize)]
    pub struct AttributeType {
        #[serde(rename="Name")]
        pub name: String,
        #[serde(rename="Value")]
        pub value: String,
    }

}

