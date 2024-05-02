//! Types for the `WorkSpacesWeb` service.

/// The [`AWS::WorkSpacesWeb::BrowserSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-browsersettings.html) resource type.
#[derive(Debug, Default)]
pub struct BrowserSettings {
    properties: BrowserSettingsProperties
}

/// Properties for the `BrowserSettings` resource.
#[derive(Debug, Default)]
pub struct BrowserSettingsProperties {
    /// Property [`AdditionalEncryptionContext`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-browsersettings.html#cfn-workspacesweb-browsersettings-additionalencryptioncontext).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub additional_encryption_context: Option<::ValueMap<String>>,
    /// Property [`BrowserPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-browsersettings.html#cfn-workspacesweb-browsersettings-browserpolicy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub browser_policy: Option<::Value<String>>,
    /// Property [`CustomerManagedKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-browsersettings.html#cfn-workspacesweb-browsersettings-customermanagedkey).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub customer_managed_key: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-browsersettings.html#cfn-workspacesweb-browsersettings-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for BrowserSettingsProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref additional_encryption_context) = self.additional_encryption_context {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdditionalEncryptionContext", additional_encryption_context)?;
        }
        if let Some(ref browser_policy) = self.browser_policy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BrowserPolicy", browser_policy)?;
        }
        if let Some(ref customer_managed_key) = self.customer_managed_key {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomerManagedKey", customer_managed_key)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for BrowserSettingsProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<BrowserSettingsProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = BrowserSettingsProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type BrowserSettingsProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut additional_encryption_context: Option<::ValueMap<String>> = None;
                let mut browser_policy: Option<::Value<String>> = None;
                let mut customer_managed_key: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AdditionalEncryptionContext" => {
                            additional_encryption_context = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BrowserPolicy" => {
                            browser_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CustomerManagedKey" => {
                            customer_managed_key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(BrowserSettingsProperties {
                    additional_encryption_context: additional_encryption_context,
                    browser_policy: browser_policy,
                    customer_managed_key: customer_managed_key,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for BrowserSettings {
    type Properties = BrowserSettingsProperties;
    const TYPE: &'static str = "AWS::WorkSpacesWeb::BrowserSettings";
    fn properties(&self) -> &BrowserSettingsProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut BrowserSettingsProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for BrowserSettings {}

impl From<BrowserSettingsProperties> for BrowserSettings {
    fn from(properties: BrowserSettingsProperties) -> BrowserSettings {
        BrowserSettings { properties }
    }
}

/// The [`AWS::WorkSpacesWeb::IdentityProvider`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-identityprovider.html) resource type.
#[derive(Debug, Default)]
pub struct IdentityProvider {
    properties: IdentityProviderProperties
}

/// Properties for the `IdentityProvider` resource.
#[derive(Debug, Default)]
pub struct IdentityProviderProperties {
    /// Property [`IdentityProviderDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-identityprovider.html#cfn-workspacesweb-identityprovider-identityproviderdetails).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub identity_provider_details: ::ValueMap<String>,
    /// Property [`IdentityProviderName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-identityprovider.html#cfn-workspacesweb-identityprovider-identityprovidername).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub identity_provider_name: ::Value<String>,
    /// Property [`IdentityProviderType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-identityprovider.html#cfn-workspacesweb-identityprovider-identityprovidertype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub identity_provider_type: ::Value<String>,
    /// Property [`PortalArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-identityprovider.html#cfn-workspacesweb-identityprovider-portalarn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub portal_arn: Option<::Value<String>>,
}

impl ::serde::Serialize for IdentityProviderProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdentityProviderDetails", &self.identity_provider_details)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdentityProviderName", &self.identity_provider_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdentityProviderType", &self.identity_provider_type)?;
        if let Some(ref portal_arn) = self.portal_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PortalArn", portal_arn)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for IdentityProviderProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<IdentityProviderProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = IdentityProviderProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type IdentityProviderProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut identity_provider_details: Option<::ValueMap<String>> = None;
                let mut identity_provider_name: Option<::Value<String>> = None;
                let mut identity_provider_type: Option<::Value<String>> = None;
                let mut portal_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "IdentityProviderDetails" => {
                            identity_provider_details = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IdentityProviderName" => {
                            identity_provider_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IdentityProviderType" => {
                            identity_provider_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PortalArn" => {
                            portal_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(IdentityProviderProperties {
                    identity_provider_details: identity_provider_details.ok_or(::serde::de::Error::missing_field("IdentityProviderDetails"))?,
                    identity_provider_name: identity_provider_name.ok_or(::serde::de::Error::missing_field("IdentityProviderName"))?,
                    identity_provider_type: identity_provider_type.ok_or(::serde::de::Error::missing_field("IdentityProviderType"))?,
                    portal_arn: portal_arn,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for IdentityProvider {
    type Properties = IdentityProviderProperties;
    const TYPE: &'static str = "AWS::WorkSpacesWeb::IdentityProvider";
    fn properties(&self) -> &IdentityProviderProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut IdentityProviderProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for IdentityProvider {}

impl From<IdentityProviderProperties> for IdentityProvider {
    fn from(properties: IdentityProviderProperties) -> IdentityProvider {
        IdentityProvider { properties }
    }
}

/// The [`AWS::WorkSpacesWeb::IpAccessSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-ipaccesssettings.html) resource type.
#[derive(Debug, Default)]
pub struct IpAccessSettings {
    properties: IpAccessSettingsProperties
}

/// Properties for the `IpAccessSettings` resource.
#[derive(Debug, Default)]
pub struct IpAccessSettingsProperties {
    /// Property [`AdditionalEncryptionContext`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-ipaccesssettings.html#cfn-workspacesweb-ipaccesssettings-additionalencryptioncontext).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub additional_encryption_context: Option<::ValueMap<String>>,
    /// Property [`CustomerManagedKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-ipaccesssettings.html#cfn-workspacesweb-ipaccesssettings-customermanagedkey).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub customer_managed_key: Option<::Value<String>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-ipaccesssettings.html#cfn-workspacesweb-ipaccesssettings-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`DisplayName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-ipaccesssettings.html#cfn-workspacesweb-ipaccesssettings-displayname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub display_name: Option<::Value<String>>,
    /// Property [`IpRules`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-ipaccesssettings.html#cfn-workspacesweb-ipaccesssettings-iprules).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ip_rules: ::ValueList<self::ip_access_settings::IpRule>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-ipaccesssettings.html#cfn-workspacesweb-ipaccesssettings-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for IpAccessSettingsProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref additional_encryption_context) = self.additional_encryption_context {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdditionalEncryptionContext", additional_encryption_context)?;
        }
        if let Some(ref customer_managed_key) = self.customer_managed_key {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomerManagedKey", customer_managed_key)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref display_name) = self.display_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisplayName", display_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IpRules", &self.ip_rules)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for IpAccessSettingsProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<IpAccessSettingsProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = IpAccessSettingsProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type IpAccessSettingsProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut additional_encryption_context: Option<::ValueMap<String>> = None;
                let mut customer_managed_key: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut display_name: Option<::Value<String>> = None;
                let mut ip_rules: Option<::ValueList<self::ip_access_settings::IpRule>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AdditionalEncryptionContext" => {
                            additional_encryption_context = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CustomerManagedKey" => {
                            customer_managed_key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DisplayName" => {
                            display_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IpRules" => {
                            ip_rules = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(IpAccessSettingsProperties {
                    additional_encryption_context: additional_encryption_context,
                    customer_managed_key: customer_managed_key,
                    description: description,
                    display_name: display_name,
                    ip_rules: ip_rules.ok_or(::serde::de::Error::missing_field("IpRules"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for IpAccessSettings {
    type Properties = IpAccessSettingsProperties;
    const TYPE: &'static str = "AWS::WorkSpacesWeb::IpAccessSettings";
    fn properties(&self) -> &IpAccessSettingsProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut IpAccessSettingsProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for IpAccessSettings {}

impl From<IpAccessSettingsProperties> for IpAccessSettings {
    fn from(properties: IpAccessSettingsProperties) -> IpAccessSettings {
        IpAccessSettings { properties }
    }
}

/// The [`AWS::WorkSpacesWeb::NetworkSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-networksettings.html) resource type.
#[derive(Debug, Default)]
pub struct NetworkSettings {
    properties: NetworkSettingsProperties
}

/// Properties for the `NetworkSettings` resource.
#[derive(Debug, Default)]
pub struct NetworkSettingsProperties {
    /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-networksettings.html#cfn-workspacesweb-networksettings-securitygroupids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub security_group_ids: ::ValueList<String>,
    /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-networksettings.html#cfn-workspacesweb-networksettings-subnetids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub subnet_ids: ::ValueList<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-networksettings.html#cfn-workspacesweb-networksettings-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`VpcId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-networksettings.html#cfn-workspacesweb-networksettings-vpcid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub vpc_id: ::Value<String>,
}

impl ::serde::Serialize for NetworkSettingsProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", &self.security_group_ids)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", &self.subnet_ids)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcId", &self.vpc_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for NetworkSettingsProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<NetworkSettingsProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = NetworkSettingsProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type NetworkSettingsProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut security_group_ids: Option<::ValueList<String>> = None;
                let mut subnet_ids: Option<::ValueList<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut vpc_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "SecurityGroupIds" => {
                            security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SubnetIds" => {
                            subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcId" => {
                            vpc_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(NetworkSettingsProperties {
                    security_group_ids: security_group_ids.ok_or(::serde::de::Error::missing_field("SecurityGroupIds"))?,
                    subnet_ids: subnet_ids.ok_or(::serde::de::Error::missing_field("SubnetIds"))?,
                    tags: tags,
                    vpc_id: vpc_id.ok_or(::serde::de::Error::missing_field("VpcId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for NetworkSettings {
    type Properties = NetworkSettingsProperties;
    const TYPE: &'static str = "AWS::WorkSpacesWeb::NetworkSettings";
    fn properties(&self) -> &NetworkSettingsProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut NetworkSettingsProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for NetworkSettings {}

impl From<NetworkSettingsProperties> for NetworkSettings {
    fn from(properties: NetworkSettingsProperties) -> NetworkSettings {
        NetworkSettings { properties }
    }
}

/// The [`AWS::WorkSpacesWeb::Portal`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-portal.html) resource type.
#[derive(Debug, Default)]
pub struct Portal {
    properties: PortalProperties
}

/// Properties for the `Portal` resource.
#[derive(Debug, Default)]
pub struct PortalProperties {
    /// Property [`AdditionalEncryptionContext`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-portal.html#cfn-workspacesweb-portal-additionalencryptioncontext).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub additional_encryption_context: Option<::ValueMap<String>>,
    /// Property [`AuthenticationType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-portal.html#cfn-workspacesweb-portal-authenticationtype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub authentication_type: Option<::Value<String>>,
    /// Property [`BrowserSettingsArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-portal.html#cfn-workspacesweb-portal-browsersettingsarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub browser_settings_arn: Option<::Value<String>>,
    /// Property [`CustomerManagedKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-portal.html#cfn-workspacesweb-portal-customermanagedkey).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub customer_managed_key: Option<::Value<String>>,
    /// Property [`DisplayName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-portal.html#cfn-workspacesweb-portal-displayname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub display_name: Option<::Value<String>>,
    /// Property [`IpAccessSettingsArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-portal.html#cfn-workspacesweb-portal-ipaccesssettingsarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub ip_access_settings_arn: Option<::Value<String>>,
    /// Property [`NetworkSettingsArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-portal.html#cfn-workspacesweb-portal-networksettingsarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub network_settings_arn: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-portal.html#cfn-workspacesweb-portal-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TrustStoreArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-portal.html#cfn-workspacesweb-portal-truststorearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub trust_store_arn: Option<::Value<String>>,
    /// Property [`UserAccessLoggingSettingsArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-portal.html#cfn-workspacesweb-portal-useraccessloggingsettingsarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub user_access_logging_settings_arn: Option<::Value<String>>,
    /// Property [`UserSettingsArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-portal.html#cfn-workspacesweb-portal-usersettingsarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub user_settings_arn: Option<::Value<String>>,
}

impl ::serde::Serialize for PortalProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref additional_encryption_context) = self.additional_encryption_context {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdditionalEncryptionContext", additional_encryption_context)?;
        }
        if let Some(ref authentication_type) = self.authentication_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthenticationType", authentication_type)?;
        }
        if let Some(ref browser_settings_arn) = self.browser_settings_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BrowserSettingsArn", browser_settings_arn)?;
        }
        if let Some(ref customer_managed_key) = self.customer_managed_key {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomerManagedKey", customer_managed_key)?;
        }
        if let Some(ref display_name) = self.display_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisplayName", display_name)?;
        }
        if let Some(ref ip_access_settings_arn) = self.ip_access_settings_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IpAccessSettingsArn", ip_access_settings_arn)?;
        }
        if let Some(ref network_settings_arn) = self.network_settings_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkSettingsArn", network_settings_arn)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref trust_store_arn) = self.trust_store_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TrustStoreArn", trust_store_arn)?;
        }
        if let Some(ref user_access_logging_settings_arn) = self.user_access_logging_settings_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserAccessLoggingSettingsArn", user_access_logging_settings_arn)?;
        }
        if let Some(ref user_settings_arn) = self.user_settings_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserSettingsArn", user_settings_arn)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PortalProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PortalProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PortalProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PortalProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut additional_encryption_context: Option<::ValueMap<String>> = None;
                let mut authentication_type: Option<::Value<String>> = None;
                let mut browser_settings_arn: Option<::Value<String>> = None;
                let mut customer_managed_key: Option<::Value<String>> = None;
                let mut display_name: Option<::Value<String>> = None;
                let mut ip_access_settings_arn: Option<::Value<String>> = None;
                let mut network_settings_arn: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut trust_store_arn: Option<::Value<String>> = None;
                let mut user_access_logging_settings_arn: Option<::Value<String>> = None;
                let mut user_settings_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AdditionalEncryptionContext" => {
                            additional_encryption_context = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AuthenticationType" => {
                            authentication_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "BrowserSettingsArn" => {
                            browser_settings_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CustomerManagedKey" => {
                            customer_managed_key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DisplayName" => {
                            display_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IpAccessSettingsArn" => {
                            ip_access_settings_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NetworkSettingsArn" => {
                            network_settings_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TrustStoreArn" => {
                            trust_store_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserAccessLoggingSettingsArn" => {
                            user_access_logging_settings_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserSettingsArn" => {
                            user_settings_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PortalProperties {
                    additional_encryption_context: additional_encryption_context,
                    authentication_type: authentication_type,
                    browser_settings_arn: browser_settings_arn,
                    customer_managed_key: customer_managed_key,
                    display_name: display_name,
                    ip_access_settings_arn: ip_access_settings_arn,
                    network_settings_arn: network_settings_arn,
                    tags: tags,
                    trust_store_arn: trust_store_arn,
                    user_access_logging_settings_arn: user_access_logging_settings_arn,
                    user_settings_arn: user_settings_arn,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Portal {
    type Properties = PortalProperties;
    const TYPE: &'static str = "AWS::WorkSpacesWeb::Portal";
    fn properties(&self) -> &PortalProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PortalProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Portal {}

impl From<PortalProperties> for Portal {
    fn from(properties: PortalProperties) -> Portal {
        Portal { properties }
    }
}

/// The [`AWS::WorkSpacesWeb::TrustStore`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-truststore.html) resource type.
#[derive(Debug, Default)]
pub struct TrustStore {
    properties: TrustStoreProperties
}

/// Properties for the `TrustStore` resource.
#[derive(Debug, Default)]
pub struct TrustStoreProperties {
    /// Property [`CertificateList`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-truststore.html#cfn-workspacesweb-truststore-certificatelist).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub certificate_list: ::ValueList<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-truststore.html#cfn-workspacesweb-truststore-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for TrustStoreProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertificateList", &self.certificate_list)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for TrustStoreProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<TrustStoreProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TrustStoreProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type TrustStoreProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut certificate_list: Option<::ValueList<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CertificateList" => {
                            certificate_list = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(TrustStoreProperties {
                    certificate_list: certificate_list.ok_or(::serde::de::Error::missing_field("CertificateList"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for TrustStore {
    type Properties = TrustStoreProperties;
    const TYPE: &'static str = "AWS::WorkSpacesWeb::TrustStore";
    fn properties(&self) -> &TrustStoreProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TrustStoreProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for TrustStore {}

impl From<TrustStoreProperties> for TrustStore {
    fn from(properties: TrustStoreProperties) -> TrustStore {
        TrustStore { properties }
    }
}

/// The [`AWS::WorkSpacesWeb::UserAccessLoggingSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-useraccessloggingsettings.html) resource type.
#[derive(Debug, Default)]
pub struct UserAccessLoggingSettings {
    properties: UserAccessLoggingSettingsProperties
}

/// Properties for the `UserAccessLoggingSettings` resource.
#[derive(Debug, Default)]
pub struct UserAccessLoggingSettingsProperties {
    /// Property [`KinesisStreamArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-useraccessloggingsettings.html#cfn-workspacesweb-useraccessloggingsettings-kinesisstreamarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub kinesis_stream_arn: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-useraccessloggingsettings.html#cfn-workspacesweb-useraccessloggingsettings-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for UserAccessLoggingSettingsProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "KinesisStreamArn", &self.kinesis_stream_arn)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for UserAccessLoggingSettingsProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<UserAccessLoggingSettingsProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = UserAccessLoggingSettingsProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type UserAccessLoggingSettingsProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut kinesis_stream_arn: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "KinesisStreamArn" => {
                            kinesis_stream_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(UserAccessLoggingSettingsProperties {
                    kinesis_stream_arn: kinesis_stream_arn.ok_or(::serde::de::Error::missing_field("KinesisStreamArn"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for UserAccessLoggingSettings {
    type Properties = UserAccessLoggingSettingsProperties;
    const TYPE: &'static str = "AWS::WorkSpacesWeb::UserAccessLoggingSettings";
    fn properties(&self) -> &UserAccessLoggingSettingsProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut UserAccessLoggingSettingsProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for UserAccessLoggingSettings {}

impl From<UserAccessLoggingSettingsProperties> for UserAccessLoggingSettings {
    fn from(properties: UserAccessLoggingSettingsProperties) -> UserAccessLoggingSettings {
        UserAccessLoggingSettings { properties }
    }
}

/// The [`AWS::WorkSpacesWeb::UserSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-usersettings.html) resource type.
#[derive(Debug, Default)]
pub struct UserSettings {
    properties: UserSettingsProperties
}

/// Properties for the `UserSettings` resource.
#[derive(Debug, Default)]
pub struct UserSettingsProperties {
    /// Property [`AdditionalEncryptionContext`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-usersettings.html#cfn-workspacesweb-usersettings-additionalencryptioncontext).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub additional_encryption_context: Option<::ValueMap<String>>,
    /// Property [`CookieSynchronizationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-usersettings.html#cfn-workspacesweb-usersettings-cookiesynchronizationconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub cookie_synchronization_configuration: Option<::Value<self::user_settings::CookieSynchronizationConfiguration>>,
    /// Property [`CopyAllowed`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-usersettings.html#cfn-workspacesweb-usersettings-copyallowed).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub copy_allowed: ::Value<String>,
    /// Property [`CustomerManagedKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-usersettings.html#cfn-workspacesweb-usersettings-customermanagedkey).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub customer_managed_key: Option<::Value<String>>,
    /// Property [`DisconnectTimeoutInMinutes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-usersettings.html#cfn-workspacesweb-usersettings-disconnecttimeoutinminutes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub disconnect_timeout_in_minutes: Option<::Value<f64>>,
    /// Property [`DownloadAllowed`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-usersettings.html#cfn-workspacesweb-usersettings-downloadallowed).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub download_allowed: ::Value<String>,
    /// Property [`IdleDisconnectTimeoutInMinutes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-usersettings.html#cfn-workspacesweb-usersettings-idledisconnecttimeoutinminutes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub idle_disconnect_timeout_in_minutes: Option<::Value<f64>>,
    /// Property [`PasteAllowed`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-usersettings.html#cfn-workspacesweb-usersettings-pasteallowed).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub paste_allowed: ::Value<String>,
    /// Property [`PrintAllowed`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-usersettings.html#cfn-workspacesweb-usersettings-printallowed).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub print_allowed: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-usersettings.html#cfn-workspacesweb-usersettings-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`UploadAllowed`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesweb-usersettings.html#cfn-workspacesweb-usersettings-uploadallowed).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub upload_allowed: ::Value<String>,
}

impl ::serde::Serialize for UserSettingsProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref additional_encryption_context) = self.additional_encryption_context {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdditionalEncryptionContext", additional_encryption_context)?;
        }
        if let Some(ref cookie_synchronization_configuration) = self.cookie_synchronization_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CookieSynchronizationConfiguration", cookie_synchronization_configuration)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CopyAllowed", &self.copy_allowed)?;
        if let Some(ref customer_managed_key) = self.customer_managed_key {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomerManagedKey", customer_managed_key)?;
        }
        if let Some(ref disconnect_timeout_in_minutes) = self.disconnect_timeout_in_minutes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisconnectTimeoutInMinutes", disconnect_timeout_in_minutes)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DownloadAllowed", &self.download_allowed)?;
        if let Some(ref idle_disconnect_timeout_in_minutes) = self.idle_disconnect_timeout_in_minutes {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdleDisconnectTimeoutInMinutes", idle_disconnect_timeout_in_minutes)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PasteAllowed", &self.paste_allowed)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrintAllowed", &self.print_allowed)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "UploadAllowed", &self.upload_allowed)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for UserSettingsProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<UserSettingsProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = UserSettingsProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type UserSettingsProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut additional_encryption_context: Option<::ValueMap<String>> = None;
                let mut cookie_synchronization_configuration: Option<::Value<self::user_settings::CookieSynchronizationConfiguration>> = None;
                let mut copy_allowed: Option<::Value<String>> = None;
                let mut customer_managed_key: Option<::Value<String>> = None;
                let mut disconnect_timeout_in_minutes: Option<::Value<f64>> = None;
                let mut download_allowed: Option<::Value<String>> = None;
                let mut idle_disconnect_timeout_in_minutes: Option<::Value<f64>> = None;
                let mut paste_allowed: Option<::Value<String>> = None;
                let mut print_allowed: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut upload_allowed: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AdditionalEncryptionContext" => {
                            additional_encryption_context = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CookieSynchronizationConfiguration" => {
                            cookie_synchronization_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CopyAllowed" => {
                            copy_allowed = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CustomerManagedKey" => {
                            customer_managed_key = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DisconnectTimeoutInMinutes" => {
                            disconnect_timeout_in_minutes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DownloadAllowed" => {
                            download_allowed = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IdleDisconnectTimeoutInMinutes" => {
                            idle_disconnect_timeout_in_minutes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PasteAllowed" => {
                            paste_allowed = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PrintAllowed" => {
                            print_allowed = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UploadAllowed" => {
                            upload_allowed = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(UserSettingsProperties {
                    additional_encryption_context: additional_encryption_context,
                    cookie_synchronization_configuration: cookie_synchronization_configuration,
                    copy_allowed: copy_allowed.ok_or(::serde::de::Error::missing_field("CopyAllowed"))?,
                    customer_managed_key: customer_managed_key,
                    disconnect_timeout_in_minutes: disconnect_timeout_in_minutes,
                    download_allowed: download_allowed.ok_or(::serde::de::Error::missing_field("DownloadAllowed"))?,
                    idle_disconnect_timeout_in_minutes: idle_disconnect_timeout_in_minutes,
                    paste_allowed: paste_allowed.ok_or(::serde::de::Error::missing_field("PasteAllowed"))?,
                    print_allowed: print_allowed.ok_or(::serde::de::Error::missing_field("PrintAllowed"))?,
                    tags: tags,
                    upload_allowed: upload_allowed.ok_or(::serde::de::Error::missing_field("UploadAllowed"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for UserSettings {
    type Properties = UserSettingsProperties;
    const TYPE: &'static str = "AWS::WorkSpacesWeb::UserSettings";
    fn properties(&self) -> &UserSettingsProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut UserSettingsProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for UserSettings {}

impl From<UserSettingsProperties> for UserSettings {
    fn from(properties: UserSettingsProperties) -> UserSettings {
        UserSettings { properties }
    }
}

pub mod ip_access_settings {
    //! Property types for the `IpAccessSettings` resource.

    /// The [`AWS::WorkSpacesWeb::IpAccessSettings.IpRule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesweb-ipaccesssettings-iprule.html) property type.
    #[derive(Debug, Default)]
    pub struct IpRule {
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesweb-ipaccesssettings-iprule.html#cfn-workspacesweb-ipaccesssettings-iprule-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`IpRange`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesweb-ipaccesssettings-iprule.html#cfn-workspacesweb-ipaccesssettings-iprule-iprange).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub ip_range: ::Value<String>,
    }

    impl ::codec::SerializeValue for IpRule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IpRange", &self.ip_range)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for IpRule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<IpRule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = IpRule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type IpRule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut description: Option<::Value<String>> = None;
                    let mut ip_range: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IpRange" => {
                                ip_range = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(IpRule {
                        description: description,
                        ip_range: ip_range.ok_or(::serde::de::Error::missing_field("IpRange"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod user_settings {
    //! Property types for the `UserSettings` resource.

    /// The [`AWS::WorkSpacesWeb::UserSettings.CookieSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesweb-usersettings-cookiespecification.html) property type.
    #[derive(Debug, Default)]
    pub struct CookieSpecification {
        /// Property [`Domain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesweb-usersettings-cookiespecification.html#cfn-workspacesweb-usersettings-cookiespecification-domain).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub domain: ::Value<String>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesweb-usersettings-cookiespecification.html#cfn-workspacesweb-usersettings-cookiespecification-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: Option<::Value<String>>,
        /// Property [`Path`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesweb-usersettings-cookiespecification.html#cfn-workspacesweb-usersettings-cookiespecification-path).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub path: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for CookieSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Domain", &self.domain)?;
            if let Some(ref name) = self.name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
            }
            if let Some(ref path) = self.path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Path", path)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CookieSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CookieSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CookieSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CookieSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut domain: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut path: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Domain" => {
                                domain = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Path" => {
                                path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CookieSpecification {
                        domain: domain.ok_or(::serde::de::Error::missing_field("Domain"))?,
                        name: name,
                        path: path,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::WorkSpacesWeb::UserSettings.CookieSynchronizationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesweb-usersettings-cookiesynchronizationconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct CookieSynchronizationConfiguration {
        /// Property [`Allowlist`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesweb-usersettings-cookiesynchronizationconfiguration.html#cfn-workspacesweb-usersettings-cookiesynchronizationconfiguration-allowlist).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub allowlist: ::ValueList<CookieSpecification>,
        /// Property [`Blocklist`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesweb-usersettings-cookiesynchronizationconfiguration.html#cfn-workspacesweb-usersettings-cookiesynchronizationconfiguration-blocklist).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub blocklist: Option<::ValueList<CookieSpecification>>,
    }

    impl ::codec::SerializeValue for CookieSynchronizationConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Allowlist", &self.allowlist)?;
            if let Some(ref blocklist) = self.blocklist {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Blocklist", blocklist)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CookieSynchronizationConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CookieSynchronizationConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CookieSynchronizationConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CookieSynchronizationConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut allowlist: Option<::ValueList<CookieSpecification>> = None;
                    let mut blocklist: Option<::ValueList<CookieSpecification>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Allowlist" => {
                                allowlist = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Blocklist" => {
                                blocklist = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CookieSynchronizationConfiguration {
                        allowlist: allowlist.ok_or(::serde::de::Error::missing_field("Allowlist"))?,
                        blocklist: blocklist,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}
