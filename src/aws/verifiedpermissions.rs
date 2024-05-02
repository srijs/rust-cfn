//! Types for the `VerifiedPermissions` service.

/// The [`AWS::VerifiedPermissions::IdentitySource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-verifiedpermissions-identitysource.html) resource type.
#[derive(Debug, Default)]
pub struct IdentitySource {
    properties: IdentitySourceProperties
}

/// Properties for the `IdentitySource` resource.
#[derive(Debug, Default)]
pub struct IdentitySourceProperties {
    /// Property [`Configuration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-verifiedpermissions-identitysource.html#cfn-verifiedpermissions-identitysource-configuration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub configuration: ::Value<self::identity_source::IdentitySourceConfiguration>,
    /// Property [`PolicyStoreId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-verifiedpermissions-identitysource.html#cfn-verifiedpermissions-identitysource-policystoreid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub policy_store_id: Option<::Value<String>>,
    /// Property [`PrincipalEntityType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-verifiedpermissions-identitysource.html#cfn-verifiedpermissions-identitysource-principalentitytype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub principal_entity_type: Option<::Value<String>>,
}

impl ::serde::Serialize for IdentitySourceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Configuration", &self.configuration)?;
        if let Some(ref policy_store_id) = self.policy_store_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyStoreId", policy_store_id)?;
        }
        if let Some(ref principal_entity_type) = self.principal_entity_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrincipalEntityType", principal_entity_type)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for IdentitySourceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<IdentitySourceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = IdentitySourceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type IdentitySourceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut configuration: Option<::Value<self::identity_source::IdentitySourceConfiguration>> = None;
                let mut policy_store_id: Option<::Value<String>> = None;
                let mut principal_entity_type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Configuration" => {
                            configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PolicyStoreId" => {
                            policy_store_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PrincipalEntityType" => {
                            principal_entity_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(IdentitySourceProperties {
                    configuration: configuration.ok_or(::serde::de::Error::missing_field("Configuration"))?,
                    policy_store_id: policy_store_id,
                    principal_entity_type: principal_entity_type,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for IdentitySource {
    type Properties = IdentitySourceProperties;
    const TYPE: &'static str = "AWS::VerifiedPermissions::IdentitySource";
    fn properties(&self) -> &IdentitySourceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut IdentitySourceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for IdentitySource {}

impl From<IdentitySourceProperties> for IdentitySource {
    fn from(properties: IdentitySourceProperties) -> IdentitySource {
        IdentitySource { properties }
    }
}

/// The [`AWS::VerifiedPermissions::Policy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-verifiedpermissions-policy.html) resource type.
#[derive(Debug, Default)]
pub struct Policy {
    properties: PolicyProperties
}

/// Properties for the `Policy` resource.
#[derive(Debug, Default)]
pub struct PolicyProperties {
    /// Property [`Definition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-verifiedpermissions-policy.html#cfn-verifiedpermissions-policy-definition).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub definition: ::Value<self::policy::PolicyDefinition>,
    /// Property [`PolicyStoreId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-verifiedpermissions-policy.html#cfn-verifiedpermissions-policy-policystoreid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub policy_store_id: ::Value<String>,
}

impl ::serde::Serialize for PolicyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Definition", &self.definition)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyStoreId", &self.policy_store_id)?;
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
                let mut definition: Option<::Value<self::policy::PolicyDefinition>> = None;
                let mut policy_store_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Definition" => {
                            definition = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PolicyStoreId" => {
                            policy_store_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PolicyProperties {
                    definition: definition.ok_or(::serde::de::Error::missing_field("Definition"))?,
                    policy_store_id: policy_store_id.ok_or(::serde::de::Error::missing_field("PolicyStoreId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Policy {
    type Properties = PolicyProperties;
    const TYPE: &'static str = "AWS::VerifiedPermissions::Policy";
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

/// The [`AWS::VerifiedPermissions::PolicyStore`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-verifiedpermissions-policystore.html) resource type.
#[derive(Debug, Default)]
pub struct PolicyStore {
    properties: PolicyStoreProperties
}

/// Properties for the `PolicyStore` resource.
#[derive(Debug, Default)]
pub struct PolicyStoreProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-verifiedpermissions-policystore.html#cfn-verifiedpermissions-policystore-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Schema`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-verifiedpermissions-policystore.html#cfn-verifiedpermissions-policystore-schema).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub schema: Option<::Value<self::policy_store::SchemaDefinition>>,
    /// Property [`ValidationSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-verifiedpermissions-policystore.html#cfn-verifiedpermissions-policystore-validationsettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub validation_settings: ::Value<self::policy_store::ValidationSettings>,
}

impl ::serde::Serialize for PolicyStoreProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref schema) = self.schema {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Schema", schema)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ValidationSettings", &self.validation_settings)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PolicyStoreProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PolicyStoreProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PolicyStoreProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PolicyStoreProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut schema: Option<::Value<self::policy_store::SchemaDefinition>> = None;
                let mut validation_settings: Option<::Value<self::policy_store::ValidationSettings>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Schema" => {
                            schema = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ValidationSettings" => {
                            validation_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PolicyStoreProperties {
                    description: description,
                    schema: schema,
                    validation_settings: validation_settings.ok_or(::serde::de::Error::missing_field("ValidationSettings"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for PolicyStore {
    type Properties = PolicyStoreProperties;
    const TYPE: &'static str = "AWS::VerifiedPermissions::PolicyStore";
    fn properties(&self) -> &PolicyStoreProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PolicyStoreProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for PolicyStore {}

impl From<PolicyStoreProperties> for PolicyStore {
    fn from(properties: PolicyStoreProperties) -> PolicyStore {
        PolicyStore { properties }
    }
}

/// The [`AWS::VerifiedPermissions::PolicyTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-verifiedpermissions-policytemplate.html) resource type.
#[derive(Debug, Default)]
pub struct PolicyTemplate {
    properties: PolicyTemplateProperties
}

/// Properties for the `PolicyTemplate` resource.
#[derive(Debug, Default)]
pub struct PolicyTemplateProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-verifiedpermissions-policytemplate.html#cfn-verifiedpermissions-policytemplate-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`PolicyStoreId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-verifiedpermissions-policytemplate.html#cfn-verifiedpermissions-policytemplate-policystoreid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub policy_store_id: Option<::Value<String>>,
    /// Property [`Statement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-verifiedpermissions-policytemplate.html#cfn-verifiedpermissions-policytemplate-statement).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub statement: ::Value<String>,
}

impl ::serde::Serialize for PolicyTemplateProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref policy_store_id) = self.policy_store_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyStoreId", policy_store_id)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Statement", &self.statement)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PolicyTemplateProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PolicyTemplateProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PolicyTemplateProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PolicyTemplateProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut policy_store_id: Option<::Value<String>> = None;
                let mut statement: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PolicyStoreId" => {
                            policy_store_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Statement" => {
                            statement = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PolicyTemplateProperties {
                    description: description,
                    policy_store_id: policy_store_id,
                    statement: statement.ok_or(::serde::de::Error::missing_field("Statement"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for PolicyTemplate {
    type Properties = PolicyTemplateProperties;
    const TYPE: &'static str = "AWS::VerifiedPermissions::PolicyTemplate";
    fn properties(&self) -> &PolicyTemplateProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PolicyTemplateProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for PolicyTemplate {}

impl From<PolicyTemplateProperties> for PolicyTemplate {
    fn from(properties: PolicyTemplateProperties) -> PolicyTemplate {
        PolicyTemplate { properties }
    }
}

pub mod identity_source {
    //! Property types for the `IdentitySource` resource.

    /// The [`AWS::VerifiedPermissions::IdentitySource.CognitoUserPoolConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-identitysource-cognitouserpoolconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct CognitoUserPoolConfiguration {
        /// Property [`ClientIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-identitysource-cognitouserpoolconfiguration.html#cfn-verifiedpermissions-identitysource-cognitouserpoolconfiguration-clientids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_ids: Option<::ValueList<String>>,
        /// Property [`UserPoolArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-identitysource-cognitouserpoolconfiguration.html#cfn-verifiedpermissions-identitysource-cognitouserpoolconfiguration-userpoolarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_pool_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for CognitoUserPoolConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref client_ids) = self.client_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientIds", client_ids)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserPoolArn", &self.user_pool_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CognitoUserPoolConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CognitoUserPoolConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CognitoUserPoolConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CognitoUserPoolConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut client_ids: Option<::ValueList<String>> = None;
                    let mut user_pool_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ClientIds" => {
                                client_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserPoolArn" => {
                                user_pool_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CognitoUserPoolConfiguration {
                        client_ids: client_ids,
                        user_pool_arn: user_pool_arn.ok_or(::serde::de::Error::missing_field("UserPoolArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::VerifiedPermissions::IdentitySource.IdentitySourceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-identitysource-identitysourceconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct IdentitySourceConfiguration {
        /// Property [`CognitoUserPoolConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-identitysource-identitysourceconfiguration.html#cfn-verifiedpermissions-identitysource-identitysourceconfiguration-cognitouserpoolconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cognito_user_pool_configuration: ::Value<CognitoUserPoolConfiguration>,
    }

    impl ::codec::SerializeValue for IdentitySourceConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CognitoUserPoolConfiguration", &self.cognito_user_pool_configuration)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IdentitySourceConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IdentitySourceConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IdentitySourceConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IdentitySourceConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cognito_user_pool_configuration: Option<::Value<CognitoUserPoolConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CognitoUserPoolConfiguration" => {
                                cognito_user_pool_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IdentitySourceConfiguration {
                        cognito_user_pool_configuration: cognito_user_pool_configuration.ok_or(::serde::de::Error::missing_field("CognitoUserPoolConfiguration"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::VerifiedPermissions::IdentitySource.IdentitySourceDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-identitysource-identitysourcedetails.html) property type.
    #[derive(Debug, Default)]
    pub struct IdentitySourceDetails {
        /// Property [`ClientIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-identitysource-identitysourcedetails.html#cfn-verifiedpermissions-identitysource-identitysourcedetails-clientids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_ids: Option<::ValueList<String>>,
        /// Property [`DiscoveryUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-identitysource-identitysourcedetails.html#cfn-verifiedpermissions-identitysource-identitysourcedetails-discoveryurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub discovery_url: Option<::Value<String>>,
        /// Property [`OpenIdIssuer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-identitysource-identitysourcedetails.html#cfn-verifiedpermissions-identitysource-identitysourcedetails-openidissuer).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub open_id_issuer: Option<::Value<String>>,
        /// Property [`UserPoolArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-identitysource-identitysourcedetails.html#cfn-verifiedpermissions-identitysource-identitysourcedetails-userpoolarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_pool_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for IdentitySourceDetails {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref client_ids) = self.client_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientIds", client_ids)?;
            }
            if let Some(ref discovery_url) = self.discovery_url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DiscoveryUrl", discovery_url)?;
            }
            if let Some(ref open_id_issuer) = self.open_id_issuer {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OpenIdIssuer", open_id_issuer)?;
            }
            if let Some(ref user_pool_arn) = self.user_pool_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserPoolArn", user_pool_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IdentitySourceDetails {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IdentitySourceDetails, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IdentitySourceDetails;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IdentitySourceDetails")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut client_ids: Option<::ValueList<String>> = None;
                    let mut discovery_url: Option<::Value<String>> = None;
                    let mut open_id_issuer: Option<::Value<String>> = None;
                    let mut user_pool_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ClientIds" => {
                                client_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DiscoveryUrl" => {
                                discovery_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OpenIdIssuer" => {
                                open_id_issuer = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserPoolArn" => {
                                user_pool_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IdentitySourceDetails {
                        client_ids: client_ids,
                        discovery_url: discovery_url,
                        open_id_issuer: open_id_issuer,
                        user_pool_arn: user_pool_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod policy {
    //! Property types for the `Policy` resource.

    /// The [`AWS::VerifiedPermissions::Policy.EntityIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-policy-entityidentifier.html) property type.
    #[derive(Debug, Default)]
    pub struct EntityIdentifier {
        /// Property [`EntityId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-policy-entityidentifier.html#cfn-verifiedpermissions-policy-entityidentifier-entityid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub entity_id: ::Value<String>,
        /// Property [`EntityType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-policy-entityidentifier.html#cfn-verifiedpermissions-policy-entityidentifier-entitytype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub entity_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for EntityIdentifier {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EntityId", &self.entity_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EntityType", &self.entity_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EntityIdentifier {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EntityIdentifier, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EntityIdentifier;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EntityIdentifier")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut entity_id: Option<::Value<String>> = None;
                    let mut entity_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EntityId" => {
                                entity_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EntityType" => {
                                entity_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EntityIdentifier {
                        entity_id: entity_id.ok_or(::serde::de::Error::missing_field("EntityId"))?,
                        entity_type: entity_type.ok_or(::serde::de::Error::missing_field("EntityType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::VerifiedPermissions::Policy.PolicyDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-policy-policydefinition.html) property type.
    #[derive(Debug, Default)]
    pub struct PolicyDefinition {
        /// Property [`Static`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-policy-policydefinition.html#cfn-verifiedpermissions-policy-policydefinition-static).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#static: Option<::Value<StaticPolicyDefinition>>,
        /// Property [`TemplateLinked`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-policy-policydefinition.html#cfn-verifiedpermissions-policy-policydefinition-templatelinked).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub template_linked: Option<::Value<TemplateLinkedPolicyDefinition>>,
    }

    impl ::codec::SerializeValue for PolicyDefinition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref r#static) = self.r#static {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Static", r#static)?;
            }
            if let Some(ref template_linked) = self.template_linked {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemplateLinked", template_linked)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PolicyDefinition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PolicyDefinition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PolicyDefinition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PolicyDefinition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut r#static: Option<::Value<StaticPolicyDefinition>> = None;
                    let mut template_linked: Option<::Value<TemplateLinkedPolicyDefinition>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Static" => {
                                r#static = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TemplateLinked" => {
                                template_linked = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PolicyDefinition {
                        r#static: r#static,
                        template_linked: template_linked,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::VerifiedPermissions::Policy.StaticPolicyDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-policy-staticpolicydefinition.html) property type.
    #[derive(Debug, Default)]
    pub struct StaticPolicyDefinition {
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-policy-staticpolicydefinition.html#cfn-verifiedpermissions-policy-staticpolicydefinition-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`Statement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-policy-staticpolicydefinition.html#cfn-verifiedpermissions-policy-staticpolicydefinition-statement).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub statement: ::Value<String>,
    }

    impl ::codec::SerializeValue for StaticPolicyDefinition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Statement", &self.statement)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StaticPolicyDefinition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StaticPolicyDefinition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StaticPolicyDefinition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StaticPolicyDefinition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut description: Option<::Value<String>> = None;
                    let mut statement: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Statement" => {
                                statement = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StaticPolicyDefinition {
                        description: description,
                        statement: statement.ok_or(::serde::de::Error::missing_field("Statement"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::VerifiedPermissions::Policy.TemplateLinkedPolicyDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-policy-templatelinkedpolicydefinition.html) property type.
    #[derive(Debug, Default)]
    pub struct TemplateLinkedPolicyDefinition {
        /// Property [`PolicyTemplateId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-policy-templatelinkedpolicydefinition.html#cfn-verifiedpermissions-policy-templatelinkedpolicydefinition-policytemplateid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub policy_template_id: ::Value<String>,
        /// Property [`Principal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-policy-templatelinkedpolicydefinition.html#cfn-verifiedpermissions-policy-templatelinkedpolicydefinition-principal).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub principal: Option<::Value<EntityIdentifier>>,
        /// Property [`Resource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-policy-templatelinkedpolicydefinition.html#cfn-verifiedpermissions-policy-templatelinkedpolicydefinition-resource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub resource: Option<::Value<EntityIdentifier>>,
    }

    impl ::codec::SerializeValue for TemplateLinkedPolicyDefinition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyTemplateId", &self.policy_template_id)?;
            if let Some(ref principal) = self.principal {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Principal", principal)?;
            }
            if let Some(ref resource) = self.resource {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Resource", resource)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TemplateLinkedPolicyDefinition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TemplateLinkedPolicyDefinition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TemplateLinkedPolicyDefinition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TemplateLinkedPolicyDefinition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut policy_template_id: Option<::Value<String>> = None;
                    let mut principal: Option<::Value<EntityIdentifier>> = None;
                    let mut resource: Option<::Value<EntityIdentifier>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PolicyTemplateId" => {
                                policy_template_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Principal" => {
                                principal = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Resource" => {
                                resource = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TemplateLinkedPolicyDefinition {
                        policy_template_id: policy_template_id.ok_or(::serde::de::Error::missing_field("PolicyTemplateId"))?,
                        principal: principal,
                        resource: resource,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod policy_store {
    //! Property types for the `PolicyStore` resource.

    /// The [`AWS::VerifiedPermissions::PolicyStore.SchemaDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-policystore-schemadefinition.html) property type.
    #[derive(Debug, Default)]
    pub struct SchemaDefinition {
        /// Property [`CedarJson`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-policystore-schemadefinition.html#cfn-verifiedpermissions-policystore-schemadefinition-cedarjson).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cedar_json: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SchemaDefinition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cedar_json) = self.cedar_json {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CedarJson", cedar_json)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SchemaDefinition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SchemaDefinition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SchemaDefinition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SchemaDefinition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cedar_json: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CedarJson" => {
                                cedar_json = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SchemaDefinition {
                        cedar_json: cedar_json,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::VerifiedPermissions::PolicyStore.ValidationSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-policystore-validationsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct ValidationSettings {
        /// Property [`Mode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-verifiedpermissions-policystore-validationsettings.html#cfn-verifiedpermissions-policystore-validationsettings-mode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mode: ::Value<String>,
    }

    impl ::codec::SerializeValue for ValidationSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Mode", &self.mode)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ValidationSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ValidationSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ValidationSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ValidationSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut mode: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Mode" => {
                                mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ValidationSettings {
                        mode: mode.ok_or(::serde::de::Error::missing_field("Mode"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
