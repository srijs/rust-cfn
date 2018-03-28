/// The [`AWS::Cognito::IdentityPool`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-identitypool.html) resource.
#[derive(Serialize, Deserialize)]
pub struct IdentityPool {
    properties: IdentityPoolProperties
}

/// Properties for the `IdentityPool` resource.
#[derive(Serialize, Deserialize)]
pub struct IdentityPoolProperties {
    #[serde(rename="AllowUnauthenticatedIdentities")]
    pub allow_unauthenticated_identities: bool,
    #[serde(rename="CognitoEvents")]
    pub cognito_events: String,
    #[serde(rename="CognitoIdentityProviders")]
    pub cognito_identity_providers: Vec<()>,
    #[serde(rename="CognitoStreams")]
    pub cognito_streams: (),
    #[serde(rename="DeveloperProviderName")]
    pub developer_provider_name: String,
    #[serde(rename="IdentityPoolName")]
    pub identity_pool_name: String,
    #[serde(rename="OpenIdConnectProviderARNs")]
    pub open_id_connect_provider_ar_ns: Vec<String>,
    #[serde(rename="PushSync")]
    pub push_sync: (),
    #[serde(rename="SamlProviderARNs")]
    pub saml_provider_ar_ns: Vec<String>,
    #[serde(rename="SupportedLoginProviders")]
    pub supported_login_providers: String,
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
#[derive(Serialize, Deserialize)]
pub struct IdentityPoolRoleAttachment {
    properties: IdentityPoolRoleAttachmentProperties
}

/// Properties for the `IdentityPoolRoleAttachment` resource.
#[derive(Serialize, Deserialize)]
pub struct IdentityPoolRoleAttachmentProperties {
    #[serde(rename="IdentityPoolId")]
    pub identity_pool_id: String,
    #[serde(rename="RoleMappings")]
    pub role_mappings: String,
    #[serde(rename="Roles")]
    pub roles: String,
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
#[derive(Serialize, Deserialize)]
pub struct UserPool {
    properties: UserPoolProperties
}

/// Properties for the `UserPool` resource.
#[derive(Serialize, Deserialize)]
pub struct UserPoolProperties {
    #[serde(rename="AdminCreateUserConfig")]
    pub admin_create_user_config: (),
    #[serde(rename="AliasAttributes")]
    pub alias_attributes: Vec<String>,
    #[serde(rename="AutoVerifiedAttributes")]
    pub auto_verified_attributes: Vec<String>,
    #[serde(rename="DeviceConfiguration")]
    pub device_configuration: (),
    #[serde(rename="EmailConfiguration")]
    pub email_configuration: (),
    #[serde(rename="EmailVerificationMessage")]
    pub email_verification_message: String,
    #[serde(rename="EmailVerificationSubject")]
    pub email_verification_subject: String,
    #[serde(rename="LambdaConfig")]
    pub lambda_config: (),
    #[serde(rename="MfaConfiguration")]
    pub mfa_configuration: String,
    #[serde(rename="Policies")]
    pub policies: (),
    #[serde(rename="Schema")]
    pub schema: Vec<()>,
    #[serde(rename="SmsAuthenticationMessage")]
    pub sms_authentication_message: String,
    #[serde(rename="SmsConfiguration")]
    pub sms_configuration: (),
    #[serde(rename="SmsVerificationMessage")]
    pub sms_verification_message: String,
    #[serde(rename="UserPoolName")]
    pub user_pool_name: String,
    #[serde(rename="UserPoolTags")]
    pub user_pool_tags: String,
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
#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
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
    pub user_attributes: Vec<()>,
    #[serde(rename="UserPoolId")]
    pub user_pool_id: String,
    #[serde(rename="Username")]
    pub username: String,
    #[serde(rename="ValidationData")]
    pub validation_data: Vec<()>,
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
#[derive(Serialize, Deserialize)]
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

