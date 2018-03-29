/// The [`AWS::IAM::AccessKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-accesskey.html) resource.
pub struct AccessKey {
    properties: AccessKeyProperties
}

/// Properties for the `AccessKey` resource.
#[derive(Serialize, Deserialize)]
pub struct AccessKeyProperties {
    #[serde(rename="Serial")]
    pub serial: u32,
    #[serde(rename="Status")]
    pub status: String,
    #[serde(rename="UserName")]
    pub user_name: String,
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

impl From<AccessKeyProperties> for AccessKey {
    fn from(properties: AccessKeyProperties) -> AccessKey {
        AccessKey { properties }
    }
}

/// The [`AWS::IAM::Group`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-group.html) resource.
pub struct Group {
    properties: GroupProperties
}

/// Properties for the `Group` resource.
#[derive(Serialize, Deserialize)]
pub struct GroupProperties {
    #[serde(rename="GroupName")]
    pub group_name: String,
    #[serde(rename="ManagedPolicyArns")]
    pub managed_policy_arns: Vec<String>,
    #[serde(rename="Path")]
    pub path: String,
    #[serde(rename="Policies")]
    pub policies: Vec<self::group::Policy>,
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

impl From<GroupProperties> for Group {
    fn from(properties: GroupProperties) -> Group {
        Group { properties }
    }
}

/// The [`AWS::IAM::InstanceProfile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-instanceprofile.html) resource.
pub struct InstanceProfile {
    properties: InstanceProfileProperties
}

/// Properties for the `InstanceProfile` resource.
#[derive(Serialize, Deserialize)]
pub struct InstanceProfileProperties {
    #[serde(rename="InstanceProfileName")]
    pub instance_profile_name: String,
    #[serde(rename="Path")]
    pub path: String,
    #[serde(rename="Roles")]
    pub roles: Vec<String>,
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

impl From<InstanceProfileProperties> for InstanceProfile {
    fn from(properties: InstanceProfileProperties) -> InstanceProfile {
        InstanceProfile { properties }
    }
}

/// The [`AWS::IAM::ManagedPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-managedpolicy.html) resource.
pub struct ManagedPolicy {
    properties: ManagedPolicyProperties
}

/// Properties for the `ManagedPolicy` resource.
#[derive(Serialize, Deserialize)]
pub struct ManagedPolicyProperties {
    #[serde(rename="Description")]
    pub description: String,
    #[serde(rename="Groups")]
    pub groups: Vec<String>,
    #[serde(rename="ManagedPolicyName")]
    pub managed_policy_name: String,
    #[serde(rename="Path")]
    pub path: String,
    #[serde(rename="PolicyDocument")]
    pub policy_document: ::json::Value,
    #[serde(rename="Roles")]
    pub roles: Vec<String>,
    #[serde(rename="Users")]
    pub users: Vec<String>,
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

impl From<ManagedPolicyProperties> for ManagedPolicy {
    fn from(properties: ManagedPolicyProperties) -> ManagedPolicy {
        ManagedPolicy { properties }
    }
}

/// The [`AWS::IAM::Policy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-policy.html) resource.
pub struct Policy {
    properties: PolicyProperties
}

/// Properties for the `Policy` resource.
#[derive(Serialize, Deserialize)]
pub struct PolicyProperties {
    #[serde(rename="Groups")]
    pub groups: Vec<String>,
    #[serde(rename="PolicyDocument")]
    pub policy_document: ::json::Value,
    #[serde(rename="PolicyName")]
    pub policy_name: String,
    #[serde(rename="Roles")]
    pub roles: Vec<String>,
    #[serde(rename="Users")]
    pub users: Vec<String>,
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

impl From<PolicyProperties> for Policy {
    fn from(properties: PolicyProperties) -> Policy {
        Policy { properties }
    }
}

/// The [`AWS::IAM::Role`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-role.html) resource.
pub struct Role {
    properties: RoleProperties
}

/// Properties for the `Role` resource.
#[derive(Serialize, Deserialize)]
pub struct RoleProperties {
    #[serde(rename="AssumeRolePolicyDocument")]
    pub assume_role_policy_document: ::json::Value,
    #[serde(rename="ManagedPolicyArns")]
    pub managed_policy_arns: Vec<String>,
    #[serde(rename="Path")]
    pub path: String,
    #[serde(rename="Policies")]
    pub policies: Vec<self::role::Policy>,
    #[serde(rename="RoleName")]
    pub role_name: String,
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

impl From<RoleProperties> for Role {
    fn from(properties: RoleProperties) -> Role {
        Role { properties }
    }
}

/// The [`AWS::IAM::User`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-user.html) resource.
pub struct User {
    properties: UserProperties
}

/// Properties for the `User` resource.
#[derive(Serialize, Deserialize)]
pub struct UserProperties {
    #[serde(rename="Groups")]
    pub groups: Vec<String>,
    #[serde(rename="LoginProfile")]
    pub login_profile: self::user::LoginProfile,
    #[serde(rename="ManagedPolicyArns")]
    pub managed_policy_arns: Vec<String>,
    #[serde(rename="Path")]
    pub path: String,
    #[serde(rename="Policies")]
    pub policies: Vec<self::user::Policy>,
    #[serde(rename="UserName")]
    pub user_name: String,
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

impl From<UserProperties> for User {
    fn from(properties: UserProperties) -> User {
        User { properties }
    }
}

/// The [`AWS::IAM::UserToGroupAddition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-addusertogroup.html) resource.
pub struct UserToGroupAddition {
    properties: UserToGroupAdditionProperties
}

/// Properties for the `UserToGroupAddition` resource.
#[derive(Serialize, Deserialize)]
pub struct UserToGroupAdditionProperties {
    #[serde(rename="GroupName")]
    pub group_name: String,
    #[serde(rename="Users")]
    pub users: Vec<String>,
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

impl From<UserToGroupAdditionProperties> for UserToGroupAddition {
    fn from(properties: UserToGroupAdditionProperties) -> UserToGroupAddition {
        UserToGroupAddition { properties }
    }
}

pub mod group {
    #[derive(Serialize, Deserialize)]
    pub struct Policy {
        #[serde(rename="PolicyDocument")]
        pub policy_document: ::json::Value,
        #[serde(rename="PolicyName")]
        pub policy_name: String,
    }

}

pub mod role {
    #[derive(Serialize, Deserialize)]
    pub struct Policy {
        #[serde(rename="PolicyDocument")]
        pub policy_document: ::json::Value,
        #[serde(rename="PolicyName")]
        pub policy_name: String,
    }

}

pub mod user {
    #[derive(Serialize, Deserialize)]
    pub struct LoginProfile {
        #[serde(rename="Password")]
        pub password: String,
        #[serde(rename="PasswordResetRequired")]
        pub password_reset_required: bool,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Policy {
        #[serde(rename="PolicyDocument")]
        pub policy_document: ::json::Value,
        #[serde(rename="PolicyName")]
        pub policy_name: String,
    }

}

