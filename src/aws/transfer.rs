//! Types for the `Transfer` service.

/// The [`AWS::Transfer::Server`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-server.html) resource type.
#[derive(Debug, Default)]
pub struct Server {
    properties: ServerProperties
}

/// Properties for the `Server` resource.
#[derive(Debug, Default)]
pub struct ServerProperties {
    /// Property [`Certificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-server.html#cfn-transfer-server-certificate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub certificate: Option<::Value<String>>,
    /// Property [`Domain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-server.html#cfn-transfer-server-domain).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub domain: Option<::Value<String>>,
    /// Property [`EndpointDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-server.html#cfn-transfer-server-endpointdetails).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub endpoint_details: Option<::Value<self::server::EndpointDetails>>,
    /// Property [`EndpointType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-server.html#cfn-transfer-server-endpointtype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub endpoint_type: Option<::Value<String>>,
    /// Property [`IdentityProviderDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-server.html#cfn-transfer-server-identityproviderdetails).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub identity_provider_details: Option<::Value<self::server::IdentityProviderDetails>>,
    /// Property [`IdentityProviderType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-server.html#cfn-transfer-server-identityprovidertype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub identity_provider_type: Option<::Value<String>>,
    /// Property [`LoggingRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-server.html#cfn-transfer-server-loggingrole).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub logging_role: Option<::Value<String>>,
    /// Property [`PostAuthenticationLoginBanner`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-server.html#cfn-transfer-server-postauthenticationloginbanner).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub post_authentication_login_banner: Option<::Value<String>>,
    /// Property [`PreAuthenticationLoginBanner`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-server.html#cfn-transfer-server-preauthenticationloginbanner).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub pre_authentication_login_banner: Option<::Value<String>>,
    /// Property [`ProtocolDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-server.html#cfn-transfer-server-protocoldetails).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub protocol_details: Option<::Value<self::server::ProtocolDetails>>,
    /// Property [`Protocols`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-server.html#cfn-transfer-server-protocols).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub protocols: Option<::ValueList<self::server::Protocol>>,
    /// Property [`SecurityPolicyName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-server.html#cfn-transfer-server-securitypolicyname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub security_policy_name: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-server.html#cfn-transfer-server-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`WorkflowDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-server.html#cfn-transfer-server-workflowdetails).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub workflow_details: Option<::Value<self::server::WorkflowDetails>>,
}

impl ::serde::Serialize for ServerProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref certificate) = self.certificate {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Certificate", certificate)?;
        }
        if let Some(ref domain) = self.domain {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Domain", domain)?;
        }
        if let Some(ref endpoint_details) = self.endpoint_details {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointDetails", endpoint_details)?;
        }
        if let Some(ref endpoint_type) = self.endpoint_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointType", endpoint_type)?;
        }
        if let Some(ref identity_provider_details) = self.identity_provider_details {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdentityProviderDetails", identity_provider_details)?;
        }
        if let Some(ref identity_provider_type) = self.identity_provider_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdentityProviderType", identity_provider_type)?;
        }
        if let Some(ref logging_role) = self.logging_role {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoggingRole", logging_role)?;
        }
        if let Some(ref post_authentication_login_banner) = self.post_authentication_login_banner {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PostAuthenticationLoginBanner", post_authentication_login_banner)?;
        }
        if let Some(ref pre_authentication_login_banner) = self.pre_authentication_login_banner {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreAuthenticationLoginBanner", pre_authentication_login_banner)?;
        }
        if let Some(ref protocol_details) = self.protocol_details {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProtocolDetails", protocol_details)?;
        }
        if let Some(ref protocols) = self.protocols {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Protocols", protocols)?;
        }
        if let Some(ref security_policy_name) = self.security_policy_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityPolicyName", security_policy_name)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref workflow_details) = self.workflow_details {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkflowDetails", workflow_details)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ServerProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ServerProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ServerProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ServerProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut certificate: Option<::Value<String>> = None;
                let mut domain: Option<::Value<String>> = None;
                let mut endpoint_details: Option<::Value<self::server::EndpointDetails>> = None;
                let mut endpoint_type: Option<::Value<String>> = None;
                let mut identity_provider_details: Option<::Value<self::server::IdentityProviderDetails>> = None;
                let mut identity_provider_type: Option<::Value<String>> = None;
                let mut logging_role: Option<::Value<String>> = None;
                let mut post_authentication_login_banner: Option<::Value<String>> = None;
                let mut pre_authentication_login_banner: Option<::Value<String>> = None;
                let mut protocol_details: Option<::Value<self::server::ProtocolDetails>> = None;
                let mut protocols: Option<::ValueList<self::server::Protocol>> = None;
                let mut security_policy_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut workflow_details: Option<::Value<self::server::WorkflowDetails>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Certificate" => {
                            certificate = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Domain" => {
                            domain = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EndpointDetails" => {
                            endpoint_details = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EndpointType" => {
                            endpoint_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IdentityProviderDetails" => {
                            identity_provider_details = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IdentityProviderType" => {
                            identity_provider_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LoggingRole" => {
                            logging_role = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PostAuthenticationLoginBanner" => {
                            post_authentication_login_banner = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PreAuthenticationLoginBanner" => {
                            pre_authentication_login_banner = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProtocolDetails" => {
                            protocol_details = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Protocols" => {
                            protocols = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecurityPolicyName" => {
                            security_policy_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "WorkflowDetails" => {
                            workflow_details = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ServerProperties {
                    certificate: certificate,
                    domain: domain,
                    endpoint_details: endpoint_details,
                    endpoint_type: endpoint_type,
                    identity_provider_details: identity_provider_details,
                    identity_provider_type: identity_provider_type,
                    logging_role: logging_role,
                    post_authentication_login_banner: post_authentication_login_banner,
                    pre_authentication_login_banner: pre_authentication_login_banner,
                    protocol_details: protocol_details,
                    protocols: protocols,
                    security_policy_name: security_policy_name,
                    tags: tags,
                    workflow_details: workflow_details,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Server {
    type Properties = ServerProperties;
    const TYPE: &'static str = "AWS::Transfer::Server";
    fn properties(&self) -> &ServerProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ServerProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Server {}

impl From<ServerProperties> for Server {
    fn from(properties: ServerProperties) -> Server {
        Server { properties }
    }
}

/// The [`AWS::Transfer::User`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-user.html) resource type.
#[derive(Debug, Default)]
pub struct User {
    properties: UserProperties
}

/// Properties for the `User` resource.
#[derive(Debug, Default)]
pub struct UserProperties {
    /// Property [`HomeDirectory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-user.html#cfn-transfer-user-homedirectory).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub home_directory: Option<::Value<String>>,
    /// Property [`HomeDirectoryMappings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-user.html#cfn-transfer-user-homedirectorymappings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub home_directory_mappings: Option<::ValueList<self::user::HomeDirectoryMapEntry>>,
    /// Property [`HomeDirectoryType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-user.html#cfn-transfer-user-homedirectorytype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub home_directory_type: Option<::Value<String>>,
    /// Property [`Policy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-user.html#cfn-transfer-user-policy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub policy: Option<::Value<String>>,
    /// Property [`PosixProfile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-user.html#cfn-transfer-user-posixprofile).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub posix_profile: Option<::Value<self::user::PosixProfile>>,
    /// Property [`Role`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-user.html#cfn-transfer-user-role).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role: ::Value<String>,
    /// Property [`ServerId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-user.html#cfn-transfer-user-serverid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub server_id: ::Value<String>,
    /// Property [`SshPublicKeys`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-user.html#cfn-transfer-user-sshpublickeys).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ssh_public_keys: Option<::ValueList<self::user::SshPublicKey>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-user.html#cfn-transfer-user-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`UserName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-user.html#cfn-transfer-user-username).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub user_name: ::Value<String>,
}

impl ::serde::Serialize for UserProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref home_directory) = self.home_directory {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HomeDirectory", home_directory)?;
        }
        if let Some(ref home_directory_mappings) = self.home_directory_mappings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HomeDirectoryMappings", home_directory_mappings)?;
        }
        if let Some(ref home_directory_type) = self.home_directory_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HomeDirectoryType", home_directory_type)?;
        }
        if let Some(ref policy) = self.policy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Policy", policy)?;
        }
        if let Some(ref posix_profile) = self.posix_profile {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PosixProfile", posix_profile)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Role", &self.role)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerId", &self.server_id)?;
        if let Some(ref ssh_public_keys) = self.ssh_public_keys {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SshPublicKeys", ssh_public_keys)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserName", &self.user_name)?;
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
                let mut home_directory: Option<::Value<String>> = None;
                let mut home_directory_mappings: Option<::ValueList<self::user::HomeDirectoryMapEntry>> = None;
                let mut home_directory_type: Option<::Value<String>> = None;
                let mut policy: Option<::Value<String>> = None;
                let mut posix_profile: Option<::Value<self::user::PosixProfile>> = None;
                let mut role: Option<::Value<String>> = None;
                let mut server_id: Option<::Value<String>> = None;
                let mut ssh_public_keys: Option<::ValueList<self::user::SshPublicKey>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut user_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "HomeDirectory" => {
                            home_directory = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HomeDirectoryMappings" => {
                            home_directory_mappings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HomeDirectoryType" => {
                            home_directory_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Policy" => {
                            policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PosixProfile" => {
                            posix_profile = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Role" => {
                            role = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServerId" => {
                            server_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SshPublicKeys" => {
                            ssh_public_keys = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserName" => {
                            user_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(UserProperties {
                    home_directory: home_directory,
                    home_directory_mappings: home_directory_mappings,
                    home_directory_type: home_directory_type,
                    policy: policy,
                    posix_profile: posix_profile,
                    role: role.ok_or(::serde::de::Error::missing_field("Role"))?,
                    server_id: server_id.ok_or(::serde::de::Error::missing_field("ServerId"))?,
                    ssh_public_keys: ssh_public_keys,
                    tags: tags,
                    user_name: user_name.ok_or(::serde::de::Error::missing_field("UserName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for User {
    type Properties = UserProperties;
    const TYPE: &'static str = "AWS::Transfer::User";
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

/// The [`AWS::Transfer::Workflow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-workflow.html) resource type.
#[derive(Debug, Default)]
pub struct Workflow {
    properties: WorkflowProperties
}

/// Properties for the `Workflow` resource.
#[derive(Debug, Default)]
pub struct WorkflowProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-workflow.html#cfn-transfer-workflow-description).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`OnExceptionSteps`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-workflow.html#cfn-transfer-workflow-onexceptionsteps).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub on_exception_steps: Option<::ValueList<self::workflow::WorkflowStep>>,
    /// Property [`Steps`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-workflow.html#cfn-transfer-workflow-steps).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub steps: ::ValueList<self::workflow::WorkflowStep>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-transfer-workflow.html#cfn-transfer-workflow-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for WorkflowProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref on_exception_steps) = self.on_exception_steps {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OnExceptionSteps", on_exception_steps)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Steps", &self.steps)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for WorkflowProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<WorkflowProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = WorkflowProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type WorkflowProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut on_exception_steps: Option<::ValueList<self::workflow::WorkflowStep>> = None;
                let mut steps: Option<::ValueList<self::workflow::WorkflowStep>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OnExceptionSteps" => {
                            on_exception_steps = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Steps" => {
                            steps = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(WorkflowProperties {
                    description: description,
                    on_exception_steps: on_exception_steps,
                    steps: steps.ok_or(::serde::de::Error::missing_field("Steps"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Workflow {
    type Properties = WorkflowProperties;
    const TYPE: &'static str = "AWS::Transfer::Workflow";
    fn properties(&self) -> &WorkflowProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut WorkflowProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Workflow {}

impl From<WorkflowProperties> for Workflow {
    fn from(properties: WorkflowProperties) -> Workflow {
        Workflow { properties }
    }
}

pub mod server {
    //! Property types for the `Server` resource.

    /// The [`AWS::Transfer::Server.EndpointDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-endpointdetails.html) property type.
    #[derive(Debug, Default)]
    pub struct EndpointDetails {
        /// Property [`AddressAllocationIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-endpointdetails.html#cfn-transfer-server-endpointdetails-addressallocationids).
        ///
        /// Update type: _Conditional_.
        /// Conditional updates can be mutable or immutable, depending on, for example, which other properties you updated.
        /// For more information, see the relevant resource type documentation.
        pub address_allocation_ids: Option<::ValueList<String>>,
        /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-endpointdetails.html#cfn-transfer-server-endpointdetails-securitygroupids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_group_ids: Option<::ValueList<String>>,
        /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-endpointdetails.html#cfn-transfer-server-endpointdetails-subnetids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subnet_ids: Option<::ValueList<String>>,
        /// Property [`VpcEndpointId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-endpointdetails.html#cfn-transfer-server-endpointdetails-vpcendpointid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vpc_endpoint_id: Option<::Value<String>>,
        /// Property [`VpcId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-endpointdetails.html#cfn-transfer-server-endpointdetails-vpcid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vpc_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EndpointDetails {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref address_allocation_ids) = self.address_allocation_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AddressAllocationIds", address_allocation_ids)?;
            }
            if let Some(ref security_group_ids) = self.security_group_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", security_group_ids)?;
            }
            if let Some(ref subnet_ids) = self.subnet_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", subnet_ids)?;
            }
            if let Some(ref vpc_endpoint_id) = self.vpc_endpoint_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcEndpointId", vpc_endpoint_id)?;
            }
            if let Some(ref vpc_id) = self.vpc_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcId", vpc_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EndpointDetails {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EndpointDetails, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EndpointDetails;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EndpointDetails")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut address_allocation_ids: Option<::ValueList<String>> = None;
                    let mut security_group_ids: Option<::ValueList<String>> = None;
                    let mut subnet_ids: Option<::ValueList<String>> = None;
                    let mut vpc_endpoint_id: Option<::Value<String>> = None;
                    let mut vpc_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AddressAllocationIds" => {
                                address_allocation_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecurityGroupIds" => {
                                security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubnetIds" => {
                                subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpcEndpointId" => {
                                vpc_endpoint_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpcId" => {
                                vpc_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EndpointDetails {
                        address_allocation_ids: address_allocation_ids,
                        security_group_ids: security_group_ids,
                        subnet_ids: subnet_ids,
                        vpc_endpoint_id: vpc_endpoint_id,
                        vpc_id: vpc_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Transfer::Server.IdentityProviderDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-identityproviderdetails.html) property type.
    #[derive(Debug, Default)]
    pub struct IdentityProviderDetails {
        /// Property [`DirectoryId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-identityproviderdetails.html#cfn-transfer-server-identityproviderdetails-directoryid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub directory_id: Option<::Value<String>>,
        /// Property [`Function`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-identityproviderdetails.html#cfn-transfer-server-identityproviderdetails-function).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub function: Option<::Value<String>>,
        /// Property [`InvocationRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-identityproviderdetails.html#cfn-transfer-server-identityproviderdetails-invocationrole).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub invocation_role: Option<::Value<String>>,
        /// Property [`Url`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-identityproviderdetails.html#cfn-transfer-server-identityproviderdetails-url).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub url: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for IdentityProviderDetails {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref directory_id) = self.directory_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DirectoryId", directory_id)?;
            }
            if let Some(ref function) = self.function {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Function", function)?;
            }
            if let Some(ref invocation_role) = self.invocation_role {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InvocationRole", invocation_role)?;
            }
            if let Some(ref url) = self.url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Url", url)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IdentityProviderDetails {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IdentityProviderDetails, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IdentityProviderDetails;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IdentityProviderDetails")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut directory_id: Option<::Value<String>> = None;
                    let mut function: Option<::Value<String>> = None;
                    let mut invocation_role: Option<::Value<String>> = None;
                    let mut url: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DirectoryId" => {
                                directory_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Function" => {
                                function = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InvocationRole" => {
                                invocation_role = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Url" => {
                                url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IdentityProviderDetails {
                        directory_id: directory_id,
                        function: function,
                        invocation_role: invocation_role,
                        url: url,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Transfer::Server.Protocol`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-protocol.html) property type.
    #[derive(Debug, Default)]
    pub struct Protocol {
    }

    impl ::codec::SerializeValue for Protocol {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Protocol {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Protocol, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Protocol;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Protocol")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(Protocol {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Transfer::Server.ProtocolDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-protocoldetails.html) property type.
    #[derive(Debug, Default)]
    pub struct ProtocolDetails {
        /// Property [`PassiveIp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-protocoldetails.html#cfn-transfer-server-protocoldetails-passiveip).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub passive_ip: Option<::Value<String>>,
        /// Property [`TlsSessionResumptionMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-protocoldetails.html#cfn-transfer-server-protocoldetails-tlssessionresumptionmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tls_session_resumption_mode: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ProtocolDetails {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref passive_ip) = self.passive_ip {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PassiveIp", passive_ip)?;
            }
            if let Some(ref tls_session_resumption_mode) = self.tls_session_resumption_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TlsSessionResumptionMode", tls_session_resumption_mode)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ProtocolDetails {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ProtocolDetails, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ProtocolDetails;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ProtocolDetails")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut passive_ip: Option<::Value<String>> = None;
                    let mut tls_session_resumption_mode: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PassiveIp" => {
                                passive_ip = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TlsSessionResumptionMode" => {
                                tls_session_resumption_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ProtocolDetails {
                        passive_ip: passive_ip,
                        tls_session_resumption_mode: tls_session_resumption_mode,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Transfer::Server.WorkflowDetail`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-workflowdetail.html) property type.
    #[derive(Debug, Default)]
    pub struct WorkflowDetail {
        /// Property [`ExecutionRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-workflowdetail.html#cfn-transfer-server-workflowdetail-executionrole).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub execution_role: ::Value<String>,
        /// Property [`WorkflowId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-workflowdetail.html#cfn-transfer-server-workflowdetail-workflowid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub workflow_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for WorkflowDetail {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExecutionRole", &self.execution_role)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkflowId", &self.workflow_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for WorkflowDetail {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<WorkflowDetail, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = WorkflowDetail;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type WorkflowDetail")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut execution_role: Option<::Value<String>> = None;
                    let mut workflow_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ExecutionRole" => {
                                execution_role = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WorkflowId" => {
                                workflow_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(WorkflowDetail {
                        execution_role: execution_role.ok_or(::serde::de::Error::missing_field("ExecutionRole"))?,
                        workflow_id: workflow_id.ok_or(::serde::de::Error::missing_field("WorkflowId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Transfer::Server.WorkflowDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-workflowdetails.html) property type.
    #[derive(Debug, Default)]
    pub struct WorkflowDetails {
        /// Property [`OnUpload`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-server-workflowdetails.html#cfn-transfer-server-workflowdetails-onupload).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub on_upload: ::ValueList<WorkflowDetail>,
    }

    impl ::codec::SerializeValue for WorkflowDetails {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OnUpload", &self.on_upload)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for WorkflowDetails {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<WorkflowDetails, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = WorkflowDetails;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type WorkflowDetails")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut on_upload: Option<::ValueList<WorkflowDetail>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "OnUpload" => {
                                on_upload = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(WorkflowDetails {
                        on_upload: on_upload.ok_or(::serde::de::Error::missing_field("OnUpload"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod user {
    //! Property types for the `User` resource.

    /// The [`AWS::Transfer::User.HomeDirectoryMapEntry`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-user-homedirectorymapentry.html) property type.
    #[derive(Debug, Default)]
    pub struct HomeDirectoryMapEntry {
        /// Property [`Entry`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-user-homedirectorymapentry.html#cfn-transfer-user-homedirectorymapentry-entry).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub entry: ::Value<String>,
        /// Property [`Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-user-homedirectorymapentry.html#cfn-transfer-user-homedirectorymapentry-target).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target: ::Value<String>,
    }

    impl ::codec::SerializeValue for HomeDirectoryMapEntry {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Entry", &self.entry)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Target", &self.target)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HomeDirectoryMapEntry {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HomeDirectoryMapEntry, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HomeDirectoryMapEntry;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HomeDirectoryMapEntry")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut entry: Option<::Value<String>> = None;
                    let mut target: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Entry" => {
                                entry = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Target" => {
                                target = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HomeDirectoryMapEntry {
                        entry: entry.ok_or(::serde::de::Error::missing_field("Entry"))?,
                        target: target.ok_or(::serde::de::Error::missing_field("Target"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Transfer::User.PosixProfile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-user-posixprofile.html) property type.
    #[derive(Debug, Default)]
    pub struct PosixProfile {
        /// Property [`Gid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-user-posixprofile.html#cfn-transfer-user-posixprofile-gid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub gid: ::Value<f64>,
        /// Property [`SecondaryGids`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-user-posixprofile.html#cfn-transfer-user-posixprofile-secondarygids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secondary_gids: Option<::ValueList<f64>>,
        /// Property [`Uid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-user-posixprofile.html#cfn-transfer-user-posixprofile-uid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub uid: ::Value<f64>,
    }

    impl ::codec::SerializeValue for PosixProfile {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Gid", &self.gid)?;
            if let Some(ref secondary_gids) = self.secondary_gids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecondaryGids", secondary_gids)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Uid", &self.uid)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PosixProfile {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PosixProfile, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PosixProfile;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PosixProfile")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut gid: Option<::Value<f64>> = None;
                    let mut secondary_gids: Option<::ValueList<f64>> = None;
                    let mut uid: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Gid" => {
                                gid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecondaryGids" => {
                                secondary_gids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Uid" => {
                                uid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PosixProfile {
                        gid: gid.ok_or(::serde::de::Error::missing_field("Gid"))?,
                        secondary_gids: secondary_gids,
                        uid: uid.ok_or(::serde::de::Error::missing_field("Uid"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Transfer::User.SshPublicKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-user-sshpublickey.html) property type.
    #[derive(Debug, Default)]
    pub struct SshPublicKey {
    }

    impl ::codec::SerializeValue for SshPublicKey {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SshPublicKey {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SshPublicKey, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SshPublicKey;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SshPublicKey")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(SshPublicKey {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod workflow {
    //! Property types for the `Workflow` resource.

    /// The [`AWS::Transfer::Workflow.WorkflowStep`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-workflowstep.html) property type.
    #[derive(Debug, Default)]
    pub struct WorkflowStep {
        /// Property [`CopyStepDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-workflowstep.html#cfn-transfer-workflow-workflowstep-copystepdetails).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub copy_step_details: Option<::Value<::json::Value>>,
        /// Property [`CustomStepDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-workflowstep.html#cfn-transfer-workflow-workflowstep-customstepdetails).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub custom_step_details: Option<::Value<::json::Value>>,
        /// Property [`DeleteStepDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-workflowstep.html#cfn-transfer-workflow-workflowstep-deletestepdetails).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub delete_step_details: Option<::Value<::json::Value>>,
        /// Property [`TagStepDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-workflowstep.html#cfn-transfer-workflow-workflowstep-tagstepdetails).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub tag_step_details: Option<::Value<::json::Value>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-transfer-workflow-workflowstep.html#cfn-transfer-workflow-workflowstep-type).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub r#type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for WorkflowStep {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref copy_step_details) = self.copy_step_details {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CopyStepDetails", copy_step_details)?;
            }
            if let Some(ref custom_step_details) = self.custom_step_details {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomStepDetails", custom_step_details)?;
            }
            if let Some(ref delete_step_details) = self.delete_step_details {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeleteStepDetails", delete_step_details)?;
            }
            if let Some(ref tag_step_details) = self.tag_step_details {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TagStepDetails", tag_step_details)?;
            }
            if let Some(ref r#type) = self.r#type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", r#type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for WorkflowStep {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<WorkflowStep, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = WorkflowStep;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type WorkflowStep")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut copy_step_details: Option<::Value<::json::Value>> = None;
                    let mut custom_step_details: Option<::Value<::json::Value>> = None;
                    let mut delete_step_details: Option<::Value<::json::Value>> = None;
                    let mut tag_step_details: Option<::Value<::json::Value>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CopyStepDetails" => {
                                copy_step_details = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CustomStepDetails" => {
                                custom_step_details = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeleteStepDetails" => {
                                delete_step_details = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TagStepDetails" => {
                                tag_step_details = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(WorkflowStep {
                        copy_step_details: copy_step_details,
                        custom_step_details: custom_step_details,
                        delete_step_details: delete_step_details,
                        tag_step_details: tag_step_details,
                        r#type: r#type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
