//! Types for the `IoT` service.

/// The [`AWS::IoT::AccountAuditConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-accountauditconfiguration.html) resource type.
#[derive(Debug, Default)]
pub struct AccountAuditConfiguration {
    properties: AccountAuditConfigurationProperties
}

/// Properties for the `AccountAuditConfiguration` resource.
#[derive(Debug, Default)]
pub struct AccountAuditConfigurationProperties {
    /// Property [`AccountId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-accountauditconfiguration.html#cfn-iot-accountauditconfiguration-accountid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub account_id: ::Value<String>,
    /// Property [`AuditCheckConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-accountauditconfiguration.html#cfn-iot-accountauditconfiguration-auditcheckconfigurations).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub audit_check_configurations: ::Value<self::account_audit_configuration::AuditCheckConfigurations>,
    /// Property [`AuditNotificationTargetConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-accountauditconfiguration.html#cfn-iot-accountauditconfiguration-auditnotificationtargetconfigurations).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub audit_notification_target_configurations: Option<::Value<self::account_audit_configuration::AuditNotificationTargetConfigurations>>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-accountauditconfiguration.html#cfn-iot-accountauditconfiguration-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: ::Value<String>,
}

impl ::serde::Serialize for AccountAuditConfigurationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccountId", &self.account_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuditCheckConfigurations", &self.audit_check_configurations)?;
        if let Some(ref audit_notification_target_configurations) = self.audit_notification_target_configurations {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuditNotificationTargetConfigurations", audit_notification_target_configurations)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AccountAuditConfigurationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AccountAuditConfigurationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AccountAuditConfigurationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AccountAuditConfigurationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut account_id: Option<::Value<String>> = None;
                let mut audit_check_configurations: Option<::Value<self::account_audit_configuration::AuditCheckConfigurations>> = None;
                let mut audit_notification_target_configurations: Option<::Value<self::account_audit_configuration::AuditNotificationTargetConfigurations>> = None;
                let mut role_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AccountId" => {
                            account_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AuditCheckConfigurations" => {
                            audit_check_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AuditNotificationTargetConfigurations" => {
                            audit_notification_target_configurations = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AccountAuditConfigurationProperties {
                    account_id: account_id.ok_or(::serde::de::Error::missing_field("AccountId"))?,
                    audit_check_configurations: audit_check_configurations.ok_or(::serde::de::Error::missing_field("AuditCheckConfigurations"))?,
                    audit_notification_target_configurations: audit_notification_target_configurations,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for AccountAuditConfiguration {
    type Properties = AccountAuditConfigurationProperties;
    const TYPE: &'static str = "AWS::IoT::AccountAuditConfiguration";
    fn properties(&self) -> &AccountAuditConfigurationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AccountAuditConfigurationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for AccountAuditConfiguration {}

impl From<AccountAuditConfigurationProperties> for AccountAuditConfiguration {
    fn from(properties: AccountAuditConfigurationProperties) -> AccountAuditConfiguration {
        AccountAuditConfiguration { properties }
    }
}

/// The [`AWS::IoT::Authorizer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-authorizer.html) resource type.
#[derive(Debug, Default)]
pub struct Authorizer {
    properties: AuthorizerProperties
}

/// Properties for the `Authorizer` resource.
#[derive(Debug, Default)]
pub struct AuthorizerProperties {
    /// Property [`AuthorizerFunctionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-authorizer.html#cfn-iot-authorizer-authorizerfunctionarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub authorizer_function_arn: ::Value<String>,
    /// Property [`AuthorizerName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-authorizer.html#cfn-iot-authorizer-authorizername).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub authorizer_name: Option<::Value<String>>,
    /// Property [`EnableCachingForHttp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-authorizer.html#cfn-iot-authorizer-enablecachingforhttp).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enable_caching_for_http: Option<::Value<bool>>,
    /// Property [`SigningDisabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-authorizer.html#cfn-iot-authorizer-signingdisabled).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub signing_disabled: Option<::Value<bool>>,
    /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-authorizer.html#cfn-iot-authorizer-status).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub status: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-authorizer.html#cfn-iot-authorizer-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TokenKeyName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-authorizer.html#cfn-iot-authorizer-tokenkeyname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub token_key_name: Option<::Value<String>>,
    /// Property [`TokenSigningPublicKeys`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-authorizer.html#cfn-iot-authorizer-tokensigningpublickeys).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub token_signing_public_keys: Option<::ValueMap<String>>,
}

impl ::serde::Serialize for AuthorizerProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthorizerFunctionArn", &self.authorizer_function_arn)?;
        if let Some(ref authorizer_name) = self.authorizer_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthorizerName", authorizer_name)?;
        }
        if let Some(ref enable_caching_for_http) = self.enable_caching_for_http {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableCachingForHttp", enable_caching_for_http)?;
        }
        if let Some(ref signing_disabled) = self.signing_disabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SigningDisabled", signing_disabled)?;
        }
        if let Some(ref status) = self.status {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", status)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref token_key_name) = self.token_key_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TokenKeyName", token_key_name)?;
        }
        if let Some(ref token_signing_public_keys) = self.token_signing_public_keys {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TokenSigningPublicKeys", token_signing_public_keys)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AuthorizerProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AuthorizerProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AuthorizerProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AuthorizerProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut authorizer_function_arn: Option<::Value<String>> = None;
                let mut authorizer_name: Option<::Value<String>> = None;
                let mut enable_caching_for_http: Option<::Value<bool>> = None;
                let mut signing_disabled: Option<::Value<bool>> = None;
                let mut status: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut token_key_name: Option<::Value<String>> = None;
                let mut token_signing_public_keys: Option<::ValueMap<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AuthorizerFunctionArn" => {
                            authorizer_function_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AuthorizerName" => {
                            authorizer_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnableCachingForHttp" => {
                            enable_caching_for_http = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SigningDisabled" => {
                            signing_disabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Status" => {
                            status = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TokenKeyName" => {
                            token_key_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TokenSigningPublicKeys" => {
                            token_signing_public_keys = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AuthorizerProperties {
                    authorizer_function_arn: authorizer_function_arn.ok_or(::serde::de::Error::missing_field("AuthorizerFunctionArn"))?,
                    authorizer_name: authorizer_name,
                    enable_caching_for_http: enable_caching_for_http,
                    signing_disabled: signing_disabled,
                    status: status,
                    tags: tags,
                    token_key_name: token_key_name,
                    token_signing_public_keys: token_signing_public_keys,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Authorizer {
    type Properties = AuthorizerProperties;
    const TYPE: &'static str = "AWS::IoT::Authorizer";
    fn properties(&self) -> &AuthorizerProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AuthorizerProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Authorizer {}

impl From<AuthorizerProperties> for Authorizer {
    fn from(properties: AuthorizerProperties) -> Authorizer {
        Authorizer { properties }
    }
}

/// The [`AWS::IoT::BillingGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-billinggroup.html) resource type.
#[derive(Debug, Default)]
pub struct BillingGroup {
    properties: BillingGroupProperties
}

/// Properties for the `BillingGroup` resource.
#[derive(Debug, Default)]
pub struct BillingGroupProperties {
    /// Property [`BillingGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-billinggroup.html#cfn-iot-billinggroup-billinggroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub billing_group_name: Option<::Value<String>>,
    /// Property [`BillingGroupProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-billinggroup.html#cfn-iot-billinggroup-billinggroupproperties).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub billing_group_properties: Option<::Value<self::billing_group::BillingGroupProperties>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-billinggroup.html#cfn-iot-billinggroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for BillingGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref billing_group_name) = self.billing_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BillingGroupName", billing_group_name)?;
        }
        if let Some(ref billing_group_properties) = self.billing_group_properties {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BillingGroupProperties", billing_group_properties)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for BillingGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<BillingGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = BillingGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type BillingGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut billing_group_name: Option<::Value<String>> = None;
                let mut billing_group_properties: Option<::Value<self::billing_group::BillingGroupProperties>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "BillingGroupName" => {
                            billing_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BillingGroupProperties" => {
                            billing_group_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(BillingGroupProperties {
                    billing_group_name: billing_group_name,
                    billing_group_properties: billing_group_properties,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for BillingGroup {
    type Properties = BillingGroupProperties;
    const TYPE: &'static str = "AWS::IoT::BillingGroup";
    fn properties(&self) -> &BillingGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut BillingGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for BillingGroup {}

impl From<BillingGroupProperties> for BillingGroup {
    fn from(properties: BillingGroupProperties) -> BillingGroup {
        BillingGroup { properties }
    }
}

/// The [`AWS::IoT::CACertificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-cacertificate.html) resource type.
#[derive(Debug, Default)]
pub struct CACertificate {
    properties: CACertificateProperties
}

/// Properties for the `CACertificate` resource.
#[derive(Debug, Default)]
pub struct CACertificateProperties {
    /// Property [`AutoRegistrationStatus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-cacertificate.html#cfn-iot-cacertificate-autoregistrationstatus).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub auto_registration_status: Option<::Value<String>>,
    /// Property [`CACertificatePem`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-cacertificate.html#cfn-iot-cacertificate-cacertificatepem).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub ca_certificate_pem: ::Value<String>,
    /// Property [`CertificateMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-cacertificate.html#cfn-iot-cacertificate-certificatemode).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub certificate_mode: Option<::Value<String>>,
    /// Property [`RegistrationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-cacertificate.html#cfn-iot-cacertificate-registrationconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub registration_config: Option<::Value<self::ca_certificate::RegistrationConfig>>,
    /// Property [`RemoveAutoRegistration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-cacertificate.html#cfn-iot-cacertificate-removeautoregistration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub remove_auto_registration: Option<::Value<bool>>,
    /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-cacertificate.html#cfn-iot-cacertificate-status).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub status: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-cacertificate.html#cfn-iot-cacertificate-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`VerificationCertificatePem`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-cacertificate.html#cfn-iot-cacertificate-verificationcertificatepem).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub verification_certificate_pem: Option<::Value<String>>,
}

impl ::serde::Serialize for CACertificateProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref auto_registration_status) = self.auto_registration_status {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoRegistrationStatus", auto_registration_status)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CACertificatePem", &self.ca_certificate_pem)?;
        if let Some(ref certificate_mode) = self.certificate_mode {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateMode", certificate_mode)?;
        }
        if let Some(ref registration_config) = self.registration_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RegistrationConfig", registration_config)?;
        }
        if let Some(ref remove_auto_registration) = self.remove_auto_registration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RemoveAutoRegistration", remove_auto_registration)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", &self.status)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref verification_certificate_pem) = self.verification_certificate_pem {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VerificationCertificatePem", verification_certificate_pem)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CACertificateProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CACertificateProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CACertificateProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CACertificateProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut auto_registration_status: Option<::Value<String>> = None;
                let mut ca_certificate_pem: Option<::Value<String>> = None;
                let mut certificate_mode: Option<::Value<String>> = None;
                let mut registration_config: Option<::Value<self::ca_certificate::RegistrationConfig>> = None;
                let mut remove_auto_registration: Option<::Value<bool>> = None;
                let mut status: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut verification_certificate_pem: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AutoRegistrationStatus" => {
                            auto_registration_status = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CACertificatePem" => {
                            ca_certificate_pem = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CertificateMode" => {
                            certificate_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RegistrationConfig" => {
                            registration_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RemoveAutoRegistration" => {
                            remove_auto_registration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Status" => {
                            status = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VerificationCertificatePem" => {
                            verification_certificate_pem = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CACertificateProperties {
                    auto_registration_status: auto_registration_status,
                    ca_certificate_pem: ca_certificate_pem.ok_or(::serde::de::Error::missing_field("CACertificatePem"))?,
                    certificate_mode: certificate_mode,
                    registration_config: registration_config,
                    remove_auto_registration: remove_auto_registration,
                    status: status.ok_or(::serde::de::Error::missing_field("Status"))?,
                    tags: tags,
                    verification_certificate_pem: verification_certificate_pem,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for CACertificate {
    type Properties = CACertificateProperties;
    const TYPE: &'static str = "AWS::IoT::CACertificate";
    fn properties(&self) -> &CACertificateProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CACertificateProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for CACertificate {}

impl From<CACertificateProperties> for CACertificate {
    fn from(properties: CACertificateProperties) -> CACertificate {
        CACertificate { properties }
    }
}

/// The [`AWS::IoT::Certificate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-certificate.html) resource type.
#[derive(Debug, Default)]
pub struct Certificate {
    properties: CertificateProperties
}

/// Properties for the `Certificate` resource.
#[derive(Debug, Default)]
pub struct CertificateProperties {
    /// Property [`CACertificatePem`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-certificate.html#cfn-iot-certificate-cacertificatepem).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub ca_certificate_pem: Option<::Value<String>>,
    /// Property [`CertificateMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-certificate.html#cfn-iot-certificate-certificatemode).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub certificate_mode: Option<::Value<String>>,
    /// Property [`CertificatePem`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-certificate.html#cfn-iot-certificate-certificatepem).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub certificate_pem: Option<::Value<String>>,
    /// Property [`CertificateSigningRequest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-certificate.html#cfn-iot-certificate-certificatesigningrequest).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub certificate_signing_request: Option<::Value<String>>,
    /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-certificate.html#cfn-iot-certificate-status).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub status: ::Value<String>,
}

impl ::serde::Serialize for CertificateProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref ca_certificate_pem) = self.ca_certificate_pem {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CACertificatePem", ca_certificate_pem)?;
        }
        if let Some(ref certificate_mode) = self.certificate_mode {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateMode", certificate_mode)?;
        }
        if let Some(ref certificate_pem) = self.certificate_pem {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificatePem", certificate_pem)?;
        }
        if let Some(ref certificate_signing_request) = self.certificate_signing_request {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateSigningRequest", certificate_signing_request)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", &self.status)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CertificateProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CertificateProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CertificateProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CertificateProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut ca_certificate_pem: Option<::Value<String>> = None;
                let mut certificate_mode: Option<::Value<String>> = None;
                let mut certificate_pem: Option<::Value<String>> = None;
                let mut certificate_signing_request: Option<::Value<String>> = None;
                let mut status: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CACertificatePem" => {
                            ca_certificate_pem = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CertificateMode" => {
                            certificate_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CertificatePem" => {
                            certificate_pem = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CertificateSigningRequest" => {
                            certificate_signing_request = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Status" => {
                            status = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CertificateProperties {
                    ca_certificate_pem: ca_certificate_pem,
                    certificate_mode: certificate_mode,
                    certificate_pem: certificate_pem,
                    certificate_signing_request: certificate_signing_request,
                    status: status.ok_or(::serde::de::Error::missing_field("Status"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Certificate {
    type Properties = CertificateProperties;
    const TYPE: &'static str = "AWS::IoT::Certificate";
    fn properties(&self) -> &CertificateProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CertificateProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Certificate {}

impl From<CertificateProperties> for Certificate {
    fn from(properties: CertificateProperties) -> Certificate {
        Certificate { properties }
    }
}

/// The [`AWS::IoT::CertificateProvider`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-certificateprovider.html) resource type.
#[derive(Debug, Default)]
pub struct CertificateProvider {
    properties: CertificateProviderProperties
}

/// Properties for the `CertificateProvider` resource.
#[derive(Debug, Default)]
pub struct CertificateProviderProperties {
    /// Property [`AccountDefaultForOperations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-certificateprovider.html#cfn-iot-certificateprovider-accountdefaultforoperations).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub account_default_for_operations: ::ValueList<String>,
    /// Property [`CertificateProviderName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-certificateprovider.html#cfn-iot-certificateprovider-certificateprovidername).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub certificate_provider_name: Option<::Value<String>>,
    /// Property [`LambdaFunctionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-certificateprovider.html#cfn-iot-certificateprovider-lambdafunctionarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub lambda_function_arn: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-certificateprovider.html#cfn-iot-certificateprovider-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for CertificateProviderProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccountDefaultForOperations", &self.account_default_for_operations)?;
        if let Some(ref certificate_provider_name) = self.certificate_provider_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateProviderName", certificate_provider_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LambdaFunctionArn", &self.lambda_function_arn)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CertificateProviderProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CertificateProviderProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CertificateProviderProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CertificateProviderProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut account_default_for_operations: Option<::ValueList<String>> = None;
                let mut certificate_provider_name: Option<::Value<String>> = None;
                let mut lambda_function_arn: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AccountDefaultForOperations" => {
                            account_default_for_operations = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CertificateProviderName" => {
                            certificate_provider_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LambdaFunctionArn" => {
                            lambda_function_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CertificateProviderProperties {
                    account_default_for_operations: account_default_for_operations.ok_or(::serde::de::Error::missing_field("AccountDefaultForOperations"))?,
                    certificate_provider_name: certificate_provider_name,
                    lambda_function_arn: lambda_function_arn.ok_or(::serde::de::Error::missing_field("LambdaFunctionArn"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for CertificateProvider {
    type Properties = CertificateProviderProperties;
    const TYPE: &'static str = "AWS::IoT::CertificateProvider";
    fn properties(&self) -> &CertificateProviderProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CertificateProviderProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for CertificateProvider {}

impl From<CertificateProviderProperties> for CertificateProvider {
    fn from(properties: CertificateProviderProperties) -> CertificateProvider {
        CertificateProvider { properties }
    }
}

/// The [`AWS::IoT::CustomMetric`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-custommetric.html) resource type.
#[derive(Debug, Default)]
pub struct CustomMetric {
    properties: CustomMetricProperties
}

/// Properties for the `CustomMetric` resource.
#[derive(Debug, Default)]
pub struct CustomMetricProperties {
    /// Property [`DisplayName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-custommetric.html#cfn-iot-custommetric-displayname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub display_name: Option<::Value<String>>,
    /// Property [`MetricName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-custommetric.html#cfn-iot-custommetric-metricname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub metric_name: Option<::Value<String>>,
    /// Property [`MetricType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-custommetric.html#cfn-iot-custommetric-metrictype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub metric_type: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-custommetric.html#cfn-iot-custommetric-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for CustomMetricProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref display_name) = self.display_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisplayName", display_name)?;
        }
        if let Some(ref metric_name) = self.metric_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricName", metric_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricType", &self.metric_type)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CustomMetricProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomMetricProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CustomMetricProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CustomMetricProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut display_name: Option<::Value<String>> = None;
                let mut metric_name: Option<::Value<String>> = None;
                let mut metric_type: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DisplayName" => {
                            display_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MetricName" => {
                            metric_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MetricType" => {
                            metric_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CustomMetricProperties {
                    display_name: display_name,
                    metric_name: metric_name,
                    metric_type: metric_type.ok_or(::serde::de::Error::missing_field("MetricType"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for CustomMetric {
    type Properties = CustomMetricProperties;
    const TYPE: &'static str = "AWS::IoT::CustomMetric";
    fn properties(&self) -> &CustomMetricProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CustomMetricProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for CustomMetric {}

impl From<CustomMetricProperties> for CustomMetric {
    fn from(properties: CustomMetricProperties) -> CustomMetric {
        CustomMetric { properties }
    }
}

/// The [`AWS::IoT::Dimension`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-dimension.html) resource type.
#[derive(Debug, Default)]
pub struct Dimension {
    properties: DimensionProperties
}

/// Properties for the `Dimension` resource.
#[derive(Debug, Default)]
pub struct DimensionProperties {
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-dimension.html#cfn-iot-dimension-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`StringValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-dimension.html#cfn-iot-dimension-stringvalues).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub string_values: ::ValueList<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-dimension.html#cfn-iot-dimension-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-dimension.html#cfn-iot-dimension-type).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub r#type: ::Value<String>,
}

impl ::serde::Serialize for DimensionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StringValues", &self.string_values)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DimensionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DimensionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DimensionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DimensionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut name: Option<::Value<String>> = None;
                let mut string_values: Option<::ValueList<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut r#type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StringValues" => {
                            string_values = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Type" => {
                            r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DimensionProperties {
                    name: name,
                    string_values: string_values.ok_or(::serde::de::Error::missing_field("StringValues"))?,
                    tags: tags,
                    r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Dimension {
    type Properties = DimensionProperties;
    const TYPE: &'static str = "AWS::IoT::Dimension";
    fn properties(&self) -> &DimensionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DimensionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Dimension {}

impl From<DimensionProperties> for Dimension {
    fn from(properties: DimensionProperties) -> Dimension {
        Dimension { properties }
    }
}

/// The [`AWS::IoT::DomainConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-domainconfiguration.html) resource type.
#[derive(Debug, Default)]
pub struct DomainConfiguration {
    properties: DomainConfigurationProperties
}

/// Properties for the `DomainConfiguration` resource.
#[derive(Debug, Default)]
pub struct DomainConfigurationProperties {
    /// Property [`AuthorizerConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-domainconfiguration.html#cfn-iot-domainconfiguration-authorizerconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub authorizer_config: Option<::Value<self::domain_configuration::AuthorizerConfig>>,
    /// Property [`DomainConfigurationName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-domainconfiguration.html#cfn-iot-domainconfiguration-domainconfigurationname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub domain_configuration_name: Option<::Value<String>>,
    /// Property [`DomainConfigurationStatus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-domainconfiguration.html#cfn-iot-domainconfiguration-domainconfigurationstatus).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub domain_configuration_status: Option<::Value<String>>,
    /// Property [`DomainName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-domainconfiguration.html#cfn-iot-domainconfiguration-domainname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub domain_name: Option<::Value<String>>,
    /// Property [`ServerCertificateArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-domainconfiguration.html#cfn-iot-domainconfiguration-servercertificatearns).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub server_certificate_arns: Option<::ValueList<String>>,
    /// Property [`ServiceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-domainconfiguration.html#cfn-iot-domainconfiguration-servicetype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub service_type: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-domainconfiguration.html#cfn-iot-domainconfiguration-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TlsConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-domainconfiguration.html#cfn-iot-domainconfiguration-tlsconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tls_config: Option<::Value<self::domain_configuration::TlsConfig>>,
    /// Property [`ValidationCertificateArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-domainconfiguration.html#cfn-iot-domainconfiguration-validationcertificatearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub validation_certificate_arn: Option<::Value<String>>,
}

impl ::serde::Serialize for DomainConfigurationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref authorizer_config) = self.authorizer_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthorizerConfig", authorizer_config)?;
        }
        if let Some(ref domain_configuration_name) = self.domain_configuration_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainConfigurationName", domain_configuration_name)?;
        }
        if let Some(ref domain_configuration_status) = self.domain_configuration_status {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainConfigurationStatus", domain_configuration_status)?;
        }
        if let Some(ref domain_name) = self.domain_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainName", domain_name)?;
        }
        if let Some(ref server_certificate_arns) = self.server_certificate_arns {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerCertificateArns", server_certificate_arns)?;
        }
        if let Some(ref service_type) = self.service_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceType", service_type)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref tls_config) = self.tls_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TlsConfig", tls_config)?;
        }
        if let Some(ref validation_certificate_arn) = self.validation_certificate_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ValidationCertificateArn", validation_certificate_arn)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DomainConfigurationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DomainConfigurationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DomainConfigurationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DomainConfigurationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut authorizer_config: Option<::Value<self::domain_configuration::AuthorizerConfig>> = None;
                let mut domain_configuration_name: Option<::Value<String>> = None;
                let mut domain_configuration_status: Option<::Value<String>> = None;
                let mut domain_name: Option<::Value<String>> = None;
                let mut server_certificate_arns: Option<::ValueList<String>> = None;
                let mut service_type: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut tls_config: Option<::Value<self::domain_configuration::TlsConfig>> = None;
                let mut validation_certificate_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AuthorizerConfig" => {
                            authorizer_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DomainConfigurationName" => {
                            domain_configuration_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DomainConfigurationStatus" => {
                            domain_configuration_status = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DomainName" => {
                            domain_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServerCertificateArns" => {
                            server_certificate_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServiceType" => {
                            service_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TlsConfig" => {
                            tls_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ValidationCertificateArn" => {
                            validation_certificate_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DomainConfigurationProperties {
                    authorizer_config: authorizer_config,
                    domain_configuration_name: domain_configuration_name,
                    domain_configuration_status: domain_configuration_status,
                    domain_name: domain_name,
                    server_certificate_arns: server_certificate_arns,
                    service_type: service_type,
                    tags: tags,
                    tls_config: tls_config,
                    validation_certificate_arn: validation_certificate_arn,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DomainConfiguration {
    type Properties = DomainConfigurationProperties;
    const TYPE: &'static str = "AWS::IoT::DomainConfiguration";
    fn properties(&self) -> &DomainConfigurationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DomainConfigurationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DomainConfiguration {}

impl From<DomainConfigurationProperties> for DomainConfiguration {
    fn from(properties: DomainConfigurationProperties) -> DomainConfiguration {
        DomainConfiguration { properties }
    }
}

/// The [`AWS::IoT::FleetMetric`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-fleetmetric.html) resource type.
#[derive(Debug, Default)]
pub struct FleetMetric {
    properties: FleetMetricProperties
}

/// Properties for the `FleetMetric` resource.
#[derive(Debug, Default)]
pub struct FleetMetricProperties {
    /// Property [`AggregationField`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-fleetmetric.html#cfn-iot-fleetmetric-aggregationfield).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub aggregation_field: Option<::Value<String>>,
    /// Property [`AggregationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-fleetmetric.html#cfn-iot-fleetmetric-aggregationtype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub aggregation_type: Option<::Value<self::fleet_metric::AggregationType>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-fleetmetric.html#cfn-iot-fleetmetric-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`IndexName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-fleetmetric.html#cfn-iot-fleetmetric-indexname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub index_name: Option<::Value<String>>,
    /// Property [`MetricName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-fleetmetric.html#cfn-iot-fleetmetric-metricname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub metric_name: ::Value<String>,
    /// Property [`Period`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-fleetmetric.html#cfn-iot-fleetmetric-period).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub period: Option<::Value<u32>>,
    /// Property [`QueryString`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-fleetmetric.html#cfn-iot-fleetmetric-querystring).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub query_string: Option<::Value<String>>,
    /// Property [`QueryVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-fleetmetric.html#cfn-iot-fleetmetric-queryversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub query_version: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-fleetmetric.html#cfn-iot-fleetmetric-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Unit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-fleetmetric.html#cfn-iot-fleetmetric-unit).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub unit: Option<::Value<String>>,
}

impl ::serde::Serialize for FleetMetricProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref aggregation_field) = self.aggregation_field {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AggregationField", aggregation_field)?;
        }
        if let Some(ref aggregation_type) = self.aggregation_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AggregationType", aggregation_type)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref index_name) = self.index_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IndexName", index_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricName", &self.metric_name)?;
        if let Some(ref period) = self.period {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Period", period)?;
        }
        if let Some(ref query_string) = self.query_string {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryString", query_string)?;
        }
        if let Some(ref query_version) = self.query_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryVersion", query_version)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref unit) = self.unit {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Unit", unit)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for FleetMetricProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<FleetMetricProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = FleetMetricProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type FleetMetricProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut aggregation_field: Option<::Value<String>> = None;
                let mut aggregation_type: Option<::Value<self::fleet_metric::AggregationType>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut index_name: Option<::Value<String>> = None;
                let mut metric_name: Option<::Value<String>> = None;
                let mut period: Option<::Value<u32>> = None;
                let mut query_string: Option<::Value<String>> = None;
                let mut query_version: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut unit: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AggregationField" => {
                            aggregation_field = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AggregationType" => {
                            aggregation_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IndexName" => {
                            index_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MetricName" => {
                            metric_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Period" => {
                            period = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "QueryString" => {
                            query_string = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "QueryVersion" => {
                            query_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Unit" => {
                            unit = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(FleetMetricProperties {
                    aggregation_field: aggregation_field,
                    aggregation_type: aggregation_type,
                    description: description,
                    index_name: index_name,
                    metric_name: metric_name.ok_or(::serde::de::Error::missing_field("MetricName"))?,
                    period: period,
                    query_string: query_string,
                    query_version: query_version,
                    tags: tags,
                    unit: unit,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for FleetMetric {
    type Properties = FleetMetricProperties;
    const TYPE: &'static str = "AWS::IoT::FleetMetric";
    fn properties(&self) -> &FleetMetricProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut FleetMetricProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for FleetMetric {}

impl From<FleetMetricProperties> for FleetMetric {
    fn from(properties: FleetMetricProperties) -> FleetMetric {
        FleetMetric { properties }
    }
}

/// The [`AWS::IoT::JobTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-jobtemplate.html) resource type.
#[derive(Debug, Default)]
pub struct JobTemplate {
    properties: JobTemplateProperties
}

/// Properties for the `JobTemplate` resource.
#[derive(Debug, Default)]
pub struct JobTemplateProperties {
    /// Property [`AbortConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-jobtemplate.html#cfn-iot-jobtemplate-abortconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub abort_config: Option<::Value<self::job_template::AbortConfig>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-jobtemplate.html#cfn-iot-jobtemplate-description).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub description: ::Value<String>,
    /// Property [`DestinationPackageVersions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-jobtemplate.html#cfn-iot-jobtemplate-destinationpackageversions).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub destination_package_versions: Option<::ValueList<String>>,
    /// Property [`Document`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-jobtemplate.html#cfn-iot-jobtemplate-document).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub document: Option<::Value<String>>,
    /// Property [`DocumentSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-jobtemplate.html#cfn-iot-jobtemplate-documentsource).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub document_source: Option<::Value<String>>,
    /// Property [`JobArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-jobtemplate.html#cfn-iot-jobtemplate-jobarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub job_arn: Option<::Value<String>>,
    /// Property [`JobExecutionsRetryConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-jobtemplate.html#cfn-iot-jobtemplate-jobexecutionsretryconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub job_executions_retry_config: Option<::Value<self::job_template::JobExecutionsRetryConfig>>,
    /// Property [`JobExecutionsRolloutConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-jobtemplate.html#cfn-iot-jobtemplate-jobexecutionsrolloutconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub job_executions_rollout_config: Option<::Value<self::job_template::JobExecutionsRolloutConfig>>,
    /// Property [`JobTemplateId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-jobtemplate.html#cfn-iot-jobtemplate-jobtemplateid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub job_template_id: ::Value<String>,
    /// Property [`MaintenanceWindows`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-jobtemplate.html#cfn-iot-jobtemplate-maintenancewindows).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub maintenance_windows: Option<::ValueList<self::job_template::MaintenanceWindow>>,
    /// Property [`PresignedUrlConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-jobtemplate.html#cfn-iot-jobtemplate-presignedurlconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub presigned_url_config: Option<::Value<self::job_template::PresignedUrlConfig>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-jobtemplate.html#cfn-iot-jobtemplate-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TimeoutConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-jobtemplate.html#cfn-iot-jobtemplate-timeoutconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub timeout_config: Option<::Value<self::job_template::TimeoutConfig>>,
}

impl ::serde::Serialize for JobTemplateProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref abort_config) = self.abort_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AbortConfig", abort_config)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        if let Some(ref destination_package_versions) = self.destination_package_versions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationPackageVersions", destination_package_versions)?;
        }
        if let Some(ref document) = self.document {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Document", document)?;
        }
        if let Some(ref document_source) = self.document_source {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentSource", document_source)?;
        }
        if let Some(ref job_arn) = self.job_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "JobArn", job_arn)?;
        }
        if let Some(ref job_executions_retry_config) = self.job_executions_retry_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "JobExecutionsRetryConfig", job_executions_retry_config)?;
        }
        if let Some(ref job_executions_rollout_config) = self.job_executions_rollout_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "JobExecutionsRolloutConfig", job_executions_rollout_config)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "JobTemplateId", &self.job_template_id)?;
        if let Some(ref maintenance_windows) = self.maintenance_windows {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaintenanceWindows", maintenance_windows)?;
        }
        if let Some(ref presigned_url_config) = self.presigned_url_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PresignedUrlConfig", presigned_url_config)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref timeout_config) = self.timeout_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeoutConfig", timeout_config)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for JobTemplateProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<JobTemplateProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = JobTemplateProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type JobTemplateProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut abort_config: Option<::Value<self::job_template::AbortConfig>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut destination_package_versions: Option<::ValueList<String>> = None;
                let mut document: Option<::Value<String>> = None;
                let mut document_source: Option<::Value<String>> = None;
                let mut job_arn: Option<::Value<String>> = None;
                let mut job_executions_retry_config: Option<::Value<self::job_template::JobExecutionsRetryConfig>> = None;
                let mut job_executions_rollout_config: Option<::Value<self::job_template::JobExecutionsRolloutConfig>> = None;
                let mut job_template_id: Option<::Value<String>> = None;
                let mut maintenance_windows: Option<::ValueList<self::job_template::MaintenanceWindow>> = None;
                let mut presigned_url_config: Option<::Value<self::job_template::PresignedUrlConfig>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut timeout_config: Option<::Value<self::job_template::TimeoutConfig>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AbortConfig" => {
                            abort_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DestinationPackageVersions" => {
                            destination_package_versions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Document" => {
                            document = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DocumentSource" => {
                            document_source = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "JobArn" => {
                            job_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "JobExecutionsRetryConfig" => {
                            job_executions_retry_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "JobExecutionsRolloutConfig" => {
                            job_executions_rollout_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "JobTemplateId" => {
                            job_template_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaintenanceWindows" => {
                            maintenance_windows = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PresignedUrlConfig" => {
                            presigned_url_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TimeoutConfig" => {
                            timeout_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(JobTemplateProperties {
                    abort_config: abort_config,
                    description: description.ok_or(::serde::de::Error::missing_field("Description"))?,
                    destination_package_versions: destination_package_versions,
                    document: document,
                    document_source: document_source,
                    job_arn: job_arn,
                    job_executions_retry_config: job_executions_retry_config,
                    job_executions_rollout_config: job_executions_rollout_config,
                    job_template_id: job_template_id.ok_or(::serde::de::Error::missing_field("JobTemplateId"))?,
                    maintenance_windows: maintenance_windows,
                    presigned_url_config: presigned_url_config,
                    tags: tags,
                    timeout_config: timeout_config,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for JobTemplate {
    type Properties = JobTemplateProperties;
    const TYPE: &'static str = "AWS::IoT::JobTemplate";
    fn properties(&self) -> &JobTemplateProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut JobTemplateProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for JobTemplate {}

impl From<JobTemplateProperties> for JobTemplate {
    fn from(properties: JobTemplateProperties) -> JobTemplate {
        JobTemplate { properties }
    }
}

/// The [`AWS::IoT::Logging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-logging.html) resource type.
#[derive(Debug, Default)]
pub struct Logging {
    properties: LoggingProperties
}

/// Properties for the `Logging` resource.
#[derive(Debug, Default)]
pub struct LoggingProperties {
    /// Property [`AccountId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-logging.html#cfn-iot-logging-accountid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub account_id: ::Value<String>,
    /// Property [`DefaultLogLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-logging.html#cfn-iot-logging-defaultloglevel).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub default_log_level: ::Value<String>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-logging.html#cfn-iot-logging-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: ::Value<String>,
}

impl ::serde::Serialize for LoggingProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccountId", &self.account_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultLogLevel", &self.default_log_level)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for LoggingProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<LoggingProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = LoggingProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type LoggingProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut account_id: Option<::Value<String>> = None;
                let mut default_log_level: Option<::Value<String>> = None;
                let mut role_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AccountId" => {
                            account_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DefaultLogLevel" => {
                            default_log_level = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(LoggingProperties {
                    account_id: account_id.ok_or(::serde::de::Error::missing_field("AccountId"))?,
                    default_log_level: default_log_level.ok_or(::serde::de::Error::missing_field("DefaultLogLevel"))?,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Logging {
    type Properties = LoggingProperties;
    const TYPE: &'static str = "AWS::IoT::Logging";
    fn properties(&self) -> &LoggingProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut LoggingProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Logging {}

impl From<LoggingProperties> for Logging {
    fn from(properties: LoggingProperties) -> Logging {
        Logging { properties }
    }
}

/// The [`AWS::IoT::MitigationAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-mitigationaction.html) resource type.
#[derive(Debug, Default)]
pub struct MitigationAction {
    properties: MitigationActionProperties
}

/// Properties for the `MitigationAction` resource.
#[derive(Debug, Default)]
pub struct MitigationActionProperties {
    /// Property [`ActionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-mitigationaction.html#cfn-iot-mitigationaction-actionname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub action_name: Option<::Value<String>>,
    /// Property [`ActionParams`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-mitigationaction.html#cfn-iot-mitigationaction-actionparams).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub action_params: ::Value<self::mitigation_action::ActionParams>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-mitigationaction.html#cfn-iot-mitigationaction-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-mitigationaction.html#cfn-iot-mitigationaction-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for MitigationActionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref action_name) = self.action_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ActionName", action_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ActionParams", &self.action_params)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for MitigationActionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<MitigationActionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = MitigationActionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type MitigationActionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut action_name: Option<::Value<String>> = None;
                let mut action_params: Option<::Value<self::mitigation_action::ActionParams>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ActionName" => {
                            action_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ActionParams" => {
                            action_params = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(MitigationActionProperties {
                    action_name: action_name,
                    action_params: action_params.ok_or(::serde::de::Error::missing_field("ActionParams"))?,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for MitigationAction {
    type Properties = MitigationActionProperties;
    const TYPE: &'static str = "AWS::IoT::MitigationAction";
    fn properties(&self) -> &MitigationActionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut MitigationActionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for MitigationAction {}

impl From<MitigationActionProperties> for MitigationAction {
    fn from(properties: MitigationActionProperties) -> MitigationAction {
        MitigationAction { properties }
    }
}

/// The [`AWS::IoT::Policy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-policy.html) resource type.
#[derive(Debug, Default)]
pub struct Policy {
    properties: PolicyProperties
}

/// Properties for the `Policy` resource.
#[derive(Debug, Default)]
pub struct PolicyProperties {
    /// Property [`PolicyDocument`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-policy.html#cfn-iot-policy-policydocument).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub policy_document: ::Value<::json::Value>,
    /// Property [`PolicyName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-policy.html#cfn-iot-policy-policyname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub policy_name: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-policy.html#cfn-iot-policy-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for PolicyProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyDocument", &self.policy_document)?;
        if let Some(ref policy_name) = self.policy_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyName", policy_name)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
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
                let mut policy_document: Option<::Value<::json::Value>> = None;
                let mut policy_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "PolicyDocument" => {
                            policy_document = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PolicyName" => {
                            policy_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PolicyProperties {
                    policy_document: policy_document.ok_or(::serde::de::Error::missing_field("PolicyDocument"))?,
                    policy_name: policy_name,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Policy {
    type Properties = PolicyProperties;
    const TYPE: &'static str = "AWS::IoT::Policy";
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

/// The [`AWS::IoT::PolicyPrincipalAttachment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-policyprincipalattachment.html) resource type.
#[derive(Debug, Default)]
pub struct PolicyPrincipalAttachment {
    properties: PolicyPrincipalAttachmentProperties
}

/// Properties for the `PolicyPrincipalAttachment` resource.
#[derive(Debug, Default)]
pub struct PolicyPrincipalAttachmentProperties {
    /// Property [`PolicyName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-policyprincipalattachment.html#cfn-iot-policyprincipalattachment-policyname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub policy_name: ::Value<String>,
    /// Property [`Principal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-policyprincipalattachment.html#cfn-iot-policyprincipalattachment-principal).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub principal: ::Value<String>,
}

impl ::serde::Serialize for PolicyPrincipalAttachmentProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PolicyName", &self.policy_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Principal", &self.principal)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PolicyPrincipalAttachmentProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PolicyPrincipalAttachmentProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PolicyPrincipalAttachmentProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PolicyPrincipalAttachmentProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut policy_name: Option<::Value<String>> = None;
                let mut principal: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "PolicyName" => {
                            policy_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Principal" => {
                            principal = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PolicyPrincipalAttachmentProperties {
                    policy_name: policy_name.ok_or(::serde::de::Error::missing_field("PolicyName"))?,
                    principal: principal.ok_or(::serde::de::Error::missing_field("Principal"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for PolicyPrincipalAttachment {
    type Properties = PolicyPrincipalAttachmentProperties;
    const TYPE: &'static str = "AWS::IoT::PolicyPrincipalAttachment";
    fn properties(&self) -> &PolicyPrincipalAttachmentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PolicyPrincipalAttachmentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for PolicyPrincipalAttachment {}

impl From<PolicyPrincipalAttachmentProperties> for PolicyPrincipalAttachment {
    fn from(properties: PolicyPrincipalAttachmentProperties) -> PolicyPrincipalAttachment {
        PolicyPrincipalAttachment { properties }
    }
}

/// The [`AWS::IoT::ProvisioningTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-provisioningtemplate.html) resource type.
#[derive(Debug, Default)]
pub struct ProvisioningTemplate {
    properties: ProvisioningTemplateProperties
}

/// Properties for the `ProvisioningTemplate` resource.
#[derive(Debug, Default)]
pub struct ProvisioningTemplateProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-provisioningtemplate.html#cfn-iot-provisioningtemplate-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-provisioningtemplate.html#cfn-iot-provisioningtemplate-enabled).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enabled: Option<::Value<bool>>,
    /// Property [`PreProvisioningHook`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-provisioningtemplate.html#cfn-iot-provisioningtemplate-preprovisioninghook).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub pre_provisioning_hook: Option<::Value<self::provisioning_template::ProvisioningHook>>,
    /// Property [`ProvisioningRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-provisioningtemplate.html#cfn-iot-provisioningtemplate-provisioningrolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub provisioning_role_arn: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-provisioningtemplate.html#cfn-iot-provisioningtemplate-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TemplateBody`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-provisioningtemplate.html#cfn-iot-provisioningtemplate-templatebody).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub template_body: ::Value<String>,
    /// Property [`TemplateName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-provisioningtemplate.html#cfn-iot-provisioningtemplate-templatename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub template_name: Option<::Value<String>>,
    /// Property [`TemplateType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-provisioningtemplate.html#cfn-iot-provisioningtemplate-templatetype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub template_type: Option<::Value<String>>,
}

impl ::serde::Serialize for ProvisioningTemplateProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref enabled) = self.enabled {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
        }
        if let Some(ref pre_provisioning_hook) = self.pre_provisioning_hook {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreProvisioningHook", pre_provisioning_hook)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProvisioningRoleArn", &self.provisioning_role_arn)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemplateBody", &self.template_body)?;
        if let Some(ref template_name) = self.template_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemplateName", template_name)?;
        }
        if let Some(ref template_type) = self.template_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemplateType", template_type)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ProvisioningTemplateProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ProvisioningTemplateProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ProvisioningTemplateProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ProvisioningTemplateProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut enabled: Option<::Value<bool>> = None;
                let mut pre_provisioning_hook: Option<::Value<self::provisioning_template::ProvisioningHook>> = None;
                let mut provisioning_role_arn: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut template_body: Option<::Value<String>> = None;
                let mut template_name: Option<::Value<String>> = None;
                let mut template_type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Enabled" => {
                            enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PreProvisioningHook" => {
                            pre_provisioning_hook = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProvisioningRoleArn" => {
                            provisioning_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TemplateBody" => {
                            template_body = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TemplateName" => {
                            template_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TemplateType" => {
                            template_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ProvisioningTemplateProperties {
                    description: description,
                    enabled: enabled,
                    pre_provisioning_hook: pre_provisioning_hook,
                    provisioning_role_arn: provisioning_role_arn.ok_or(::serde::de::Error::missing_field("ProvisioningRoleArn"))?,
                    tags: tags,
                    template_body: template_body.ok_or(::serde::de::Error::missing_field("TemplateBody"))?,
                    template_name: template_name,
                    template_type: template_type,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ProvisioningTemplate {
    type Properties = ProvisioningTemplateProperties;
    const TYPE: &'static str = "AWS::IoT::ProvisioningTemplate";
    fn properties(&self) -> &ProvisioningTemplateProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ProvisioningTemplateProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ProvisioningTemplate {}

impl From<ProvisioningTemplateProperties> for ProvisioningTemplate {
    fn from(properties: ProvisioningTemplateProperties) -> ProvisioningTemplate {
        ProvisioningTemplate { properties }
    }
}

/// The [`AWS::IoT::ResourceSpecificLogging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-resourcespecificlogging.html) resource type.
#[derive(Debug, Default)]
pub struct ResourceSpecificLogging {
    properties: ResourceSpecificLoggingProperties
}

/// Properties for the `ResourceSpecificLogging` resource.
#[derive(Debug, Default)]
pub struct ResourceSpecificLoggingProperties {
    /// Property [`LogLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-resourcespecificlogging.html#cfn-iot-resourcespecificlogging-loglevel).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub log_level: ::Value<String>,
    /// Property [`TargetName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-resourcespecificlogging.html#cfn-iot-resourcespecificlogging-targetname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub target_name: ::Value<String>,
    /// Property [`TargetType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-resourcespecificlogging.html#cfn-iot-resourcespecificlogging-targettype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub target_type: ::Value<String>,
}

impl ::serde::Serialize for ResourceSpecificLoggingProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogLevel", &self.log_level)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetName", &self.target_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetType", &self.target_type)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ResourceSpecificLoggingProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourceSpecificLoggingProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ResourceSpecificLoggingProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ResourceSpecificLoggingProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut log_level: Option<::Value<String>> = None;
                let mut target_name: Option<::Value<String>> = None;
                let mut target_type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "LogLevel" => {
                            log_level = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetName" => {
                            target_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetType" => {
                            target_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ResourceSpecificLoggingProperties {
                    log_level: log_level.ok_or(::serde::de::Error::missing_field("LogLevel"))?,
                    target_name: target_name.ok_or(::serde::de::Error::missing_field("TargetName"))?,
                    target_type: target_type.ok_or(::serde::de::Error::missing_field("TargetType"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ResourceSpecificLogging {
    type Properties = ResourceSpecificLoggingProperties;
    const TYPE: &'static str = "AWS::IoT::ResourceSpecificLogging";
    fn properties(&self) -> &ResourceSpecificLoggingProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ResourceSpecificLoggingProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ResourceSpecificLogging {}

impl From<ResourceSpecificLoggingProperties> for ResourceSpecificLogging {
    fn from(properties: ResourceSpecificLoggingProperties) -> ResourceSpecificLogging {
        ResourceSpecificLogging { properties }
    }
}

/// The [`AWS::IoT::RoleAlias`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-rolealias.html) resource type.
#[derive(Debug, Default)]
pub struct RoleAlias {
    properties: RoleAliasProperties
}

/// Properties for the `RoleAlias` resource.
#[derive(Debug, Default)]
pub struct RoleAliasProperties {
    /// Property [`CredentialDurationSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-rolealias.html#cfn-iot-rolealias-credentialdurationseconds).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub credential_duration_seconds: Option<::Value<u32>>,
    /// Property [`RoleAlias`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-rolealias.html#cfn-iot-rolealias-rolealias).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub role_alias: Option<::Value<String>>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-rolealias.html#cfn-iot-rolealias-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-rolealias.html#cfn-iot-rolealias-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for RoleAliasProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref credential_duration_seconds) = self.credential_duration_seconds {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CredentialDurationSeconds", credential_duration_seconds)?;
        }
        if let Some(ref role_alias) = self.role_alias {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleAlias", role_alias)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for RoleAliasProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<RoleAliasProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = RoleAliasProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type RoleAliasProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut credential_duration_seconds: Option<::Value<u32>> = None;
                let mut role_alias: Option<::Value<String>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CredentialDurationSeconds" => {
                            credential_duration_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleAlias" => {
                            role_alias = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(RoleAliasProperties {
                    credential_duration_seconds: credential_duration_seconds,
                    role_alias: role_alias,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for RoleAlias {
    type Properties = RoleAliasProperties;
    const TYPE: &'static str = "AWS::IoT::RoleAlias";
    fn properties(&self) -> &RoleAliasProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut RoleAliasProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for RoleAlias {}

impl From<RoleAliasProperties> for RoleAlias {
    fn from(properties: RoleAliasProperties) -> RoleAlias {
        RoleAlias { properties }
    }
}

/// The [`AWS::IoT::ScheduledAudit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-scheduledaudit.html) resource type.
#[derive(Debug, Default)]
pub struct ScheduledAudit {
    properties: ScheduledAuditProperties
}

/// Properties for the `ScheduledAudit` resource.
#[derive(Debug, Default)]
pub struct ScheduledAuditProperties {
    /// Property [`DayOfMonth`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-scheduledaudit.html#cfn-iot-scheduledaudit-dayofmonth).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub day_of_month: Option<::Value<String>>,
    /// Property [`DayOfWeek`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-scheduledaudit.html#cfn-iot-scheduledaudit-dayofweek).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub day_of_week: Option<::Value<String>>,
    /// Property [`Frequency`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-scheduledaudit.html#cfn-iot-scheduledaudit-frequency).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub frequency: ::Value<String>,
    /// Property [`ScheduledAuditName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-scheduledaudit.html#cfn-iot-scheduledaudit-scheduledauditname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub scheduled_audit_name: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-scheduledaudit.html#cfn-iot-scheduledaudit-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TargetCheckNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-scheduledaudit.html#cfn-iot-scheduledaudit-targetchecknames).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub target_check_names: ::ValueList<String>,
}

impl ::serde::Serialize for ScheduledAuditProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref day_of_month) = self.day_of_month {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DayOfMonth", day_of_month)?;
        }
        if let Some(ref day_of_week) = self.day_of_week {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DayOfWeek", day_of_week)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Frequency", &self.frequency)?;
        if let Some(ref scheduled_audit_name) = self.scheduled_audit_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScheduledAuditName", scheduled_audit_name)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetCheckNames", &self.target_check_names)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ScheduledAuditProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ScheduledAuditProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ScheduledAuditProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ScheduledAuditProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut day_of_month: Option<::Value<String>> = None;
                let mut day_of_week: Option<::Value<String>> = None;
                let mut frequency: Option<::Value<String>> = None;
                let mut scheduled_audit_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut target_check_names: Option<::ValueList<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DayOfMonth" => {
                            day_of_month = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DayOfWeek" => {
                            day_of_week = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Frequency" => {
                            frequency = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ScheduledAuditName" => {
                            scheduled_audit_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetCheckNames" => {
                            target_check_names = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ScheduledAuditProperties {
                    day_of_month: day_of_month,
                    day_of_week: day_of_week,
                    frequency: frequency.ok_or(::serde::de::Error::missing_field("Frequency"))?,
                    scheduled_audit_name: scheduled_audit_name,
                    tags: tags,
                    target_check_names: target_check_names.ok_or(::serde::de::Error::missing_field("TargetCheckNames"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ScheduledAudit {
    type Properties = ScheduledAuditProperties;
    const TYPE: &'static str = "AWS::IoT::ScheduledAudit";
    fn properties(&self) -> &ScheduledAuditProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ScheduledAuditProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ScheduledAudit {}

impl From<ScheduledAuditProperties> for ScheduledAudit {
    fn from(properties: ScheduledAuditProperties) -> ScheduledAudit {
        ScheduledAudit { properties }
    }
}

/// The [`AWS::IoT::SecurityProfile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-securityprofile.html) resource type.
#[derive(Debug, Default)]
pub struct SecurityProfile {
    properties: SecurityProfileProperties
}

/// Properties for the `SecurityProfile` resource.
#[derive(Debug, Default)]
pub struct SecurityProfileProperties {
    /// Property [`AdditionalMetricsToRetainV2`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-securityprofile.html#cfn-iot-securityprofile-additionalmetricstoretainv2).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub additional_metrics_to_retain_v2: Option<::ValueList<self::security_profile::MetricToRetain>>,
    /// Property [`AlertTargets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-securityprofile.html#cfn-iot-securityprofile-alerttargets).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub alert_targets: Option<::ValueMap<self::security_profile::AlertTarget>>,
    /// Property [`Behaviors`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-securityprofile.html#cfn-iot-securityprofile-behaviors).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub behaviors: Option<::ValueList<self::security_profile::Behavior>>,
    /// Property [`MetricsExportConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-securityprofile.html#cfn-iot-securityprofile-metricsexportconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub metrics_export_config: Option<::Value<self::security_profile::MetricsExportConfig>>,
    /// Property [`SecurityProfileDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-securityprofile.html#cfn-iot-securityprofile-securityprofiledescription).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub security_profile_description: Option<::Value<String>>,
    /// Property [`SecurityProfileName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-securityprofile.html#cfn-iot-securityprofile-securityprofilename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub security_profile_name: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-securityprofile.html#cfn-iot-securityprofile-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TargetArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-securityprofile.html#cfn-iot-securityprofile-targetarns).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub target_arns: Option<::ValueList<String>>,
}

impl ::serde::Serialize for SecurityProfileProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref additional_metrics_to_retain_v2) = self.additional_metrics_to_retain_v2 {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdditionalMetricsToRetainV2", additional_metrics_to_retain_v2)?;
        }
        if let Some(ref alert_targets) = self.alert_targets {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AlertTargets", alert_targets)?;
        }
        if let Some(ref behaviors) = self.behaviors {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Behaviors", behaviors)?;
        }
        if let Some(ref metrics_export_config) = self.metrics_export_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricsExportConfig", metrics_export_config)?;
        }
        if let Some(ref security_profile_description) = self.security_profile_description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityProfileDescription", security_profile_description)?;
        }
        if let Some(ref security_profile_name) = self.security_profile_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityProfileName", security_profile_name)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref target_arns) = self.target_arns {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetArns", target_arns)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SecurityProfileProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SecurityProfileProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SecurityProfileProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SecurityProfileProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut additional_metrics_to_retain_v2: Option<::ValueList<self::security_profile::MetricToRetain>> = None;
                let mut alert_targets: Option<::ValueMap<self::security_profile::AlertTarget>> = None;
                let mut behaviors: Option<::ValueList<self::security_profile::Behavior>> = None;
                let mut metrics_export_config: Option<::Value<self::security_profile::MetricsExportConfig>> = None;
                let mut security_profile_description: Option<::Value<String>> = None;
                let mut security_profile_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut target_arns: Option<::ValueList<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AdditionalMetricsToRetainV2" => {
                            additional_metrics_to_retain_v2 = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AlertTargets" => {
                            alert_targets = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Behaviors" => {
                            behaviors = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MetricsExportConfig" => {
                            metrics_export_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecurityProfileDescription" => {
                            security_profile_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecurityProfileName" => {
                            security_profile_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetArns" => {
                            target_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SecurityProfileProperties {
                    additional_metrics_to_retain_v2: additional_metrics_to_retain_v2,
                    alert_targets: alert_targets,
                    behaviors: behaviors,
                    metrics_export_config: metrics_export_config,
                    security_profile_description: security_profile_description,
                    security_profile_name: security_profile_name,
                    tags: tags,
                    target_arns: target_arns,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for SecurityProfile {
    type Properties = SecurityProfileProperties;
    const TYPE: &'static str = "AWS::IoT::SecurityProfile";
    fn properties(&self) -> &SecurityProfileProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SecurityProfileProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SecurityProfile {}

impl From<SecurityProfileProperties> for SecurityProfile {
    fn from(properties: SecurityProfileProperties) -> SecurityProfile {
        SecurityProfile { properties }
    }
}

/// The [`AWS::IoT::SoftwarePackage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-softwarepackage.html) resource type.
#[derive(Debug, Default)]
pub struct SoftwarePackage {
    properties: SoftwarePackageProperties
}

/// Properties for the `SoftwarePackage` resource.
#[derive(Debug, Default)]
pub struct SoftwarePackageProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-softwarepackage.html#cfn-iot-softwarepackage-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`PackageName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-softwarepackage.html#cfn-iot-softwarepackage-packagename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub package_name: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-softwarepackage.html#cfn-iot-softwarepackage-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for SoftwarePackageProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref package_name) = self.package_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PackageName", package_name)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SoftwarePackageProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SoftwarePackageProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SoftwarePackageProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SoftwarePackageProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut package_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PackageName" => {
                            package_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SoftwarePackageProperties {
                    description: description,
                    package_name: package_name,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for SoftwarePackage {
    type Properties = SoftwarePackageProperties;
    const TYPE: &'static str = "AWS::IoT::SoftwarePackage";
    fn properties(&self) -> &SoftwarePackageProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SoftwarePackageProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SoftwarePackage {}

impl From<SoftwarePackageProperties> for SoftwarePackage {
    fn from(properties: SoftwarePackageProperties) -> SoftwarePackage {
        SoftwarePackage { properties }
    }
}

/// The [`AWS::IoT::SoftwarePackageVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-softwarepackageversion.html) resource type.
#[derive(Debug, Default)]
pub struct SoftwarePackageVersion {
    properties: SoftwarePackageVersionProperties
}

/// Properties for the `SoftwarePackageVersion` resource.
#[derive(Debug, Default)]
pub struct SoftwarePackageVersionProperties {
    /// Property [`Attributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-softwarepackageversion.html#cfn-iot-softwarepackageversion-attributes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub attributes: Option<::ValueMap<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-softwarepackageversion.html#cfn-iot-softwarepackageversion-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`PackageName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-softwarepackageversion.html#cfn-iot-softwarepackageversion-packagename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub package_name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-softwarepackageversion.html#cfn-iot-softwarepackageversion-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`VersionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-softwarepackageversion.html#cfn-iot-softwarepackageversion-versionname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub version_name: Option<::Value<String>>,
}

impl ::serde::Serialize for SoftwarePackageVersionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref attributes) = self.attributes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Attributes", attributes)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PackageName", &self.package_name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref version_name) = self.version_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VersionName", version_name)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for SoftwarePackageVersionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<SoftwarePackageVersionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = SoftwarePackageVersionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type SoftwarePackageVersionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut attributes: Option<::ValueMap<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut package_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut version_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Attributes" => {
                            attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PackageName" => {
                            package_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VersionName" => {
                            version_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(SoftwarePackageVersionProperties {
                    attributes: attributes,
                    description: description,
                    package_name: package_name.ok_or(::serde::de::Error::missing_field("PackageName"))?,
                    tags: tags,
                    version_name: version_name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for SoftwarePackageVersion {
    type Properties = SoftwarePackageVersionProperties;
    const TYPE: &'static str = "AWS::IoT::SoftwarePackageVersion";
    fn properties(&self) -> &SoftwarePackageVersionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut SoftwarePackageVersionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for SoftwarePackageVersion {}

impl From<SoftwarePackageVersionProperties> for SoftwarePackageVersion {
    fn from(properties: SoftwarePackageVersionProperties) -> SoftwarePackageVersion {
        SoftwarePackageVersion { properties }
    }
}

/// The [`AWS::IoT::Thing`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-thing.html) resource type.
#[derive(Debug, Default)]
pub struct Thing {
    properties: ThingProperties
}

/// Properties for the `Thing` resource.
#[derive(Debug, Default)]
pub struct ThingProperties {
    /// Property [`AttributePayload`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-thing.html#cfn-iot-thing-attributepayload).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub attribute_payload: Option<::Value<self::thing::AttributePayload>>,
    /// Property [`ThingName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-thing.html#cfn-iot-thing-thingname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub thing_name: Option<::Value<String>>,
}

impl ::serde::Serialize for ThingProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref attribute_payload) = self.attribute_payload {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttributePayload", attribute_payload)?;
        }
        if let Some(ref thing_name) = self.thing_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThingName", thing_name)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ThingProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ThingProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ThingProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ThingProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut attribute_payload: Option<::Value<self::thing::AttributePayload>> = None;
                let mut thing_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AttributePayload" => {
                            attribute_payload = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ThingName" => {
                            thing_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ThingProperties {
                    attribute_payload: attribute_payload,
                    thing_name: thing_name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Thing {
    type Properties = ThingProperties;
    const TYPE: &'static str = "AWS::IoT::Thing";
    fn properties(&self) -> &ThingProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ThingProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Thing {}

impl From<ThingProperties> for Thing {
    fn from(properties: ThingProperties) -> Thing {
        Thing { properties }
    }
}

/// The [`AWS::IoT::ThingGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-thinggroup.html) resource type.
#[derive(Debug, Default)]
pub struct ThingGroup {
    properties: ThingGroupProperties
}

/// Properties for the `ThingGroup` resource.
#[derive(Debug, Default)]
pub struct ThingGroupProperties {
    /// Property [`ParentGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-thinggroup.html#cfn-iot-thinggroup-parentgroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub parent_group_name: Option<::Value<String>>,
    /// Property [`QueryString`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-thinggroup.html#cfn-iot-thinggroup-querystring).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub query_string: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-thinggroup.html#cfn-iot-thinggroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`ThingGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-thinggroup.html#cfn-iot-thinggroup-thinggroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub thing_group_name: Option<::Value<String>>,
    /// Property [`ThingGroupProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-thinggroup.html#cfn-iot-thinggroup-thinggroupproperties).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub thing_group_properties: Option<::Value<self::thing_group::ThingGroupProperties>>,
}

impl ::serde::Serialize for ThingGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref parent_group_name) = self.parent_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParentGroupName", parent_group_name)?;
        }
        if let Some(ref query_string) = self.query_string {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueryString", query_string)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref thing_group_name) = self.thing_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThingGroupName", thing_group_name)?;
        }
        if let Some(ref thing_group_properties) = self.thing_group_properties {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThingGroupProperties", thing_group_properties)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ThingGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ThingGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ThingGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ThingGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut parent_group_name: Option<::Value<String>> = None;
                let mut query_string: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut thing_group_name: Option<::Value<String>> = None;
                let mut thing_group_properties: Option<::Value<self::thing_group::ThingGroupProperties>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ParentGroupName" => {
                            parent_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "QueryString" => {
                            query_string = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ThingGroupName" => {
                            thing_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ThingGroupProperties" => {
                            thing_group_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ThingGroupProperties {
                    parent_group_name: parent_group_name,
                    query_string: query_string,
                    tags: tags,
                    thing_group_name: thing_group_name,
                    thing_group_properties: thing_group_properties,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ThingGroup {
    type Properties = ThingGroupProperties;
    const TYPE: &'static str = "AWS::IoT::ThingGroup";
    fn properties(&self) -> &ThingGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ThingGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ThingGroup {}

impl From<ThingGroupProperties> for ThingGroup {
    fn from(properties: ThingGroupProperties) -> ThingGroup {
        ThingGroup { properties }
    }
}

/// The [`AWS::IoT::ThingPrincipalAttachment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-thingprincipalattachment.html) resource type.
#[derive(Debug, Default)]
pub struct ThingPrincipalAttachment {
    properties: ThingPrincipalAttachmentProperties
}

/// Properties for the `ThingPrincipalAttachment` resource.
#[derive(Debug, Default)]
pub struct ThingPrincipalAttachmentProperties {
    /// Property [`Principal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-thingprincipalattachment.html#cfn-iot-thingprincipalattachment-principal).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub principal: ::Value<String>,
    /// Property [`ThingName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-thingprincipalattachment.html#cfn-iot-thingprincipalattachment-thingname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub thing_name: ::Value<String>,
}

impl ::serde::Serialize for ThingPrincipalAttachmentProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Principal", &self.principal)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThingName", &self.thing_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ThingPrincipalAttachmentProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ThingPrincipalAttachmentProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ThingPrincipalAttachmentProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ThingPrincipalAttachmentProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut principal: Option<::Value<String>> = None;
                let mut thing_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Principal" => {
                            principal = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ThingName" => {
                            thing_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ThingPrincipalAttachmentProperties {
                    principal: principal.ok_or(::serde::de::Error::missing_field("Principal"))?,
                    thing_name: thing_name.ok_or(::serde::de::Error::missing_field("ThingName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ThingPrincipalAttachment {
    type Properties = ThingPrincipalAttachmentProperties;
    const TYPE: &'static str = "AWS::IoT::ThingPrincipalAttachment";
    fn properties(&self) -> &ThingPrincipalAttachmentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ThingPrincipalAttachmentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ThingPrincipalAttachment {}

impl From<ThingPrincipalAttachmentProperties> for ThingPrincipalAttachment {
    fn from(properties: ThingPrincipalAttachmentProperties) -> ThingPrincipalAttachment {
        ThingPrincipalAttachment { properties }
    }
}

/// The [`AWS::IoT::ThingType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-thingtype.html) resource type.
#[derive(Debug, Default)]
pub struct ThingType {
    properties: ThingTypeProperties
}

/// Properties for the `ThingType` resource.
#[derive(Debug, Default)]
pub struct ThingTypeProperties {
    /// Property [`DeprecateThingType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-thingtype.html#cfn-iot-thingtype-deprecatethingtype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub deprecate_thing_type: Option<::Value<bool>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-thingtype.html#cfn-iot-thingtype-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`ThingTypeName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-thingtype.html#cfn-iot-thingtype-thingtypename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub thing_type_name: Option<::Value<String>>,
    /// Property [`ThingTypeProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-thingtype.html#cfn-iot-thingtype-thingtypeproperties).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub thing_type_properties: Option<::Value<self::thing_type::ThingTypeProperties>>,
}

impl ::serde::Serialize for ThingTypeProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref deprecate_thing_type) = self.deprecate_thing_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeprecateThingType", deprecate_thing_type)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref thing_type_name) = self.thing_type_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThingTypeName", thing_type_name)?;
        }
        if let Some(ref thing_type_properties) = self.thing_type_properties {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThingTypeProperties", thing_type_properties)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ThingTypeProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ThingTypeProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ThingTypeProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ThingTypeProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut deprecate_thing_type: Option<::Value<bool>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut thing_type_name: Option<::Value<String>> = None;
                let mut thing_type_properties: Option<::Value<self::thing_type::ThingTypeProperties>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DeprecateThingType" => {
                            deprecate_thing_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ThingTypeName" => {
                            thing_type_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ThingTypeProperties" => {
                            thing_type_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ThingTypeProperties {
                    deprecate_thing_type: deprecate_thing_type,
                    tags: tags,
                    thing_type_name: thing_type_name,
                    thing_type_properties: thing_type_properties,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ThingType {
    type Properties = ThingTypeProperties;
    const TYPE: &'static str = "AWS::IoT::ThingType";
    fn properties(&self) -> &ThingTypeProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ThingTypeProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ThingType {}

impl From<ThingTypeProperties> for ThingType {
    fn from(properties: ThingTypeProperties) -> ThingType {
        ThingType { properties }
    }
}

/// The [`AWS::IoT::TopicRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-topicrule.html) resource type.
#[derive(Debug, Default)]
pub struct TopicRule {
    properties: TopicRuleProperties
}

/// Properties for the `TopicRule` resource.
#[derive(Debug, Default)]
pub struct TopicRuleProperties {
    /// Property [`RuleName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-topicrule.html#cfn-iot-topicrule-rulename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub rule_name: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-topicrule.html#cfn-iot-topicrule-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TopicRulePayload`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-topicrule.html#cfn-iot-topicrule-topicrulepayload).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub topic_rule_payload: ::Value<self::topic_rule::TopicRulePayload>,
}

impl ::serde::Serialize for TopicRuleProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref rule_name) = self.rule_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleName", rule_name)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TopicRulePayload", &self.topic_rule_payload)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for TopicRuleProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<TopicRuleProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TopicRuleProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type TopicRuleProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut rule_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut topic_rule_payload: Option<::Value<self::topic_rule::TopicRulePayload>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "RuleName" => {
                            rule_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TopicRulePayload" => {
                            topic_rule_payload = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(TopicRuleProperties {
                    rule_name: rule_name,
                    tags: tags,
                    topic_rule_payload: topic_rule_payload.ok_or(::serde::de::Error::missing_field("TopicRulePayload"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for TopicRule {
    type Properties = TopicRuleProperties;
    const TYPE: &'static str = "AWS::IoT::TopicRule";
    fn properties(&self) -> &TopicRuleProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TopicRuleProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for TopicRule {}

impl From<TopicRuleProperties> for TopicRule {
    fn from(properties: TopicRuleProperties) -> TopicRule {
        TopicRule { properties }
    }
}

/// The [`AWS::IoT::TopicRuleDestination`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-topicruledestination.html) resource type.
#[derive(Debug, Default)]
pub struct TopicRuleDestination {
    properties: TopicRuleDestinationProperties
}

/// Properties for the `TopicRuleDestination` resource.
#[derive(Debug, Default)]
pub struct TopicRuleDestinationProperties {
    /// Property [`HttpUrlProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-topicruledestination.html#cfn-iot-topicruledestination-httpurlproperties).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub http_url_properties: Option<::Value<self::topic_rule_destination::HttpUrlDestinationSummary>>,
    /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-topicruledestination.html#cfn-iot-topicruledestination-status).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub status: Option<::Value<String>>,
    /// Property [`VpcProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iot-topicruledestination.html#cfn-iot-topicruledestination-vpcproperties).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub vpc_properties: Option<::Value<self::topic_rule_destination::VpcDestinationProperties>>,
}

impl ::serde::Serialize for TopicRuleDestinationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref http_url_properties) = self.http_url_properties {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HttpUrlProperties", http_url_properties)?;
        }
        if let Some(ref status) = self.status {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", status)?;
        }
        if let Some(ref vpc_properties) = self.vpc_properties {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcProperties", vpc_properties)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for TopicRuleDestinationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<TopicRuleDestinationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TopicRuleDestinationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type TopicRuleDestinationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut http_url_properties: Option<::Value<self::topic_rule_destination::HttpUrlDestinationSummary>> = None;
                let mut status: Option<::Value<String>> = None;
                let mut vpc_properties: Option<::Value<self::topic_rule_destination::VpcDestinationProperties>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "HttpUrlProperties" => {
                            http_url_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Status" => {
                            status = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcProperties" => {
                            vpc_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(TopicRuleDestinationProperties {
                    http_url_properties: http_url_properties,
                    status: status,
                    vpc_properties: vpc_properties,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for TopicRuleDestination {
    type Properties = TopicRuleDestinationProperties;
    const TYPE: &'static str = "AWS::IoT::TopicRuleDestination";
    fn properties(&self) -> &TopicRuleDestinationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TopicRuleDestinationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for TopicRuleDestination {}

impl From<TopicRuleDestinationProperties> for TopicRuleDestination {
    fn from(properties: TopicRuleDestinationProperties) -> TopicRuleDestination {
        TopicRuleDestination { properties }
    }
}

pub mod account_audit_configuration {
    //! Property types for the `AccountAuditConfiguration` resource.

    /// The [`AWS::IoT::AccountAuditConfiguration.AuditCheckConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-accountauditconfiguration-auditcheckconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct AuditCheckConfiguration {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-accountauditconfiguration-auditcheckconfiguration.html#cfn-iot-accountauditconfiguration-auditcheckconfiguration-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for AuditCheckConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enabled) = self.enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AuditCheckConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AuditCheckConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AuditCheckConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AuditCheckConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AuditCheckConfiguration {
                        enabled: enabled,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::AccountAuditConfiguration.AuditCheckConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-accountauditconfiguration-auditcheckconfigurations.html) property type.
    #[derive(Debug, Default)]
    pub struct AuditCheckConfigurations {
        /// Property [`AuthenticatedCognitoRoleOverlyPermissiveCheck`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-accountauditconfiguration-auditcheckconfigurations.html#cfn-iot-accountauditconfiguration-auditcheckconfigurations-authenticatedcognitoroleoverlypermissivecheck).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub authenticated_cognito_role_overly_permissive_check: Option<::Value<AuditCheckConfiguration>>,
        /// Property [`CaCertificateExpiringCheck`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-accountauditconfiguration-auditcheckconfigurations.html#cfn-iot-accountauditconfiguration-auditcheckconfigurations-cacertificateexpiringcheck).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ca_certificate_expiring_check: Option<::Value<AuditCheckConfiguration>>,
        /// Property [`CaCertificateKeyQualityCheck`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-accountauditconfiguration-auditcheckconfigurations.html#cfn-iot-accountauditconfiguration-auditcheckconfigurations-cacertificatekeyqualitycheck).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ca_certificate_key_quality_check: Option<::Value<AuditCheckConfiguration>>,
        /// Property [`ConflictingClientIdsCheck`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-accountauditconfiguration-auditcheckconfigurations.html#cfn-iot-accountauditconfiguration-auditcheckconfigurations-conflictingclientidscheck).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub conflicting_client_ids_check: Option<::Value<AuditCheckConfiguration>>,
        /// Property [`DeviceCertificateExpiringCheck`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-accountauditconfiguration-auditcheckconfigurations.html#cfn-iot-accountauditconfiguration-auditcheckconfigurations-devicecertificateexpiringcheck).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub device_certificate_expiring_check: Option<::Value<AuditCheckConfiguration>>,
        /// Property [`DeviceCertificateKeyQualityCheck`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-accountauditconfiguration-auditcheckconfigurations.html#cfn-iot-accountauditconfiguration-auditcheckconfigurations-devicecertificatekeyqualitycheck).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub device_certificate_key_quality_check: Option<::Value<AuditCheckConfiguration>>,
        /// Property [`DeviceCertificateSharedCheck`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-accountauditconfiguration-auditcheckconfigurations.html#cfn-iot-accountauditconfiguration-auditcheckconfigurations-devicecertificatesharedcheck).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub device_certificate_shared_check: Option<::Value<AuditCheckConfiguration>>,
        /// Property [`IntermediateCaRevokedForActiveDeviceCertificatesCheck`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-accountauditconfiguration-auditcheckconfigurations.html#cfn-iot-accountauditconfiguration-auditcheckconfigurations-intermediatecarevokedforactivedevicecertificatescheck).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub intermediate_ca_revoked_for_active_device_certificates_check: Option<::Value<AuditCheckConfiguration>>,
        /// Property [`IoTPolicyPotentialMisConfigurationCheck`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-accountauditconfiguration-auditcheckconfigurations.html#cfn-iot-accountauditconfiguration-auditcheckconfigurations-iotpolicypotentialmisconfigurationcheck).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub io_t_policy_potential_mis_configuration_check: Option<::Value<AuditCheckConfiguration>>,
        /// Property [`IotPolicyOverlyPermissiveCheck`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-accountauditconfiguration-auditcheckconfigurations.html#cfn-iot-accountauditconfiguration-auditcheckconfigurations-iotpolicyoverlypermissivecheck).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub iot_policy_overly_permissive_check: Option<::Value<AuditCheckConfiguration>>,
        /// Property [`IotRoleAliasAllowsAccessToUnusedServicesCheck`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-accountauditconfiguration-auditcheckconfigurations.html#cfn-iot-accountauditconfiguration-auditcheckconfigurations-iotrolealiasallowsaccesstounusedservicescheck).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub iot_role_alias_allows_access_to_unused_services_check: Option<::Value<AuditCheckConfiguration>>,
        /// Property [`IotRoleAliasOverlyPermissiveCheck`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-accountauditconfiguration-auditcheckconfigurations.html#cfn-iot-accountauditconfiguration-auditcheckconfigurations-iotrolealiasoverlypermissivecheck).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub iot_role_alias_overly_permissive_check: Option<::Value<AuditCheckConfiguration>>,
        /// Property [`LoggingDisabledCheck`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-accountauditconfiguration-auditcheckconfigurations.html#cfn-iot-accountauditconfiguration-auditcheckconfigurations-loggingdisabledcheck).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub logging_disabled_check: Option<::Value<AuditCheckConfiguration>>,
        /// Property [`RevokedCaCertificateStillActiveCheck`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-accountauditconfiguration-auditcheckconfigurations.html#cfn-iot-accountauditconfiguration-auditcheckconfigurations-revokedcacertificatestillactivecheck).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub revoked_ca_certificate_still_active_check: Option<::Value<AuditCheckConfiguration>>,
        /// Property [`RevokedDeviceCertificateStillActiveCheck`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-accountauditconfiguration-auditcheckconfigurations.html#cfn-iot-accountauditconfiguration-auditcheckconfigurations-revokeddevicecertificatestillactivecheck).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub revoked_device_certificate_still_active_check: Option<::Value<AuditCheckConfiguration>>,
        /// Property [`UnauthenticatedCognitoRoleOverlyPermissiveCheck`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-accountauditconfiguration-auditcheckconfigurations.html#cfn-iot-accountauditconfiguration-auditcheckconfigurations-unauthenticatedcognitoroleoverlypermissivecheck).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unauthenticated_cognito_role_overly_permissive_check: Option<::Value<AuditCheckConfiguration>>,
    }

    impl ::codec::SerializeValue for AuditCheckConfigurations {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref authenticated_cognito_role_overly_permissive_check) = self.authenticated_cognito_role_overly_permissive_check {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthenticatedCognitoRoleOverlyPermissiveCheck", authenticated_cognito_role_overly_permissive_check)?;
            }
            if let Some(ref ca_certificate_expiring_check) = self.ca_certificate_expiring_check {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CaCertificateExpiringCheck", ca_certificate_expiring_check)?;
            }
            if let Some(ref ca_certificate_key_quality_check) = self.ca_certificate_key_quality_check {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CaCertificateKeyQualityCheck", ca_certificate_key_quality_check)?;
            }
            if let Some(ref conflicting_client_ids_check) = self.conflicting_client_ids_check {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConflictingClientIdsCheck", conflicting_client_ids_check)?;
            }
            if let Some(ref device_certificate_expiring_check) = self.device_certificate_expiring_check {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceCertificateExpiringCheck", device_certificate_expiring_check)?;
            }
            if let Some(ref device_certificate_key_quality_check) = self.device_certificate_key_quality_check {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceCertificateKeyQualityCheck", device_certificate_key_quality_check)?;
            }
            if let Some(ref device_certificate_shared_check) = self.device_certificate_shared_check {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceCertificateSharedCheck", device_certificate_shared_check)?;
            }
            if let Some(ref intermediate_ca_revoked_for_active_device_certificates_check) = self.intermediate_ca_revoked_for_active_device_certificates_check {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntermediateCaRevokedForActiveDeviceCertificatesCheck", intermediate_ca_revoked_for_active_device_certificates_check)?;
            }
            if let Some(ref io_t_policy_potential_mis_configuration_check) = self.io_t_policy_potential_mis_configuration_check {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IoTPolicyPotentialMisConfigurationCheck", io_t_policy_potential_mis_configuration_check)?;
            }
            if let Some(ref iot_policy_overly_permissive_check) = self.iot_policy_overly_permissive_check {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IotPolicyOverlyPermissiveCheck", iot_policy_overly_permissive_check)?;
            }
            if let Some(ref iot_role_alias_allows_access_to_unused_services_check) = self.iot_role_alias_allows_access_to_unused_services_check {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IotRoleAliasAllowsAccessToUnusedServicesCheck", iot_role_alias_allows_access_to_unused_services_check)?;
            }
            if let Some(ref iot_role_alias_overly_permissive_check) = self.iot_role_alias_overly_permissive_check {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IotRoleAliasOverlyPermissiveCheck", iot_role_alias_overly_permissive_check)?;
            }
            if let Some(ref logging_disabled_check) = self.logging_disabled_check {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoggingDisabledCheck", logging_disabled_check)?;
            }
            if let Some(ref revoked_ca_certificate_still_active_check) = self.revoked_ca_certificate_still_active_check {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RevokedCaCertificateStillActiveCheck", revoked_ca_certificate_still_active_check)?;
            }
            if let Some(ref revoked_device_certificate_still_active_check) = self.revoked_device_certificate_still_active_check {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RevokedDeviceCertificateStillActiveCheck", revoked_device_certificate_still_active_check)?;
            }
            if let Some(ref unauthenticated_cognito_role_overly_permissive_check) = self.unauthenticated_cognito_role_overly_permissive_check {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UnauthenticatedCognitoRoleOverlyPermissiveCheck", unauthenticated_cognito_role_overly_permissive_check)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AuditCheckConfigurations {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AuditCheckConfigurations, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AuditCheckConfigurations;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AuditCheckConfigurations")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut authenticated_cognito_role_overly_permissive_check: Option<::Value<AuditCheckConfiguration>> = None;
                    let mut ca_certificate_expiring_check: Option<::Value<AuditCheckConfiguration>> = None;
                    let mut ca_certificate_key_quality_check: Option<::Value<AuditCheckConfiguration>> = None;
                    let mut conflicting_client_ids_check: Option<::Value<AuditCheckConfiguration>> = None;
                    let mut device_certificate_expiring_check: Option<::Value<AuditCheckConfiguration>> = None;
                    let mut device_certificate_key_quality_check: Option<::Value<AuditCheckConfiguration>> = None;
                    let mut device_certificate_shared_check: Option<::Value<AuditCheckConfiguration>> = None;
                    let mut intermediate_ca_revoked_for_active_device_certificates_check: Option<::Value<AuditCheckConfiguration>> = None;
                    let mut io_t_policy_potential_mis_configuration_check: Option<::Value<AuditCheckConfiguration>> = None;
                    let mut iot_policy_overly_permissive_check: Option<::Value<AuditCheckConfiguration>> = None;
                    let mut iot_role_alias_allows_access_to_unused_services_check: Option<::Value<AuditCheckConfiguration>> = None;
                    let mut iot_role_alias_overly_permissive_check: Option<::Value<AuditCheckConfiguration>> = None;
                    let mut logging_disabled_check: Option<::Value<AuditCheckConfiguration>> = None;
                    let mut revoked_ca_certificate_still_active_check: Option<::Value<AuditCheckConfiguration>> = None;
                    let mut revoked_device_certificate_still_active_check: Option<::Value<AuditCheckConfiguration>> = None;
                    let mut unauthenticated_cognito_role_overly_permissive_check: Option<::Value<AuditCheckConfiguration>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AuthenticatedCognitoRoleOverlyPermissiveCheck" => {
                                authenticated_cognito_role_overly_permissive_check = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CaCertificateExpiringCheck" => {
                                ca_certificate_expiring_check = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CaCertificateKeyQualityCheck" => {
                                ca_certificate_key_quality_check = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConflictingClientIdsCheck" => {
                                conflicting_client_ids_check = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeviceCertificateExpiringCheck" => {
                                device_certificate_expiring_check = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeviceCertificateKeyQualityCheck" => {
                                device_certificate_key_quality_check = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeviceCertificateSharedCheck" => {
                                device_certificate_shared_check = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IntermediateCaRevokedForActiveDeviceCertificatesCheck" => {
                                intermediate_ca_revoked_for_active_device_certificates_check = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IoTPolicyPotentialMisConfigurationCheck" => {
                                io_t_policy_potential_mis_configuration_check = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IotPolicyOverlyPermissiveCheck" => {
                                iot_policy_overly_permissive_check = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IotRoleAliasAllowsAccessToUnusedServicesCheck" => {
                                iot_role_alias_allows_access_to_unused_services_check = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IotRoleAliasOverlyPermissiveCheck" => {
                                iot_role_alias_overly_permissive_check = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LoggingDisabledCheck" => {
                                logging_disabled_check = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RevokedCaCertificateStillActiveCheck" => {
                                revoked_ca_certificate_still_active_check = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RevokedDeviceCertificateStillActiveCheck" => {
                                revoked_device_certificate_still_active_check = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UnauthenticatedCognitoRoleOverlyPermissiveCheck" => {
                                unauthenticated_cognito_role_overly_permissive_check = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AuditCheckConfigurations {
                        authenticated_cognito_role_overly_permissive_check: authenticated_cognito_role_overly_permissive_check,
                        ca_certificate_expiring_check: ca_certificate_expiring_check,
                        ca_certificate_key_quality_check: ca_certificate_key_quality_check,
                        conflicting_client_ids_check: conflicting_client_ids_check,
                        device_certificate_expiring_check: device_certificate_expiring_check,
                        device_certificate_key_quality_check: device_certificate_key_quality_check,
                        device_certificate_shared_check: device_certificate_shared_check,
                        intermediate_ca_revoked_for_active_device_certificates_check: intermediate_ca_revoked_for_active_device_certificates_check,
                        io_t_policy_potential_mis_configuration_check: io_t_policy_potential_mis_configuration_check,
                        iot_policy_overly_permissive_check: iot_policy_overly_permissive_check,
                        iot_role_alias_allows_access_to_unused_services_check: iot_role_alias_allows_access_to_unused_services_check,
                        iot_role_alias_overly_permissive_check: iot_role_alias_overly_permissive_check,
                        logging_disabled_check: logging_disabled_check,
                        revoked_ca_certificate_still_active_check: revoked_ca_certificate_still_active_check,
                        revoked_device_certificate_still_active_check: revoked_device_certificate_still_active_check,
                        unauthenticated_cognito_role_overly_permissive_check: unauthenticated_cognito_role_overly_permissive_check,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::AccountAuditConfiguration.AuditNotificationTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-accountauditconfiguration-auditnotificationtarget.html) property type.
    #[derive(Debug, Default)]
    pub struct AuditNotificationTarget {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-accountauditconfiguration-auditnotificationtarget.html#cfn-iot-accountauditconfiguration-auditnotificationtarget-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: Option<::Value<bool>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-accountauditconfiguration-auditnotificationtarget.html#cfn-iot-accountauditconfiguration-auditnotificationtarget-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: Option<::Value<String>>,
        /// Property [`TargetArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-accountauditconfiguration-auditnotificationtarget.html#cfn-iot-accountauditconfiguration-auditnotificationtarget-targetarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AuditNotificationTarget {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enabled) = self.enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
            }
            if let Some(ref role_arn) = self.role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", role_arn)?;
            }
            if let Some(ref target_arn) = self.target_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetArn", target_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AuditNotificationTarget {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AuditNotificationTarget, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AuditNotificationTarget;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AuditNotificationTarget")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut target_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetArn" => {
                                target_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AuditNotificationTarget {
                        enabled: enabled,
                        role_arn: role_arn,
                        target_arn: target_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::AccountAuditConfiguration.AuditNotificationTargetConfigurations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-accountauditconfiguration-auditnotificationtargetconfigurations.html) property type.
    #[derive(Debug, Default)]
    pub struct AuditNotificationTargetConfigurations {
        /// Property [`Sns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-accountauditconfiguration-auditnotificationtargetconfigurations.html#cfn-iot-accountauditconfiguration-auditnotificationtargetconfigurations-sns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sns: Option<::Value<AuditNotificationTarget>>,
    }

    impl ::codec::SerializeValue for AuditNotificationTargetConfigurations {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref sns) = self.sns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Sns", sns)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AuditNotificationTargetConfigurations {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AuditNotificationTargetConfigurations, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AuditNotificationTargetConfigurations;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AuditNotificationTargetConfigurations")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut sns: Option<::Value<AuditNotificationTarget>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Sns" => {
                                sns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AuditNotificationTargetConfigurations {
                        sns: sns,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod billing_group {
    //! Property types for the `BillingGroup` resource.

    /// The [`AWS::IoT::BillingGroup.BillingGroupProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-billinggroup-billinggroupproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct BillingGroupProperties {
        /// Property [`BillingGroupDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-billinggroup-billinggroupproperties.html#cfn-iot-billinggroup-billinggroupproperties-billinggroupdescription).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub billing_group_description: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for BillingGroupProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref billing_group_description) = self.billing_group_description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BillingGroupDescription", billing_group_description)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BillingGroupProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BillingGroupProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BillingGroupProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BillingGroupProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut billing_group_description: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BillingGroupDescription" => {
                                billing_group_description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BillingGroupProperties {
                        billing_group_description: billing_group_description,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod ca_certificate {
    //! Property types for the `CACertificate` resource.

    /// The [`AWS::IoT::CACertificate.RegistrationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-cacertificate-registrationconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct RegistrationConfig {
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-cacertificate-registrationconfig.html#cfn-iot-cacertificate-registrationconfig-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: Option<::Value<String>>,
        /// Property [`TemplateBody`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-cacertificate-registrationconfig.html#cfn-iot-cacertificate-registrationconfig-templatebody).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub template_body: Option<::Value<String>>,
        /// Property [`TemplateName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-cacertificate-registrationconfig.html#cfn-iot-cacertificate-registrationconfig-templatename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub template_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for RegistrationConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref role_arn) = self.role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", role_arn)?;
            }
            if let Some(ref template_body) = self.template_body {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemplateBody", template_body)?;
            }
            if let Some(ref template_name) = self.template_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemplateName", template_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RegistrationConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RegistrationConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RegistrationConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RegistrationConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut template_body: Option<::Value<String>> = None;
                    let mut template_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TemplateBody" => {
                                template_body = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TemplateName" => {
                                template_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RegistrationConfig {
                        role_arn: role_arn,
                        template_body: template_body,
                        template_name: template_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod domain_configuration {
    //! Property types for the `DomainConfiguration` resource.

    /// The [`AWS::IoT::DomainConfiguration.AuthorizerConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-domainconfiguration-authorizerconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct AuthorizerConfig {
        /// Property [`AllowAuthorizerOverride`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-domainconfiguration-authorizerconfig.html#cfn-iot-domainconfiguration-authorizerconfig-allowauthorizeroverride).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allow_authorizer_override: Option<::Value<bool>>,
        /// Property [`DefaultAuthorizerName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-domainconfiguration-authorizerconfig.html#cfn-iot-domainconfiguration-authorizerconfig-defaultauthorizername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_authorizer_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AuthorizerConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref allow_authorizer_override) = self.allow_authorizer_override {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowAuthorizerOverride", allow_authorizer_override)?;
            }
            if let Some(ref default_authorizer_name) = self.default_authorizer_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultAuthorizerName", default_authorizer_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AuthorizerConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AuthorizerConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AuthorizerConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AuthorizerConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allow_authorizer_override: Option<::Value<bool>> = None;
                    let mut default_authorizer_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AllowAuthorizerOverride" => {
                                allow_authorizer_override = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DefaultAuthorizerName" => {
                                default_authorizer_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AuthorizerConfig {
                        allow_authorizer_override: allow_authorizer_override,
                        default_authorizer_name: default_authorizer_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::DomainConfiguration.ServerCertificateSummary`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-domainconfiguration-servercertificatesummary.html) property type.
    #[derive(Debug, Default)]
    pub struct ServerCertificateSummary {
        /// Property [`ServerCertificateArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-domainconfiguration-servercertificatesummary.html#cfn-iot-domainconfiguration-servercertificatesummary-servercertificatearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub server_certificate_arn: Option<::Value<String>>,
        /// Property [`ServerCertificateStatus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-domainconfiguration-servercertificatesummary.html#cfn-iot-domainconfiguration-servercertificatesummary-servercertificatestatus).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub server_certificate_status: Option<::Value<String>>,
        /// Property [`ServerCertificateStatusDetail`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-domainconfiguration-servercertificatesummary.html#cfn-iot-domainconfiguration-servercertificatesummary-servercertificatestatusdetail).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub server_certificate_status_detail: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ServerCertificateSummary {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref server_certificate_arn) = self.server_certificate_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerCertificateArn", server_certificate_arn)?;
            }
            if let Some(ref server_certificate_status) = self.server_certificate_status {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerCertificateStatus", server_certificate_status)?;
            }
            if let Some(ref server_certificate_status_detail) = self.server_certificate_status_detail {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerCertificateStatusDetail", server_certificate_status_detail)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ServerCertificateSummary {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ServerCertificateSummary, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ServerCertificateSummary;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ServerCertificateSummary")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut server_certificate_arn: Option<::Value<String>> = None;
                    let mut server_certificate_status: Option<::Value<String>> = None;
                    let mut server_certificate_status_detail: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ServerCertificateArn" => {
                                server_certificate_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServerCertificateStatus" => {
                                server_certificate_status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServerCertificateStatusDetail" => {
                                server_certificate_status_detail = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ServerCertificateSummary {
                        server_certificate_arn: server_certificate_arn,
                        server_certificate_status: server_certificate_status,
                        server_certificate_status_detail: server_certificate_status_detail,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::DomainConfiguration.TlsConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-domainconfiguration-tlsconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct TlsConfig {
        /// Property [`SecurityPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-domainconfiguration-tlsconfig.html#cfn-iot-domainconfiguration-tlsconfig-securitypolicy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_policy: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for TlsConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref security_policy) = self.security_policy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityPolicy", security_policy)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TlsConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TlsConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TlsConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TlsConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut security_policy: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SecurityPolicy" => {
                                security_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TlsConfig {
                        security_policy: security_policy,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod fleet_metric {
    //! Property types for the `FleetMetric` resource.

    /// The [`AWS::IoT::FleetMetric.AggregationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-fleetmetric-aggregationtype.html) property type.
    #[derive(Debug, Default)]
    pub struct AggregationType {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-fleetmetric-aggregationtype.html#cfn-iot-fleetmetric-aggregationtype-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Values`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-fleetmetric-aggregationtype.html#cfn-iot-fleetmetric-aggregationtype-values).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub values: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for AggregationType {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", &self.values)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AggregationType {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AggregationType, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AggregationType;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AggregationType")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut values: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Values" => {
                                values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AggregationType {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        values: values.ok_or(::serde::de::Error::missing_field("Values"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod job_template {
    //! Property types for the `JobTemplate` resource.

    /// The [`AWS::IoT::JobTemplate.AbortConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-jobtemplate-abortconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct AbortConfig {
        /// Property [`CriteriaList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-jobtemplate-abortconfig.html#cfn-iot-jobtemplate-abortconfig-criterialist).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub criteria_list: ::ValueList<AbortCriteria>,
    }

    impl ::codec::SerializeValue for AbortConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CriteriaList", &self.criteria_list)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AbortConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AbortConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AbortConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AbortConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut criteria_list: Option<::ValueList<AbortCriteria>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CriteriaList" => {
                                criteria_list = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AbortConfig {
                        criteria_list: criteria_list.ok_or(::serde::de::Error::missing_field("CriteriaList"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::JobTemplate.AbortCriteria`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-jobtemplate-abortcriteria.html) property type.
    #[derive(Debug, Default)]
    pub struct AbortCriteria {
        /// Property [`Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-jobtemplate-abortcriteria.html#cfn-iot-jobtemplate-abortcriteria-action).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub action: ::Value<String>,
        /// Property [`FailureType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-jobtemplate-abortcriteria.html#cfn-iot-jobtemplate-abortcriteria-failuretype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub failure_type: ::Value<String>,
        /// Property [`MinNumberOfExecutedThings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-jobtemplate-abortcriteria.html#cfn-iot-jobtemplate-abortcriteria-minnumberofexecutedthings).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub min_number_of_executed_things: ::Value<u32>,
        /// Property [`ThresholdPercentage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-jobtemplate-abortcriteria.html#cfn-iot-jobtemplate-abortcriteria-thresholdpercentage).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub threshold_percentage: ::Value<f64>,
    }

    impl ::codec::SerializeValue for AbortCriteria {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Action", &self.action)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FailureType", &self.failure_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinNumberOfExecutedThings", &self.min_number_of_executed_things)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThresholdPercentage", &self.threshold_percentage)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AbortCriteria {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AbortCriteria, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AbortCriteria;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AbortCriteria")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut action: Option<::Value<String>> = None;
                    let mut failure_type: Option<::Value<String>> = None;
                    let mut min_number_of_executed_things: Option<::Value<u32>> = None;
                    let mut threshold_percentage: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Action" => {
                                action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FailureType" => {
                                failure_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MinNumberOfExecutedThings" => {
                                min_number_of_executed_things = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ThresholdPercentage" => {
                                threshold_percentage = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AbortCriteria {
                        action: action.ok_or(::serde::de::Error::missing_field("Action"))?,
                        failure_type: failure_type.ok_or(::serde::de::Error::missing_field("FailureType"))?,
                        min_number_of_executed_things: min_number_of_executed_things.ok_or(::serde::de::Error::missing_field("MinNumberOfExecutedThings"))?,
                        threshold_percentage: threshold_percentage.ok_or(::serde::de::Error::missing_field("ThresholdPercentage"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::JobTemplate.ExponentialRolloutRate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-jobtemplate-exponentialrolloutrate.html) property type.
    #[derive(Debug, Default)]
    pub struct ExponentialRolloutRate {
        /// Property [`BaseRatePerMinute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-jobtemplate-exponentialrolloutrate.html#cfn-iot-jobtemplate-exponentialrolloutrate-baserateperminute).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub base_rate_per_minute: ::Value<u32>,
        /// Property [`IncrementFactor`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-jobtemplate-exponentialrolloutrate.html#cfn-iot-jobtemplate-exponentialrolloutrate-incrementfactor).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub increment_factor: ::Value<f64>,
        /// Property [`RateIncreaseCriteria`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-jobtemplate-exponentialrolloutrate.html#cfn-iot-jobtemplate-exponentialrolloutrate-rateincreasecriteria).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub rate_increase_criteria: ::Value<RateIncreaseCriteria>,
    }

    impl ::codec::SerializeValue for ExponentialRolloutRate {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BaseRatePerMinute", &self.base_rate_per_minute)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IncrementFactor", &self.increment_factor)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RateIncreaseCriteria", &self.rate_increase_criteria)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ExponentialRolloutRate {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ExponentialRolloutRate, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ExponentialRolloutRate;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ExponentialRolloutRate")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut base_rate_per_minute: Option<::Value<u32>> = None;
                    let mut increment_factor: Option<::Value<f64>> = None;
                    let mut rate_increase_criteria: Option<::Value<RateIncreaseCriteria>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BaseRatePerMinute" => {
                                base_rate_per_minute = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IncrementFactor" => {
                                increment_factor = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RateIncreaseCriteria" => {
                                rate_increase_criteria = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ExponentialRolloutRate {
                        base_rate_per_minute: base_rate_per_minute.ok_or(::serde::de::Error::missing_field("BaseRatePerMinute"))?,
                        increment_factor: increment_factor.ok_or(::serde::de::Error::missing_field("IncrementFactor"))?,
                        rate_increase_criteria: rate_increase_criteria.ok_or(::serde::de::Error::missing_field("RateIncreaseCriteria"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::JobTemplate.JobExecutionsRetryConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-jobtemplate-jobexecutionsretryconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct JobExecutionsRetryConfig {
        /// Property [`RetryCriteriaList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-jobtemplate-jobexecutionsretryconfig.html#cfn-iot-jobtemplate-jobexecutionsretryconfig-retrycriterialist).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub retry_criteria_list: Option<::ValueList<RetryCriteria>>,
    }

    impl ::codec::SerializeValue for JobExecutionsRetryConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref retry_criteria_list) = self.retry_criteria_list {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RetryCriteriaList", retry_criteria_list)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for JobExecutionsRetryConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<JobExecutionsRetryConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = JobExecutionsRetryConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type JobExecutionsRetryConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut retry_criteria_list: Option<::ValueList<RetryCriteria>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RetryCriteriaList" => {
                                retry_criteria_list = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(JobExecutionsRetryConfig {
                        retry_criteria_list: retry_criteria_list,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::JobTemplate.JobExecutionsRolloutConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-jobtemplate-jobexecutionsrolloutconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct JobExecutionsRolloutConfig {
        /// Property [`ExponentialRolloutRate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-jobtemplate-jobexecutionsrolloutconfig.html#cfn-iot-jobtemplate-jobexecutionsrolloutconfig-exponentialrolloutrate).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub exponential_rollout_rate: Option<::Value<ExponentialRolloutRate>>,
        /// Property [`MaximumPerMinute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-jobtemplate-jobexecutionsrolloutconfig.html#cfn-iot-jobtemplate-jobexecutionsrolloutconfig-maximumperminute).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub maximum_per_minute: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for JobExecutionsRolloutConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref exponential_rollout_rate) = self.exponential_rollout_rate {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExponentialRolloutRate", exponential_rollout_rate)?;
            }
            if let Some(ref maximum_per_minute) = self.maximum_per_minute {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumPerMinute", maximum_per_minute)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for JobExecutionsRolloutConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<JobExecutionsRolloutConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = JobExecutionsRolloutConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type JobExecutionsRolloutConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut exponential_rollout_rate: Option<::Value<ExponentialRolloutRate>> = None;
                    let mut maximum_per_minute: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ExponentialRolloutRate" => {
                                exponential_rollout_rate = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaximumPerMinute" => {
                                maximum_per_minute = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(JobExecutionsRolloutConfig {
                        exponential_rollout_rate: exponential_rollout_rate,
                        maximum_per_minute: maximum_per_minute,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::JobTemplate.MaintenanceWindow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-jobtemplate-maintenancewindow.html) property type.
    #[derive(Debug, Default)]
    pub struct MaintenanceWindow {
        /// Property [`DurationInMinutes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-jobtemplate-maintenancewindow.html#cfn-iot-jobtemplate-maintenancewindow-durationinminutes).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub duration_in_minutes: Option<::Value<u32>>,
        /// Property [`StartTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-jobtemplate-maintenancewindow.html#cfn-iot-jobtemplate-maintenancewindow-starttime).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub start_time: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for MaintenanceWindow {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref duration_in_minutes) = self.duration_in_minutes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DurationInMinutes", duration_in_minutes)?;
            }
            if let Some(ref start_time) = self.start_time {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartTime", start_time)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MaintenanceWindow {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MaintenanceWindow, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MaintenanceWindow;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MaintenanceWindow")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut duration_in_minutes: Option<::Value<u32>> = None;
                    let mut start_time: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DurationInMinutes" => {
                                duration_in_minutes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StartTime" => {
                                start_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MaintenanceWindow {
                        duration_in_minutes: duration_in_minutes,
                        start_time: start_time,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::JobTemplate.PresignedUrlConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-jobtemplate-presignedurlconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct PresignedUrlConfig {
        /// Property [`ExpiresInSec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-jobtemplate-presignedurlconfig.html#cfn-iot-jobtemplate-presignedurlconfig-expiresinsec).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub expires_in_sec: Option<::Value<u32>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-jobtemplate-presignedurlconfig.html#cfn-iot-jobtemplate-presignedurlconfig-rolearn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub role_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for PresignedUrlConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref expires_in_sec) = self.expires_in_sec {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExpiresInSec", expires_in_sec)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PresignedUrlConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PresignedUrlConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PresignedUrlConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PresignedUrlConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut expires_in_sec: Option<::Value<u32>> = None;
                    let mut role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ExpiresInSec" => {
                                expires_in_sec = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PresignedUrlConfig {
                        expires_in_sec: expires_in_sec,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::JobTemplate.RateIncreaseCriteria`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-jobtemplate-rateincreasecriteria.html) property type.
    #[derive(Debug, Default)]
    pub struct RateIncreaseCriteria {
        /// Property [`NumberOfNotifiedThings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-jobtemplate-rateincreasecriteria.html#cfn-iot-jobtemplate-rateincreasecriteria-numberofnotifiedthings).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub number_of_notified_things: Option<::Value<u32>>,
        /// Property [`NumberOfSucceededThings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-jobtemplate-rateincreasecriteria.html#cfn-iot-jobtemplate-rateincreasecriteria-numberofsucceededthings).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub number_of_succeeded_things: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for RateIncreaseCriteria {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref number_of_notified_things) = self.number_of_notified_things {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumberOfNotifiedThings", number_of_notified_things)?;
            }
            if let Some(ref number_of_succeeded_things) = self.number_of_succeeded_things {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumberOfSucceededThings", number_of_succeeded_things)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RateIncreaseCriteria {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RateIncreaseCriteria, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RateIncreaseCriteria;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RateIncreaseCriteria")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut number_of_notified_things: Option<::Value<u32>> = None;
                    let mut number_of_succeeded_things: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "NumberOfNotifiedThings" => {
                                number_of_notified_things = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NumberOfSucceededThings" => {
                                number_of_succeeded_things = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RateIncreaseCriteria {
                        number_of_notified_things: number_of_notified_things,
                        number_of_succeeded_things: number_of_succeeded_things,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::JobTemplate.RetryCriteria`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-jobtemplate-retrycriteria.html) property type.
    #[derive(Debug, Default)]
    pub struct RetryCriteria {
        /// Property [`FailureType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-jobtemplate-retrycriteria.html#cfn-iot-jobtemplate-retrycriteria-failuretype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub failure_type: Option<::Value<String>>,
        /// Property [`NumberOfRetries`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-jobtemplate-retrycriteria.html#cfn-iot-jobtemplate-retrycriteria-numberofretries).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub number_of_retries: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for RetryCriteria {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref failure_type) = self.failure_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FailureType", failure_type)?;
            }
            if let Some(ref number_of_retries) = self.number_of_retries {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumberOfRetries", number_of_retries)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RetryCriteria {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RetryCriteria, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RetryCriteria;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RetryCriteria")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut failure_type: Option<::Value<String>> = None;
                    let mut number_of_retries: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FailureType" => {
                                failure_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NumberOfRetries" => {
                                number_of_retries = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RetryCriteria {
                        failure_type: failure_type,
                        number_of_retries: number_of_retries,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::JobTemplate.TimeoutConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-jobtemplate-timeoutconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct TimeoutConfig {
        /// Property [`InProgressTimeoutInMinutes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-jobtemplate-timeoutconfig.html#cfn-iot-jobtemplate-timeoutconfig-inprogresstimeoutinminutes).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub in_progress_timeout_in_minutes: ::Value<u32>,
    }

    impl ::codec::SerializeValue for TimeoutConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InProgressTimeoutInMinutes", &self.in_progress_timeout_in_minutes)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TimeoutConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TimeoutConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TimeoutConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TimeoutConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut in_progress_timeout_in_minutes: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InProgressTimeoutInMinutes" => {
                                in_progress_timeout_in_minutes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TimeoutConfig {
                        in_progress_timeout_in_minutes: in_progress_timeout_in_minutes.ok_or(::serde::de::Error::missing_field("InProgressTimeoutInMinutes"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod mitigation_action {
    //! Property types for the `MitigationAction` resource.

    /// The [`AWS::IoT::MitigationAction.ActionParams`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-mitigationaction-actionparams.html) property type.
    #[derive(Debug, Default)]
    pub struct ActionParams {
        /// Property [`AddThingsToThingGroupParams`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-mitigationaction-actionparams.html#cfn-iot-mitigationaction-actionparams-addthingstothinggroupparams).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub add_things_to_thing_group_params: Option<::Value<AddThingsToThingGroupParams>>,
        /// Property [`EnableIoTLoggingParams`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-mitigationaction-actionparams.html#cfn-iot-mitigationaction-actionparams-enableiotloggingparams).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable_io_t_logging_params: Option<::Value<EnableIoTLoggingParams>>,
        /// Property [`PublishFindingToSnsParams`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-mitigationaction-actionparams.html#cfn-iot-mitigationaction-actionparams-publishfindingtosnsparams).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub publish_finding_to_sns_params: Option<::Value<PublishFindingToSnsParams>>,
        /// Property [`ReplaceDefaultPolicyVersionParams`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-mitigationaction-actionparams.html#cfn-iot-mitigationaction-actionparams-replacedefaultpolicyversionparams).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub replace_default_policy_version_params: Option<::Value<ReplaceDefaultPolicyVersionParams>>,
        /// Property [`UpdateCACertificateParams`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-mitigationaction-actionparams.html#cfn-iot-mitigationaction-actionparams-updatecacertificateparams).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub update_ca_certificate_params: Option<::Value<UpdateCACertificateParams>>,
        /// Property [`UpdateDeviceCertificateParams`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-mitigationaction-actionparams.html#cfn-iot-mitigationaction-actionparams-updatedevicecertificateparams).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub update_device_certificate_params: Option<::Value<UpdateDeviceCertificateParams>>,
    }

    impl ::codec::SerializeValue for ActionParams {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref add_things_to_thing_group_params) = self.add_things_to_thing_group_params {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AddThingsToThingGroupParams", add_things_to_thing_group_params)?;
            }
            if let Some(ref enable_io_t_logging_params) = self.enable_io_t_logging_params {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableIoTLoggingParams", enable_io_t_logging_params)?;
            }
            if let Some(ref publish_finding_to_sns_params) = self.publish_finding_to_sns_params {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PublishFindingToSnsParams", publish_finding_to_sns_params)?;
            }
            if let Some(ref replace_default_policy_version_params) = self.replace_default_policy_version_params {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplaceDefaultPolicyVersionParams", replace_default_policy_version_params)?;
            }
            if let Some(ref update_ca_certificate_params) = self.update_ca_certificate_params {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UpdateCACertificateParams", update_ca_certificate_params)?;
            }
            if let Some(ref update_device_certificate_params) = self.update_device_certificate_params {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UpdateDeviceCertificateParams", update_device_certificate_params)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ActionParams {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ActionParams, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ActionParams;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ActionParams")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut add_things_to_thing_group_params: Option<::Value<AddThingsToThingGroupParams>> = None;
                    let mut enable_io_t_logging_params: Option<::Value<EnableIoTLoggingParams>> = None;
                    let mut publish_finding_to_sns_params: Option<::Value<PublishFindingToSnsParams>> = None;
                    let mut replace_default_policy_version_params: Option<::Value<ReplaceDefaultPolicyVersionParams>> = None;
                    let mut update_ca_certificate_params: Option<::Value<UpdateCACertificateParams>> = None;
                    let mut update_device_certificate_params: Option<::Value<UpdateDeviceCertificateParams>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AddThingsToThingGroupParams" => {
                                add_things_to_thing_group_params = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnableIoTLoggingParams" => {
                                enable_io_t_logging_params = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PublishFindingToSnsParams" => {
                                publish_finding_to_sns_params = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ReplaceDefaultPolicyVersionParams" => {
                                replace_default_policy_version_params = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UpdateCACertificateParams" => {
                                update_ca_certificate_params = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UpdateDeviceCertificateParams" => {
                                update_device_certificate_params = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ActionParams {
                        add_things_to_thing_group_params: add_things_to_thing_group_params,
                        enable_io_t_logging_params: enable_io_t_logging_params,
                        publish_finding_to_sns_params: publish_finding_to_sns_params,
                        replace_default_policy_version_params: replace_default_policy_version_params,
                        update_ca_certificate_params: update_ca_certificate_params,
                        update_device_certificate_params: update_device_certificate_params,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::MitigationAction.AddThingsToThingGroupParams`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-mitigationaction-addthingstothinggroupparams.html) property type.
    #[derive(Debug, Default)]
    pub struct AddThingsToThingGroupParams {
        /// Property [`OverrideDynamicGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-mitigationaction-addthingstothinggroupparams.html#cfn-iot-mitigationaction-addthingstothinggroupparams-overridedynamicgroups).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub override_dynamic_groups: Option<::Value<bool>>,
        /// Property [`ThingGroupNames`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-mitigationaction-addthingstothinggroupparams.html#cfn-iot-mitigationaction-addthingstothinggroupparams-thinggroupnames).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub thing_group_names: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for AddThingsToThingGroupParams {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref override_dynamic_groups) = self.override_dynamic_groups {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OverrideDynamicGroups", override_dynamic_groups)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThingGroupNames", &self.thing_group_names)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AddThingsToThingGroupParams {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AddThingsToThingGroupParams, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AddThingsToThingGroupParams;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AddThingsToThingGroupParams")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut override_dynamic_groups: Option<::Value<bool>> = None;
                    let mut thing_group_names: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "OverrideDynamicGroups" => {
                                override_dynamic_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ThingGroupNames" => {
                                thing_group_names = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AddThingsToThingGroupParams {
                        override_dynamic_groups: override_dynamic_groups,
                        thing_group_names: thing_group_names.ok_or(::serde::de::Error::missing_field("ThingGroupNames"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::MitigationAction.EnableIoTLoggingParams`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-mitigationaction-enableiotloggingparams.html) property type.
    #[derive(Debug, Default)]
    pub struct EnableIoTLoggingParams {
        /// Property [`LogLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-mitigationaction-enableiotloggingparams.html#cfn-iot-mitigationaction-enableiotloggingparams-loglevel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_level: ::Value<String>,
        /// Property [`RoleArnForLogging`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-mitigationaction-enableiotloggingparams.html#cfn-iot-mitigationaction-enableiotloggingparams-rolearnforlogging).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn_for_logging: ::Value<String>,
    }

    impl ::codec::SerializeValue for EnableIoTLoggingParams {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogLevel", &self.log_level)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArnForLogging", &self.role_arn_for_logging)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EnableIoTLoggingParams {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EnableIoTLoggingParams, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EnableIoTLoggingParams;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EnableIoTLoggingParams")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut log_level: Option<::Value<String>> = None;
                    let mut role_arn_for_logging: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LogLevel" => {
                                log_level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArnForLogging" => {
                                role_arn_for_logging = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EnableIoTLoggingParams {
                        log_level: log_level.ok_or(::serde::de::Error::missing_field("LogLevel"))?,
                        role_arn_for_logging: role_arn_for_logging.ok_or(::serde::de::Error::missing_field("RoleArnForLogging"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::MitigationAction.PublishFindingToSnsParams`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-mitigationaction-publishfindingtosnsparams.html) property type.
    #[derive(Debug, Default)]
    pub struct PublishFindingToSnsParams {
        /// Property [`TopicArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-mitigationaction-publishfindingtosnsparams.html#cfn-iot-mitigationaction-publishfindingtosnsparams-topicarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub topic_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for PublishFindingToSnsParams {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TopicArn", &self.topic_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PublishFindingToSnsParams {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PublishFindingToSnsParams, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PublishFindingToSnsParams;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PublishFindingToSnsParams")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut topic_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TopicArn" => {
                                topic_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PublishFindingToSnsParams {
                        topic_arn: topic_arn.ok_or(::serde::de::Error::missing_field("TopicArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::MitigationAction.ReplaceDefaultPolicyVersionParams`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-mitigationaction-replacedefaultpolicyversionparams.html) property type.
    #[derive(Debug, Default)]
    pub struct ReplaceDefaultPolicyVersionParams {
        /// Property [`TemplateName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-mitigationaction-replacedefaultpolicyversionparams.html#cfn-iot-mitigationaction-replacedefaultpolicyversionparams-templatename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub template_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for ReplaceDefaultPolicyVersionParams {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TemplateName", &self.template_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ReplaceDefaultPolicyVersionParams {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ReplaceDefaultPolicyVersionParams, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ReplaceDefaultPolicyVersionParams;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ReplaceDefaultPolicyVersionParams")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut template_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TemplateName" => {
                                template_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ReplaceDefaultPolicyVersionParams {
                        template_name: template_name.ok_or(::serde::de::Error::missing_field("TemplateName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::MitigationAction.UpdateCACertificateParams`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-mitigationaction-updatecacertificateparams.html) property type.
    #[derive(Debug, Default)]
    pub struct UpdateCACertificateParams {
        /// Property [`Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-mitigationaction-updatecacertificateparams.html#cfn-iot-mitigationaction-updatecacertificateparams-action).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub action: ::Value<String>,
    }

    impl ::codec::SerializeValue for UpdateCACertificateParams {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Action", &self.action)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for UpdateCACertificateParams {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<UpdateCACertificateParams, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = UpdateCACertificateParams;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type UpdateCACertificateParams")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut action: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Action" => {
                                action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(UpdateCACertificateParams {
                        action: action.ok_or(::serde::de::Error::missing_field("Action"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::MitigationAction.UpdateDeviceCertificateParams`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-mitigationaction-updatedevicecertificateparams.html) property type.
    #[derive(Debug, Default)]
    pub struct UpdateDeviceCertificateParams {
        /// Property [`Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-mitigationaction-updatedevicecertificateparams.html#cfn-iot-mitigationaction-updatedevicecertificateparams-action).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub action: ::Value<String>,
    }

    impl ::codec::SerializeValue for UpdateDeviceCertificateParams {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Action", &self.action)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for UpdateDeviceCertificateParams {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<UpdateDeviceCertificateParams, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = UpdateDeviceCertificateParams;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type UpdateDeviceCertificateParams")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut action: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Action" => {
                                action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(UpdateDeviceCertificateParams {
                        action: action.ok_or(::serde::de::Error::missing_field("Action"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod provisioning_template {
    //! Property types for the `ProvisioningTemplate` resource.

    /// The [`AWS::IoT::ProvisioningTemplate.ProvisioningHook`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-provisioningtemplate-provisioninghook.html) property type.
    #[derive(Debug, Default)]
    pub struct ProvisioningHook {
        /// Property [`PayloadVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-provisioningtemplate-provisioninghook.html#cfn-iot-provisioningtemplate-provisioninghook-payloadversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub payload_version: Option<::Value<String>>,
        /// Property [`TargetArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-provisioningtemplate-provisioninghook.html#cfn-iot-provisioningtemplate-provisioninghook-targetarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ProvisioningHook {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref payload_version) = self.payload_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PayloadVersion", payload_version)?;
            }
            if let Some(ref target_arn) = self.target_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetArn", target_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ProvisioningHook {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ProvisioningHook, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ProvisioningHook;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ProvisioningHook")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut payload_version: Option<::Value<String>> = None;
                    let mut target_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PayloadVersion" => {
                                payload_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetArn" => {
                                target_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ProvisioningHook {
                        payload_version: payload_version,
                        target_arn: target_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod security_profile {
    //! Property types for the `SecurityProfile` resource.

    /// The [`AWS::IoT::SecurityProfile.AlertTarget`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-alerttarget.html) property type.
    #[derive(Debug, Default)]
    pub struct AlertTarget {
        /// Property [`AlertTargetArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-alerttarget.html#cfn-iot-securityprofile-alerttarget-alerttargetarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub alert_target_arn: ::Value<String>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-alerttarget.html#cfn-iot-securityprofile-alerttarget-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for AlertTarget {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AlertTargetArn", &self.alert_target_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AlertTarget {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AlertTarget, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AlertTarget;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AlertTarget")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut alert_target_arn: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AlertTargetArn" => {
                                alert_target_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AlertTarget {
                        alert_target_arn: alert_target_arn.ok_or(::serde::de::Error::missing_field("AlertTargetArn"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::SecurityProfile.Behavior`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-behavior.html) property type.
    #[derive(Debug, Default)]
    pub struct Behavior {
        /// Property [`Criteria`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-behavior.html#cfn-iot-securityprofile-behavior-criteria).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub criteria: Option<::Value<BehaviorCriteria>>,
        /// Property [`ExportMetric`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-behavior.html#cfn-iot-securityprofile-behavior-exportmetric).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub export_metric: Option<::Value<bool>>,
        /// Property [`Metric`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-behavior.html#cfn-iot-securityprofile-behavior-metric).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric: Option<::Value<String>>,
        /// Property [`MetricDimension`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-behavior.html#cfn-iot-securityprofile-behavior-metricdimension).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric_dimension: Option<::Value<MetricDimension>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-behavior.html#cfn-iot-securityprofile-behavior-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`SuppressAlerts`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-behavior.html#cfn-iot-securityprofile-behavior-suppressalerts).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub suppress_alerts: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for Behavior {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref criteria) = self.criteria {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Criteria", criteria)?;
            }
            if let Some(ref export_metric) = self.export_metric {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExportMetric", export_metric)?;
            }
            if let Some(ref metric) = self.metric {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Metric", metric)?;
            }
            if let Some(ref metric_dimension) = self.metric_dimension {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricDimension", metric_dimension)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            if let Some(ref suppress_alerts) = self.suppress_alerts {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SuppressAlerts", suppress_alerts)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Behavior {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Behavior, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Behavior;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Behavior")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut criteria: Option<::Value<BehaviorCriteria>> = None;
                    let mut export_metric: Option<::Value<bool>> = None;
                    let mut metric: Option<::Value<String>> = None;
                    let mut metric_dimension: Option<::Value<MetricDimension>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut suppress_alerts: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Criteria" => {
                                criteria = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ExportMetric" => {
                                export_metric = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Metric" => {
                                metric = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetricDimension" => {
                                metric_dimension = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SuppressAlerts" => {
                                suppress_alerts = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Behavior {
                        criteria: criteria,
                        export_metric: export_metric,
                        metric: metric,
                        metric_dimension: metric_dimension,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        suppress_alerts: suppress_alerts,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::SecurityProfile.BehaviorCriteria`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-behaviorcriteria.html) property type.
    #[derive(Debug, Default)]
    pub struct BehaviorCriteria {
        /// Property [`ComparisonOperator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-behaviorcriteria.html#cfn-iot-securityprofile-behaviorcriteria-comparisonoperator).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub comparison_operator: Option<::Value<String>>,
        /// Property [`ConsecutiveDatapointsToAlarm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-behaviorcriteria.html#cfn-iot-securityprofile-behaviorcriteria-consecutivedatapointstoalarm).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub consecutive_datapoints_to_alarm: Option<::Value<u32>>,
        /// Property [`ConsecutiveDatapointsToClear`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-behaviorcriteria.html#cfn-iot-securityprofile-behaviorcriteria-consecutivedatapointstoclear).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub consecutive_datapoints_to_clear: Option<::Value<u32>>,
        /// Property [`DurationSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-behaviorcriteria.html#cfn-iot-securityprofile-behaviorcriteria-durationseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub duration_seconds: Option<::Value<u32>>,
        /// Property [`MlDetectionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-behaviorcriteria.html#cfn-iot-securityprofile-behaviorcriteria-mldetectionconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ml_detection_config: Option<::Value<MachineLearningDetectionConfig>>,
        /// Property [`StatisticalThreshold`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-behaviorcriteria.html#cfn-iot-securityprofile-behaviorcriteria-statisticalthreshold).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub statistical_threshold: Option<::Value<StatisticalThreshold>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-behaviorcriteria.html#cfn-iot-securityprofile-behaviorcriteria-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: Option<::Value<MetricValue>>,
    }

    impl ::codec::SerializeValue for BehaviorCriteria {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref comparison_operator) = self.comparison_operator {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComparisonOperator", comparison_operator)?;
            }
            if let Some(ref consecutive_datapoints_to_alarm) = self.consecutive_datapoints_to_alarm {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConsecutiveDatapointsToAlarm", consecutive_datapoints_to_alarm)?;
            }
            if let Some(ref consecutive_datapoints_to_clear) = self.consecutive_datapoints_to_clear {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConsecutiveDatapointsToClear", consecutive_datapoints_to_clear)?;
            }
            if let Some(ref duration_seconds) = self.duration_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DurationSeconds", duration_seconds)?;
            }
            if let Some(ref ml_detection_config) = self.ml_detection_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MlDetectionConfig", ml_detection_config)?;
            }
            if let Some(ref statistical_threshold) = self.statistical_threshold {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StatisticalThreshold", statistical_threshold)?;
            }
            if let Some(ref value) = self.value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BehaviorCriteria {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BehaviorCriteria, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BehaviorCriteria;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BehaviorCriteria")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut comparison_operator: Option<::Value<String>> = None;
                    let mut consecutive_datapoints_to_alarm: Option<::Value<u32>> = None;
                    let mut consecutive_datapoints_to_clear: Option<::Value<u32>> = None;
                    let mut duration_seconds: Option<::Value<u32>> = None;
                    let mut ml_detection_config: Option<::Value<MachineLearningDetectionConfig>> = None;
                    let mut statistical_threshold: Option<::Value<StatisticalThreshold>> = None;
                    let mut value: Option<::Value<MetricValue>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ComparisonOperator" => {
                                comparison_operator = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConsecutiveDatapointsToAlarm" => {
                                consecutive_datapoints_to_alarm = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConsecutiveDatapointsToClear" => {
                                consecutive_datapoints_to_clear = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DurationSeconds" => {
                                duration_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MlDetectionConfig" => {
                                ml_detection_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StatisticalThreshold" => {
                                statistical_threshold = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BehaviorCriteria {
                        comparison_operator: comparison_operator,
                        consecutive_datapoints_to_alarm: consecutive_datapoints_to_alarm,
                        consecutive_datapoints_to_clear: consecutive_datapoints_to_clear,
                        duration_seconds: duration_seconds,
                        ml_detection_config: ml_detection_config,
                        statistical_threshold: statistical_threshold,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::SecurityProfile.MachineLearningDetectionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-machinelearningdetectionconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct MachineLearningDetectionConfig {
        /// Property [`ConfidenceLevel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-machinelearningdetectionconfig.html#cfn-iot-securityprofile-machinelearningdetectionconfig-confidencelevel).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub confidence_level: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for MachineLearningDetectionConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref confidence_level) = self.confidence_level {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfidenceLevel", confidence_level)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MachineLearningDetectionConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MachineLearningDetectionConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MachineLearningDetectionConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MachineLearningDetectionConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut confidence_level: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConfidenceLevel" => {
                                confidence_level = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MachineLearningDetectionConfig {
                        confidence_level: confidence_level,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::SecurityProfile.MetricDimension`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-metricdimension.html) property type.
    #[derive(Debug, Default)]
    pub struct MetricDimension {
        /// Property [`DimensionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-metricdimension.html#cfn-iot-securityprofile-metricdimension-dimensionname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dimension_name: ::Value<String>,
        /// Property [`Operator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-metricdimension.html#cfn-iot-securityprofile-metricdimension-operator).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub operator: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for MetricDimension {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DimensionName", &self.dimension_name)?;
            if let Some(ref operator) = self.operator {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Operator", operator)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MetricDimension {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MetricDimension, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MetricDimension;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MetricDimension")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut dimension_name: Option<::Value<String>> = None;
                    let mut operator: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DimensionName" => {
                                dimension_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Operator" => {
                                operator = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MetricDimension {
                        dimension_name: dimension_name.ok_or(::serde::de::Error::missing_field("DimensionName"))?,
                        operator: operator,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::SecurityProfile.MetricToRetain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-metrictoretain.html) property type.
    #[derive(Debug, Default)]
    pub struct MetricToRetain {
        /// Property [`ExportMetric`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-metrictoretain.html#cfn-iot-securityprofile-metrictoretain-exportmetric).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub export_metric: Option<::Value<bool>>,
        /// Property [`Metric`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-metrictoretain.html#cfn-iot-securityprofile-metrictoretain-metric).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric: ::Value<String>,
        /// Property [`MetricDimension`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-metrictoretain.html#cfn-iot-securityprofile-metrictoretain-metricdimension).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric_dimension: Option<::Value<MetricDimension>>,
    }

    impl ::codec::SerializeValue for MetricToRetain {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref export_metric) = self.export_metric {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExportMetric", export_metric)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Metric", &self.metric)?;
            if let Some(ref metric_dimension) = self.metric_dimension {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricDimension", metric_dimension)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MetricToRetain {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MetricToRetain, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MetricToRetain;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MetricToRetain")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut export_metric: Option<::Value<bool>> = None;
                    let mut metric: Option<::Value<String>> = None;
                    let mut metric_dimension: Option<::Value<MetricDimension>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ExportMetric" => {
                                export_metric = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Metric" => {
                                metric = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetricDimension" => {
                                metric_dimension = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MetricToRetain {
                        export_metric: export_metric,
                        metric: metric.ok_or(::serde::de::Error::missing_field("Metric"))?,
                        metric_dimension: metric_dimension,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::SecurityProfile.MetricValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-metricvalue.html) property type.
    #[derive(Debug, Default)]
    pub struct MetricValue {
        /// Property [`Cidrs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-metricvalue.html#cfn-iot-securityprofile-metricvalue-cidrs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cidrs: Option<::ValueList<String>>,
        /// Property [`Count`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-metricvalue.html#cfn-iot-securityprofile-metricvalue-count).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub count: Option<::Value<String>>,
        /// Property [`Number`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-metricvalue.html#cfn-iot-securityprofile-metricvalue-number).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub number: Option<::Value<f64>>,
        /// Property [`Numbers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-metricvalue.html#cfn-iot-securityprofile-metricvalue-numbers).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub numbers: Option<::ValueList<f64>>,
        /// Property [`Ports`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-metricvalue.html#cfn-iot-securityprofile-metricvalue-ports).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ports: Option<::ValueList<u32>>,
        /// Property [`Strings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-metricvalue.html#cfn-iot-securityprofile-metricvalue-strings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub strings: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for MetricValue {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cidrs) = self.cidrs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cidrs", cidrs)?;
            }
            if let Some(ref count) = self.count {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Count", count)?;
            }
            if let Some(ref number) = self.number {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Number", number)?;
            }
            if let Some(ref numbers) = self.numbers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Numbers", numbers)?;
            }
            if let Some(ref ports) = self.ports {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Ports", ports)?;
            }
            if let Some(ref strings) = self.strings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Strings", strings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MetricValue {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MetricValue, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MetricValue;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MetricValue")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cidrs: Option<::ValueList<String>> = None;
                    let mut count: Option<::Value<String>> = None;
                    let mut number: Option<::Value<f64>> = None;
                    let mut numbers: Option<::ValueList<f64>> = None;
                    let mut ports: Option<::ValueList<u32>> = None;
                    let mut strings: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Cidrs" => {
                                cidrs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Count" => {
                                count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Number" => {
                                number = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Numbers" => {
                                numbers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Ports" => {
                                ports = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Strings" => {
                                strings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MetricValue {
                        cidrs: cidrs,
                        count: count,
                        number: number,
                        numbers: numbers,
                        ports: ports,
                        strings: strings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::SecurityProfile.MetricsExportConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-metricsexportconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct MetricsExportConfig {
        /// Property [`MqttTopic`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-metricsexportconfig.html#cfn-iot-securityprofile-metricsexportconfig-mqtttopic).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mqtt_topic: ::Value<String>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-metricsexportconfig.html#cfn-iot-securityprofile-metricsexportconfig-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for MetricsExportConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MqttTopic", &self.mqtt_topic)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MetricsExportConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MetricsExportConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MetricsExportConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MetricsExportConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut mqtt_topic: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MqttTopic" => {
                                mqtt_topic = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MetricsExportConfig {
                        mqtt_topic: mqtt_topic.ok_or(::serde::de::Error::missing_field("MqttTopic"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::SecurityProfile.StatisticalThreshold`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-statisticalthreshold.html) property type.
    #[derive(Debug, Default)]
    pub struct StatisticalThreshold {
        /// Property [`Statistic`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-securityprofile-statisticalthreshold.html#cfn-iot-securityprofile-statisticalthreshold-statistic).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub statistic: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for StatisticalThreshold {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref statistic) = self.statistic {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Statistic", statistic)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StatisticalThreshold {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StatisticalThreshold, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StatisticalThreshold;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StatisticalThreshold")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut statistic: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Statistic" => {
                                statistic = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StatisticalThreshold {
                        statistic: statistic,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod thing {
    //! Property types for the `Thing` resource.

    /// The [`AWS::IoT::Thing.AttributePayload`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-thing-attributepayload.html) property type.
    #[derive(Debug, Default)]
    pub struct AttributePayload {
        /// Property [`Attributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-thing-attributepayload.html#cfn-iot-thing-attributepayload-attributes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attributes: Option<::ValueMap<String>>,
    }

    impl ::codec::SerializeValue for AttributePayload {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref attributes) = self.attributes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Attributes", attributes)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AttributePayload {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AttributePayload, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AttributePayload;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AttributePayload")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attributes: Option<::ValueMap<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Attributes" => {
                                attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AttributePayload {
                        attributes: attributes,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod thing_group {
    //! Property types for the `ThingGroup` resource.

    /// The [`AWS::IoT::ThingGroup.AttributePayload`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-thinggroup-attributepayload.html) property type.
    #[derive(Debug, Default)]
    pub struct AttributePayload {
        /// Property [`Attributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-thinggroup-attributepayload.html#cfn-iot-thinggroup-attributepayload-attributes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attributes: Option<::ValueMap<String>>,
    }

    impl ::codec::SerializeValue for AttributePayload {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref attributes) = self.attributes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Attributes", attributes)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AttributePayload {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AttributePayload, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AttributePayload;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AttributePayload")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attributes: Option<::ValueMap<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Attributes" => {
                                attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AttributePayload {
                        attributes: attributes,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::ThingGroup.ThingGroupProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-thinggroup-thinggroupproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct ThingGroupProperties {
        /// Property [`AttributePayload`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-thinggroup-thinggroupproperties.html#cfn-iot-thinggroup-thinggroupproperties-attributepayload).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub attribute_payload: Option<::Value<AttributePayload>>,
        /// Property [`ThingGroupDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-thinggroup-thinggroupproperties.html#cfn-iot-thinggroup-thinggroupproperties-thinggroupdescription).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub thing_group_description: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ThingGroupProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref attribute_payload) = self.attribute_payload {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AttributePayload", attribute_payload)?;
            }
            if let Some(ref thing_group_description) = self.thing_group_description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThingGroupDescription", thing_group_description)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ThingGroupProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ThingGroupProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ThingGroupProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ThingGroupProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut attribute_payload: Option<::Value<AttributePayload>> = None;
                    let mut thing_group_description: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AttributePayload" => {
                                attribute_payload = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ThingGroupDescription" => {
                                thing_group_description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ThingGroupProperties {
                        attribute_payload: attribute_payload,
                        thing_group_description: thing_group_description,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod thing_type {
    //! Property types for the `ThingType` resource.

    /// The [`AWS::IoT::ThingType.ThingTypeProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-thingtype-thingtypeproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct ThingTypeProperties {
        /// Property [`SearchableAttributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-thingtype-thingtypeproperties.html#cfn-iot-thingtype-thingtypeproperties-searchableattributes).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub searchable_attributes: Option<::ValueList<String>>,
        /// Property [`ThingTypeDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-thingtype-thingtypeproperties.html#cfn-iot-thingtype-thingtypeproperties-thingtypedescription).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub thing_type_description: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ThingTypeProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref searchable_attributes) = self.searchable_attributes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SearchableAttributes", searchable_attributes)?;
            }
            if let Some(ref thing_type_description) = self.thing_type_description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ThingTypeDescription", thing_type_description)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ThingTypeProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ThingTypeProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ThingTypeProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ThingTypeProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut searchable_attributes: Option<::ValueList<String>> = None;
                    let mut thing_type_description: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SearchableAttributes" => {
                                searchable_attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ThingTypeDescription" => {
                                thing_type_description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ThingTypeProperties {
                        searchable_attributes: searchable_attributes,
                        thing_type_description: thing_type_description,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod topic_rule {
    //! Property types for the `TopicRule` resource.

    /// The [`AWS::IoT::TopicRule.Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-action.html) property type.
    #[derive(Debug, Default)]
    pub struct Action {
        /// Property [`CloudwatchAlarm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-action.html#cfn-iot-topicrule-action-cloudwatchalarm).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloudwatch_alarm: Option<::Value<CloudwatchAlarmAction>>,
        /// Property [`CloudwatchLogs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-action.html#cfn-iot-topicrule-action-cloudwatchlogs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloudwatch_logs: Option<::Value<CloudwatchLogsAction>>,
        /// Property [`CloudwatchMetric`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-action.html#cfn-iot-topicrule-action-cloudwatchmetric).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cloudwatch_metric: Option<::Value<CloudwatchMetricAction>>,
        /// Property [`DynamoDB`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-action.html#cfn-iot-topicrule-action-dynamodb).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dynamo_db: Option<::Value<DynamoDBAction>>,
        /// Property [`DynamoDBv2`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-action.html#cfn-iot-topicrule-action-dynamodbv2).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dynamo_d_bv2: Option<::Value<DynamoDBv2Action>>,
        /// Property [`Elasticsearch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-action.html#cfn-iot-topicrule-action-elasticsearch).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub elasticsearch: Option<::Value<ElasticsearchAction>>,
        /// Property [`Firehose`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-action.html#cfn-iot-topicrule-action-firehose).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub firehose: Option<::Value<FirehoseAction>>,
        /// Property [`Http`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-action.html#cfn-iot-topicrule-action-http).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub http: Option<::Value<HttpAction>>,
        /// Property [`IotAnalytics`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-action.html#cfn-iot-topicrule-action-iotanalytics).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub iot_analytics: Option<::Value<IotAnalyticsAction>>,
        /// Property [`IotEvents`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-action.html#cfn-iot-topicrule-action-iotevents).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub iot_events: Option<::Value<IotEventsAction>>,
        /// Property [`IotSiteWise`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-action.html#cfn-iot-topicrule-action-iotsitewise).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub iot_site_wise: Option<::Value<IotSiteWiseAction>>,
        /// Property [`Kafka`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-action.html#cfn-iot-topicrule-action-kafka).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kafka: Option<::Value<KafkaAction>>,
        /// Property [`Kinesis`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-action.html#cfn-iot-topicrule-action-kinesis).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kinesis: Option<::Value<KinesisAction>>,
        /// Property [`Lambda`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-action.html#cfn-iot-topicrule-action-lambda).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lambda: Option<::Value<LambdaAction>>,
        /// Property [`Location`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-action.html#cfn-iot-topicrule-action-location).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub location: Option<::Value<LocationAction>>,
        /// Property [`OpenSearch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-action.html#cfn-iot-topicrule-action-opensearch).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub open_search: Option<::Value<OpenSearchAction>>,
        /// Property [`Republish`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-action.html#cfn-iot-topicrule-action-republish).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub republish: Option<::Value<RepublishAction>>,
        /// Property [`S3`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-action.html#cfn-iot-topicrule-action-s3).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3: Option<::Value<S3Action>>,
        /// Property [`Sns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-action.html#cfn-iot-topicrule-action-sns).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sns: Option<::Value<SnsAction>>,
        /// Property [`Sqs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-action.html#cfn-iot-topicrule-action-sqs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sqs: Option<::Value<SqsAction>>,
        /// Property [`StepFunctions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-action.html#cfn-iot-topicrule-action-stepfunctions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub step_functions: Option<::Value<StepFunctionsAction>>,
        /// Property [`Timestream`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-action.html#cfn-iot-topicrule-action-timestream).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timestream: Option<::Value<TimestreamAction>>,
    }

    impl ::codec::SerializeValue for Action {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cloudwatch_alarm) = self.cloudwatch_alarm {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudwatchAlarm", cloudwatch_alarm)?;
            }
            if let Some(ref cloudwatch_logs) = self.cloudwatch_logs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudwatchLogs", cloudwatch_logs)?;
            }
            if let Some(ref cloudwatch_metric) = self.cloudwatch_metric {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CloudwatchMetric", cloudwatch_metric)?;
            }
            if let Some(ref dynamo_db) = self.dynamo_db {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DynamoDB", dynamo_db)?;
            }
            if let Some(ref dynamo_d_bv2) = self.dynamo_d_bv2 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DynamoDBv2", dynamo_d_bv2)?;
            }
            if let Some(ref elasticsearch) = self.elasticsearch {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Elasticsearch", elasticsearch)?;
            }
            if let Some(ref firehose) = self.firehose {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Firehose", firehose)?;
            }
            if let Some(ref http) = self.http {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Http", http)?;
            }
            if let Some(ref iot_analytics) = self.iot_analytics {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IotAnalytics", iot_analytics)?;
            }
            if let Some(ref iot_events) = self.iot_events {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IotEvents", iot_events)?;
            }
            if let Some(ref iot_site_wise) = self.iot_site_wise {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IotSiteWise", iot_site_wise)?;
            }
            if let Some(ref kafka) = self.kafka {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Kafka", kafka)?;
            }
            if let Some(ref kinesis) = self.kinesis {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Kinesis", kinesis)?;
            }
            if let Some(ref lambda) = self.lambda {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Lambda", lambda)?;
            }
            if let Some(ref location) = self.location {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Location", location)?;
            }
            if let Some(ref open_search) = self.open_search {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OpenSearch", open_search)?;
            }
            if let Some(ref republish) = self.republish {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Republish", republish)?;
            }
            if let Some(ref s3) = self.s3 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3", s3)?;
            }
            if let Some(ref sns) = self.sns {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Sns", sns)?;
            }
            if let Some(ref sqs) = self.sqs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Sqs", sqs)?;
            }
            if let Some(ref step_functions) = self.step_functions {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StepFunctions", step_functions)?;
            }
            if let Some(ref timestream) = self.timestream {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Timestream", timestream)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Action {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Action, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Action;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Action")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cloudwatch_alarm: Option<::Value<CloudwatchAlarmAction>> = None;
                    let mut cloudwatch_logs: Option<::Value<CloudwatchLogsAction>> = None;
                    let mut cloudwatch_metric: Option<::Value<CloudwatchMetricAction>> = None;
                    let mut dynamo_db: Option<::Value<DynamoDBAction>> = None;
                    let mut dynamo_d_bv2: Option<::Value<DynamoDBv2Action>> = None;
                    let mut elasticsearch: Option<::Value<ElasticsearchAction>> = None;
                    let mut firehose: Option<::Value<FirehoseAction>> = None;
                    let mut http: Option<::Value<HttpAction>> = None;
                    let mut iot_analytics: Option<::Value<IotAnalyticsAction>> = None;
                    let mut iot_events: Option<::Value<IotEventsAction>> = None;
                    let mut iot_site_wise: Option<::Value<IotSiteWiseAction>> = None;
                    let mut kafka: Option<::Value<KafkaAction>> = None;
                    let mut kinesis: Option<::Value<KinesisAction>> = None;
                    let mut lambda: Option<::Value<LambdaAction>> = None;
                    let mut location: Option<::Value<LocationAction>> = None;
                    let mut open_search: Option<::Value<OpenSearchAction>> = None;
                    let mut republish: Option<::Value<RepublishAction>> = None;
                    let mut s3: Option<::Value<S3Action>> = None;
                    let mut sns: Option<::Value<SnsAction>> = None;
                    let mut sqs: Option<::Value<SqsAction>> = None;
                    let mut step_functions: Option<::Value<StepFunctionsAction>> = None;
                    let mut timestream: Option<::Value<TimestreamAction>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CloudwatchAlarm" => {
                                cloudwatch_alarm = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CloudwatchLogs" => {
                                cloudwatch_logs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CloudwatchMetric" => {
                                cloudwatch_metric = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DynamoDB" => {
                                dynamo_db = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DynamoDBv2" => {
                                dynamo_d_bv2 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Elasticsearch" => {
                                elasticsearch = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Firehose" => {
                                firehose = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Http" => {
                                http = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IotAnalytics" => {
                                iot_analytics = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IotEvents" => {
                                iot_events = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IotSiteWise" => {
                                iot_site_wise = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Kafka" => {
                                kafka = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Kinesis" => {
                                kinesis = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Lambda" => {
                                lambda = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Location" => {
                                location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OpenSearch" => {
                                open_search = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Republish" => {
                                republish = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3" => {
                                s3 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Sns" => {
                                sns = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Sqs" => {
                                sqs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StepFunctions" => {
                                step_functions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Timestream" => {
                                timestream = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Action {
                        cloudwatch_alarm: cloudwatch_alarm,
                        cloudwatch_logs: cloudwatch_logs,
                        cloudwatch_metric: cloudwatch_metric,
                        dynamo_db: dynamo_db,
                        dynamo_d_bv2: dynamo_d_bv2,
                        elasticsearch: elasticsearch,
                        firehose: firehose,
                        http: http,
                        iot_analytics: iot_analytics,
                        iot_events: iot_events,
                        iot_site_wise: iot_site_wise,
                        kafka: kafka,
                        kinesis: kinesis,
                        lambda: lambda,
                        location: location,
                        open_search: open_search,
                        republish: republish,
                        s3: s3,
                        sns: sns,
                        sqs: sqs,
                        step_functions: step_functions,
                        timestream: timestream,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.AssetPropertyTimestamp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-assetpropertytimestamp.html) property type.
    #[derive(Debug, Default)]
    pub struct AssetPropertyTimestamp {
        /// Property [`OffsetInNanos`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-assetpropertytimestamp.html#cfn-iot-topicrule-assetpropertytimestamp-offsetinnanos).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub offset_in_nanos: Option<::Value<String>>,
        /// Property [`TimeInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-assetpropertytimestamp.html#cfn-iot-topicrule-assetpropertytimestamp-timeinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub time_in_seconds: ::Value<String>,
    }

    impl ::codec::SerializeValue for AssetPropertyTimestamp {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref offset_in_nanos) = self.offset_in_nanos {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OffsetInNanos", offset_in_nanos)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeInSeconds", &self.time_in_seconds)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AssetPropertyTimestamp {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AssetPropertyTimestamp, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AssetPropertyTimestamp;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AssetPropertyTimestamp")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut offset_in_nanos: Option<::Value<String>> = None;
                    let mut time_in_seconds: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "OffsetInNanos" => {
                                offset_in_nanos = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TimeInSeconds" => {
                                time_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AssetPropertyTimestamp {
                        offset_in_nanos: offset_in_nanos,
                        time_in_seconds: time_in_seconds.ok_or(::serde::de::Error::missing_field("TimeInSeconds"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.AssetPropertyValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-assetpropertyvalue.html) property type.
    #[derive(Debug, Default)]
    pub struct AssetPropertyValue {
        /// Property [`Quality`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-assetpropertyvalue.html#cfn-iot-topicrule-assetpropertyvalue-quality).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub quality: Option<::Value<String>>,
        /// Property [`Timestamp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-assetpropertyvalue.html#cfn-iot-topicrule-assetpropertyvalue-timestamp).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timestamp: ::Value<AssetPropertyTimestamp>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-assetpropertyvalue.html#cfn-iot-topicrule-assetpropertyvalue-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<AssetPropertyVariant>,
    }

    impl ::codec::SerializeValue for AssetPropertyValue {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref quality) = self.quality {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Quality", quality)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Timestamp", &self.timestamp)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AssetPropertyValue {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AssetPropertyValue, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AssetPropertyValue;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AssetPropertyValue")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut quality: Option<::Value<String>> = None;
                    let mut timestamp: Option<::Value<AssetPropertyTimestamp>> = None;
                    let mut value: Option<::Value<AssetPropertyVariant>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Quality" => {
                                quality = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Timestamp" => {
                                timestamp = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AssetPropertyValue {
                        quality: quality,
                        timestamp: timestamp.ok_or(::serde::de::Error::missing_field("Timestamp"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.AssetPropertyVariant`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-assetpropertyvariant.html) property type.
    #[derive(Debug, Default)]
    pub struct AssetPropertyVariant {
        /// Property [`BooleanValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-assetpropertyvariant.html#cfn-iot-topicrule-assetpropertyvariant-booleanvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub boolean_value: Option<::Value<String>>,
        /// Property [`DoubleValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-assetpropertyvariant.html#cfn-iot-topicrule-assetpropertyvariant-doublevalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub double_value: Option<::Value<String>>,
        /// Property [`IntegerValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-assetpropertyvariant.html#cfn-iot-topicrule-assetpropertyvariant-integervalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub integer_value: Option<::Value<String>>,
        /// Property [`StringValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-assetpropertyvariant.html#cfn-iot-topicrule-assetpropertyvariant-stringvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub string_value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AssetPropertyVariant {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref boolean_value) = self.boolean_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BooleanValue", boolean_value)?;
            }
            if let Some(ref double_value) = self.double_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DoubleValue", double_value)?;
            }
            if let Some(ref integer_value) = self.integer_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IntegerValue", integer_value)?;
            }
            if let Some(ref string_value) = self.string_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StringValue", string_value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AssetPropertyVariant {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AssetPropertyVariant, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AssetPropertyVariant;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AssetPropertyVariant")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut boolean_value: Option<::Value<String>> = None;
                    let mut double_value: Option<::Value<String>> = None;
                    let mut integer_value: Option<::Value<String>> = None;
                    let mut string_value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BooleanValue" => {
                                boolean_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DoubleValue" => {
                                double_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IntegerValue" => {
                                integer_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StringValue" => {
                                string_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AssetPropertyVariant {
                        boolean_value: boolean_value,
                        double_value: double_value,
                        integer_value: integer_value,
                        string_value: string_value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.CloudwatchAlarmAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-cloudwatchalarmaction.html) property type.
    #[derive(Debug, Default)]
    pub struct CloudwatchAlarmAction {
        /// Property [`AlarmName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-cloudwatchalarmaction.html#cfn-iot-topicrule-cloudwatchalarmaction-alarmname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub alarm_name: ::Value<String>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-cloudwatchalarmaction.html#cfn-iot-topicrule-cloudwatchalarmaction-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`StateReason`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-cloudwatchalarmaction.html#cfn-iot-topicrule-cloudwatchalarmaction-statereason).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub state_reason: ::Value<String>,
        /// Property [`StateValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-cloudwatchalarmaction.html#cfn-iot-topicrule-cloudwatchalarmaction-statevalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub state_value: ::Value<String>,
    }

    impl ::codec::SerializeValue for CloudwatchAlarmAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AlarmName", &self.alarm_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StateReason", &self.state_reason)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StateValue", &self.state_value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CloudwatchAlarmAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CloudwatchAlarmAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CloudwatchAlarmAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CloudwatchAlarmAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut alarm_name: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut state_reason: Option<::Value<String>> = None;
                    let mut state_value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AlarmName" => {
                                alarm_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StateReason" => {
                                state_reason = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StateValue" => {
                                state_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CloudwatchAlarmAction {
                        alarm_name: alarm_name.ok_or(::serde::de::Error::missing_field("AlarmName"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                        state_reason: state_reason.ok_or(::serde::de::Error::missing_field("StateReason"))?,
                        state_value: state_value.ok_or(::serde::de::Error::missing_field("StateValue"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.CloudwatchLogsAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-cloudwatchlogsaction.html) property type.
    #[derive(Debug, Default)]
    pub struct CloudwatchLogsAction {
        /// Property [`BatchMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-cloudwatchlogsaction.html#cfn-iot-topicrule-cloudwatchlogsaction-batchmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub batch_mode: Option<::Value<bool>>,
        /// Property [`LogGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-cloudwatchlogsaction.html#cfn-iot-topicrule-cloudwatchlogsaction-loggroupname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub log_group_name: ::Value<String>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-cloudwatchlogsaction.html#cfn-iot-topicrule-cloudwatchlogsaction-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for CloudwatchLogsAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref batch_mode) = self.batch_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BatchMode", batch_mode)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogGroupName", &self.log_group_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CloudwatchLogsAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CloudwatchLogsAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CloudwatchLogsAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CloudwatchLogsAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut batch_mode: Option<::Value<bool>> = None;
                    let mut log_group_name: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BatchMode" => {
                                batch_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LogGroupName" => {
                                log_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CloudwatchLogsAction {
                        batch_mode: batch_mode,
                        log_group_name: log_group_name.ok_or(::serde::de::Error::missing_field("LogGroupName"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.CloudwatchMetricAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-cloudwatchmetricaction.html) property type.
    #[derive(Debug, Default)]
    pub struct CloudwatchMetricAction {
        /// Property [`MetricName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-cloudwatchmetricaction.html#cfn-iot-topicrule-cloudwatchmetricaction-metricname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric_name: ::Value<String>,
        /// Property [`MetricNamespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-cloudwatchmetricaction.html#cfn-iot-topicrule-cloudwatchmetricaction-metricnamespace).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric_namespace: ::Value<String>,
        /// Property [`MetricTimestamp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-cloudwatchmetricaction.html#cfn-iot-topicrule-cloudwatchmetricaction-metrictimestamp).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric_timestamp: Option<::Value<String>>,
        /// Property [`MetricUnit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-cloudwatchmetricaction.html#cfn-iot-topicrule-cloudwatchmetricaction-metricunit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric_unit: ::Value<String>,
        /// Property [`MetricValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-cloudwatchmetricaction.html#cfn-iot-topicrule-cloudwatchmetricaction-metricvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub metric_value: ::Value<String>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-cloudwatchmetricaction.html#cfn-iot-topicrule-cloudwatchmetricaction-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for CloudwatchMetricAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricName", &self.metric_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricNamespace", &self.metric_namespace)?;
            if let Some(ref metric_timestamp) = self.metric_timestamp {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricTimestamp", metric_timestamp)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricUnit", &self.metric_unit)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetricValue", &self.metric_value)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CloudwatchMetricAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CloudwatchMetricAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CloudwatchMetricAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CloudwatchMetricAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut metric_name: Option<::Value<String>> = None;
                    let mut metric_namespace: Option<::Value<String>> = None;
                    let mut metric_timestamp: Option<::Value<String>> = None;
                    let mut metric_unit: Option<::Value<String>> = None;
                    let mut metric_value: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MetricName" => {
                                metric_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetricNamespace" => {
                                metric_namespace = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetricTimestamp" => {
                                metric_timestamp = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetricUnit" => {
                                metric_unit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MetricValue" => {
                                metric_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CloudwatchMetricAction {
                        metric_name: metric_name.ok_or(::serde::de::Error::missing_field("MetricName"))?,
                        metric_namespace: metric_namespace.ok_or(::serde::de::Error::missing_field("MetricNamespace"))?,
                        metric_timestamp: metric_timestamp,
                        metric_unit: metric_unit.ok_or(::serde::de::Error::missing_field("MetricUnit"))?,
                        metric_value: metric_value.ok_or(::serde::de::Error::missing_field("MetricValue"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.DynamoDBAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-dynamodbaction.html) property type.
    #[derive(Debug, Default)]
    pub struct DynamoDBAction {
        /// Property [`HashKeyField`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-dynamodbaction.html#cfn-iot-topicrule-dynamodbaction-hashkeyfield).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hash_key_field: ::Value<String>,
        /// Property [`HashKeyType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-dynamodbaction.html#cfn-iot-topicrule-dynamodbaction-hashkeytype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hash_key_type: Option<::Value<String>>,
        /// Property [`HashKeyValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-dynamodbaction.html#cfn-iot-topicrule-dynamodbaction-hashkeyvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hash_key_value: ::Value<String>,
        /// Property [`PayloadField`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-dynamodbaction.html#cfn-iot-topicrule-dynamodbaction-payloadfield).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub payload_field: Option<::Value<String>>,
        /// Property [`RangeKeyField`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-dynamodbaction.html#cfn-iot-topicrule-dynamodbaction-rangekeyfield).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub range_key_field: Option<::Value<String>>,
        /// Property [`RangeKeyType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-dynamodbaction.html#cfn-iot-topicrule-dynamodbaction-rangekeytype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub range_key_type: Option<::Value<String>>,
        /// Property [`RangeKeyValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-dynamodbaction.html#cfn-iot-topicrule-dynamodbaction-rangekeyvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub range_key_value: Option<::Value<String>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-dynamodbaction.html#cfn-iot-topicrule-dynamodbaction-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`TableName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-dynamodbaction.html#cfn-iot-topicrule-dynamodbaction-tablename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub table_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for DynamoDBAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HashKeyField", &self.hash_key_field)?;
            if let Some(ref hash_key_type) = self.hash_key_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "HashKeyType", hash_key_type)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HashKeyValue", &self.hash_key_value)?;
            if let Some(ref payload_field) = self.payload_field {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PayloadField", payload_field)?;
            }
            if let Some(ref range_key_field) = self.range_key_field {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RangeKeyField", range_key_field)?;
            }
            if let Some(ref range_key_type) = self.range_key_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RangeKeyType", range_key_type)?;
            }
            if let Some(ref range_key_value) = self.range_key_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RangeKeyValue", range_key_value)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableName", &self.table_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DynamoDBAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DynamoDBAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DynamoDBAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DynamoDBAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut hash_key_field: Option<::Value<String>> = None;
                    let mut hash_key_type: Option<::Value<String>> = None;
                    let mut hash_key_value: Option<::Value<String>> = None;
                    let mut payload_field: Option<::Value<String>> = None;
                    let mut range_key_field: Option<::Value<String>> = None;
                    let mut range_key_type: Option<::Value<String>> = None;
                    let mut range_key_value: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut table_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "HashKeyField" => {
                                hash_key_field = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HashKeyType" => {
                                hash_key_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "HashKeyValue" => {
                                hash_key_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PayloadField" => {
                                payload_field = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RangeKeyField" => {
                                range_key_field = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RangeKeyType" => {
                                range_key_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RangeKeyValue" => {
                                range_key_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TableName" => {
                                table_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DynamoDBAction {
                        hash_key_field: hash_key_field.ok_or(::serde::de::Error::missing_field("HashKeyField"))?,
                        hash_key_type: hash_key_type,
                        hash_key_value: hash_key_value.ok_or(::serde::de::Error::missing_field("HashKeyValue"))?,
                        payload_field: payload_field,
                        range_key_field: range_key_field,
                        range_key_type: range_key_type,
                        range_key_value: range_key_value,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                        table_name: table_name.ok_or(::serde::de::Error::missing_field("TableName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.DynamoDBv2Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-dynamodbv2action.html) property type.
    #[derive(Debug, Default)]
    pub struct DynamoDBv2Action {
        /// Property [`PutItem`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-dynamodbv2action.html#cfn-iot-topicrule-dynamodbv2action-putitem).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub put_item: Option<::Value<PutItemInput>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-dynamodbv2action.html#cfn-iot-topicrule-dynamodbv2action-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DynamoDBv2Action {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref put_item) = self.put_item {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PutItem", put_item)?;
            }
            if let Some(ref role_arn) = self.role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", role_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DynamoDBv2Action {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DynamoDBv2Action, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DynamoDBv2Action;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DynamoDBv2Action")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut put_item: Option<::Value<PutItemInput>> = None;
                    let mut role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PutItem" => {
                                put_item = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DynamoDBv2Action {
                        put_item: put_item,
                        role_arn: role_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.ElasticsearchAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-elasticsearchaction.html) property type.
    #[derive(Debug, Default)]
    pub struct ElasticsearchAction {
        /// Property [`Endpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-elasticsearchaction.html#cfn-iot-topicrule-elasticsearchaction-endpoint).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub endpoint: ::Value<String>,
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-elasticsearchaction.html#cfn-iot-topicrule-elasticsearchaction-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: ::Value<String>,
        /// Property [`Index`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-elasticsearchaction.html#cfn-iot-topicrule-elasticsearchaction-index).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub index: ::Value<String>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-elasticsearchaction.html#cfn-iot-topicrule-elasticsearchaction-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-elasticsearchaction.html#cfn-iot-topicrule-elasticsearchaction-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for ElasticsearchAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Endpoint", &self.endpoint)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Index", &self.index)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ElasticsearchAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ElasticsearchAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ElasticsearchAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ElasticsearchAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut endpoint: Option<::Value<String>> = None;
                    let mut id: Option<::Value<String>> = None;
                    let mut index: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Endpoint" => {
                                endpoint = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Index" => {
                                index = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ElasticsearchAction {
                        endpoint: endpoint.ok_or(::serde::de::Error::missing_field("Endpoint"))?,
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                        index: index.ok_or(::serde::de::Error::missing_field("Index"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.FirehoseAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-firehoseaction.html) property type.
    #[derive(Debug, Default)]
    pub struct FirehoseAction {
        /// Property [`BatchMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-firehoseaction.html#cfn-iot-topicrule-firehoseaction-batchmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub batch_mode: Option<::Value<bool>>,
        /// Property [`DeliveryStreamName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-firehoseaction.html#cfn-iot-topicrule-firehoseaction-deliverystreamname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub delivery_stream_name: ::Value<String>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-firehoseaction.html#cfn-iot-topicrule-firehoseaction-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`Separator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-firehoseaction.html#cfn-iot-topicrule-firehoseaction-separator).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub separator: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for FirehoseAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref batch_mode) = self.batch_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BatchMode", batch_mode)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeliveryStreamName", &self.delivery_stream_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            if let Some(ref separator) = self.separator {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Separator", separator)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FirehoseAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FirehoseAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FirehoseAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FirehoseAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut batch_mode: Option<::Value<bool>> = None;
                    let mut delivery_stream_name: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut separator: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BatchMode" => {
                                batch_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeliveryStreamName" => {
                                delivery_stream_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Separator" => {
                                separator = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FirehoseAction {
                        batch_mode: batch_mode,
                        delivery_stream_name: delivery_stream_name.ok_or(::serde::de::Error::missing_field("DeliveryStreamName"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                        separator: separator,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.HttpAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-httpaction.html) property type.
    #[derive(Debug, Default)]
    pub struct HttpAction {
        /// Property [`Auth`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-httpaction.html#cfn-iot-topicrule-httpaction-auth).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub auth: Option<::Value<HttpAuthorization>>,
        /// Property [`ConfirmationUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-httpaction.html#cfn-iot-topicrule-httpaction-confirmationurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub confirmation_url: Option<::Value<String>>,
        /// Property [`Headers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-httpaction.html#cfn-iot-topicrule-httpaction-headers).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub headers: Option<::ValueList<HttpActionHeader>>,
        /// Property [`Url`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-httpaction.html#cfn-iot-topicrule-httpaction-url).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub url: ::Value<String>,
    }

    impl ::codec::SerializeValue for HttpAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref auth) = self.auth {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Auth", auth)?;
            }
            if let Some(ref confirmation_url) = self.confirmation_url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfirmationUrl", confirmation_url)?;
            }
            if let Some(ref headers) = self.headers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Headers", headers)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Url", &self.url)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HttpAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HttpAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HttpAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HttpAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut auth: Option<::Value<HttpAuthorization>> = None;
                    let mut confirmation_url: Option<::Value<String>> = None;
                    let mut headers: Option<::ValueList<HttpActionHeader>> = None;
                    let mut url: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Auth" => {
                                auth = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConfirmationUrl" => {
                                confirmation_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Headers" => {
                                headers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Url" => {
                                url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HttpAction {
                        auth: auth,
                        confirmation_url: confirmation_url,
                        headers: headers,
                        url: url.ok_or(::serde::de::Error::missing_field("Url"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.HttpActionHeader`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-httpactionheader.html) property type.
    #[derive(Debug, Default)]
    pub struct HttpActionHeader {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-httpactionheader.html#cfn-iot-topicrule-httpactionheader-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-httpactionheader.html#cfn-iot-topicrule-httpactionheader-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for HttpActionHeader {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HttpActionHeader {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HttpActionHeader, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HttpActionHeader;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HttpActionHeader")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

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

                    Ok(HttpActionHeader {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.HttpAuthorization`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-httpauthorization.html) property type.
    #[derive(Debug, Default)]
    pub struct HttpAuthorization {
        /// Property [`Sigv4`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-httpauthorization.html#cfn-iot-topicrule-httpauthorization-sigv4).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sigv4: Option<::Value<SigV4Authorization>>,
    }

    impl ::codec::SerializeValue for HttpAuthorization {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref sigv4) = self.sigv4 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Sigv4", sigv4)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HttpAuthorization {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HttpAuthorization, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HttpAuthorization;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HttpAuthorization")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut sigv4: Option<::Value<SigV4Authorization>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Sigv4" => {
                                sigv4 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HttpAuthorization {
                        sigv4: sigv4,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.IotAnalyticsAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-iotanalyticsaction.html) property type.
    #[derive(Debug, Default)]
    pub struct IotAnalyticsAction {
        /// Property [`BatchMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-iotanalyticsaction.html#cfn-iot-topicrule-iotanalyticsaction-batchmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub batch_mode: Option<::Value<bool>>,
        /// Property [`ChannelName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-iotanalyticsaction.html#cfn-iot-topicrule-iotanalyticsaction-channelname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub channel_name: ::Value<String>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-iotanalyticsaction.html#cfn-iot-topicrule-iotanalyticsaction-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for IotAnalyticsAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref batch_mode) = self.batch_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BatchMode", batch_mode)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ChannelName", &self.channel_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IotAnalyticsAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IotAnalyticsAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IotAnalyticsAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IotAnalyticsAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut batch_mode: Option<::Value<bool>> = None;
                    let mut channel_name: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BatchMode" => {
                                batch_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ChannelName" => {
                                channel_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IotAnalyticsAction {
                        batch_mode: batch_mode,
                        channel_name: channel_name.ok_or(::serde::de::Error::missing_field("ChannelName"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.IotEventsAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-ioteventsaction.html) property type.
    #[derive(Debug, Default)]
    pub struct IotEventsAction {
        /// Property [`BatchMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-ioteventsaction.html#cfn-iot-topicrule-ioteventsaction-batchmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub batch_mode: Option<::Value<bool>>,
        /// Property [`InputName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-ioteventsaction.html#cfn-iot-topicrule-ioteventsaction-inputname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub input_name: ::Value<String>,
        /// Property [`MessageId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-ioteventsaction.html#cfn-iot-topicrule-ioteventsaction-messageid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub message_id: Option<::Value<String>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-ioteventsaction.html#cfn-iot-topicrule-ioteventsaction-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for IotEventsAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref batch_mode) = self.batch_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BatchMode", batch_mode)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InputName", &self.input_name)?;
            if let Some(ref message_id) = self.message_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MessageId", message_id)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IotEventsAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IotEventsAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IotEventsAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IotEventsAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut batch_mode: Option<::Value<bool>> = None;
                    let mut input_name: Option<::Value<String>> = None;
                    let mut message_id: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BatchMode" => {
                                batch_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InputName" => {
                                input_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MessageId" => {
                                message_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IotEventsAction {
                        batch_mode: batch_mode,
                        input_name: input_name.ok_or(::serde::de::Error::missing_field("InputName"))?,
                        message_id: message_id,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.IotSiteWiseAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-iotsitewiseaction.html) property type.
    #[derive(Debug, Default)]
    pub struct IotSiteWiseAction {
        /// Property [`PutAssetPropertyValueEntries`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-iotsitewiseaction.html#cfn-iot-topicrule-iotsitewiseaction-putassetpropertyvalueentries).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub put_asset_property_value_entries: ::ValueList<PutAssetPropertyValueEntry>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-iotsitewiseaction.html#cfn-iot-topicrule-iotsitewiseaction-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for IotSiteWiseAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PutAssetPropertyValueEntries", &self.put_asset_property_value_entries)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IotSiteWiseAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IotSiteWiseAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IotSiteWiseAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IotSiteWiseAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut put_asset_property_value_entries: Option<::ValueList<PutAssetPropertyValueEntry>> = None;
                    let mut role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PutAssetPropertyValueEntries" => {
                                put_asset_property_value_entries = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IotSiteWiseAction {
                        put_asset_property_value_entries: put_asset_property_value_entries.ok_or(::serde::de::Error::missing_field("PutAssetPropertyValueEntries"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.KafkaAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-kafkaaction.html) property type.
    #[derive(Debug, Default)]
    pub struct KafkaAction {
        /// Property [`ClientProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-kafkaaction.html#cfn-iot-topicrule-kafkaaction-clientproperties).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub client_properties: ::ValueMap<String>,
        /// Property [`DestinationArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-kafkaaction.html#cfn-iot-topicrule-kafkaaction-destinationarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub destination_arn: ::Value<String>,
        /// Property [`Headers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-kafkaaction.html#cfn-iot-topicrule-kafkaaction-headers).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub headers: Option<::ValueList<KafkaActionHeader>>,
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-kafkaaction.html#cfn-iot-topicrule-kafkaaction-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: Option<::Value<String>>,
        /// Property [`Partition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-kafkaaction.html#cfn-iot-topicrule-kafkaaction-partition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub partition: Option<::Value<String>>,
        /// Property [`Topic`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-kafkaaction.html#cfn-iot-topicrule-kafkaaction-topic).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub topic: ::Value<String>,
    }

    impl ::codec::SerializeValue for KafkaAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientProperties", &self.client_properties)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationArn", &self.destination_arn)?;
            if let Some(ref headers) = self.headers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Headers", headers)?;
            }
            if let Some(ref key) = self.key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", key)?;
            }
            if let Some(ref partition) = self.partition {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Partition", partition)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Topic", &self.topic)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KafkaAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KafkaAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KafkaAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KafkaAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut client_properties: Option<::ValueMap<String>> = None;
                    let mut destination_arn: Option<::Value<String>> = None;
                    let mut headers: Option<::ValueList<KafkaActionHeader>> = None;
                    let mut key: Option<::Value<String>> = None;
                    let mut partition: Option<::Value<String>> = None;
                    let mut topic: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ClientProperties" => {
                                client_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DestinationArn" => {
                                destination_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Headers" => {
                                headers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Partition" => {
                                partition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Topic" => {
                                topic = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KafkaAction {
                        client_properties: client_properties.ok_or(::serde::de::Error::missing_field("ClientProperties"))?,
                        destination_arn: destination_arn.ok_or(::serde::de::Error::missing_field("DestinationArn"))?,
                        headers: headers,
                        key: key,
                        partition: partition,
                        topic: topic.ok_or(::serde::de::Error::missing_field("Topic"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.KafkaActionHeader`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-kafkaactionheader.html) property type.
    #[derive(Debug, Default)]
    pub struct KafkaActionHeader {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-kafkaactionheader.html#cfn-iot-topicrule-kafkaactionheader-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-kafkaactionheader.html#cfn-iot-topicrule-kafkaactionheader-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for KafkaActionHeader {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KafkaActionHeader {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KafkaActionHeader, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KafkaActionHeader;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KafkaActionHeader")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

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

                    Ok(KafkaActionHeader {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.KinesisAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-kinesisaction.html) property type.
    #[derive(Debug, Default)]
    pub struct KinesisAction {
        /// Property [`PartitionKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-kinesisaction.html#cfn-iot-topicrule-kinesisaction-partitionkey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub partition_key: Option<::Value<String>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-kinesisaction.html#cfn-iot-topicrule-kinesisaction-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`StreamName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-kinesisaction.html#cfn-iot-topicrule-kinesisaction-streamname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stream_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for KinesisAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref partition_key) = self.partition_key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PartitionKey", partition_key)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamName", &self.stream_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KinesisAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KinesisAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KinesisAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KinesisAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut partition_key: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut stream_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PartitionKey" => {
                                partition_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StreamName" => {
                                stream_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KinesisAction {
                        partition_key: partition_key,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                        stream_name: stream_name.ok_or(::serde::de::Error::missing_field("StreamName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.LambdaAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-lambdaaction.html) property type.
    #[derive(Debug, Default)]
    pub struct LambdaAction {
        /// Property [`FunctionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-lambdaaction.html#cfn-iot-topicrule-lambdaaction-functionarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub function_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LambdaAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref function_arn) = self.function_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FunctionArn", function_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LambdaAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LambdaAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LambdaAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LambdaAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut function_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FunctionArn" => {
                                function_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LambdaAction {
                        function_arn: function_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.LocationAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-locationaction.html) property type.
    #[derive(Debug, Default)]
    pub struct LocationAction {
        /// Property [`DeviceId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-locationaction.html#cfn-iot-topicrule-locationaction-deviceid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub device_id: ::Value<String>,
        /// Property [`Latitude`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-locationaction.html#cfn-iot-topicrule-locationaction-latitude).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub latitude: ::Value<String>,
        /// Property [`Longitude`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-locationaction.html#cfn-iot-topicrule-locationaction-longitude).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub longitude: ::Value<String>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-locationaction.html#cfn-iot-topicrule-locationaction-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`Timestamp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-locationaction.html#cfn-iot-topicrule-locationaction-timestamp).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timestamp: Option<::Value<Timestamp>>,
        /// Property [`TrackerName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-locationaction.html#cfn-iot-topicrule-locationaction-trackername).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub tracker_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for LocationAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceId", &self.device_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Latitude", &self.latitude)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Longitude", &self.longitude)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            if let Some(ref timestamp) = self.timestamp {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Timestamp", timestamp)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TrackerName", &self.tracker_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LocationAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LocationAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LocationAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LocationAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut device_id: Option<::Value<String>> = None;
                    let mut latitude: Option<::Value<String>> = None;
                    let mut longitude: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut timestamp: Option<::Value<Timestamp>> = None;
                    let mut tracker_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DeviceId" => {
                                device_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Latitude" => {
                                latitude = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Longitude" => {
                                longitude = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Timestamp" => {
                                timestamp = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TrackerName" => {
                                tracker_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(LocationAction {
                        device_id: device_id.ok_or(::serde::de::Error::missing_field("DeviceId"))?,
                        latitude: latitude.ok_or(::serde::de::Error::missing_field("Latitude"))?,
                        longitude: longitude.ok_or(::serde::de::Error::missing_field("Longitude"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                        timestamp: timestamp,
                        tracker_name: tracker_name.ok_or(::serde::de::Error::missing_field("TrackerName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.OpenSearchAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-opensearchaction.html) property type.
    #[derive(Debug, Default)]
    pub struct OpenSearchAction {
        /// Property [`Endpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-opensearchaction.html#cfn-iot-topicrule-opensearchaction-endpoint).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub endpoint: ::Value<String>,
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-opensearchaction.html#cfn-iot-topicrule-opensearchaction-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: ::Value<String>,
        /// Property [`Index`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-opensearchaction.html#cfn-iot-topicrule-opensearchaction-index).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub index: ::Value<String>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-opensearchaction.html#cfn-iot-topicrule-opensearchaction-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-opensearchaction.html#cfn-iot-topicrule-opensearchaction-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for OpenSearchAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Endpoint", &self.endpoint)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Index", &self.index)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OpenSearchAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OpenSearchAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OpenSearchAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OpenSearchAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut endpoint: Option<::Value<String>> = None;
                    let mut id: Option<::Value<String>> = None;
                    let mut index: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Endpoint" => {
                                endpoint = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Index" => {
                                index = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OpenSearchAction {
                        endpoint: endpoint.ok_or(::serde::de::Error::missing_field("Endpoint"))?,
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                        index: index.ok_or(::serde::de::Error::missing_field("Index"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.PutAssetPropertyValueEntry`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-putassetpropertyvalueentry.html) property type.
    #[derive(Debug, Default)]
    pub struct PutAssetPropertyValueEntry {
        /// Property [`AssetId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-putassetpropertyvalueentry.html#cfn-iot-topicrule-putassetpropertyvalueentry-assetid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub asset_id: Option<::Value<String>>,
        /// Property [`EntryId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-putassetpropertyvalueentry.html#cfn-iot-topicrule-putassetpropertyvalueentry-entryid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub entry_id: Option<::Value<String>>,
        /// Property [`PropertyAlias`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-putassetpropertyvalueentry.html#cfn-iot-topicrule-putassetpropertyvalueentry-propertyalias).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub property_alias: Option<::Value<String>>,
        /// Property [`PropertyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-putassetpropertyvalueentry.html#cfn-iot-topicrule-putassetpropertyvalueentry-propertyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub property_id: Option<::Value<String>>,
        /// Property [`PropertyValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-putassetpropertyvalueentry.html#cfn-iot-topicrule-putassetpropertyvalueentry-propertyvalues).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub property_values: ::ValueList<AssetPropertyValue>,
    }

    impl ::codec::SerializeValue for PutAssetPropertyValueEntry {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref asset_id) = self.asset_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssetId", asset_id)?;
            }
            if let Some(ref entry_id) = self.entry_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EntryId", entry_id)?;
            }
            if let Some(ref property_alias) = self.property_alias {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PropertyAlias", property_alias)?;
            }
            if let Some(ref property_id) = self.property_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PropertyId", property_id)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PropertyValues", &self.property_values)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PutAssetPropertyValueEntry {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PutAssetPropertyValueEntry, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PutAssetPropertyValueEntry;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PutAssetPropertyValueEntry")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut asset_id: Option<::Value<String>> = None;
                    let mut entry_id: Option<::Value<String>> = None;
                    let mut property_alias: Option<::Value<String>> = None;
                    let mut property_id: Option<::Value<String>> = None;
                    let mut property_values: Option<::ValueList<AssetPropertyValue>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AssetId" => {
                                asset_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EntryId" => {
                                entry_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PropertyAlias" => {
                                property_alias = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PropertyId" => {
                                property_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PropertyValues" => {
                                property_values = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PutAssetPropertyValueEntry {
                        asset_id: asset_id,
                        entry_id: entry_id,
                        property_alias: property_alias,
                        property_id: property_id,
                        property_values: property_values.ok_or(::serde::de::Error::missing_field("PropertyValues"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.PutItemInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-putiteminput.html) property type.
    #[derive(Debug, Default)]
    pub struct PutItemInput {
        /// Property [`TableName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-putiteminput.html#cfn-iot-topicrule-putiteminput-tablename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub table_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for PutItemInput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableName", &self.table_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PutItemInput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PutItemInput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PutItemInput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PutItemInput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut table_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "TableName" => {
                                table_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PutItemInput {
                        table_name: table_name.ok_or(::serde::de::Error::missing_field("TableName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.RepublishAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-republishaction.html) property type.
    #[derive(Debug, Default)]
    pub struct RepublishAction {
        /// Property [`Headers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-republishaction.html#cfn-iot-topicrule-republishaction-headers).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub headers: Option<::Value<RepublishActionHeaders>>,
        /// Property [`Qos`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-republishaction.html#cfn-iot-topicrule-republishaction-qos).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub qos: Option<::Value<u32>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-republishaction.html#cfn-iot-topicrule-republishaction-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`Topic`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-republishaction.html#cfn-iot-topicrule-republishaction-topic).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub topic: ::Value<String>,
    }

    impl ::codec::SerializeValue for RepublishAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref headers) = self.headers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Headers", headers)?;
            }
            if let Some(ref qos) = self.qos {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Qos", qos)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Topic", &self.topic)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RepublishAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RepublishAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RepublishAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RepublishAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut headers: Option<::Value<RepublishActionHeaders>> = None;
                    let mut qos: Option<::Value<u32>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut topic: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Headers" => {
                                headers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Qos" => {
                                qos = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Topic" => {
                                topic = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RepublishAction {
                        headers: headers,
                        qos: qos,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                        topic: topic.ok_or(::serde::de::Error::missing_field("Topic"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.RepublishActionHeaders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-republishactionheaders.html) property type.
    #[derive(Debug, Default)]
    pub struct RepublishActionHeaders {
        /// Property [`ContentType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-republishactionheaders.html#cfn-iot-topicrule-republishactionheaders-contenttype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub content_type: Option<::Value<String>>,
        /// Property [`CorrelationData`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-republishactionheaders.html#cfn-iot-topicrule-republishactionheaders-correlationdata).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub correlation_data: Option<::Value<String>>,
        /// Property [`MessageExpiry`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-republishactionheaders.html#cfn-iot-topicrule-republishactionheaders-messageexpiry).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub message_expiry: Option<::Value<String>>,
        /// Property [`PayloadFormatIndicator`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-republishactionheaders.html#cfn-iot-topicrule-republishactionheaders-payloadformatindicator).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub payload_format_indicator: Option<::Value<String>>,
        /// Property [`ResponseTopic`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-republishactionheaders.html#cfn-iot-topicrule-republishactionheaders-responsetopic).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub response_topic: Option<::Value<String>>,
        /// Property [`UserProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-republishactionheaders.html#cfn-iot-topicrule-republishactionheaders-userproperties).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_properties: Option<::ValueList<UserProperty>>,
    }

    impl ::codec::SerializeValue for RepublishActionHeaders {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref content_type) = self.content_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContentType", content_type)?;
            }
            if let Some(ref correlation_data) = self.correlation_data {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CorrelationData", correlation_data)?;
            }
            if let Some(ref message_expiry) = self.message_expiry {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MessageExpiry", message_expiry)?;
            }
            if let Some(ref payload_format_indicator) = self.payload_format_indicator {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PayloadFormatIndicator", payload_format_indicator)?;
            }
            if let Some(ref response_topic) = self.response_topic {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResponseTopic", response_topic)?;
            }
            if let Some(ref user_properties) = self.user_properties {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserProperties", user_properties)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RepublishActionHeaders {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RepublishActionHeaders, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RepublishActionHeaders;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RepublishActionHeaders")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut content_type: Option<::Value<String>> = None;
                    let mut correlation_data: Option<::Value<String>> = None;
                    let mut message_expiry: Option<::Value<String>> = None;
                    let mut payload_format_indicator: Option<::Value<String>> = None;
                    let mut response_topic: Option<::Value<String>> = None;
                    let mut user_properties: Option<::ValueList<UserProperty>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContentType" => {
                                content_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CorrelationData" => {
                                correlation_data = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MessageExpiry" => {
                                message_expiry = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PayloadFormatIndicator" => {
                                payload_format_indicator = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ResponseTopic" => {
                                response_topic = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserProperties" => {
                                user_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RepublishActionHeaders {
                        content_type: content_type,
                        correlation_data: correlation_data,
                        message_expiry: message_expiry,
                        payload_format_indicator: payload_format_indicator,
                        response_topic: response_topic,
                        user_properties: user_properties,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.S3Action`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-s3action.html) property type.
    #[derive(Debug, Default)]
    pub struct S3Action {
        /// Property [`BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-s3action.html#cfn-iot-topicrule-s3action-bucketname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_name: ::Value<String>,
        /// Property [`CannedAcl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-s3action.html#cfn-iot-topicrule-s3action-cannedacl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub canned_acl: Option<::Value<String>>,
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-s3action.html#cfn-iot-topicrule-s3action-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-s3action.html#cfn-iot-topicrule-s3action-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for S3Action {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketName", &self.bucket_name)?;
            if let Some(ref canned_acl) = self.canned_acl {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CannedAcl", canned_acl)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3Action {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3Action, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3Action;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3Action")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket_name: Option<::Value<String>> = None;
                    let mut canned_acl: Option<::Value<String>> = None;
                    let mut key: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketName" => {
                                bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CannedAcl" => {
                                canned_acl = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Action {
                        bucket_name: bucket_name.ok_or(::serde::de::Error::missing_field("BucketName"))?,
                        canned_acl: canned_acl,
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.SigV4Authorization`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-sigv4authorization.html) property type.
    #[derive(Debug, Default)]
    pub struct SigV4Authorization {
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-sigv4authorization.html#cfn-iot-topicrule-sigv4authorization-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`ServiceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-sigv4authorization.html#cfn-iot-topicrule-sigv4authorization-servicename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub service_name: ::Value<String>,
        /// Property [`SigningRegion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-sigv4authorization.html#cfn-iot-topicrule-sigv4authorization-signingregion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub signing_region: ::Value<String>,
    }

    impl ::codec::SerializeValue for SigV4Authorization {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceName", &self.service_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SigningRegion", &self.signing_region)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SigV4Authorization {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SigV4Authorization, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SigV4Authorization;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SigV4Authorization")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut service_name: Option<::Value<String>> = None;
                    let mut signing_region: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServiceName" => {
                                service_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SigningRegion" => {
                                signing_region = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SigV4Authorization {
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                        service_name: service_name.ok_or(::serde::de::Error::missing_field("ServiceName"))?,
                        signing_region: signing_region.ok_or(::serde::de::Error::missing_field("SigningRegion"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.SnsAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-snsaction.html) property type.
    #[derive(Debug, Default)]
    pub struct SnsAction {
        /// Property [`MessageFormat`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-snsaction.html#cfn-iot-topicrule-snsaction-messageformat).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub message_format: Option<::Value<String>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-snsaction.html#cfn-iot-topicrule-snsaction-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`TargetArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-snsaction.html#cfn-iot-topicrule-snsaction-targetarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub target_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for SnsAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref message_format) = self.message_format {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MessageFormat", message_format)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetArn", &self.target_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SnsAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SnsAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SnsAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SnsAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut message_format: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut target_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MessageFormat" => {
                                message_format = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TargetArn" => {
                                target_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SnsAction {
                        message_format: message_format,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                        target_arn: target_arn.ok_or(::serde::de::Error::missing_field("TargetArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.SqsAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-sqsaction.html) property type.
    #[derive(Debug, Default)]
    pub struct SqsAction {
        /// Property [`QueueUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-sqsaction.html#cfn-iot-topicrule-sqsaction-queueurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub queue_url: ::Value<String>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-sqsaction.html#cfn-iot-topicrule-sqsaction-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`UseBase64`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-sqsaction.html#cfn-iot-topicrule-sqsaction-usebase64).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub use_base64: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for SqsAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueueUrl", &self.queue_url)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            if let Some(ref use_base64) = self.use_base64 {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UseBase64", use_base64)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SqsAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SqsAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SqsAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SqsAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut queue_url: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut use_base64: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "QueueUrl" => {
                                queue_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UseBase64" => {
                                use_base64 = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SqsAction {
                        queue_url: queue_url.ok_or(::serde::de::Error::missing_field("QueueUrl"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                        use_base64: use_base64,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.StepFunctionsAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-stepfunctionsaction.html) property type.
    #[derive(Debug, Default)]
    pub struct StepFunctionsAction {
        /// Property [`ExecutionNamePrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-stepfunctionsaction.html#cfn-iot-topicrule-stepfunctionsaction-executionnameprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub execution_name_prefix: Option<::Value<String>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-stepfunctionsaction.html#cfn-iot-topicrule-stepfunctionsaction-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`StateMachineName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-stepfunctionsaction.html#cfn-iot-topicrule-stepfunctionsaction-statemachinename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub state_machine_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for StepFunctionsAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref execution_name_prefix) = self.execution_name_prefix {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExecutionNamePrefix", execution_name_prefix)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StateMachineName", &self.state_machine_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StepFunctionsAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StepFunctionsAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StepFunctionsAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StepFunctionsAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut execution_name_prefix: Option<::Value<String>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut state_machine_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ExecutionNamePrefix" => {
                                execution_name_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StateMachineName" => {
                                state_machine_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StepFunctionsAction {
                        execution_name_prefix: execution_name_prefix,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                        state_machine_name: state_machine_name.ok_or(::serde::de::Error::missing_field("StateMachineName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.Timestamp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-timestamp.html) property type.
    #[derive(Debug, Default)]
    pub struct Timestamp {
        /// Property [`Unit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-timestamp.html#cfn-iot-topicrule-timestamp-unit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unit: Option<::Value<String>>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-timestamp.html#cfn-iot-topicrule-timestamp-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for Timestamp {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref unit) = self.unit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Unit", unit)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Timestamp {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Timestamp, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Timestamp;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Timestamp")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut unit: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Unit" => {
                                unit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Timestamp {
                        unit: unit,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.TimestreamAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-timestreamaction.html) property type.
    #[derive(Debug, Default)]
    pub struct TimestreamAction {
        /// Property [`DatabaseName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-timestreamaction.html#cfn-iot-topicrule-timestreamaction-databasename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub database_name: ::Value<String>,
        /// Property [`Dimensions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-timestreamaction.html#cfn-iot-topicrule-timestreamaction-dimensions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub dimensions: ::ValueList<TimestreamDimension>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-timestreamaction.html#cfn-iot-topicrule-timestreamaction-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`TableName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-timestreamaction.html#cfn-iot-topicrule-timestreamaction-tablename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub table_name: ::Value<String>,
        /// Property [`Timestamp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-timestreamaction.html#cfn-iot-topicrule-timestreamaction-timestamp).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub timestamp: Option<::Value<TimestreamTimestamp>>,
    }

    impl ::codec::SerializeValue for TimestreamAction {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", &self.database_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Dimensions", &self.dimensions)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TableName", &self.table_name)?;
            if let Some(ref timestamp) = self.timestamp {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Timestamp", timestamp)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TimestreamAction {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TimestreamAction, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TimestreamAction;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TimestreamAction")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut database_name: Option<::Value<String>> = None;
                    let mut dimensions: Option<::ValueList<TimestreamDimension>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut table_name: Option<::Value<String>> = None;
                    let mut timestamp: Option<::Value<TimestreamTimestamp>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DatabaseName" => {
                                database_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Dimensions" => {
                                dimensions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TableName" => {
                                table_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Timestamp" => {
                                timestamp = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TimestreamAction {
                        database_name: database_name.ok_or(::serde::de::Error::missing_field("DatabaseName"))?,
                        dimensions: dimensions.ok_or(::serde::de::Error::missing_field("Dimensions"))?,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                        table_name: table_name.ok_or(::serde::de::Error::missing_field("TableName"))?,
                        timestamp: timestamp,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.TimestreamDimension`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-timestreamdimension.html) property type.
    #[derive(Debug, Default)]
    pub struct TimestreamDimension {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-timestreamdimension.html#cfn-iot-topicrule-timestreamdimension-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-timestreamdimension.html#cfn-iot-topicrule-timestreamdimension-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for TimestreamDimension {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TimestreamDimension {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TimestreamDimension, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TimestreamDimension;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TimestreamDimension")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TimestreamDimension {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.TimestreamTimestamp`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-timestreamtimestamp.html) property type.
    #[derive(Debug, Default)]
    pub struct TimestreamTimestamp {
        /// Property [`Unit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-timestreamtimestamp.html#cfn-iot-topicrule-timestreamtimestamp-unit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub unit: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-timestreamtimestamp.html#cfn-iot-topicrule-timestreamtimestamp-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for TimestreamTimestamp {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Unit", &self.unit)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TimestreamTimestamp {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TimestreamTimestamp, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TimestreamTimestamp;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TimestreamTimestamp")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut unit: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Unit" => {
                                unit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TimestreamTimestamp {
                        unit: unit.ok_or(::serde::de::Error::missing_field("Unit"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.TopicRulePayload`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-topicrulepayload.html) property type.
    #[derive(Debug, Default)]
    pub struct TopicRulePayload {
        /// Property [`Actions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-topicrulepayload.html#cfn-iot-topicrule-topicrulepayload-actions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub actions: ::ValueList<Action>,
        /// Property [`AwsIotSqlVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-topicrulepayload.html#cfn-iot-topicrule-topicrulepayload-awsiotsqlversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub aws_iot_sql_version: Option<::Value<String>>,
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-topicrulepayload.html#cfn-iot-topicrule-topicrulepayload-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`ErrorAction`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-topicrulepayload.html#cfn-iot-topicrule-topicrulepayload-erroraction).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub error_action: Option<::Value<Action>>,
        /// Property [`RuleDisabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-topicrulepayload.html#cfn-iot-topicrule-topicrulepayload-ruledisabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub rule_disabled: Option<::Value<bool>>,
        /// Property [`Sql`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-topicrulepayload.html#cfn-iot-topicrule-topicrulepayload-sql).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sql: ::Value<String>,
    }

    impl ::codec::SerializeValue for TopicRulePayload {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Actions", &self.actions)?;
            if let Some(ref aws_iot_sql_version) = self.aws_iot_sql_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AwsIotSqlVersion", aws_iot_sql_version)?;
            }
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            if let Some(ref error_action) = self.error_action {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ErrorAction", error_action)?;
            }
            if let Some(ref rule_disabled) = self.rule_disabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RuleDisabled", rule_disabled)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Sql", &self.sql)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TopicRulePayload {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TopicRulePayload, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TopicRulePayload;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TopicRulePayload")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut actions: Option<::ValueList<Action>> = None;
                    let mut aws_iot_sql_version: Option<::Value<String>> = None;
                    let mut description: Option<::Value<String>> = None;
                    let mut error_action: Option<::Value<Action>> = None;
                    let mut rule_disabled: Option<::Value<bool>> = None;
                    let mut sql: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Actions" => {
                                actions = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AwsIotSqlVersion" => {
                                aws_iot_sql_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ErrorAction" => {
                                error_action = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RuleDisabled" => {
                                rule_disabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Sql" => {
                                sql = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TopicRulePayload {
                        actions: actions.ok_or(::serde::de::Error::missing_field("Actions"))?,
                        aws_iot_sql_version: aws_iot_sql_version,
                        description: description,
                        error_action: error_action,
                        rule_disabled: rule_disabled,
                        sql: sql.ok_or(::serde::de::Error::missing_field("Sql"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRule.UserProperty`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-userproperty.html) property type.
    #[derive(Debug, Default)]
    pub struct UserProperty {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-userproperty.html#cfn-iot-topicrule-userproperty-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicrule-userproperty.html#cfn-iot-topicrule-userproperty-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<String>,
    }

    impl ::codec::SerializeValue for UserProperty {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for UserProperty {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<UserProperty, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = UserProperty;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type UserProperty")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut value: Option<::Value<String>> = None;

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

                    Ok(UserProperty {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod topic_rule_destination {
    //! Property types for the `TopicRuleDestination` resource.

    /// The [`AWS::IoT::TopicRuleDestination.HttpUrlDestinationSummary`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicruledestination-httpurldestinationsummary.html) property type.
    #[derive(Debug, Default)]
    pub struct HttpUrlDestinationSummary {
        /// Property [`ConfirmationUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicruledestination-httpurldestinationsummary.html#cfn-iot-topicruledestination-httpurldestinationsummary-confirmationurl).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub confirmation_url: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for HttpUrlDestinationSummary {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref confirmation_url) = self.confirmation_url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfirmationUrl", confirmation_url)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HttpUrlDestinationSummary {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HttpUrlDestinationSummary, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HttpUrlDestinationSummary;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HttpUrlDestinationSummary")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut confirmation_url: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConfirmationUrl" => {
                                confirmation_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HttpUrlDestinationSummary {
                        confirmation_url: confirmation_url,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::IoT::TopicRuleDestination.VpcDestinationProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicruledestination-vpcdestinationproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct VpcDestinationProperties {
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicruledestination-vpcdestinationproperties.html#cfn-iot-topicruledestination-vpcdestinationproperties-rolearn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub role_arn: Option<::Value<String>>,
        /// Property [`SecurityGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicruledestination-vpcdestinationproperties.html#cfn-iot-topicruledestination-vpcdestinationproperties-securitygroups).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub security_groups: Option<::ValueList<String>>,
        /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicruledestination-vpcdestinationproperties.html#cfn-iot-topicruledestination-vpcdestinationproperties-subnetids).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub subnet_ids: Option<::ValueList<String>>,
        /// Property [`VpcId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iot-topicruledestination-vpcdestinationproperties.html#cfn-iot-topicruledestination-vpcdestinationproperties-vpcid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub vpc_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for VpcDestinationProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref role_arn) = self.role_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", role_arn)?;
            }
            if let Some(ref security_groups) = self.security_groups {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroups", security_groups)?;
            }
            if let Some(ref subnet_ids) = self.subnet_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", subnet_ids)?;
            }
            if let Some(ref vpc_id) = self.vpc_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcId", vpc_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VpcDestinationProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VpcDestinationProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VpcDestinationProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VpcDestinationProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut security_groups: Option<::ValueList<String>> = None;
                    let mut subnet_ids: Option<::ValueList<String>> = None;
                    let mut vpc_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecurityGroups" => {
                                security_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubnetIds" => {
                                subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpcId" => {
                                vpc_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VpcDestinationProperties {
                        role_arn: role_arn,
                        security_groups: security_groups,
                        subnet_ids: subnet_ids,
                        vpc_id: vpc_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
