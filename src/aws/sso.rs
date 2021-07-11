//! Types for the `SSO` service.

/// The [`AWS::SSO::Assignment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sso-assignment.html) resource type.
#[derive(Debug, Default)]
pub struct Assignment {
    properties: AssignmentProperties
}

/// Properties for the `Assignment` resource.
#[derive(Debug, Default)]
pub struct AssignmentProperties {
    /// Property [`InstanceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sso-assignment.html#cfn-sso-assignment-instancearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub instance_arn: ::Value<String>,
    /// Property [`PermissionSetArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sso-assignment.html#cfn-sso-assignment-permissionsetarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub permission_set_arn: ::Value<String>,
    /// Property [`PrincipalId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sso-assignment.html#cfn-sso-assignment-principalid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub principal_id: ::Value<String>,
    /// Property [`PrincipalType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sso-assignment.html#cfn-sso-assignment-principaltype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub principal_type: ::Value<String>,
    /// Property [`TargetId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sso-assignment.html#cfn-sso-assignment-targetid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub target_id: ::Value<String>,
    /// Property [`TargetType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sso-assignment.html#cfn-sso-assignment-targettype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub target_type: ::Value<String>,
}

impl ::serde::Serialize for AssignmentProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceArn", &self.instance_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PermissionSetArn", &self.permission_set_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrincipalId", &self.principal_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrincipalType", &self.principal_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetId", &self.target_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetType", &self.target_type)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AssignmentProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AssignmentProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AssignmentProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AssignmentProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut instance_arn: Option<::Value<String>> = None;
                let mut permission_set_arn: Option<::Value<String>> = None;
                let mut principal_id: Option<::Value<String>> = None;
                let mut principal_type: Option<::Value<String>> = None;
                let mut target_id: Option<::Value<String>> = None;
                let mut target_type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "InstanceArn" => {
                            instance_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PermissionSetArn" => {
                            permission_set_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PrincipalId" => {
                            principal_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PrincipalType" => {
                            principal_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetId" => {
                            target_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetType" => {
                            target_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AssignmentProperties {
                    instance_arn: instance_arn.ok_or(::serde::de::Error::missing_field("InstanceArn"))?,
                    permission_set_arn: permission_set_arn.ok_or(::serde::de::Error::missing_field("PermissionSetArn"))?,
                    principal_id: principal_id.ok_or(::serde::de::Error::missing_field("PrincipalId"))?,
                    principal_type: principal_type.ok_or(::serde::de::Error::missing_field("PrincipalType"))?,
                    target_id: target_id.ok_or(::serde::de::Error::missing_field("TargetId"))?,
                    target_type: target_type.ok_or(::serde::de::Error::missing_field("TargetType"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Assignment {
    type Properties = AssignmentProperties;
    const TYPE: &'static str = "AWS::SSO::Assignment";
    fn properties(&self) -> &AssignmentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AssignmentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Assignment {}

impl From<AssignmentProperties> for Assignment {
    fn from(properties: AssignmentProperties) -> Assignment {
        Assignment { properties }
    }
}

/// The [`AWS::SSO::InstanceAccessControlAttributeConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sso-instanceaccesscontrolattributeconfiguration.html) resource type.
#[derive(Debug, Default)]
pub struct InstanceAccessControlAttributeConfiguration {
    properties: InstanceAccessControlAttributeConfigurationProperties
}

/// Properties for the `InstanceAccessControlAttributeConfiguration` resource.
#[derive(Debug, Default)]
pub struct InstanceAccessControlAttributeConfigurationProperties {
    /// Property [`AccessControlAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sso-instanceaccesscontrolattributeconfiguration.html#cfn-sso-instanceaccesscontrolattributeconfiguration-accesscontrolattributes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub access_control_attributes: Option<::ValueList<self::instance_access_control_attribute_configuration::AccessControlAttribute>>,
    /// Property [`InstanceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sso-instanceaccesscontrolattributeconfiguration.html#cfn-sso-instanceaccesscontrolattributeconfiguration-instancearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub instance_arn: ::Value<String>,
}

impl ::serde::Serialize for InstanceAccessControlAttributeConfigurationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref access_control_attributes) = self.access_control_attributes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessControlAttributes", access_control_attributes)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceArn", &self.instance_arn)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for InstanceAccessControlAttributeConfigurationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<InstanceAccessControlAttributeConfigurationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = InstanceAccessControlAttributeConfigurationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type InstanceAccessControlAttributeConfigurationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut access_control_attributes: Option<::ValueList<self::instance_access_control_attribute_configuration::AccessControlAttribute>> = None;
                let mut instance_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AccessControlAttributes" => {
                            access_control_attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceArn" => {
                            instance_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(InstanceAccessControlAttributeConfigurationProperties {
                    access_control_attributes: access_control_attributes,
                    instance_arn: instance_arn.ok_or(::serde::de::Error::missing_field("InstanceArn"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for InstanceAccessControlAttributeConfiguration {
    type Properties = InstanceAccessControlAttributeConfigurationProperties;
    const TYPE: &'static str = "AWS::SSO::InstanceAccessControlAttributeConfiguration";
    fn properties(&self) -> &InstanceAccessControlAttributeConfigurationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut InstanceAccessControlAttributeConfigurationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for InstanceAccessControlAttributeConfiguration {}

impl From<InstanceAccessControlAttributeConfigurationProperties> for InstanceAccessControlAttributeConfiguration {
    fn from(properties: InstanceAccessControlAttributeConfigurationProperties) -> InstanceAccessControlAttributeConfiguration {
        InstanceAccessControlAttributeConfiguration { properties }
    }
}

/// The [`AWS::SSO::PermissionSet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sso-permissionset.html) resource type.
#[derive(Debug, Default)]
pub struct PermissionSet {
    properties: PermissionSetProperties
}

/// Properties for the `PermissionSet` resource.
#[derive(Debug, Default)]
pub struct PermissionSetProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sso-permissionset.html#cfn-sso-permissionset-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`InlinePolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sso-permissionset.html#cfn-sso-permissionset-inlinepolicy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub inline_policy: Option<::Value<::json::Value>>,
    /// Property [`InstanceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sso-permissionset.html#cfn-sso-permissionset-instancearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub instance_arn: ::Value<String>,
    /// Property [`ManagedPolicies`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sso-permissionset.html#cfn-sso-permissionset-managedpolicies).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub managed_policies: Option<::ValueList<String>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sso-permissionset.html#cfn-sso-permissionset-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`RelayStateType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sso-permissionset.html#cfn-sso-permissionset-relaystatetype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub relay_state_type: Option<::Value<String>>,
    /// Property [`SessionDuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sso-permissionset.html#cfn-sso-permissionset-sessionduration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub session_duration: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sso-permissionset.html#cfn-sso-permissionset-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for PermissionSetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref inline_policy) = self.inline_policy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InlinePolicy", inline_policy)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceArn", &self.instance_arn)?;
        if let Some(ref managed_policies) = self.managed_policies {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ManagedPolicies", managed_policies)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref relay_state_type) = self.relay_state_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RelayStateType", relay_state_type)?;
        }
        if let Some(ref session_duration) = self.session_duration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SessionDuration", session_duration)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PermissionSetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PermissionSetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PermissionSetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PermissionSetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut inline_policy: Option<::Value<::json::Value>> = None;
                let mut instance_arn: Option<::Value<String>> = None;
                let mut managed_policies: Option<::ValueList<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut relay_state_type: Option<::Value<String>> = None;
                let mut session_duration: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InlinePolicy" => {
                            inline_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceArn" => {
                            instance_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ManagedPolicies" => {
                            managed_policies = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RelayStateType" => {
                            relay_state_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SessionDuration" => {
                            session_duration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PermissionSetProperties {
                    description: description,
                    inline_policy: inline_policy,
                    instance_arn: instance_arn.ok_or(::serde::de::Error::missing_field("InstanceArn"))?,
                    managed_policies: managed_policies,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    relay_state_type: relay_state_type,
                    session_duration: session_duration,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for PermissionSet {
    type Properties = PermissionSetProperties;
    const TYPE: &'static str = "AWS::SSO::PermissionSet";
    fn properties(&self) -> &PermissionSetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PermissionSetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for PermissionSet {}

impl From<PermissionSetProperties> for PermissionSet {
    fn from(properties: PermissionSetProperties) -> PermissionSet {
        PermissionSet { properties }
    }
}

pub mod instance_access_control_attribute_configuration {
    //! Property types for the `InstanceAccessControlAttributeConfiguration` resource.

    /// The [`AWS::SSO::InstanceAccessControlAttributeConfiguration.AccessControlAttribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sso-instanceaccesscontrolattributeconfiguration-accesscontrolattribute.html) property type.
    #[derive(Debug, Default)]
    pub struct AccessControlAttribute {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sso-instanceaccesscontrolattributeconfiguration-accesscontrolattribute.html#cfn-sso-instanceaccesscontrolattributeconfiguration-accesscontrolattribute-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sso-instanceaccesscontrolattributeconfiguration-accesscontrolattribute.html#cfn-sso-instanceaccesscontrolattributeconfiguration-accesscontrolattribute-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<AccessControlAttributeValue>,
    }

    impl ::codec::SerializeValue for AccessControlAttribute {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AccessControlAttribute {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AccessControlAttribute, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AccessControlAttribute;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AccessControlAttribute")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut value: Option<::Value<AccessControlAttributeValue>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AccessControlAttribute {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSO::InstanceAccessControlAttributeConfiguration.AccessControlAttributeValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sso-instanceaccesscontrolattributeconfiguration-accesscontrolattributevalue.html) property type.
    #[derive(Debug, Default)]
    pub struct AccessControlAttributeValue {
        /// Property [`Source`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sso-instanceaccesscontrolattributeconfiguration-accesscontrolattributevalue.html#cfn-sso-instanceaccesscontrolattributeconfiguration-accesscontrolattributevalue-source).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub source: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for AccessControlAttributeValue {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Source", &self.source)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AccessControlAttributeValue {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AccessControlAttributeValue, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AccessControlAttributeValue;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AccessControlAttributeValue")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut source: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Source" => {
                                source = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AccessControlAttributeValue {
                        source: source.ok_or(::serde::de::Error::missing_field("Source"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
