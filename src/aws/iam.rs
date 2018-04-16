//! Types for the `IAM` service.

/// The [`AWS::IAM::AccessKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-accesskey.html) resource type.
#[derive(Debug, Default)]
pub struct AccessKey {
    properties: AccessKeyProperties
}

/// Properties for the `AccessKey` resource.
#[derive(Debug, Default)]
pub struct AccessKeyProperties {
    /// Property [`Serial`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-accesskey.html#cfn-iam-accesskey-serial).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub serial: Option<::Value<u32>>,
    /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-accesskey.html#cfn-iam-accesskey-status).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub status: Option<::Value<String>>,
    /// Property [`UserName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-accesskey.html#cfn-iam-accesskey-username).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub user_name: ::Value<String>,
}

impl ::serde::Serialize for AccessKeyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref serial) = self.serial {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Serial", serial)?;
        }
        if let Some(ref status) = self.status {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", status)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserName", &self.user_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AccessKeyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AccessKeyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AccessKeyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AccessKeyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut serial: Option<::Value<u32>> = None;
                let mut status: Option<::Value<String>> = None;
                let mut user_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Serial" => {
                            serial = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Status" => {
                            status = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserName" => {
                            user_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AccessKeyProperties {
                    serial: serial,
                    status: status,
                    user_name: user_name.ok_or(::serde::de::Error::missing_field("UserName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for AccessKey {
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
#[derive(Debug, Default)]
pub struct Group {
    properties: GroupProperties
}

/// Properties for the `Group` resource.
#[derive(Debug, Default)]
pub struct GroupProperties {
    /// Property [`GroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-group.html#cfn-iam-group-groupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub group_name: Option<::Value<String>>,
    /// Property [`ManagedPolicyArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-group.html#cfn-iam-group-managepolicyarns).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub managed_policy_arns: Option<::ValueList<String>>,
    /// Property [`Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-group.html#cfn-iam-group-path).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub path: Option<::Value<String>>,
    /// Property [`Policies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-group.html#cfn-iam-group-policies).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub policies: Option<::ValueList<self::group::Policy>>,
}

impl ::serde::Serialize for GroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref group_name) = self.group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupName", group_name)?;
        }
        if let Some(ref managed_policy_arns) = self.managed_policy_arns {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManagedPolicyArns", managed_policy_arns)?;
        }
        if let Some(ref path) = self.path {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", path)?;
        }
        if let Some(ref policies) = self.policies {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Policies", policies)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for GroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<GroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = GroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type GroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut group_name: Option<::Value<String>> = None;
                let mut managed_policy_arns: Option<::ValueList<String>> = None;
                let mut path: Option<::Value<String>> = None;
                let mut policies: Option<::ValueList<self::group::Policy>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "GroupName" => {
                            group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ManagedPolicyArns" => {
                            managed_policy_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Path" => {
                            path = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Policies" => {
                            policies = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(GroupProperties {
                    group_name: group_name,
                    managed_policy_arns: managed_policy_arns,
                    path: path,
                    policies: policies,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Group {
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
#[derive(Debug, Default)]
pub struct InstanceProfile {
    properties: InstanceProfileProperties
}

/// Properties for the `InstanceProfile` resource.
#[derive(Debug, Default)]
pub struct InstanceProfileProperties {
    /// Property [`InstanceProfileName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-instanceprofile.html#cfn-iam-instanceprofile-instanceprofilename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub instance_profile_name: Option<::Value<String>>,
    /// Property [`Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-instanceprofile.html#cfn-iam-instanceprofile-path).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub path: Option<::Value<String>>,
    /// Property [`Roles`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-instanceprofile.html#cfn-iam-instanceprofile-roles).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub roles: ::ValueList<String>,
}

impl ::serde::Serialize for InstanceProfileProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref instance_profile_name) = self.instance_profile_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceProfileName", instance_profile_name)?;
        }
        if let Some(ref path) = self.path {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", path)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Roles", &self.roles)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for InstanceProfileProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<InstanceProfileProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = InstanceProfileProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type InstanceProfileProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut instance_profile_name: Option<::Value<String>> = None;
                let mut path: Option<::Value<String>> = None;
                let mut roles: Option<::ValueList<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "InstanceProfileName" => {
                            instance_profile_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Path" => {
                            path = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Roles" => {
                            roles = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(InstanceProfileProperties {
                    instance_profile_name: instance_profile_name,
                    path: path,
                    roles: roles.ok_or(::serde::de::Error::missing_field("Roles"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for InstanceProfile {
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
#[derive(Debug, Default)]
pub struct ManagedPolicy {
    properties: ManagedPolicyProperties
}

/// Properties for the `ManagedPolicy` resource.
#[derive(Debug, Default)]
pub struct ManagedPolicyProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-managedpolicy.html#cfn-iam-managedpolicy-description).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Groups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-managedpolicy.html#cfn-iam-managedpolicy-groups).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub groups: Option<::ValueList<String>>,
    /// Property [`ManagedPolicyName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-managedpolicy.html#cfn-iam-managedpolicy-managedpolicyname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub managed_policy_name: Option<::Value<String>>,
    /// Property [`Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-managedpolicy.html#cfn-ec2-dhcpoptions-path).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub path: Option<::Value<String>>,
    /// Property [`PolicyDocument`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-managedpolicy.html#cfn-iam-managedpolicy-policydocument).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub policy_document: ::Value<::json::Value>,
    /// Property [`Roles`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-managedpolicy.html#cfn-iam-managedpolicy-roles).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub roles: Option<::ValueList<String>>,
    /// Property [`Users`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-managedpolicy.html#cfn-iam-managedpolicy-users).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub users: Option<::ValueList<String>>,
}

impl ::serde::Serialize for ManagedPolicyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref groups) = self.groups {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Groups", groups)?;
        }
        if let Some(ref managed_policy_name) = self.managed_policy_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManagedPolicyName", managed_policy_name)?;
        }
        if let Some(ref path) = self.path {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", path)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyDocument", &self.policy_document)?;
        if let Some(ref roles) = self.roles {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Roles", roles)?;
        }
        if let Some(ref users) = self.users {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Users", users)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ManagedPolicyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ManagedPolicyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ManagedPolicyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ManagedPolicyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut groups: Option<::ValueList<String>> = None;
                let mut managed_policy_name: Option<::Value<String>> = None;
                let mut path: Option<::Value<String>> = None;
                let mut policy_document: Option<::Value<::json::Value>> = None;
                let mut roles: Option<::ValueList<String>> = None;
                let mut users: Option<::ValueList<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Groups" => {
                            groups = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ManagedPolicyName" => {
                            managed_policy_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Path" => {
                            path = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PolicyDocument" => {
                            policy_document = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Roles" => {
                            roles = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Users" => {
                            users = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ManagedPolicyProperties {
                    description: description,
                    groups: groups,
                    managed_policy_name: managed_policy_name,
                    path: path,
                    policy_document: policy_document.ok_or(::serde::de::Error::missing_field("PolicyDocument"))?,
                    roles: roles,
                    users: users,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ManagedPolicy {
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
#[derive(Debug, Default)]
pub struct Policy {
    properties: PolicyProperties
}

/// Properties for the `Policy` resource.
#[derive(Debug, Default)]
pub struct PolicyProperties {
    /// Property [`Groups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-policy.html#cfn-iam-policy-groups).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub groups: Option<::ValueList<String>>,
    /// Property [`PolicyDocument`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-policy.html#cfn-iam-policy-policydocument).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub policy_document: ::Value<::json::Value>,
    /// Property [`PolicyName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-policy.html#cfn-iam-policy-policyname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub policy_name: ::Value<String>,
    /// Property [`Roles`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-policy.html#cfn-iam-policy-roles).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub roles: Option<::ValueList<String>>,
    /// Property [`Users`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-policy.html#cfn-iam-policy-users).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub users: Option<::ValueList<String>>,
}

impl ::serde::Serialize for PolicyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref groups) = self.groups {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Groups", groups)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyDocument", &self.policy_document)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyName", &self.policy_name)?;
        if let Some(ref roles) = self.roles {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Roles", roles)?;
        }
        if let Some(ref users) = self.users {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Users", users)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PolicyProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PolicyProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PolicyProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PolicyProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut groups: Option<::ValueList<String>> = None;
                let mut policy_document: Option<::Value<::json::Value>> = None;
                let mut policy_name: Option<::Value<String>> = None;
                let mut roles: Option<::ValueList<String>> = None;
                let mut users: Option<::ValueList<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Groups" => {
                            groups = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PolicyDocument" => {
                            policy_document = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PolicyName" => {
                            policy_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Roles" => {
                            roles = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Users" => {
                            users = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PolicyProperties {
                    groups: groups,
                    policy_document: policy_document.ok_or(::serde::de::Error::missing_field("PolicyDocument"))?,
                    policy_name: policy_name.ok_or(::serde::de::Error::missing_field("PolicyName"))?,
                    roles: roles,
                    users: users,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Policy {
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
#[derive(Debug, Default)]
pub struct Role {
    properties: RoleProperties
}

/// Properties for the `Role` resource.
#[derive(Debug, Default)]
pub struct RoleProperties {
    /// Property [`AssumeRolePolicyDocument`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-role.html#cfn-iam-role-assumerolepolicydocument).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub assume_role_policy_document: ::Value<::json::Value>,
    /// Property [`ManagedPolicyArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-role.html#cfn-iam-role-managepolicyarns).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub managed_policy_arns: Option<::ValueList<String>>,
    /// Property [`Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-role.html#cfn-iam-role-path).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub path: Option<::Value<String>>,
    /// Property [`Policies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-role.html#cfn-iam-role-policies).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub policies: Option<::ValueList<self::role::Policy>>,
    /// Property [`RoleName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iam-role.html#cfn-iam-role-rolename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub role_name: Option<::Value<String>>,
}

impl ::serde::Serialize for RoleProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssumeRolePolicyDocument", &self.assume_role_policy_document)?;
        if let Some(ref managed_policy_arns) = self.managed_policy_arns {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManagedPolicyArns", managed_policy_arns)?;
        }
        if let Some(ref path) = self.path {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", path)?;
        }
        if let Some(ref policies) = self.policies {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Policies", policies)?;
        }
        if let Some(ref role_name) = self.role_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleName", role_name)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for RoleProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<RoleProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RoleProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type RoleProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut assume_role_policy_document: Option<::Value<::json::Value>> = None;
                let mut managed_policy_arns: Option<::ValueList<String>> = None;
                let mut path: Option<::Value<String>> = None;
                let mut policies: Option<::ValueList<self::role::Policy>> = None;
                let mut role_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AssumeRolePolicyDocument" => {
                            assume_role_policy_document = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ManagedPolicyArns" => {
                            managed_policy_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Path" => {
                            path = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Policies" => {
                            policies = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleName" => {
                            role_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(RoleProperties {
                    assume_role_policy_document: assume_role_policy_document.ok_or(::serde::de::Error::missing_field("AssumeRolePolicyDocument"))?,
                    managed_policy_arns: managed_policy_arns,
                    path: path,
                    policies: policies,
                    role_name: role_name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Role {
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
#[derive(Debug, Default)]
pub struct User {
    properties: UserProperties
}

/// Properties for the `User` resource.
#[derive(Debug, Default)]
pub struct UserProperties {
    /// Property [`Groups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-user.html#cfn-iam-user-groups).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub groups: Option<::ValueList<String>>,
    /// Property [`LoginProfile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-user.html#cfn-iam-user-loginprofile).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub login_profile: Option<::Value<self::user::LoginProfile>>,
    /// Property [`ManagedPolicyArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-user.html#cfn-iam-user-managepolicyarns).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub managed_policy_arns: Option<::ValueList<String>>,
    /// Property [`Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-user.html#cfn-iam-user-path).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub path: Option<::Value<String>>,
    /// Property [`Policies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-user.html#cfn-iam-user-policies).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub policies: Option<::ValueList<self::user::Policy>>,
    /// Property [`UserName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-user.html#cfn-iam-user-username).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub user_name: Option<::Value<String>>,
}

impl ::serde::Serialize for UserProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref groups) = self.groups {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Groups", groups)?;
        }
        if let Some(ref login_profile) = self.login_profile {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoginProfile", login_profile)?;
        }
        if let Some(ref managed_policy_arns) = self.managed_policy_arns {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManagedPolicyArns", managed_policy_arns)?;
        }
        if let Some(ref path) = self.path {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", path)?;
        }
        if let Some(ref policies) = self.policies {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Policies", policies)?;
        }
        if let Some(ref user_name) = self.user_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserName", user_name)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for UserProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<UserProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = UserProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type UserProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut groups: Option<::ValueList<String>> = None;
                let mut login_profile: Option<::Value<self::user::LoginProfile>> = None;
                let mut managed_policy_arns: Option<::ValueList<String>> = None;
                let mut path: Option<::Value<String>> = None;
                let mut policies: Option<::ValueList<self::user::Policy>> = None;
                let mut user_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Groups" => {
                            groups = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LoginProfile" => {
                            login_profile = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ManagedPolicyArns" => {
                            managed_policy_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Path" => {
                            path = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Policies" => {
                            policies = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserName" => {
                            user_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(UserProperties {
                    groups: groups,
                    login_profile: login_profile,
                    managed_policy_arns: managed_policy_arns,
                    path: path,
                    policies: policies,
                    user_name: user_name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for User {
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
#[derive(Debug, Default)]
pub struct UserToGroupAddition {
    properties: UserToGroupAdditionProperties
}

/// Properties for the `UserToGroupAddition` resource.
#[derive(Debug, Default)]
pub struct UserToGroupAdditionProperties {
    /// Property [`GroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-addusertogroup.html#cfn-iam-addusertogroup-groupname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub group_name: ::Value<String>,
    /// Property [`Users`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-addusertogroup.html#cfn-iam-addusertogroup-users).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub users: ::ValueList<String>,
}

impl ::serde::Serialize for UserToGroupAdditionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupName", &self.group_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Users", &self.users)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for UserToGroupAdditionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<UserToGroupAdditionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = UserToGroupAdditionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type UserToGroupAdditionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut group_name: Option<::Value<String>> = None;
                let mut users: Option<::ValueList<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "GroupName" => {
                            group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Users" => {
                            users = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(UserToGroupAdditionProperties {
                    group_name: group_name.ok_or(::serde::de::Error::missing_field("GroupName"))?,
                    users: users.ok_or(::serde::de::Error::missing_field("Users"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for UserToGroupAddition {
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
    #[derive(Debug, Default)]
    pub struct Policy {
        /// Property [`PolicyDocument`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-policy.html#cfn-iam-policies-policydocument).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub policy_document: ::Value<::json::Value>,
        /// Property [`PolicyName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-policy.html#cfn-iam-policies-policyname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub policy_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for Policy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyDocument", &self.policy_document)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyName", &self.policy_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Policy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Policy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Policy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Policy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut policy_document: Option<::Value<::json::Value>> = None;
                    let mut policy_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PolicyDocument" => {
                                policy_document = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PolicyName" => {
                                policy_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Policy {
                        policy_document: policy_document.ok_or(::serde::de::Error::missing_field("PolicyDocument"))?,
                        policy_name: policy_name.ok_or(::serde::de::Error::missing_field("PolicyName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod role {
    //! Property types for the `Role` resource.

    /// The [`AWS::IAM::Role.Policy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-policy.html) property type.
    #[derive(Debug, Default)]
    pub struct Policy {
        /// Property [`PolicyDocument`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-policy.html#cfn-iam-policies-policydocument).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub policy_document: ::Value<::json::Value>,
        /// Property [`PolicyName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-policy.html#cfn-iam-policies-policyname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub policy_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for Policy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyDocument", &self.policy_document)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyName", &self.policy_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Policy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Policy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Policy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Policy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut policy_document: Option<::Value<::json::Value>> = None;
                    let mut policy_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PolicyDocument" => {
                                policy_document = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PolicyName" => {
                                policy_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Policy {
                        policy_document: policy_document.ok_or(::serde::de::Error::missing_field("PolicyDocument"))?,
                        policy_name: policy_name.ok_or(::serde::de::Error::missing_field("PolicyName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod user {
    //! Property types for the `User` resource.

    /// The [`AWS::IAM::User.LoginProfile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-user-loginprofile.html) property type.
    #[derive(Debug, Default)]
    pub struct LoginProfile {
        /// Property [`Password`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-user-loginprofile.html#cfn-iam-user-loginprofile-password).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub password: ::Value<String>,
        /// Property [`PasswordResetRequired`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-user-loginprofile.html#cfn-iam-user-loginprofile-passwordresetrequired).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub password_reset_required: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for LoginProfile {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Password", &self.password)?;
            if let Some(ref password_reset_required) = self.password_reset_required {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PasswordResetRequired", password_reset_required)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LoginProfile {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LoginProfile, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LoginProfile;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LoginProfile")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut password: Option<::Value<String>> = None;
                    let mut password_reset_required: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Password" => {
                                password = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PasswordResetRequired" => {
                                password_reset_required = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LoginProfile {
                        password: password.ok_or(::serde::de::Error::missing_field("Password"))?,
                        password_reset_required: password_reset_required,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IAM::User.Policy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-policy.html) property type.
    #[derive(Debug, Default)]
    pub struct Policy {
        /// Property [`PolicyDocument`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-policy.html#cfn-iam-policies-policydocument).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub policy_document: ::Value<::json::Value>,
        /// Property [`PolicyName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iam-policy.html#cfn-iam-policies-policyname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub policy_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for Policy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyDocument", &self.policy_document)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyName", &self.policy_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Policy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Policy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Policy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Policy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut policy_document: Option<::Value<::json::Value>> = None;
                    let mut policy_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PolicyDocument" => {
                                policy_document = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PolicyName" => {
                                policy_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Policy {
                        policy_document: policy_document.ok_or(::serde::de::Error::missing_field("PolicyDocument"))?,
                        policy_name: policy_name.ok_or(::serde::de::Error::missing_field("PolicyName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
