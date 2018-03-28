/// The [`AWS::Cognito::IdentityPool`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-identitypool.html) resource.
#[derive(Serialize, Deserialize)]
pub struct IdentityPool {
    properties: IdentityPoolProperties
}

/// Properties for the `IdentityPool` resource.
#[derive(Serialize, Deserialize)]
pub struct IdentityPoolProperties {
    #[serde(rename="AllowUnauthenticatedIdentities")]
    pub allow_unauthenticated_identities: (),
    #[serde(rename="CognitoEvents")]
    pub cognito_events: (),
    #[serde(rename="CognitoIdentityProviders")]
    pub cognito_identity_providers: (),
    #[serde(rename="CognitoStreams")]
    pub cognito_streams: (),
    #[serde(rename="DeveloperProviderName")]
    pub developer_provider_name: (),
    #[serde(rename="IdentityPoolName")]
    pub identity_pool_name: (),
    #[serde(rename="OpenIdConnectProviderARNs")]
    pub open_id_connect_provider_ar_ns: (),
    #[serde(rename="PushSync")]
    pub push_sync: (),
    #[serde(rename="SamlProviderARNs")]
    pub saml_provider_ar_ns: (),
    #[serde(rename="SupportedLoginProviders")]
    pub supported_login_providers: (),
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
    pub identity_pool_id: (),
    #[serde(rename="RoleMappings")]
    pub role_mappings: (),
    #[serde(rename="Roles")]
    pub roles: (),
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
    pub alias_attributes: (),
    #[serde(rename="AutoVerifiedAttributes")]
    pub auto_verified_attributes: (),
    #[serde(rename="DeviceConfiguration")]
    pub device_configuration: (),
    #[serde(rename="EmailConfiguration")]
    pub email_configuration: (),
    #[serde(rename="EmailVerificationMessage")]
    pub email_verification_message: (),
    #[serde(rename="EmailVerificationSubject")]
    pub email_verification_subject: (),
    #[serde(rename="LambdaConfig")]
    pub lambda_config: (),
    #[serde(rename="MfaConfiguration")]
    pub mfa_configuration: (),
    #[serde(rename="Policies")]
    pub policies: (),
    #[serde(rename="Schema")]
    pub schema: (),
    #[serde(rename="SmsAuthenticationMessage")]
    pub sms_authentication_message: (),
    #[serde(rename="SmsConfiguration")]
    pub sms_configuration: (),
    #[serde(rename="SmsVerificationMessage")]
    pub sms_verification_message: (),
    #[serde(rename="UserPoolName")]
    pub user_pool_name: (),
    #[serde(rename="UserPoolTags")]
    pub user_pool_tags: (),
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
    pub client_name: (),
    #[serde(rename="ExplicitAuthFlows")]
    pub explicit_auth_flows: (),
    #[serde(rename="GenerateSecret")]
    pub generate_secret: (),
    #[serde(rename="ReadAttributes")]
    pub read_attributes: (),
    #[serde(rename="RefreshTokenValidity")]
    pub refresh_token_validity: (),
    #[serde(rename="UserPoolId")]
    pub user_pool_id: (),
    #[serde(rename="WriteAttributes")]
    pub write_attributes: (),
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
    pub description: (),
    #[serde(rename="GroupName")]
    pub group_name: (),
    #[serde(rename="Precedence")]
    pub precedence: (),
    #[serde(rename="RoleArn")]
    pub role_arn: (),
    #[serde(rename="UserPoolId")]
    pub user_pool_id: (),
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
    pub desired_delivery_mediums: (),
    #[serde(rename="ForceAliasCreation")]
    pub force_alias_creation: (),
    #[serde(rename="MessageAction")]
    pub message_action: (),
    #[serde(rename="UserAttributes")]
    pub user_attributes: (),
    #[serde(rename="UserPoolId")]
    pub user_pool_id: (),
    #[serde(rename="Username")]
    pub username: (),
    #[serde(rename="ValidationData")]
    pub validation_data: (),
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
    pub group_name: (),
    #[serde(rename="UserPoolId")]
    pub user_pool_id: (),
    #[serde(rename="Username")]
    pub username: (),
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

