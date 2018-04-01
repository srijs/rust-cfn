//! Types for the `IAM` service.

/// The [`AWS::IAM::AccessKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-accesskey.html) resource type.
#[derive(Debug)]
pub struct AccessKey {
    properties: AccessKeyProperties
}

/// Properties for the `AccessKey` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct AccessKeyProperties {
    /// Property `Serial`.
    #[serde(rename = "Serial")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serial: Option<::Value<u32>>,
    /// Property `Status`.
    #[serde(rename = "Status")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<::Value<String>>,
    /// Property `UserName`.
    #[serde(rename = "UserName")]
    pub user_name: ::Value<String>,
}

impl<'a> ::Resource<'a> for AccessKey {
    type Properties = AccessKeyProperties;
    const TYPE: &'static str = "AWS::IAM::AccessKey";
    fn properties(&self) -> &AccessKeyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AccessKeyProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for AccessKey {}

impl From<AccessKeyProperties> for AccessKey {
    fn from(properties: AccessKeyProperties) -> AccessKey {
        AccessKey { properties }
    }
}

/// The [`AWS::IAM::Group`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-group.html) resource type.
#[derive(Debug)]
pub struct Group {
    properties: GroupProperties
}

/// Properties for the `Group` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct GroupProperties {
    /// Property `GroupName`.
    #[serde(rename = "GroupName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_name: Option<::Value<String>>,
    /// Property `ManagedPolicyArns`.
    #[serde(rename = "ManagedPolicyArns")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed_policy_arns: Option<::ValueList<String>>,
    /// Property `Path`.
    #[serde(rename = "Path")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<::Value<String>>,
    /// Property `Policies`.
    #[serde(rename = "Policies")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policies: Option<::ValueList<self::group::Policy>>,
}

impl<'a> ::Resource<'a> for Group {
    type Properties = GroupProperties;
    const TYPE: &'static str = "AWS::IAM::Group";
    fn properties(&self) -> &GroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut GroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Group {}

impl From<GroupProperties> for Group {
    fn from(properties: GroupProperties) -> Group {
        Group { properties }
    }
}

/// The [`AWS::IAM::InstanceProfile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-instanceprofile.html) resource type.
#[derive(Debug)]
pub struct InstanceProfile {
    properties: InstanceProfileProperties
}

/// Properties for the `InstanceProfile` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct InstanceProfileProperties {
    /// Property `InstanceProfileName`.
    #[serde(rename = "InstanceProfileName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance_profile_name: Option<::Value<String>>,
    /// Property `Path`.
    #[serde(rename = "Path")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<::Value<String>>,
    /// Property `Roles`.
    #[serde(rename = "Roles")]
    pub roles: ::ValueList<String>,
}

impl<'a> ::Resource<'a> for InstanceProfile {
    type Properties = InstanceProfileProperties;
    const TYPE: &'static str = "AWS::IAM::InstanceProfile";
    fn properties(&self) -> &InstanceProfileProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut InstanceProfileProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for InstanceProfile {}

impl From<InstanceProfileProperties> for InstanceProfile {
    fn from(properties: InstanceProfileProperties) -> InstanceProfile {
        InstanceProfile { properties }
    }
}

/// The [`AWS::IAM::ManagedPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-managedpolicy.html) resource type.
#[derive(Debug)]
pub struct ManagedPolicy {
    properties: ManagedPolicyProperties
}

/// Properties for the `ManagedPolicy` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct ManagedPolicyProperties {
    /// Property `Description`.
    #[serde(rename = "Description")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<::Value<String>>,
    /// Property `Groups`.
    #[serde(rename = "Groups")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<::ValueList<String>>,
    /// Property `ManagedPolicyName`.
    #[serde(rename = "ManagedPolicyName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed_policy_name: Option<::Value<String>>,
    /// Property `Path`.
    #[serde(rename = "Path")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<::Value<String>>,
    /// Property `PolicyDocument`.
    #[serde(rename = "PolicyDocument")]
    pub policy_document: ::Value<::json::Value>,
    /// Property `Roles`.
    #[serde(rename = "Roles")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<::ValueList<String>>,
    /// Property `Users`.
    #[serde(rename = "Users")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub users: Option<::ValueList<String>>,
}

impl<'a> ::Resource<'a> for ManagedPolicy {
    type Properties = ManagedPolicyProperties;
    const TYPE: &'static str = "AWS::IAM::ManagedPolicy";
    fn properties(&self) -> &ManagedPolicyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ManagedPolicyProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ManagedPolicy {}

impl From<ManagedPolicyProperties> for ManagedPolicy {
    fn from(properties: ManagedPolicyProperties) -> ManagedPolicy {
        ManagedPolicy { properties }
    }
}

/// The [`AWS::IAM::Policy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-policy.html) resource type.
#[derive(Debug)]
pub struct Policy {
    properties: PolicyProperties
}

/// Properties for the `Policy` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct PolicyProperties {
    /// Property `Groups`.
    #[serde(rename = "Groups")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<::ValueList<String>>,
    /// Property `PolicyDocument`.
    #[serde(rename = "PolicyDocument")]
    pub policy_document: ::Value<::json::Value>,
    /// Property `PolicyName`.
    #[serde(rename = "PolicyName")]
    pub policy_name: ::Value<String>,
    /// Property `Roles`.
    #[serde(rename = "Roles")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<::ValueList<String>>,
    /// Property `Users`.
    #[serde(rename = "Users")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub users: Option<::ValueList<String>>,
}

impl<'a> ::Resource<'a> for Policy {
    type Properties = PolicyProperties;
    const TYPE: &'static str = "AWS::IAM::Policy";
    fn properties(&self) -> &PolicyProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PolicyProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Policy {}

impl From<PolicyProperties> for Policy {
    fn from(properties: PolicyProperties) -> Policy {
        Policy { properties }
    }
}

/// The [`AWS::IAM::Role`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-role.html) resource type.
#[derive(Debug)]
pub struct Role {
    properties: RoleProperties
}

/// Properties for the `Role` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct RoleProperties {
    /// Property `AssumeRolePolicyDocument`.
    #[serde(rename = "AssumeRolePolicyDocument")]
    pub assume_role_policy_document: ::Value<::json::Value>,
    /// Property `ManagedPolicyArns`.
    #[serde(rename = "ManagedPolicyArns")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed_policy_arns: Option<::ValueList<String>>,
    /// Property `Path`.
    #[serde(rename = "Path")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<::Value<String>>,
    /// Property `Policies`.
    #[serde(rename = "Policies")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policies: Option<::ValueList<self::role::Policy>>,
    /// Property `RoleName`.
    #[serde(rename = "RoleName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_name: Option<::Value<String>>,
}

impl<'a> ::Resource<'a> for Role {
    type Properties = RoleProperties;
    const TYPE: &'static str = "AWS::IAM::Role";
    fn properties(&self) -> &RoleProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RoleProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Role {}

impl From<RoleProperties> for Role {
    fn from(properties: RoleProperties) -> Role {
        Role { properties }
    }
}

/// The [`AWS::IAM::User`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-user.html) resource type.
#[derive(Debug)]
pub struct User {
    properties: UserProperties
}

/// Properties for the `User` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct UserProperties {
    /// Property `Groups`.
    #[serde(rename = "Groups")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<::ValueList<String>>,
    /// Property `LoginProfile`.
    #[serde(rename = "LoginProfile")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub login_profile: Option<::Value<self::user::LoginProfile>>,
    /// Property `ManagedPolicyArns`.
    #[serde(rename = "ManagedPolicyArns")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed_policy_arns: Option<::ValueList<String>>,
    /// Property `Path`.
    #[serde(rename = "Path")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<::Value<String>>,
    /// Property `Policies`.
    #[serde(rename = "Policies")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policies: Option<::ValueList<self::user::Policy>>,
    /// Property `UserName`.
    #[serde(rename = "UserName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<::Value<String>>,
}

impl<'a> ::Resource<'a> for User {
    type Properties = UserProperties;
    const TYPE: &'static str = "AWS::IAM::User";
    fn properties(&self) -> &UserProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut UserProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for User {}

impl From<UserProperties> for User {
    fn from(properties: UserProperties) -> User {
        User { properties }
    }
}

/// The [`AWS::IAM::UserToGroupAddition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-addusertogroup.html) resource type.
#[derive(Debug)]
pub struct UserToGroupAddition {
    properties: UserToGroupAdditionProperties
}

/// Properties for the `UserToGroupAddition` resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct UserToGroupAdditionProperties {
    /// Property `GroupName`.
    #[serde(rename = "GroupName")]
    pub group_name: ::Value<String>,
    /// Property `Users`.
    #[serde(rename = "Users")]
    pub users: ::ValueList<String>,
}

impl<'a> ::Resource<'a> for UserToGroupAddition {
    type Properties = UserToGroupAdditionProperties;
    const TYPE: &'static str = "AWS::IAM::UserToGroupAddition";
    fn properties(&self) -> &UserToGroupAdditionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut UserToGroupAdditionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for UserToGroupAddition {}

impl From<UserToGroupAdditionProperties> for UserToGroupAddition {
    fn from(properties: UserToGroupAdditionProperties) -> UserToGroupAddition {
        UserToGroupAddition { properties }
    }
}

pub mod group {
    //! Property types for the `Group` resource.

    /// The [`AWS::IAM::Group.Policy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-policy.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Policy {
        /// Property `PolicyDocument`.
        #[serde(rename = "PolicyDocument")]
        pub policy_document: ::Value<::json::Value>,
        /// Property `PolicyName`.
        #[serde(rename = "PolicyName")]
        pub policy_name: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(Policy);
}

pub mod role {
    //! Property types for the `Role` resource.

    /// The [`AWS::IAM::Role.Policy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-policy.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Policy {
        /// Property `PolicyDocument`.
        #[serde(rename = "PolicyDocument")]
        pub policy_document: ::Value<::json::Value>,
        /// Property `PolicyName`.
        #[serde(rename = "PolicyName")]
        pub policy_name: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(Policy);
}

pub mod user {
    //! Property types for the `User` resource.

    /// The [`AWS::IAM::User.LoginProfile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-user-loginprofile.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LoginProfile {
        /// Property `Password`.
        #[serde(rename = "Password")]
        pub password: ::Value<String>,
        /// Property `PasswordResetRequired`.
        #[serde(rename = "PasswordResetRequired")]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub password_reset_required: Option<::Value<bool>>,
    }

    cfn_internal__inherit_codec_impls!(LoginProfile);

    /// The [`AWS::IAM::User.Policy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-policy.html) property type.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Policy {
        /// Property `PolicyDocument`.
        #[serde(rename = "PolicyDocument")]
        pub policy_document: ::Value<::json::Value>,
        /// Property `PolicyName`.
        #[serde(rename = "PolicyName")]
        pub policy_name: ::Value<String>,
    }

    cfn_internal__inherit_codec_impls!(Policy);
}
